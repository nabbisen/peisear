use super::*;
use crate::StorageError;
use std::path::Path;

/// A fresh, fully-migrated database, file-backed (not `:memory:` --
/// [`crate::pool::connect`]'s own default pool has up to 8
/// connections, and SQLite's `:memory:` is a distinct database per
/// connection unless a shared-cache URI is used, which nothing here
/// needs to reach for).
async fn migrated_pool() -> Pool {
    let path = std::env::temp_dir().join(format!("peisear-dia001-r3-{}.db", uuid::Uuid::new_v4()));
    let pool = crate::pool::connect(&format!("sqlite://{}", path.display()))
        .await
        .expect("connect");
    crate::pool::migrate(&pool).await.expect("migrate");
    pool
}

/// The live check: a real database's `sqlite_master` and the real
/// known-code list satisfy both directions of `DIA-001` §3/§13's
/// guard.
#[tokio::test]
async fn the_live_triggers_and_the_real_codes_agree_both_ways() {
    let pool = migrated_pool().await;
    let messages = live_trigger_raise_messages(&pool).await;
    assert!(
        !messages.is_empty(),
        "found no RAISE messages at all -- the marker or sqlite_master's own shape has drifted"
    );
    let a = every_raise_code_is_known(&messages, TRIGGER_ERROR_CODES);
    assert!(a.is_empty(), "direction 1: {a:?}");
    let b = every_known_code_appears_in_a_live_trigger(&messages, TRIGGER_ERROR_CODES);
    assert!(b.is_empty(), "direction 2: {b:?}");
}

/// Plant 1 (the handoff's own example): a `RAISE` with a code nothing
/// knows. Caught by direction 1, against the real live triggers' own
/// messages plus one planted on top -- not a committed file edit.
#[tokio::test]
async fn direction_1_catches_a_raise_with_an_unknown_code() {
    let pool = migrated_pool().await;
    let mut messages = live_trigger_raise_messages(&pool).await;
    messages.push("[E_BOGUS] this code is not in the registry".to_string());
    let v = every_raise_code_is_known(&messages, TRIGGER_ERROR_CODES);
    assert!(v.iter().any(|m| m.contains("E_BOGUS")), "got {v:?}");
}

/// A `RAISE` with no bracket at all is also direction 1's to catch --
/// not a silent pass just because there was nothing to compare. Pure
/// strings, no database needed for this one case.
#[test]
fn direction_1_catches_a_raise_with_no_code_prefix() {
    let messages = vec!["no bracket here at all".to_string()];
    let v = every_raise_code_is_known(&messages, TRIGGER_ERROR_CODES);
    assert!(
        v.iter().any(|m| m.contains("no [CODE] prefix")),
        "got {v:?}"
    );
}

/// Plant 2 (the handoff's own example): remove a code the translation
/// knows. Planted on the `known` side rather than edited into
/// `issues.rs`, against the real live triggers' own messages -- the
/// removed code is still raised, so direction 1 now calls it unknown.
#[tokio::test]
async fn direction_1_catches_a_known_code_removed_from_the_registry() {
    let pool = migrated_pool().await;
    let messages = live_trigger_raise_messages(&pool).await;
    let shrunk: Vec<(&str, peisear_i18n::MessageKey)> = TRIGGER_ERROR_CODES
        .iter()
        .filter(|(c, _)| *c != "E_ISSUE_SELF_PARENT")
        .cloned()
        .collect();
    let v = every_raise_code_is_known(&messages, &shrunk);
    assert!(
        v.iter().any(|m| m.contains("E_ISSUE_SELF_PARENT")),
        "got {v:?}"
    );
}

