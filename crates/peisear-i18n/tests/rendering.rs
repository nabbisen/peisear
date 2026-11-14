//! Rendering-mechanism tests (`I18N-001` §5, tests 4–5): locale
//! switching is wholesale, and no raw key ever leaks into rendered
//! text.

mod common;

use peisear_i18n::{Locale, MessageKey, TrendDirectionLabel};

/// Test 4: the fixture locale renders every key differently from
/// English — not just some keys, which would suggest the mechanism
/// quietly falls through to English somewhere in the middle (the
/// exact failure mode `I18N-001` §4.5 exists to catch).
#[test]
fn fixture_locale_renders_every_key_differently_from_english() {
    for key in MessageKey::all() {
        let english = Locale::English.render(key.clone());
        let fixture = common::fixture_locale::render(key.clone());
        assert_ne!(
            english, fixture,
            "{key:?} rendered identically in English and the fixture locale — \
             the mechanism may be silently falling through to English"
        );
        // Distinct is necessary but not sufficient — every fixture
        // rendering also carries its own `[fx` marker, so a switch
        // that produced *some other* English-shaped string (rather
        // than genuinely routing through the fixture table) would
        // still be caught.
        assert!(
            fixture.contains("[fx"),
            "{key:?}'s fixture rendering {fixture:?} doesn't look like it came from the fixture table"
        );
    }
}

/// Test 5: no rendered output contains a raw, key-shaped literal
/// (RFC 006's test plan: no `[a-z_]+\.[a-z_]+`-shaped text reaches
/// visible output). This crate has no real surfaces to check yet, so
/// this checks its own rendering path — the property a converted
/// surface will need to keep holding once handoff 4 wires this crate
/// into `peisear-web`.
#[test]
fn no_rendered_output_contains_a_key_shaped_literal() {
    fn looks_key_shaped(word: &str) -> bool {
        // `foo.bar_baz` shape: lowercase segments joined by a dot,
        // each segment lowercase-with-underscores. Matches how a
        // `Debug`-formatted key or a stray format-string typo would
        // look if it leaked through unrendered.
        let Some((left, right)) = word.split_once('.') else {
            return false;
        };
        let is_key_segment =
            |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase() || c == '_');
        is_key_segment(left) && is_key_segment(right)
    }

    for key in MessageKey::all() {
        for rendered in [
            Locale::English.render(key.clone()),
            common::fixture_locale::render(key.clone()),
        ] {
            for word in rendered.split_whitespace() {
                let trimmed = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '.');
                assert!(
                    !looks_key_shaped(trimmed),
                    "{key:?} rendered {rendered:?}, which contains a key-shaped \
                     literal ({trimmed:?}) — a raw key leaked into visible text"
                );
            }
        }
    }
}

/// `I18N-005d` §7: `/today`'s privacy claim (`FR-PER-001`/`NFR-PRIV-001`)
/// must survive conversion — a conversion that dropped it would be invisible
/// to every other test, since nothing else on this screen depends on the
/// exact wording.
#[test]
fn personal_dashboard_privacy_subtitle_renders_byte_identically() {
    assert_eq!(
        Locale::English.render(MessageKey::PersonalDashboardSubtitle {
            display_name: "Alex".to_string(),
        }),
        "Personal metrics for Alex. Visible only to you."
    );
}

/// `I18N-005c` §5: `FR-TEAM-005`'s privacy footnote is a requirement-quoted
/// string ("management role, not an oversight role" is the acceptance
/// criterion) — "semantically identical" isn't a tight enough bar for it, so
/// this pins the exact bytes `TeamPrivacyFootnote` must render as.
///
/// This is the current source's actual wording, converted byte-exactly per
/// the handoff. It differs from the handoff §2's quoted "normative" text
/// (which reads "visible to all members... burnout panel, /today... Admin is
/// a management role") — that divergence is a pre-existing discrepancy
/// between the shipped copy and the requirement doc, escalated in this
/// handoff's review request rather than resolved by editing either one.
#[test]
fn team_privacy_footnote_renders_byte_identically() {
    assert_eq!(
        Locale::English.render(MessageKey::TeamPrivacyFootnote),
        "Privacy note: project trends and workload distribution are visible \
         to all team members. Personal sustainability data (your burnout panel, \
         your dashboard) remains visible to you only — admin role is a \
         management role, not an oversight role."
    );
}

