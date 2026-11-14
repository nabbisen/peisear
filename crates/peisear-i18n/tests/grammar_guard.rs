//! `GRAM-002` round 2 (`061`, `§10.59`) — round 1 fixed the fourteen
//! sentences it had already decided to look for and wrote
//! `rendering.rs`'s `gram_002_*` tests as `assert_eq!` lists over that
//! same fourteen. **That is the shape that could not see the original
//! defect**: a list can only pin what someone already found, the same
//! way `basis_route`/`health_explainability`'s substring checks could
//! not see a grammar defect outside the part they quoted. This file is
//! the guard the architect's review asked for instead: it renders
//! [`MessageKey::all`]'s entire population and reads every string, so
//! a count-bearing variant nobody has looked at yet is still covered.
//!
//! **The rule, `§3` of the round 2 handoff**: a number immediately
//! followed by the noun it counts must agree with that number. The
//! guard below is `\b1\s+([A-Za-z-]+s)\b` written out by hand rather
//! than taking `regex` on as a new direct dependency — no crate in
//! this workspace uses `regex` directly today (`Cargo.lock` carries it
//! only as some other crate's transitive dependency), and every other
//! scan in this crate (`language_guard.rs`, and `peisear-web`'s
//! `prose_scan`/`static_js_scan`) is hand-written for the same reason.
//! Word-splitting gives the same word-boundary behaviour the `\b`s ask
//! for "for free": `"1.0 issues"` tokenises as `"1.0"` and `"issues"`,
//! and `"1.0" != "1"`, so the float case the rule names is correctly
//! never flagged, the same way `\b1\b` would not match inside `1.0`.
//!
//! **The skip set is closed and is grammar, not an allowlist of
//! strings** — `is`, `as`, `vs`, `was`, `has` are words that end in
//! `s` without being the plural of anything;
//! `guard_does_not_reject_grammar_words_or_correctly_uninflected_shapes`
//! below is the negative-space check that keeps it that way, the same
//! role `guard_does_not_reject_legitimate_shipped_symbols` plays in
//! `language_guard.rs`.
//!
//! **Round 3 (`061`, `§10.59`) widens the adjacency, to two words, not
//! three.** Round 2's own rule said "immediately", and round 2
//! implemented that sentence exactly -- which is what let `"1
//! consecutive snapshots"` through: an adjective between the number
//! and its noun. A three-word window was tried first and measured
//! against the real population before being written down here: it
//! produced three false positives, each from reading straight past the
//! noun phrase the `1` actually governs and into an unrelated clause
//! (`NotificationBurnoutOverloadBody`'s `"1 consecutive snapshot. This"`
//! across a full stop, `BarChartAriaLabel`'s `"1 sprint, completed
//! points"` across a comma, `ReadFirstWipBody`'s `"1 issue in
//! progress;"` across a semicolon). **The noun a number counts is the
//! head of its own noun phrase, at most one modifier away in this
//! interface's English** -- `"1 issue"`, `"1 consecutive snapshot"`,
//! `"1 total event"` are all at most one word past the number. A
//! two-word window reaches every adjective case in the population and
//! none of the three false positives, without a fourth stop condition
//! or an allowlist for the three strings -- both of which were the
//! tempting wrong fix, named and declined for the same reason: a
//! punctuation stop would be machinery this window doesn't need, and
//! skipping `"progress"` would be the grammar-word skip set doing an
//! allowlist's job in a different hat.
//!
//! The scan looks ahead up to two words from a bare `1`, stopping at
//! the first digit-bearing token or at `of`/`or`/`/` -- the three that
//! turn a count into a set description (`"1 of last 14"`, `"1 or more
//! issues"`, `"1 / 12 of in-flight issues"`) -- and flags the first
//! plural-shaped word reached before a stop. The grammar-word skip set
//! keeps its job unchanged: skipped, not a stop, so `"1 was over
//! capacity"` keeps scanning past `"was"` and finds nothing.
//!
//! **Only `MessageKey::all()`'s own population is visible to this
//! guard.** Before this round, sixteen of the seventeen count-bearing
//! variants named in the handoff's own `§3` table carried no sample at
//! the number `1` at all (`ReadFirstLongStaleBody` was the one
//! exception) — invisible to a census that only reads what `all()`
//! hands it, regardless of what the rendering code does. `message.rs`
//! now carries a `1`-sample for each of this round's own seventeen
//! (`§10.59`'s own "GRAM-002 round 2" comments mark each one); a
//! count-bearing variant outside that seventeen that still lacks one
//! is a real, named, remaining gap in this guard's reach, not a case
//! this file claims to have ruled out — see this round's own report
//! for why that gap is not closed structurally here.
//!
//! **`ReadFirstLongStaleBody` is itself the proof the number+noun
//! check, run alone, is not sufficient.** Pre-fix, at
//! `long_stale_count: 1`, `en.rs`'s own hand-written `if` already
//! chose the singular noun ("1 issue"), so the word immediately after
//! `1` was never wrong — the defect was the verb ("haven't") and the
//! pronoun ("them"/"their") three and nine words further on, which
//! this guard's own adjacency check cannot see either, pre- or
//! post-fix. It is fixed in this round regardless, because the
//! handoff's `§2` step 3 says so categorically, not because this
//! guard caught it — see the round's own report for the verbatim
//! pre-fix run this claim is measured from, not predicted.

