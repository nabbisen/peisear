//! `STORE-002` §1: the five `julianday`-mediated optimistic-lock guards,
//! demonstrated directly, not merely measured.
//!
//! Three entities carry a **second** lock guard, in SQL, beside the
//! handler's own Rust comparison (`check_optimistic_lock`):
//!
//! ```sql
//! AND (?N IS NULL OR julianday(updated_at) = julianday(?N))
//! ```
//!
//! `projects::update_guarded`, `projects::delete_guarded`,
//! `sprints::update_guarded`, `sprints::delete_guarded`,
//! `user_capacities::delete_guarded` — five call sites, three entities.
//! `STORE-001`'s own demonstration went through the handlers, where the
//! Rust check refuses a stale token *first*, so no handler-level test
//! can ever reach these. These tests call the storage functions
//! directly, the one way to exercise the SQL predicate itself.
//!
//! Each test asserts **both halves**, per the handoff's own instruction:
//! a stale millisecond token is refused (no row changed), and the
//! *current* token succeeds — a guard that refuses everything would
//! still pass a check that only tried the first half.
//!
//! No sleep between the legitimate update and the stale-token retry —
//! same discipline as `STORE-001`'s own demonstration, and for the same
//! reason: the defect this exists to catch is a same-second collision,
//! and a test with a delay between its two writes could not find it.
//!
//! **Found running this file, not read in it** (`SEC-001`): each test's
//! "stale" token is read straight after seeding — whole-second, the
//! column `DEFAULT` — and the legitimate edit that follows fires
//! `0021`'s trigger, millisecond precision. These two coincide with
//! real, if rare (~1-in-1000), probability: if the trigger's own
//! fraction happens to land on exactly `.000`, the "stale" token and the
//! post-edit one are the *same value*, and there is nothing stale about
//! it to refuse — a correct `Written(())` that an assertion expecting
//! `Stale` would wrongly read as a guard regression. Each test below
//! retries its legitimate edit (bounded, and only with a token read
//! fresh from the row, so a retry is itself still a legitimate edit)
//! until the post-edit token is provably distinct from the seed one,
//! rather than let the next unlucky run fail on nothing.

use crate::{Guarded, Pool, pool, projects, sprints, user_capacities};
use chrono::{DateTime, NaiveDate, Utc};
use peisear_auth::jwt::{RequesterId, issue, verify};

/// `CON-002` (`RFC 018` P4): a fresh, fully-migrated database. SQLite's
/// own brand-new temp file made reusing fixed literal ids ('u', 'p',
/// 't') across this file's five tests safe regardless of order.
///
/// `CON-003` retired this function's own earlier PostgreSQL body,
/// which reset three fixed-id rows by hand before every test and
/// depended on `test.yml`'s `--test-threads=1` to stay correct —
/// exactly the isolation shape that round's own handoff named as a
/// fourth, pre-existing instance of the problem it was written to
/// solve everywhere else. The PostgreSQL body below now uses the same
/// database-per-test mechanism `peisear-web`'s own lock tests use
/// (`pool::connect_isolated_test_database`), so this file keeps its
/// fixed literal ids for the same reason SQLite's temp file let it
/// use them in the first place: nothing else can be in the database.
#[cfg(feature = "sqlite")]
async fn migrated_pool() -> (Pool, std::path::PathBuf) {
    let path = std::env::temp_dir().join(format!("peisear-store002-{}.db", uuid::Uuid::new_v4()));
    let url = format!("sqlite://{}", path.display());
    let pool = pool::connect(&url).await.expect("connect");
    pool::migrate(&pool).await.expect("migrate");
    (pool, path)
}

/// PostgreSQL's own body of [`migrated_pool`] (above). See its doc
/// comment for why this no longer resets fixed-id rows in a shared
/// database. [`cleanup`]'s own postgres body is a no-op: the returned
/// [`pool::TestDatabase`]'s `Drop` does the cleanup once the test
/// function's local variable goes out of scope.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
async fn migrated_pool() -> (Pool, pool::TestDatabase) {
    let test_database = pool::connect_isolated_test_database().await;
    let pool = test_database.pool().clone();
    (pool, test_database)
}

/// The SQLite half of this file's own cleanup: removes the temp file
/// [`migrated_pool`] created, plus its WAL/SHM siblings.
#[cfg(feature = "sqlite")]
fn cleanup(path: &std::path::Path) {
    for ext in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
    }
}

/// A no-op on PostgreSQL — every call site keeps one shape across
/// both backends, but the real cleanup is [`pool::TestDatabase`]'s
/// own `Drop`, which runs when the test function's `_db` local
/// finally goes out of scope, not at this call.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
fn cleanup(_db: &pool::TestDatabase) {}

/// A real `RequesterId` — obtainable only from [`verify`], so a test that
/// needs one goes through [`issue`] first, exactly as a login would.
fn requester(id: &str) -> RequesterId {
    let token = issue(id, &format!("{id}@example.test"), "store002-test-secret")
        .expect("issue a test token");
    verify(&token, "store002-test-secret").expect("verify the token just issued")
}

async fn seed_user(pool: &Pool, id: &str) {
    crate::users::insert(pool, id, &format!("{id}@example.test"), "x", "U")
        .await
        .expect("seed user");
}

async fn seed_team(pool: &Pool, creator_id: &str) -> String {
    sqlx::query(
        "INSERT INTO teams (id, name, slug, description) VALUES ('t', 'Team', 'team', NULL)",
    )
    .execute(pool)
    .await
    .expect("seed team");
    let _ = creator_id;
    "t".to_string()
}

#[cfg(test)]
mod tests;
