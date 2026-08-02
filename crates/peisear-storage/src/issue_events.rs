//! Append-only issue event log.
//!
//! See `migrations/0006_issue_events.sql` for the schema design
//! discussion. This module exposes:
//!
//! 1. **`insert_event`** — write a single event row inside an
//!    existing transaction. Callers are mutating helpers in
//!    [`crate::issues`] which already own a `&mut Transaction`.
//! 2. **Read helpers** for querying the log:
//!    [`latest_status_change`] (most-recent status_changed event
//!    per in-flight issue, used by long-stale detection) and
//!    [`active_in_progress_seconds`] (sum of in_progress dwell
//!    time for one issue, used by personal estimation skew).
//!
//! All event writes go through this module so the schema stays in
//! one place. Read queries can be added freely; writes shouldn't be.

use sqlx::Sqlite;
use uuid::Uuid;

use crate::StorageResult;

/// Event kinds. Mirrors the CHECK constraint in the migration.
/// Callers pass these as `&'static str` rather than constructing
/// strings, so a typo at the call site is a compile error.
pub mod kind {
    pub const CREATED: &str = "created";
    pub const STATUS_CHANGED: &str = "status_changed";
    pub const ASSIGNEE_CHANGED: &str = "assignee_changed";
    pub const EFFORT_CHANGED: &str = "effort_changed";
    pub const DELETED: &str = "deleted";
}

/// Insert one event row inside an in-progress transaction.
///
/// `actor_id` is `Option<&str>` because system-internal events
/// (cron-driven cleanups, future webhook ingest, etc.) may not
/// have a logged-in user behind them. 0.8.0 user-driven mutations
/// always pass `Some(actor)`.
pub async fn insert_event(
    tx: &mut sqlx::Transaction<'_, Sqlite>,
    issue_id: &str,
    project_id: &str,
    actor_id: Option<&str>,
    event_type: &str,
    previous_value: Option<&str>,
    new_value: Option<&str>,
) -> StorageResult<()> {
    let event_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO issue_events
            (id, issue_id, project_id, actor_id, event_type,
             previous_value, new_value)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
    )
    .bind(event_id)
    .bind(issue_id)
    .bind(project_id)
    .bind(actor_id)
    .bind(event_type)
    .bind(previous_value)
    .bind(new_value)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// For each in-flight (`open` / `in_progress`) issue in `project_id`,
/// the timestamp of its most recent `status_changed` event, in days
/// before now. `None` for issues that have no status_changed events
/// (most likely pre-0.8.0 issues whose status changes happened
/// before this release).
///
/// Returned as `(issue_id, days_since_last_status_change)`. The
/// caller decides what to do with `None` vs missing rows; the
/// `project_health` long-stale path falls back to the 0.7.0
/// `updated_at`-based approximation when this query has no entry
/// for a given issue.
pub async fn days_since_last_status_change_per_in_flight_issue(
    pool: &crate::Pool,
    project_id: &str,
) -> StorageResult<Vec<(String, f64)>> {
    let rows: Vec<(String, f64)> = sqlx::query_as(
        r#"
        SELECT
            i.id AS issue_id,
            julianday('now') - julianday(MAX(e.occurred_at)) AS days
        FROM issues i
        JOIN issue_events e
            ON e.issue_id = i.id
           AND e.event_type = 'status_changed'
        WHERE i.project_id = ?1
          AND i.status IN ('open', 'in_progress')
        GROUP BY i.id
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Sum of seconds spent in `in_progress` for one issue, computed
/// from its event log.
///
/// Returned as `Option<f64>` (seconds). `None` means "no
/// in_progress dwell could be reconstructed from events" — the
/// issue might have no status_changed events at all, or it never
/// entered in_progress. Callers should fall back to the calendar-
/// time approximation in that case.
///
/// ## Algorithm
///
/// We walk the status_changed event timeline for this issue:
/// every transition `* -> in_progress` opens a window, every
/// transition `in_progress -> *` closes it. If the issue is
/// currently `in_progress`, the open window's end is `now`.
///
/// SQL window functions would express this neatly, but SQLite's
/// `LAG` is fine with a small CTE; we keep it portable rather than
/// pull the calculation into Rust.
pub async fn in_progress_seconds_for_issue(
    pool: &crate::Pool,
    issue_id: &str,
) -> StorageResult<Option<f64>> {
    // Pull the timeline plus the issue's current status, so we can
    // tell whether the last open in_progress window is still open.
    let timeline: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT new_value AS status, occurred_at
        FROM issue_events
        WHERE issue_id = ?1
          AND event_type = 'status_changed'
        ORDER BY occurred_at ASC
        "#,
    )
    .bind(issue_id)
    .fetch_all(pool)
    .await?;

    if timeline.is_empty() {
        return Ok(None);
    }

    // Pull current status to know whether we're still in_progress.
    let current_status: Option<String> =
        sqlx::query_scalar(r#"SELECT status FROM issues WHERE id = ?1"#)
            .bind(issue_id)
            .fetch_optional(pool)
            .await?;

    let mut total_seconds = 0.0_f64;
    let mut window_start: Option<chrono::NaiveDateTime> = None;

    for (status, occurred_at) in &timeline {
        let occurred_at =
            chrono::NaiveDateTime::parse_from_str(occurred_at, "%Y-%m-%d %H:%M:%S").ok();
        let Some(occurred_at) = occurred_at else {
            continue;
        };

        if status == "in_progress" {
            // Opening a window. If one is already open (which would
            // be a duplicate event — shouldn't happen in normal
            // use), keep the earlier start.
            if window_start.is_none() {
                window_start = Some(occurred_at);
            }
        } else {
            // Closing a window if one is open.
            if let Some(start) = window_start.take() {
                let dur = (occurred_at - start).num_seconds().max(0) as f64;
                total_seconds += dur;
            }
        }
    }

    // If a window is still open and the issue is currently in
    // in_progress, count up to now. If the issue is no longer in
    // in_progress but our last event was 'in_progress' (which
    // shouldn't happen if events are written in sync with the
    // issues table, but be defensive), discard the orphan window.
    if let Some(start) = window_start {
        if current_status.as_deref() == Some("in_progress") {
            let now = chrono::Utc::now().naive_utc();
            let dur = (now - start).num_seconds().max(0) as f64;
            total_seconds += dur;
        }
    }

    Ok(Some(total_seconds))
}
