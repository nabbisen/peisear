use super::*;

#[test]
fn no_hardcoded_prose_outside_the_message_table() {
    let unallowed: Vec<Violation> = all_violations()
        .into_iter()
        .filter(|v| !is_allowlisted(v))
        .collect();
    assert!(
        unallowed.is_empty(),
        "hardcoded prose found outside the message table (route through peisear_i18n::MessageKey, \
         or add a reasoned allowlist entry in prose_scan.rs):\n{}",
        unallowed
            .iter()
            .map(|v| format!("  {}:{} [{}] {:?}", v.file, v.line, v.kind, v.text))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// `I18N-007` §3's "prove it works": every allowlist entry must
/// still match something real. If a confirm() dialog is later
/// converted, its entry becomes dead weight that silently stops
/// testing anything -- this fails loudly instead.
#[test]
fn every_allowlist_entry_still_matches_something() {
    let violations = all_violations();
    for entry in ALLOWLIST {
        let matched = violations
            .iter()
            .any(|v| v.file.ends_with(entry.file) && v.text.contains(entry.snippet));
        assert!(
            matched,
            "allowlist entry for {} ({:?}) no longer matches any scanned literal -- \
             remove it, or if it was fixed, celebrate and remove it",
            entry.file, entry.snippet
        );
    }
}

/// `QA-002` item 3, test 6. Reproduces the exact false positive found
/// in `CONF-001`'s review: a doc comment that quotes attribute-shaped
/// text (the old `onsubmit="return confirm(...)"` defect,
/// documented in prose) must not be scanned as if it were real
/// markup, now that `strip_line_comments` removes it before the attr
/// scan sees it.
#[test]
fn a_doc_comment_quoting_attribute_markup_does_not_fail_the_scan() {
    let source = "//! An old defect quoted for context: onsubmit=\"return confirm('Delete?')\"\nfn example() {}\n";
    let without_comments = strip_line_comments(source);
    let mut out = Vec::new();
    scan_attrs(&without_comments, "fake/path.rs", &mut out);
    assert!(
        out.is_empty(),
        "a doc comment quoting attribute-shaped text must not be scanned as real markup: {out:?}"
    );
}

/// `QA-002` item 3, test 7 — the half that matters more (`§4`'s own
/// framing): a guard made quieter about comments is only an
/// improvement if it stayed loud where it should. A real
/// `aria-label` literal in real markup, on the same line shape the
/// scan expects, must still be caught after the port.
#[test]
fn a_real_literal_in_real_markup_still_fails_the_scan() {
    let source = "fn example() -> impl IntoView {\n    view! {\n        <button aria-label=\"Delete this thing\">{\"Delete\"}</button>\n    }\n}\n";
    let without_comments = strip_line_comments(source);
    let mut out = Vec::new();
    scan_attrs(&without_comments, "fake/path.rs", &mut out);
    assert!(
        !out.is_empty(),
        "a real aria-label literal in real markup must still be caught after the port"
    );
}
