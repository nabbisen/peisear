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

use crate::{Guarded, Pool, pool, projects, sprints, user_capacities};
use chrono::{DateTime, NaiveDate, Utc};
use peisear_auth::jwt::{RequesterId, issue, verify};

async fn migrated_pool() -> (Pool, std::path::PathBuf) {
    let path = std::env::temp_dir().join(format!("peisear-store002-{}.db", uuid::Uuid::new_v4()));
    let url = format!("sqlite://{}", path.display());
    let pool = pool::connect(&url).await.expect("connect");
    pool::migrate(&pool).await.expect("migrate");
    (pool, path)
}

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

#[tokio::test]
async fn projects_update_guard_refuses_a_stale_millisecond_token_and_accepts_the_current_one() {
    let (pool, path) = migrated_pool().await;
    seed_user(&pool, "u").await;
    projects::insert(&pool, "p", "u", "P", "", None)
        .await
        .expect("seed project");
    let t0: DateTime<Utc> = sqlx::query_scalar("SELECT updated_at FROM projects WHERE id = 'p'")
        .fetch_one(&pool)
        .await
        .unwrap();

    // Legitimate first edit -- fires 0021's trigger, bumps to millisecond
    // precision. No sleep before or after: the collision this guards
    // against is same-second.
    projects::update_guarded(&pool, "p", "u", "P2", "", Some(t0))
        .await
        .expect("first edit");

    let outcome = projects::update_guarded(&pool, "p", "u", "P-stale", "", Some(t0))
        .await
        .expect("guarded call itself must not error");
    assert!(
        matches!(outcome, Guarded::Stale { .. }),
        "a stale millisecond token must be refused, got {outcome:?}"
    );
    let name: String = sqlx::query_scalar("SELECT name FROM projects WHERE id = 'p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(name, "P2", "the refused write must not have applied");

    let t1: DateTime<Utc> = sqlx::query_scalar("SELECT updated_at FROM projects WHERE id = 'p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let outcome = projects::update_guarded(&pool, "p", "u", "P-current", "", Some(t1))
        .await
        .expect("guarded call itself must not error");
    assert_eq!(
        outcome,
        Guarded::Written(()),
        "the current token must succeed"
    );

    pool.close().await;
    cleanup(&path);
}

#[tokio::test]
async fn projects_delete_guard_refuses_a_stale_millisecond_token_and_accepts_the_current_one() {
    let (pool, path) = migrated_pool().await;
    seed_user(&pool, "u").await;
    projects::insert(&pool, "p", "u", "P", "", None)
        .await
        .expect("seed project");
    let t0: DateTime<Utc> = sqlx::query_scalar("SELECT updated_at FROM projects WHERE id = 'p'")
        .fetch_one(&pool)
        .await
        .unwrap();

    // Legitimate edit, so the row's token has moved past t0.
    projects::update_guarded(&pool, "p", "u", "P2", "", Some(t0))
        .await
        .expect("edit before the delete attempt");

    let outcome = projects::delete_guarded(&pool, "p", "u", Some(t0))
        .await
        .expect("guarded call itself must not error");
    assert!(
        matches!(outcome, Guarded::Stale { .. }),
        "a stale millisecond token must refuse the delete, got {outcome:?}"
    );
    let still_there: Option<String> = sqlx::query_scalar("SELECT id FROM projects WHERE id = 'p'")
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert_eq!(
        still_there.as_deref(),
        Some("p"),
        "the refused delete must not have applied"
    );

    let t1: DateTime<Utc> = sqlx::query_scalar("SELECT updated_at FROM projects WHERE id = 'p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let outcome = projects::delete_guarded(&pool, "p", "u", Some(t1))
        .await
        .expect("guarded call itself must not error");
    assert_eq!(
        outcome,
        Guarded::Written(()),
        "the current token must succeed"
    );
    let gone: Option<String> = sqlx::query_scalar("SELECT id FROM projects WHERE id = 'p'")
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert_eq!(gone, None, "the accepted delete must have applied");

    pool.close().await;
    cleanup(&path);
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

#[tokio::test]
async fn sprints_update_guard_refuses_a_stale_millisecond_token_and_accepts_the_current_one() {
    let (pool, path) = migrated_pool().await;
    seed_user(&pool, "u").await;
    let team_id = seed_team(&pool, "u").await;
    let starts_on = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let ends_on = NaiveDate::from_ymd_opt(2026, 1, 14).unwrap();
    let sprint_id = sprints::insert(&pool, &team_id, "Sprint 1", None, starts_on, ends_on)
        .await
        .expect("seed sprint");
    let t0: DateTime<Utc> = sqlx::query_scalar("SELECT updated_at FROM sprints WHERE id = ?1")
        .bind(&sprint_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    sprints::update_guarded(
        &pool,
        &sprint_id,
        "Sprint 1 renamed",
        None,
        starts_on,
        ends_on,
        Some(t0),
    )
    .await
    .expect("first edit");

    let outcome = sprints::update_guarded(
        &pool,
        &sprint_id,
        "Sprint 1 stale write",
        None,
        starts_on,
        ends_on,
        Some(t0),
    )
    .await
    .expect("guarded call itself must not error");
    assert!(
        matches!(outcome, Guarded::Stale { .. }),
        "a stale millisecond token must be refused, got {outcome:?}"
    );
    let name: String = sqlx::query_scalar("SELECT name FROM sprints WHERE id = ?1")
        .bind(&sprint_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        name, "Sprint 1 renamed",
        "the refused write must not have applied"
    );

    let t1: DateTime<Utc> = sqlx::query_scalar("SELECT updated_at FROM sprints WHERE id = ?1")
        .bind(&sprint_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let outcome = sprints::update_guarded(
        &pool,
        &sprint_id,
        "Sprint 1 current write",
        None,
        starts_on,
        ends_on,
        Some(t1),
    )
    .await
    .expect("guarded call itself must not error");
    assert_eq!(
        outcome,
        Guarded::Written(()),
        "the current token must succeed"
    );

    pool.close().await;
    cleanup(&path);
}

#[tokio::test]
async fn sprints_delete_guard_refuses_a_stale_millisecond_token_and_accepts_the_current_one() {
    let (pool, path) = migrated_pool().await;
    seed_user(&pool, "u").await;
    let team_id = seed_team(&pool, "u").await;
    let starts_on = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let ends_on = NaiveDate::from_ymd_opt(2026, 1, 14).unwrap();
    let sprint_id = sprints::insert(&pool, &team_id, "Sprint 1", None, starts_on, ends_on)
        .await
        .expect("seed sprint");
    let t0: DateTime<Utc> = sqlx::query_scalar("SELECT updated_at FROM sprints WHERE id = ?1")
        .bind(&sprint_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    sprints::update_guarded(
        &pool,
        &sprint_id,
        "Sprint 1 renamed",
        None,
        starts_on,
        ends_on,
        Some(t0),
    )
    .await
    .expect("edit before the delete attempt");

    let outcome = sprints::delete_guarded(&pool, &sprint_id, Some(t0))
        .await
        .expect("guarded call itself must not error");
    assert!(
        matches!(outcome, Guarded::Stale { .. }),
        "a stale millisecond token must refuse the delete, got {outcome:?}"
    );
    let still_there: Option<String> = sqlx::query_scalar("SELECT id FROM sprints WHERE id = ?1")
        .bind(&sprint_id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert!(
        still_there.is_some(),
        "the refused delete must not have applied"
    );

    let t1: DateTime<Utc> = sqlx::query_scalar("SELECT updated_at FROM sprints WHERE id = ?1")
        .bind(&sprint_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let outcome = sprints::delete_guarded(&pool, &sprint_id, Some(t1))
        .await
        .expect("guarded call itself must not error");
    assert_eq!(
        outcome,
        Guarded::Written(()),
        "the current token must succeed"
    );
    let gone: Option<String> = sqlx::query_scalar("SELECT id FROM sprints WHERE id = ?1")
        .bind(&sprint_id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert_eq!(gone, None, "the accepted delete must have applied");

    pool.close().await;
    cleanup(&path);
}

#[tokio::test]
async fn user_capacities_delete_guard_refuses_a_stale_millisecond_token_and_accepts_the_current_one()
 {
    let (pool, path) = migrated_pool().await;
    seed_user(&pool, "u").await;
    let user = requester("u");
    let capacity_id = user_capacities::insert(&pool, &user, 10, None, None, None)
        .await
        .expect("seed capacity row");
    let t0: DateTime<Utc> =
        sqlx::query_scalar("SELECT updated_at FROM user_capacities WHERE id = ?1")
            .bind(&capacity_id)
            .fetch_one(&pool)
            .await
            .unwrap();

    // Legitimate edit, through the update function's own guarded form,
    // so the row's token moves past t0.
    user_capacities::update_guarded(&pool, &user, &capacity_id, 12, None, None, None, Some(t0))
        .await
        .expect("edit before the delete attempt");

    let outcome = user_capacities::delete_guarded(&pool, &user, &capacity_id, Some(t0))
        .await
        .expect("guarded call itself must not error");
    assert!(
        matches!(outcome, Guarded::Stale { .. }),
        "a stale millisecond token must refuse the delete, got {outcome:?}"
    );
    let still_there: Option<String> =
        sqlx::query_scalar("SELECT id FROM user_capacities WHERE id = ?1")
            .bind(&capacity_id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(
        still_there.is_some(),
        "the refused delete must not have applied"
    );

    let t1: DateTime<Utc> =
        sqlx::query_scalar("SELECT updated_at FROM user_capacities WHERE id = ?1")
            .bind(&capacity_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    let outcome = user_capacities::delete_guarded(&pool, &user, &capacity_id, Some(t1))
        .await
        .expect("guarded call itself must not error");
    assert_eq!(
        outcome,
        Guarded::Written(()),
        "the current token must succeed"
    );
    let gone: Option<String> = sqlx::query_scalar("SELECT id FROM user_capacities WHERE id = ?1")
        .bind(&capacity_id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert_eq!(gone, None, "the accepted delete must have applied");

    pool.close().await;
    cleanup(&path);
}
