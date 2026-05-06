//! Period-scoped per-user capacity.
//!
//! Replaces the 0.5.0 `users.capacity_points` field with a
//! separate table that lets capacity vary over time. See
//! `migrations/0009_user_capacities.sql` for the schema rationale.
//!
//! ## API shape
//!
//! - [`effective_for_user`] — "what's their capacity today?"
//!   The hot path read; returns `Option<i64>` where `None`
//!   means no row covers today (i.e., no capacity set).
//! - [`effective_for_user_on_date`] — same, but for an
//!   arbitrary date. Used by `user_metrics_snapshots` so a
//!   snapshot taken in the past records the capacity that was
//!   effective then, not now.
//! - [`list_for_user`] — all rows for one user, for the
//!   `/settings` UI.
//! - [`insert`] / [`update`] / [`delete`] — CRUD with
//!   application-layer overlap detection. Conflicting rows are
//!   rejected with [`StorageError::Conflict`].
//!
//! ## Why application-layer overlap detection
//!
//! Schema-level CHECK constraints in SQLite are row-local: they
//! can validate within a single row but cannot enforce "no two
//! rows for the same user have overlapping periods". A trigger
//! could, but triggers are easy to bypass and harder to test.
//!
//! Doing the check in Rust:
//! - keeps the SQL surface simple,
//! - lets us produce a *useful* error (the conflicting row's id
//!   and dates), rather than a generic constraint violation,
//! - is straightforward to test in isolation.
//!
//! There is a small window between `overlaps_existing` and the
//! INSERT where a concurrent writer could land a conflicting row.
//! For peisear's single-process / WAL-serialized-write model
//! this window is zero in practice, but a future PostgreSQL
//! backend would want a transaction-level lock or an exclusion
//! constraint. Documented; not addressed today.

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{Pool, StorageError, StorageResult};

/// One capacity row, deserialised from storage.
#[derive(Debug, Clone)]
pub struct CapacityRow {
    pub id: String,
    pub user_id: String,
    pub points: i64,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub note: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Find the row whose period covers `today`, returning its
/// `points` value. `None` means no row covers today (the user
/// has no capacity set, or all rows are out of range).
pub async fn effective_for_user(pool: &Pool, user_id: &str) -> StorageResult<Option<i64>> {
    let today = chrono::Utc::now().date_naive();
    effective_for_user_on_date(pool, user_id, today).await
}

/// Find the full row that's effective today. Distinct from
/// [`effective_for_user`] in that it returns the row, not just
/// the points value, so callers can tell whether the active row
/// is period-bounded ("(this period)" UI hint, etc.).
pub async fn effective_row_for_user(
    pool: &Pool,
    user_id: &str,
) -> StorageResult<Option<CapacityRow>> {
    let today = chrono::Utc::now().date_naive();
    let row: Option<(
        String,
        String,
        i64,
        Option<NaiveDate>,
        Option<NaiveDate>,
        Option<String>,
        chrono::DateTime<chrono::Utc>,
    )> = sqlx::query_as(
        r#"
        SELECT id, user_id, points, period_start, period_end, note, created_at
        FROM user_capacities
        WHERE user_id = ?1
          AND (period_start IS NULL OR period_start <= ?2)
          AND (period_end   IS NULL OR period_end   >= ?2)
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .bind(today)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| CapacityRow {
        id: r.0,
        user_id: r.1,
        points: r.2,
        period_start: r.3,
        period_end: r.4,
        note: r.5,
        created_at: r.6,
    }))
}

/// Find the row whose period covers `on_date`. Used by snapshot
/// writers to honour the capacity that was effective at the time
/// of the snapshot.
///
/// Multiple matching rows are an invariant violation (the
/// overlap check should have prevented it). If it happens
/// anyway, we log and pick the most recently created row. Better
/// to render a number than to fail the page render.
pub async fn effective_for_user_on_date(
    pool: &Pool,
    user_id: &str,
    on_date: NaiveDate,
) -> StorageResult<Option<i64>> {
    let rows: Vec<(i64,)> = sqlx::query_as(
        r#"
        SELECT points FROM user_capacities
        WHERE user_id = ?1
          AND (period_start IS NULL OR period_start <= ?2)
          AND (period_end   IS NULL OR period_end   >= ?2)
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .bind(on_date)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().next().map(|(p,)| p))
}

