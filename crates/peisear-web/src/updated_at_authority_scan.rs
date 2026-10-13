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

#[cfg(test)]
mod tests;
