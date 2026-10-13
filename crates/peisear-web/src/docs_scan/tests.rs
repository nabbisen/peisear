use super::*;

#[test]
fn the_repository_satisfies_the_live_rules() {
    let r = real();
    let v = run(&r);
    assert!(v.is_empty(), "docs scan violations:\n{}", v.join("\n"));
}

#[test]
fn rule_1_catches_a_crate_missing_from_the_readme() {
    let mut r = real();
    r.readme = r.readme.replace("`i18n`, ", "");
    let v = only(run(&r), "rule 1:");
    assert!(v.iter().any(|m| m.contains("peisear-i18n")), "got {v:?}");
}

#[test]
fn rule_1_catches_a_crate_missing_from_workspace_layout() {
    let mut r = real();
    r.workspace_layout = r.workspace_layout.replace("peisear-i18n", "");
    let v = only(run(&r), "rule 1:");
    assert!(
        v.iter()
            .any(|m| m.contains("workspace-layout.md") && m.contains("peisear-i18n")),
        "got {v:?}"
    );
}

#[test]
fn rule_1_catches_a_crate_missing_from_crate_boundaries() {
    let mut r = real();
    r.crate_boundaries = r.crate_boundaries.replace("peisear-i18n", "");
    let v = only(run(&r), "rule 1:");
    assert!(
        v.iter()
            .any(|m| m.contains("crate-boundaries.md") && m.contains("peisear-i18n")),
        "got {v:?}"
    );
}

#[test]
fn rule_1b_catches_a_stale_count_in_workspace_layout() {
    let mut r = real();
    r.workspace_layout =
        r.workspace_layout
            .replacen("seven published crates", "six published crates", 1);
    let v = only(run(&r), "rule 1b:");
    assert!(
        v.iter()
            .any(|m| m.contains("workspace-layout.md") && m.contains("6 published crates")),
        "got {v:?}"
    );
}

#[test]
fn rule_1b_catches_a_stale_count_in_docs_readme() {
    let mut r = real();
    r.docs_readme = r.docs_readme.replacen(
        "seven crates and what belongs",
        "six crates and what belongs",
        1,
    );
    let v = only(run(&r), "rule 1b:");
    assert!(
        v.iter()
            .any(|m| m.contains("docs/README.md") && m.contains("6 crates")),
        "got {v:?}"
    );
}

#[test]
fn rule_1b_catches_a_stale_count_in_crate_boundaries_intro() {
    let mut r = real();
    r.crate_boundaries = r.crate_boundaries.replacen(
        "six implementation crates plus a thin facade",
        "five implementation crates plus a thin facade",
        1,
    );
    let v = only(run(&r), "rule 1b:");
    assert!(
        v.iter()
            .any(|m| m.contains("intro") && m.contains("5 implementation crates")),
        "got {v:?}"
    );
}

#[test]
fn rule_1b_catches_a_stale_count_in_crate_boundaries_heading() {
    let mut r = real();
    r.crate_boundaries = r.crate_boundaries.replacen(
        "## The six implementation crates",
        "## The five implementation crates",
        1,
    );
    let v = only(run(&r), "rule 1b:");
    assert!(
        v.iter()
            .any(|m| m.contains("heading") && m.contains("5 implementation crates")),
        "got {v:?}"
    );
}

#[test]
fn rule_2_catches_a_stale_msrv_sentence() {
    let mut r = real();
    r.installation = r.installation.replace("1.88 or later", "1.85 or later");
    let v = only(run(&r), "rule 2:");
    assert!(v.iter().any(|m| m.contains("1.85")), "got {v:?}");
}

#[test]
fn rule_2_catches_a_stale_msrv_in_contributing() {
    let mut r = real();
    r.contributing = r.contributing.replace("1.88 or later", "1.85 or later");
    let v = only(run(&r), "rule 2:");
    assert!(
        v.iter()
            .any(|m| m.contains("CONTRIBUTING.md") && m.contains("1.85")),
        "got {v:?}"
    );
}

#[test]
fn rule_2_catches_a_stale_verify_block() {
    let mut r = real();
    r.installation = r.installation.replace("# rustc 1.88.x", "# rustc 1.85.x");
    let v = only(run(&r), "rule 2:");
    assert!(v.iter().any(|m| m.contains("rustc 1.85.x")), "got {v:?}");
}

