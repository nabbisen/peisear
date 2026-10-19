use super::*;
use crate::StorageError;

/// Unwraps the `MessageKey` out of a `StorageError::Validation`,
/// panicking with the actual value otherwise -- so a validation that
/// translates to the wrong `StorageError` variant fails loudly
/// rather than comparing `None == None`.
fn validation_key(e: StorageError) -> peisear_i18n::MessageKey {
    let StorageError::Validation(key) = e else {
        panic!("expected StorageError::Validation, got {e:?}");
    };
    key
}

/// A fresh, fully-migrated PostgreSQL database. Reads `DATABASE_URL`,
/// the same convention every other real-server test in this crate
/// uses (`PG-001`'s own `pool::tests` precedent) -- CI's service
/// container, not a local one.
async fn migrated_pool() -> Pool {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must point at a real PostgreSQL server for this test");
    let pool = crate::pool::connect(&url).await.expect("connect");
    crate::pool::migrate(&pool).await.expect("migrate");
    pool
}

/// `PG-003` §1: eleven triggers, measured against a live, migrated
/// database rather than assumed from the migration files -- the
/// same discipline `DIA-001` round 3 applied to SQLite, now against
/// `pg_trigger` (`NOT tgisinternal`, so this project's own foreign
/// keys' internal RI triggers are excluded, not counted as this
/// schema's own).
#[tokio::test]
async fn eleven_triggers_exist_and_none_of_them_are_foreign_key_internals() {
    let pool = migrated_pool().await;
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM pg_trigger WHERE NOT tgisinternal")
        .fetch_one(&pool)
        .await
        .expect("read pg_trigger");
    assert_eq!(
        count, 11,
        "expected the same eleven triggers SQLite carries"
    );
}

/// `PG-003` §2's own both-directions guard, run against the real
/// database: the live `ERRCODE`s found and the real registry agree,
/// both ways.
#[tokio::test]
async fn the_live_errcodes_and_the_real_registry_agree_both_ways() {
    let pool = migrated_pool().await;
    let codes = live_trigger_errcodes_postgres(&pool).await;
    assert!(
        !codes.is_empty(),
        "found no ERRCODEs at all -- the marker or pg_proc's own shape has drifted"
    );
    let a = every_errcode_is_known_postgres(&codes, TRIGGER_ERROR_CODES_POSTGRES);
    assert!(a.is_empty(), "direction 1: {a:?}");
    let b = every_known_errcode_appears_in_a_live_trigger_postgres(
        &codes,
        TRIGGER_ERROR_CODES_POSTGRES,
    );
    assert!(b.is_empty(), "direction 2: {b:?}");
}

/// Plant 1, the PostgreSQL side of `DIA-001`'s own example: an
/// `ERRCODE` nothing in the registry knows. Planted on the live
/// codes, not a committed file edit.
#[tokio::test]
async fn direction_1_catches_an_errcode_with_no_known_registry_entry() {
    let pool = migrated_pool().await;
    let mut codes = live_trigger_errcodes_postgres(&pool).await;
    codes.push("ZZ999".to_string());
    let v = every_errcode_is_known_postgres(&codes, TRIGGER_ERROR_CODES_POSTGRES);
    assert!(v.iter().any(|m| m.contains("ZZ999")), "got {v:?}");
}

/// Plant 2: remove a code the translation knows, planted on the
/// `known` side rather than edited into the migration -- the removed
/// code is still raised live, so direction 1 now calls it unknown.
#[tokio::test]
async fn direction_1_catches_a_known_code_removed_from_the_registry() {
    let pool = migrated_pool().await;
    let codes = live_trigger_errcodes_postgres(&pool).await;
    let shrunk: Vec<(&str, peisear_i18n::MessageKey)> = TRIGGER_ERROR_CODES_POSTGRES
        .iter()
        .filter(|(c, _)| *c != "PE003")
        .cloned()
        .collect();
    let v = every_errcode_is_known_postgres(&codes, &shrunk);
    assert!(v.iter().any(|m| m.contains("PE003")), "got {v:?}");
}

/// Direction 2: a code the registry lists that no live trigger raises
/// at all -- dead rather than mismatched.
#[tokio::test]
async fn direction_2_catches_a_known_code_no_live_trigger_raises() {
    let pool = migrated_pool().await;
    let codes = live_trigger_errcodes_postgres(&pool).await;
    let mut widened: Vec<(&str, peisear_i18n::MessageKey)> = TRIGGER_ERROR_CODES_POSTGRES.to_vec();
    widened.push((
        "ZZ998",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ));
    let v = every_known_errcode_appears_in_a_live_trigger_postgres(&codes, &widened);
    assert!(v.iter().any(|m| m.contains("ZZ998")), "got {v:?}");
}

async fn seed_one_issue(pool: &Pool, issue_id: &str) {
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, display_name) \
         VALUES ('u', 'u@example.test', 'x', 'U') ON CONFLICT (id) DO NOTHING",
    )
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO projects (id, owner_id, name) VALUES ('p', 'u', 'P') ON CONFLICT (id) DO NOTHING")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(&format!(
        "INSERT INTO issues (id, project_id, author_id, title, status) \
         VALUES ('{issue_id}', 'p', 'u', 'T', 'open')"
    ))
    .execute(pool)
    .await
    .unwrap();
}

