//! `QA-020` §4 (RFC 005 §11) — one `percent_encode_query`
//! (`handlers.rs`), every redirect that carries dynamic text through
//! it. `.replace(' ', "+")` was the hand-rolled idiom 23 sites used
//! instead — correct only because every flash message this project
//! has ever written happened to be plain ASCII with no `&`, `=`,
//! `#`, `%`, `+`, `?`, or non-ASCII byte, a property of the copy
//! rather than of the code (`NFR-LANG-001`'s `find_violations`
//! constrains tone, not character set). This keeps the idiom from
//! coming back.
//!
//! **Forbids the known-wrong idiom; does not prove the right one.**
//! Asserting "every `Redirect::to` argument is percent-encoded"
//! would need to distinguish which of this crate's 48 `Redirect::to`
//! sites carry user-or-copy-derived text from the (much larger)
//! group that are static paths — a text scan cannot tell dynamic
//! content from a hardcoded route string. That is the honest limit,
//! not a gap being quietly accepted: this scan bans the one thing it
//! can name with certainty, the same trade `contrast_scan`/
//! `touch_target_scan` make against a pinned CSS fact rather than
//! rendering every page.

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
fn hand_rolled_space_replace_appears_nowhere() {
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
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("one_encoder_scan.rs"))
    {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        if source.contains(".replace(' ', \"+\")") {
            offenders.push(path.display().to_string());
        }
    }

    assert!(
        offenders.is_empty(),
        "`.replace(' ', \"+\")` only encodes the space character -- it was safe \
         at every QA-020 site by luck of the copy, not by construction. Use \
         `super::percent_encode_query` (handlers.rs) instead:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
