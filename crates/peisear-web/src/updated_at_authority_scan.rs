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
//! **The needle is the literal write, not the column name.**
//! `updated_at` appears throughout `peisear-storage/src` in `SELECT`
//! and `WHERE` clauses reading the column, which are not the defect;
//! only `updated_at = CURRENT_TIMESTAMP` — the write — is banned. The
//! same phrase inside the trigger bodies themselves lives in
//! `migrations/*.sql`, a different directory this scan does not walk,
//! so the triggers' own `SET updated_at = CURRENT_TIMESTAMP` is not a
//! false positive.

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
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        if source.contains("updated_at = CURRENT_TIMESTAMP") {
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