#[test]
fn rule_3_catches_a_manifest_entry_that_does_not_exist() {
    let mut r = real();
    r.deployment = r.deployment.replace("app.css", "app-renamed.css");
    let v = only(run(&r), "rule 3:");
    assert!(v.iter().any(|m| m.contains("app-renamed.css")), "got {v:?}");
}

#[test]
fn rule_3_catches_a_static_file_missing_from_the_manifest() {
    let mut r = real();
    r.deployment = r.deployment.replace("    ├── calendar.js\n", "");
    let v = only(run(&r), "rule 3:");
    assert!(
        v.iter()
            .any(|m| m.contains("static/calendar.js") && m.contains("not listed")),
        "got {v:?}"
    );
}

#[test]
fn rule_4_catches_a_documented_default_that_does_not_match_config_rs() {
    let mut r = real();
    r.configuration = r
        .configuration
        .replace("`0.0.0.0:3000`", "`127.0.0.1:3000`");
    let v = only(run(&r), "rule 4:");
    assert!(v.iter().any(|m| m.contains("BIND_ADDR")), "got {v:?}");
}

#[test]
fn rule_4_catches_a_documented_boolean_default_that_does_not_match_config_rs() {
    let mut r = real();
    let needle = table_default(&r.configuration, "COOKIE_SECURE").expect("COOKIE_SECURE row");
    r.configuration = r.configuration.replacen(&format!("`{needle}`"), "`1`", 1);
    let v = only(run(&r), "rule 4:");
    assert!(v.iter().any(|m| m.contains("COOKIE_SECURE")), "got {v:?}");
}

#[test]
fn rule_6_catches_a_raw_url_naming_a_file_that_does_not_exist() {
    let mut r = real();
    r.readme = format!(
        "{}\n![missing](https://github.com/nabbisen/peisear/raw/main/docs/src/assets/screenshots/does-not-exist.png)\n",
        r.readme
    );
    let v = only(run(&r), "rule 6:");
    assert!(
        v.iter().any(|m| m.contains("does-not-exist.png")),
        "got {v:?}"
    );
}

#[test]
fn the_helpers_do_what_the_rules_assume() {
    assert_eq!(short_crate_name("peisear-i18n"), "i18n");
    assert_eq!(short_crate_name("peisear"), "peisear");
    assert!(mentions_crate("the `peisear-core` crate", "peisear-core"));
    assert!(mentions_crate("Seven crates: `core`, ...", "peisear-core"));
    assert!(!mentions_crate("Six crates: `auth`, ...", "peisear-i18n"));
    assert_eq!(
        bold_or_later("needs `rustc` **1.88 or later**."),
        Some("1.88".into())
    );
    assert_eq!(word_to_number("seven"), Some(7));
    assert_eq!(word_to_number("eleven"), None);
    assert_eq!(
        number_before(
            "peisear is split into six implementation crates plus a thin facade",
            " implementation crates plus a thin facade"
        ),
        Some(6)
    );
    // The subset sentences rule 1b must not match: no fixed phrase shape
    // appears in either, so none of the extractors find anything to flag.
    assert_eq!(
        implementation_crate_count_intro("Depends on five of the six implementation crates"),
        None
    );
    assert_eq!(
        implementation_crate_count_heading("four of the other five crates depend on it"),
        None
    );
    assert_eq!(
        implementation_crate_count_heading("## The six implementation crates"),
        Some(6)
    );
    assert_eq!(
        published_crate_count("containing seven published crates plus static assets"),
        Some(7)
    );
    assert_eq!(
        readme_index_crate_count("why there are\n  seven crates and what belongs in each"),
        Some(7)
    );
    assert_eq!(
        verify_block_version("# rustc 1.88.x ...   (or newer)", "rustc"),
        Some("1.88".into())
    );
    assert_eq!(
        literal_string_default(
            "env::var(\"DATABASE_URL\").unwrap_or_else(|_| \"sqlite://data/app.db\".to_string());",
            "DATABASE_URL"
        ),
        Some("sqlite://data/app.db".into())
    );
    assert_eq!(
        literal_bool_default(
            "env::var(\"COOKIE_SECURE\").map(|v| v==\"1\").unwrap_or(false)",
            "COOKIE_SECURE"
        ),
        Some(false)
    );
    assert_eq!(
        table_default("| `BIND_ADDR` | `0.0.0.0:3000` | notes |", "BIND_ADDR"),
        Some("0.0.0.0:3000".into())
    );
    assert_eq!(
        manifest_static_files(
            "## What to ship\n\n```\n/var/lib/peisear/\n├── peisear\n└── static/\n    ├── app.css\n    └── board.js\n```\n"
        ),
        vec!["app.css".to_string(), "board.js".to_string()]
    );
    assert_eq!(
        raw_url_paths(
            "![a](https://github.com/nabbisen/peisear/raw/main/docs/a.png) and [b](https://github.com/nabbisen/peisear/raw/main/docs/b.png \"title\")"
        ),
        vec!["docs/a.png".to_string(), "docs/b.png".to_string()]
    );
    // `DOCS-008` review, F-1: rule 6 passes vacuously if `RAW_URL_PREFIX`
    // ever stops matching anything in README.md (a renamed repository, a
    // host change) -- the plant test can't catch that, since it builds
    // its own URL from the same literal. This is the guard: the real
    // README.md must still contain at least one URL the prefix matches.
    assert!(
        !raw_url_paths(&real().readme).is_empty(),
        "README.md has no raw_url_paths() match -- rule 6 would pass by matching nothing"
    );
}

