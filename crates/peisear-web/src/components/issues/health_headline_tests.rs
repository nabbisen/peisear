//! `VIEW-001`: tests for [`super::compute_health_headline`] and the
//! precedence order it walks.
//!
//! The chain is only a chain where two conditions compete (the same
//! standard `FR-PER-006`'s own tests hold `/today`'s chain to): each
//! adjacent pair in [`super::HEALTH_HEADLINE_PRECEDENCE`] gets a test
//! that sets *both* members to `Watch` and confirms the
//! higher-precedence one wins, which only passes if the order is
//! actually consulted rather than some other property (declaration
//! order, iteration order) happening to agree with it today.

use super::*;
use peisear_core::HealthIndicator;

/// All six kinds, each `Good` by default. `watch` names the ones to
/// set to `Watch` instead. `raw` is shared fixture data sized so
/// every kind's `human_explanation` can construct its sentence
/// without panicking regardless of which kinds are `Watch` --
/// `BusFactor`'s non-solo branch in particular needs `in_flight_issues`
/// non-zero, and `Staleness` needs `oldest_in_flight_age_days` to be
/// `Some`.
fn indicators_with_watch(watch: &[IndicatorKind]) -> Vec<Indicator> {
    const ALL_KINDS: [IndicatorKind; 6] = [
        IndicatorKind::Throughput,
        IndicatorKind::Staleness,
        IndicatorKind::Activity,
        IndicatorKind::BusFactor,
        IndicatorKind::LongStale,
        IndicatorKind::WipCompliance,
    ];
    ALL_KINDS
        .iter()
        .map(|&kind| Indicator {
            kind,
            value_display: MessageKey::IndicatorValueWipAllWithin,
            state: if watch.contains(&kind) {
                HealthIndicator::Watch
            } else {
                HealthIndicator::Good
            },
            normalized: 1.0,
            weight: 0.15,
        })
        .collect()
}

fn fixture_raw() -> ProjectHealthRaw {
    ProjectHealthRaw {
        total_issues: 10,
        done_issues: 3,
        oldest_in_flight_age_days: Some(20),
        recent_activity_count: 2,
        in_flight_issues: 10,
        top_assignee_in_flight_issues: 5,
        long_stale_in_flight_issues: 4,
        wip_violators: 2,
        active_assignees: 3,
        ..Default::default()
    }
}

/// The whitelist this slot may ever return: the seven per-indicator
/// sentences `FR-HLT-005` requires, or the one neutral line. Pinned
/// here the way `FR-HLT-006`'s own vocabulary guard is pinned against
/// `MessageKey::all()` -- a future edit that routes a composite
/// score, a raw number, or any other new key into this slot fails
/// loudly by name instead of silently rendering whatever `Display`
/// that key happens to produce.
fn assert_is_a_sentence_not_a_score(key: &MessageKey) {
    let allowed = matches!(
        key,
        MessageKey::IndicatorExplanationThroughput { .. }
            | MessageKey::IndicatorExplanationStaleness { .. }
            | MessageKey::IndicatorExplanationActivity { .. }
            | MessageKey::IndicatorExplanationBusFactorSolo
            | MessageKey::IndicatorExplanationBusFactor { .. }
            | MessageKey::IndicatorExplanationLongStale { .. }
            | MessageKey::IndicatorExplanationWipCompliance { .. }
            | MessageKey::HealthSummaryHealthy
    );
    assert!(
        allowed,
        "compute_health_headline returned {key:?}, which is none of the \
         seven per-indicator sentences or the neutral line -- a composite \
         score must never reach this slot (FR-HLT-008)"
    );
}

#[test]
fn healthy_project_renders_the_neutral_line() {
    let headline = compute_health_headline(&indicators_with_watch(&[]), &fixture_raw());
    assert_eq!(headline, MessageKey::HealthSummaryHealthy);
    assert_is_a_sentence_not_a_score(&headline);
}

