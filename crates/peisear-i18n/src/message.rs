//! The message key. Every user-visible string in peisear is, or will
//! be, a [`MessageKey`] variant plus typed parameters — never a
//! positional `{0}` and never prose composed inline at the call site
//! (`RFC 006` requirement 7; no concatenation, per the handoff).
//!
//! Seeded here with a small, real set of `AppError`/validation
//! messages (`peisear-web/src/error.rs`, `handlers/settings.rs`,
//! `handlers/issues.rs` as of 0.20.0) — the shortest, least ambiguous
//! copy in the system, per the handoff's guidance not to attempt
//! breadth in this unit. Converting the real call sites to use this
//! crate is out of scope here; that is handoff 4e.

/// The entity kinds `check_optimistic_lock` and its callers name in
/// conflict messages. Matches the documented set in
/// `peisear-web/src/error.rs`'s `OptimisticLockConflict` doc comment.
/// A closed, system-controlled vocabulary — not user data — so it is
/// exhaustively enumerable, unlike an issue title or display name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Issue,
    Project,
    Sprint,
    Team,
    CapacityPeriod,
    TeamMembership,
}

impl EntityKind {
    pub fn all() -> [EntityKind; 6] {
        [
            EntityKind::Issue,
            EntityKind::Project,
            EntityKind::Sprint,
            EntityKind::Team,
            EntityKind::CapacityPeriod,
            EntityKind::TeamMembership,
        ]
    }
}

/// A named form field, for validation messages that need to say
/// which field was wrong. Closed and system-controlled for the same
/// reason as [`EntityKind`] — the field being validated is a fixed
/// property of the form, not something a user typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    EffortPoints,
    CapacityPoints,
    CloseDate,
}

impl Field {
    pub fn all() -> [Field; 3] {
        [Field::EffortPoints, Field::CapacityPoints, Field::CloseDate]
    }
}

/// One message this crate can render, in every shipped locale.
///
/// A key enum, not a string constant and not a `HashMap<&str, &str>`:
/// a `match` over this type that omits a variant fails to compile,
/// which is what makes a missing rendering a compile-time error
/// (`RFC 006` requirement 2) rather than a runtime fallback to the
/// key name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKey {
    /// `AppError::Forbidden`'s public message.
    Forbidden,
    /// `AppError::NotFound`'s public message.
    NotFound,
    /// `AppError::Internal`'s public message — no internal detail
    /// leaks into this rendering; that stays in the `tracing::error!`
    /// call at the point the error is constructed.
    InternalError,
    /// `AppError::OptimisticLockConflict`'s public message.
    OptimisticLockConflict { entity: EntityKind },
    /// `check_optimistic_lock`'s parse-failure message — entity-
    /// neutral since 0.20.0 (`DEV-001-004-review.md` §1.4).
    LockValueUnreadable,
    /// A named field is required and was left blank.
    FieldRequired { field: Field },
    /// A named field must be a positive integer.
    FieldMustBePositiveInteger { field: Field },
    /// The `status` form field didn't parse to a known `IssueStatus`.
    InvalidStatus,
    /// The `priority` form field didn't parse to a known `Priority`.
    InvalidPriority,
}

impl MessageKey {
    /// Every key this crate defines, with every closed-set parameter
    /// combination it can carry — the full, finite set of messages a
    /// locale table must render. Parameters here ([`EntityKind`],
    /// [`Field`]) are closed system vocabulary, never open-ended user
    /// data, so full enumeration is both possible and meaningful: this
    /// is what "every entry of every locale table" means in practice
    /// for a guard that must not inspect interpolated user text.
    pub fn all() -> Vec<MessageKey> {
        let mut keys = vec![
            MessageKey::Forbidden,
            MessageKey::NotFound,
            MessageKey::InternalError,
            MessageKey::LockValueUnreadable,
            MessageKey::InvalidStatus,
            MessageKey::InvalidPriority,
        ];
        keys.extend(
            EntityKind::all()
                .into_iter()
                .map(|entity| MessageKey::OptimisticLockConflict { entity }),
        );
        keys.extend(
            Field::all()
                .into_iter()
                .map(|field| MessageKey::FieldRequired { field }),
        );
        keys.extend(
            Field::all()
                .into_iter()
                .map(|field| MessageKey::FieldMustBePositiveInteger { field }),
        );
        keys
    }
}
