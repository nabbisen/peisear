//! `GRAM-002` round 3 (`061`, `§10.59`) `§2` — the rendering-level
//! guard in `grammar_guard.rs` cannot see every shape of disagreement.
//! `ReadFirstLongStaleBody`'s own defect was a verb and a pronoun,
//! three and nine words past a number its hand-written `if` had
//! already inflected correctly on its own — no adjacency window, at
//! any width, can read that far without reading into unrelated
//! clauses (round 3's own `§1` measured exactly that, over the real
//! population, before this file existed). **So this file checks the
//! cause instead of the symptom**: the mechanism that produced an
//! unguardable disagreement was a hand-rolled `if … == 1 { "…" } else
//! { "…" }` choosing between two string literals outside `counted()`'s
//! own body. Assert that shape cannot come back, and the one defect
//! class this crate has no other way to catch stays caught.
//!
//! **This is a smaller claim than "no sentence disagrees," and it is
//! honest about being smaller.** It does not prove any two strings
//! agree with each other — `counted()` itself already guarantees that,
//! by construction, for everything that goes through it. It proves
//! the one mechanism that bypassed `counted()` and produced a
//! defect nothing else here could see is gone from `en.rs`. A new way
//! to hand-roll the same defect (an early `return`, a `match` on
//! `n == 1`, two different `format!` templates chosen by an `if`) is
//! not this test's job -- `§6`'s own prohibition on new machinery
//! applies here too, and the textual shape this test looks for is
//! exactly the shape `en.rs` is known to have had, not a generalised
//! parser for every way Rust could express the same choice.
//!
//! **Reads `en.rs` by path**, in the shape `peisear-web`'s
//! `prose_scan` already uses for source reading
//! (`env!("CARGO_MANIFEST_DIR")` plus `std::fs::read_to_string`), not
//! a second copy of a Rust parser. Line-based rather than a real
//! parser: this crate's own `cargo fmt` convention always puts an
//! `if <cond> {` condition and brace on one line, each branch's sole
//! expression on its own line, and `} else {` on its own line between
//! them -- true of every hand-rolled branch this project has ever
//! had, including the one this test exists to keep from coming back.
//! `counted()`'s own body is excluded by name (its branches are
//! `format!(...)` calls, not bare string literals, so the textual
//! shape below does not match it anyway -- excluded explicitly all
//! the same, so this test does not depend on that happening to stay
//! true).
//!
//! **`ConfirmDeleteIssueCascadeNote`'s `referent` is the one named
//! exception**, per the existing comment immediately above that match
//! arm in `en.rs`: *"'that sub-issue'/'all of them' has no number in
//! it and stays its own plain `if`, per this helper's own named
//! limit."* It chooses between two string literals on `sub_issue_count
//! == 1`, same textual shape as the defect, but the two literals carry
//! no number of their own to disagree with -- `counted()`'s whole job
//! is keeping a number and the text that follows it from saying two
//! different things about the same count, and this `if` has no count
//! in its own output to get wrong.

use std::fs;
use std::path::Path;

/// `en.rs`'s own source, read fresh each call rather than cached --
/// this is a test, not a hot path, and a cached read risks silently
/// checking a stale copy across a plant/restore cycle.
fn en_rs_source() -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src/en.rs");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

/// True if `line`, trimmed, is nothing but a double-quoted string
/// literal (with or without a trailing comma) -- a branch whose sole
/// content is a string, not a call, not a `format!`, not an
/// expression built from anything else.
fn is_bare_string_literal_line(line: &str) -> bool {
    let trimmed = line.trim();
    let inner = trimmed.strip_suffix(',').unwrap_or(trimmed);
    inner.len() >= 2 && inner.starts_with('"') && inner.ends_with('"')
}