/// The direction that direction 1 cannot catch: a code the registry
/// lists that no live trigger raises at all -- dead rather than
/// mismatched. This is also the shape of the review's own named gap
/// ("a trigger dropped and never re-created"): dropping a trigger and
/// widening the registry produce the identical observable fact, a
/// known code nothing in `sqlite_master` raises, so one plant covers
/// both.
#[tokio::test]
async fn direction_2_catches_a_known_code_no_live_trigger_raises() {
    let pool = migrated_pool().await;
    let messages = live_trigger_raise_messages(&pool).await;
    let mut widened: Vec<(&str, peisear_i18n::MessageKey)> = TRIGGER_ERROR_CODES.to_vec();
    widened.push((
        "E_NEVER_RAISED",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ));
    let v = every_known_code_appears_in_a_live_trigger(&messages, &widened);
    assert!(v.iter().any(|m| m.contains("E_NEVER_RAISED")), "got {v:?}");
}

/// The gap a synthetic-string-only suite cannot see: `sqlx::Error`'s
/// real `Display` wraps the driver message, which is exactly what
/// broke `trigger_error_code`'s first version (anchored at position
/// 0) against a real trigger firing, caught only by
/// `peisear-web`'s own integration tests. This fires a real trigger
/// against a real database and asserts on `translate_trigger_error`'s
/// actual output, not a string this test constructs by hand --
/// closing that gap at the crate that owns the function, not only at
/// the crate two layers up that happened to exercise it end to end.
#[tokio::test]
async fn translate_trigger_error_maps_a_real_self_parent_violation() {
    let pool = migrated_pool().await;
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
    sqlx::query(
        "INSERT INTO issues (id, project_id, author_id, title, status) \
         VALUES ('i', 'p', 'u', 'T', 'open')",
    )
    .execute(&pool)
    .await
    .unwrap();

    let err = sqlx::query("UPDATE issues SET parent_issue_id = 'i' WHERE id = 'i'")
        .execute(&pool)
        .await
        .expect_err("self-parent must be rejected");
    let translated = translate_trigger_error(err);
    let StorageError::Validation(key) = translated else {
        panic!(
            "expected StorageError::Validation, got {translated:?} -- a real trigger firing \
             must translate the same way the synthetic-string tests above assume it does"
        );
    };
    assert_eq!(key, peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage);
}

/// Every trigger `sqlite_master` records, as `(name, sql)` pairs --
/// the general form `DIA-001` §13 asks for, covering however many
/// triggers exist rather than a hard-coded list of four. Includes
/// triggers unrelated to this round (the `updated_at`-authority ones
/// from `0014`/`0017`/`0021`) on purpose: the upgrade-vs-fresh
/// comparison below is a claim about the whole schema reaching the
/// same state, not only the five this round's own guard cares about.
async fn all_trigger_sql(pool: &Pool) -> std::collections::BTreeMap<String, String> {
    let rows: Vec<(String, String)> =
        sqlx::query_as("SELECT name, sql FROM sqlite_master WHERE type = 'trigger'")
            .fetch_all(pool)
            .await
            .expect("read sqlite_master triggers");
    rows.into_iter().collect()
}

