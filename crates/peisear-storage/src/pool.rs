//! Connection pool, per active backend (`DIA-002`'s own seam: the
//! `Pool` type lives in [`crate::dialect`], but `connect`/`migrate`'s
//! *bodies* are what actually differ between backends, so they are
//! cfg-gated here rather than there — `dialect.rs` owns the type and
//! the error translation; this file owns what a connection and a
//! migration run actually mean per backend).

#[cfg(feature = "sqlite")]
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
#[cfg(feature = "sqlite")]
use std::{path::Path, str::FromStr, time::Duration};

#[cfg(feature = "sqlite")]
use crate::StorageError;
use crate::{Pool, StorageResult};

/// Build a connection pool and ensure the target DB file exists with sane
/// pragmas. Uses WAL for better concurrent reads; `foreign_keys` on.
#[cfg(feature = "sqlite")]
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
        // `STORE-002` (`RFC 018` P0b), measured 2026-10-11 on an AMD Ryzen 9
        // 9950X (NVMe-backed, LUKS-encrypted filesystem): `NORMAL` can lose
        // recently committed transactions on a power cut or an OS crash;
        // `WAL`+`NORMAL` only fsyncs at checkpoint, where `FULL` fsyncs the
        // WAL on every commit. The write-heavy path `browser-checks/`'s own
        // fixture builder exercises (a registration, a team, a sprint, a
        // project, several issues, through the real forms) was timed under
        // both, ten rounds each, order randomised per round to cancel out
        // drift over the run: 443.9 ms mean under `NORMAL`, 441.9 ms under
        // `FULL` -- a 2 ms difference on a ~440 ms base, inside the ~10 ms
        // round-to-round noise either setting shows on this machine. `FULL`'s
        // extra fsync is not a measurable cost here, which settles this in
        // the direction self-hosting already implies: 5-30 users on one
        // node, human-paced writes, and durability that is part of why
        // someone self-hosts in the first place.
        .synchronous(SqliteSynchronous::Full)
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
#[cfg(feature = "sqlite")]
pub async fn migrate(pool: &Pool) -> StorageResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

/// `DIA-002`: proves the seam's own compile-time half, nothing more.
/// **Not a connection string** — the handoff's own bound on this
/// round. `P3` writes the real `PgConnectOptions`/`PgPoolOptions`
/// tuning (chosen against a real server, not guessed here, the same
/// reason [`crate::dialect`]'s own doc comment gives for leaving the
/// placeholder style and migration syntax unabstracted) and replaces
/// this body. Until then, a build with `postgres` selected compiles
/// and links — proving the type seam holds — and panics the moment
/// anything actually tries to open a connection, which nothing in
/// this project's own CI does (`DIA-002` §4: "No PostgreSQL in CI
/// yet").
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub async fn connect(_url: &str) -> StorageResult<Pool> {
    unimplemented!(
        "PostgreSQL support is not implemented yet (DIA-002 builds the seam, not the backend) \
         -- P3 replaces this"
    )
}

/// See [`connect`]'s own doc comment — the same reasoning, the same
/// bound, the same `P3` deferral.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub async fn migrate(_pool: &Pool) -> StorageResult<()> {
    unimplemented!(
        "PostgreSQL support is not implemented yet (DIA-002 builds the seam, not the backend) \
         -- P3 replaces this"
    )
}

#[cfg(test)]
mod tests;
