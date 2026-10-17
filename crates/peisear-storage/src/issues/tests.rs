use super::*;
use std::path::Path;

/// `0022` specifically, not the whole `migrations/` directory --
/// `all_raise_messages`'s own doc comment says why: `0015`/`0016` are
/// restored to their shipped `0.53.0` content and carry prose
/// forever; `0022` is the one file whose `RAISE` text describes a
/// live trigger (`DIA-001` round 2).
fn trigger_error_codes_migration() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations/0022_trigger_error_codes.sql")
}

/// The live check: `0022`'s own `RAISE` text and the real known-code
/// list satisfy both directions of `DIA-001` §3's guard.
#[test]
fn the_real_migrations_and_the_real_codes_agree_both_ways() {
    let messages = all_raise_messages(&trigger_error_codes_migration());
    assert!(
        !messages.is_empty(),
        "found no RAISE messages at all -- the marker or the migrations path has drifted"
    );
    let a = every_raise_code_is_known(&messages, TRIGGER_ERROR_CODES);
    assert!(a.is_empty(), "direction 1: {a:?}");
    let b = every_known_code_appears_in_a_migration(&messages, TRIGGER_ERROR_CODES);
    assert!(b.is_empty(), "direction 2: {b:?}");
}

/// Plant 1 (the handoff's own example): a `RAISE` with a code nothing
/// knows. Caught by direction 1, against the real migrations' own
/// messages plus one planted on top -- not a committed file edit.
#[test]
fn direction_1_catches_a_raise_with_an_unknown_code() {
    let mut messages = all_raise_messages(&trigger_error_codes_migration());
    messages.push("[E_BOGUS] this code is not in the registry".to_string());
    let v = every_raise_code_is_known(&messages, TRIGGER_ERROR_CODES);
    assert!(v.iter().any(|m| m.contains("E_BOGUS")), "got {v:?}");
}

/// A `RAISE` with no bracket at all is also direction 1's to catch --
/// not a silent pass just because there was nothing to compare.
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
/// `issues.rs`, against the real migrations' own messages -- the
/// removed code is still raised, so direction 1 now calls it unknown.
#[test]
fn direction_1_catches_a_known_code_removed_from_the_registry() {
    let messages = all_raise_messages(&trigger_error_codes_migration());
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
/// lists that no migration raises at all -- dead rather than
/// mismatched.
#[test]
fn direction_2_catches_a_known_code_no_migration_raises() {
    let messages = all_raise_messages(&trigger_error_codes_migration());
    let mut widened: Vec<(&str, peisear_i18n::MessageKey)> = TRIGGER_ERROR_CODES.to_vec();
    widened.push((
        "E_NEVER_RAISED",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ));
    let v = every_known_code_appears_in_a_migration(&messages, &widened);
    assert!(v.iter().any(|m| m.contains("E_NEVER_RAISED")), "got {v:?}");
}

/// The gap a synthetic-string-only suite cannot see: `sqlx::Error`'s
/// real `Display` wraps the driver message, which is exactly what
/// broke `trigger_error_code`'s first version (anchored at position
/// 0) against a real trigger firing, caught only by
/// `peisear-web`'s own integration tests. This fires a real trigger
/// against a real in-memory database and asserts on
/// `translate_trigger_error`'s actual output, not a string this test
/// constructs by hand -- closing that gap at the crate that owns the
/// function, not only at the crate two layers up that happened to
/// exercise it end to end.
#[tokio::test]
async fn translate_trigger_error_maps_a_real_self_parent_violation() {
    let pool = crate::pool::connect("sqlite::memory:")
        .await
        .expect("connect");
    crate::pool::migrate(&pool).await.expect("migrate");
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

/// `sqlite_master`'s own recorded SQL for a trigger -- the ground
/// truth for "what does this database's trigger actually say," not a
/// re-read of a migration file.
async fn trigger_sql(pool: &Pool, name: &str) -> String {
    sqlx::query_scalar::<_, String>(
        "SELECT sql FROM sqlite_master WHERE type = 'trigger' AND name = ?",
    )
    .bind(name)
    .fetch_one(pool)
    .await
    .unwrap_or_else(|e| panic!("read trigger {name} from sqlite_master: {e}"))
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

    let fresh_db_path =
        std::env::temp_dir().join(format!("peisear-dia001-fresh-{}.db", uuid::Uuid::new_v4()));
    let fresh_pool = crate::pool::connect(&format!("sqlite://{}", fresh_db_path.display()))
        .await
        .expect("connect");
    crate::pool::migrate(&fresh_pool)
        .await
        .expect("migrate a fresh database from empty");

    for trigger in [
        "prevent_sub_issue_nesting_insert",
        "prevent_sub_issue_nesting_update",
        "issues_planned_range_check_insert",
        "issues_planned_range_check_update",
    ] {
        let upgraded_sql = trigger_sql(&upgraded_pool, trigger).await;
        let fresh_sql = trigger_sql(&fresh_pool, trigger).await;
        assert_eq!(
            upgraded_sql, fresh_sql,
            "trigger {trigger} differs between an upgraded 0.53.0-era database and a fresh one"
        );
        assert!(
            upgraded_sql.contains("[E_"),
            "trigger {trigger} does not carry a coded RAISE message after upgrading"
        );
    }
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
