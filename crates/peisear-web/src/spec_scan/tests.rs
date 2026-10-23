use super::*;

fn only(v: Vec<String>, rule: &str) -> Vec<String> {
    v.into_iter()
        .filter(|m| m.starts_with(&format!("{rule}: ")))
        .collect()
}

fn real_input() -> Input {
    let root = repo_root();
    Input {
        maintained_docs: maintained_docs(),
        doc_comments: published_doc_comments(),
        requirements: fs::read_to_string(root.join("docs/specification/requirements.md"))
            .expect("read requirements.md"),
    }
}

// ---------- rule 1: §10.N citations resolve to a real heading ----------

#[test]
fn heading_numbers_reads_the_maximal_digit_run() {
    let text = "### 10.4 Short\n\nprose\n\n### 10.47 Long\n";
    let h = heading_numbers(text);
    assert!(h.contains(&4));
    assert!(h.contains(&47));
    assert_eq!(h.len(), 2);
}

#[test]
fn heading_numbers_ignores_a_heading_not_at_the_start_of_the_line() {
    // Indented or quoted, never a real heading.
    let text = "> ### 10.4 Not a heading\n";
    assert!(heading_numbers(text).is_empty());
}

#[test]
fn section_citations_finds_every_occurrence_including_duplicates() {
    let text = "§10.20 is cited here and again, §10.20, and also §10.47.";
    assert_eq!(section_citations(text), vec![20, 20, 47]);
}

#[test]
fn section_citations_reads_the_maximal_digit_run_not_a_prefix() {
    // `§10.4` must not be read out of `§10.47`.
    let text = "see §10.47 for detail";
    assert_eq!(section_citations(text), vec![47]);
}

#[test]
fn a_dangling_citation_is_caught() {
    let input = Input {
        maintained_docs: vec![(
            "docs/specification/requirements.md".into(),
            "Cites §10.99 which does not exist. ### 10.1 Real heading\n".into(),
        )],
        doc_comments: vec![],
        requirements: "### 10.1 Real heading\n".into(),
    };
    let v = only(violations(&input), "rule 1");
    assert_eq!(
        v.len(),
        1,
        "expected exactly one rule 1 violation, got {v:?}"
    );
    assert!(v[0].contains("§10.99"));
}

#[test]
fn a_citation_with_a_real_heading_is_not_flagged() {
    let input = Input {
        maintained_docs: vec![(
            "docs/some-doc.md".into(),
            "See §10.1 for the history.\n".into(),
        )],
        doc_comments: vec![],
        requirements: "### 10.1 Real heading\n".into(),
    };
    assert!(only(violations(&input), "rule 1").is_empty());
}

#[test]
fn rule_1_is_scoped_to_docs_and_never_reaches_rfcs() {
    assert!(in_scope("docs/specification/requirements.md"));
    assert!(!in_scope(
        "docs/specification/history/peisear-0.19.1-requirements-en.md"
    ));
    assert!(!in_scope("rfcs/handoffs/some-round/SOME-001-a-handoff.md"));
    assert!(!in_scope("rfcs/accepted/018-some-rfc.md"));
}

#[test]
fn rule_1_is_green_on_the_real_tree_for_every_heading_that_exists() {
    // Every citation to a number that DOES have a ### 10.N heading must
    // never be flagged -- the positive half of rule 1, proven against
    // the real tree rather than only a synthetic Input.
    let input = real_input();
    let headings = heading_numbers(&input.requirements);
    let v = only(violations(&input), "rule 1");
    for m in &v {
        let n: u32 = m
            .split("§10.")
            .nth(1)
            .and_then(|s| s.split(',').next())
            .and_then(|s| {
                s.trim_end_matches(|c: char| !c.is_ascii_digit())
                    .parse()
                    .ok()
            })
            .expect("violation names a number");
        assert!(
            !headings.contains(&n),
            "{m} names a number that does have a heading"
        );
    }
}

/// `SPEC-001` round 2: the thirteen `docs/` citations to `§10.20`,
/// `§10.22`, `§10.23` and `§10.24` that round 1 found and the
/// architect's own review resolved -- all four were real, already-
/// closed defects recorded only in `requirements.md`'s own older
/// compact `§10.N | description | status` table, never promoted to
/// the `### 10.N` heading convention every neighbouring number
/// eventually received. Promoted (content kept to what the table row
/// and its own citing sentences already said, per the review's own
/// budget), so rule 1 asserts clean now rather than naming a known,
/// bounded, pending set -- the claim round 1 could not honestly make.
#[test]
fn rule_1_is_clean_on_the_real_tree() {
    let input = real_input();
    let v = only(violations(&input), "rule 1");
    assert!(v.is_empty(), "rule 1 found real dangling citations: {v:?}");
}

