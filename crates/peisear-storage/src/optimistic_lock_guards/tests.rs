use super::*;

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
    // against is same-second. Retried (bounded) until the post-edit
    // token is provably distinct from t0 -- see this module's doc
    // comment on why that is not always true on the first try.
    let mut current = t0;
    for attempt in 0.. {
        assert!(
            attempt < 20,
            "the trigger never produced a token distinct from t0"
        );
        projects::update_guarded(&pool, "p", "u", "P2", "", Some(current))
            .await
            .expect("first edit");
        current = sqlx::query_scalar("SELECT updated_at FROM projects WHERE id = 'p'")
            .fetch_one(&pool)
            .await
            .unwrap();
        if current != t0 {
            break;
        }
    }

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

    // Legitimate edit, so the row's token has moved past t0. Retried
    // (bounded) for the same reason as the update-guard test above.
    let mut current = t0;
    for attempt in 0.. {
        assert!(
            attempt < 20,
            "the trigger never produced a token distinct from t0"
        );
        projects::update_guarded(&pool, "p", "u", "P2", "", Some(current))
            .await
            .expect("edit before the delete attempt");
        current = sqlx::query_scalar("SELECT updated_at FROM projects WHERE id = 'p'")
            .fetch_one(&pool)
            .await
            .unwrap();
        if current != t0 {
            break;
        }
    }

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
    let t0: DateTime<Utc> = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT updated_at FROM sprints WHERE id = ?1",
    ))
    .bind(&sprint_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Retried (bounded) -- see this module's doc comment.
    let mut current = t0;
    for attempt in 0.. {
        assert!(
            attempt < 20,
            "the trigger never produced a token distinct from t0"
        );
        sprints::update_guarded(
            &pool,
            &sprint_id,
            "Sprint 1 renamed",
            None,
            starts_on,
            ends_on,
            Some(current),
        )
        .await
        .expect("first edit");
        current = sqlx::query_scalar(&crate::dialect::sql(
            "SELECT updated_at FROM sprints WHERE id = ?1",
        ))
        .bind(&sprint_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        if current != t0 {
            break;
        }
    }

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
    let name: String = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT name FROM sprints WHERE id = ?1",
    ))
    .bind(&sprint_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        name, "Sprint 1 renamed",
        "the refused write must not have applied"
    );

    let t1: DateTime<Utc> = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT updated_at FROM sprints WHERE id = ?1",
    ))
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
    let t0: DateTime<Utc> = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT updated_at FROM sprints WHERE id = ?1",
    ))
    .bind(&sprint_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Retried (bounded) -- see this module's doc comment.
    let mut current = t0;
    for attempt in 0.. {
        assert!(
            attempt < 20,
            "the trigger never produced a token distinct from t0"
        );
        sprints::update_guarded(
            &pool,
            &sprint_id,
            "Sprint 1 renamed",
            None,
            starts_on,
            ends_on,
            Some(current),
        )
        .await
        .expect("edit before the delete attempt");
        current = sqlx::query_scalar(&crate::dialect::sql(
            "SELECT updated_at FROM sprints WHERE id = ?1",
        ))
        .bind(&sprint_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        if current != t0 {
            break;
        }
    }

    let outcome = sprints::delete_guarded(&pool, &sprint_id, Some(t0))
        .await
        .expect("guarded call itself must not error");
    assert!(
        matches!(outcome, Guarded::Stale { .. }),
        "a stale millisecond token must refuse the delete, got {outcome:?}"
    );
    let still_there: Option<String> =
        sqlx::query_scalar(&crate::dialect::sql("SELECT id FROM sprints WHERE id = ?1"))
            .bind(&sprint_id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(
        still_there.is_some(),
        "the refused delete must not have applied"
    );

    let t1: DateTime<Utc> = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT updated_at FROM sprints WHERE id = ?1",
    ))
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
    let gone: Option<String> =
        sqlx::query_scalar(&crate::dialect::sql("SELECT id FROM sprints WHERE id = ?1"))
            .bind(&sprint_id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert_eq!(gone, None, "the accepted delete must have applied");

    pool.close().await;
    cleanup(&path);
}

// `CON-002` found that `user_capacities::insert` -- this test's own
// seed step -- opened `pool.begin_with("BEGIN IMMEDIATE")` directly,
// SQLite transaction-mode syntax with no PostgreSQL equivalent
// spelling (`syntax error at or near "IMMEDIATE"`, measured in CI),
// shared by fourteen production call sites across `user_capacities.rs`
// (3), `issues.rs` (4), `sprints.rs` (5) and `teams.rs` (2). `CON-005`
// closed it: every one of the fourteen now opens its transaction
// through `dialect::begin_guarded` and takes whichever lock its own
// classified shape needs (`dialect`'s own module doc).
//
// **A second, unrelated defect, found the moment that one was out of
// the way, by `CON-005`, closed here by `CON-006`**: `migrations-
// postgres/0009_user_capacities.sql`'s `points` column was `INTEGER`
// (PostgreSQL's own 4-byte type), a mechanical port of the SQLite
// migration's own `INTEGER` -- which in SQLite is a type *affinity*,
// not a fixed width, and holds any integer value transparently.
// Every Rust binding of this column is `i64` (`user_capacities::
// insert`'s own `points: i64` parameter, carried through
// `overlaps_existing`'s `ConflictInfo`), so the first real
// round-trip read against PostgreSQL -- this test's own legitimate
// edit, through `update_guarded` -- used to decode it and fail:
// `ColumnDecode { index: "points", source: "mismatched types; Rust
// type i64 (as SQL type INT8) is not compatible with SQL type INT4"
// }`. `CON-006` fixed the invariant -- every integer column in
// `migrations-postgres/` is now `BIGINT` -- rather than this one
// column, so the `#[ignore]` `CON-005` left here is deleted, not
// widened.
#[tokio::test]
async fn user_capacities_delete_guard_refuses_a_stale_millisecond_token_and_accepts_the_current_one()
 {
    let (pool, path) = migrated_pool().await;
    seed_user(&pool, "u").await;
    let user = requester("u");
    let capacity_id = user_capacities::insert(&pool, &user, 10, None, None, None)
        .await
        .expect("seed capacity row");
    let t0: DateTime<Utc> = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT updated_at FROM user_capacities WHERE id = ?1",
    ))
    .bind(&capacity_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Legitimate edit, through the update function's own guarded form,
    // so the row's token moves past t0. Retried (bounded) -- see this
    // module's doc comment.
    let mut current = t0;
    for attempt in 0.. {
        assert!(
            attempt < 20,
            "the trigger never produced a token distinct from t0"
        );
        user_capacities::update_guarded(
            &pool,
            &user,
            &capacity_id,
            12,
            None,
            None,
            None,
            Some(current),
        )
        .await
        .expect("edit before the delete attempt");
        current = sqlx::query_scalar(&crate::dialect::sql(
            "SELECT updated_at FROM user_capacities WHERE id = ?1",
        ))
        .bind(&capacity_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        if current != t0 {
            break;
        }
    }

    let outcome = user_capacities::delete_guarded(&pool, &user, &capacity_id, Some(t0))
        .await
        .expect("guarded call itself must not error");
    assert!(
        matches!(outcome, Guarded::Stale { .. }),
        "a stale millisecond token must refuse the delete, got {outcome:?}"
    );
    let still_there: Option<String> = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT id FROM user_capacities WHERE id = ?1",
    ))
    .bind(&capacity_id)
    .fetch_optional(&pool)
    .await
    .unwrap();
    assert!(
        still_there.is_some(),
        "the refused delete must not have applied"
    );

    let t1: DateTime<Utc> = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT updated_at FROM user_capacities WHERE id = ?1",
    ))
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
    let gone: Option<String> = sqlx::query_scalar(&crate::dialect::sql(
        "SELECT id FROM user_capacities WHERE id = ?1",
    ))
    .bind(&capacity_id)
    .fetch_optional(&pool)
    .await
    .unwrap();
    assert_eq!(gone, None, "the accepted delete must have applied");

    pool.close().await;
    cleanup(&path);
}
