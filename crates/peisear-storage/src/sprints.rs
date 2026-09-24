//! Sprint storage (0.15.0).
//!
//! ## API surface
//!
//! Reads:
//! - [`find_by_id`] — single sprint
//! - [`list_for_team`] — all sprints in a team, ordered by
//!   start date (newest first)
//! - [`active_for_team`] — the (zero or one) currently active
//!   sprint
//! - [`issues_in_sprint`] — issues linked to one sprint
//! - [`summary`] — current point/count totals (committed,
//!   completed, carried-over). Computed live; no caching.
//! - [`burndown`] — a series of per-day cumulative data points
//!   for the burndown chart
//! - [`recent_completed_for_team`] — for the velocity chart
//! - [`distinct_contributors`] — `QA-017`, `NFR-PRIV-007`: distinct
//!   people who completed at least one issue across a set of sprints,
//!   gating the burndown's trajectory and the velocity chart's median
//!   line
//!
//! Writes:
//! - [`insert`] / [`update`] / [`delete`]
//! - [`start`] / [`complete`] — lifecycle transitions
//! - [`add_issue`] / [`remove_issue`] / [`move_issue_to_sprint`]
//!   — issue ↔ sprint membership
//!
//! ## What's *not* here
//!
//! No completion-percentage helper, no estimated-finish
//! prediction, no "ahead of schedule / behind" classifier.
//! These would be evaluative; the team forms its own view of
//! the data.

use chrono::{DateTime, NaiveDate, Utc};
use peisear_core::sprints::{BurndownPoint, Sprint, SprintStatus, SprintSummary};
use peisear_core::{Issue, IssueStatus, Priority};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{Guarded, Pool, StorageError, StorageResult, stamp_matches, zero_rows_outcome};

/// Raw `sprints` row as returned by sqlx. Kept private — the
/// public API returns [`peisear_core::sprints::Sprint`], which
/// carries a parsed `SprintStatus` rather than the raw column
/// string.
#[derive(FromRow)]
struct SprintRow {
    id: String,
    team_id: String,
    name: String,
    goal: Option<String>,
    starts_on: NaiveDate,
    ends_on: NaiveDate,
    status: String,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<SprintRow> for Sprint {
    fn from(r: SprintRow) -> Self {
        Sprint {
            id: r.id,
            team_id: r.team_id,
            name: r.name,
            goal: r.goal,
            starts_on: r.starts_on,
            ends_on: r.ends_on,
            status: SprintStatus::from_storage_str(&r.status),
            started_at: r.started_at,
            completed_at: r.completed_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

pub async fn find_by_id<'e, E>(executor: E, id: &str) -> StorageResult<Option<Sprint>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let row = sqlx::query_as::<_, SprintRow>(
        r#"
        SELECT id, team_id, name, goal, starts_on, ends_on,
               status, started_at, completed_at, created_at, updated_at
        FROM sprints
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(executor)
    .await?;
    Ok(row.map(Sprint::from))
}

pub async fn list_for_team(pool: &Pool, team_id: &str) -> StorageResult<Vec<Sprint>> {
    let rows = sqlx::query_as::<_, SprintRow>(
        r#"
        SELECT id, team_id, name, goal, starts_on, ends_on,
               status, started_at, completed_at, created_at, updated_at
        FROM sprints
        WHERE team_id = ?1
        ORDER BY starts_on DESC, created_at DESC, rowid DESC
        "#,
    )
    .bind(team_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Sprint::from).collect())
}

/// The (zero or one) currently active sprint. The application
/// allows at most one `active` sprint per team — enforced at the
/// `start` call site, not the schema level.
pub async fn active_for_team<'e, E>(executor: E, team_id: &str) -> StorageResult<Option<Sprint>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let row = sqlx::query_as::<_, SprintRow>(
        r#"
        SELECT id, team_id, name, goal, starts_on, ends_on,
               status, started_at, completed_at, created_at, updated_at
        FROM sprints
        WHERE team_id = ?1 AND status = 'active'
        ORDER BY started_at DESC, rowid DESC
        LIMIT 1
        "#,
    )
    .bind(team_id)
    .fetch_optional(executor)
    .await?;
    Ok(row.map(Sprint::from))
}

pub async fn insert(
    pool: &Pool,
    team_id: &str,
    name: &str,
    goal: Option<&str>,
    starts_on: NaiveDate,
    ends_on: NaiveDate,
) -> StorageResult<String> {
    if starts_on > ends_on {
        return Err(StorageError::Validation(
            peisear_i18n::MessageKey::SprintEndDateMustBeOnOrAfterStartMessage,
        ));
    }
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO sprints (id, team_id, name, goal, starts_on, ends_on)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(team_id)
    .bind(name)
    .bind(goal)
    .bind(starts_on)
    .bind(ends_on)
    .execute(pool)
    .await?;
    Ok(id)
}

pub async fn update(
    pool: &Pool,
    id: &str,
    name: &str,
    goal: Option<&str>,
    starts_on: NaiveDate,
    ends_on: NaiveDate,
) -> StorageResult<()> {
    update_guarded(pool, id, name, goal, starts_on, ends_on, None)
        .await
        .map(Guarded::unguarded)
}