// ---------- rule 1's own excluded population: rfcs/, counted not enforced ----------

/// `SPEC-001` §3: `rfcs/` is out of rule 1's scope entirely, so a
/// dangling `§10.N` citation inside a closed handoff is a historical
/// fact to count, not a live defect this rule asserts against. This
/// names the population and its count rather than only excluding it
/// silently -- `§10.36`'s own shape for a scope decision that hides
/// what it scoped out. Walks `rfcs/` directly (not through `in_scope`,
/// which already says no to every path under it) to measure the
/// excluded population on its own terms.
///
/// **Excludes this very series' own handoff directory**
/// (`050-the-specification-checks-itself/`), for the same reason rule
/// 2's own doc comment cannot cite a literal four-digit example: a
/// document describing the citation pattern in prose, to scope this
/// round, inevitably contains the same literal numbers as examples --
/// measured directly, first without this exclusion: the naive count
/// was **11**, not the expected 3, entirely from `SPEC-001`'s own
/// table and this series' own `README.md` both describing the
/// population rather than citing the numbers. Not yet closed history
/// either way -- this round is still open -- so excluding it is not
/// the shipped-history precedent rule 1 itself relies on; it is
/// simply not part of what the census was ever about.
#[test]
fn rfcs_citations_to_the_four_numbers_are_counted_not_enforced() {
    let root = repo_root();
    let mut files = Vec::new();
    walk_markdown_unfiltered(&root, &root.join("rfcs"), &mut files);
    files.retain(|(path, _)| {
        !path.starts_with("rfcs/handoffs/050-the-specification-checks-itself/")
    });
    let dangling: BTreeSet<u32> = [20, 22, 23, 24].into_iter().collect();
    let mut count = 0usize;
    for (_, text) in &files {
        for n in section_citations(text) {
            if dangling.contains(&n) {
                count += 1;
            }
        }
    }
    assert_eq!(
        count, 3,
        "SPEC-001 §3's own measured count of dangling-number citations inside rfcs/ -- a \
         historical record, not enforced; update this count only if the real population changed"
    );
}

fn walk_markdown_unfiltered(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk_markdown_unfiltered(root, &p, out);
        } else if p.extension().is_some_and(|e| e == "md")
            && let Ok(text) = fs::read_to_string(&p)
        {
            let rel = p
                .strip_prefix(root)
                .expect("under root")
                .to_string_lossy()
                .replace('\\', "/");
            out.push((rel, text));
        }
    }
}

// ---------- rule 2: no four-digit RFC 0NNN reference outside history/ ----------

#[test]
fn four_digit_rfc_citations_matches_exactly_four_digits() {
    assert_eq!(four_digit_rfc_citations("RFC 0017 is old"), vec!["0017"]);
    assert!(four_digit_rfc_citations("RFC 018 is current").is_empty());
    assert!(four_digit_rfc_citations("RFC 00175 is five digits, not four").is_empty());
}

#[test]
fn a_four_digit_reference_in_a_maintained_doc_is_caught() {
    let input = Input {
        maintained_docs: vec![(
            "docs/some-doc.md".into(),
            "See RFC 0017 for the history.\n".into(),
        )],
        doc_comments: vec![],
        requirements: String::new(),
    };
    let v = only(violations(&input), "rule 2");
    assert_eq!(
        v.len(),
        1,
        "expected exactly one rule 2 violation, got {v:?}"
    );
    assert!(v[0].contains("RFC 0017"));
}

#[test]
fn a_four_digit_reference_in_a_published_doc_comment_is_caught() {
    let input = Input {
        maintained_docs: vec![],
        doc_comments: vec![(
            "crates/peisear-core/src/lib.rs".into(),
            "//! See RFC 0017.\n".into(),
        )],
        requirements: String::new(),
    };
    let v = only(violations(&input), "rule 2");
    assert_eq!(
        v.len(),
        1,
        "expected exactly one rule 2 violation, got {v:?}"
    );
}

