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
    r.build_from_source = r
        .build_from_source
        .replace("1.88 or later", "1.85 or later");
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
    r.build_from_source = r
        .build_from_source
        .replace("# rustc 1.88.x", "# rustc 1.85.x");
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
    r.deployment = r.deployment.replace("    calendar.js\n", "");
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
    let needle = table_default(&r.configuration, "BIND_ADDR").expect("BIND_ADDR row");
    r.configuration = r
        .configuration
        .replacen(&format!("`{needle}`"), "`0.0.0.0:9999`", 1);
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

/// `WIZ-002`'s own review, `§10.36`'s class: `literal_string_default`/
/// `literal_bool_default` anchor on `config.rs`'s own literal text
/// (`env::var("DATABASE_URL")`), and a refactor that moves that
/// literal -- a shared constant, a different call shape -- used to
/// make the lookup return `None`, which the `if let` above simply did
/// not match: zero violations, nothing examined. These three plant
/// exactly that shape, one call site at a time, and must see rule 4
/// name the missing variable rather than go quiet.
#[test]
fn rule_4_reports_rather_than_silently_skips_when_database_url_s_literal_moves() {
    let mut r = real();
    r.config_rs = r
        .config_rs
        .replace("env::var(\"DATABASE_URL\")", "env::var(DATABASE_URL_VAR)");
    let v = only(run(&r), "rule 4:");
    assert!(
        v.iter()
            .any(|m| m.contains("DATABASE_URL") && m.contains("call shape")),
        "got {v:?}"
    );
}

#[test]
fn rule_4_reports_rather_than_silently_skips_when_bind_addr_s_literal_moves() {
    let mut r = real();
    r.config_rs = r
        .config_rs
        .replace("env::var(\"BIND_ADDR\")", "env::var(BIND_ADDR_VAR)");
    let v = only(run(&r), "rule 4:");
    assert!(
        v.iter()
            .any(|m| m.contains("BIND_ADDR") && m.contains("call shape")),
        "got {v:?}"
    );
}

