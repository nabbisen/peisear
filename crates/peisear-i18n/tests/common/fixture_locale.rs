//! A locale that exists only here, in test code — never a
//! [`peisear_i18n::Locale`] variant, never shipped.
//!
//! `I18N-001` §4.5 / RFC 006 open question 4's default: proving the
//! mechanism is not secretly English-shaped requires a second,
//! genuinely distinct rendering path exercised by the same tests as
//! the shipped English table — without committing to a real second
//! locale (Japanese, the obvious candidate) that would then drift
//! unmaintained while `NFR-LANG-005` keeps it unshipped. Every value
//! below is deliberately unlike English (bracketed `[fx …]` tokens)
//! so a test comparing the two outputs proves rendering switches
//! wholesale, not that it happens to differ in one place.

use peisear_i18n::{EntityKind, Field, IndicatorLabel, MessageKey};

pub fn render(key: MessageKey) -> String {
    match key {
        MessageKey::Forbidden => "[fx] access denied".to_string(),
        MessageKey::NotFound => "[fx] nothing here".to_string(),
        MessageKey::InternalError => "[fx] internal trouble — try again".to_string(),
        MessageKey::OptimisticLockConflict { entity } => format!(
            "[fx] {} changed elsewhere while editing",
            entity_label(entity)
        ),
        MessageKey::LockValueUnreadable => "[fx] stale page — reload".to_string(),
        MessageKey::FieldRequired { field } => format!("[fx] {} needed", field_label(field)),
        MessageKey::FieldMustBePositiveInteger { field } => {
            format!("[fx] {} needs a positive whole number", field_label(field))
        }
        MessageKey::InvalidStatus => "[fx] bad status".to_string(),
        MessageKey::InvalidPriority => "[fx] bad priority".to_string(),

        // ---- I18N-002: format_value ----
        MessageKey::IndicatorValueUnavailable => "[fx-none]".to_string(),
        MessageKey::IndicatorValueThroughput { done, total } => {
            format!("[fx] {done} of {total} done")
        }
        MessageKey::IndicatorValueStaleness { days } => format!("[fx] {days} days old"),
        MessageKey::IndicatorValueActivity { count } => format!("[fx-activity-{count}]"),
        MessageKey::IndicatorValueBusFactorSolo => "[fx-solo]".to_string(),
        MessageKey::IndicatorValueBusFactor { pct } => format!("[fx] {pct} pct concentrated"),
        MessageKey::IndicatorValueLongStale { stale, in_flight } => {
            format!("[fx] {stale} of {in_flight} stale")
        }
        MessageKey::IndicatorValueWipAllWithin => "[fx-wip-ok]".to_string(),
        MessageKey::IndicatorValueWipOver { count } => format!("[fx] {count} over wip"),

        // ---- I18N-002: human_explanation ----
        MessageKey::IndicatorExplanationThroughput { done, total } => {
            format!("[fx] throughput note: {done} of {total}")
        }
        MessageKey::IndicatorExplanationStaleness { days } => {
            format!("[fx] stale note: {days} days")
        }
        MessageKey::IndicatorExplanationActivity { count } => {
            format!("[fx] activity note: {count}")
        }
        MessageKey::IndicatorExplanationBusFactorSolo => "[fx] solo note".to_string(),
        MessageKey::IndicatorExplanationBusFactor { pct } => {
            format!("[fx] concentration note: {pct} pct")
        }
        MessageKey::IndicatorExplanationLongStale { stale, in_flight } => {
            format!("[fx] stale note: {stale} of {in_flight}")
        }
        MessageKey::IndicatorExplanationWipCompliance { count } => {
            format!("[fx] wip note: {count}")
        }

        // ---- I18N-002: project_health::summarize ----
        MessageKey::HealthSummaryHealthy => "[fx-healthy]".to_string(),
        MessageKey::HealthSummaryOneWatch { label } => {
            format!("[fx] {} worth a look", indicator_label(label))
        }
        MessageKey::HealthSummaryOneConcern { label } => {
            format!("[fx] {} flagged", indicator_label(label))
        }
        MessageKey::HealthSummaryTwoWatch { first, second } => format!(
            "[fx] {} and {} worth a look",
            indicator_label(first),
            indicator_label(second)
        ),
        MessageKey::HealthSummaryConcernPlusOne { first, second } => format!(
            "[fx] {} flagged, {} too",
            indicator_label(first),
            indicator_label(second)
        ),

        // ---- I18N-002: user_burnout::summarize ----
        MessageKey::BurnoutSummarySteady => "[fx-steady]".to_string(),
        MessageKey::BurnoutSummaryOverloadOnly { days } => {
            format!("[fx] overload note: {days} days")
        }
        MessageKey::BurnoutSummaryStalledOnly { days } => {
            format!("[fx] stalled note: {days} days")
        }
        MessageKey::BurnoutSummaryBoth {
            overload_days,
            stalled_days,
        } => format!("[fx] overload {overload_days}, stalled {stalled_days}"),
    }
}

fn entity_label(entity: EntityKind) -> &'static str {
    match entity {
        EntityKind::Issue => "[fx-issue]",
        EntityKind::Project => "[fx-project]",
        EntityKind::Sprint => "[fx-sprint]",
        EntityKind::Team => "[fx-team]",
        EntityKind::CapacityPeriod => "[fx-capacity-period]",
        EntityKind::TeamMembership => "[fx-team-membership]",
    }
}

fn field_label(field: Field) -> &'static str {
    match field {
        Field::EffortPoints => "[fx-effort]",
        Field::CapacityPoints => "[fx-capacity-points]",
        Field::CloseDate => "[fx-close-date]",
    }
}

fn indicator_label(label: IndicatorLabel) -> &'static str {
    match label {
        IndicatorLabel::Throughput => "[fx-throughput]",
        IndicatorLabel::Staleness => "[fx-staleness]",
        IndicatorLabel::Activity => "[fx-activity]",
        IndicatorLabel::BusFactor => "[fx-busfactor]",
        IndicatorLabel::LongStale => "[fx-longstale]",
        IndicatorLabel::WipCompliance => "[fx-wipcompliance]",
    }
}