/// Every line number (1-indexed) where a hand-rolled `if … == 1 { "…"
/// } else { "…" }` chooses between two bare string literals, in the
/// exact four-line shape `cargo fmt` gives it, outside `counted()`'s
/// own body (located textually and excluded by line range, not by
/// trusting that its `format!` branches would fail the string-literal
/// check anyway).
fn hand_rolled_literal_choice_lines(source: &str) -> Vec<usize> {
    let lines: Vec<&str> = source.lines().collect();

    // `counted()`'s own span, when this source has one -- optional
    // rather than required, so the same function can scan a bare
    // excerpt (the planted reproduction below) that has no `counted`
    // of its own.
    let counted_span = lines
        .iter()
        .position(|l| l.starts_with("fn counted("))
        .and_then(|start| {
            lines[start..]
                .iter()
                .position(|l| *l == "}")
                .map(|offset| (start, start + offset))
        });

    let mut hits = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if counted_span.is_some_and(|(start, end)| start <= i && i <= end) {
            continue;
        }
        if !line.trim_end().ends_with("== 1 {") {
            continue;
        }
        let Some(then_line) = lines.get(i + 1) else {
            continue;
        };
        let Some(else_marker) = lines.get(i + 2).map(|l| l.trim()) else {
            continue;
        };
        let Some(else_line) = lines.get(i + 3) else {
            continue;
        };
        if is_bare_string_literal_line(then_line)
            && else_marker == "} else {"
            && is_bare_string_literal_line(else_line)
        {
            hits.push(i + 1);
        }
    }
    hits
}

/// The one named exception (`§2`'s own words): `en.rs`'s
/// `ConfirmDeleteIssueCascadeNote` arm, matching on `sub_issue_count`.
/// Identified by the condition text on the hit line itself, not by
/// line number, so it survives the file being edited above it.
fn is_the_named_referent_exception(source: &str, line_no: usize) -> bool {
    source
        .lines()
        .nth(line_no - 1)
        .is_some_and(|l| l.contains("sub_issue_count == 1"))
}

/// The assertion: every hit is the one named exception, or there are
/// no hits at all.
#[test]
fn no_hand_rolled_if_outside_counted_chooses_between_two_string_literals() {
    let source = en_rs_source();
    let hits = hand_rolled_literal_choice_lines(&source);

    let unexplained: Vec<usize> = hits
        .into_iter()
        .filter(|&line_no| !is_the_named_referent_exception(&source, line_no))
        .collect();

    assert!(
        unexplained.is_empty(),
        "en.rs:{unexplained:?} -- a hand-rolled `if ... == 1 {{ \"...\" }} else {{ \"...\" }}` \
         chooses between two string literals outside counted()'s own body and outside the one \
         named exception (ConfirmDeleteIssueCascadeNote's referent); this is the mechanism \
         that produced ReadFirstLongStaleBody's own unguardable defect"
    );
}

/// "A guard never observed failing is not known to work." Plant the
/// `ccbf67c` text directly as a string, the same way
/// `grammar_guard.rs`'s own planted tests do, rather than only
/// relying on the file-copy plant over the real source (recorded in
/// this round's own report).
#[test]
fn guard_rejects_a_planted_hand_rolled_choice() {
    let planted = r#"        MessageKey::ReadFirstLongStaleBody { long_stale_count } => {
            let plural = if long_stale_count == 1 {
                "issue"
            } else {
                "issues"
            };
            format!(
                "{long_stale_count} {plural} haven't been touched in over two weeks. \
                 Closing or re-assigning them clears your queue."
            )
        }
"#;
    let hits = hand_rolled_literal_choice_lines(planted);
    assert_eq!(
        hits.len(),
        1,
        "expected exactly one hand-rolled choice in the ccbf67c reproduction text, found {hits:?}"
    );
}

/// The negative-space check, in two parts: `counted()`'s own body
/// (`format!` branches, not string literals) is never flagged, and
/// the named `referent` exception *is* found by the raw detector --
/// proving it is excluded deliberately in the real test above, not
/// silently absent from what the detector can see at all.
#[test]
fn guard_does_not_reject_counted_itself_and_does_find_the_named_exception() {
    let source = en_rs_source();
    let hits = hand_rolled_literal_choice_lines(&source);

    let counted_body = r#"fn counted(n: i64, singular: &str, plural: &str) -> String {
    if n == 1 {
        format!("{n} {singular}")
    } else {
        format!("{n} {plural}")
    }
}"#;
    assert!(
        hand_rolled_literal_choice_lines(counted_body).is_empty(),
        "counted()'s own if/else, whose branches are format!() calls rather than bare string \
         literals, must not be flagged"
    );

    assert!(
        hits.iter()
            .any(|&line_no| is_the_named_referent_exception(&source, line_no)),
        "expected the raw detector to find ConfirmDeleteIssueCascadeNote's referent choice -- \
         if it does not, the exception in the real test above is excluding nothing"
    );
}
