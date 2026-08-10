//! The English locale table.
//!
//! `render` is an exhaustive `match` with no wildcard (`_`) arm —
//! that is the whole of the compile-time guarantee `RFC 006`
//! requirement 2 asks for. Add a [`MessageKey`] variant without
//! adding an arm here and this function fails to compile. See
//! `I18N-001`'s review request for a demonstration (temporarily
//! removing an arm and capturing the resulting compiler error).
//!
//! The two `#![deny]`s below are what keep that guarantee from being
//! able to quietly dissolve: without them, a future `_ => ...` arm
//! added to any match in this file would compile cleanly and
//! silently stop distinguishing a missing key from a handled one
//! (`I18N-001-review.md` §4) — the exhaustiveness guarantee would go
//! back to being a convention, exactly the failure mode this release
//! exists to replace. Two lints, not one: `wildcard_enum_match_arm`
//! alone does **not** fire when the wildcard covers exactly one
//! remaining variant — verified empirically while implementing this
//! correction, not assumed from the lint's name — which is precisely
//! the shape a real regression would most likely take (one arm
//! quietly swapped for `_`, not several at once).
//! `match_wildcard_for_single_variants` covers that gap. Scoped to
//! this module rather than crate-wide, since both are restriction
//! lints that would also fire on unrelated matches over enums this
//! crate doesn't own.
#![deny(clippy::wildcard_enum_match_arm)]
#![deny(clippy::match_wildcard_for_single_variants)]

use crate::message::{EntityKind, Field, IndicatorLabel, MessageKey, NavSection};

pub(crate) fn render(key: MessageKey) -> String {
    match key {
        MessageKey::Forbidden => "permission denied".to_string(),
        MessageKey::NotFound => "resource not found".to_string(),
        MessageKey::InternalError => "An internal error occurred. Please try again.".to_string(),
        MessageKey::OptimisticLockConflict { entity } => format!(
            "Someone else updated this {} while you were editing. \
             Please reload the page and re-apply your change so you \
             don't overwrite their work.",
            entity_label(entity)
        ),
        MessageKey::LockValueUnreadable => {
            "This page is showing an earlier version. Reload to see the current state.".to_string()
        }
        MessageKey::FieldRequired { field } => format!("{} is required.", field_label(field)),
        MessageKey::FieldMustBePositiveInteger { field } => {
            format!("{} must be a positive integer.", field_label(field))
        }
        MessageKey::InvalidStatus => "Invalid status".to_string(),
        MessageKey::InvalidPriority => "Invalid priority".to_string(),

        // ---- I18N-004: IndicatorKind ----
        MessageKey::IndicatorName { label } => indicator_label(label).to_string(),

        // ---- I18N-002: format_value ----
        MessageKey::IndicatorValueUnavailable => "—".to_string(),
        MessageKey::IndicatorValueThroughput { done, total } => throughput_value(done, total),
        MessageKey::IndicatorValueStaleness { days } => format!("{days} d"),
        MessageKey::IndicatorValueActivity { count } => format!("{count}"),
        MessageKey::IndicatorValueBusFactorSolo => "solo".to_string(),
        MessageKey::IndicatorValueBusFactor { pct } => format!("{pct}% on top"),
        MessageKey::IndicatorValueLongStale { stale, in_flight } => {
            format!("{stale} / {in_flight}")
        }
        MessageKey::IndicatorValueWipAllWithin => "all within".to_string(),
        MessageKey::IndicatorValueWipOver { count } => format!("{count} over"),

        // ---- I18N-002: human_explanation ----
        MessageKey::IndicatorExplanationThroughput { done, total } => format!(
            "Throughput is {} — fewer issues are reaching Done than the rest of the project's history.",
            throughput_value(done, total)
        ),
        MessageKey::IndicatorExplanationStaleness { days } => {
            format!("The oldest in-flight issue has been open for {days} d.")
        }
        MessageKey::IndicatorExplanationActivity { count } => {
            format!("Issue activity in the last two weeks is {count}.")
        }
        // I18N-004 fix for ISSUE-006 finding 2 -- see message.rs's
        // doc comment on this variant.
        MessageKey::IndicatorExplanationBusFactorSolo => {
            "In-flight work is currently carried by one person.".to_string()
        }
        MessageKey::IndicatorExplanationBusFactor { pct } => {
            format!("{pct}% on top of in-flight work is concentrated on one person.")
        }
        MessageKey::IndicatorExplanationLongStale { stale, in_flight } => format!(
            "{stale} / {in_flight} of in-flight issues haven't been touched in over two weeks."
        ),
        // I18N-004 fix for ISSUE-006 finding 3 -- the count is now a
        // typed parameter rather than an embedded "N over" string,
        // which is what produced the doubling.
        MessageKey::IndicatorExplanationWipCompliance { count } => {
            format!("{count} active assignees are over their WIP limit.")
        }

        // ---- I18N-002/004: project_health::summarize ----
        MessageKey::HealthSummaryHealthy => "Looking healthy.".to_string(),
        // I18N-004: the only two reachable shapes now -- see this
        // variant's doc comment in message.rs.
        MessageKey::HealthSummaryOneWatch { label } => {
            format!("{} is worth a glance.", indicator_label(label))
        }
        MessageKey::HealthSummaryTwoWatch { first, second } => format!(
            "{} and {} are worth a glance.",
            indicator_label(first),
            indicator_label(second)
        ),

        // ---- I18N-002: user_burnout::summarize ----
        MessageKey::BurnoutSummarySteady => "Steady so far.".to_string(),
        MessageKey::BurnoutSummaryOverloadOnly { days } => format!(
            "you've been over capacity for {days} recent snapshots — \
             consider whether some work can wait or move"
        ),
        MessageKey::BurnoutSummaryStalledOnly { days } => format!(
            "an assigned issue has been stuck for {days} days — \
             worth a quick check whether it's blocked"
        ),
        MessageKey::BurnoutSummaryBoth {
            overload_days,
            stalled_days,
        } => format!(
            "you've been over capacity for {overload_days} recent snapshots — \
             consider whether some work can wait or move; \
             an assigned issue has been stuck for {stalled_days} days — \
             worth a quick check whether it's blocked"
        ),

        // ---- I18N-003: peisear_notify::edge ----
        MessageKey::NotificationBurnoutOverloadTitle => {
            "Sustained over-capacity streak".to_string()
        }
        MessageKey::NotificationBurnoutOverloadBody { streak_snapshots } => format!(
            "Your in-flight load has been over capacity for {streak_snapshots} \
             consecutive snapshots. This is a description of the recent rhythm, \
             not an evaluation of your work — many streaks have legitimate causes. \
             You can review at /today."
        ),
        MessageKey::NotificationBurnoutStalledTitle => "Long-stalled assigned work".to_string(),
        MessageKey::NotificationBurnoutStalledBody { stalled_days } => format!(
            "An assigned issue has been in flight for {stalled_days} days. \
             May be worth a glance — sometimes a quick check-in turns out to be \
             all that's needed. Visit /today for context."
        ),

        // ---- I18N-005a: components/{layout,breadcrumb,error_page} ----
        MessageKey::AppBrandName => "Issue Tracker".to_string(),
        MessageKey::NavBellLabelNone => "Notifications".to_string(),
        MessageKey::NavBellLabelUnread { count } => format!("Notifications ({count} unread)"),
        MessageKey::NavBellCount { count } => bell_count(count),
        MessageKey::NavSearchFormLabel => "Search projects and open issues".to_string(),
        MessageKey::NavSearchPlaceholder => "Search...".to_string(),
        MessageKey::NavSearchQueryLabel => "Search query".to_string(),
        MessageKey::NavSearchSuggestionsLabel => "Search suggestions".to_string(),
        MessageKey::NavLinkToday => "Today".to_string(),
        MessageKey::NavLinkTeams => "Teams".to_string(),
        MessageKey::NavLinkInbox => "Inbox".to_string(),
        MessageKey::NavLinkSettings => "Settings".to_string(),
        MessageKey::NavSignOut => "Sign out".to_string(),
        MessageKey::BreadcrumbNavLabel => "Breadcrumb".to_string(),
        MessageKey::BackToSection { section } => format!("Back to {}", nav_section(section)),
        MessageKey::ErrorPageTitle => "Error — Issue Tracker".to_string(),
        MessageKey::ErrorPageGoHomeLink => "Go home".to_string(),
    }
}

