use super::*;

#[test]
fn a_continued_string_joins_before_splitting() {
    let source = "class=grow(\"inline-flex items-center gap-1 text-sm text-base-content/70 \\\n                  hover:text-base-content mb-3\")";
    let spans = quoted_string_spans(source);
    assert_eq!(
        spans.len(),
        1,
        "exactly one string literal in this snippet: {spans:?}"
    );
    let (quote_pos, content_start, content_end) = spans[0];
    assert!(is_token_source(source, quote_pos));
    let tokens = tokens_from_content(&source[content_start..content_end]);
    assert_eq!(
        tokens,
        vec![
            "inline-flex",
            "items-center",
            "gap-1",
            "text-sm",
            "text-base-content/70",
            "hover:text-base-content",
            "mb-3",
        ]
    );
    assert!(
        !tokens.iter().any(|t| t == "\\"),
        "the continuation backslash must not survive as its own token: {tokens:?}"
    );
}

#[test]
fn an_interpolated_token_is_skipped_not_flagged() {
    let source = r#"class="btn badge-{status}""#;
    let spans = quoted_string_spans(source);
    let (quote_pos, content_start, content_end) = spans[0];
    assert!(is_token_source(source, quote_pos));
    let tokens = tokens_from_content(&source[content_start..content_end]);
    assert_eq!(
        tokens,
        vec!["btn"],
        "the interpolated token must be dropped, not checked: {tokens:?}"
    );
}

#[test]
fn a_const_argument_contributes_no_tokens() {
    let source = "class=grow_text(MENU_FOCUS_RING)";
    let spans = quoted_string_spans(source);
    assert!(
        spans.is_empty(),
        "a bare identifier argument carries no string literal for this scan to find: {spans:?}"
    );
}

#[test]
fn a_prefix_token_does_not_match_a_longer_class() {
    assert!(!token_has_rule("btn", ".btn-ghost{color:red}"));
    assert!(token_has_rule(
        "btn",
        ".btn{color:red}.btn-ghost{color:blue}"
    ));
}

#[test]
fn boundary_characters_other_than_open_brace_are_accepted() {
    assert!(token_has_rule(
        "hover:underline",
        ".hover\\:underline:hover{text-decoration-line:underline}"
    ));
    assert!(token_has_rule(
        "md:flex",
        "@media (min-width:768px){.md\\:flex{display:flex}}"
    ));
    assert!(token_has_rule(
        "column-drop",
        ".column-drop.drag-over{outline:2px dashed}"
    ));
}

#[test]
fn escape_matches_the_pinned_stylesheets_own_convention() {
    assert_eq!(escape_css_token("md:flex"), "md\\:flex");
    assert_eq!(escape_css_token("w-1/2"), "w-1\\/2");
    assert_eq!(escape_css_token("mt-0.5"), "mt-0\\.5");
    assert_eq!(escape_css_token("text-[11px]"), "text-\\[11px\\]");
    assert_eq!(
        escape_css_token("focus-visible:!outline"),
        "focus-visible\\:\\!outline"
    );
}

#[test]
fn an_if_else_branch_resolves_to_its_enclosing_grow_call() {
    let source = "let board_classes = grow(if is_board {\n        \"join-item btn btn-sm btn-active btn-primary\"\n    } else {\n        \"join-item btn btn-sm btn-ghost\"\n    });";
    let spans = quoted_string_spans(source);
    assert_eq!(spans.len(), 2, "{spans:?}");
    for (quote_pos, _, _) in &spans {
        assert!(
            is_token_source(source, *quote_pos),
            "an if/else branch whose whole value is grow(...)'s sole argument must resolve as a token source"
        );
    }
}

#[test]
fn every_allowlist_entry_is_queried_by_some_static_js() {
    let files = static_js_files();
    assert!(
        !files.is_empty(),
        "found no static/*.js files -- the layout assumption this scan depends on may have changed"
    );
    for (name, note) in JS_HOOK_ALLOWLIST {
        let found = files
            .iter()
            .any(|p| contains_whole_selector(&fs::read_to_string(p).unwrap_or_default(), name));
        assert!(
            found,
            "allowlist entry {name:?} ({note}) is not queried by any static/*.js file -- \
             an allowlist entry that nothing queries is itself a defect"
        );
    }
}

#[test]
fn the_repository_satisfies_the_live_rule() {
    let result = run_scan();
    assert!(
        result.population > 0,
        "the scan examined zero tokens -- a scan that examined nothing must not be able to pass (§10.36)"
    );
    assert!(
        result.violations.is_empty(),
        "{} class token(s) have no rule in any shipped stylesheet and are not allowlisted:\n{}",
        result.violations.len(),
        result
            .violations
            .iter()
            .map(|v| format!("  {}: {:?}", v.file, v.token))
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert_eq!(
        (result.population, result.allowlisted),
        (291, 2),
        "the measured population or allowlist count drifted from this round's own baseline -- \
         re-measure (components/**/*.rs changed) and update this assertion with the reason, \
         never just the numbers"
    );
}