/// `CAL-001` §2.1: RFC 002 must-have 9 and external design §10.3 carry two
/// different "do not paraphrase" versions of the project-axis calendar
/// footer. External design's is normative (handoff §2.1 rules it so — it is
/// the complete one; its third sentence is the only place the per-member
/// privacy guarantee is actually stated to the user). This pins the exact
/// bytes so "normative" means something the way `team_privacy_footnote_
/// renders_byte_identically` above already does for `FR-TEAM-005`.
#[test]
fn project_calendar_privacy_footnote_renders_byte_identically() {
    assert_eq!(
        Locale::English.render(MessageKey::ProjectCalendarPrivacyFootnote),
        "Calendar note: this view shows planned issue work for this project. \
         Personal schedules are not aggregated here. Each member's individual \
         calendar is private to that person."
    );
}

/// `GRAM-001`: at `count == 1`, noun and verb must agree --
/// "1 active assignee is", not "1 active assignees are". Asserted
/// exactly, not by a substring match on the shared tail -- three
/// existing tests already quote that tail and none of them could see
/// the defect, which is why this pins the singular's own full
/// sentence, byte-exact, same as the plural beside it.
#[test]
fn wip_compliance_explanation_singular_and_plural_agree() {
    assert_eq!(
        Locale::English.render(MessageKey::IndicatorExplanationWipCompliance { count: 1 }),
        "1 active assignee is over their WIP limit."
    );
    assert_eq!(
        Locale::English.render(MessageKey::IndicatorExplanationWipCompliance { count: 2 }),
        "2 active assignees are over their WIP limit."
    );
    // `count == 0` is reachable (a Good indicator can still carry a
    // zero count through this key) and "0 active assignees are" is
    // correct English, so it takes the plural branch -- checked
    // rather than assumed.
    assert_eq!(
        Locale::English.render(MessageKey::IndicatorExplanationWipCompliance { count: 0 }),
        "0 active assignees are over their WIP limit."
    );
}