/// `PG-003` §3: "the same operation, rejected on both backends,
/// producing the same `MessageKey`." This is the PostgreSQL half of
/// `dialect::tests::translate_trigger_error_maps_a_real_self_parent_violation`
/// -- same scenario, same assertion, the other pool. A rejection for
/// a different reason would not be agreement; asserting the
/// `MessageKey`, not merely that an error occurred, is what rules
/// that out.
#[tokio::test]
async fn translate_trigger_error_maps_a_real_self_parent_violation() {
    let pool = migrated_pool().await;
    seed_one_issue(&pool, "i-self-parent").await;

    let err = sqlx::query(
        "UPDATE issues SET parent_issue_id = 'i-self-parent' WHERE id = 'i-self-parent'",
    )
    .execute(&pool)
    .await
    .expect_err("self-parent must be rejected");
    let translated = translate_trigger_error(err);
    let StorageError::Validation(key) = translated else {
        panic!("expected StorageError::Validation, got {translated:?}");
    };
    assert_eq!(key, peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage);
}

/// The remaining four validations, each fired for real and asserted
/// on `MessageKey` -- `§3`'s "same assertion, two pools" for every
/// one of the five, not only the self-parent case the SQLite suite
/// already covers in detail.
#[tokio::test]
async fn translate_trigger_error_maps_all_four_remaining_validations() {
    let pool = migrated_pool().await;

    // E_SUB_ISSUE_DEPTH: a sub-issue of a sub-issue.
    seed_one_issue(&pool, "i-parent-1").await;
    sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status, parent_issue_id) \
         VALUES ('i-child-1', 'p', 'u', 'T', 'open', 'i-parent-1')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let err = sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status, parent_issue_id) \
         VALUES ('i-grandchild-1', 'p', 'u', 'T', 'open', 'i-child-1')",
    )
    .execute(&pool)
    .await
    .expect_err("a sub-issue of a sub-issue must be rejected");
    assert_eq!(
        validation_key(translate_trigger_error(err)),
        peisear_i18n::MessageKey::SubIssueCannotHaveSubIssueMessage
    );

    // E_SUB_ISSUE_PROJECT_MISMATCH: parent in a different project.
    sqlx::query("INSERT INTO projects (id, owner_id, name) VALUES ('p2', 'u', 'P2')")
        .execute(&pool)
        .await
        .unwrap();
    let err = sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status, parent_issue_id) \
         VALUES ('i-cross-project', 'p2', 'u', 'T', 'open', 'i-parent-1')",
    )
    .execute(&pool)
    .await
    .expect_err("a cross-project parent must be rejected");
    assert_eq!(
        validation_key(translate_trigger_error(err)),
        peisear_i18n::MessageKey::SubIssueMustShareProjectMessage
    );

    // E_ISSUE_DEMOTE_HAS_SUBISSUES: demoting a parent that has children.
    // The new parent must itself be top-level (no parent of its own),
    // or the depth check (PE001) fires first -- same precedence the
    // SQLite original has, checked in the same order here.
    sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status) \
         VALUES ('i-other-top', 'p', 'u', 'T', 'open')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let err =
        sqlx::query("UPDATE issues SET parent_issue_id = 'i-other-top' WHERE id = 'i-parent-1'")
            .execute(&pool)
            .await
            .expect_err("demoting an issue with children must be rejected");
    assert_eq!(
        validation_key(translate_trigger_error(err)),
        peisear_i18n::MessageKey::CannotDemoteIssueWithSubIssuesMessage
    );

    // E_ISSUE_PLANNED_DATE_ORDER: end before start.
    seed_one_issue(&pool, "i-dates").await;
    let err = sqlx::query(
        "UPDATE issues SET planned_start_at = '2026-01-02', planned_end_at = '2026-01-01' \
         WHERE id = 'i-dates'",
    )
    .execute(&pool)
    .await
    .expect_err("end before start must be rejected");
    assert_eq!(
        validation_key(translate_trigger_error(err)),
        peisear_i18n::MessageKey::IssuePlannedEndBeforeStartMessage
    );
}

/// `0021`'s own doc comment, demonstrated rather than assumed:
/// `clock_timestamp()` advances between two statements in the same
/// transaction, where `CURRENT_TIMESTAMP`/`NOW()` would not --
/// exactly the shape of the whole-second defect `0021` fixes on
/// SQLite, which this project chose not to reintroduce at the
/// transaction level on PostgreSQL.
#[tokio::test]
async fn two_writes_in_one_postgres_transaction_still_advance_the_lock_token() {
    let pool = migrated_pool().await;
    seed_one_issue(&pool, "i-token").await;

    let mut tx = pool.begin().await.expect("begin");
    sqlx::query("UPDATE issues SET title = 'T1' WHERE id = 'i-token'")
        .execute(&mut *tx)
        .await
        .unwrap();
    let first: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT updated_at FROM issues WHERE id = 'i-token'")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    sqlx::query("UPDATE issues SET title = 'T2' WHERE id = 'i-token'")
        .execute(&mut *tx)
        .await
        .unwrap();
    let second: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT updated_at FROM issues WHERE id = 'i-token'")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    tx.commit().await.expect("commit");

    assert_ne!(
        first, second,
        "two writes to the same row in one transaction must not share a lock token -- \
         CURRENT_TIMESTAMP would have frozen both at the transaction's own start time"
    );
}