/// `DIA-001` round 2 §10/§11: the restoration's whole point, proven
/// rather than argued. A database carrying only `0015`/`0016`'s
/// restored, `0.53.0`-era content (built from a scratch copy of
/// `migrations/` with `0022` removed, so the compile-time
/// `sqlx::migrate!` embed in [`crate::pool::migrate`] is never asked
/// to apply anything newer) is then upgraded by running this crate's
/// real, current migrator against it -- the same function a real
/// deployment would call. A second, fresh database reaches the same
/// end state by running the full chain from empty. Both must exist to
/// start (neither `Migrator::run` nor `pool::migrate` panicking is
/// itself part of what this test asserts), and both must carry
/// byte-identical trigger definitions once there -- not merely "both
/// started," which the restoration alone would already guarantee.
///
/// **Round 3**: the four-name loop this test used to run is gone,
/// replaced by comparing every trigger `sqlite_master` has in each
/// database -- the same reasoning `live_trigger_raise_messages` used
/// to drop its own file-scoped predecessor. The per-trigger `[E_`
/// check that used to live here is not repeated: it is
/// `the_live_triggers_and_the_real_codes_agree_both_ways`'s own job,
/// against the one invariant this test exists to prove (upgraded and
/// fresh reach the *same* state, whatever that state is).
#[tokio::test]
async fn an_upgrade_from_a_0_53_0_era_database_and_a_fresh_one_reach_identical_triggers() {
    let real_migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let old_dir = std::env::temp_dir().join(format!(
        "peisear-dia001-old-migrations-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&old_dir).expect("create scratch migrations dir");
    for entry in std::fs::read_dir(&real_migrations_dir)
        .expect("read migrations/")
        .flatten()
    {
        let name = entry.file_name();
        if name.to_string_lossy() == "0022_trigger_error_codes.sql" {
            continue;
        }
        std::fs::copy(entry.path(), old_dir.join(&name)).expect("copy migration");
    }

    let old_db_path = std::env::temp_dir().join(format!(
        "peisear-dia001-upgrade-{}.db",
        uuid::Uuid::new_v4()
    ));
    let upgraded_pool = crate::pool::connect(&format!("sqlite://{}", old_db_path.display()))
        .await
        .expect("connect");
    sqlx::migrate::Migrator::new(old_dir)
        .await
        .expect("build 0.53.0-era migrator")
        .run(&upgraded_pool)
        .await
        .expect("apply 0.53.0-era migrations (0001-0021)");
    // The upgrade itself: the current, compile-time migrator (which
    // embeds 0001-0022) against a database that already has 0001-0021
    // applied and recorded. If 0015/0016 were not byte-identical to
    // what that database already ran, this is where it would refuse.
    crate::pool::migrate(&upgraded_pool)
        .await
        .expect("upgrade a 0.53.0-era database to 0022");

    let fresh_pool = migrated_pool().await;

    let upgraded_triggers = all_trigger_sql(&upgraded_pool).await;
    let fresh_triggers = all_trigger_sql(&fresh_pool).await;
    assert!(
        !upgraded_triggers.is_empty(),
        "found no triggers at all -- sqlite_master's own shape has drifted"
    );
    assert_eq!(
        upgraded_triggers, fresh_triggers,
        "the upgraded (0.53.0-era, then migrated to 0022) and fresh (migrated from empty) \
         databases carry different triggers"
    );
}

/// `trigger_error_code`'s own shape: the bracket is found anywhere in
/// the message, not only at the very start -- `sqlx::Error`'s own
/// `Display` wraps the driver message in `"error returned from
/// database: {0}"`, so the bracket this function is actually asked to
/// find in a real error never opens at position 0.
#[test]
fn trigger_error_code_finds_the_bracket_anywhere_in_the_message() {
    assert_eq!(trigger_error_code("[E_FOO] bar"), Some("E_FOO"));
    assert_eq!(
        trigger_error_code("error returned from database: [E_FOO] bar"),
        Some("E_FOO")
    );
    assert_eq!(trigger_error_code("no bracket"), None);
}

/// `PG-003` round 2, `§8`/`§9`: `TRIGGER_ERROR_CODES`'s own
/// `MessageKey`s, in order, equal the one shared list both backends'
/// tables are checked against -- not each other, which the mutually
/// exclusive `sqlite`/`postgres` features never allow in one test
/// anyway.
#[test]
fn trigger_error_codes_match_the_shared_validation_list() {
    assert!(message_keys_match_the_shared_list(TRIGGER_ERROR_CODES));
}

/// `§10`'s own instruction: the new assertion seen to fail, not just
/// written. A sixth entry added to this table alone, with no matching
/// entry in the shared list, must be caught -- planted on a cloned
/// copy, never on the committed `const`.
#[test]
fn a_sixth_entry_added_to_only_this_table_is_caught() {
    let mut widened = TRIGGER_ERROR_CODES.to_vec();
    widened.push((
        "E_PLANTED",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ));
    assert!(!message_keys_match_the_shared_list(&widened));
}
