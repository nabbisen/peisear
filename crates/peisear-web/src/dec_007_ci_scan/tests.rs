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