/// `GRAM-002` (`061`, `§10.59`): every counted sentence this round
/// routed through `en.rs::counted`, asserted byte-exact at 1 and 2 --
/// not by a substring on the shared tail, the same reason `GRAM-001`'s
/// own test above gives. `count == 0` checked per string rather than
/// assumed, since not every one of these is reachable at zero in
/// practice and the point is the text, not the reachability.
#[test]
fn gram_002_counted_sentences_agree_with_their_own_number() {
    assert_eq!(
        Locale::English.render(MessageKey::IssuesCountText { count: 1 }),
        "1 issue"
    );
    assert_eq!(
        Locale::English.render(MessageKey::IssuesCountText { count: 2 }),
        "2 issues"
    );
    assert_eq!(
        Locale::English.render(MessageKey::IssuesCountText { count: 0 }),
        "0 issues"
    );

    assert_eq!(
        Locale::English.render(MessageKey::SprintCardSummaryPlanned {
            committed_points: 3,
            committed_count: 1,
        }),
        "3 pt committed across 1 issue"
    );
    assert_eq!(
        Locale::English.render(MessageKey::SprintCardSummaryPlanned {
            committed_points: 3,
            committed_count: 2,
        }),
        "3 pt committed across 2 issues"
    );
    assert_eq!(
        Locale::English.render(MessageKey::SprintCardSummaryPlanned {
            committed_points: 3,
            committed_count: 0,
        }),
        "3 pt committed across 0 issues"
    );

    assert_eq!(
        Locale::English.render(MessageKey::VelocitySummaryPointsList {
            points_list: "8".to_string(),
            sprint_count: 1,
        }),
        "Completed points across the last 1 sprint: 8."
    );
    assert_eq!(
        Locale::English.render(MessageKey::VelocitySummaryPointsList {
            points_list: "8, 13".to_string(),
            sprint_count: 2,
        }),
        "Completed points across the last 2 sprints: 8, 13."
    );

    assert_eq!(
        Locale::English.render(MessageKey::BarChartAriaLabel {
            sprint_count: 1,
            min_completed: 5,
            max_completed: 5,
        }),
        "Bar chart of 1 sprint, completed points from 5 to 5"
    );
    assert_eq!(
        Locale::English.render(MessageKey::BarChartAriaLabel {
            sprint_count: 2,
            min_completed: 5,
            max_completed: 13,
        }),
        "Bar chart of 2 sprints, completed points from 5 to 13"
    );

    assert_eq!(
        Locale::English.render(MessageKey::TrendAriaLabel {
            direction: TrendDirectionLabel::Up,
            delta: 1,
        }),
        "trend: up by 1 point"
    );
    assert_eq!(
        Locale::English.render(MessageKey::TrendAriaLabel {
            direction: TrendDirectionLabel::Down,
            delta: 2,
        }),
        "trend: down by 2 points"
    );
    assert_eq!(
        Locale::English.render(MessageKey::TrendAriaLabel {
            direction: TrendDirectionLabel::Up,
            delta: 0,
        }),
        "trend: up by 0 points"
    );

    assert_eq!(
        Locale::English.render(MessageKey::CapacityRowAriaLabel {
            points: 1,
            from: "2026-08-01".to_string(),
            to: "2026-08-31".to_string(),
        }),
        "Capacity 1 point, period 2026-08-01 to 2026-08-31."
    );
    assert_eq!(
        Locale::English.render(MessageKey::CapacityRowAriaLabel {
            points: 2,
            from: "2026-08-01".to_string(),
            to: "2026-08-31".to_string(),
        }),
        "Capacity 2 points, period 2026-08-01 to 2026-08-31."
    );
    assert_eq!(
        Locale::English.render(MessageKey::CapacityRowAriaLabel {
            points: 0,
            from: "2026-08-01".to_string(),
            to: "2026-08-31".to_string(),
        }),
        "Capacity 0 points, period 2026-08-01 to 2026-08-31."
    );

    assert_eq!(
        Locale::English.render(MessageKey::WorkloadTitle {
            display_name: "Alex Rivera".to_string(),
            in_flight_issues: 1,
        }),
        "Alex Rivera — 1 in-flight issue"
    );
    assert_eq!(
        Locale::English.render(MessageKey::WorkloadTitle {
            display_name: "Alex Rivera".to_string(),
            in_flight_issues: 2,
        }),
        "Alex Rivera — 2 in-flight issues"
    );
    assert_eq!(
        Locale::English.render(MessageKey::WorkloadTitle {
            display_name: "Alex Rivera".to_string(),
            in_flight_issues: 0,
        }),
        "Alex Rivera — 0 in-flight issues"
    );

    assert_eq!(
        Locale::English.render(MessageKey::CalendarCellAriaLabel {
            month: 5,
            day: 3,
            count: 1,
        }),
        "May 3, 1 issue scheduled"
    );
    assert_eq!(
        Locale::English.render(MessageKey::CalendarCellAriaLabel {
            month: 5,
            day: 3,
            count: 2,
        }),
        "May 3, 2 issues scheduled"
    );
    assert_eq!(
        Locale::English.render(MessageKey::CalendarCellAriaLabel {
            month: 5,
            day: 3,
            count: 0,
        }),
        "May 3, 0 issues scheduled"
    );
}

