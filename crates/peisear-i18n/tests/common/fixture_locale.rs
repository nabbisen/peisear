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

use peisear_i18n::{EntityKind, Field, MessageKey};

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
