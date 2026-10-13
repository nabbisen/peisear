use super::*;

#[test]
fn test_harnesses_do_not_derive_temp_paths_from_the_clock() {
    let files = all_test_harness_files();
    assert!(
        !files.is_empty(),
        "found no crates/*/tests/**.rs files -- the workspace layout \
         assumption this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in &files {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        if constructs_a_path_from_the_clock(&source) {
            offenders.push(path.display().to_string());
        }
    }

    assert!(
        offenders.is_empty(),
        "these test harness files construct a temp path from SystemTime::now() \
         and create_dir_all -- the QA-001 collision shape (baseline §10.13). \
         Use tempfile::TempDir instead, which is unique by construction and \
         cleans up on drop:\n{}",
        offenders.join("\n")
    );
}
