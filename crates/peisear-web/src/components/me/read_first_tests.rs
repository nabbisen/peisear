/// `PER-001` (`FR-PER-006`, `§10.35`). `compute_read_first` is a pure
/// four-condition precedence chain (`DOC` comment above it) and `me.rs`
/// had no `#[cfg(test)]` module at all before this -- the acceptance
/// cited one test, `today_renders_no_callout_for_fresh_user`, which
/// exercises only the vacuous "nothing applies" case.
///
/// Unit-testing the private function directly, rather than widening it
/// to `pub(crate)` or driving `/today` over HTTP for every case: the
/// chain is combinatorial (four conditions, three tie-breaks, two
/// boundaries) and a pure function is where combinations are cheap.
/// `TT-007`'s precedent is reusing what exists rather than exposing
/// more, and a `#[cfg(test)]` module needs no visibility change at
/// all. One HTTP test, in `tests/today_panel.rs`, covers the half a
/// pure-function test cannot: that the chosen callout actually reaches
/// the rendered page, and that it is the only one there.
///
/// Thresholds are read from `peisear_core::user_burnout`'s own
/// constants rather than hardcoded, so a future change to either
/// constant updates these tests' expectations along with the
/// behaviour they gate.
///
/// **Moved to its own file** (`DOC-007`, `NFR-MNT-005`), from an inline
/// `#[cfg(test)]` module in `me.rs`: a file-based child module's
/// privacy relationship to its parent is the same as an inline one's,
/// so `compute_read_first` staying private needed no change here either.
use super::compute_read_first;
use peisear_core::user_burnout::{OVERLOAD_STREAK_WATCH, STALLED_WATCH_DAYS, UserBurnoutSignals};

fn signals(overload_streak_days: i64, stalled_assigned_max_days: i64) -> UserBurnoutSignals {
    UserBurnoutSignals {
        overload_streak_days,
        stalled_assigned_max_days,
        window_days: 14,
        estimation_drift: None,
        cognitive_switching: None,
    }
}

// -- §2 item 1: each of the four in isolation --

#[test]
fn overload_streak_alone_triggers_the_overload_callout() {
    let b = signals(OVERLOAD_STREAK_WATCH, 0);
    let result = compute_read_first(0, 3, 0, Some(&b));
    assert_eq!(
        result.map(|r| r.title),
        Some(super::t(super::MessageKey::ReadFirstOverloadTitle))
    );
}

#[test]
fn stalled_assigned_alone_triggers_the_stalled_callout() {
    let b = signals(0, STALLED_WATCH_DAYS);
    let result = compute_read_first(0, 3, 0, Some(&b));
    assert_eq!(
        result.map(|r| r.title),
        Some(super::t(super::MessageKey::ReadFirstStalledTitle))
    );
}

#[test]
fn wip_over_limit_alone_triggers_the_wip_callout() {
    let result = compute_read_first(4, 3, 0, None);
    assert_eq!(
        result.map(|r| r.title),
        Some(super::t(super::MessageKey::ReadFirstWipTitle))
    );
}

#[test]
fn long_stale_count_of_one_alone_triggers_the_long_stale_callout() {
    let result = compute_read_first(0, 3, 1, None);
    assert_eq!(
        result.map(|r| r.title),
        Some(super::t(super::MessageKey::ReadFirstLongStaleTitle))
    );
}

// -- §0 / §2 item 4: the two boundaries the comments call deliberate --

#[test]
fn wip_exactly_at_limit_does_not_trigger_the_wip_callout() {
    // "being exactly at the limit is the limit, not over it."
    let result = compute_read_first(3, 3, 0, None);
    assert!(
        result.is_none(),
        "WIP == limit must not trigger the WIP callout (strict `>`), got {:?}",
        result.map(|r| r.title)
    );
}

#[test]
fn long_stale_count_of_zero_does_not_trigger_anything() {
    // "even one stale issue is worth surfacing" -- the inverse:
    // zero must not.
    let result = compute_read_first(0, 3, 0, None);
    assert!(
        result.is_none(),
        "long_stale_count == 0 must not trigger a callout, got {:?}",
        result.map(|r| r.title)
    );
}

// -- §2 item 2: the tie-breaks, which are the requirement's actual
// content. A chain is only a chain where two conditions compete. --

#[test]
fn overload_beats_stalled_when_both_apply() {
    let b = signals(OVERLOAD_STREAK_WATCH, STALLED_WATCH_DAYS);
    let result = compute_read_first(0, 3, 0, Some(&b));
    assert_eq!(
        result.map(|r| r.title),
        Some(super::t(super::MessageKey::ReadFirstOverloadTitle)),
        "overload and stalled both qualify; overload must win"
    );
}

#[test]
fn stalled_beats_wip_when_both_apply() {
    let b = signals(0, STALLED_WATCH_DAYS);
    // current_wip (4) > effective_wip_limit (3) also qualifies.
    let result = compute_read_first(4, 3, 0, Some(&b));
    assert_eq!(
        result.map(|r| r.title),
        Some(super::t(super::MessageKey::ReadFirstStalledTitle)),
        "stalled and WIP-over-limit both qualify; stalled must win"
    );
}

#[test]
fn wip_beats_long_stale_when_both_apply() {
    let result = compute_read_first(4, 3, 1, None);
    assert_eq!(
        result.map(|r| r.title),
        Some(super::t(super::MessageKey::ReadFirstWipTitle)),
        "WIP-over-limit and long-stale both qualify; WIP must win"
    );
}

// -- §2 item 5: "none" is already covered by
// `today_renders_no_callout_for_fresh_user` (tests/today_panel.rs)
// -- left alone, per the handoff.
