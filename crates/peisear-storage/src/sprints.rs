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

use crate::{Pool, StorageError, StorageResult};

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

pub async fn find_by_id(pool: &Pool, id: &str) -> StorageResult<Option<Sprint>> {
    let row = sqlx::query_as::<_, SprintRow>(
        r#"
        SELECT id, team_id, name, goal, starts_on, ends_on,
               status, started_at, completed_at, created_at, updated_at
        FROM sprints
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
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
        ORDER BY starts_on DESC, created_at DESC
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
pub async fn active_for_team(pool: &Pool, team_id: &str) -> StorageResult<Option<Sprint>> {
    let row = sqlx::query_as::<_, SprintRow>(
        r#"
        SELECT id, team_id, name, goal, starts_on, ends_on,
               status, started_at, completed_at, created_at, updated_at
        FROM sprints
        WHERE team_id = ?1 AND status = 'active'
        ORDER BY started_at DESC
        LIMIT 1
        "#,
    )
    .bind(team_id)
    .fetch_optional(pool)
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
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(goal)
    .bind(starts_on)
    .bind(ends_on)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

pub async fn delete(pool: &Pool, id: &str) -> StorageResult<()> {
    let res = sqlx::query(r#"DELETE FROM sprints WHERE id = ?1"#)
        .bind(id)
        .execute(pool)
        .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// Transition `planned` → `active`. Refuses if another sprint
/// in the same team is already active (prevents two-active-
/// sprint ambiguity).
pub async fn start(pool: &Pool, sprint_id: &str) -> StorageResult<()> {
    let sprint = find_by_id(pool, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
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
    if let Some(other) = active_for_team(pool, &sprint.team_id).await? {
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
    .execute(pool)
    .await?;
    Ok(())
}

/// Transition `active` → `completed`. Sprint can only be
/// completed if currently active.
pub async fn complete(pool: &Pool, sprint_id: &str) -> StorageResult<()> {
    let sprint = find_by_id(pool, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
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
    .execute(pool)
    .await?;
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

/// Remove an issue from its sprint. Idempotent — removing an
/// issue that wasn't in a sprint is not an error (matches the
/// natural "make sure it isn't" mental model).
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
/// status. Used by the sprint detail page.
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
    let rows = sqlx::query_as(
        r#"
        SELECT i.id, i.project_id, i.title, i.effort, i.status
        FROM sprint_issues si
        JOIN issues i ON i.id = si.issue_id
        WHERE si.sprint_id = ?1
          AND i.parent_issue_id IS NULL
        ORDER BY i.status ASC, si.assigned_at ASC
        "#,
    )
    .bind(sprint_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Compute the live summary numbers (committed/completed/
/// carried-over) for one sprint.
pub async fn summary(pool: &Pool, sprint_id: &str) -> StorageResult<SprintSummary> {
    // Use COALESCE because SUM on an empty set returns NULL.
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
            SUM(CASE WHEN i.status = 'done' THEN 1 ELSE 0 END)
                AS completed_count
        FROM sprint_issues si
        JOIN issues i ON i.id = si.issue_id
        WHERE si.sprint_id = ?1
        "#,
    )
    .bind(sprint_id)
    .fetch_one(pool)
    .await?;

    let (committed_points, completed_points, committed_count, completed_count) = row;

    // Carried-over is meaningful only on completed sprints.
    // For active/planned, it's 0 by convention.
    let sprint = find_by_id(pool, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;
    let (carried_over_points, carried_over_count) = match sprint.status {
        SprintStatus::Completed => {
            // Carried-over = committed − completed at the time
            // of completion. Today's view is the same as
            // "current committed − current completed" since
            // completion freezes status semantically (the
            // sprint is done; remaining issues will move to
            // another sprint via `move_issue_to_sprint`, but
            // the summary captures the moment).
            let cp = committed_points - completed_points;
            let cc = committed_count - completed_count;
            (cp.max(0), cc.max(0))
        }
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

/// Compute the burndown timeline for a sprint.
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
pub async fn burndown(pool: &Pool, sprint_id: &str) -> StorageResult<Vec<BurndownPoint>> {
    let sprint = find_by_id(pool, sprint_id)
        .await?
        .ok_or(StorageError::NotFound)?;

    let issues: Vec<BurndownIssueRow> = sqlx::query_as(
        r#"
            SELECT i.id, i.effort, i.status, si.assigned_at, i.updated_at
            FROM sprint_issues si
            JOIN issues i ON i.id = si.issue_id
            WHERE si.sprint_id = ?1
            ORDER BY si.assigned_at ASC
            "#,
    )
    .bind(sprint_id)
    .fetch_all(pool)
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
    let done_events: Vec<(String, DateTime<Utc>)> = q.fetch_all(pool).await?;
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
            ORDER BY completed_at DESC
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
    position: i64,
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
                position: self.position,
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
               i.status, i.priority, i.position, i.effort, i.assignee_id,
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
        ORDER BY p.name ASC, i.priority DESC, i.created_at DESC
        "#,
    )
    .bind(team_id)
    .bind(filter.project_id.as_deref())
    .bind(filter.priority.map(|p| p.as_str()))
    .bind(filter.assignee_id.as_deref())
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(BacklogIssueRow::into_backlog_row)
        .collect()
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
