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
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

/// List all issues in a project (for list view).
pub async fn list_in_project(pool: &Pool, project_id: &str) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, effort, assignee_id, created_at, updated_at
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

pub async fn find(pool: &Pool, issue_id: &str, project_id: &str) -> StorageResult<Issue> {
    let row = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, effort, assignee_id, created_at, updated_at
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

pub async fn insert(
    pool: &Pool,
    id: &str,
    project_id: &str,
    author_id: &str,
    title: &str,
    description: &str,
    status: IssueStatus,
    priority: Priority,
    effort: Option<i64>,
    assignee_id: Option<&str>,
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
    .bind(status.as_str())
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO issues
            (id, project_id, author_id, title, description, status, priority,
             position, effort, assignee_id)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(author_id)
    .bind(title)
    .bind(description)
    .bind(status.as_str())
    .bind(priority.as_str())
    .bind(next_pos)
    .bind(effort)
    .bind(assignee_id)
    .execute(&mut *tx)
    .await?;

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
        Some(status.as_str()),
    )
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn update(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
    title: &str,
    description: &str,
    status: IssueStatus,
    priority: Priority,
    effort: Option<i64>,
    assignee_id: Option<&str>,
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

    let res = sqlx::query(
        r#"
        UPDATE issues
        SET title = ?3, description = ?4, status = ?5, priority = ?6,
            effort = ?7,
            assignee_id = ?8,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(title)
    .bind(description)
    .bind(status.as_str())
    .bind(priority.as_str())
    .bind(effort)
    .bind(assignee_id)
    .execute(&mut *tx)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }

    // One event per field that actually moved. We want
    // status_changed events to be the single source of truth for
    // the in-progress dwell-time analysis, so emitting them only
    // on real status moves is important.
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

    if prev_effort != effort {
        let prev_str = prev_effort.map(|n| n.to_string());
        let new_str = effort.map(|n| n.to_string());
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

    let new_assignee_owned = assignee_id.map(|s| s.to_string());
    if prev_assignee != new_assignee_owned {
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::ASSIGNEE_CHANGED,
            prev_assignee.as_deref(),
            assignee_id,
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

pub async fn delete(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
) -> StorageResult<()> {
    let mut tx = pool.begin().await?;

    // Read the previous status / current state so we can record
    // it in the deletion event. After the DELETE, the cascade
    // SET NULL on issue_id loses the link, but the project_id and
    // event metadata still tell the story.
    let prev_status: Option<String> = sqlx::query_scalar(
        r#"SELECT status FROM issues WHERE id = ?1 AND project_id = ?2"#,
    )
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

/// List the users who are valid assignee candidates for issues in a
/// given project.
///
/// Today's single-tenant model returns only the project owner, but
/// callers should not assume that — when team / organisation support
/// lands (Medium-term roadmap), this function will return all members
/// of the project's team. Keeping the surface as a query function
/// rather than inlining `vec![owner]` at call sites means UI code does
/// not change when that happens.
pub async fn list_assignee_candidates(
    pool: &Pool,
    project_id: &str,
) -> StorageResult<Vec<peisear_core::AssigneeOption>> {
    sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT u.id, u.display_name
        FROM users u
        JOIN projects p ON p.owner_id = u.id
        WHERE p.id = ?1
        ORDER BY u.display_name ASC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, display_name)| {
        Ok(peisear_core::AssigneeOption { id, display_name })
    })
    .collect()
}

/// Per-user workload report for a project.
///
/// Returns one [`UserLoad`] per assignee candidate, aggregating the
/// effort and issue count across the user's currently in-flight
/// assignments. Users with no in-flight issues still appear in the
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
    sqlx::query_as::<_, (String, String, Option<i64>, Option<i64>, i64)>(
        r#"
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
        JOIN projects p ON p.owner_id = u.id
        LEFT JOIN issues i ON i.assignee_id = u.id AND i.project_id = p.id
        WHERE p.id = ?1
        GROUP BY u.id, u.display_name
        ORDER BY u.display_name ASC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(user_id, display_name, capacity_points, in_flight_points, in_flight_issues)| {
        Ok(peisear_core::UserLoad {
            user_id,
            display_name,
            capacity_points,
            in_flight_points: in_flight_points.unwrap_or(0),
            in_flight_issues,
        })
    })
    .collect()
}
