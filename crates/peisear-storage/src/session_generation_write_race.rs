//! `PWD-002` §4.3: `users::change_password`'s `RETURNING` fix, proven
//! against an interleaving that is real rather than timed.
//!
//! The defect it replaces computed `session_generation + 1` from a row
//! read *before* the write (`user.session_generation + 1` in
//! `handlers::settings::change_password`), which is correct only if
//! nothing else wrote the row in between. `advance_session_generation`
//! — the `logout` path — is the writer that actually races in
//! production (`PWD-002` §1); this module's test calls it directly,
//! between a read and a `change_password` call, so the interleaving is
//! forced rather than hoped for. No sleep and no second task: both
//! calls run on one `Pool` in this one `async fn`, so there is nothing
//! to wait on — the row simply holds `G+1` by the time
//! `change_password` runs, which is the whole of what needs
//! demonstrating.

use crate::{Pool, pool, users};

#[cfg(feature = "sqlite")]
async fn migrated_pool() -> (Pool, std::path::PathBuf) {
    let path = std::env::temp_dir().join(format!("peisear-pwd002-{}.db", uuid::Uuid::new_v4()));
    let url = format!("sqlite://{}", path.display());
    let pool = pool::connect(&url).await.expect("connect");
    pool::migrate(&pool).await.expect("migrate");
    (pool, path)
}

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
async fn migrated_pool() -> (Pool, pool::TestDatabase) {
    let test_database = pool::connect_isolated_test_database().await;
    let pool = test_database.pool().clone();
    (pool, test_database)
}

#[cfg(feature = "sqlite")]
fn cleanup(path: &std::path::Path) {
    for ext in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
    }
}

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
fn cleanup(_db: &pool::TestDatabase) {}

/// **Plant**: against the pre-`PWD-002` arithmetic
/// (`user.session_generation + 1`, computed from a row read before
/// `advance_session_generation` ran), this assertion reads `G + 1`
/// where the row actually holds `G + 2` — confirmed by running this
/// test against that arithmetic before the `RETURNING` fix landed, not
/// assumed from reading the diff.
#[tokio::test]
async fn change_password_returns_the_generation_it_actually_committed_through_a_concurrent_logout()
{
    let (pool, db) = migrated_pool().await;
    users::insert(&pool, "u", "u@example.test", "old-hash", "U")
        .await
        .expect("seed user");

    let before = users::find_by_id(&pool, "u")
        .await
        .expect("query the seeded row")
        .expect("the row exists");
    let g = before.session_generation;

    // The writer that actually races in production: a `logout` on
    // another device, landing between this handler's read and its
    // write.
    users::advance_session_generation(&pool, "u")
        .await
        .expect("advance the generation (the logout path)");

    let new_generation = users::change_password(&pool, "u", "new-hash")
        .await
        .expect("change the password");

    assert_eq!(
        new_generation,
        g + 2,
        "the returned generation must be what the UPDATE actually committed (g+2, one from \
         the interleaved logout and one from this change), not g+1 computed from a row read \
         before either write"
    );

    let after = users::find_by_id(&pool, "u")
        .await
        .expect("query the row again")
        .expect("the row still exists");
    assert_eq!(
        after.session_generation,
        g + 2,
        "the returned value must match what the row actually stores, not merely what the \
         caller computed"
    );

    pool.close().await;
    cleanup(&db);
}