/// `GRAM-002`: already correct (`I18N-004`), now routed through
/// `counted()` for the count+noun segment -- byte-exact at 1 and 2 to
/// confirm the refactor changed nothing a reader sees.
#[test]
fn confirm_delete_issue_cascade_note_unchanged_by_the_refactor() {
    assert_eq!(
        Locale::English.render(MessageKey::ConfirmDeleteIssueCascadeNote { sub_issue_count: 1 }),
        "This issue has 1 sub-issue. Deleting it deletes that sub-issue too. \
         This cannot be undone."
    );
    assert_eq!(
        Locale::English.render(MessageKey::ConfirmDeleteIssueCascadeNote { sub_issue_count: 2 }),
        "This issue has 2 sub-issues. Deleting it deletes all of them too. \
         This cannot be undone."
    );
}

/// `GRAM-002`: the burnout panel's own summary and its two notification
/// bodies, found independently of the handoff's own named five while
/// re-deriving the population from the enum -- short, direct sentences
/// read whole, not fragments of a longer threshold explanation, the
/// same shape as this round's other fixes.
#[test]
fn gram_002_burnout_sentences_agree_with_their_own_number() {
    assert_eq!(
        Locale::English.render(MessageKey::BurnoutSummaryOverloadOnly { days: 1 }),
        "you've been over capacity for 1 recent snapshot — consider whether \
         some work can wait or move"
    );
    assert_eq!(
        Locale::English.render(MessageKey::BurnoutSummaryOverloadOnly { days: 2 }),
        "you've been over capacity for 2 recent snapshots — consider whether \
         some work can wait or move"
    );

    assert_eq!(
        Locale::English.render(MessageKey::BurnoutSummaryStalledOnly { days: 1 }),
        "an assigned issue has been stuck for 1 day — worth a quick check \
         whether it's blocked"
    );
    assert_eq!(
        Locale::English.render(MessageKey::BurnoutSummaryStalledOnly { days: 2 }),
        "an assigned issue has been stuck for 2 days — worth a quick check \
         whether it's blocked"
    );

    assert_eq!(
        Locale::English.render(MessageKey::BurnoutSummaryBoth {
            overload_days: 1,
            stalled_days: 1,
        }),
        "you've been over capacity for 1 recent snapshot — consider whether \
         some work can wait or move; an assigned issue has been stuck for \
         1 day — worth a quick check whether it's blocked"
    );
    assert_eq!(
        Locale::English.render(MessageKey::BurnoutSummaryBoth {
            overload_days: 2,
            stalled_days: 2,
        }),
        "you've been over capacity for 2 recent snapshots — consider whether \
         some work can wait or move; an assigned issue has been stuck for \
         2 days — worth a quick check whether it's blocked"
    );

    assert_eq!(
        Locale::English.render(MessageKey::NotificationBurnoutOverloadBody {
            streak_snapshots: 1
        }),
        "Your in-flight load has been over capacity for 1 consecutive \
         snapshot. This is a description of the recent rhythm, not an \
         evaluation of your work — many streaks have legitimate causes. \
         You can review at /today."
    );
    assert_eq!(
        Locale::English.render(MessageKey::NotificationBurnoutOverloadBody {
            streak_snapshots: 2
        }),
        "Your in-flight load has been over capacity for 2 consecutive \
         snapshots. This is a description of the recent rhythm, not an \
         evaluation of your work — many streaks have legitimate causes. \
         You can review at /today."
    );

    assert_eq!(
        Locale::English.render(MessageKey::NotificationBurnoutStalledBody { stalled_days: 1 }),
        "An assigned issue has been in flight for 1 day. May be worth a \
         glance — sometimes a quick check-in turns out to be all that's \
         needed. Visit /today for context."
    );
    assert_eq!(
        Locale::English.render(MessageKey::NotificationBurnoutStalledBody { stalled_days: 2 }),
        "An assigned issue has been in flight for 2 days. May be worth a \
         glance — sometimes a quick check-in turns out to be all that's \
         needed. Visit /today for context."
    );
}
