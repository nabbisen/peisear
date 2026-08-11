//! The single place every user-visible string in peisear lives.
//!
//! `peisear-core`, `peisear-notify`, and `peisear-web` will depend on
//! this crate (starting with `I18N-002`); it depends on none of them
//! — a leaf crate, so the domain crate can emit message keys without
//! a dependency cycle back to presentation.
//!
//! ## What this handoff builds
//!
//! The mechanism, proved to work, with nothing converted yet:
//!
//! - [`MessageKey`] — a key enum. [`Locale::render`] is an exhaustive
//!   `match` per locale, so a key without a rendering fails to
//!   compile rather than falling back to a raw key string at runtime.
//! - [`guard::find_violations`] — walks rendered text for §1.7
//!   prohibited vocabulary, word-boundary aware and case-insensitive.
//!   `tests/guard.rs` runs it over every entry of every locale table,
//!   shipped and fixture, as a **blocking** CI job, and separately
//!   proves the guard actually rejects a planted violation (a guard
//!   never observed failing is not known to work).
//!
//! ## What this handoff does not build
//!
//! No existing crate is modified and no real surface is converted —
//! that is `I18N-002` through `I18N-004e`. The message set here is a
//! small, real seed (`AppError`/validation copy as of 0.20.0), enough
//! to exercise the mechanism honestly, not a migration.
//!
//! ## Locales
//!
//! English only ships ([`Locale`], `NFR-LANG-005`). The mechanism is
//! proved locale-agnostic by a fixture locale that lives only in this
//! crate's tests (`tests/common/`), never as a [`Locale`] variant —
//! see that module's doc comment for why.

pub mod guard;
pub mod locale;
pub mod message;

mod en;

pub use guard::{PROHIBITED_TERMS, ProhibitedTerm, find_violations};
pub use locale::Locale;
pub use message::{
    DriftDirectionLabel, EntityKind, Field, HealthStateLabel, IndicatorLabel, IssueStatusLabel,
    MessageKey, NavSection, NotificationChannelLabel, NotificationKindLabel, PriorityLabel,
    SprintStatusLabel, TeamRoleLabel,
};
