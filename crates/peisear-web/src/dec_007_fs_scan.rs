//! `TT-003` (RFC 012 step 3) §3 — `§10.16` reopened. `dec_007_scan` and
//! `dec_007_ci_scan` check two links: workspace members → `DEC-007`
//! block, and block → CI. Neither checks the link before either:
//! **filesystem → block**. Verified during `TT-002` round 1's review:
//! deleting `touch_target`'s lines from `.github/CONTRIBUTING.md` left
//! all three existing scans green. So a new `crates/*/tests/*.rs` gets
//! no loop entry and no CI job, silently, and runs only under `cargo
//! test --workspace` — precisely the exposure `§10.16` was opened to
//! describe and was believed closed at 0.27.0. `TT-002` hit this and
//! wired `touch_target` in by hand; nothing would have told it to.
//!
//! **Combined with the existing block → CI scan, this gives
//! filesystem → CI transitively** — stated here because the
//! transitivity is the actual guarantee this project wants, and it is
//! not obvious from either scan alone: this module proves every test
//! file has a block obligation, `dec_007_ci_scan` proves every block
//! obligation has a matching CI `run:` line. Neither proves the
//! composed claim by itself, and neither module's own doc comment
//! could state it without assuming the other exists.
//!
//! **Scope: integration test files only**
//! (`crates/*/tests/*.rs`, direct children — not `tests/common/*.rs`,
//! which are shared support modules pulled in via `mod common;`, not
//! their own cargo test targets, the same distinction
//! `test_harness_scan` already draws when it walks `tests/`).
//! Deliberately **not** extended to `--lib` targets (`dec_007_scan`
//! already covers every crate's own lib target existing in the block)
//! or to benches (none exist in this workspace today) — `TT-003`'s own
//! instruction.
//!
//! **A crate with a bare `-p <crate>` block line needs no per-file
//! entry.** `cargo test -p <crate>` with neither `--lib` nor `--test
//! <target>` runs the crate's lib tests, every integration test
//! binary, and its doctests — `peisear-i18n`'s and `peisear-notify`'s
//! block lines are this shape, and correctly cover their `tests/*.rs`
//! files with no per-file enumeration. Only a crate reached
//! exclusively through `--test <target>` lines (`peisear-web`'s, for
//! the linker-memory reason `.github/CONTRIBUTING.md` documents) needs
//! every one of its `tests/*.rs` files individually named. Treating
//! every crate as needing per-file enumeration would fail on
//! `peisear-i18n`/`peisear-notify`'s own correct, bare-covered lines —
//! exactly the "fails on a correct tree" shape this project's other
//! scans all avoid.
//!
//! **Reuses `dec_007_scan`/`dec_007_ci_scan`'s own block-reading and
//! shell-loop-parsing rather than a third, subtly different copy** —
//! `dec_007_block`, `appears_at_word_boundary`, and `for_loop_targets`
//! are all `pub(crate)` for exactly this reason.

use crate::dec_007_ci_scan::for_loop_targets;
use crate::dec_007_scan::{appears_at_word_boundary, dec_007_block};
use std::fs;
use std::path::Path;

/// Every crate directory under `crates/` that has a `tests/` directory
/// containing at least one top-level `.rs` file — `(crate name, [file
/// stem, ...])`, stems sorted. `tests/common/` and any other
/// subdirectory is not walked; those files are shared modules, not
/// cargo test targets.
fn crates_with_integration_test_targets() -> Vec<(String, Vec<String>)> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let crates_dir = manifest_dir.join("..");
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(&crates_dir) else {
        return out;
    };
    let mut crate_dirs: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    crate_dirs.sort();

    for crate_dir in crate_dirs {
        if !crate_dir.is_dir() {
            continue;
        }
        let crate_name = crate_dir
            .file_name()
            .and_then(|n| n.to_str())
            .expect("crate directory name is valid UTF-8")
            .to_string();
        let Ok(tests_entries) = fs::read_dir(crate_dir.join("tests")) else {
            continue;
        };
        let mut stems: Vec<String> = tests_entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "rs"))
            .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string))
            .collect();
        if !stems.is_empty() {
            stems.sort();
            out.push((crate_name, stems));
        }
    }
    out
}

/// True if `block` has a bare `-p <crate>` line — not commented out,
/// no `--lib`, no `--test `. That single line runs every integration
/// test binary the crate has, so no per-file check is needed for it.
fn has_bare_line(block: &str, crate_name: &str) -> bool {
    let flag = format!("-p {crate_name}");
    block.lines().any(|line| {
        !line.trim_start().starts_with('#')
            && appears_at_word_boundary(line, &flag)
            && !line.contains("--test ")
            && !appears_at_word_boundary(line, "--lib")
    })
}

/// True if `block` names `stem` as a `--test <stem>` target for
/// `crate_name` — either a literal `-p <crate> --test <stem>` line, or
/// (`peisear-web`'s present shape) a shell `for t in ...; do` loop
/// whose body runs `-p <crate> --test "$t"` and whose literal target
/// list (`for_loop_targets`) contains `stem`.
fn covers_test_target(block: &str, crate_name: &str, stem: &str) -> bool {
    let literal_line = block.lines().any(|line| {
        !line.trim_start().starts_with('#')
            && appears_at_word_boundary(line, &format!("-p {crate_name}"))
            && appears_at_word_boundary(line, &format!("--test {stem}"))
    });
    if literal_line {
        return true;
    }

    let loop_covers_crate = block.lines().any(|line| {
        !line.trim_start().starts_with('#')
            && appears_at_word_boundary(line, &format!("-p {crate_name}"))
            && line.contains("--test \"$t\"")
    });
    loop_covers_crate && for_loop_targets(block).iter().any(|t| t == stem)
}

#[test]
fn every_integration_test_file_appears_in_the_dec_007_block() {
    let crates = crates_with_integration_test_targets();
    assert!(
        !crates.is_empty(),
        "found no crates with tests/*.rs files -- the workspace layout \
         assumption this scan depends on may have changed"
    );

    let block = dec_007_block();
    let mut missing = Vec::new();
    for (crate_name, stems) in &crates {
        if has_bare_line(&block, crate_name) {
            continue;
        }
        for stem in stems {
            if !covers_test_target(&block, crate_name, stem) {
                missing.push(format!("{crate_name}/tests/{stem}.rs"));
            }
        }
    }

    assert!(
        missing.is_empty(),
        "these integration test files have no line in DEC-007's command block in \
         .github/CONTRIBUTING.md that runs them -- combined with dec_007_ci_scan \
         (block -> CI), this scan is the filesystem -> block half of \
         filesystem -> CI; a file missing here means the documented gate silently \
         never exercises it (baseline `§10.16`):\n{}",
        missing
            .iter()
            .map(|f| format!("  {f}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
