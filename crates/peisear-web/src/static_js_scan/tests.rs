use super::*;

#[test]
fn no_hardcoded_prose_in_static_js_outside_the_message_table() {
    let files = all_static_js_files();
    assert!(
        !files.is_empty(),
        "found no static/*.js files -- the workspace layout assumption \
         this scan depends on may have changed"
    );

    let violations = all_violations();
    assert!(
        violations.is_empty(),
        "user-visible prose found in static/*.js (route it through peisear_i18n::MessageKey \
         and a JSON island, `dm.js`'s pattern, or add a reasoned allowlist entry in \
         static_js_scan.rs):\n{}",
        violations
            .iter()
            .map(|v| format!("  {}:{} {:?}", v.file, v.line, v.text))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// `search.js` genuinely has string literals this guard would flag
/// if it weren't allowlisted -- proves the allowlist entry excludes a
/// real file with real matches, not a name that no longer needs it.
#[test]
fn search_js_allowlist_entry_still_matches_something() {
    let files = all_static_js_files();
    let (path, _) = files
        .iter()
        .find(|(_, name)| name == "search.js")
        .expect("search.js present under static/");
    let source = fs::read_to_string(path).expect("read search.js");
    let stripped = strip_line_comments(&source);
    let mut violations = Vec::new();
    scan_string_literals(&stripped, "search.js", &mut violations);
    assert!(
        !violations.is_empty(),
        "search.js no longer has anything this guard would flag -- if it was converted, \
         remove the allowlist entry and let this guard cover it"
    );
}

/// `STATIC-001`: nothing under `static/` that is not a servable asset
/// type. See the module doc for what this does and does not check.
#[test]
fn every_entry_under_static_is_a_servable_asset_type() {
    let static_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("static");
    let mut files = Vec::new();
    collect_all_files(&static_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found nothing under static/ -- the workspace layout assumption this scan \
         depends on may have changed"
    );

    let mut offenders: Vec<String> = files
        .iter()
        .filter(|p| !is_servable_asset_type(p))
        .map(|p| {
            p.strip_prefix(&static_dir)
                .unwrap_or(p)
                .display()
                .to_string()
        })
        .collect();
    offenders.sort();
    assert!(
        offenders.is_empty(),
        "static/ is embedded and served in full (`static_assets.rs`, mounted in app.rs), so \
         every file in it is a public URL on every deployment -- and these are not an allowed asset \
         type (allowed: {}). Move a document to docs/; to publish a new kind of asset, add \
         its extension and a reason to SERVABLE_ASSET_TYPES in static_js_scan.rs:\n{}",
        SERVABLE_ASSET_TYPES
            .iter()
            .map(|(ext, _)| format!(".{ext}"))
            .collect::<Vec<_>>()
            .join(", "),
        offenders
            .iter()
            .map(|o| format!("  static/{o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
