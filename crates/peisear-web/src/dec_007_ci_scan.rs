//! `QA-008` §4 (RFC 005 §1) — the second link in the chain
//! `dec_007_scan`'s own module doc names: workspace members → `DEC-007`
//! block (that module) → CI jobs (here). `dec_007_scan` pins the first
//! link; nothing pinned the second, so the exact gap `QA-005` exists to
//! close — a guard that isn't actually running in CI — was fully
//! reconstructible by deleting YAML: the architect deleted
//! `test-peisear-web-lib` from `.github/workflows/test.yml` and changed
//! nothing else, and `cargo test --workspace` stayed at 195 passed, 0
//! failed, with `.github/CONTRIBUTING.md` still claiming the guard
//! runs. Reproduced before writing this file — see
//! `evidence/section4-plant-deleted-job.log` in the review request.
//!
//! **What this asserts**: every `cargo test -p <crate> ...` obligation
//! named in the `DEC-007` block has a corresponding `run:` line
//! somewhere in `test.yml` — a fact about two files' text, the same
//! shape `QA-005` §3 used to close the analogous `--test`-line gap in
//! the other module. **Deliberately one-directional**: CI legitimately
//! runs `fmt`, `clippy`, `build` and `msrv`, none of which the block
//! mentions, so requiring the reverse would fail on the current,
//! correct tree.
//!
//! **Per-crate, per-shape, not per-crate alone.** `peisear-web`
//! appears in the block twice, in two shapes that mean different
//! things: a `--test "$t"` line inside a `for` loop (its 20 individual
//! test targets) and a separate bare `cargo test -p peisear-web --lib`
//! line (its own library target — the one `test-peisear-web-lib`
//! runs, and the one the guards from `QA-003`–`QA-007` all live under).
//! A check that only asked "does `peisear-web` appear anywhere in
//! `test.yml`'s `run:` lines" would still pass with
//! `test-peisear-web-lib` deleted, because the 20 per-target jobs are
//! still there — the exact false-pass shape `dec_007_scan`'s own
//! history keeps hitting (`QA-005` §3: a `--test` line standing in for
//! a crate's own library scope). So each `-p <name>` occurrence in the
//! block is classified into one of three shapes — `--lib`, `--test`
//! (a real `--test <target>` selector, not `--test-threads=N`), or
//! bare — and the corresponding `run:` line in `test.yml` must match
//! the *same* shape, not merely the same crate name. The loop's
//! dynamic `"$t"` is never matched literally; every `--test <target>`
//! line for a crate is treated as one shape regardless of which target
//! it names, since `test.yml` is free to split them into as many jobs
//! as it wants (it splits `peisear-web`'s into twenty) — the block
//! only requires that shape to be covered *somewhere*.
//!
//! **A qualifying line, on either side, must not be commented out** —
//! same convention as `dec_007_scan`'s own commented-line check
//! (`QA-005-review.md` §2): a `#`-prefixed `run:` step names a job
//! step that never runs, same as a `#`-prefixed line in the
//! `CONTRIBUTING.md` block.
//!
//! **Documented limit, per `QA-008` §4's own instruction not to add a
//! YAML-parsing dependency**: this does not interpret job-level YAML
//! semantics. A job with `if: false` or `continue-on-error: true`
//! still has an uncommented `run:` line and will satisfy this guard
//! even though the job may not meaningfully run or may not fail the
//! workflow when its command fails. Catching that would need an actual
//! YAML parser plus job-level semantic interpretation ("is this job
//! reachable, does its failure propagate") — a materially bigger and
//! more fragile guard than a fact about two files' text. This guard
//! catches the deletion case it was built for (`QA-008` §4's own plant)
//! and stops there by design, not by oversight.

use crate::dec_007_scan::{appears_at_word_boundary, dec_007_block};
use std::fs;
use std::path::Path;

/// Which flag shape a `-p <name>` occurrence appears under. `Lib` and
/// `Test` are mutually exclusive selectors on the same crate; `Bare`
/// is neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Shape {
    Lib,
    Test,
    Bare,
}

/// Every `(crate name, shape)` obligation a `-p <name>` occurrence in
/// `text` creates — one per non-commented line that contains `-p `.
/// Shared between the block side and the `test.yml` side so the two
/// are classified identically.
fn p_flag_targets(text: &str) -> Vec<(String, Shape)> {
    text.lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .filter_map(|line| {
            let after = line.split_once("-p ")?.1;
            let name: String = after.chars().take_while(|c| !c.is_whitespace()).collect();
            if name.is_empty() {
                return None;
            }
            let shape = if line.contains("--test ") {
                Shape::Test
            } else if appears_at_word_boundary(line, "--lib") {
                Shape::Lib
            } else {
                Shape::Bare
            };
            Some((name, shape))
        })
        .collect()
}

/// `test.yml`'s own `run:` step lines, one entry per non-commented
/// step — `- run: <command>` (a leading `#`, before or after the `- `
/// is stripped, counts as commented out).
fn test_yml_run_lines() -> Vec<String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workflow = manifest_dir
        .join("..")
        .join("..")
        .join(".github")
        .join("workflows")
        .join("test.yml");
    let source = fs::read_to_string(&workflow)
        .unwrap_or_else(|e| panic!("read {}: {e}", workflow.display()));

    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                return None;
            }
            let after_dash = trimmed.strip_prefix("- ").unwrap_or(trimmed);
            after_dash.strip_prefix("run:").map(str::to_string)
        })
        .collect()
}

#[test]
fn every_dec_007_block_target_has_a_matching_ci_run_line() {
    let block = dec_007_block();
    let mut required = p_flag_targets(&block);
    required.sort();
    required.dedup();
    assert!(
        !required.is_empty(),
        "found no `-p <crate>` obligations in the DEC-007 block -- the block's own \
         format may have changed underneath this scan"
    );

    let run_lines: Vec<String> = test_yml_run_lines();
    assert!(
        !run_lines.is_empty(),
        "found no `run:` steps in .github/workflows/test.yml -- the workflow's own \
         format may have changed underneath this scan"
    );

    let missing: Vec<String> = required
        .iter()
        .filter(|(name, shape)| {
            !run_lines.iter().any(|line| {
                appears_at_word_boundary(line, &format!("-p {name}"))
                    && match shape {
                        Shape::Test => line.contains("--test "),
                        Shape::Lib => {
                            !line.contains("--test ") && appears_at_word_boundary(line, "--lib")
                        }
                        Shape::Bare => {
                            !line.contains("--test ") && !appears_at_word_boundary(line, "--lib")
                        }
                    }
            })
        })
        .map(|(name, shape)| format!("  {name} ({shape:?})"))
        .collect();

    assert!(
        missing.is_empty(),
        "these DEC-007 block obligations have no matching `run:` line in \
         .github/workflows/test.yml -- a `--test <target>` job does not cover a \
         crate's `--lib` obligation or vice versa:\n{}",
        missing.join("\n")
    );
}
