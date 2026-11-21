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
fn every_allowlist_entry_is_queried_by_some_consumer() {
    let js_files = static_js_files();
    assert!(
        !js_files.is_empty(),
        "found no static/*.js files -- the layout assumption this scan depends on may have changed"
    );
    let gate_files = browser_check_files();
    assert!(
        !gate_files.is_empty(),
        "found no browser-checks/*.mjs files -- the layout assumption this scan depends on may have changed"
    );
    for (name, note) in JS_HOOK_ALLOWLIST {
        let found = js_files
            .iter()
            .chain(gate_files.iter())
            .any(|p| contains_whole_selector(&fs::read_to_string(p).unwrap_or_default(), name));
        assert!(
            found,
            "allowlist entry {name:?} ({note}) is not queried by any static/*.js or \
             browser-checks/*.mjs file -- an allowlist entry that nothing queries is itself \
             a defect"
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
    // `TYPE-001` (`§10.60`): population dropped 291 -> 290. `issues.rs`'s
    // and `projects.rs`'s own `text-[11px]` sites were raised to the
    // already-used `text-xs`, which removes that arbitrary-value token
    // from the live tree entirely rather than adding a new one --
    // `calendar.rs` still has one `text-[10px]` site left (kept, not
    // raised; see that file's own comment at the site), so the token
    // itself survives, just with one fewer distinct class string overall.
    // `LBL-001` (series `063`): population dropped 290 -> 289.
    // `settings.rs`'s own five `input-xs` sites (the "Close on" and
    // "Points edit" dropdowns) all moved to `input-sm` as part of the
    // cluster raise -- `input-xs` is no longer rendered anywhere
    // under `components/`, which is this scan's own scope (this
    // module's doc comment, "Files: `components/**/*.rs` only"), so
    // the token leaves this count entirely rather than merely
    // relocating within it; confirmed with `grep -rn 'input-xs'` over
    // live `class=` attributes, not assumed from the five sites
    // changed. **`input-xs` itself is not gone from the tree** — it
    // stays, correctly, in `touch_target_scan.rs`'s own
    // `SIZING_CLASSES` list, the guard that stops it being
    // reintroduced; that list is outside this scan's own scope and so
    // does not affect this number either way.
    // `SIZE-002`: allowlisted 2 -> 3, population unchanged at 289.
    // `issues.rs:717`'s workload chip carries a new named hook,
    // `workload-chip-name`, added so `overflow-gate.mjs` could stop
    // matching the chip by quoting a style class (`text-sm
    // font-medium`) that had already drifted once and, by this
    // round, matched seven sites. A named hook with no style of its
    // own is allowlisted rather than counted against the population
    // -- it is a new *distinct token*, so it adds one to
    // `allowlisted`, but nothing a stylesheet rule was ever checked
    // against left or joined the tree, so `population` does not
    // move. The four raised sites and the one converted spelling
    // this round touched (`teams.rs`) only changed which *existing*
    // token a site already in the population renders -- `text-xs`
    // and `text-sm`, `opacity-70` and `text-base-content/70`, are
    // each still rendered elsewhere -- so none of those five moved
    // this number either.
    // `SIZE-003`: population dropped 289 -> 288. `settings.rs:81`'s
    // own bare `opacity-80` converted to `text-base-content/80`,
    // which removed that token from `components/` entirely (it was
    // rendered at exactly that one site).
    // `SIZE-003` round 2: population returns 288 -> 289. The
    // conversion above is reverted -- `§10.60`'s single-level check
    // is necessary but not sufficient; this site's colour comes from
    // `alert-warning`'s own foreground token (`--wac`), not an
    // incidental one, and the guarded spelling measured a real
    // contrast drop here (9.01:1 -> 6.61:1) rather than the
    // equivalence the condition assumes. `opacity-80` re-enters
    // `components/` as a distinct token at that one site, confirmed
    // with `grep -rn 'opacity-80'` over live `class=` attributes, not
    // assumed from the one site reverted. Allowlisted unchanged at 3.
    // `CLR-001`: population unchanged at 289. `text-error` and
    // `btn-error` both still render at the three sites `CLR-003`
    // owns (`error_page.rs:18`, `confirmation.rs:56`/`:64`,
    // `projects.rs:286`), confirmed by `grep -rn`, so neither token
    // left the tree -- this round only moved which sites render an
    // already-present token.
    // `CLR-002`: population dropped 289 -> 287. `link-primary` and
    // `text-success` both leave `components/` entirely -- confirmed
    // by `grep -rn 'link-primary\b'` / `'text-success\b'` returning
    // no render sites, not assumed from the six sites changed.
    // Allowlisted unchanged at 3.
    assert_eq!(
        (result.population, result.allowlisted),
        (287, 3),
        "the measured population or allowlist count drifted from this round's own baseline -- \
         re-measure (components/**/*.rs changed) and update this assertion with the reason, \
         never just the numbers"
    );
}
