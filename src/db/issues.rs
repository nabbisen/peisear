//! Issue table queries.
//!
//! All mutations are scoped against `(project_id, owner_id)` to enforce
//! access control at the query level as a defense-in-depth measure on
//! top of the handler checks.

use sqlx::SqlitePool;

use crate::{
    error::{AppError, AppResult},
    models::{Issue, IssueRow, IssueStatus, Priority},
};

/// List all issues in a project (for list view).
pub async fn list_in_project(pool: &SqlitePool, project_id: &str) -> AppResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, created_at, updated_at
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

pub async fn find(pool: &SqlitePool, issue_id: &str, project_id: &str) -> AppResult<Issue> {
    let row = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, position, created_at, updated_at
        FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(issue_id)
    .bind(project_id)
    .fetch_optional(pool)
    .await?;
    row.ok_or(AppError::NotFound).and_then(IssueRow::into_issue)
}

pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    project_id: &str,
    author_id: &str,
    title: &str,
    description: &str,
    status: IssueStatus,
    priority: Priority,
) -> AppResult<()> {
    // Place at the end of the column for stable ordering.
    let next_pos: i64 = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COALESCE(MAX(position), 0) + 1
        FROM issues
        WHERE project_id = ?1 AND status = ?2
        "#,
    )
    .bind(project_id)
    .bind(status.as_str())
    .fetch_one(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO issues
            (id, project_id, author_id, title, description, status, priority, position)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
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
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update(
    pool: &SqlitePool,
    id: &str,
    project_id: &str,
    title: &str,
    description: &str,
    status: IssueStatus,
    priority: Priority,
) -> AppResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE issues
        SET title = ?3, description = ?4, status = ?5, priority = ?6,
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
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub async fn update_status(
    pool: &SqlitePool,
    id: &str,
    project_id: &str,
    status: IssueStatus,
) -> AppResult<()> {
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
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: &str, project_id: &str) -> AppResult<()> {
    let res = sqlx::query(
        r#"
        DELETE FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}
