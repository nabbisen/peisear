//! Project table queries.

use sqlx::SqlitePool;

use crate::{
    error::{AppError, AppResult},
    models::Project,
};

pub async fn list_for_user(pool: &SqlitePool, user_id: &str) -> AppResult<Vec<Project>> {
    let rows = sqlx::query_as::<_, Project>(
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
    Ok(rows)
}

pub async fn find_accessible(
    pool: &SqlitePool,
    project_id: &str,
    user_id: &str,
) -> AppResult<Project> {
    let row = sqlx::query_as::<_, Project>(
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
    row.ok_or(AppError::NotFound)
}

pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    owner_id: &str,
    name: &str,
    description: &str,
) -> AppResult<()> {
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
    pool: &SqlitePool,
    id: &str,
    owner_id: &str,
    name: &str,
    description: &str,
) -> AppResult<()> {
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
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: &str, owner_id: &str) -> AppResult<()> {
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
        return Err(AppError::NotFound);
    }
    Ok(())
}
