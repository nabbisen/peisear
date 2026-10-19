use super::*;

fn real_sqlite_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations")
}

fn real_postgres_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations-postgres")
}

/// `PG-002`'s own gate: the two real directories agree today. This
/// is the test that actually protects `§2`'s bound — a future
/// migration landing in one directory and not the other fails this,
/// not a code review.
#[test]
fn the_real_directories_agree() {
    let found = mismatches(&real_sqlite_dir(), &real_postgres_dir());
    assert!(found.is_empty(), "drift found: {found:?}");
}

/// Direction 1: a file planted in `migrations/` alone must be
/// caught, not silently passed because the postgres side has
/// "enough" files.
#[test]
fn a_file_only_in_sqlite_is_caught() {
    let dir = std::env::temp_dir().join(format!("peisear-drift-sqlite-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("0023_planted.sql"), "-- planted").unwrap();

    let found = mismatches(&dir, &real_postgres_dir());
    assert!(
        found
            .iter()
            .any(|m| m.contains("0023_planted.sql")
                && m.contains("missing from migrations-postgres")),
        "expected the planted file to be reported, got: {found:?}"
    );

    std::fs::remove_dir_all(&dir).unwrap();
}

/// Direction 2, the one a check built only for "does postgres have
/// fewer files" would miss: a file planted in `migrations-postgres/`
/// alone, with nothing in `migrations/` at that version at all.
#[test]
fn a_file_only_in_postgres_is_caught() {
    let dir = std::env::temp_dir().join(format!("peisear-drift-postgres-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("0023_planted.sql"), "-- planted").unwrap();

    let found = mismatches(&real_sqlite_dir(), &dir);
    assert!(
        found
            .iter()
            .any(|m| m.contains("0023_planted.sql") && m.contains("missing from migrations/")),
        "expected the planted file to be reported, got: {found:?}"
    );

    std::fs::remove_dir_all(&dir).unwrap();
}

/// Two directories with the same filenames agree, even if this test
/// cannot see their contents -- which is exactly `§2`'s bound: the
/// guard is filename-level, and says so by not noticing a content
/// difference planted here.
#[test]
fn identical_filenames_with_different_content_still_agree() {
    let a = std::env::temp_dir().join(format!("peisear-drift-a-{}", uuid::Uuid::new_v4()));
    let b = std::env::temp_dir().join(format!("peisear-drift-b-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&a).unwrap();
    std::fs::create_dir_all(&b).unwrap();
    std::fs::write(a.join("0001_initial.sql"), "-- sqlite content").unwrap();
    std::fs::write(
        b.join("0001_initial.sql"),
        "-- totally different postgres content",
    )
    .unwrap();

    let found = mismatches(&a, &b);
    assert!(
        found.is_empty(),
        "filename-level agreement must not read content: {found:?}"
    );

    std::fs::remove_dir_all(&a).unwrap();
    std::fs::remove_dir_all(&b).unwrap();
}
