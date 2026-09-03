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
//! **`QA-009` §4 — `if: false` is now closed, `continue-on-error` is
//! deliberately left open, for two different reasons.** `QA-008`
//! stopped at "this does not interpret job-level YAML semantics" for
//! both. Revisited:
//!
//! - **`if: false`** is closable and now closed, indentation-based, no
//!   YAML parser: `job_blocks` reads `test.yml`'s own consistent shape
//!   (a job name at exactly two spaces, everything inside it indented
//!   four or more) to find each job's body, and `job_is_disabled`
//!   checks whether that body contains a literal `if: false` line. A
//!   disabled job's `run:` lines are dropped before either test above
//!   sees them, so a target whose only coverage lives in a
//!   `if: false` job reads as missing, correctly. Deliberately narrow:
//!   only the exact text `if: false` is recognised — `if: 'false'`,
//!   `if: ${{ false }}` and other YAML-truthy spellings are not, since
//!   closing all of them needs real expression evaluation. That is a
//!   named limit, not a silent gap: `job_is_disabled`'s own doc comment
//!   says so.
//! - **`continue-on-error: true`** is a different property from
//!   `if: false`, not merely a harder-to-parse version of the same
//!   one. `if: false` means the job never runs, so its `run:` line
//!   never executes anything — this guard's whole question ("does a
//!   `run:` line exist that will exercise this target") is answered
//!   "no", correctly. `continue-on-error: true` means the job *does*
//!   run — the target is genuinely exercised — but a failure there
//!   does not turn the workflow red. That is a fact about whether
//!   failure blocks merging, not about whether coverage exists, and
//!   this guard was never asked the first question. Folding it in
//!   would silently redefine what "covered" means here, from "the
//!   command runs" to "the command runs and is enforced" — a real,
//!   different property, and not one this scan reports.
//!
//! **`QA-009` §3 — the `Test` shape above is too coarse for the `for`
//! loop specifically.** It only asked "is *some* `--test <target>`
//! job present for `peisear-web`", so deleting nineteen of the loop's
//! twenty per-target CI jobs and keeping one still satisfies it — a
//! whole integration suite stops running in CI and neither `dec_007`
//! guard says so. Closable without a YAML or shell parser: the block
//! names all twenty targets **literally**, in the `for t in …` list,
//! across `\`-continued lines. `for_loop_targets` reads from `for t in
//! ` to `; do`, and `split_whitespace` already treats a line
//! continuation's newline the same as the spaces around it — the lone
//! `\` tokens are filtered out explicitly, everything else is a real
//! target name. `every_dec_007_for_loop_target_has_a_matching_ci_run_line`
//! then requires each of the twenty to have its own `run:` line, not
//! merely "some `--test` job for `peisear-web` exists somewhere" —
//! narrowing the `Test` shape's claim for this one crate without
//! touching what it means for any other.

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

fn test_yml_source() -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workflow = manifest_dir
        .join("..")
        .join("..")
        .join(".github")
        .join("workflows")
        .join("test.yml");
    fs::read_to_string(&workflow).unwrap_or_else(|e| panic!("read {}: {e}", workflow.display()))
}

/// `(job name, job body)` pairs from `source`'s `jobs:` section — a
/// job is a line indented by exactly two spaces, ending in `:`,
/// directly under top-level `jobs:`; its body is every line up to the
/// next such line or EOF. Indentation-based, not a YAML parser: this
/// is `test.yml`'s own consistent shape (every job name at exactly
/// two spaces, every step and key inside a job indented four or
/// more), the same reliance on `rustfmt`-enforced structure
/// `enumeration_guard.rs` places on `message.rs`'s own formatting.
fn job_blocks(source: &str) -> Vec<(String, String)> {
    let lines: Vec<&str> = source.lines().collect();
    let jobs_at = lines
        .iter()
        .position(|l| l.trim_end() == "jobs:")
        .expect("test.yml declares a top-level `jobs:` key");

    let mut blocks = Vec::new();
    let mut current: Option<(String, Vec<&str>)> = None;
    for line in &lines[jobs_at + 1..] {
        let is_job_header =
            line.starts_with("  ") && !line.starts_with("   ") && line.trim_end().ends_with(':');
        if is_job_header {
            if let Some((name, body)) = current.take() {
                blocks.push((name, body.join("\n")));
            }
            current = Some((line.trim().trim_end_matches(':').to_string(), Vec::new()));
        } else if let Some((_, body)) = current.as_mut() {
            body.push(line);
        }
    }
    if let Some((name, body)) = current {
        blocks.push((name, body.join("\n")));
    }
    blocks
}

