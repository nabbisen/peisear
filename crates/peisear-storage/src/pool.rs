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

/// True if `url` begins with one of PostgreSQL's own two accepted
/// schemes (`postgres://`, `postgresql://` — both, since `psql` and
/// `libpq` accept both and rejecting either would be a defect, not a
/// fix). **Not a general URL parser, and deliberately not a
/// `sqlite`-scheme check either** (`PG-001` round 3, the round 2
/// review's own finding, `§12`): `sqlx-sqlite-0.8.6`'s own `FromStr`
/// is
///
/// ```text
/// url = url.trim_start_matches("sqlite://").trim_start_matches("sqlite:");
/// ```
///
/// — it trims the scheme *if present* and treats whatever remains as
/// a path, so a URL with no scheme at all has always been accepted:
/// `data/app.db`, `/abs/path.db`, and, since `0.53.0` shipped an
/// `x86_64-pc-windows-msvc` archive, a Windows `C:\...` path whose own
/// drive-letter colon is not a scheme either. Round 2's `url_scheme`
/// (removed) extracted whatever sat before the first `:` and required
/// it to equal `"sqlite"` exactly — rejecting every one of those
/// pre-`PG-001` `DATABASE_URL` shapes the moment this check landed,
/// the thing it exists to prevent, done to the backend it was
/// supposed to leave alone. **This function is therefore asymmetric
/// on purpose, and must stay that way**: the SQLite branch below asks
/// only "is this unambiguously the *other* backend's URL?", never
/// "does this look like my own?", because `sqlx` itself does not
/// require the latter — "fixing" the two branches to match would
/// reintroduce this exact defect.
fn is_a_postgres_url(url: &str) -> bool {
    url.starts_with("postgres://") || url.starts_with("postgresql://")
}

/// The leading fragment of `url` up to its first `:`, for an error
/// message only — this is always the scheme (or, for a schemeless
/// path, whatever sits before its own first `:`, such as a Windows
/// drive letter), **never** anything from a userinfo or password,
/// which can only appear after a URL's own scheme-terminating `:`.
/// Only the PostgreSQL branch needs this: its own check is an
/// allow-list, so a rejection has no single named "other backend" the
/// way the SQLite branch's deny-list rejection does.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
fn leading_fragment(url: &str) -> &str {
    url.split(':').next().unwrap_or(url)
}

/// Build a connection pool and ensure the target DB file exists with sane
/// pragmas. Uses WAL for better concurrent reads; `foreign_keys` on.
///
/// **Rejects a PostgreSQL URL immediately** (`PG-001` round 2, fixed
/// in round 3 — see [`is_a_postgres_url`]'s own doc comment for why
/// this is a deny-list, not an allow-list), rather than reading it as
/// a nonsense filename and failing with SQLite's own "unable to open
/// database file".
#[cfg(feature = "sqlite")]
pub async fn connect(url: &str) -> StorageResult<Pool> {
    if is_a_postgres_url(url) {
        return Err(StorageError::Bootstrap(
            "this build was compiled with the `sqlite` feature, but the URL is a PostgreSQL \
             connection string (postgres:// or postgresql://)"
                .to_string(),
        ));
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
/// **Requires [`is_a_postgres_url`]** (`PG-001` round 2) — unlike the
/// SQLite branch, this one *is* an allow-list, and that is still
/// correct after round 3's fix: PostgreSQL has no bare-path form for
/// sqlx to have ever accepted, so there is no pre-existing
/// `DATABASE_URL` shape here to avoid breaking, only the brand-new
/// shape this round defines. Before this check existed, a
/// `sqlite://` URL parsed as a nonsense hostname and hung for
/// `PgPoolOptions`'s own 30 s acquire timeout before failing with
/// `PoolTimedOut` — reached, but not usefully.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub async fn connect(url: &str) -> StorageResult<Pool> {
    if !is_a_postgres_url(url) {
        return Err(StorageError::Bootstrap(format!(
            "this build was compiled with the `postgres` feature, but the URL does not start \
             with postgres:// or postgresql:// (got {:?})",
            leading_fragment(url)
        )));
    }

    let opts = PgConnectOptions::from_str(url)?.application_name("peisear");

    let pool = PgPoolOptions::new().connect_with(opts).await?;

    Ok(pool)
}

/// Run the PostgreSQL migration set (`PG-002`, `RFC 018` P3). A
/// second, separate embed — `migrations-postgres/`, not
/// `migrations/` — behind the same `cfg` that already selects which
/// `connect` body compiles; **two directories, two `migrate!`
/// calls**, chosen over a single parameterised embed because
/// `sqlx::migrate!`'s path argument must be a string literal known
/// at compile time, not a runtime choice — the `cfg` is what makes
/// the choice, once, at compile time, the same way it already does
/// for [`connect`] above.
///
/// **Partial, honestly**: `migrations-postgres/` mirrors every
/// SQLite migration's version number and intent, but omits every
/// `CREATE TRIGGER`/`DROP TRIGGER` statement — all 11 of this
/// schema's triggers are `PG-003`'s (`RFC 018` §5's own framing),
/// not this round's. A PostgreSQL database this function has
/// migrated has every table, column and index the SQLite one has,
/// and **none of the triggers** — no `updated_at` bump, no sub-issue
/// nesting guard, no planned-date ordering guard. See each ported
/// file's own header for exactly what it omits and why.
///
/// **The drift guard** between the two directories is
/// `migration_drift::mismatches` (test-only, so not a doc-linkable
/// item here) — a filename-level comparison (version number and
/// description both present in both directories), never SQL
/// semantics, per the handoff's own bound
/// (`PG-002` §2: "propose it; do not build a comparison that reads
/// SQL semantics").
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub async fn migrate(pool: &Pool) -> StorageResult<()> {
    sqlx::migrate!("./migrations-postgres").run(pool).await?;
    Ok(())
}

/// `CON-003`: PostgreSQL's own answer to the isolation a fresh SQLite
/// temp file already gives every `TestApp::spawn` for free. One
/// server, many tests -- a database per test is the unconditional
/// guarantee the round's own §2 chose over a shared schema
/// (`search_path` is per-connection, a property to keep proving
/// rather than one you have) or a shared, truncated database
/// (correctness would depend on `test.yml`'s own `--test-threads=1`
/// rather than on the fixture itself). Not `#[cfg(test)]`: it has to
/// be visible to `peisear-web`'s own integration-test binaries, a
/// separate crate that compiles this one in ordinary (non-test) mode.
///
/// Holds the pool so `Drop` can close it before the database it
/// names is dropped -- see `Drop`'s own doc comment for why that is
/// `Option` and why cleanup is best-effort.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub struct TestDatabase {
    pool: Option<Pool>,
    name: String,
    admin_url: String,
}

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
impl TestDatabase {
    /// The pool connected to this test's own database. Clone it
    /// freely (`sqlx::Pool` is a cheap, reference-counted handle);
    /// keep the `TestDatabase` itself alive for exactly as long as
    /// the test needs its database to exist.
    pub fn pool(&self) -> &Pool {
        self.pool
            .as_ref()
            .expect("TestDatabase pool taken before use")
    }
}

