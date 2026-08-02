//! Notifications + preferences persistence.
//!
//! The two tables are populated separately:
//!
//! - `notifications` is written by the dispatch pipeline on every
//!   notification, regardless of whether any channel succeeded.
//!   This is both inbox storage (for the in-app channel) and an
//!   audit trail (for "what was sent at all").
//! - `notification_preferences` is written by the user via the
//!   `/settings/notifications` page. Absent rows fall back to
//!   [`peisear_core::notifications::DEFAULT_CHANNELS`] +
//!   [`peisear_core::notifications::DEFAULT_MIN_SEVERITY`].
//!
//! Reads:
//!
//! - [`recent_for_user`] feeds the inbox view.
//! - [`unread_count_for_user`] feeds the nav badge.
//! - [`last_dispatched_at_for_user_kind`] is the cooldown query.
//! - [`preferences_for_user`] returns all user-set rows; the
//!   web layer merges with defaults.
//!
//! Writes:
//!
//! - [`insert`] persists a new notification row.
//! - [`mark_read`] sets `read_at`.
//! - [`upsert_preference`] / [`set_global_acknowledged`] for the
//!   preferences page.

use chrono::{DateTime, Utc};
use peisear_core::notifications::{Notification, Preference, Severity};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{Pool, StorageError, StorageResult};

/// The content of a notification about to be persisted — every
/// field but the recipient (`user_id`, kept as its own argument
/// since it's the routing key, not content).
pub struct NewNotification<'a> {
    pub kind: &'a str,
    pub severity: Severity,
    pub title: &'a str,
    pub body: &'a str,
    pub payload_json: Option<&'a str>,
    /// Channel ids that successfully delivered. Empty is allowed
    /// (the row is then audit-only).
    pub dispatched_via: &'a [&'a str],
}

/// Persist a new notification row. Returns the new id.
pub async fn insert(pool: &Pool, user_id: &str, new: NewNotification<'_>) -> StorageResult<String> {
    let id = Uuid::new_v4().to_string();
    let dispatched_str = new.dispatched_via.join(",");

    sqlx::query(
        r#"
        INSERT INTO notifications
            (id, user_id, kind, severity, title, body, payload_json, dispatched_via)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(user_id)
    .bind(new.kind)
    .bind(new.severity.as_str())
    .bind(new.title)
    .bind(new.body)
    .bind(new.payload_json)
    .bind(&dispatched_str)
    .execute(pool)
    .await?;
    Ok(id)
}

/// Raw `notifications` row as returned by sqlx. Kept private —
/// the public API returns [`Notification`], which parses
/// `severity` and splits `dispatched_via` into a `Vec`.
#[derive(FromRow)]
struct NotificationRow {
    id: String,
    user_id: String,
    kind: String,
    severity: String,
    title: String,
    body: String,
    payload_json: Option<String>,
    created_at: DateTime<Utc>,
    read_at: Option<DateTime<Utc>>,
    dispatched_via: String,
}

impl From<NotificationRow> for Notification {
    fn from(r: NotificationRow) -> Self {
        Notification {
            id: r.id,
            user_id: r.user_id,
            kind: r.kind,
            severity: Severity::from_storage_str(&r.severity),
            title: r.title,
            body: r.body,
            payload_json: r.payload_json,
            created_at: r.created_at,
            read_at: r.read_at,
            dispatched_via: if r.dispatched_via.is_empty() {
                Vec::new()
            } else {
                r.dispatched_via.split(',').map(|s| s.to_string()).collect()
            },
        }
    }
}

/// Recent notifications for a user, newest first. Used by the
/// `/notifications` inbox.
pub async fn recent_for_user(
    pool: &Pool,
    user_id: &str,
    limit: i64,
) -> StorageResult<Vec<Notification>> {
    let rows = sqlx::query_as::<_, NotificationRow>(
        r#"
        SELECT id, user_id, kind, severity, title, body, payload_json,
               created_at, read_at, dispatched_via
        FROM notifications
        WHERE user_id = ?1
        ORDER BY created_at DESC
        LIMIT ?2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(Notification::from).collect())
}

/// Count of unread notifications. Used by the nav badge.
pub async fn unread_count_for_user(pool: &Pool, user_id: &str) -> StorageResult<i64> {
    let n: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM notifications
        WHERE user_id = ?1 AND read_at IS NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(n)
}

/// Mark one notification as read. Idempotent; already-read rows
/// keep their original `read_at` (the UPDATE filters on `read_at
/// IS NULL`).
pub async fn mark_read(pool: &Pool, user_id: &str, id: &str) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE notifications
        SET read_at = CURRENT_TIMESTAMP
        WHERE id = ?1 AND user_id = ?2 AND read_at IS NULL
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        // Either the row doesn't exist (or isn't ours) or it
        // was already read. Either is fine; the page hits
        // /mark-read on click, and re-clicking shouldn't 404.
        // We still verify the row exists at all so a typo
        // doesn't silently succeed.
        let exists: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM notifications WHERE id = ?1 AND user_id = ?2"#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        if exists == 0 {
            return Err(StorageError::NotFound);
        }
    }
    Ok(())
}

