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
/// 't') across this file's five tests safe regardless of order — a
/// shared, persistent PostgreSQL database (the CI service container,
/// `dialect::postgres_tests::migrated_pool`'s own convention) does
/// not, so the PostgreSQL body below resets those same three rows
/// before every test runs. See [`cleanup`] for the other half.
#[cfg(feature = "sqlite")]
async fn migrated_pool() -> (Pool, std::path::PathBuf) {
    let path = std::env::temp_dir().join(format!("peisear-store002-{}.db", uuid::Uuid::new_v4()));
    let url = format!("sqlite://{}", path.display());
    let pool = pool::connect(&url).await.expect("connect");
    pool::migrate(&pool).await.expect("migrate");
    (pool, path)
}

/// PostgreSQL's own body of [`migrated_pool`] (above). Deletes the
/// three roots this file's own tests ever seed — team `'t'`, project
/// `'p'`, user `'u'` — relying on this schema's own `ON DELETE CASCADE`
/// for what each one owns (sprints and their issues/records,
/// `team_memberships`, `user_capacities`), the same way a fresh SQLite
/// file discarded them. Order between the three deletes does not
/// matter: PostgreSQL resolves cascades correctly regardless of which
/// statement triggers them.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
async fn migrated_pool() -> (Pool, std::path::PathBuf) {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must point at a real PostgreSQL server for this test");
    let pool = pool::connect(&url).await.expect("connect");
    pool::migrate(&pool).await.expect("migrate");
    sqlx::query("DELETE FROM teams WHERE id = 't'")
        .execute(&pool)
        .await
        .expect("reset team 't' left by an earlier test");
    sqlx::query("DELETE FROM projects WHERE id = 'p'")
        .execute(&pool)
        .await
        .expect("reset project 'p' left by an earlier test");
    sqlx::query("DELETE FROM users WHERE id = 'u'")
        .execute(&pool)
        .await
        .expect("reset user 'u' left by an earlier test");
    (pool, std::path::PathBuf::new())
}

/// The SQLite half of this file's own cleanup: removes the temp file
/// [`migrated_pool`] created, plus its WAL/SHM siblings. A no-op on
/// PostgreSQL — `path` is an empty placeholder there (see
/// `migrated_pool`'s postgres body above), and `remove_file` on it
/// simply fails and is discarded, the same way a missing file already
/// was.
fn cleanup(path: &std::path::Path) {
    for ext in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
    }
}

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
