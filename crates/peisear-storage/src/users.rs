//! User table queries.

use chrono::{DateTime, Utc};
use peisear_core::User;
use sqlx::FromRow;

use crate::{Pool, StorageError, StorageResult};

#[derive(FromRow)]
struct UserRow {
    id: String,
    email: String,
    password_hash: String,
    display_name: String,
    capacity_points: Option<i64>,
    wip_limit: Option<i64>,
    created_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        User {
            id: r.id,
            email: r.email,
            password_hash: r.password_hash,
            display_name: r.display_name,
            capacity_points: r.capacity_points,
            wip_limit: r.wip_limit,
            created_at: r.created_at,
        }
    }
}

pub async fn find_by_email(pool: &Pool, email: &str) -> StorageResult<Option<User>> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, password_hash, display_name, capacity_points, wip_limit, created_at
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
        SELECT id, email, password_hash, display_name, capacity_points, wip_limit, created_at
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

/// Update a user's capacity. `Some(n > 0)` sets it; `None` clears it
/// (the user opts out of workload monitoring).
pub async fn set_capacity(
    pool: &Pool,
    user_id: &str,
    capacity_points: Option<i64>,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE users
        SET capacity_points = ?2
        WHERE id = ?1
        "#,
    )
    .bind(user_id)
    .bind(capacity_points)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// Update a user's personal WIP limit. `Some(n > 0)` sets it;
/// `None` clears it (the user falls back to the project default
/// or the system default of [`peisear_core::personal_metrics::DEFAULT_WIP_LIMIT`]).
pub async fn set_wip_limit(
    pool: &Pool,
    user_id: &str,
    wip_limit: Option<i64>,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE users
        SET wip_limit = ?2
        WHERE id = ?1
        "#,
    )
    .bind(user_id)
    .bind(wip_limit)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}
