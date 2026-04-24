//! Project table queries.

use chrono::{DateTime, Utc};
use peisear_core::Project;
use sqlx::FromRow;

use crate::{Pool, StorageError, StorageResult};

#[derive(FromRow)]
struct ProjectRow {
    id: String,
    owner_id: String,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ProjectRow> for Project {
    fn from(r: ProjectRow) -> Self {
        Project {
            id: r.id,
            owner_id: r.owner_id,
            name: r.name,
            description: r.description,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

pub async fn list_for_user(pool: &Pool, user_id: &str) -> StorageResult<Vec<Project>> {
    let rows = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT id, owner_id, name, description, created_at, updated_at
        FROM projects
        WHERE owner_id = ?1
        ORDER BY updated_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn find_accessible(
    pool: &Pool,
    project_id: &str,
    user_id: &str,
) -> StorageResult<Project> {
    let row = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT id, owner_id, name, description, created_at, updated_at
        FROM projects
        WHERE id = ?1 AND owner_id = ?2
        "#,
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    row.map(Into::into).ok_or(StorageError::NotFound)
}

pub async fn insert(
    pool: &Pool,
    id: &str,
    owner_id: &str,
    name: &str,
    description: &str,
) -> StorageResult<()> {
    sqlx::query(
        r#"
        INSERT INTO projects (id, owner_id, name, description)
        VALUES (?1, ?2, ?3, ?4)
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .bind(name)
    .bind(description)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update(
    pool: &Pool,
    id: &str,
    owner_id: &str,
    name: &str,
    description: &str,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE projects
        SET name = ?3, description = ?4, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?1 AND owner_id = ?2
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .bind(name)
    .bind(description)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

pub async fn delete(pool: &Pool, id: &str, owner_id: &str) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        DELETE FROM projects
        WHERE id = ?1 AND owner_id = ?2
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}