/// All rows for one user, ordered by `period_start` ascending.
/// `NULL period_start` (the open-beginning default) sorts first.
/// Used by the `/settings` UI to render the capacity table.
pub async fn list_for_user(pool: &Pool, user_id: &str) -> StorageResult<Vec<CapacityRow>> {
    let rows: Vec<(
        String,
        String,
        i64,
        Option<NaiveDate>,
        Option<NaiveDate>,
        Option<String>,
        chrono::DateTime<chrono::Utc>,
    )> = sqlx::query_as(
        r#"
        SELECT id, user_id, points, period_start, period_end, note, created_at
        FROM user_capacities
        WHERE user_id = ?1
        ORDER BY
            CASE WHEN period_start IS NULL THEN 0 ELSE 1 END,
            period_start ASC,
            created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| CapacityRow {
            id: r.0,
            user_id: r.1,
            points: r.2,
            period_start: r.3,
            period_end: r.4,
            note: r.5,
            created_at: r.6,
        })
        .collect())
}

/// Find one row by id (scoped to user_id for safety). Returns
/// `None` when no such row exists for this user.
pub async fn find(
    pool: &Pool,
    user_id: &str,
    id: &str,
) -> StorageResult<Option<CapacityRow>> {
    let row: Option<(
        String,
        String,
        i64,
        Option<NaiveDate>,
        Option<NaiveDate>,
        Option<String>,
        chrono::DateTime<chrono::Utc>,
    )> = sqlx::query_as(
        r#"
        SELECT id, user_id, points, period_start, period_end, note, created_at
        FROM user_capacities
        WHERE id = ?1 AND user_id = ?2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| CapacityRow {
        id: r.0,
        user_id: r.1,
        points: r.2,
        period_start: r.3,
        period_end: r.4,
        note: r.5,
        created_at: r.6,
    }))
}

/// Information about a row that conflicts with a proposed
/// insert/update. Surfaced in [`StorageError::Conflict`] so the
/// UI can produce a useful error message.
#[derive(Debug, Clone)]
pub struct ConflictInfo {
    pub id: String,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub points: i64,
}

/// Find an existing row whose period overlaps with the proposed
/// `[period_start, period_end]`. Optionally excludes a row id
/// (for the update case).
///
/// "Overlap" semantics: two periods overlap unless one ends
/// strictly before the other starts. Treating NULL as "infinity"
/// on either side, this becomes:
///
/// - existing ends before proposed starts: existing.period_end IS NOT NULL
///   AND proposed.period_start IS NOT NULL
///   AND existing.period_end < proposed.period_start
/// - existing starts after proposed ends: existing.period_start IS NOT NULL
///   AND proposed.period_end IS NOT NULL
///   AND existing.period_start > proposed.period_end
///
/// Negate that and you get the overlap condition.
pub async fn overlaps_existing(
    pool: &Pool,
    user_id: &str,
    period_start: Option<NaiveDate>,
    period_end: Option<NaiveDate>,
    excluding_id: Option<&str>,
) -> StorageResult<Option<ConflictInfo>> {
    // We bind period_start/period_end as Option<NaiveDate>; sqlx
    // turns None into NULL and the SQL expression handles it.
    //
    // The expression "NOT (a OR b)" form below reads more
    // naturally as "they overlap iff neither ends-before nor
    // starts-after holds".
    let exclude_id = excluding_id.unwrap_or("");
    let row: Option<(String, Option<NaiveDate>, Option<NaiveDate>, i64)> = sqlx::query_as(
        r#"
        SELECT id, period_start, period_end, points
        FROM user_capacities
        WHERE user_id = ?1
          AND id != ?2
          AND NOT (
              (period_end   IS NOT NULL AND ?3 IS NOT NULL AND period_end   < ?3)
           OR (period_start IS NOT NULL AND ?4 IS NOT NULL AND period_start > ?4)
          )
        ORDER BY created_at ASC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .bind(exclude_id)
    .bind(period_start)
    .bind(period_end)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(id, ps, pe, points)| ConflictInfo {
        id,
        period_start: ps,
        period_end: pe,
        points,
    }))
}

