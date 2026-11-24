use super::*;

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

/// `GATE-001`: every `browser-checks/*.mjs` file is either named in
/// `EXEMPT_BROWSER_CHECK_FILES` (with its own reason) or appears inside
/// some `test.yml` `run:` line — `disclosure-marker-gate.mjs`'s own
/// shape (written, passing, never wired) fails here by name rather
/// than by a human noticing `git log -S` returns nothing.
#[test]
fn every_browser_check_is_wired_or_exempt() {
    let files = browser_check_files();
    assert!(
        !files.is_empty(),
        "found no browser-checks/*.mjs files -- the layout assumption this scan \
         depends on may have changed"
    );
    let unwired = unwired_browser_check_files();
    assert!(
        unwired.is_empty(),
        "these browser-checks/*.mjs files are neither named in \
         EXEMPT_BROWSER_CHECK_FILES nor found inside any test.yml `run:` line -- a \
         gate that passes locally and nothing in CI ever runs:\n{}",
        unwired
            .iter()
            .map(|n| format!("  {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// An exclusion that no longer names a real file is stale the same way
/// a stale acknowledgement is elsewhere in this codebase
/// (`PLC-003`/`EDG-003`) -- caught here rather than silently covering
/// nothing.
#[test]
fn every_exempt_browser_check_file_still_exists() {
    let names: Vec<String> = browser_check_files()
        .into_iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .collect();
    let stale: Vec<&str> = EXEMPT_BROWSER_CHECK_FILES
        .iter()
        .map(|(n, _)| *n)
        .filter(|n| !names.contains(&n.to_string()))
        .collect();
    assert!(
        stale.is_empty(),
        "these EXEMPT_BROWSER_CHECK_FILES entries no longer name a real \
         browser-checks/*.mjs file -- renamed or removed, and this exclusion is \
         now stale:\n{}",
        stale.join("\n")
    );
}
