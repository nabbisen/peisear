use super::*;

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