#[test]
fn a_planted_git_exclude_citation_is_counted() {
    let files = vec![(
        "x.md".to_string(),
        "see .git-exclude/foo and .git-exclude/bar".to_string(),
    )];
    assert_eq!(git_exclude_citations(&files), 2);
}

/// `DOC-002` §7: both halves of the scope boundary, proven on the
/// literal paths the rule reasons about -- a document in scope, a
/// handoff that must not be, and all four named exceptions.
#[test]
fn rule_5_scope_catches_a_document_and_excludes_a_handoff() {
    assert!(in_rule_5_scope("docs/architecture/workspace-layout.md"));
    assert!(in_rule_5_scope("rfcs/done/999-example.md"));
    assert!(
        !in_rule_5_scope("rfcs/handoffs/999-example/README.md"),
        "a handoff is an execution record, not documentation"
    );
    assert!(!in_rule_5_scope(SELF_PATH));
    assert!(!in_rule_5_scope("docs/specification/README.md"));
    assert!(!in_rule_5_scope(
        "docs/specification/history/peisear-0.20.0-requirements-en.md"
    ));
    assert!(!in_rule_5_scope(
        "rfcs/accepted/019-what-is-in-the-repository.md"
    ));
}

/// Both halves again, this time against the real tree rather than
/// literal strings -- `rule_5_scope_catches_a_document_and_excludes_a_
/// handoff` proves the predicate; this proves the *walker* applies it
/// correctly to what is actually on disk, which a predicate test alone
/// cannot: a directory-traversal bug would still pass that one.
#[test]
fn rule_5_population_includes_docs_and_rfcs_but_excludes_handoffs() {
    let files = docs_and_rfcs_files();
    assert!(
        files.iter().any(|(p, _)| p.starts_with("docs/")),
        "expected at least one real docs/ file in rule 5's population"
    );
    assert!(
        files
            .iter()
            .any(|(p, _)| p.starts_with("rfcs/") && !p.starts_with("rfcs/handoffs/")),
        "expected at least one real rfcs/ file (not a handoff) in rule 5's population"
    );
    assert!(
        !files.iter().any(|(p, _)| p.starts_with("rfcs/handoffs/")),
        "rfcs/handoffs/ is an execution record, not documentation, and must not appear"
    );
    assert!(
        !files
            .iter()
            .any(|(p, _)| p == "docs/specification/README.md"),
        "the definition row's own document must be excepted"
    );
}

/// Live, not `#[ignore]`d: `DOC-002` closed the real gap (19 citations,
/// 15 of them resolved to a rewrite or a cut, four kept as the
/// exceptions above), so the rule that could never pass before now can,
/// and is enforced rather than merely reported.
#[test]
fn no_document_rfc_or_published_doc_comment_cites_git_exclude() {
    let mut files = docs_and_rfcs_files();
    files.extend(published_doc_comment_lines());
    let count = git_exclude_citations(&files);
    assert_eq!(
        count, 0,
        "{count} citations to .git-exclude/ remain outside the named exceptions"
    );
}
