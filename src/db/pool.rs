//! SQLite connection pool.

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::{path::Path, str::FromStr, time::Duration};

use crate::error::AppResult;

/// Build a connection pool and ensure the target DB file exists with sane
/// pragmas. Uses WAL for better concurrent reads; `foreign_keys` on.
pub async fn connect(url: &str) -> AppResult<SqlitePool> {
    // Ensure the parent directory exists when the URL points at a file.
    if let Some(path) = url.strip_prefix("sqlite://") {
        if !path.is_empty() && path != ":memory:" {
            if let Some(parent) = Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    tokio::fs::create_dir_all(parent).await.map_err(|e| {
                        crate::error::AppError::Internal(format!(
                            "failed to create db dir: {e}"
                        ))
                    })?;
                }
            }
        }
    }

    let opts = SqliteConnectOptions::from_str(url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(opts)
        .await?;

    Ok(pool)
}

/// Run embedded migrations.
pub async fn migrate(pool: &SqlitePool) -> AppResult<()> {
    sqlx::migrate!("./migrations").run(pool).await.map_err(|e| {
        crate::error::AppError::Internal(format!("migration failed: {e}"))
    })?;
    Ok(())
}
