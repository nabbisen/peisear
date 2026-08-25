//! `QA-008` §2 (RFC 005 §3) — RFC 005 §3 originally asked for a
//! Japanese-to-English *conversion*; that work is done, and every
//! remaining non-Latin occurrence in the tree (a comment citing a
//! Japanese source document, or `search.rs`'s
//! `escape_like_meta("ログイン")` fixture) is meant to stay exactly as
//! it is. What was never built is the *reverse* guard: nothing stops
//! non-English copy from reaching [`en::render`](peisear_i18n::en)
//! itself. Planted, `cargo test --workspace` stays at 195 passed, 0
//! failed — `prose_scan` only tests `is_ascii_alphabetic` (invisible
//! to a non-Latin literal) and `find_violations` looks for specific
//! prohibited *English* phrases (absent from a string with no English
//! in it at all). Reproduced by planting
//! `MessageKey::NewSubIssueLabel => "新しいサブ課題".to_string()` before
//! writing this file — see `evidence/reproduce-section2-plant.log` in
//! the review request.
//!
//! **The assertion is "no non-Latin script", not "ASCII only".** The
//! shipped copy legitimately uses characters like `—`, `←`, `→`, `✓`
//! and `⚠` elsewhere in the tree, and curly quotes are fair game too
//! — none of those are alphabetic, so a script-based rule (below)
//! passes them while still catching Hiragana, Katakana, CJK, Cyrillic,
//! Greek, Arabic, Hebrew, or any other non-Latin alphabetic script.
//!
//! **Checks rendered output, not source text** (`en.rs` scanned as
//! source the way `prose_scan`/`static_js_scan` scan their targets was
//! the other option). Source-scanning would have to exclude comments
//! to avoid flagging the Japanese-source-document citations — and
//! those citations are exactly where the legitimate Japanese content
//! in this file lives, unlike `prose_scan`'s comment-stripping, which
//! has no such tension. Rendering instead checks what a user actually
//! sees, costs nothing extra ([`MessageKey::all`] and [`Locale`] are
//! already public), and automatically covers any key added after this
//! guard is written. It mirrors `find_violations`'s own established
//! shape in `guard.rs`.
//!
//! **Scope limit, matching `guard.rs`'s own
//! `the_guard_itself_does_not_distinguish_copy_from_data`**: some
//! keys interpolate genuine user data (`MoveIssueAriaLabel`'s
//! `issue_title`, `WorkloadTitle`'s `display_name`, ...) —
//! real display names and issue titles can legitimately contain
//! non-Latin script, and this guard does not and cannot distinguish
//! that from non-Latin copy in the static template around it. It only
//! checks [`MessageKey::all`]'s fixed representative values, which are
//! Latin-script by existing convention ("Alex Rivera", "Login error",
//! ...) — the same convention `guard.rs`'s vocabulary check already
//! depends on. That responsibility stays with whoever picks
//! `all()`'s representative values, not with this guard.
//!
//! Runs in `peisear-i18n`'s own CI job (`test-peisear-i18n`, a bare
//! `cargo test -p peisear-i18n`) rather than `peisear-web`'s `--lib`
//! job that holds `prose_scan`/`static_js_scan`/`test_harness_scan`/
//! `dec_007_scan` — this is a fact about `peisear-i18n`'s own source
//! (its message table and renderer), the same reasoning `QA-008` §2.3
//! gives for either placement being defensible.

use peisear_i18n::{Locale, MessageKey};

/// True if `c` is alphabetic and falls inside a Latin-script Unicode
/// range. Deliberately wider than ASCII (English prose may reasonably
/// carry an accented loanword — café, naïve) and deliberately
/// narrower than "every alphabetic character" — Hiragana, Katakana,
/// CJK Unified Ideographs, Cyrillic, Greek, Arabic and Hebrew letters
/// are all `is_alphabetic()` but none fall in these ranges.
fn is_latin_alphabetic(c: char) -> bool {
    matches!(c,
        'A'..='Z' | 'a'..='z'
        | '\u{00C0}'..='\u{00FF}' // Latin-1 Supplement letters (À-ÿ)
        | '\u{0100}'..='\u{017F}' // Latin Extended-A
        | '\u{0180}'..='\u{024F}' // Latin Extended-B
        | '\u{1E00}'..='\u{1EFF}' // Latin Extended Additional
    )
}

/// Every character in `text` that Unicode classifies as alphabetic
/// but that is not Latin script. Punctuation and symbols — `—`, `←`,
/// `→`, `✓`, `⚠`, curly quotes — are never `is_alphabetic()`, so none
/// of them can appear here regardless of this function's Latin-range
/// list.
fn non_latin_alphabetic_chars(text: &str) -> Vec<char> {
    text.chars()
        .filter(|c| c.is_alphabetic() && !is_latin_alphabetic(*c))
        .collect()
}

/// The guard itself: every key's English rendering, checked against
/// the real, current message table. `QA-008` §2.1 requires this rule
/// be run against the shipped table and the hit count reported before
/// it is trusted — this test *is* that report, in assertion form: a
/// failure here means a non-zero hit count, and the message names
/// exactly which key and which characters.
#[test]
fn english_rendering_contains_no_non_latin_script() {
    let keys = MessageKey::all();
    assert!(
        !keys.is_empty(),
        "MessageKey::all() must enumerate at least the seeded set, or this test proves nothing"
    );

    for key in &keys {
        let rendered = Locale::English.render(key.clone());
        let offenders = non_latin_alphabetic_chars(&rendered);
        assert!(
            offenders.is_empty(),
            "English rendering of {key:?} contains non-Latin-script character(s) {offenders:?} \
             (text: {rendered:?})"
        );
    }
}

/// "A guard never observed failing is not known to work" — the same
/// standard `guard.rs`'s `guard_rejects_a_planted_violation` holds
/// itself to. Uses the exact string `QA-008` §2 planted into `en.rs`
/// to reproduce the defect, without touching `en.rs` itself.
#[test]
fn guard_rejects_a_planted_non_latin_string() {
    let offenders = non_latin_alphabetic_chars("新しいサブ課題");
    assert!(
        !offenders.is_empty(),
        "expected non-Latin script to be flagged in the QA-008 §2 reproduction string, \
         but the guard found nothing"
    );
}

/// The negative-space check for the test above: every symbol the
/// shipped copy is known to use legitimately (`QA-008` §2.1) must
/// stay clean, so the planted-violation test is proving the guard
/// fires on non-Latin script specifically, not on any non-ASCII
/// character.
#[test]
fn guard_does_not_reject_legitimate_shipped_symbols() {
    for text in [
        "—", "←", "→", "✓", "⚠", "…", "§", "·", "≈", "\u{2018}", "\u{2019}", "\u{201C}",
        "\u{201D}", "café", "naïve",
    ] {
        assert!(
            non_latin_alphabetic_chars(text).is_empty(),
            "{text:?} is legitimate shipped copy and must not be flagged"
        );
    }
}
