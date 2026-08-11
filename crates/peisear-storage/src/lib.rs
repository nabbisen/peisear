//! Persistence layer.
//!
//! The current implementation is backed by SQLite via `sqlx`. The public
//! shape (query functions and the [`StorageError`] type) is intentionally
//! concrete rather than trait‑abstracted — trait abstraction becomes
//! useful once a second backend (PostgreSQL, per the roadmap) is in
//! flight; until then it is speculative infrastructure.
//!
//! The `Pool` alias below names the backend‑specific pool type so that a
//! future `storage-postgres` sibling crate, or a `backend` feature flag,
//! can swap the type without callers changing their signatures.

pub mod issue_events;
pub mod issues;
pub mod metrics_snapshots;
pub mod notifications;
pub mod personal_metrics;
pub mod pool;
pub mod project_health;
pub mod projects;
pub mod search;
pub mod sprints;
pub mod teams;
pub mod user_burnout;
pub mod user_capacities;
pub mod user_metrics_snapshots;
pub mod users;
pub mod view_states;

/// Active backend‑specific pool type. Swap this alias (or trait‑abstract
/// it) when adding another backend.
pub type Pool = sqlx::SqlitePool;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// Underlying database error (network, schema, etc.).
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    /// Migration runner error at startup.
    #[error("migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// A value pulled back from the database cannot be mapped onto a
    /// domain enum. Always an internal invariant violation (e.g. the
    /// CHECK constraint was bypassed).
    #[error("invalid data in storage: {0}")]
    InvalidData(String),

    /// The requested row does not exist or is not accessible by the
    /// caller. Used both for genuinely missing rows and for access
    /// control (e.g. find_accessible) — the caller is not told which.
    #[error("not found")]
    NotFound,

    /// Environment / filesystem problem before the query could run
    /// (e.g. the DB file's parent directory could not be created).
    #[error("storage bootstrap error: {0}")]
    Bootstrap(String),

    /// Application-level invariant violation: the proposed write
    /// would conflict with existing data, but the conflict is
    /// detectable cleanly enough to return a useful message.
    /// Used by `user_capacities` for period overlaps. Carries a
    /// [`peisear_i18n::MessageKey`], not a rendered `String`
    /// (`I18N-006` §5) — `peisear-web` renders it at the crossing
    /// boundary via `From<StorageError>`.
    #[error("conflict: {0:?}")]
    Conflict(peisear_i18n::MessageKey),

    /// The proposed write fails a domain rule before the SQL
    /// constraint catches it. Distinct from `Database(...)` so the
    /// web layer can map it to `400 Bad Request` instead of
    /// `500 Internal Server Error`. Carries a
    /// [`peisear_i18n::MessageKey`], not a rendered `String`
    /// (`I18N-006` §5) — same rationale as `Conflict` above.
    #[error("validation: {0:?}")]
    Validation(peisear_i18n::MessageKey),
}

pub type StorageResult<T> = Result<T, StorageError>;