#[test]
fn a_single_watch_indicator_produces_its_own_explanation() {
    let headline = compute_health_headline(
        &indicators_with_watch(&[IndicatorKind::LongStale]),
        &fixture_raw(),
    );
    assert!(matches!(
        headline,
        MessageKey::IndicatorExplanationLongStale { .. }
    ));
    assert_is_a_sentence_not_a_score(&headline);
}

#[test]
fn wip_compliance_outranks_staleness() {
    let raw = fixture_raw();
    let watch = [IndicatorKind::WipCompliance, IndicatorKind::Staleness];
    let headline = compute_health_headline(&indicators_with_watch(&watch), &raw);
    assert!(matches!(
        headline,
        MessageKey::IndicatorExplanationWipCompliance { .. }
    ));
    // Inverted: with WipCompliance alone absent, Staleness now wins.
    let headline_inverted =
        compute_health_headline(&indicators_with_watch(&[IndicatorKind::Staleness]), &raw);
    assert!(matches!(
        headline_inverted,
        MessageKey::IndicatorExplanationStaleness { .. }
    ));
}

#[test]
fn staleness_outranks_long_stale() {
    let raw = fixture_raw();
    let watch = [IndicatorKind::Staleness, IndicatorKind::LongStale];
    let headline = compute_health_headline(&indicators_with_watch(&watch), &raw);
    assert!(matches!(
        headline,
        MessageKey::IndicatorExplanationStaleness { .. }
    ));
    let headline_inverted =
        compute_health_headline(&indicators_with_watch(&[IndicatorKind::LongStale]), &raw);
    assert!(matches!(
        headline_inverted,
        MessageKey::IndicatorExplanationLongStale { .. }
    ));
}

#[test]
fn long_stale_outranks_bus_factor() {
    let raw = fixture_raw();
    let watch = [IndicatorKind::LongStale, IndicatorKind::BusFactor];
    let headline = compute_health_headline(&indicators_with_watch(&watch), &raw);
    assert!(matches!(
        headline,
        MessageKey::IndicatorExplanationLongStale { .. }
    ));
    let headline_inverted =
        compute_health_headline(&indicators_with_watch(&[IndicatorKind::BusFactor]), &raw);
    assert!(matches!(
        headline_inverted,
        MessageKey::IndicatorExplanationBusFactor { .. }
    ));
}

#[test]
fn bus_factor_outranks_throughput() {
    let raw = fixture_raw();
    let watch = [IndicatorKind::BusFactor, IndicatorKind::Throughput];
    let headline = compute_health_headline(&indicators_with_watch(&watch), &raw);
    assert!(matches!(
        headline,
        MessageKey::IndicatorExplanationBusFactor { .. }
    ));
    let headline_inverted =
        compute_health_headline(&indicators_with_watch(&[IndicatorKind::Throughput]), &raw);
    assert!(matches!(
        headline_inverted,
        MessageKey::IndicatorExplanationThroughput { .. }
    ));
}

#[test]
fn throughput_outranks_activity() {
    let raw = fixture_raw();
    let watch = [IndicatorKind::Throughput, IndicatorKind::Activity];
    let headline = compute_health_headline(&indicators_with_watch(&watch), &raw);
    assert!(matches!(
        headline,
        MessageKey::IndicatorExplanationThroughput { .. }
    ));
    let headline_inverted =
        compute_health_headline(&indicators_with_watch(&[IndicatorKind::Activity]), &raw);
    assert!(matches!(
        headline_inverted,
        MessageKey::IndicatorExplanationActivity { .. }
    ));
}

#[test]
fn every_precedence_slot_is_pinned_a_sentence_not_a_score() {
    // Every single-Watch case, exercised through the same whitelist
    // the healthy case above uses -- a sweep rather than six repeats
    // of the same assertion shape.
    for kind in HEALTH_HEADLINE_PRECEDENCE {
        let headline = compute_health_headline(&indicators_with_watch(&[kind]), &fixture_raw());
        assert_is_a_sentence_not_a_score(&headline);
    }
}
