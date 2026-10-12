//! `QA-019` §5 (`NFR-CONC-003`, RFC 005 §8) — `updated_at` has exactly
//! one authority: the database triggers `0014_updated_at_columns.sql`
//! and `0017_updated_at_single_authority.sql` define, not application
//! code. `QA-018`'s audit found four sites across three tables
//! (`issues`, `projects`, `user_view_states`) writing the column
//! directly, with no trigger backing them — a silent optimistic-lock
//! bypass class, the same shape `§10.6` was. `QA-019` removed all
//! four and gave the three tables triggers of their own. This keeps
//! the class from coming back.
//!
//! **Rests on a schema fact, not a pinned third-party CSS version**
//! (`contrast_scan`/`touch_target_scan`'s own shape) — the trigger
//! migrations are the authority this scan protects, so it cites them
//! rather than a version number.
//!
//! **Scans `peisear-storage/src/`, a different crate from this
//! module's own.** Same cross-crate read `dec_007_ci_scan` already
//! does for `.github/workflows/test.yml` — a relative path from
//! `CARGO_MANIFEST_DIR`, not an assumption about the current working
//! directory.
//!
//! **The needle, widened at `DOC-007`.** `updated_at` appears
//! throughout `peisear-storage/src` in `SELECT` and `WHERE` clauses
//! reading the column, which are not the defect; only a `SET
//! updated_at = …` write is banned, and the needle used to spell that
//! one way: `updated_at = CURRENT_TIMESTAMP`. `SET updated_at = ?`,
//! `= datetime('now')`, or a literal all passed it, and `0021` made
//! the likeliest *future* mistake the one spelling it still could not
//! see — the migrations' own triggers now say `strftime(…)`, so a
//! slip that copies that idiom into application code is exactly the
//! shape nobody was checking for. `SET updated_at =` matches any
//! right-hand side. The same phrase inside the trigger bodies
//! themselves lives in `migrations/*.sql`, a different directory this
//! scan does not walk, so the triggers' own `SET updated_at = …` is
//! not a false positive.
//!
//! **Excludes test paths, by path rather than by name, since
//! `DOC-007` §1.** `*/tests.rs` and `*/tests/*` hold exactly the kind
//! of planted `UPDATE … SET updated_at = …` a test proving the lock
//! legitimately needs (`pool/tests.rs`'s own plant, from `REG-001`) —
//! indistinguishable from a real violation by content alone, which is
//! why this waited for kind 1's move: before it, `optimistic_lock_
//! guards.rs` carried no such plant and excluding by name would have
//! meant guessing at a list rather than reading a path. Checked
//! directly, not assumed, that no kind-2 file left in place needs an
//! entry of its own: `peisear-storage/src/`'s only `SET updated_at =`
//! occurrence anywhere is the one plant, already under `pool/tests.rs`.

use std::fs;
use std::path::{Path, PathBuf};

/// Every `.rs` file under `dir`, recursively — the same walk
/// `prose_scan`/`contrast_scan`/`touch_target_scan` use.
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

/// True if `path` is test code by `DOC-007`'s own convention: a file
/// literally named `tests.rs`, or any file under a directory literally
/// named `tests`. Path-based rather than name-based on purpose — a
/// future test file does not need adding to a list, it only needs to
/// live where the convention says it does.
fn is_test_path(path: &Path) -> bool {
    if path.file_name().is_some_and(|n| n == "tests.rs") {
        return true;
    }
    path.components()
        .any(|c| c.as_os_str() == std::ffi::OsStr::new("tests"))
}

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