#[test]
fn rule_2_is_green_on_the_real_tree() {
    // SPEC-001 §4: the four-digit form appears 43 times, every one of
    // them inside docs/specification/history/, which rule 1's own
    // `in_scope` already excludes -- this is the rule that is "green
    // today, which is the problem" until it has been seen to fail.
    let input = real_input();
    let v = only(violations(&input), "rule 2");
    assert!(
        v.is_empty(),
        "rule 2 found real four-digit citations: {v:?}"
    );
}

// ---------- SPEC-002: entry parsing and the governing status ----------

#[test]
fn requirement_entries_stops_at_the_next_heading_not_only_the_next_marker() {
    // The heading-to-heading split artefact (`NFR-CMP-006`, documented at
    // requirements.md:6360-6400): a status-labelled line that belongs to
    // a non-entry section after the last entry in a group must not be
    // folded into that entry just because no further `**ID**` marker
    // appears before it.
    let text = "**NFR-X-001 — Title**\nBody. *Status*: Implemented.\n\n## 6. Next section\n\n*Status*: Not defined.\n";
    let entries = requirement_entries(text);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].status_token.as_deref(), Some("Implemented"));
}

#[test]
fn requirement_entries_handles_an_area_segment_containing_a_digit() {
    // Trap 1: `(FR|NFR)-[A-Z]+-[0-9]+` matches none of these. The real
    // parser splits on the last `-` before the numeric id, never on a
    // character class over the area segment.
    let text = "**NFR-A11Y-002 — Title**\n*Status*: Implemented.\n";
    let entries = requirement_entries(text);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].area, "A11Y");
    assert_eq!(entries[0].id, "NFR-A11Y-002");
}

#[test]
fn governing_status_token_is_the_last_one_in_the_body_not_the_first() {
    // `FR-NAV-003`'s own shape: an early status superseded by a later
    // correction, in the same entry body.
    let text = "**FR-X-001 — Title**\n*Status*: **Met at 0.46.0**.\nSome correction narrative.\n*Status*: Implemented. *Priority*: P2.\n";
    let entries = requirement_entries(text);
    assert_eq!(entries[0].status_token.as_deref(), Some("Implemented"));
}

#[test]
fn governing_status_token_matches_known_multi_word_tokens_before_falling_back() {
    assert_eq!(
        governing_status_token("*Status*: Not implemented — Phase E."),
        Some("Not implemented".to_string())
    );
    assert_eq!(
        governing_status_token("*Status*: **Met at 0.44.0**."),
        Some("Met at".to_string())
    );
    assert_eq!(
        governing_status_token("*Status*: Deferred."),
        Some("Deferred".to_string())
    );
}

#[test]
fn requirement_entries_is_168_on_the_real_tree() {
    // SPEC-002 §1: 99 FR + 69 NFR, matching Appendix A's own totals --
    // the population this whole round's checks depend on.
    let input = real_input();
    let entries = requirement_entries(&input.requirements);
    assert_eq!(entries.len(), 168, "expected 99 FR + 69 NFR entries");
    assert_eq!(
        entries.iter().filter(|e| e.id.starts_with("FR-")).count(),
        99
    );
    assert_eq!(
        entries.iter().filter(|e| e.id.starts_with("NFR-")).count(),
        69
    );
}

// ---------- SPEC-002 vocabulary: every real entry maps to a known column ----------

/// Closes `SPEC-002` §3: every one of the 168 entries' own governing
/// status has a leading token this rule recognises, and that token maps
/// to one of Appendix A's five columns. Planted by construction -- an
/// entry with no recognised token is exactly what `rule vocab:` reports,
/// proven on the real tree rather than only in the synthetic case above.
#[test]
fn every_real_entry_s_governing_status_is_in_the_closed_vocabulary() {
    let input = real_input();
    let v = only(appendix_violations(&input.requirements), "rule vocab");
    assert!(v.is_empty(), "unmapped or missing status tokens: {v:?}");
}

#[test]
fn an_entry_with_an_unrecognised_status_token_is_caught() {
    let text = "**FR-X-001 — Title**\n*Status*: Experimental.\n";
    let v = only(appendix_violations(text), "rule vocab");
    assert_eq!(v.len(), 1);
    assert!(v[0].contains("FR-X-001"));
}

// ---------- SPEC-002 Trap 1: the rule asserts the population it examined ----------

