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

/// The outcome of a write guarded by an optimistic-lock stamp
/// (`RACE-002`): either it was written, or the row's `updated_at` was no
/// longer the one the caller read and **nothing was written**.
///
/// A write guarded by a stamp is one the caller read a row's `updated_at`
/// for, and asks the storage layer to apply only if the row still carries
/// exactly that value. The comparison happens *in* the write -- either as
/// a `WHERE` predicate on the one statement, or, for a write that is
/// several statements, under a `BEGIN IMMEDIATE` transaction that holds
/// the write lock from the stamp's read to the commit -- rather than in
/// the caller, ahead of it, against a value read earlier. A missing row is
/// still [`StorageError::NotFound`]; only "the row is there and has
/// moved" is [`Guarded::Stale`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Guarded<T> {
    /// The stamp still matched (or none was supplied) and the write landed.
    Written(T),
    /// The row's stamp had moved; carries its current value, so a caller
    /// can build the conflict a stale page is owed.
    Stale {
        current_updated_at: chrono::DateTime<chrono::Utc>,
    },
}

impl<T> Guarded<T> {
    /// For an unguarded call (no stamp supplied), where `Stale` cannot
    /// occur: the value written.
    pub(crate) fn unguarded(self) -> T {
        match self {
            Self::Written(v) => v,
            Self::Stale { .. } => unreachable!("no stamp was supplied, so nothing can be stale"),
        }
    }
}

/// `None` (no stamp) always matches; `Some` must equal the row's value
/// exactly, as `check_optimistic_lock` always compared them.
pub(crate) fn stamp_matches(
    expected: Option<chrono::DateTime<chrono::Utc>>,
    current: chrono::DateTime<chrono::Utc>,
) -> bool {
    expected.is_none_or(|e| e == current)
}

/// A single-statement guarded write matched zero rows. Read the row
/// again -- *after* the failed write, so it cannot reintroduce the race --
/// to say which of the two outcomes it was: gone or not visible to this
/// caller ([`StorageError::NotFound`], exactly as an unguarded zero-row
/// write always was), or present with a different stamp ([`Guarded::Stale`]).
///
/// `table` and `scope_column` are literals at every call site.
pub(crate) async fn zero_rows_outcome<'e, E, T>(
    executor: E,
    table: &str,
    id: &str,
    scope: Option<(&str, &str)>,
) -> StorageResult<Guarded<T>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let sql = match scope {
        Some((col, _)) => format!("SELECT updated_at FROM {table} WHERE id = ?1 AND {col} = ?2"),
        None => format!("SELECT updated_at FROM {table} WHERE id = ?1"),
    };
    let mut q = sqlx::query_scalar::<_, chrono::DateTime<chrono::Utc>>(&sql).bind(id);
    if let Some((_, value)) = scope {
        q = q.bind(value);
    }
    match q.fetch_optional(executor).await? {
        None => Err(StorageError::NotFound),
        Some(current_updated_at) => Ok(Guarded::Stale { current_updated_at }),
    }
}
