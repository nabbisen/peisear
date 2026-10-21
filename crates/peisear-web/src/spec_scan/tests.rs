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

/// `SPEC-001` §2, measured 2026-10-22 at this round's own commit:
/// every `§10.N` citation in `docs/` to a number with no
/// `### 10.N` heading, named exactly rather than asserted away.
/// **All four are the same outcome** (`SPEC-001` §2 case 2: a real,
/// already-closed defect, described in `requirements.md`'s own older
/// compact `§10.N | description | status` table — a format that
/// predates the `### 10.N` heading convention every neighbouring
/// number eventually received — never promoted to it. Escalated to
/// the architect rather than resolved here (`SPEC-001` §7: "writing a
/// new §10.N entry is a specification change"), so this test names the
/// known, current, bounded set rather than asserting the rule is
/// clean — a claim this round cannot make honestly yet. It fails
/// loudly, not silently, the moment a *new* dangling citation appears,
/// or the moment these four are resolved and this list needs shrinking
/// to empty.
#[test]
fn rule_1_s_known_pending_violations_are_exactly_these_and_no_others() {
    let input = real_input();
    let mut v = only(violations(&input), "rule 1");
    v.sort();
    let mut expected: Vec<String> = Vec::new();
    for n in [20u32, 22, 23, 24] {
        // Count every occurrence by path, matching SPEC-001's own
        // census: each citation gets its own violation line.
        for (path, text) in &input.maintained_docs {
            for c in section_citations(text) {
                if c == n {
                    expected.push(format!(
                        "rule 1: {path} cites §10.{n}, which has no ### 10.{n} heading in requirements.md"
                    ));
                }
            }
        }
    }
    expected.sort();
    assert_eq!(
        v, expected,
        "rule 1's real-tree violations changed -- either a new dangling citation appeared, \
         or §10.20/22/23/24 were resolved and this pending list needs updating to reflect it"
    );
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
