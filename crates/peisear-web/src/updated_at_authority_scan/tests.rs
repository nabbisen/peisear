use super::*;

#[test]
fn application_code_never_writes_updated_at() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let storage_src = manifest_dir.join("..").join("peisear-storage").join("src");
    let mut files = Vec::new();
    collect_rs_files(&storage_src, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under peisear-storage/src -- the workspace \
         layout assumption this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in &files {
        if is_test_path(path) {
            continue;
        }
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        if source.contains("SET updated_at =") {
            offenders.push(path.display().to_string());
        }
    }

    assert!(
        offenders.is_empty(),
        "NFR-CONC-003: the application MUST NOT write `updated_at` -- database \
         triggers (0014_updated_at_columns.sql, \
         0017_updated_at_single_authority.sql) are its one authority. QA-019 \
         removed every application-layer write; a new one reopens the silent \
         optimistic-lock bypass class QA-018's audit found:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn is_test_path_catches_tests_rs_and_tests_dir() {
    assert!(is_test_path(Path::new("src/pool/tests.rs")));
    assert!(is_test_path(Path::new("src/tests/foo.rs")));
    assert!(!is_test_path(Path::new("src/optimistic_lock_guards.rs")));
    assert!(!is_test_path(Path::new("src/pool.rs")));
}

/// Both halves, `DOC-007` §2: an application write in a real
/// (non-test) file is caught, and the identical statement in a
/// `tests.rs` is not -- planted directly against a scratch
/// directory tree rather than against the real `peisear-storage`, so
/// this test cannot be made to pass by editing the real crate.
#[test]
fn widened_needle_catches_a_real_write_and_excepts_a_planted_test() {
    let dir = std::env::temp_dir().join(format!(
        "peisear-updated-at-scan-plant-{}",
        std::process::id()
    ));
    let pool_dir = dir.join("pool");
    fs::create_dir_all(&pool_dir).expect("create scratch dirs");

    let real_file = dir.join("issues.rs");
    fs::write(
        &real_file,
        "pub fn touch(id: &str) { let _ = format!(\"UPDATE issues SET updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now') WHERE id = '{id}'\"); }",
    )
    .expect("write the planted real-file violation");

    let test_file = pool_dir.join("tests.rs");
    fs::write(
        &test_file,
        "UPDATE issues SET updated_at = '2026-01-01 00:00:23.414' WHERE id = 'i'",
    )
    .expect("write the planted test-file statement");

    let mut files = Vec::new();
    collect_rs_files(&dir, &mut files);
    assert_eq!(files.len(), 2, "expected both planted files to be found");

    let caught: Vec<&PathBuf> = files
        .iter()
        .filter(|p| !is_test_path(p))
        .filter(|p| fs::read_to_string(p).unwrap().contains("SET updated_at ="))
        .collect();

    assert_eq!(
        caught,
        vec![&real_file],
        "the real file must be caught and the test file must not be"
    );

    let _ = fs::remove_dir_all(&dir);
}
