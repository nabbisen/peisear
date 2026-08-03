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

use crate::message::{EntityKind, Field, MessageKey};

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
