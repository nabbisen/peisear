use super::*;

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
        .filter(|name| !covers_own_lib_tests(&block, name))
        .collect();

    assert!(
        missing.is_empty(),
        "these workspace members have no line in DEC-007's command block in \
         .github/CONTRIBUTING.md that covers their own library target -- a \
         `--test <target>` line for a crate does not run that crate's own lib \
         tests. Add a bare `cargo test -p <crate>` or `cargo test -p <crate> \
         --lib` line for each:\n{}",
        missing
            .iter()
            .map(|n| format!("  {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
