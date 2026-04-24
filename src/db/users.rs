//! User table queries.

use sqlx::SqlitePool;

use crate::{error::AppResult, models::User};

pub async fn find_by_email(pool: &SqlitePool, email: &str) -> AppResult<Option<User>> {
    let row = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, display_name, created_at
        FROM users
        WHERE email = ?1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> AppResult<Option<User>> {
    let row = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, display_name, created_at
        FROM users
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn insert(
    pool: &SqlitePool,
    id: &str,
    email: &str,
    password_hash: &str,
    display_name: &str,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password_hash, display_name)
        VALUES (?1, ?2, ?3, ?4)
        "#,
    )
    .bind(id)
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .execute(pool)
    .await?;
    Ok(())
}