#[test]
fn a_new_area_segment_outside_the_known_census_is_reported_not_silently_dropped() {
    let text = "**NFR-ZZZ-001 — Title**\n*Status*: Implemented.\n";
    let v = only(appendix_violations(text), "rule b");
    assert!(
        v.iter().any(|m| m.contains("ZZZ")),
        "a new area segment must be named, not silently excluded: {v:?}"
    );
}

#[test]
fn known_area_census_is_exactly_25_segments() {
    assert_eq!(FR_AREAS.len(), 16);
    assert_eq!(NFR_AREAS.len(), 9);
    assert_eq!(AREA_NAMES.len(), 25);
}

// ---------- SPEC-002 rule (a): Appendix A's own arithmetic ----------

#[test]
fn appendix_a_rows_reads_both_divergent_cell_shapes() {
    // Health's own "1 (+1 unimplemented)" cell sums to 2; a plain
    // "2 unimplemented" cell (Privacy's) sums to 2 as well.
    let text = "## Appendix A\n\n| Area | Total | Implemented | Partial | Specified | Deferred | Divergent |\n|---|---|---|---|---|---|---|\n| Health | 9 | 5 | 2 | — | — | 1 (+1 unimplemented) |\n| Privacy | 8 | 6 | — | — | — | 2 unimplemented |\n\n## Appendix B\n";
    let rows = appendix_a_rows(text);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].cells, [9, 5, 2, 0, 0, 2]);
    assert_eq!(rows[1].cells, [8, 6, 0, 0, 0, 2]);
}

#[test]
fn a_row_whose_columns_do_not_sum_to_its_total_is_caught() {
    // `§10`'s own first row: two compensating cell errors leave the
    // total looking right unless every cell is read, not only recomputed from the total.
    let text = "## Appendix A\n\n| Area | Total | Implemented | Partial | Specified | Deferred | Divergent |\n|---|---|---|---|---|---|---|\n| Authentication | 5 | 6 | — | — | — | — |\n| **Functional total** | **5** | **6** | **0** | **0** | **0** | **0** |\n\n## Appendix B\n";
    let v = only(appendix_violations(text), "rule a");
    assert!(
        v.iter().any(|m| m.contains("Authentication")),
        "expected the row's own arithmetic to be caught: {v:?}"
    );
}

#[test]
fn a_grand_total_not_matching_its_member_rows_is_caught() {
    let text = "## Appendix A\n\n| Area | Total | Implemented | Partial | Specified | Deferred | Divergent |\n|---|---|---|---|---|---|---|\n| Authentication | 5 | 5 | — | — | — | — |\n| **Functional total** | **9** | **9** | **0** | **0** | **0** | **0** |\n\n## Appendix B\n";
    let v = only(appendix_violations(text), "rule a");
    assert!(
        v.iter().any(|m| m.contains("Functional total")),
        "expected the grand total's own mismatch against its one member row to be caught: {v:?}"
    );
}

#[test]
fn rule_a_is_green_on_the_real_tree() {
    let input = real_input();
    let v = only(appendix_violations(&input.requirements), "rule a");
    assert!(
        v.is_empty(),
        "Appendix A's own arithmetic does not sum: {v:?}"
    );
}

// ---------- SPEC-002 rule (b): the table against the entries ----------

#[test]
fn a_real_entry_count_disagreeing_with_the_table_is_caught() {
    let text = "## Appendix A\n\n| Area | Total | Implemented | Partial | Specified | Deferred | Divergent |\n|---|---|---|---|---|---|---|\n| Authentication | 1 | 1 | — | — | — | — |\n\n## Appendix B\n\n**FR-AUTH-001 — A**\n*Status*: Implemented.\n**FR-AUTH-002 — B**\n*Status*: Implemented.\n";
    let v = only(appendix_violations(text), "rule b");
    assert!(
        v.iter()
            .any(|m| m.contains("Authentication") && m.contains("2 real entries")),
        "expected the per-area entry count mismatch: {v:?}"
    );
}

