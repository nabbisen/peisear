//! The vocabulary guard, blocking CI job (`I18N-001` §4.4/§4.6, RFC
//! 006 open question 3's default: blocking from 0.21.0).
//!
//! Two things this file proves, both required by the handoff:
//!
//! 1. The guard finds **nothing** over every real, shipped entry —
//!    the English table and the fixture locale, every key, every
//!    closed-set parameter combination ([`MessageKey::all`]).
//! 2. The guard **does** find something when given something to find
//!    — a planted violation in a test-only table. "A guard never
//!    observed failing is not known to work" (handoff §5, test 3),
//!    and this is the test most likely to be skipped, so it gets
//!    both a positive and negative case rather than one assertion.

mod common;

use peisear_i18n::{Locale, MessageKey, find_violations};

/// Test 2: every entry of every locale table, shipped and fixture,
/// is clean.
#[test]
fn shipped_and_fixture_tables_contain_no_prohibited_vocabulary() {
    let keys = MessageKey::all();
    assert!(
        !keys.is_empty(),
        "MessageKey::all() must enumerate at least the seeded set, or this test proves nothing"
    );

    for key in &keys {
        let english = Locale::English.render(*key);
        let violations = find_violations(&english);
        assert!(
            violations.is_empty(),
            "English rendering of {key:?} contains prohibited vocabulary: {:?} (text: {english:?})",
            violations.iter().map(|v| v.phrase).collect::<Vec<_>>()
        );

        let fixture = common::fixture_locale::render(*key);
        let violations = find_violations(&fixture);
        assert!(
            violations.is_empty(),
            "fixture rendering of {key:?} contains prohibited vocabulary: {:?} (text: {fixture:?})",
            violations.iter().map(|v| v.phrase).collect::<Vec<_>>()
        );
    }
}

/// Test 3: the guard actually rejects vocabulary it should reject.
/// Deliberately prohibited entries, checked one at a time so a
/// failure here names exactly which term the guard failed to catch.
#[test]
fn guard_rejects_a_planted_violation() {
    let planted = [
        (
            "Great work — you're making good progress this sprint.",
            "good progress",
        ),
        ("Your velocity dropped compared to last week.", "velocity"),
        (
            "Failed to save your changes. Please try again.",
            "failed to",
        ),
        ("This user is a top performer on the team.", "top performer"),
        ("You should close this issue before Friday.", "you should"),
    ];

    for (text, expected_term) in planted {
        let violations = find_violations(text);
        assert!(
            !violations.is_empty(),
            "expected the guard to reject {text:?}, but it found nothing — \
             a guard never observed failing is not known to work"
        );
        assert!(
            violations.iter().any(|v| v.phrase == expected_term),
            "expected {text:?} to be flagged for {expected_term:?}, got {:?}",
            violations.iter().map(|v| v.phrase).collect::<Vec<_>>()
        );
    }
}

/// The negative-space check for test 3: a real, clean rendering
/// button-pressed right next to the planted violations above must
/// stay clean, so the planted-violation test is proving the guard
/// fires on the bad text specifically, not on every input.
#[test]
fn guard_does_not_reject_the_real_seeded_table() {
    for key in MessageKey::all() {
        assert!(find_violations(&Locale::English.render(key)).is_empty());
    }
}

/// Handoff §4.4: case-insensitive.
#[test]
fn matching_is_case_insensitive() {
    for text in ["VELOCITY", "Velocity", "vElOcItY"] {
        assert!(
            !find_violations(text).is_empty(),
            "{text:?} should be caught regardless of case"
        );
    }
}

/// Handoff §4.4: word-boundary aware — must not match a substring
/// inside an unrelated word.
#[test]
fn matching_does_not_fire_inside_an_unrelated_word() {
    // "rank" is a substring of "franking" and "ranking" is a
    // substring of "rankingsystem" (no boundary) — neither should
    // trip the "ranking" entry, since the false-positive risk is
    // exactly what word-boundary matching exists to prevent.
    assert!(find_violations("the franking machine is broken").is_empty());
    assert!(find_violations("our rankingsystem needs work").is_empty());
    // But "ranking" as its own word must still fire.
    assert!(!find_violations("check the ranking page").is_empty());
}

/// §1.7's explicit example: interpolated user data is not a
/// violation. This crate's guard only ever sees closed-set copy in
/// this handoff (no free-text parameters are seeded yet), but the
/// function itself must not special-case anything about *why* a
/// word appears — it only sees the string it's given. Documenting
/// the scope limit here since a future caller (handoff 4) will feed
/// it rendered output that does include user data outside the
/// parameter position, and needs to know this function doesn't
/// distinguish "copy" from "data" — that responsibility stays with
/// the caller (RFC 006 D4's stated scope limit).
#[test]
fn the_guard_itself_does_not_distinguish_copy_from_data() {
    // "velocity" inside what would be user-authored text is still
    // flagged — proving the guard has no user-data exemption baked
    // in. Callers converting real surfaces must not feed it raw user
    // strings expecting them to be skipped.
    assert!(!find_violations("issue titled: velocity spike").is_empty());
}
