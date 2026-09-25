//! SQLite connection pool.

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use std::{path::Path, str::FromStr, time::Duration};

use crate::{Pool, StorageError, StorageResult};

/// Build a connection pool and ensure the target DB file exists with sane
/// pragmas. Uses WAL for better concurrent reads; `foreign_keys` on.
pub async fn connect(url: &str) -> StorageResult<Pool> {
    // Ensure the parent directory exists when the URL points at a file.
    if let Some(path) = url.strip_prefix("sqlite://")
        && !path.is_empty()
        && path != ":memory:"
        && let Some(parent) = Path::new(path).parent()
        && !parent.as_os_str().is_empty()
    {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| StorageError::Bootstrap(format!("create_dir_all: {e}")))?;
    }

    let opts = SqliteConnectOptions::from_str(url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5))
        // `PERF-002`: let each install's own data choose its plans. Nothing
        // here (or anywhere) runs `ANALYZE`, so SQLite plans from built-in
        // guesses, and *which* guesses depends on the linked SQLite version
        // (`PERF-001`). `PRAGMA optimize` on close is a no-op until a table
        // has changed enough, or has never been analysed, and it only notes
        // the tables *this connection's* queries could have used statistics
        // for. It runs when sqlx closes a connection -- the pool's idle and
        // lifetime reapers, or `Pool::close` -- never at connect and never on
        // a request. The row limit is left to SQLite (`PRAGMA optimize` caps
        // it at 1000): the manual's 400 was measured on a 60,000-issue
        // database and gave statistics too coarse for the plan change that
        // made this worth doing, so it is deliberately not passed.
        .optimize_on_close(true, None);

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(opts)
        .await?;

    Ok(pool)
}

/// Run embedded migrations. The migration directory lives inside this
/// crate so its path is relative to the crate's `CARGO_MANIFEST_DIR`,
/// not the workspace root.
pub async fn migrate(pool: &Pool) -> StorageResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `PERF-002`: a used install gains statistics when its connections close,
    /// and not before -- so nothing runs at startup or on a request.
    #[tokio::test]
    async fn statistics_appear_when_a_used_pool_closes_and_not_before() {
        let path =
            std::env::temp_dir().join(format!("peisear-perf002-{}.db", uuid::Uuid::new_v4()));
        let url = format!("sqlite://{}", path.display());

        async fn has_stats(pool: &Pool) -> bool {
            let n: i64 = sqlx::query_scalar(
                "SELECT count(*) FROM sqlite_master WHERE name = 'sqlite_stat1'",
            )
            .fetch_one(pool)
            .await
            .unwrap();
            n == 1
        }

        let pool = connect(&url).await.expect("connect");
        migrate(&pool).await.expect("migrate");
        assert!(!has_stats(&pool).await, "nothing analyses at startup");

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, display_name) \
             VALUES ('u', 'u@example.test', 'x', 'U')",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO projects (id, owner_id, name) VALUES ('p', 'u', 'P')")
            .execute(&pool)
            .await
            .unwrap();
        // Enough rows that there is something for `optimize` to decide about.
        sqlx::query(
            "WITH RECURSIVE n(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM n WHERE x < 600) \
             INSERT INTO issues (id, project_id, author_id, title, status) \
             SELECT 'i' || x, 'p', 'u', 'T', CASE x % 3 WHEN 0 THEN 'done' ELSE 'open' END FROM n",
        )
        .execute(&pool)
        .await
        .unwrap();

        // Use it: queries that could have used statistics, on one connection.
        {
            let mut conn = pool.acquire().await.unwrap();
            for _ in 0..3 {
                sqlx::query("SELECT id FROM issues WHERE project_id = 'p' AND status = 'open'")
                    .fetch_all(&mut *conn)
                    .await
                    .unwrap();
                sqlx::query("SELECT id FROM issues WHERE assignee_id = 'u'")
                    .fetch_all(&mut *conn)
                    .await
                    .unwrap();
            }
        }
        assert!(
            !has_stats(&pool).await,
            "use alone must not analyse -- nothing on the request path"
        );

        pool.close().await;
        let reopened = connect(&url).await.expect("reopen");
        assert!(
            has_stats(&reopened).await,
            "closing a used pool must have run PRAGMA optimize"
        );
        let analysed: i64 =
            sqlx::query_scalar("SELECT count(*) FROM sqlite_stat1 WHERE tbl = 'issues'")
                .fetch_one(&reopened)
                .await
                .unwrap();
        assert!(analysed > 0, "the issues table must have been analysed");
        reopened.close().await;
        for ext in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
        }
    }
}
