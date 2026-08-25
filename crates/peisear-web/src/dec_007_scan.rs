//! `QA-004` (RFC 005 §13) — the `DEC-007` command block in
//! `.github/CONTRIBUTING.md` had drifted from the workspace it claims
//! to cover: `peisear`, the facade crate, was absent from the block
//! entirely — not present with the wrong flags, absent. Discovered
//! when a release candidate's own gate table carried a count no
//! command in the block actually produced.
//!
//! This asserts every workspace member's crate name appears somewhere
//! in the block, so the list cannot drift silently again. Reads
//! `Cargo.toml`'s `members` array and `.github/CONTRIBUTING.md` as
//! plain text — no TOML dependency added for this (`QA-004` §6: "a
//! guard is not worth a new dependency; there are cheaper shapes"),
//! matching how `prose_scan`/`static_js_scan`/`test_harness_scan` all
//! already read their targets as source text rather than through a
//! parser for the format.
//!
//! **Matches on a word boundary, not a substring.** `peisear` is a
//! substring of every other member's name (`peisear-core`,
//! `peisear-auth`, ...), so a naive `contains("peisear")` check would
//! have passed even with the facade's own line missing — the exact
//! defect this guard exists to catch would have slipped through it.
//! Proved by planting that specific case, and by planting a second
//! member's line removed too — see `evidence/` in the review request
//! for both transcripts; a real `.github/CONTRIBUTING.md` edit is not
//! something to leave embedded in this file's own test code.
//!
//! **Matches on `-p <name>`, not on `<name>` alone**
//! (`QA-004-review.md` §2). Every line in the block runs `-p
//! <crate>`, so that pair is the actual invariant — matching the bare
//! name would let a comment that merely *mentions* a crate (`# note:
//! peisear itself is the facade crate`) satisfy the scan with no
//! command present at all, which is a silent pass on the exact defect
//! this guard exists for. The one thing this gives up: `cargo test
//! --package peisear` (the long flag) would fail the guard even
//! though it runs the crate. That is a false alarm with a clear
//! message pointing at this file, not a false pass — the direction
//! worth erring toward.

use std::fs;
use std::path::Path;

/// Every crate name declared in the workspace `Cargo.toml`'s
/// `members` array — the last path segment of each entry
/// (`"crates/peisear-i18n"` → `"peisear-i18n"`), read as text rather
/// than parsed as TOML.
fn workspace_member_crate_names() -> Vec<String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_toml = manifest_dir.join("..").join("..").join("Cargo.toml");
    let source = fs::read_to_string(&workspace_toml)
        .unwrap_or_else(|e| panic!("read {}: {e}", workspace_toml.display()));

    let start = source
        .find("members = [")
        .expect("workspace Cargo.toml has a `members = [` array");
    let after = &source[start + "members = [".len()..];
    let end = after.find(']').expect("`members` array is closed with `]`");
    let list = &after[..end];

    list.lines()
        .filter_map(|line| {
            let trimmed = line.trim().trim_end_matches(',');
            let path = trimmed.strip_prefix('"')?.strip_suffix('"')?;
            path.rsplit('/').next().map(str::to_string)
        })
        .collect()
}

/// True if `needle` appears in `haystack` at a word boundary — not
/// merely as a substring of a longer crate name (`peisear` inside
/// `peisear-core`). A boundary character is anything that is not
/// alphanumeric, `_`, or `-`.
fn appears_at_word_boundary(haystack: &str, needle: &str) -> bool {
    let bytes = haystack.as_bytes();
    let is_ident_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'-';
    let mut start = 0;
    while let Some(rel) = haystack[start..].find(needle) {
        let idx = start + rel;
        let before_ok = idx == 0 || !is_ident_char(bytes[idx - 1]);
        let after_idx = idx + needle.len();
        let after_ok = after_idx >= bytes.len() || !is_ident_char(bytes[after_idx]);
        if before_ok && after_ok {
            return true;
        }
        start = idx + 1;
    }
    false
}

/// The `DEC-007` code block's contents — everything between the first
/// fenced ` ```bash ` after "DEC-007" and its closing fence.
fn dec_007_block() -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let contributing = manifest_dir
        .join("..")
        .join("..")
        .join(".github")
        .join("CONTRIBUTING.md");
    let source = fs::read_to_string(&contributing)
        .unwrap_or_else(|e| panic!("read {}: {e}", contributing.display()));

    let marker = source
        .find("DEC-007")
        .expect("CONTRIBUTING.md mentions DEC-007");
    let after_marker = &source[marker..];
    let fence_start = after_marker
        .find("```bash")
        .expect("a ```bash fence follows the DEC-007 mention");
    let after_fence = &after_marker[fence_start + "```bash".len()..];
    let fence_end = after_fence
        .find("```")
        .expect("the ```bash fence is closed");
    after_fence[..fence_end].to_string()
}

#[test]
fn every_workspace_member_appears_in_the_dec_007_block() {
    let members = workspace_member_crate_names();
    assert!(
        members.len() >= 2,
        "found suspiciously few workspace members ({}) -- the Cargo.toml \
         parsing assumption this scan depends on may have changed",
        members.len()
    );

    let block = dec_007_block();
    let missing: Vec<&String> = members
        .iter()
        .filter(|name| !appears_at_word_boundary(&block, &format!("-p {name}")))
        .collect();

    assert!(
        missing.is_empty(),
        "these workspace members are missing from DEC-007's command block in \
         .github/CONTRIBUTING.md -- add a `cargo test -p <crate>` line for each:\n{}",
        missing
            .iter()
            .map(|n| format!("  {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