/// Mark all of this user's unread notifications as read in one
/// query. Used by the inbox "mark all read" button.
pub async fn mark_all_read(pool: &Pool, user_id: &str) -> StorageResult<i64> {
    let res = sqlx::query(
        r#"
        UPDATE notifications
        SET read_at = CURRENT_TIMESTAMP
        WHERE user_id = ?1 AND read_at IS NULL
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected() as i64)
}

/// "When was the last time we sent this kind to this user?"
/// Used by the cooldown filter at dispatch time. Returns `None`
/// if the user has never received this kind.
pub async fn last_dispatched_at_for_user_kind(
    pool: &Pool,
    user_id: &str,
    kind: &str,
) -> StorageResult<Option<DateTime<Utc>>> {
    let ts: Option<DateTime<Utc>> = sqlx::query_scalar(
        r#"
        SELECT MAX(created_at)
        FROM notifications
        WHERE user_id = ?1 AND kind = ?2
        "#,
    )
    .bind(user_id)
    .bind(kind)
    .fetch_optional(pool)
    .await?
    .flatten();
    Ok(ts)
}

/// All preference rows for a user. The web layer merges these
/// with the system defaults to render the full preferences page.
pub async fn preferences_for_user(pool: &Pool, user_id: &str) -> StorageResult<Vec<Preference>> {
    let rows: Vec<(String, String, String, String)> = sqlx::query_as(
        r#"
        SELECT user_id, kind, channels, min_severity
        FROM notification_preferences
        WHERE user_id = ?1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Preference {
            user_id: r.0,
            kind: r.1,
            channels: if r.2.is_empty() {
                Vec::new()
            } else {
                r.2.split(',').map(|s| s.to_string()).collect()
            },
            min_severity: Severity::from_storage_str(&r.3),
        })
        .collect())
}

/// Find the preference for one (user, kind), or `None` if
/// absent (caller falls back to defaults).
pub async fn preference_for_user_kind(
    pool: &Pool,
    user_id: &str,
    kind: &str,
) -> StorageResult<Option<Preference>> {
    let row: Option<(String, String, String, String)> = sqlx::query_as(
        r#"
        SELECT user_id, kind, channels, min_severity
        FROM notification_preferences
        WHERE user_id = ?1 AND kind = ?2
        "#,
    )
    .bind(user_id)
    .bind(kind)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Preference {
        user_id: r.0,
        kind: r.1,
        channels: if r.2.is_empty() {
            Vec::new()
        } else {
            r.2.split(',').map(|s| s.to_string()).collect()
        },
        min_severity: Severity::from_storage_str(&r.3),
    }))
}

/// Insert or update a preference row. Channels are normalised
/// before persistence (sorted, lowercased, de-duplicated).
pub async fn upsert_preference(
    pool: &Pool,
    user_id: &str,
    kind: &str,
    channels: &[&str],
    min_severity: Severity,
) -> StorageResult<()> {
    let mut chans: Vec<String> = channels
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    chans.sort();
    chans.dedup();
    let chans_str = chans.join(",");

    sqlx::query(
        r#"
        INSERT INTO notification_preferences (user_id, kind, channels, min_severity)
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(user_id, kind) DO UPDATE SET
            channels = excluded.channels,
            min_severity = excluded.min_severity
        "#,
    )
    .bind(user_id)
    .bind(kind)
    .bind(&chans_str)
    .bind(min_severity.as_str())
    .execute(pool)
    .await?;
    Ok(())
}

/// "Has this user been prompted for the first-login email
/// opt-in?" Implemented as a sentinel row with kind = '_global'.
/// Returns true if the row exists.
pub async fn global_acknowledged(pool: &Pool, user_id: &str) -> StorageResult<bool> {
    let n: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM notification_preferences
        WHERE user_id = ?1 AND kind = ?2
        "#,
    )
    .bind(user_id)
    .bind(peisear_core::notifications::kind::GLOBAL)
    .fetch_one(pool)
    .await?;
    Ok(n > 0)
}

/// Record that the first-login email opt-in has been answered,
/// regardless of whether the user said yes or no.
///
/// If `email_opt_in` is true, the global preference row stores
/// `channels = "in_app,email"`. Otherwise `channels = "in_app"`.
/// Per-kind defaults inherit from this only at the dispatch
/// layer's discretion — the global row is informational.
pub async fn set_global_acknowledged(
    pool: &Pool,
    user_id: &str,
    email_opt_in: bool,
) -> StorageResult<()> {
    let channels = if email_opt_in {
        "in_app,email"
    } else {
        "in_app"
    };
    sqlx::query(
        r#"
        INSERT INTO notification_preferences (user_id, kind, channels, min_severity)
        VALUES (?1, ?2, ?3, 'info')
        ON CONFLICT(user_id, kind) DO UPDATE SET
            channels = excluded.channels
        "#,
    )
    .bind(user_id)
    .bind(peisear_core::notifications::kind::GLOBAL)
    .bind(channels)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find the global preference row to know whether email is
/// globally opted in. If absent, returns `None` (no default
/// channel application; call `global_acknowledged` first to
/// decide whether to prompt).
pub async fn global_preference(pool: &Pool, user_id: &str) -> StorageResult<Option<Preference>> {
    preference_for_user_kind(pool, user_id, peisear_core::notifications::kind::GLOBAL).await
}