/// Insert a new capacity row. Refuses with `StorageError::Conflict`
/// if the proposed period overlaps any existing row for this user.
pub async fn insert(
    pool: &Pool,
    user_id: &str,
    points: i64,
    period_start: Option<NaiveDate>,
    period_end: Option<NaiveDate>,
    note: Option<&str>,
) -> StorageResult<String> {
    if let (Some(s), Some(e)) = (period_start, period_end) {
        if s > e {
            // Defensive — the schema CHECK catches this too, but we
            // produce a nicer error message before hitting the DB.
            return Err(StorageError::Validation(
                "period_start must be on or before period_end".into(),
            ));
        }
    }
    if let Some(conflict) = overlaps_existing(pool, user_id, period_start, period_end, None).await? {
        return Err(StorageError::Conflict(format!(
            "row {} ({} to {}, {} pt) overlaps the proposed period",
            conflict.id,
            conflict.period_start.map(|d| d.to_string()).unwrap_or_else(|| "—".into()),
            conflict.period_end.map(|d| d.to_string()).unwrap_or_else(|| "—".into()),
            conflict.points,
        )));
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO user_capacities
            (id, user_id, points, period_start, period_end, note)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(user_id)
    .bind(points)
    .bind(period_start)
    .bind(period_end)
    .bind(note)
    .execute(pool)
    .await?;
    Ok(id)
}

/// Update an existing capacity row. Same conflict rules as
/// [`insert`], excluding the row being updated.
pub async fn update(
    pool: &Pool,
    user_id: &str,
    id: &str,
    points: i64,
    period_start: Option<NaiveDate>,
    period_end: Option<NaiveDate>,
    note: Option<&str>,
) -> StorageResult<()> {
    if let Some(conflict) =
        overlaps_existing(pool, user_id, period_start, period_end, Some(id)).await?
    {
        return Err(StorageError::Conflict(format!(
            "row {} ({} to {}, {} pt) overlaps the proposed period",
            conflict.id,
            conflict.period_start.map(|d| d.to_string()).unwrap_or_else(|| "—".into()),
            conflict.period_end.map(|d| d.to_string()).unwrap_or_else(|| "—".into()),
            conflict.points,
        )));
    }

    let res = sqlx::query(
        r#"
        UPDATE user_capacities
        SET points = ?3, period_start = ?4, period_end = ?5, note = ?6
        WHERE id = ?1 AND user_id = ?2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(points)
    .bind(period_start)
    .bind(period_end)
    .bind(note)
    .execute(pool)
    .await?;

    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// Delete one row. Scoped to `user_id` for safety.
pub async fn delete(pool: &Pool, user_id: &str, id: &str) -> StorageResult<()> {
    let res = sqlx::query(
        r#"DELETE FROM user_capacities WHERE id = ?1 AND user_id = ?2"#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// "Close" a row by setting its `period_end` to a specific date.
/// A common UI flow when the user wants to add a new row that
/// would conflict with an open-ended existing one. This is just
/// `update` with a constrained call shape, but having it as a
/// dedicated method makes the handler code clearer.
pub async fn close_at(
    pool: &Pool,
    user_id: &str,
    id: &str,
    new_period_end: NaiveDate,
) -> StorageResult<()> {
    let row = find(pool, user_id, id).await?.ok_or(StorageError::NotFound)?;
    update(
        pool,
        user_id,
        id,
        row.points,
        row.period_start,
        Some(new_period_end),
        row.note.as_deref(),
    )
    .await
}