use peisear_i18n::{Locale, MessageKey};

/// Grammar words that end in `s` without being a plural noun agreeing
/// with the number `1` immediately before them — a closed set of five,
/// not a list grown to fit whatever string shows up next.
const GRAMMAR_WORDS_ENDING_IN_S: [&str; 5] = ["is", "as", "vs", "was", "has"];

/// True if `word`, once its leading/trailing punctuation is stripped,
/// is a bare plural-shaped word: ends in `s`, is made only of letters
/// and hyphens (so `"1%"`'s neighbour or a stray digit can't match),
/// has more than the one trailing `s` (so `"s"` alone, were it ever a
/// token, cannot match), and is not in the closed grammar-word skip
/// set.
fn is_plural_shaped(word: &str) -> bool {
    let trimmed = word.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-');
    trimmed.len() > 1
        && trimmed.ends_with('s')
        && trimmed.chars().all(|c| c.is_ascii_alphabetic() || c == '-')
        && !GRAMMAR_WORDS_ENDING_IN_S.contains(&trimmed.to_ascii_lowercase().as_str())
}

/// True if `word`, once its leading/trailing punctuation is stripped,
/// is exactly the digit `1` — not `"11"`, not `"1.0"`, not `"1,000"`.
fn is_bare_one(word: &str) -> bool {
    word.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-') == "1"
}

/// True if `word` contains a digit anywhere in it — `"14"`, `"1.0"`,
/// `"12"` — a sibling count the scan must not read past, since a
/// second number in the window means the first `1` is no longer the
/// count the trailing noun agrees with.
fn contains_digit(word: &str) -> bool {
    word.chars().any(|c| c.is_ascii_digit())
}

/// True if `word`, once its leading/trailing punctuation is stripped,
/// is one of the closed three that turn a count into a set
/// description rather than leaving it a count: `of` (`"1 of last
/// 14"`), `or` (`"1 or more issues"`), `/` (`"1 / 12 of in-flight
/// issues"`). Not an allowlist of strings -- these three are the
/// grammatical join words the round 3 handoff names, and the window
/// stops at the first one reached exactly because a noun past it
/// agrees with the set, not with `1`.
fn is_set_description_stop_word(word: &str) -> bool {
    let trimmed = word.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '/');
    matches!(trimmed.to_ascii_lowercase().as_str(), "of" | "or" | "/")
}

/// The guard itself. Round 2's own `\b1\s+([A-Za-z-]+s)\b`, widened by
/// round 3 to look ahead up to two words rather than one -- the
/// number and its noun may be separated by one adjective
/// (`"1 consecutive snapshots"`), but not by a word that ends the
/// noun phrase (a second number, or `of`/`or`/`/`), and not by more
/// than one modifier -- a three-word window read past the noun this
/// interface's English ever puts there and found other clauses
/// instead (measured, not assumed; see this file's own module doc).
/// Returns the offending span, from the `1` through the flagged word,
/// so a failure names what it found, not just that it found
/// something.
fn find_singular_plural_disagreement(text: &str) -> Option<String> {
    const WINDOW: usize = 2;
    let words: Vec<&str> = text.split_whitespace().collect();
    for i in 0..words.len() {
        if !is_bare_one(words[i]) {
            continue;
        }
        let window_end = (i + 1 + WINDOW).min(words.len());
        for (j, word) in words.iter().enumerate().take(window_end).skip(i + 1) {
            if contains_digit(word) || is_set_description_stop_word(word) {
                break;
            }
            if is_plural_shaped(word) {
                return Some(words[i..=j].join(" "));
            }
        }
    }
    None
}

