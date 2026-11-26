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

    let exclusions = named_exclusions();
    let mut offenders = Vec::new();
    for path in &files {
        offenders.extend(semantic_violations_in(path, &exclusions));
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
/// `contrast-exclusions.json` entry's `classValue` must match its own
/// `liveCount` exactly — not "at least one." `SWP-001`: unchanged in
/// substance by the move to a shared file; the count is still read and
/// enforced only here.
#[test]
fn every_named_exclusion_matches_its_exact_live_count() {
    let dir = components_dir();
    let mut files = Vec::new();
    collect_rs_files(&dir, &mut files);
    assert!(!files.is_empty());

    let exclusions = named_exclusions();
    assert!(
        !exclusions.is_empty(),
        "contrast-exclusions.json parsed to zero entries -- the file moved \
         or its shape changed"
    );
    let mut counts = vec![0usize; exclusions.len()];
    for path in &files {
        let raw =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&raw);
        for (quote_pos, content_start, content_end) in quoted_string_spans(&stripped) {
            if !is_token_source(&stripped, quote_pos) {
                continue;
            }
            let content = &stripped[content_start..content_end];
            for (i, exclusion) in exclusions.iter().enumerate() {
                if content == exclusion.class_value {
                    counts[i] += 1;
                }
            }
        }
    }

    for (i, exclusion) in exclusions.iter().enumerate() {
        assert_eq!(
            counts[i], exclusion.live_count,
            "contrast-exclusions.json entry {:?} matches {} live class \
             value(s) under components/, not the liveCount ({}) the file \
             records -- if it dropped to zero, the site it was written for \
             has moved or changed and this exclusion is now stale; if it \
             grew, a new site is claiming this exemption and must be \
             measured and justified on its own, not inherited silently",
            exclusion.class_value, counts[i], exclusion.live_count
        );
    }
}

/// `PLC-003` (`§10.62`): no `components/**`-reachable element meets a
/// below-`/70` text colour through a rule the *stylesheet* applies by
/// element/pseudo-element selector or by a component's own baked-in
/// class — a colour no `components/**` class token itself carries,
/// which is exactly what the two scans above cannot see. Every such
/// rule must be named in `ACKNOWLEDGED_SHEET_RULES` (what beats it) or
/// `EXEMPT_SHEET_RULES` (why it is exempt); one that is neither fails
/// here, not downstream in a screenshot nobody takes.
#[test]
fn every_reachable_sheet_leak_is_acknowledged() {
    let result = sheet_leak_scan();
    assert!(
        result.unacknowledged.is_empty(),
        "these stylesheet rules apply a below-/70 text colour to an \
         element, or to a class some component actually writes, and \
         nothing in ACKNOWLEDGED_SHEET_RULES/EXEMPT_SHEET_RULES names \
         what beats them or why they're exempt -- see this module's own \
         doc comment (`PLC-001`-`PLC-003`, `§10.62`):\n{}",
        result
            .unacknowledged
            .iter()
            .map(|r| format!("  {} -> {}", r.selector, r.value))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// An acknowledgement that no longer points at anything live is itself
/// a defect, in either of its two parts: the underlying sheet rule
/// could have moved or been removed by a vendored-CSS version bump
/// (`stale_acknowledgements`), or the thing this acknowledgement
/// claims beats the rule could have been reverted without the
/// acknowledgement itself being touched (`no_longer_beaten`) — the
/// exact shape `PLC-003`'s own second plant (removing `app.css`'s
/// placeholder override in a scratch worktree) was written to catch.
#[test]
fn every_acknowledged_sheet_rule_is_still_live_and_still_beaten() {
    let result = sheet_leak_scan();
    assert!(
        result.stale_acknowledgements.is_empty(),
        "these ACKNOWLEDGED_SHEET_RULES selectors no longer appear as a \
         below-/70 color: rule in any served sheet -- the vendored CSS \
         moved or changed and this acknowledgement is now stale:\n{}",
        result.stale_acknowledgements.join("\n")
    );
    assert!(
        result.no_longer_beaten.is_empty(),
        "these ACKNOWLEDGED_SHEET_RULES entries' own beaten_by_needle no \
         longer appears in beaten_by_file -- the thing that was supposed \
         to stop this rule reaching a reader is gone, but the \
         acknowledgement does not know that:\n{}",
        result.no_longer_beaten.join("\n")
    );
}

/// `EDG-001`–`EDG-003` (`§10.63`): no `components/**`-reachable element
/// meets a below-`/70` `border-color` or `outline-color` through a rule
/// the *stylesheet* applies — `§10.62`'s own shape, one property over.
/// Every such rule must be named in `ACKNOWLEDGED_BOUNDARY_RULES` (what
/// beats it) or `EXEMPT_BOUNDARY_RULES` (a deliberate, visually-checked
/// choice to leave it, `EDG-002 §4`); one that is neither fails here,
/// not downstream in a screenshot nobody takes.
#[test]
fn every_reachable_boundary_leak_is_acknowledged_or_exempt() {
    let result = boundary_leak_scan();
    assert!(
        result.unacknowledged.is_empty(),
        "these stylesheet rules apply a below-/70 border-color or \
         outline-color to an element, or to a class some component \
         actually writes, and nothing in ACKNOWLEDGED_BOUNDARY_RULES/\
         EXEMPT_BOUNDARY_RULES names what beats them or why they're left \
         -- see this module's own doc comment (`EDG-001`-`EDG-003`, \
         `§10.63`):\n{}",
        result
            .unacknowledged
            .iter()
            .map(|r| format!("  {} {} -> {}", r.selector, r.property, r.value))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// An acknowledgement that no longer points at anything live is itself
/// a defect, in either of its two parts — `§10.62`'s own
/// [`every_acknowledged_sheet_rule_is_still_live_and_still_beaten`],
/// one property over.
#[test]
fn every_acknowledged_boundary_rule_is_still_live_and_still_beaten() {
    let result = boundary_leak_scan();
    assert!(
        result.stale_acknowledgements.is_empty(),
        "these ACKNOWLEDGED_BOUNDARY_RULES (selector, property) pairs no \
         longer appear as a below-/70 rule in any served sheet -- the \
         vendored CSS moved or changed and this acknowledgement is now \
         stale:\n{}",
        result
            .stale_acknowledgements
            .iter()
            .map(|(s, p)| format!("  {s} {p}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        result.no_longer_beaten.is_empty(),
        "these ACKNOWLEDGED_BOUNDARY_RULES entries' own beaten_by_needle \
         no longer appears in beaten_by_file -- the thing that was \
         supposed to stop this rule reaching a reader is gone, but the \
         acknowledgement does not know that:\n{}",
        result
            .no_longer_beaten
            .iter()
            .map(|(s, p)| format!("  {s} {p}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// An exemption that no longer points at anything live is the same
/// defect shape as a stale acknowledgement, named separately because
/// `EXEMPT_BOUNDARY_RULES` (unlike `EXEMPT_SHEET_RULES`'s unconditional
/// WCAG 1.4.3 carve-out) names a specific rule and can go stale the
/// same way.
#[test]
fn every_exempt_boundary_rule_is_still_live() {
    let result = boundary_leak_scan();
    assert!(
        result.stale_exemptions.is_empty(),
        "these EXEMPT_BOUNDARY_RULES (selector, property) pairs no longer \
         appear as a below-/70 rule in any served sheet -- the vendored \
         CSS moved and this exemption is now stale:\n{}",
        result
            .stale_exemptions
            .iter()
            .map(|(s, p)| format!("  {s} {p}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
