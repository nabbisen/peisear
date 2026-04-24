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

pub mod issues;
pub mod pool;
pub mod projects;
pub mod users;

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
}

pub type StorageResult<T> = Result<T, StorageError>;
