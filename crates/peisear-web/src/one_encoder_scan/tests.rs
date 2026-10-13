use super::*;

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
