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
//! `s` without being the plural of anything; `guard_does_not_reject_`
//! `grammar_words_ending_in_s` below is the negative-space check that
//! keeps it that way, the same role `guard_does_not_reject_legitimate_`
//! `shipped_symbols` plays in `language_guard.rs`.
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

/// The guard itself, as a value rather than a bool: `\b1\s+([A-Za-z-]+s)\b`
/// over `text`, by hand. Returns the offending two-word span so a
/// failure names what it found, not just that it found something.
fn find_singular_plural_disagreement(text: &str) -> Option<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    for i in 0..words.len().saturating_sub(1) {
        if is_bare_one(words[i]) && is_plural_shaped(words[i + 1]) {
            return Some(format!("{} {}", words[i], words[i + 1]));
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

/// The negative-space check for the test above, and for the skip
/// set's own claim to be grammar rather than an allowlist: every
/// shape this round's own `§3` table names as correctly left alone
/// must stay clean, plus the five grammar words the skip set exists
/// for, each immediately after a bare `1`.
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
    ] {
        assert!(
            find_singular_plural_disagreement(text).is_none(),
            "{text:?} is a shape this round's own §3 names as correct and must not be flagged"
        );
    }
}
