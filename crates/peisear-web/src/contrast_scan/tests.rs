use super::*;

#[test]
fn no_muted_text_below_the_seventy_floor() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/ -- the workspace layout assumption \
         this scan depends on may have changed"
    );

    // This module's own file is excluded: `BANNED_MODIFIERS` has to
    // name the literal strings it bans, so a naive scan of every `.rs`
    // file would flag this file for containing its own ban list —
    // the same self-mention shape `prose_scan`'s doc comment and
    // `dec_007_scan`'s history paragraph both had to be excluded from
    // their own scans for, just via a literal array instead of prose.
    let mut offenders = Vec::new();
    for path in files
        .iter()
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("contrast_scan.rs"))
    {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for needle in BANNED_MODIFIERS {
            if source.contains(needle) {
                offenders.push(format!("{}: {needle}", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "these files use a text-base-content opacity modifier below the /70 floor \
         -- QA-012 and QA-013 measured every one of /10 through /60 as failing AA \
         against at least one real background this theme defines (see this module's \
         doc comment for the resolved values and DaisyUI version):\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
