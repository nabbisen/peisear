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

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
use std::str::FromStr;

use crate::StorageError;
use crate::{Pool, StorageResult};

/// The scheme a connection URL names, bounded to the part before the
/// first `:` — not a full URL parser, deliberately (`PG-001` round 2).
/// A build selects its dialect at compile time, so a URL naming the
/// *other* one is a real mistake, not a hypothetical: before this
/// check existed, a `sqlite`-feature build given a `postgres://` URL
/// read it as a nonsense filename and failed with SQLite's own
/// "unable to open database file", and a `postgres`-feature build
/// given a `sqlite://` URL read it as a nonsense hostname and hung
/// for `PgPoolOptions`'s own 30 s acquire timeout before failing —
/// both reached, both "working", neither telling the operator what
/// was actually wrong. Checking the scheme first, before either
/// dialect's own parser touches the rest of the string, turns both
/// into one immediate, named error.
fn url_scheme(url: &str) -> &str {
    url.split(':').next().unwrap_or(url)
}

/// Build a connection pool and ensure the target DB file exists with sane
/// pragmas. Uses WAL for better concurrent reads; `foreign_keys` on.
///
/// **Checks [`url_scheme`] first** (`PG-001` round 2) — a non-`sqlite`
/// URL given to this build is rejected immediately, named, rather
/// than read as a nonsense filename and failed with SQLite's own
/// "unable to open database file".
#[cfg(feature = "sqlite")]
pub async fn connect(url: &str) -> StorageResult<Pool> {
    let scheme = url_scheme(url);
    if scheme != "sqlite" {
        return Err(StorageError::Bootstrap(format!(
            "expected a sqlite:// URL (this build was compiled with the `sqlite` feature), got \
             a {scheme:?} URL"
        )));
    }

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

/// Build a connection pool against a real PostgreSQL server (`PG-001`,
/// `RFC 018` P3's own first round, 0.55.0). Replaces `DIA-002`'s
/// `unimplemented!()` stub — this is the first PostgreSQL code this
/// project runs, proved against a real server in CI (`test.yml`'s
/// `test-postgres-connection` job), not merely compiled.
///
/// **No `create_dir_all`, unlike the SQLite branch above.** A SQLite
/// URL names a file this process may need to create; a PostgreSQL URL
/// names a database that must already exist — `CREATE DATABASE` is an
/// operator's action, not something a connection pool does on a
/// caller's behalf, and no PostgreSQL equivalent of "create if
/// missing" exists at this layer.
///
/// **Options evaluated against the profile this project already has
/// (`NFR-PERF-001`: 5–30 users, one node), not guessed:**
/// - `application_name("peisear")` — genuinely new, not a port of
///   anything on the SQLite side, since SQLite has no concept of
///   several applications sharing one server. `docs/operations/scaling.md`
///   names a single shared PostgreSQL instance as an anticipated
///   topology; an operator reading `pg_stat_activity` or the server
///   log on that instance can now tell which connections are this
///   project's.
/// - `ssl_mode` — left at `PgConnectOptions`'s own default (`Prefer`:
///   try TLS, fall back to plaintext). Already the right shape for a
///   self-hosted deployment that may or may not have TLS configured
///   on its own PostgreSQL server — a self-hosted install choosing
///   `require`/`verify-full` sets it in the URL itself
///   (`sslmode=require`), which `PgConnectOptions::from_str` already
///   honours.
/// - `statement_cache_capacity` — left at its default (100). This
///   crate issues no `sqlx::query!` macros (runtime API throughout,
///   `dialect.rs`'s own module doc), but the runtime API still
///   benefits from statement caching, and nothing measured here gives
///   a reason to move off the default.
/// - `PgPoolOptions`'s own defaults (10 max connections, 0 min, 30 s
///   acquire timeout, 10-minute idle timeout, 30-minute max lifetime)
///   are **left unchanged, deliberately**. `max_connections(8)` on
///   the SQLite side above carries no comment tying it to a specific
///   number of users, so it is not a number this round can match
///   against — but 10 is the same order of magnitude, comfortably
///   inside `NFR-PERF-001`'s 5–30-user, single-node profile, and this
///   round has no real server to load-test a different number
///   against. Per the handoff's own §5: an unchanged default with a
///   reason beats a tuned number without one.
///
/// **Checks [`url_scheme`] first** (`PG-001` round 2, same reasoning
/// as the SQLite branch's own check) — `postgres` and `postgresql`
/// both accepted, since PostgreSQL's own tooling accepts both and
/// rejecting one would be a new defect, not a round 2 fix. Before
/// this check existed, a `sqlite://` URL parsed as a nonsense
/// hostname and hung for `PgPoolOptions`'s own 30 s acquire timeout
/// before failing with `PoolTimedOut` — reached, but not usefully.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub async fn connect(url: &str) -> StorageResult<Pool> {
    let scheme = url_scheme(url);
    if scheme != "postgres" && scheme != "postgresql" {
        return Err(StorageError::Bootstrap(format!(
            "expected a postgres:// or postgresql:// URL (this build was compiled with the \
             `postgres` feature), got a {scheme:?} URL"
        )));
    }

    let opts = PgConnectOptions::from_str(url)?.application_name("peisear");

    let pool = PgPoolOptions::new().connect_with(opts).await?;

    Ok(pool)
}

/// Run the PostgreSQL migration set. **Still a stub** — `PG-002` has
/// not landed yet, so there is no PostgreSQL migration directory to
/// embed. [`connect`] above is real; this is not, and says so rather
/// than silently inheriting [`connect`]'s own doc comment the way the
/// pre-`PG-001` version of this file did for both functions alike.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub async fn migrate(_pool: &Pool) -> StorageResult<()> {
    unimplemented!(
        "PostgreSQL migrations are not implemented yet (PG-001 implements connect; PG-002 is \
         where the migration set and this function land)"
    )
}

#[cfg(test)]
mod tests;