/// [`update`], applied only if the sprint still carries `expected` as its
/// `updated_at` (`RACE-002`) -- a predicate on the one `UPDATE` (see
/// `projects::update_guarded` for why the `julianday` form).
pub async fn update_guarded(
    pool: &Pool,
    id: &str,
    name: &str,
    goal: Option<&str>,
    starts_on: NaiveDate,
    ends_on: NaiveDate,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<()>> {
    if starts_on > ends_on {
        return Err(StorageError::Validation(
            peisear_i18n::MessageKey::SprintEndDateMustBeOnOrAfterStartMessage,
        ));
    }
    let res = sqlx::query(
        r#"
        UPDATE sprints
        SET name = ?2, goal = ?3, starts_on = ?4, ends_on = ?5
        WHERE id = ?1
          AND (?6 IS NULL OR julianday(updated_at) = julianday(?6))
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(goal)
    .bind(starts_on)
    .bind(ends_on)
    .bind(expected)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return zero_rows_outcome(pool, "sprints", id, None).await;
    }
    Ok(Guarded::Written(()))
}

pub async fn delete(pool: &Pool, id: &str) -> StorageResult<()> {
    delete_guarded(pool, id, None).await.map(Guarded::unguarded)
}

/// [`delete`], applied only if the sprint still carries `expected` as its
/// `updated_at` (`RACE-002`). This is also what closes the
/// delete-versus-`start` race `RACE-001` found: `start` moves the stamp,
/// so a delete that read the sprint as planned and lands after a start
/// finds the stamp changed and is refused, rather than deleting a running
/// sprint.
pub async fn delete_guarded(
    pool: &Pool,
    id: &str,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<()>> {
    let res = sqlx::query(
        r#"
        DELETE FROM sprints
        WHERE id = ?1
          AND (?2 IS NULL OR julianday(updated_at) = julianday(?2))
        "#,
    )
    .bind(id)
    .bind(expected)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return zero_rows_outcome(pool, "sprints", id, None).await;
    }
    Ok(Guarded::Written(()))
}

/// Transition `planned` → `active`. Refuses if another sprint
/// in the same team is already active (prevents two-active-
/// sprint ambiguity).
///
/// **The check and the write are one transaction (`RACE-001`).** This
/// used to read on the pool and write on the pool, so two concurrent
/// starts on different sprints of one team both saw no active sprint
/// and both started, leaving two -- a state this function's own rule
/// says cannot exist, and one [`active_for_team`]'s deterministic pick
/// would then have hidden. It opens `BEGIN IMMEDIATE` (the reasoning
/// for `IMMEDIATE` over a deferred `BEGIN` is in
/// `user_capacities`'s module docs) so the second start waits, sees the
/// first, and is refused. Starts in different teams contend only for
/// the database's one write lock, briefly.
pub async fn start(pool: &Pool, sprint_id: &str) -> StorageResult<()> {
    start_guarded(pool, sprint_id, None)
        .await
        .map(Guarded::unguarded)
}

/// [`start`], applied only if the sprint still carries `expected` as its
/// `updated_at` (`RACE-002`). The stamp comes from the same read, inside
/// the same `BEGIN IMMEDIATE` transaction, as the state checks below, and
/// is answered first -- the order the handler's own comparison had.
pub async fn start_guarded(
    pool: &Pool,
    sprint_id: &str,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<()>> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    let sprint = find_by_id(&mut *tx, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
    if !stamp_matches(expected, sprint.updated_at) {
        return Ok(Guarded::Stale {
            current_updated_at: sprint.updated_at,
        });
    }
    match sprint.status {
        SprintStatus::Planned => {}
        SprintStatus::Active => {
            return Err(StorageError::Validation(
                peisear_i18n::MessageKey::SprintAlreadyActiveMessage,
            ));
        }
        SprintStatus::Completed => {
            return Err(StorageError::Validation(
                peisear_i18n::MessageKey::SprintCannotRestartCompletedMessage,
            ));
        }
    }
    if let Some(other) = active_for_team(&mut *tx, &sprint.team_id).await? {
        return Err(StorageError::Conflict(
            peisear_i18n::MessageKey::OtherSprintActiveInTeamMessage {
                sprint_name: other.name,
            },
        ));
    }
    sqlx::query(
        r#"
        UPDATE sprints
        SET status = 'active', started_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(sprint_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Guarded::Written(()))
}

/// Transition `active` → `completed`. Sprint can only be
/// completed if currently active.
pub async fn complete(pool: &Pool, sprint_id: &str) -> StorageResult<()> {
    complete_guarded(pool, sprint_id, None)
        .await
        .map(Guarded::unguarded)
}

/// [`complete`], applied only if the sprint still carries `expected` as
/// its `updated_at` (`RACE-002`). Like [`start_guarded`] it reads and
/// writes under one `BEGIN IMMEDIATE` transaction; that also makes two
/// simultaneous completes a single one (the second sees `Completed`),
/// where before both passed and both wrote `completed_at`.
pub async fn complete_guarded(
    pool: &Pool,
    sprint_id: &str,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<()>> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    let sprint = find_by_id(&mut *tx, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
    if !stamp_matches(expected, sprint.updated_at) {
        return Ok(Guarded::Stale {
            current_updated_at: sprint.updated_at,
        });
    }
    match sprint.status {
        SprintStatus::Active => {}
        SprintStatus::Planned => {
            return Err(StorageError::Validation(
                peisear_i18n::MessageKey::SprintNotStartedYetMessage,
            ));
        }
        SprintStatus::Completed => {
            return Err(StorageError::Validation(
                peisear_i18n::MessageKey::SprintAlreadyCompletedMessage,
            ));
        }
    }
    sqlx::query(
        r#"
        UPDATE sprints
        SET status = 'completed', completed_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(sprint_id)
    .execute(&mut *tx)
    .await?;
    // Capture what the sprint reports, in this same transaction (`DEC-054`).
    // Inside it are: the status `UPDATE` above, the reads of current
    // membership and issue state that `live_totals` and `burndown_live` make,
    // and the inserts of the record and the series. Outside it is nothing --
    // a crash or a competing writer cannot leave a `completed` sprint without
    // its record, nor a record for a sprint that did not complete.
    capture_record(&mut tx, sprint_id).await?;
    tx.commit().await?;
    Ok(Guarded::Written(()))
}

/// Write the record of a sprint that has just been set `completed`, on the
/// transaction that set it. `completed_at` is already written there, which
/// the burndown's end date reads.
async fn capture_record(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    sprint_id: &str,
) -> StorageResult<()> {
    let sprint = find_by_id(&mut **tx, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
    let (committed_points, completed_points, committed_count, completed_count) =
        live_totals(tx, sprint_id).await?;
    let points = burndown_live(tx, &sprint).await?;
    let (contributor_count, unassigned) = live_contributor_basis(tx, sprint_id).await?;

    // A stale record cannot be here (`reopen` deletes it), but replacing is
    // the right answer if one were: the record is what the sprint reports *now*.
    sqlx::query("DELETE FROM sprint_burndown_points WHERE sprint_id = ?1")
        .bind(sprint_id)
        .execute(&mut **tx)
        .await?;
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO sprint_records
            (sprint_id, committed_points, completed_points, committed_count, completed_count,
             contributor_count, had_unassigned_contributor)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
    )
    .bind(sprint_id)
    .bind(committed_points)
    .bind(completed_points)
    .bind(committed_count)
    .bind(completed_count)
    .bind(contributor_count)
    .bind(i64::from(unassigned > 0))
    .execute(&mut **tx)
    .await?;
    for p in points {
        sqlx::query(
            r#"
            INSERT INTO sprint_burndown_points
                (sprint_id, day, cumulative_committed, cumulative_completed)
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(sprint_id)
        .bind(p.day)
        .bind(p.cumulative_committed)
        .bind(p.cumulative_completed)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

/// Return a `completed` sprint to `active` (`DEC-053`; `SPRINT-004`): **un-capture
/// and resume**. The record captured at completion is deleted, so the sprint is
/// live again -- its figures and burndown follow its membership -- and
/// completing it again captures afresh.
pub async fn reopen(pool: &Pool, sprint_id: &str) -> StorageResult<()> {
    reopen_guarded(pool, sprint_id, None)
        .await
        .map(Guarded::unguarded)
}

/// [`reopen`], applied only if the sprint still carries `expected` as its
/// `updated_at` (`RACE-002`). One `BEGIN IMMEDIATE` transaction holds: the
/// stamp and status reads, the check that the team has no other active sprint
/// (`FR-SPR-002` binds reopen exactly as it binds [`start`] -- and non-atomically
/// this is the defect that left six active sprints, `RACE-001`), the status
/// write, and the two deletes. A sprint that is not `completed` is refused with
/// `SprintNotCompletedMessage`; another active sprint with
/// `OtherSprintActiveInTeamMessage`, the words `start` uses.
pub async fn reopen_guarded(
    pool: &Pool,
    sprint_id: &str,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<()>> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    let sprint = find_by_id(&mut *tx, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
    if !stamp_matches(expected, sprint.updated_at) {
        return Ok(Guarded::Stale {
            current_updated_at: sprint.updated_at,
        });
    }
    if !matches!(sprint.status, SprintStatus::Completed) {
        return Err(StorageError::Validation(
            peisear_i18n::MessageKey::SprintNotCompletedMessage,
        ));
    }
    if let Some(other) = active_for_team(&mut *tx, &sprint.team_id).await? {
        return Err(StorageError::Conflict(
            peisear_i18n::MessageKey::OtherSprintActiveInTeamMessage {
                sprint_name: other.name,
            },
        ));
    }
    sqlx::query(
        r#"
        UPDATE sprints
        SET status = 'active', completed_at = NULL
        WHERE id = ?1
        "#,
    )
    .bind(sprint_id)
    .execute(&mut *tx)
    .await?;
    sqlx::query("DELETE FROM sprint_burndown_points WHERE sprint_id = ?1")
        .bind(sprint_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM sprint_records WHERE sprint_id = ?1")
        .bind(sprint_id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(Guarded::Written(()))
}

/// [`add_issue`], but only if the sprint's status is one of `allowed` **at
/// the moment of the write** (`RACE-003`). The caller passes the refusal it
/// owes, so each route keeps its own message.
///
/// The routes that add an issue read the sprint's status first and refuse
/// on it; that read is on the pool, so a `start` or `complete` landing
/// between it and the `INSERT` left an issue in a sprint the request would
/// have refused. Neither route carries an optimistic lock (a deliberate
/// choice, not this function's to change), so there is no stamp to move.
/// The status read and the insert now share one `BEGIN IMMEDIATE`
/// transaction -- see `user_capacities`'s module docs for why `IMMEDIATE`
/// -- and [`start_guarded`] / [`complete_guarded`] take the same lock, so
/// they serialise. A vanished sprint is `NotFound`; a status outside
/// `allowed` is [`StorageError::Conflict`] carrying `refusal`.
pub async fn add_issue_if_status(
    pool: &Pool,
    sprint_id: &str,
    issue_id: &str,
    allowed: &[SprintStatus],
    refusal: peisear_i18n::MessageKey,
) -> StorageResult<()> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    let sprint = find_by_id(&mut *tx, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
    if !allowed.contains(&sprint.status) {
        return Err(StorageError::Conflict(refusal));
    }
    sqlx::query(
        r#"
        INSERT INTO sprint_issues (issue_id, sprint_id)
        VALUES (?1, ?2)
        ON CONFLICT(issue_id) DO UPDATE SET
            sprint_id = excluded.sprint_id,
            assigned_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(issue_id)
    .bind(sprint_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

/// Add an issue to a sprint. If the issue is already in
/// another sprint, it gets moved (single-sprint-per-issue
/// invariant). Returns Ok regardless of whether this was a
/// fresh add or a move.
pub async fn add_issue(pool: &Pool, sprint_id: &str, issue_id: &str) -> StorageResult<()> {
    sqlx::query(
        r#"
        INSERT INTO sprint_issues (issue_id, sprint_id)
        VALUES (?1, ?2)
        ON CONFLICT(issue_id) DO UPDATE SET
            sprint_id = excluded.sprint_id,
            assigned_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(issue_id)
    .bind(sprint_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// [`remove_issue`], but only if the sprint the issue is currently in has a
/// status in `allowed` **at the moment of the delete** (`SPRINT-001` §2,
/// `RACE-003`'s twin of [`add_issue_if_status`]). The caller passes the
/// refusal it owes, so each route keeps its own message.
///
/// `plan_remove` reads the sprint's status and refuses on it; that read is
/// on the pool, so a `start` landing between it and the delete removed an
/// issue from a sprint that was no longer plannable. The status read and the
/// delete share one `BEGIN IMMEDIATE` transaction (see `user_capacities`'s
/// module docs for why `IMMEDIATE`); [`start_guarded`] and
/// [`complete_guarded`] take the same lock.
///
/// An issue that is in no sprint is still not an error -- nothing to
/// remove, as [`remove_issue`] always was.
pub async fn remove_issue_if_status(
    pool: &Pool,
    issue_id: &str,
    allowed: &[SprintStatus],
    refusal: peisear_i18n::MessageKey,
) -> StorageResult<()> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    let status: Option<String> = sqlx::query_scalar(
        r#"
        SELECT s.status
        FROM sprint_issues si
        JOIN sprints s ON s.id = si.sprint_id
        WHERE si.issue_id = ?1
        "#,
    )
    .bind(issue_id)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(status) = status else {
        return Ok(());
    };
    if !allowed.contains(&SprintStatus::from_storage_str(&status)) {
        return Err(StorageError::Conflict(refusal));
    }
    sqlx::query(r#"DELETE FROM sprint_issues WHERE issue_id = ?1"#)
        .bind(issue_id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

/// Remove an issue from its sprint. Idempotent — removing an
/// issue that wasn't in a sprint is not an error (matches the
/// natural "make sure it isn't" mental model).
///
/// **Unconditional: it removes the membership wherever it is.** That is
/// deliberate on a completed sprint too (`DEC-054`): the record of a
/// completed sprint is captured at completion, so its membership may change
/// (carry-over moves issues out of it). `plan_remove` uses
/// [`remove_issue_if_status`] because a plan is only editable while planned.
pub async fn remove_issue(pool: &Pool, issue_id: &str) -> StorageResult<()> {
    sqlx::query(r#"DELETE FROM sprint_issues WHERE issue_id = ?1"#)
        .bind(issue_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Find the sprint an issue is in, or `None` if the issue is
/// not in any sprint.
///
/// Phase C PR1 (peisear-feature-spec-v2.1 §8.5): for sub-
/// issues, this returns the **parent's** sprint. Sub-issues
/// follow the parent's sprint membership and don't get their
/// own row in `sprint_issues` — the planning surface only
/// schedules top-level issues, and a sub-issue's effort is
/// considered scheduled when its parent is.
///
/// Implementation: the SQL coalesces. If the issue itself has
/// a `sprint_issues` row, return it; otherwise look up the
/// parent's row. The single query keeps the round-trip count
/// at one whether the issue is top-level or sub.
pub async fn sprint_for_issue(pool: &Pool, issue_id: &str) -> StorageResult<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT sprint_id FROM sprint_issues
        WHERE issue_id = ?1
           OR issue_id = (
               SELECT parent_issue_id FROM issues
               WHERE id = ?1 AND parent_issue_id IS NOT NULL
           )
        LIMIT 1
        "#,
    )
    .bind(issue_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(s,)| s))
}

/// Issues currently linked to a sprint, with their effort and
/// status. Used by the sprint detail page and the sprint plan's
/// sprint column.
///
/// **Returned in display order (`ORD-003`): Open, In progress, Done,
/// and in assignment order within each status.**
///
/// Phase C PR1: only **top-level** issues are listed here.
/// Sub-issues follow the parent's sprint membership; rendering
/// them as separate rows would double-count them in
/// "committed work" displays. The sub-issues belonging to a
/// listed parent are visible on the parent's detail page.
pub async fn issues_in_sprint(
    pool: &Pool,
    sprint_id: &str,
) -> StorageResult<Vec<(String, String, String, Option<i64>, String)>> {
    // Returns (issue_id, project_id, title, effort, status).
    let mut rows: Vec<(String, String, String, Option<i64>, String)> = sqlx::query_as(
        r#"
        SELECT i.id, i.project_id, i.title, i.effort, i.status
        FROM sprint_issues si
        JOIN issues i ON i.id = si.issue_id
        WHERE si.sprint_id = ?1
          AND i.parent_issue_id IS NULL
        -- Status is deliberately absent from this ORDER BY: it is a TEXT
        -- column, so `status ASC` sorts alphabetically (done, in_progress,
        -- open -- Done first). Lifecycle order is applied in Rust below with
        -- `IssueStatus::lifecycle_rank`. Do not put it back here.
        ORDER BY si.assigned_at ASC, si.rowid ASC
        "#,
    )
    .bind(sprint_id)
    .fetch_all(pool)
    .await?;
    // Grouped Open, In progress, Done. `sort_by_key` is stable, and that
    // is load-bearing: it keeps the query's `assigned_at` order inside each
    // status. A status string that does not parse cannot occur (the column
    // is CHECK-constrained), but sorts last rather than panicking.
    rows.sort_by_key(|(.., status)| {
        peisear_core::IssueStatus::parse(status).map_or(u8::MAX, |s| s.lifecycle_rank())
    });
    Ok(rows)
}

/// The four totals a sprint's figures are made of, computed from **current**
/// membership and issue state: committed points, completed points, committed
/// count, completed count. This is the live computation; a completed sprint
/// does not use it for display (see [`summary`]) -- `complete` calls it once,
/// inside its transaction, to write the record.
async fn live_totals(
    conn: &mut sqlx::SqliteConnection,
    sprint_id: &str,
) -> StorageResult<(i64, i64, i64, i64)> {
    // COALESCE because SUM on an empty set returns NULL.
    let row: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COALESCE(SUM(COALESCE(i.effort, 0)), 0)
                AS committed_points,
            COALESCE(SUM(CASE WHEN i.status = 'done'
                              THEN COALESCE(i.effort, 0) ELSE 0 END), 0)
                AS completed_points,
            COUNT(*)
                AS committed_count,
            COALESCE(SUM(CASE WHEN i.status = 'done' THEN 1 ELSE 0 END), 0)
                AS completed_count
        FROM sprint_issues si
        JOIN issues i ON i.id = si.issue_id
        WHERE si.sprint_id = ?1
        "#,
    )
    .bind(sprint_id)
    .fetch_one(conn)
    .await?;
    Ok(row)
}

/// The summary figures for one sprint.
///
/// **A `completed` sprint reads the record captured when it completed**
/// (`DEC-054`, RFC 0013): what it reported does not change afterwards, however
/// its issues are carried over, finished, edited or re-estimated. Any other
/// sprint computes live from its current membership; carried-over is `0` for
/// those by convention.
///
/// A completed sprint with **no** record is a fault, not a case to fall back
/// on: computing live there would be the drift this function exists to stop,
/// returning quietly with figures that look right. It is an error
/// ([`StorageError::InvalidData`]). Migration `0019` backfills every sprint
/// that was completed before it, and `complete` writes the record in the
/// transaction that sets the status, so a missing record means a bug.
pub async fn summary(pool: &Pool, sprint_id: &str) -> StorageResult<SprintSummary> {
    let sprint = find_by_id(pool, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;

    let (committed_points, completed_points, committed_count, completed_count) = match sprint.status
    {
        SprintStatus::Completed => {
            let row: Option<(i64, i64, i64, i64)> = sqlx::query_as(
                r#"
                SELECT committed_points, completed_points, committed_count, completed_count
                FROM sprint_records
                WHERE sprint_id = ?1
                "#,
            )
            .bind(sprint_id)
            .fetch_optional(pool)
            .await?;
            row.ok_or_else(|| missing_record(sprint_id))?
        }
        _ => {
            let mut conn = pool.acquire().await?;
            live_totals(&mut conn, sprint_id).await?
        }
    };

    // Carried-over is meaningful only on completed sprints: what the sprint
    // reported as committed but not completed. Derived, never stored.
    let (carried_over_points, carried_over_count) = match sprint.status {
        SprintStatus::Completed => (
            (committed_points - completed_points).max(0),
            (committed_count - completed_count).max(0),
        ),
        _ => (0, 0),
    };

    Ok(SprintSummary {
        sprint_id: sprint_id.to_string(),
        committed_points,
        completed_points,
        committed_count,
        completed_count,
        carried_over_points,
        carried_over_count,
    })
}

fn missing_record(sprint_id: &str) -> StorageError {
    StorageError::InvalidData(format!(
        "completed sprint {sprint_id} has no captured record (DEC-054); migration 0019 \
         backfills earlier sprints and `complete` writes one, so this is a fault"
    ))
}

/// One issue's contribution to a sprint's burndown, as read from
/// the join in [`burndown`]. Kept private and local to this
/// function's computation — not a table row on its own.
#[derive(FromRow)]
struct BurndownIssueRow {
    id: String,
    effort: Option<i64>,
    status: String,
    assigned_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// The burndown timeline for a sprint.
///
/// **A `completed` sprint reads the series captured when it completed**
/// (`DEC-054`); any other sprint computes it live. As for [`summary`], a
/// completed sprint with no record is an error, not a reason to compute live.
pub async fn burndown(pool: &Pool, sprint_id: &str) -> StorageResult<Vec<BurndownPoint>> {
    let sprint = find_by_id(pool, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
    if !matches!(sprint.status, SprintStatus::Completed) {
        let mut conn = pool.acquire().await?;
        return burndown_live(&mut conn, &sprint).await;
    }
    let has_record: Option<i64> =
        sqlx::query_scalar("SELECT 1 FROM sprint_records WHERE sprint_id = ?1")
            .bind(sprint_id)
            .fetch_optional(pool)
            .await?;
    if has_record.is_none() {
        return Err(missing_record(sprint_id));
    }
    let rows: Vec<(NaiveDate, i64, i64)> = sqlx::query_as(
        r#"
        SELECT day, cumulative_committed, cumulative_completed
        FROM sprint_burndown_points
        WHERE sprint_id = ?1
        ORDER BY day ASC
        "#,
    )
    .bind(sprint_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(
            |(day, cumulative_committed, cumulative_completed)| BurndownPoint {
                day,
                cumulative_committed,
                cumulative_completed,
            },
        )
        .collect())
}

/// Compute the burndown timeline for a sprint **from current state**. Used
/// for sprints that are not completed, and once by `complete` to write the
/// captured series.
///
/// Strategy:
/// - One row per calendar day from `starts_on` to `min(today,
///   ends_on, completed_at)`.
/// - `cumulative_committed` at day D = sum of effort over
///   issues whose `assigned_at <= end_of_day(D)`.
/// - `cumulative_completed` at day D = sum of effort over
///   issues that reached status `done` on or before D, where
///   "reached done" is the most recent `status_changed -> done`
///   event from `issue_events` (which 0.8.0 introduced).
///
/// Issues without any `status_changed -> done` event but whose
/// current status is `done` are credited at their
/// `updated_at` (a 0.7.0-era fallback for issues that predate
/// the event log).
///
/// We compute in Rust rather than SQL to keep the date-bucketing
/// logic readable. The data volume is bounded (< 30 days × < 100
/// issues = a few thousand rows in the worst case).
async fn burndown_live(
    conn: &mut sqlx::SqliteConnection,
    sprint: &Sprint,
) -> StorageResult<Vec<BurndownPoint>> {
    let sprint_id = sprint.id.as_str();
    let issues: Vec<BurndownIssueRow> = sqlx::query_as(
        r#"
            SELECT i.id, i.effort, i.status, si.assigned_at, i.updated_at
            FROM sprint_issues si
            JOIN issues i ON i.id = si.issue_id
            WHERE si.sprint_id = ?1
            ORDER BY si.assigned_at ASC, si.rowid ASC
            "#,
    )
    .bind(sprint_id)
    .fetch_all(&mut *conn)
    .await?;

    if issues.is_empty() {
        return Ok(Vec::new());
    }

    // Determine the time window. End at the earlier of: today,
    // sprint end, sprint completed_at. Start at sprint start.
    let today = chrono::Utc::now().date_naive();
    let effective_end = sprint
        .completed_at
        .map(|t| t.date_naive())
        .unwrap_or(today)
        .min(sprint.ends_on)
        .min(today);
    let start = sprint.starts_on;

    if effective_end < start {
        return Ok(Vec::new());
    }

    // For each issue currently linked to the sprint, find the
    // date it transitioned to `done` (or None if not done yet).
    let issue_ids: Vec<String> = issues.iter().map(|i| i.id.clone()).collect();
    let placeholders = issue_ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(", ");

    // Last status_changed -> done event per issue. Returns
    // (issue_id, max(occurred_at)).
    let done_events_query = format!(
        r#"
        SELECT issue_id, MAX(occurred_at) AS done_at
        FROM issue_events
        WHERE event_type = 'status_changed'
          AND new_value = 'done'
          AND issue_id IN ({})
        GROUP BY issue_id
        "#,
        placeholders
    );
    let mut q = sqlx::query_as::<_, (String, DateTime<Utc>)>(&done_events_query);
    for id in &issue_ids {
        q = q.bind(id);
    }
    let done_events: Vec<(String, DateTime<Utc>)> = q.fetch_all(&mut *conn).await?;
    let done_map: std::collections::HashMap<String, NaiveDate> = done_events
        .into_iter()
        .map(|(id, ts)| (id, ts.date_naive()))
        .collect();

    // Build the timeline.
    let mut points = Vec::new();
    let mut day = start;
    while day <= effective_end {
        let mut committed: i64 = 0;
        let mut completed: i64 = 0;

        for row in &issues {
            let effort_pt = row.effort.unwrap_or(0);
            // Committed on day D if assigned at or before end-of-D.
            if row.assigned_at.date_naive() <= day {
                committed += effort_pt;
            }
            // Completed on day D if there's a done-event on or
            // before D; otherwise fallback to updated_at if
            // current status is done. The fallback covers
            // issues that predate 0.8.0 events.
            let done_date: Option<NaiveDate> = done_map.get(&row.id).copied().or_else(|| {
                if row.status == "done" {
                    Some(row.updated_at.date_naive())
                } else {
                    None
                }
            });
            if let Some(d) = done_date
                && d <= day
            {
                completed += effort_pt;
            }
        }

        points.push(BurndownPoint {
            day,
            cumulative_committed: committed,
            cumulative_completed: completed,
        });

        day = match day.succ_opt() {
            Some(d) => d,
            None => break,
        };
    }

    Ok(points)
}

/// The contributor floor's input for the given sprints — `QA-017`
/// (`NFR-PRIV-007`): the **largest number of distinct people who completed
/// at least one issue in any single one of them**, or `None` when that is
/// unknown. Contributor is scoped to completed work, not sprint membership: a
/// sprint where Alice completed everything and Bob holds one still-open issue
/// is a sprint whose completion trajectory is Alice's alone, and counting Bob
/// would let his uninvolved presence launder a disclosure that is still
/// entirely about Alice.
///
/// **Each sprint is read from the basis it had when it completed**
/// (`SPRINT-005`, `DEC-054`). A `completed` sprint displays its captured
/// record, and whether that record is reversible to one person is a property
/// of who contributed to it, fixed at completion; asking about *now* let work
/// finished afterwards open a trajectory that was hidden, and carrying a done
/// issue out hide one that was shown. Sprints that are not completed compute
/// their basis live.
///
/// Takes a **slice**, not one id — the burndown's predicate is over a single
/// sprint, but the velocity chart's median spans up to
/// [`peisear_core::sprints::VELOCITY_MEDIAN_WINDOW`] sprints. **The set passes
/// only if at least one sprint clears the floor on its own** (the maximum of
/// the per-sprint counts), which is deliberately stricter than the old union
/// across the window: several sprints each the output of a *different single*
/// person made a union of several people, yet each sprint's number is one
/// person's output, so the aggregate was reversible per sprint. Five solo
/// sprints by the same person still fail, as before.
///
/// Returns `Ok(None)` when the true count is **unknown** rather than
/// computing a possibly-wrong number: if any completed issue in any of the
/// sprints has no assignee, that issue's real contributor could be the same
/// person as every other completed issue, or someone new — there is no way to
/// tell from this data. The safe direction for a privacy predicate is to treat
/// "unknown" the same as "fewer than two"; callers do that by treating `None`
/// as not-enough-to-show, same as `Some(0)` or `Some(1)`. A completed sprint
/// with no record is a fault ([`StorageError::InvalidData`]), as for
/// [`summary`].
pub async fn distinct_contributors(
    pool: &Pool,
    sprint_ids: &[String],
) -> StorageResult<Option<i64>> {
    if sprint_ids.is_empty() {
        return Ok(Some(0));
    }

    // Each sprint's own basis: `(distinct assignees of done issues, whether any
    // done issue had none)`. A completed sprint reads the basis captured with
    // its record; any other sprint computes it live.
    let mut best = 0_i64;
    for id in sprint_ids {
        let Some(sprint) = find_by_id(pool, id).await? else {
            continue;
        };
        let (known, unassigned) = match sprint.status {
            SprintStatus::Completed => {
                let row: Option<(i64, i64)> = sqlx::query_as(
                    r#"
                    SELECT contributor_count, had_unassigned_contributor
                    FROM sprint_records
                    WHERE sprint_id = ?1
                    "#,
                )
                .bind(id)
                .fetch_optional(pool)
                .await?;
                row.ok_or_else(|| missing_record(id))?
            }
            _ => {
                let mut conn = pool.acquire().await?;
                live_contributor_basis(&mut conn, id).await?
            }
        };
        if unassigned > 0 {
            return Ok(None);
        }
        best = best.max(known);
    }
    Ok(Some(best))
}

/// A sprint's contributor basis computed from current membership and status:
/// `(COUNT(DISTINCT assignee_id) over done issues, number of done issues with no
/// assignee)`. Used live for sprints that are not completed, and once by
/// `complete` to write the captured basis.
async fn live_contributor_basis(
    conn: &mut sqlx::SqliteConnection,
    sprint_id: &str,
) -> StorageResult<(i64, i64)> {
    let row: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COUNT(DISTINCT i.assignee_id) AS known,
            COALESCE(SUM(CASE WHEN i.assignee_id IS NULL THEN 1 ELSE 0 END), 0)
                AS unassigned
        FROM sprint_issues si
        JOIN issues i ON i.id = si.issue_id
        WHERE si.sprint_id = ?1
          AND i.status = 'done'
        "#,
    )
    .bind(sprint_id)
    .fetch_one(conn)
    .await?;
    Ok(row)
}

/// Recently completed sprints' summaries for the velocity
/// chart. Returned newest-first; the chart usually displays
/// them oldest-first (left to right), so the caller reverses.
pub async fn recent_completed_for_team(
    pool: &Pool,
    team_id: &str,
    limit: i64,
) -> StorageResult<Vec<(Sprint, SprintSummary)>> {
    let sprints: Vec<Sprint> = {
        let rows = sqlx::query_as::<_, SprintRow>(
            r#"
            SELECT id, team_id, name, goal, starts_on, ends_on,
                   status, started_at, completed_at, created_at, updated_at
            FROM sprints
            WHERE team_id = ?1 AND status = 'completed'
            ORDER BY completed_at DESC, rowid DESC
            LIMIT ?2
            "#,
        )
        .bind(team_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;
        rows.into_iter().map(Sprint::from).collect()
    };

    let mut out = Vec::with_capacity(sprints.len());
    for s in sprints {
        let sum = summary(pool, &s.id).await?;
        out.push((s, sum));
    }
    Ok(out)
}

// ──────────────────────────────────────────────────────────────
// Sprint planning page (RFC 001 / `PLAN-001`)
// ──────────────────────────────────────────────────────────────

/// Optional facets for [`backlog_for_team`]. `None` on `project_id`/
/// `priority` means "no constraint on that facet" — matches the
/// project detail list view's existing `Option<String>` filter
/// convention (`ProjectViewQuery`).
///
/// `assignee_id` carries one extra sentinel beyond that convention:
/// `None` is "no constraint", `Some("unassigned")` is "assignee is
/// nobody", and `Some(other)` is "assignee is exactly this user id"
/// — the same three-way shape the project detail list's assignee
/// filter already exposes in its UI (`UnassignedOption`), so the
/// backlog filter matches it rather than inventing a fourth
/// filtering convention. Safe as a string sentinel because assignee
/// ids are UUIDs and can never literally equal `"unassigned"`.
#[derive(Default)]
pub struct BacklogFilter {
    pub project_id: Option<String>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<String>,
}

/// One backlog candidate: the issue plus the name of the project
/// it belongs to (the backlog spans every project on the team, so
/// the planner needs that context per row — a single-project
/// listing wouldn't).
pub struct BacklogRow {
    pub issue: Issue,
    pub project_name: String,
}

/// Row shape as returned by the `backlog_for_team` query. Kept
/// private and separate from [`IssueRow`] (this module has no
/// `IssueRow`; that type lives in `peisear-storage::issues`) —
/// this crate doesn't share a row type across the two tables'
/// query modules, and duplicating the eleven `issues` columns
/// here is the existing convention (`issues.rs` does the same
/// for `IssueRow` / `BurndownIssueRow`) rather than reaching for
/// `#[sqlx(flatten)]`, which nothing else in this crate uses.
#[derive(FromRow)]
struct BacklogIssueRow {
    id: String,
    project_id: String,
    author_id: String,
    title: String,
    description: String,
    status: String,
    priority: String,
    effort: Option<i64>,
    assignee_id: Option<String>,
    parent_issue_id: Option<String>,
    planned_start_at: Option<DateTime<Utc>>,
    planned_end_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    project_name: String,
}

impl BacklogIssueRow {
    fn into_backlog_row(self) -> StorageResult<BacklogRow> {
        let status = IssueStatus::parse(&self.status)
            .ok_or_else(|| StorageError::InvalidData(format!("status={}", self.status)))?;
        let priority = Priority::parse(&self.priority)
            .ok_or_else(|| StorageError::InvalidData(format!("priority={}", self.priority)))?;
        Ok(BacklogRow {
            issue: Issue {
                id: self.id,
                project_id: self.project_id,
                author_id: self.author_id,
                title: self.title,
                description: self.description,
                status,
                priority,
                effort: self.effort,
                assignee_id: self.assignee_id,
                parent_issue_id: self.parent_issue_id,
                planned_start_at: self.planned_start_at,
                planned_end_at: self.planned_end_at,
                created_at: self.created_at,
                updated_at: self.updated_at,
            },
            project_name: self.project_name,
        })
    }
}

/// Top-level open issues across the team's team-scoped projects
/// (RFC 001 open question 1's default: personal projects are out
/// of scope for team sprint planning) that are not in any
/// **active** sprint — a planned sprint's committed items stay
/// visible as backlog candidates elsewhere until that sprint goes
/// active, matching the RFC's own wording ("not in any active
/// sprint", not "not in any sprint"). This does **not** exclude
/// the sprint currently being planned on its own page — a planned
/// sprint's own items would otherwise still show as backlog
/// candidates for itself. The caller (the plan-page handler)
/// subtracts the current sprint's own `issues_in_sprint` set
/// before rendering, since that's a page-composition concern
/// (which sprint is "this one") that this team-wide query has no
/// way to know from its own parameters — `backlog_for_team` takes
/// a `team_id`, not a `sprint_id`.
///
/// "Open" means `status = 'open'` literally, not "not done" —
/// the narrower reading of RFC 001's "top-level open issues"; an
/// `in_progress` issue is already committed to work in a way a
/// backlog candidate isn't.
///
/// **Returned in display order**: project name, then severity
/// (urgent, high, medium, low), then newest first. The severity term
/// is applied here in Rust, not in the SQL -- see the comments at the
/// `ORDER BY` and on `Priority::severity_rank` for why.
///
/// Filters are applied in SQL via the `(?n IS NULL OR col = ?n)`
/// idiom already used by `user_capacities`'s period-overlap
/// queries, rather than fetched-then-filtered in Rust — the
/// dataset is team-scoped either way, so this is a style choice,
/// not a performance one.
pub async fn backlog_for_team(
    pool: &Pool,
    team_id: &str,
    filter: BacklogFilter,
) -> StorageResult<Vec<BacklogRow>> {
    let rows = sqlx::query_as::<_, BacklogIssueRow>(
        r#"
        SELECT i.id, i.project_id, i.author_id, i.title, i.description,
               i.status, i.priority, i.effort, i.assignee_id,
               i.parent_issue_id, i.planned_start_at, i.planned_end_at,
               i.created_at, i.updated_at,
               p.name AS project_name
        FROM issues i
        JOIN projects p ON p.id = i.project_id
        WHERE p.team_id = ?1
          AND i.parent_issue_id IS NULL
          AND i.status = 'open'
          AND NOT EXISTS (
              SELECT 1 FROM sprint_issues si
              JOIN sprints s ON s.id = si.sprint_id
              WHERE si.issue_id = i.id AND s.status = 'active'
          )
          AND (?2 IS NULL OR i.project_id = ?2)
          AND (?3 IS NULL OR i.priority = ?3)
          AND (
              ?4 IS NULL
              OR (?4 = 'unassigned' AND i.assignee_id IS NULL)
              OR i.assignee_id = ?4
          )
        -- Priority is deliberately absent from this ORDER BY: it is a TEXT
        -- column, so `priority DESC` sorts alphabetically (urgent, medium,
        -- low, high -- `high` last). Severity is applied in Rust below with
        -- `Priority::severity_rank`. Do not put it back here.
        ORDER BY p.name ASC, i.created_at DESC, i.rowid DESC
        "#,
    )
    .bind(team_id)
    .bind(filter.project_id.as_deref())
    .bind(filter.priority.map(|p| p.as_str()))
    .bind(filter.assignee_id.as_deref())
    .fetch_all(pool)
    .await?;
    let mut backlog = rows
        .into_iter()
        .map(BacklogIssueRow::into_backlog_row)
        .collect::<StorageResult<Vec<_>>>()?;
    // `PLAN-003`: the query orders by project name, then newest first;
    // this puts severity between them. **`sort_by` is stable, and that is
    // load-bearing**: it is what keeps `created_at DESC` within each
    // (project, priority) group, so only the priority term's behaviour
    // changes. `project_name` compares as byte order, which is what
    // SQLite's default BINARY collation did for `p.name` (no column in
    // this schema declares another), so this cannot reorder projects.
    backlog.sort_by(|a, b| {
        a.project_name.cmp(&b.project_name).then_with(|| {
            a.issue
                .priority
                .severity_rank()
                .cmp(&b.issue.priority.severity_rank())
        })
    });
    Ok(backlog)
}

// ──────────────────────────────────────────────────────────────
// Calendar sprint band (`CAL-002` / RFC 002)
// ──────────────────────────────────────────────────────────────

/// Active sprints overlapping `[from, to]` in the given project's
/// team. Project axis only (`CAL-002` §4) — the personal axis
/// intentionally never renders a sprint band, since it can span
/// multiple teams' sprints and surfacing them all would turn the
/// page into a sprint dashboard (RFC 002 must-have 7).
///
/// Only `active` sprints — a planned or completed sprint's band on a
/// calendar would assert something about time that is not true (a
/// planned sprint hasn't started; a completed one is over).
///
/// Reuses [`active_for_team`] rather than a new query: this
/// project's business rule is at most one active sprint per team
/// (enforced at `start`'s call site), so "active sprints overlapping
/// a window" is "the team's one active sprint, if it overlaps" —
/// `Vec` in the return type matches RFC 002's own signature, not a
/// claim that more than one is possible.
pub async fn active_sprints_overlapping(
    pool: &Pool,
    project_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> StorageResult<Vec<Sprint>> {
    let team_id: Option<Option<String>> =
        sqlx::query_scalar(r#"SELECT team_id FROM projects WHERE id = ?1"#)
            .bind(project_id)
            .fetch_optional(pool)
            .await?;
    // Outer `Option`: does the project row exist. Inner `Option`:
    // is `team_id` NULL (a personal project). `.flatten()` collapses
    // "no row" and "row, but no team" into the same outcome — no
    // team, no sprint, nothing to overlap.
    let Some(team_id) = team_id.flatten() else {
        return Ok(Vec::new());
    };
    let Some(sprint) = active_for_team(pool, &team_id).await? else {
        return Ok(Vec::new());
    };
    // sprints.starts_on/ends_on are DATE; widen to start-of-day and
    // end-of-day UTC for the overlap check, per RFC 002 §Design.
    let sprint_start = sprint
        .starts_on
        .and_hms_opt(0, 0, 0)
        .expect("00:00:00 is always valid")
        .and_utc();
    let sprint_end = sprint
        .ends_on
        .and_hms_opt(23, 59, 59)
        .expect("23:59:59 is always valid")
        .and_utc();
    if sprint_start <= to && sprint_end >= from {
        Ok(vec![sprint])
    } else {
        Ok(Vec::new())
    }
}
