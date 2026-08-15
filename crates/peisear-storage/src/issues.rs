//! Issue table queries.
//!
//! All mutations are scoped against `(project_id, owner_id)` to enforce
//! access control at the query level as a defense-in-depth measure on
//! top of the handler checks.

use chrono::{DateTime, Utc};
use peisear_core::{Issue, IssueStatus, Priority};
use sqlx::FromRow;

use crate::{Pool, StorageError, StorageResult, issue_events};

/// Raw row as returned by sqlx. Kept private — the public API returns
/// [`peisear_core::Issue`] with parsed enum fields.
#[derive(FromRow)]
struct IssueRow {
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
    /// Sub-issue parent reference (Phase C PR1, migration
    /// 0015). NULL for top-level issues; FK to `issues(id)`
    /// for sub-issues. The 1-level constraint is enforced by
    /// trigger.
    parent_issue_id: Option<String>,
    planned_start_at: Option<DateTime<Utc>>,
    planned_end_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl IssueRow {
    fn into_issue(self) -> StorageResult<Issue> {
        let status = IssueStatus::parse(&self.status)
            .ok_or_else(|| StorageError::InvalidData(format!("status={}", self.status)))?;
        let priority = Priority::parse(&self.priority)
            .ok_or_else(|| StorageError::InvalidData(format!("priority={}", self.priority)))?;
        Ok(Issue {
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
        })
    }
}

/// List top-level issues in a project. Per
/// peisear-feature-spec-v2.1 §8.5, the project board / list /
/// kanban surface only the top-level rows; sub-issues are
/// rendered inline in their parent's detail page. This
/// function applies that filter — callers that want every row
/// (e.g. analytics, project_workload, health computation)
/// should use [`list_all_in_project`].
pub async fn list_in_project(pool: &Pool, project_id: &str) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE project_id = ?1
          AND parent_issue_id IS NULL
        ORDER BY status ASC, position ASC, created_at DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}

/// List **every** issue in a project, top-level and sub-issue
/// alike. Use for analytics surfaces (workload, health,
/// project_health) where excluding sub-issues would skew the
/// numbers — sub-issues represent real assigned work even if
/// they're not shown on the kanban.
pub async fn list_all_in_project(pool: &Pool, project_id: &str) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE project_id = ?1
        ORDER BY status ASC, position ASC, created_at DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}

/// List the sub-issues of a given parent. Returns them in
/// creation order (oldest first) — this is the order they
/// were defined in, which usually reads as "natural reading
/// order" in the parent's detail panel.
///
/// Empty vec if the parent has no children. The parent itself
/// is not included.
pub async fn list_sub_issues_of(pool: &Pool, parent_issue_id: &str) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE parent_issue_id = ?1
        ORDER BY created_at ASC
        "#,
    )
    .bind(parent_issue_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}

pub async fn find(pool: &Pool, issue_id: &str, project_id: &str) -> StorageResult<Issue> {
    let row = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(issue_id)
    .bind(project_id)
    .fetch_optional(pool)
    .await?;
    row.ok_or(StorageError::NotFound)
        .and_then(IssueRow::into_issue)
}

/// The mutable content fields of an issue — everything a create
/// or edit form supplies, as opposed to the id/project/actor
/// coordinates that route the write. Shared between [`insert`]
/// and [`update`] so both take the same shape.
pub struct IssueFields<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub status: IssueStatus,
    pub priority: Priority,
    pub effort: Option<i64>,
    pub assignee_id: Option<&'a str>,
    /// `CAL-001` / RFC 002. Only the issue edit form's inputs set
    /// these to `Some` — the create form does not carry them
    /// (handoff §1: "the issue edit form's date inputs", not the
    /// create form's), so every `create`/`create_sub_issue` call
    /// site passes `None` for both.
    pub planned_start_at: Option<DateTime<Utc>>,
    pub planned_end_at: Option<DateTime<Utc>>,
}

