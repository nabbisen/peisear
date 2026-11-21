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

/// `CLR-003` (`§10.61`): no `components/**` element reaches a reader
/// through a semantic colour's own text/link class, or through
/// `btn-outline` paired with a semantic `btn-{family}`, except the two
/// sites measured and excluded by their own exact literal.
#[test]
fn no_semantic_colour_in_a_text_position() {
    let dir = components_dir();
    let mut files = Vec::new();
    collect_rs_files(&dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/components/ -- the directory \
         this scan depends on may have moved"
    );

    let mut offenders = Vec::new();
    for path in &files {
        offenders.extend(semantic_violations_in(path));
    }

    assert!(
        offenders.is_empty(),
        "these elements reach a reader through a semantic DaisyUI colour \
         (text-{{family}}, link-{{family}}, or btn-outline paired with \
         btn-{{family}}) rather than base-content or a solid, AA-passing \
         variant -- see this module's own doc comment for the measured \
         ratios and the two excluded sites:\n{}",
        offenders
            .iter()
            .map(|(file, what)| format!("  {file}: {what}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// An exclusion that matches the wrong number of live sites is itself
/// a defect — zero means stale (the `SIZE-002`/`css_scan` allowlist
/// precedent), but **more than expected is the sharper failure**:
/// `CLR-003` round 2 found that a bare "matches something" check lets
/// a short, generic literal (`"text-primary"`) absorb a brand-new
/// violation of the exact shape this guard exists to catch, silently,
/// because the exclusion itself reads as satisfied either way. Each
/// [`NAMED_EXCLUSION_CLASS_VALUES`] entry must match its own
/// [`NAMED_EXCLUSION_COUNTS`] exactly — not "at least one."
#[test]
fn every_named_exclusion_matches_its_exact_live_count() {
    let dir = components_dir();
    let mut files = Vec::new();
    collect_rs_files(&dir, &mut files);
    assert!(!files.is_empty());

    let mut counts = [0usize; NAMED_EXCLUSION_CLASS_VALUES.len()];
    for path in &files {
        let raw =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&raw);
        for (quote_pos, content_start, content_end) in quoted_string_spans(&stripped) {
            if !is_token_source(&stripped, quote_pos) {
                continue;
            }
            let content = &stripped[content_start..content_end];
            for (i, exclusion) in NAMED_EXCLUSION_CLASS_VALUES.iter().enumerate() {
                if content == *exclusion {
                    counts[i] += 1;
                }
            }
        }
    }

    for (i, exclusion) in NAMED_EXCLUSION_CLASS_VALUES.iter().enumerate() {
        assert_eq!(
            counts[i], NAMED_EXCLUSION_COUNTS[i],
            "NAMED_EXCLUSION_CLASS_VALUES[{i}] ({exclusion:?}) matches {} live \
             class value(s) under components/, not the {} this module's own \
             constant records -- if it dropped to zero, the site it was \
             written for has moved or changed and this exclusion is now \
             stale; if it grew, a new site is claiming this exemption and \
             must be measured and justified on its own, not inherited \
             silently",
            counts[i], NAMED_EXCLUSION_COUNTS[i]
        );
    }
}