/// True if `job_body` disables its job outright via a literal `if:
/// false` line — `QA-009` §4. Deliberately narrow: only the exact
/// trimmed text `if: false` counts. `if: 'false'`, `if: "false"`,
/// `if: ${{ false }}` and other YAML-truthy spellings that mean the
/// same thing are not recognised — closing all of them needs a real
/// YAML/expression parser, which §4 says to report rather than add a
/// dependency for. This catches the literal shape the handoff names
/// and stops there, the same trade `dec_007_scan` already made for
/// `--test`-line detection.
fn job_is_disabled(job_body: &str) -> bool {
    job_body.lines().any(|line| line.trim() == "if: false")
}

/// `test.yml`'s own `run:` step lines that will actually execute:
/// non-commented (`- run: <command>`, a leading `#` before or after
/// the `- ` counts as commented out), and not inside a job whose body
/// contains a literal `if: false`. `continue-on-error: true` is
/// deliberately **not** treated the same way — see this module's doc
/// comment for why that is a different property this guard does not
/// need to interpret.
fn test_yml_run_lines() -> Vec<String> {
    job_blocks(&test_yml_source())
        .into_iter()
        .filter(|(_, body)| !job_is_disabled(body))
        .flat_map(|(_, body)| {
            body.lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.starts_with('#') {
                        return None;
                    }
                    let after_dash = trimmed.strip_prefix("- ").unwrap_or(trimmed);
                    after_dash.strip_prefix("run:").map(str::to_string)
                })
                .collect::<Vec<String>>()
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

/// The literal target list from the block's `for t in … ; do` loop —
/// `QA-009` §3. Reads the text from `for t in ` to `; do`;
/// `str::split_whitespace` already splits on the newlines inside the
/// `\`-continued list the same way it splits on the spaces between
/// names, so the only cleanup needed is dropping the lone `\`
/// continuation tokens themselves.
///
/// `pub(crate)` since `TT-003` (`dec_007_fs_scan`): the filesystem→block
/// link needs the same literal list this module already extracts to
/// know which `peisear-web` integration test files the block's shell
/// loop covers, and re-parsing the loop a second, subtly different way
/// is exactly what `dec_007_block`/`appears_at_word_boundary` already
/// avoid for the sibling link.
pub(crate) fn for_loop_targets(block: &str) -> Vec<String> {
    let start_marker = "for t in ";
    let Some(start) = block.find(start_marker) else {
        return Vec::new();
    };
    let after_start = &block[start + start_marker.len()..];
    let Some(end) = after_start.find("; do") else {
        return Vec::new();
    };
    after_start[..end]
        .split_whitespace()
        .filter(|tok| *tok != "\\")
        .map(str::to_string)
        .collect()
}

#[test]
fn every_dec_007_for_loop_target_has_a_matching_ci_run_line() {
    let block = dec_007_block();
    let targets = for_loop_targets(&block);
    assert!(
        targets.len() >= 10,
        "found suspiciously few DEC-007 for-loop targets ({}) -- the block's own \
         `for t in ... ; do` shape may have changed underneath this scan",
        targets.len()
    );

    let run_lines: Vec<String> = test_yml_run_lines();
    let missing: Vec<&String> = targets
        .iter()
        .filter(|target| {
            !run_lines.iter().any(|line| {
                appears_at_word_boundary(line, "-p peisear-web")
                    && appears_at_word_boundary(line, &format!("--test {target}"))
            })
        })
        .collect();

    assert!(
        missing.is_empty(),
        "these DEC-007 for-loop targets (peisear-web --test <target>) have no \
         matching `run:` line in .github/workflows/test.yml -- another `--test` job \
         existing for peisear-web is not the same as this specific target's own job \
         existing:\n{}",
        missing
            .iter()
            .map(|n| format!("  {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
