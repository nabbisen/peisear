use super::*;
use std::path::Path;

fn migrations_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations")
}

/// The live check: the real migrations and the real known-code list
/// satisfy both directions of `DIA-001` §3's guard.
#[test]
fn the_real_migrations_and_the_real_codes_agree_both_ways() {
    let messages = all_raise_messages(&migrations_dir());
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
    let mut messages = all_raise_messages(&migrations_dir());
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
    let messages = all_raise_messages(&migrations_dir());
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

/// The direction direction 1 cannot catch: a code the registry lists
/// that no migration raises at all -- dead rather than mismatched.
#[test]
fn direction_2_catches_a_known_code_no_migration_raises() {
    let messages = all_raise_messages(&migrations_dir());
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

/// `trigger_error_code`'s own shape: the bracket must open at the
/// anywhere in the message, not only at the very start -- `sqlx::Error`'s
/// own `Display` wraps the driver message in `"error returned from
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