/// Shared by `IndicatorValueThroughput` and
/// `IndicatorExplanationThroughput` so the two stay byte-identical in
/// how they render the same underlying value — the explanation
/// sentence embeds exactly what the value chip shows, not a
/// re-derived approximation of it.
fn throughput_value(done: i64, total: i64) -> String {
    let pct = (done * 100) / total;
    format!("{done} / {total} ({pct}%)")
}

/// The navbar bell badge's own visible number: `IndicatorValueActivity`'s
/// bare-count shape, plus a display cap so the badge doesn't grow
/// wider than its fixed-size circle for a busy inbox.
fn bell_count(count: i64) -> String {
    if count > 99 {
        "99+".to_string()
    } else {
        count.to_string()
    }
}

/// The three destinations a back-link can name. Lowercase throughout,
/// consistent with `NavSignOut`'s sentence-case convention ("Sign
/// out", not "Sign Out") — the leading word of the enclosing "Back
/// to " phrase carries the capital, not this word. `I18N-005a-review.md`
/// §2's own evidence for why this needed a table: the two call sites
/// this crate never saw had already drifted to "Projects" and
/// "sprints".
fn nav_section(section: NavSection) -> &'static str {
    match section {
        NavSection::Projects => "projects",
        NavSection::Issues => "issues",
        NavSection::Sprints => "sprints",
    }
}

fn indicator_label(label: IndicatorLabel) -> &'static str {
    match label {
        IndicatorLabel::Throughput => "Throughput",
        IndicatorLabel::Staleness => "Oldest in-flight",
        IndicatorLabel::Activity => "Activity (14d)",
        IndicatorLabel::BusFactor => "Bus factor",
        IndicatorLabel::LongStale => "Long-stale",
        IndicatorLabel::WipCompliance => "WIP compliance",
    }
}

fn entity_label(entity: EntityKind) -> &'static str {
    match entity {
        EntityKind::Issue => "issue",
        EntityKind::Project => "project",
        EntityKind::Sprint => "sprint",
        EntityKind::Team => "team",
        EntityKind::CapacityPeriod => "capacity period",
        EntityKind::TeamMembership => "team membership",
    }
}

fn field_label(field: Field) -> &'static str {
    match field {
        Field::EffortPoints => "Effort",
        Field::CapacityPoints => "Capacity points",
        Field::CloseDate => "Close date",
    }
}
