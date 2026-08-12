//! `QA-001-review.md` §3.1 — a deterministic guard for the property
//! the repeated-run gate in `CONTRIBUTING.md` was standing in for
//! and cannot reliably detect: **the test harness must not derive a
//! temporary path from the clock.**
//!
//! The review measured the repeated `cargo test --workspace` gate's
//! detection rate directly (not reasoned about it) and found it is a
//! property of machine load, not of the defect: 3/6 failures on a
//! loaded machine, 0/6 on a quiet one, with the harness bug fully
//! present both times. A probabilistic gate cannot protect a
//! deterministic property. `TestApp::spawn`
//! (`peisear-web/tests/common/server.rs`) and `fresh_pool`
//! (`peisear-notify/tests/dispatch_integration.rs`) both had this
//! defect in the identical shape — this is `prose_scan.rs`'s
//! source-scanning pattern pointed at test infrastructure instead of
//! product copy.
//!
//! **Signature checked**: a test-harness file that calls
//! `SystemTime::now()` *and* `create_dir_all` is presumed to be
//! building a directory name from the clock — the exact shape both
//! historical defects took, and not a shape any legitimate use in
//! these files takes (`peisear-notify`'s `create_user_subscribed_to_
//! email` also calls `SystemTime::now()`, for a synthetic user id,
//! but never calls `create_dir_all`; `tempfile::TempDir` creates its
//! directory internally, so a harness that already migrated to it
//! never calls `create_dir_all` for any reason).
//!
//! **`QA-001-corrections-review.md` §3**: the first version of this
//! guard watched two named files. The property holds over every test
//! harness, not two chosen ones — the same defect already appeared
//! independently in two crates, which is evidence about how it
//! spreads, not about how many files it lives in. Now walks
//! `crates/*/tests/**.rs` workspace-wide instead. The signature is
//! narrow enough that this costs nothing: a comment-stripped scan
//! over all matching files (23, at review time) returns zero hits
//! outside the two files that ever had the defect.

use std::fs;
use std::path::{Path, PathBuf};

/// Strips `//`-style line comments (plain, `///` doc comments, and
/// `//!` inner doc comments alike -- all start with `//`) so a
/// comment that *mentions* the pattern this scan looks for, the way
/// this very file's own doc comment does, is not mistaken for code
/// that uses it. Line-based and does not account for `//` appearing
/// inside a string literal; no file in scope has that today, and
/// `prose_scan.rs` carries the same class of documented limitation.
fn strip_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn constructs_a_path_from_the_clock(source: &str) -> bool {
    let code = strip_line_comments(source);
    code.contains("SystemTime::now()") && code.contains("create_dir_all")
}

/// Every `.rs` file under `dir`, recursively.
fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Every `.rs` file under any workspace crate's `tests/` directory --
/// `crates/*/tests/**.rs`, reached via `CARGO_MANIFEST_DIR/../..`
/// from `peisear-web` (`.../crates/peisear-web` -> `.../crates`).
/// Crates without a `tests/` directory (`peisear-core`, `peisear-
/// auth`, `peisear`) are silently skipped, same as a directory with
/// no matches would be.
fn all_test_harness_files() -> Vec<PathBuf> {
    let crates_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(&crates_dir) else {
        panic!("read crates dir {}", crates_dir.display());
    };
    for entry in entries {
        let crate_dir = entry.expect("dir entry").path();
        if !crate_dir.is_dir() {
            continue;
        }
        let tests_dir = crate_dir.join("tests");
        if tests_dir.is_dir() {
            collect_rs_files(&tests_dir, &mut out);
        }
    }
    out
}

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