/// Create and migrate a fresh, uniquely-named database on the server
/// named by `DATABASE_URL`, for one test's exclusive use. Panics on
/// any failure -- there is no legitimate way for a test to recover
/// from being unable to set up its own fixture, the same convention
/// `TestApp::spawn`'s SQLite body already uses.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub async fn connect_isolated_test_database() -> TestDatabase {
    let admin_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must point at a real PostgreSQL server for this test");
    let admin_opts = PgConnectOptions::from_str(&admin_url)
        .expect("parse DATABASE_URL for the admin connection");
    let admin_pool = PgPoolOptions::new()
        .connect_with(admin_opts.clone())
        .await
        .expect("connect the admin pool used to create an isolated test database");

    // No hyphens (unlike `Uuid`'s default `Display`) and well under
    // PostgreSQL's 63-byte identifier limit.
    let name = format!("peisear_it_{}", uuid::Uuid::new_v4().simple());
    sqlx::query(&format!(r#"CREATE DATABASE "{name}""#))
        .execute(&admin_pool)
        .await
        .expect("create this test's own isolated database");
    admin_pool.close().await;

    let pool = PgPoolOptions::new()
        .connect_with(admin_opts.database(&name))
        .await
        .expect("connect to the isolated test database just created");
    migrate(&pool)
        .await
        .expect("migrate the isolated test database");

    TestDatabase {
        pool: Some(pool),
        name,
        admin_url,
    }
}

/// Best-effort cleanup: closes this test's own pool, then drops the
/// database, inside a detached task -- `Drop` cannot be `async`, and
/// nothing in this project's CI needs the result, since the whole
/// service container (every database it holds, with it) is discarded
/// at the end of the job regardless. A long-lived local server
/// accumulates `peisear_it_*` databases across local runs the same
/// way a cancelled local run can leave a `peisear-web-it-*` directory
/// under `/tmp` -- `tests/common/server.rs`'s own doc comment names
/// that trade-off for SQLite; this is the same trade-off for
/// PostgreSQL, not a new one.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
impl Drop for TestDatabase {
    fn drop(&mut self) {
        let Some(pool) = self.pool.take() else {
            return;
        };
        let name = self.name.clone();
        let admin_url = self.admin_url.clone();
        tokio::spawn(async move {
            pool.close().await;
            let Ok(opts) = PgConnectOptions::from_str(&admin_url) else {
                return;
            };
            let Ok(admin_pool) = PgPoolOptions::new().connect_with(opts).await else {
                return;
            };
            let _ = sqlx::query(&format!(r#"DROP DATABASE IF EXISTS "{name}""#))
                .execute(&admin_pool)
                .await;
        });
    }
}

#[cfg(test)]
mod tests;
