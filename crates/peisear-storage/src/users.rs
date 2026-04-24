//! User table queries.

use chrono::{DateTime, Utc};
use peisear_core::User;
use sqlx::FromRow;

use crate::{Pool, StorageResult};

#[derive(FromRow)]
struct UserRow {
    id: String,
    email: String,
    password_hash: String,
    display_name: String,
    created_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        User {
            id: r.id,
            email: r.email,
            password_hash: r.password_hash,
            display_name: r.display_name,
            created_at: r.created_at,
        }
    }
}

pub async fn find_by_email(pool: &Pool, email: &str) -> StorageResult<Option<User>> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, password_hash, display_name, created_at
        FROM users
        WHERE email = ?1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Into::into))
}

pub async fn find_by_id(pool: &Pool, id: &str) -> StorageResult<Option<User>> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, password_hash, display_name, created_at
        FROM users
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Into::into))
}

pub async fn insert(
    pool: &Pool,
    id: &str,
    email: &str,
    password_hash: &str,
    display_name: &str,
) -> StorageResult<()> {
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
