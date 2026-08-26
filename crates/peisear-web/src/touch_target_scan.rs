//! `QA-015` §5 (`NFR-A11Y-007`, RFC 005 §6) — `checkbox-xs` resolves to
//! `16×16px` in `daisyui@4.12.14`'s pinned CSS
//! (`https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css`,
//! the version `components/layout.rs` loads), below WCAG 2.2's own AA
//! floor (2.5.8: 24×24) — the only class `QA-014`'s survey found doing
//! that, everything else in `src/components/` is 24px or larger.
//! `QA-015` removed it from the three real checkbox controls that
//! carried it (`notification_preferences.rs`), reaching the bare
//! `.checkbox` class's own `24px` (`height:1.5rem;width:1.5rem`). This
//! keeps it from coming back.
//!
//! **Sibling to `contrast_scan`, not folded into it.** Same reasoning
//! `dec_007_scan`/`dec_007_ci_scan` already established: contrast
//! (`WCAG` 1.4.3) and touch-target size (`WCAG` 2.5.8) are different
//! properties measured against a different pinned CSS fact, and a
//! regression in one should not read as a failure of the other's own
//! name. `contrast_scan`'s own doc comment names the same DaisyUI
//! version and a disjoint set of resolved values; this module names its
//! own.
//!
//! **One needle, not a class of needles.** `QA-014`'s survey found six
//! other classes below 44px (`btn-sm`/`-xs`, `input-sm`/`-xs`,
//! `select-sm`/`-xs`) still in deliberate use pending `0.30.0`'s
//! touch-target design pass — banning any of those today would fail on
//! the current, correct tree and get weakened until it passed, which is
//! worse than no guard (`QA-013`'s own reasoning for why `contrast_scan`
//! stops at `/60` and not higher). `checkbox-xs` is different: `QA-015`
//! removed every use, and the class serves no purpose this codebase
//! still needs — `checkbox-sm` (`20px`, still below AA) and the bare
//! `checkbox` (`24px`, the class this project now uses) are the only
//! smaller-than-default sizes with any legitimate reason to exist here,
//! and neither is `checkbox-xs`.

use std::fs;
use std::path::{Path, PathBuf};

/// Every `.rs` file under `dir`, recursively — the same walk
/// `prose_scan`/`contrast_scan` use, duplicated rather than shared for
/// a fifteen-line helper.
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
fn checkbox_xs_appears_nowhere() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/ -- the workspace layout assumption \
         this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in files
        .iter()
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("touch_target_scan.rs"))
    {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        if source.contains("checkbox-xs") {
            offenders.push(path.display().to_string());
        }
    }

    assert!(
        offenders.is_empty(),
        "checkbox-xs resolves to 16x16px in the pinned DaisyUI CSS, below WCAG \
         2.2's AA touch-target floor (2.5.8: 24x24) -- QA-015 removed every use; \
         use the bare `checkbox` class (24px) instead:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