/// `SPEC-002` §4(b), §6: checked on the real tree, rule (b) disagrees
/// with Appendix A in nine areas, none of them a vocabulary problem --
/// the vocabulary closure above is green on its own. Each is a row this
/// document's own narrative (`requirements.md:7394`-`:7409`) already
/// admits was never re-derived, or was re-derived before an entry it
/// covers changed again afterward (`Privacy`'s own `NFR-PRIV-005`/`-007`/
/// `-008`, all now `Met`/`Implemented`, dated at or after the 0.43.0
/// re-derivation the Divergent cell still reflects). **Asserted as the
/// known, exact, bounded set rather than claimed clean** -- the same
/// honest alternative `SPEC-001` round 1 used for a citation population
/// this round is not authorised to fix by rewriting the table
/// (`SPEC-002` §6: "stop and report the disagreement with both numbers;
/// do not edit the table to match"). Escalated in full in this round's
/// own completion report.
#[test]
fn rule_b_s_known_pending_disagreements_are_exactly_these_and_no_others() {
    let input = real_input();
    let v = only(appendix_violations(&input.requirements), "rule b");
    let expected: BTreeSet<&str> = [
        "rule b: Sub-issues's own entries mapped to Implemented number 10, Appendix A says 9",
        "rule b: Sub-issues's own entries mapped to Partial number 0, Appendix A says 1",
        "rule b: Sprints's own entries mapped to Implemented number 4, Appendix A says 3",
        "rule b: Sprints's own entries mapped to Specified number 0, Appendix A says 1",
        "rule b: Sprint planning's own entries mapped to Implemented number 4, Appendix A says 0",
        "rule b: Sprint planning's own entries mapped to Specified number 1, Appendix A says 5",
        "rule b: Health's own entries mapped to Implemented number 8, Appendix A says 5",
        "rule b: Health's own entries mapped to Partial number 1, Appendix A says 2",
        "rule b: Health's own entries mapped to Divergent number 0, Appendix A says 2",
        "rule b: Notifications's own entries mapped to Implemented number 9, Appendix A says 6",
        "rule b: Notifications's own entries mapped to Partial number 0, Appendix A says 1",
        "rule b: Notifications's own entries mapped to Specified number 0, Appendix A says 2",
        "rule b: Calendar's own entries mapped to Implemented number 8, Appendix A says 1",
        "rule b: Calendar's own entries mapped to Specified number 0, Appendix A says 7",
        "rule b: Privacy's own entries mapped to Implemented number 8, Appendix A says 6",
        "rule b: Privacy's own entries mapped to Divergent number 0, Appendix A says 2",
        "rule b: Accessibility's own entries mapped to Deferred number 0, Appendix A says 1",
        "rule b: Accessibility's own entries mapped to Divergent number 3, Appendix A says 2",
        "rule b: Language's own entries mapped to Implemented number 2, Appendix A says 1",
        "rule b: Language's own entries mapped to Partial number 1, Appendix A says 2",
    ]
    .into_iter()
    .collect();
    let actual: BTreeSet<&str> = v.iter().map(String::as_str).collect();
    assert_eq!(
        actual, expected,
        "rule b's real-tree result changed -- either a real fix landed (shrink the \
         expected set and say so in the report) or a new disagreement appeared"
    );
}

// ---------- the 24 numbers in both the compact register and the full heading ----------

/// `SPEC-001` §1a / `SPEC-002` §3a, carried forward and closed here as
/// "record, do not build a second checker": `requirements.md` has 45
/// `### 10.N` heading entries and (pre-`SPEC-001` round 2) 28 rows in an
/// older compact register, 24 numbers in both. The two forms use
/// unrelated vocabularies -- the compact register's own values are
/// `Closed` / `Open` / `Closed in tree`, nothing like the `FR`/`NFR`
/// status columns this round's vocabulary closes -- so there is no
/// cheap reuse of `status_column` or `governing_status_token` for it;
/// a second parser and a second mapping would have to be built and
/// maintained for a population this round was not asked to restructure.
/// **Left as a known, unchecked, named population**: 24 `§10.N` numbers
/// whose compact-register row and full-heading entry could disagree on
/// status with nothing here to notice. Whoever retires the duplication
/// (folding the compact register into the heading form, or vice versa)
/// removes the gap by removing one of the two homes, not by building a
/// third rule to compare them.
#[test]
fn the_24_dual_register_numbers_are_a_recorded_not_a_checked_population() {
    // Not a behavioural assertion -- this test exists so the doc comment
    // above has a place to live next to the code it describes, per this
    // rule's own standing requirement that every rule's gaps are written
    // down where a reader will find them.
}

#[test]
fn rule_2_is_green_on_the_real_tree_regression_guard_for_spec_002_edits() {
    // SPEC-002 touched requirements.md's own §10.20 entry (§3b) and
    // added no new RFC 0NNN-shaped text; re-run alongside the new rules
    // so a regression in either shows up in the same suite.
    let input = real_input();
    assert!(only(violations(&input), "rule 2").is_empty());
}