/// `CFG-001` moved the anchor from `env::var("COOKIE_SECURE")` to the
/// `resolve_cookie_secure` function's own name (see
/// `literal_bool_default`'s doc comment) -- this plants that
/// function's own literal moving, not the `env::var` call, which is
/// no longer what the rule reads.
#[test]
fn rule_4_reports_rather_than_silently_skips_when_cookie_secure_s_literal_moves() {
    let mut r = real();
    r.config_rs = r.config_rs.replace(
        "fn resolve_cookie_secure(",
        "fn resolve_cookie_secure_renamed(",
    );
    let v = only(run(&r), "rule 4:");
    assert!(
        v.iter()
            .any(|m| m.contains("COOKIE_SECURE") && m.contains("call shape")),
        "got {v:?}"
    );
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

/// `DIA-003` §2's own table, case 1: an uncommented key whose value
/// disagrees with config.rs's default fails.
#[test]
fn rule_7_catches_an_uncommented_default_that_disagrees_with_config_rs() {
    let mut r = real();
    r.env_example = format!("{}\nBIND_ADDR=0.0.0.0:3000\n", r.env_example);
    let v = only(run(&r), "rule 7:");
    assert!(v.iter().any(|m| m.contains("BIND_ADDR")), "got {v:?}");
}

/// `DIA-003` §2's own table, case 2: the same line, commented, passes
/// -- a commented line is documentation, not a live override.
#[test]
fn rule_7_passes_the_same_line_commented() {
    let mut r = real();
    r.env_example = format!("{}\n# BIND_ADDR=0.0.0.0:3000\n", r.env_example);
    let v = only(run(&r), "rule 7:");
    assert!(v.is_empty(), "got {v:?}");
}

/// `DIA-003` §2's own table, case 3: an uncommented key no known
/// source recognises fails, rather than being silently skipped.
#[test]
fn rule_7_catches_an_unrecognised_key() {
    let mut r = real();
    r.env_example = format!("{}\nFRONTEND_URL=https://example.com\n", r.env_example);
    let v = only(run(&r), "rule 7:");
    assert!(
        v.iter()
            .any(|m| m.contains("FRONTEND_URL") && m.contains("no known source")),
        "got {v:?}"
    );
}

/// `JWT_SECRET` has no default to disagree with -- the violation is
/// the line being uncommented at all, regardless of its value. This is
/// the shape of the original defect (`§10.40`, 0.53.0) this rule
/// exists to make checkable.
#[test]
fn rule_7_catches_jwt_secret_uncommented_with_any_value() {
    let mut r = real();
    r.env_example = format!(
        "{}\nJWT_SECRET=change-me-to-a-long-random-string\n",
        r.env_example
    );
    let v = only(run(&r), "rule 7:");
    assert!(v.iter().any(|m| m.contains("JWT_SECRET")), "got {v:?}");
}

/// A known key with no fixed default to compare -- required with no
/// fallback at all (`SMTP_HOST`) -- passes rather than being flagged,
/// since there is nothing for an uncommented value to silently
/// override.
#[test]
fn rule_7_passes_a_known_key_with_no_fixed_default() {
    let mut r = real();
    r.env_example = format!("{}\nSMTP_HOST=smtp.example.com\n", r.env_example);
    let v = only(run(&r), "rule 7:");
    assert!(v.is_empty(), "got {v:?}");
}

/// `SMTP_PORT` and `RUST_LOG` do have fixed defaults, from sources
/// other than `config.rs` -- confirms rule 7 reaches both, not just
/// the `peisear-web` ones.
#[test]
fn rule_7_catches_a_mismatched_default_from_a_non_config_rs_source() {
    let mut r = real();
    r.env_example = format!("{}\nRUST_LOG=debug\nSMTP_PORT=587\n", r.env_example);
    let v = only(run(&r), "rule 7:");
    assert!(v.iter().any(|m| m.contains("RUST_LOG")), "got {v:?}");
    assert!(v.iter().any(|m| m.contains("SMTP_PORT")), "got {v:?}");
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
            "fn resolve_cookie_secure(v: Option<&str>) -> Result<bool, ConfigError> {\n    \
             match v {\n        None => Ok(false),\n        Some(_) => todo!(),\n    }\n}",
            "resolve_cookie_secure"
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
    assert_eq!(
        rust_log_default(
            "fn build_env_filter(raw: Option<String>) -> Result<EnvFilter, RustLogError> {\n    \
             const DEFAULT_FILTER: &str = \"info,sqlx=warn\";\n    todo!()\n}"
        ),
        Some("info,sqlx=warn".into())
    );
    assert_eq!(
        smtp_port_default(
            "fn resolve_smtp_port(raw: Option<&str>) -> Result<u16, SmtpConfigError> {\n    \
             match raw {\n        None => Ok(465),\n        Some(v) => todo!(),\n    }\n}"
        ),
        Some(465)
    );
    assert_eq!(
        env_example_entries(
            "# comment\nBIND_ADDR=0.0.0.0:3000\n\n# JWT_SECRET=change-me\nFOO = bar \n"
        ),
        vec![
            ("BIND_ADDR".to_string(), "0.0.0.0:3000".to_string()),
            ("FOO".to_string(), "bar".to_string()),
        ]
    );
    // Same guard as rule 6's own, above: confirm each new parser finds
    // its real target rather than only a synthetic one, so a rename or
    // refactor in main.rs/peisear-notify that silently stops matching
    // is a failing assertion here, not a rule that quietly checks
    // nothing against the real sources.
    let real = real();
    assert_eq!(
        rust_log_default(&real.main_rs),
        Some("info,sqlx=warn,hyper=warn,tower_http=info".into()),
        "main.rs's RUST_LOG fallback no longer matches rust_log_default's own pattern"
    );
    assert_eq!(
        smtp_port_default(&real.notify_config_rs),
        Some(465),
        "peisear-notify's SMTP_PORT fallback no longer matches smtp_port_default's own pattern"
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

/// Live, not `#[ignore]`d. `DOC-002` closed the path-spelling gap (19
/// citations); `GRD-003` closes the bare-file-name gap the path needle
/// could never see (64 citations across 21 files, the same shape
/// `§10.36` names: a rule named for a subject and keyed on a string
/// enforces the string). Both needles summed here -- a citation that
/// spells the path and one that spells the bare name are the same
/// defect under this rule's own name.
#[test]
fn no_document_rfc_or_published_doc_comment_cites_a_private_file() {
    let mut files = docs_and_rfcs_files();
    files.extend(published_doc_comment_lines());
    let path_count = git_exclude_citations(&files);
    let review_count = review_file_citation_count(&files);
    assert_eq!(
        path_count, 0,
        "{path_count} citations to .git-exclude/ remain outside the named exceptions"
    );
    assert_eq!(
        review_count, 0,
        "{review_count} bare review-file-name citations remain -- extract the substance \
         or re-point to a published handoff, never publish the private file itself"
    );
}

/// `GRD-003 §4` constraint 2: both house-style spellings, with and
/// without `.md`, must be caught.
#[test]
fn review_file_citations_catches_both_spellings() {
    let with_md = "see `QA-002-review.md` §3 for the reasoning";
    let without_md = "see `QA-002-review` §3 for the reasoning";
    assert_eq!(review_file_citations(with_md), vec!["QA-002-review.md"]);
    assert_eq!(review_file_citations(without_md), vec!["QA-002-review"]);
}

/// `GRD-003 §4` constraint 3 and `§1`'s own residual: an ordinary
/// hyphenated prose word ending in "review" must not match, and must
/// not need an exception list to exclude -- the identifier-shape check
/// on the prefix is what does this, not a name on a list.
#[test]
fn review_file_citations_does_not_match_ordinary_prose() {
    let text = "the author is mid-review; this needs a design-review before merge";
    assert!(
        review_file_citations(text).is_empty(),
        "an ordinary lowercase-prefixed word ending in -review must not match: {:?}",
        review_file_citations(text)
    );
}

/// A longer word built on the suffix (`reviewed`/`reviewing`/`reviewer`)
/// must not match -- the right-hand boundary check, not just the left.
#[test]
fn review_file_citations_does_not_match_a_longer_word_on_the_suffix() {
    let text = "QA-002-reviewed this already; QA-002-reviewer noted it; QA-002-reviewing now";
    assert!(
        review_file_citations(text).is_empty(),
        "a longer word built on the suffix must not match: {:?}",
        review_file_citations(text)
    );
}

/// Widened past `-review` alone (`GRD-003 §4`'s residual): the private
/// area's other real suffixes, `-decision`/`-decisions`/
/// `-recommendation`, measured against its own listing, not assumed.
#[test]
fn review_file_citations_catches_the_widened_suffixes() {
    let text = "`ISSUE-006-decision.md` says one thing, `DOC-900-decisions` another, \
                `QA-900-recommendation.md` a third";
    let mut found = review_file_citations(text);
    found.sort();
    let mut expected = vec![
        "ISSUE-006-decision.md".to_string(),
        "DOC-900-decisions".to_string(),
        "QA-900-recommendation.md".to_string(),
    ];
    expected.sort();
    assert_eq!(found, expected);
}

/// `GRD-003 §10`'s own exit condition: the widened rule must be shown
/// failing on a planted citation, not merely passing because nothing
/// was ever tried against it.
#[test]
fn a_planted_bare_review_citation_is_caught() {
    let files = vec![(
        "x.md".to_string(),
        "see `QA-999-review.md` §1 and `TT-999-review` §2".to_string(),
    )];
    assert_eq!(review_file_citation_count(&files), 2);
}
