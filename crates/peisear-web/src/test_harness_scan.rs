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
//! directory internally and this crate's test harnesses never call
//! `create_dir_all` for any other reason). Narrower than a general
//! heuristic and deliberately so: this checks a specific, known
//! regression shape in two named files, not an open-ended pattern.

use std::fs;
use std::path::Path;

/// Files this guard watches, relative to `peisear-web`'s
/// `CARGO_MANIFEST_DIR`. `..` reaches the sibling crate — the
/// workspace's `crates/*` layout is a load-bearing assumption this
/// crate already makes elsewhere (`prose_scan.rs` reaches `src/`
/// under the same crate only; this is the first place a scan
/// reaches across a crate boundary, and it does so because the
/// defect it watches for was found in two crates, not because a
/// general pattern demands it).
const WATCHED_FILES: &[&str] = &[
    "tests/common/server.rs",
    "../peisear-notify/tests/dispatch_integration.rs",
];

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

#[test]
fn test_harnesses_do_not_derive_temp_paths_from_the_clock() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut offenders = Vec::new();

    for rel in WATCHED_FILES {
        let path = manifest_dir.join(rel);
        let source =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        if constructs_a_path_from_the_clock(&source) {
            offenders.push(*rel);
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