pub async fn insert(
    pool: &Pool,
    id: &str,
    project_id: &str,
    author_id: &str,
    fields: IssueFields<'_>,
) -> StorageResult<()> {
    // Open a transaction so the issue insert and the 'created'
    // event row land atomically. If anything fails, neither
    // commits.
    let mut tx = pool.begin().await?;

    let next_pos: i64 = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COALESCE(MAX(position), 0) + 1
        FROM issues
        WHERE project_id = ?1 AND status = ?2
        "#,
    )
    .bind(project_id)
    .bind(fields.status.as_str())
    .fetch_one(&mut *tx)
    .await?;

    let res = sqlx::query(
        r#"
        INSERT INTO issues
            (id, project_id, author_id, title, description, status, priority,
             position, effort, assignee_id, planned_start_at, planned_end_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(author_id)
    .bind(fields.title)
    .bind(fields.description)
    .bind(fields.status.as_str())
    .bind(fields.priority.as_str())
    .bind(next_pos)
    .bind(fields.effort)
    .bind(fields.assignee_id)
    .bind(fields.planned_start_at)
    .bind(fields.planned_end_at)
    .execute(&mut *tx)
    .await;

    // Migration 0016's insert trigger can now fire here (it could
    // not before — nothing on this path set planned dates until
    // CAL-001). Translate the same way insert_sub_issue already
    // does, per DEC-011.
    if let Err(e) = res {
        return Err(translate_trigger_error(e));
    }

    // The 'created' event records the initial state. Storing
    // status as `new_value` lets later analysis answer "what
    // status did this issue start in?" without a join.
    issue_events::insert_event(
        &mut tx,
        id,
        project_id,
        Some(author_id),
        issue_events::kind::CREATED,
        None,
        Some(fields.status.as_str()),
    )
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Insert a sub-issue under an existing parent. Same shape as
/// `insert` but with an additional `parent_issue_id` argument.
///
/// The 1-level constraint (a sub-issue can't have its own
/// sub-issue) and the same-project constraint are both
/// enforced by triggers in migration 0015 — this function
/// surfaces those as `StorageError::Validation` rather than
/// raw SQL errors so the caller can render them to the user.
///
/// Sub-issues use a separate position counter from their
/// parent's status group: they're listed under the parent in
/// creation order (see `list_sub_issues_of`), not interspersed
/// in the project's main kanban. We therefore default position
/// to 0 — it's not meaningful for sub-issues, but the column
/// is NOT NULL.
#[allow(clippy::too_many_arguments)]
pub async fn insert_sub_issue(
    pool: &Pool,
    id: &str,
    project_id: &str,
    parent_issue_id: &str,
    author_id: &str,
    title: &str,
    description: &str,
    status: IssueStatus,
    priority: Priority,
    effort: Option<i64>,
    assignee_id: Option<&str>,
) -> StorageResult<()> {
    let mut tx = pool.begin().await?;

    let res = sqlx::query(
        r#"
        INSERT INTO issues
            (id, project_id, author_id, title, description, status, priority,
             position, effort, assignee_id, parent_issue_id)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8, ?9, ?10)
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(author_id)
    .bind(title)
    .bind(description)
    .bind(status.as_str())
    .bind(priority.as_str())
    .bind(effort)
    .bind(assignee_id)
    .bind(parent_issue_id)
    .execute(&mut *tx)
    .await;

    // Trigger violations show up as raw sqlx errors with
    // SQLite's RAISE message inside. Translate them to
    // Validation so the handler can render them as 400 instead
    // of 500.
    if let Err(e) = res {
        return Err(translate_trigger_error(e));
    }

    issue_events::insert_event(
        &mut tx,
        id,
        project_id,
        Some(author_id),
        issue_events::kind::CREATED,
        None,
        Some(status.as_str()),
    )
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Promote a sub-issue back to top-level (sets
/// `parent_issue_id = NULL`). Per spec §8.3 this is a single
/// column update — the assignee, status, sprint membership
/// etc. all stay as they were.
///
/// No-op if the issue is already top-level.
pub async fn promote_to_top_level(pool: &Pool, id: &str, project_id: &str) -> StorageResult<()> {
    sqlx::query(
        r#"
        UPDATE issues
        SET parent_issue_id = NULL
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Demote a top-level issue to be a sub-issue of `new_parent_id`.
/// Validates that the move doesn't create a 2-level chain
/// (existing children must be promoted first) — that check is
/// done in the trigger.
pub async fn demote_to_sub_issue(
    pool: &Pool,
    id: &str,
    project_id: &str,
    new_parent_id: &str,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE issues
        SET parent_issue_id = ?3
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(new_parent_id)
    .execute(pool)
    .await;

    if let Err(e) = res {
        return Err(translate_trigger_error(e));
    }
    Ok(())
}

/// Translate a raw `sqlx::Error` into the closest `StorageError`
/// for sub-issue trigger violations. We match on the error's
/// message text (SQLite's RAISE produces a specific string)
/// because SQLite doesn't expose machine-readable codes for
/// trigger-RAISE'd errors.
///
/// This is brittle to RAISE message wording — keep the test
/// for it close (storage unit tests assert on the exact
/// translated `Validation` payload), and update both together
/// if the migration ever changes the strings.
fn translate_trigger_error(e: sqlx::Error) -> StorageError {
    let msg = e.to_string();
    // Known trigger messages from migrations 0015 and 0016, paired
    // with the MessageKey that carries the same text (I18N-006 §5 —
    // the needle text itself is unchanged, only the returned type
    // is).
    let known = [
        (
            "sub-issue cannot have a sub-issue",
            peisear_i18n::MessageKey::SubIssueCannotHaveSubIssueMessage,
        ),
        (
            "sub-issue must share project with its parent",
            peisear_i18n::MessageKey::SubIssueMustShareProjectMessage,
        ),
        (
            "an issue cannot be its own parent",
            peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
        ),
        (
            "cannot demote an issue that has its own sub-issues",
            peisear_i18n::MessageKey::CannotDemoteIssueWithSubIssuesMessage,
        ),
        (
            "Planned end date must be on or after planned start date.",
            peisear_i18n::MessageKey::IssuePlannedEndBeforeStartMessage,
        ),
    ];
    for (needle, key) in known.into_iter() {
        if msg.contains(needle) {
            return StorageError::Validation(key);
        }
    }
    e.into()
}

pub async fn update(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
    fields: IssueFields<'_>,
) -> StorageResult<()> {
    let mut tx = pool.begin().await?;

    // Read the previous values inside the same transaction so we
    // can diff and emit one event per changed field. SELECT
    // first, UPDATE second, COMMIT once — the previous read is
    // self-consistent with the write.
    let prev: Option<(String, Option<i64>, Option<String>)> = sqlx::query_as(
        r#"
        SELECT status, effort, assignee_id
        FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some((prev_status, prev_effort, prev_assignee)) = prev else {
        return Err(StorageError::NotFound);
    };

    // CAL-001 §2.4: planned_start_at/planned_end_at join this
    // existing UPDATE's SET clause rather than getting a statement
    // of their own. `issues` has no `updated_at` trigger (DEC-013's
    // machinery covers sprints/teams/team_memberships/
    // user_capacities, not this table) — updated_at only moves
    // because this statement's own SET clause sets it. A separate
    // UPDATE for the two date columns would leave updated_at
    // unmoved, and a concurrent plan-date edit would silently win
    // with no error and no symptom (NFR-CONC-004).
    let res = sqlx::query(
        r#"
        UPDATE issues
        SET title = ?3, description = ?4, status = ?5, priority = ?6,
            effort = ?7,
            assignee_id = ?8,
            planned_start_at = ?9,
            planned_end_at = ?10,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(fields.title)
    .bind(fields.description)
    .bind(fields.status.as_str())
    .bind(fields.priority.as_str())
    .bind(fields.effort)
    .bind(fields.assignee_id)
    .bind(fields.planned_start_at)
    .bind(fields.planned_end_at)
    .execute(&mut *tx)
    .await;

    let res = match res {
        Ok(r) => r,
        Err(e) => return Err(translate_trigger_error(e)),
    };
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }

    // One event per field that actually moved. We want
    // status_changed events to be the single source of truth for
    // the in-progress dwell-time analysis, so emitting them only
    // on real status moves is important.
    if prev_status != fields.status.as_str() {
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::STATUS_CHANGED,
            Some(&prev_status),
            Some(fields.status.as_str()),
        )
        .await?;
    }

    if prev_effort != fields.effort {
        let prev_str = prev_effort.map(|n| n.to_string());
        let new_str = fields.effort.map(|n| n.to_string());
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::EFFORT_CHANGED,
            prev_str.as_deref(),
            new_str.as_deref(),
        )
        .await?;
    }

    let new_assignee_owned = fields.assignee_id.map(|s| s.to_string());
    if prev_assignee != new_assignee_owned {
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::ASSIGNEE_CHANGED,
            prev_assignee.as_deref(),
            fields.assignee_id,
        )
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn update_status(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
    status: IssueStatus,
) -> StorageResult<()> {
    let mut tx = pool.begin().await?;

    let prev_status: Option<String> = sqlx::query_scalar(
        r#"
        SELECT status FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(prev_status) = prev_status else {
        return Err(StorageError::NotFound);
    };

    let res = sqlx::query(
        r#"
        UPDATE issues
        SET status = ?3, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(status.as_str())
    .execute(&mut *tx)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }

    if prev_status != status.as_str() {
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::STATUS_CHANGED,
            Some(&prev_status),
            Some(status.as_str()),
        )
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn delete(pool: &Pool, id: &str, project_id: &str, actor_id: &str) -> StorageResult<()> {
    let mut tx = pool.begin().await?;

    // Read the previous status / current state so we can record
    // it in the deletion event. After the DELETE, the cascade
    // SET NULL on issue_id loses the link, but the project_id and
    // event metadata still tell the story.
    let prev_status: Option<String> =
        sqlx::query_scalar(r#"SELECT status FROM issues WHERE id = ?1 AND project_id = ?2"#)
            .bind(id)
            .bind(project_id)
            .fetch_optional(&mut *tx)
            .await?;

    if prev_status.is_none() {
        return Err(StorageError::NotFound);
    }

    // Write the deletion event BEFORE the actual delete, while
    // the issue_id reference is still valid. The CASCADE then
    // sets issue_id to NULL on this freshly-written row, leaving
    // it standalone in the log. previous_value carries the
    // last-known status as a small affordance.
    issue_events::insert_event(
        &mut tx,
        id,
        project_id,
        Some(actor_id),
        issue_events::kind::DELETED,
        prev_status.as_deref(),
        None,
    )
    .await?;

    let res = sqlx::query(
        r#"
        DELETE FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .execute(&mut *tx)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }

    tx.commit().await?;
    Ok(())
}

/// The assignee-candidate set for a project, as a `WITH` CTE
/// fragment: the project's owner, plus any `admin`/`member` (never
/// `viewer`) of the project's team, if it has one.
///
/// `TEAM-001` / RFC 009 §D1: `list_assignee_candidates` and
/// `project_workload` used to be two independently-written queries
/// that both joined `projects p ON p.owner_id = u.id` — a join that
/// can only ever produce the owner, regardless of team membership.
/// They diverged from the same wrong text because they were written
/// twice; the fix is one definition both derive from, not two
/// corrected copies.
///
/// **Mechanism chosen**: a `WITH` CTE, embedded as a shared Rust
/// `&str` constant in each query, rather than a SQL view or the
/// `query_as!` compile-time macro. A view is a migration and a
/// bigger commitment than this fix needs; `peisear-storage` has no
/// `query_as!`/`query!` call anywhere (confirmed by grep) and no
/// `DATABASE_URL`/offline-cache machinery to support one, so
/// introducing the macro here would be a first for the crate, not a
/// fit with this fix's scope. The runtime-checked `query_as::<_,
/// T>()` form every other query in this file uses is preserved.
///
/// `?1` is the project id, bound once and reused — this project's
/// existing convention for repeated positional SQLite params (see
/// `projects::list_for_user`'s `?1` reused three times with one
/// `.bind()`).
///
/// `viewer` is excluded in the `LEFT JOIN`'s `ON` clause, not a
/// `WHERE` filter: a `WHERE tm.role IN (...)` would still be
/// correct here, since it only ever narrows which `tm` rows can
/// satisfy `u.id = tm.user_id`, but filtering in the join condition
/// makes the exclusion visible at the point the membership rows are
/// selected, alongside the team-id join, rather than downstream of
/// it — `0011_teams.sql`'s own comment: a viewer is read-only, no
/// assignment. RFC 009 §D1's own sample SQL has no role filter at
/// all (it would incorrectly admit viewers); this fixes that gap
/// too, per handoff §4's explicit requirement that only `admin`/
/// `member` are candidates.
///
/// The `LEFT JOIN` cross-matches every user against every
/// membership row for the project's team (it is not correlated to
/// `u` in the join condition), so a user who is both the owner and
/// a team member can produce more than one matching row before
/// `GROUP BY`/`DISTINCT` collapses it — same shape RFC 009 §D1's own
/// sample query has, and why it (and this one) end in `GROUP BY`/
/// `SELECT DISTINCT`.
const CANDIDATE_SET_CTE: &str = r#"
    candidates AS (
        SELECT DISTINCT u.id, u.display_name
        FROM users u
        JOIN projects p ON p.id = ?1
        LEFT JOIN team_memberships tm
               ON tm.team_id = p.team_id AND tm.role IN ('admin', 'member')
        WHERE u.id = p.owner_id OR u.id = tm.user_id
    )
"#;

/// List the users who are valid assignee candidates for issues in a
/// given project: the project's owner, plus any `admin`/`member` of
/// the project's team. A personal project (`team_id IS NULL`) yields
/// exactly the owner, since the `LEFT JOIN` in [`CANDIDATE_SET_CTE`]
/// then contributes nothing.
pub async fn list_assignee_candidates(
    pool: &Pool,
    project_id: &str,
) -> StorageResult<Vec<peisear_core::AssigneeOption>> {
    sqlx::query_as::<_, (String, String)>(&format!(
        r#"
        WITH {CANDIDATE_SET_CTE}
        SELECT id, display_name FROM candidates
        ORDER BY display_name ASC
        "#,
    ))
    .bind(project_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, display_name)| Ok(peisear_core::AssigneeOption { id, display_name }))
    .collect()
}

/// Per-user workload report for a project.
///
/// Returns one [`UserLoad`] per assignee candidate ([`CANDIDATE_SET_CTE`]),
/// **plus** any user holding an in-flight issue in the project even if
/// they are no longer a candidate (RFC 009 §D3, settled: a user
/// removed from a team keeps issues already assigned to them, so the
/// candidate set is a subset of the workload set, not equal to it —
/// the form describes policy going forward, the report describes
/// reality). Users with no in-flight issues still appear in the
/// result with zero counts so the UI can show their (empty) chip.
///
/// Today this is a project-level report. The query intentionally
/// keeps a `project_id` filter even though the user-level capacity
/// is global — this means the chip strip on a project page reflects
/// "what this team has on its plate from this project". The full
/// global view (across all projects a user is on) is a future
/// addition that takes the same shape with `project_id = NULL`.
///
/// When period support lands, this query gains a period filter on
/// the issue side without changing the [`UserLoad`] result shape.
pub async fn project_workload(
    pool: &Pool,
    project_id: &str,
) -> StorageResult<Vec<peisear_core::UserLoad>> {
    // 0.12.0: capacity comes from `user_capacities`, resolved as
    // "the row whose period covers today, most recently created".
    // The correlated sub-select keeps the per-user join tractable
    // even with the period filter.
    sqlx::query_as::<_, (String, String, Option<i64>, Option<i64>, i64)>(&format!(
        r#"
        WITH {CANDIDATE_SET_CTE}
        SELECT
            u.id,
            u.display_name,
            (SELECT uc.points
             FROM user_capacities uc
             WHERE uc.user_id = u.id
               AND (uc.period_start IS NULL OR uc.period_start <= date('now'))
               AND (uc.period_end   IS NULL OR uc.period_end   >= date('now'))
             ORDER BY uc.created_at DESC
             LIMIT 1
            ) AS capacity_points,
            COALESCE(SUM(CASE
                WHEN i.status IN ('open', 'in_progress') THEN i.effort
                ELSE NULL
            END), 0) AS in_flight_points,
            COALESCE(SUM(CASE
                WHEN i.status IN ('open', 'in_progress') THEN 1
                ELSE 0
            END), 0) AS in_flight_issues
        FROM users u
        LEFT JOIN issues i ON i.assignee_id = u.id AND i.project_id = ?1
        WHERE u.id IN (SELECT id FROM candidates)
           OR EXISTS (
                SELECT 1 FROM issues i2
                WHERE i2.assignee_id = u.id
                  AND i2.project_id = ?1
                  AND i2.status IN ('open', 'in_progress')
              )
        GROUP BY u.id, u.display_name
        ORDER BY u.display_name ASC
        "#,
    ))
    .bind(project_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(
        |(user_id, display_name, capacity_points, in_flight_points, in_flight_issues)| {
            Ok(peisear_core::UserLoad {
                user_id,
                display_name,
                capacity_points,
                in_flight_points: in_flight_points.unwrap_or(0),
                in_flight_issues,
            })
        },
    )
    .collect()
}

// ──────────────────────────────────────────────────────────────
// Calendar window queries (`CAL-001` / RFC 002)
// ──────────────────────────────────────────────────────────────

/// The overlap predicate RFC 002 §Design specifies, as a `WHERE`
/// fragment shared by [`planned_for_user`] and [`planned_for_project`]
/// — one definition, not two independently-written copies of the
/// same three-way NULL-handling logic. `?2`/`?3` (`from`/`to`) are
/// hardcoded because both callers bind their scope value first (`?1`
/// — assignee or project) and `from`/`to` second and third, in that
/// order, so the positions are identical in both queries; this is
/// the same `?N`-reuse-via-shared-const-`&str`-embedded-with-`format!`
/// mechanism `TEAM-001`'s `CANDIDATE_SET_CTE` established.
///
/// A `NULL` `planned_end_at` is treated as a half-hour anchor at
/// `planned_start_at` (must-have 5) — the second arm of the `OR`
/// keeps such a row in the overlap set using `planned_start_at`
/// itself as the stand-in end bound, so it appears whenever the
/// window covers its start instant.
const PLANNED_WINDOW_OVERLAP_PREDICATE: &str = r#"
    planned_start_at IS NOT NULL
    AND (
        (planned_end_at IS NOT NULL AND planned_end_at >= ?2)
        OR (planned_end_at IS NULL AND planned_start_at >= ?2)
    )
    AND planned_start_at <= ?3
"#;

/// Issues planned to overlap `[from, to]` for the given assignee.
/// Personal axis (`/today/calendar`, CAL-002) — self-only by
/// construction (`§11.5`): the caller supplies `user_id`, there is no
/// path to another user's planned issues through this function.
///
/// Top-level only — `CAL-002` §5 test 10 ("sub-issues appear on
/// neither axis"). This filter was missing when `CAL-001` shipped
/// this function: RFC 002 must-have 6 states "top-level" explicitly
/// for the project axis but only implies it for the personal one
/// ("sub-issues follow parent; they don't appear separately on the
/// calendar" reads as a general rule, not a project-axis-only one),
/// and `planned_for_project` already had the filter while this one
/// didn't. Found while implementing CAL-002's test 10, fixed here
/// rather than worked around at the render layer.
pub async fn planned_for_user(
    pool: &Pool,
    user_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(&format!(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE assignee_id = ?1
          AND parent_issue_id IS NULL
          AND {PLANNED_WINDOW_OVERLAP_PREDICATE}
        ORDER BY planned_start_at ASC
        "#,
    ))
    .bind(user_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}

/// Issues planned to overlap `[from, to]` in the given project.
/// Top-level only — sub-issues inherit their parent's position on
/// the calendar by way of the parent appearing in the result (RFC
/// 002 §Design), matching [`list_in_project`]'s existing
/// top-level-only filter.
pub async fn planned_for_project(
    pool: &Pool,
    project_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(&format!(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE project_id = ?1
          AND parent_issue_id IS NULL
          AND {PLANNED_WINDOW_OVERLAP_PREDICATE}
        ORDER BY planned_start_at ASC
        "#,
    ))
    .bind(project_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}