/// The guard: every key [`MessageKey::all`] enumerates, rendered and
/// read, with every offender collected rather than stopping at the
/// first — `§5.1` of the round 2 handoff asks for the list the guard
/// finds, not just whether it found anything.
#[test]
fn no_rendered_message_disagrees_with_the_number_one() {
    let keys = MessageKey::all();
    assert!(
        !keys.is_empty(),
        "MessageKey::all() must enumerate at least the seeded set, or this test proves nothing"
    );

    let offenders: Vec<String> = keys
        .into_iter()
        .filter_map(|key| {
            let rendered = Locale::English.render(key.clone());
            find_singular_plural_disagreement(&rendered)
                .map(|span| format!("{key:?} -- {span:?} in {rendered:?}"))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "{} rendering(s) disagree with their own number:\n{}",
        offenders.len(),
        offenders.join("\n")
    );
}

/// "A guard never observed failing is not known to work" —
/// `language_guard.rs`'s own standard, held here the same way. Uses
/// one of the sixteen sentences this round's own `§3` table names as
/// a true number-immediately-followed-by-a-bare-plural-noun case
/// (`StalledAriaLabel`'s "Oldest stalled assigned issue: 1 days
/// (steady).", `ccbf67c`'s own pre-fix text), without touching
/// `en.rs` itself.
#[test]
fn guard_rejects_a_planted_disagreement() {
    let found =
        find_singular_plural_disagreement("Oldest stalled assigned issue: 1 days (steady).");
    assert_eq!(
        found,
        Some("1 days".to_string()),
        "expected the guard to flag \"1 days\" in the GRAM-002 round 2 reproduction string, \
         but it found {found:?}"
    );
}

/// Round 3's own proof for the widened window: `"1 consecutive
/// snapshots"` (`OverloadStreakAriaLabel`'s `ccbf67c` text, the case
/// round 2's narrower rule let through) is caught now that an
/// adjective between the number and its noun no longer defeats the
/// scan.
#[test]
fn guard_rejects_a_planted_disagreement_separated_by_an_adjective() {
    let found = find_singular_plural_disagreement(
        "Overload streak: 1 consecutive snapshots over capacity (steady).",
    );
    assert_eq!(
        found,
        Some("1 consecutive snapshots".to_string()),
        "expected the widened guard to flag \"1 consecutive snapshots\", but it found {found:?}"
    );
}

/// The negative-space check for the tests above, and for the skip
/// set's and stop set's own claim to be grammar rather than an
/// allowlist: every shape this round's own `§3` table names as
/// correctly left alone must stay clean, plus the five grammar words
/// the skip set exists for, plus the three stop-set shapes
/// (`of`/`or`/`/`) and a digit-bearing token, each exercised at the
/// window width that now matters, plus the three real renderings the
/// three-word window measurement found and rejected (`NFR-LANG-005`'s
/// own discovery becoming part of the check, not left as a claim about
/// a window nobody tried).
#[test]
fn guard_does_not_reject_grammar_words_or_correctly_uninflected_shapes() {
    for text in [
        "1 is enough",
        "1 as many",
        "1 vs 14",
        "1 was over capacity",
        "1 has moved",
        "1.0 issues per active day",
        "1 pt",
        "1 d",
        "1 of last 14",
        "1 / 12 of in-flight issues",
        "1 or more issues",
        "1%",
        "your effective limit is 1. Pushing one to Done",
        "Your in-flight load has been over capacity for 1 consecutive snapshot. This is a \
         description of the recent rhythm, not an evaluation of your work — many streaks \
         have legitimate causes. You can review at /today.",
        "Bar chart of 1 sprint, completed points from 5 to 5",
        "You have 1 issue in progress; your effective limit is 1. Pushing one to Done before \
         starting more keeps focus crisp.",
    ] {
        assert!(
            find_singular_plural_disagreement(text).is_none(),
            "{text:?} is a shape this round's own §3 names as correct and must not be flagged"
        );
    }
}
