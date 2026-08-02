//! Per-user fatigue / burnout signals. Sibling to
//! [`crate::personal_metrics`].
//!
//! ## Scope
//!
//! Four indicators ship across 0.10.x and 0.11.x:
//!
//! - **Sustained-overload streak** — consecutive snapshots the
//!   user was over their capacity. Read from
//!   [`crate::user_metrics_snapshots`]. *Shipped 0.10.0.*
//! - **Stalled-assigned streak** — for the user's oldest in-flight
//!   issue, the days since its last status_changed event. Read
//!   from [`crate::issue_events`]; falls back to `updated_at` for
//!   pre-0.8.0 issues, same pattern as [`crate::project_health`].
//!   *Shipped 0.10.0.*
//! - **Estimation drift trend** — median dwell-time-per-point
//!   over the recent two weeks vs. the prior two weeks. Surfaces
//!   "recent issues are taking longer per point than older ones"
//!   as a directional fact, with no warning palette. *Shipped
//!   0.11.0.*
//! - **Cognitive switching** — median count of
//!   `status_changed -> in_progress` events per active day,
//!   surfaced as a rhythm number. No threshold, no warning.
//!   *Shipped 0.11.0.*
//!
//! ## Privacy posture
//!
//! All queries here take a `user_id` and return that user's
//! signals only. The web layer (`peisear-web::handlers::me`)
//! enforces that the requesting session matches that user. The
//! manager / neutral-third-party scopes the V2.1 brief calls for
//! arrive with the planned Team feature; until then the data is
//! self-only.

use peisear_core::user_burnout::{
    CognitiveSwitchingPattern, DRIFT_STEADY_THRESHOLD_RATIO, DRIFT_WINDOW_DAYS, DriftDirection,
    EstimationDriftTrend, SWITCHING_MIN_EVENTS, SWITCHING_WINDOW_DAYS, UserBurnoutSignals,
};

use crate::{Pool, StorageResult, user_metrics_snapshots};

/// Window over which the overload-streak is counted. Two weeks is
/// long enough to distinguish "had a busy week" from a
/// genuinely-sustained pattern that warrants attention. Same
/// window used elsewhere in the personal-metrics module for
/// symmetry (V2.1 §1.2 calls for self-reflection support, not
/// shorter alarming).
const STREAK_WINDOW_DAYS: i64 = 14;

/// Compute the per-user burnout signals from snapshots and events.
///
/// Returns `None` if the user does not exist (matches
/// [`crate::personal_metrics`] semantics). Returns an empty signal
/// set (zero streaks, no warnings) when the user has no history
/// — this is correct for fresh users / empty backups.
pub async fn for_user(pool: &Pool, user_id: &str) -> StorageResult<Option<UserBurnoutSignals>> {
    // Confirm the user exists (defensive — callers usually have
    // already done so via auth, but it's cheap to check).
    let exists: Option<(String,)> = sqlx::query_as(r#"SELECT id FROM users WHERE id = ?1"#)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Ok(None);
    }

    let overload_streak = consecutive_overload_days(pool, user_id).await?;
    let stalled_streak = oldest_assigned_stalled_days(pool, user_id).await?;
    let estimation_drift = estimation_drift_for_user(pool, user_id).await?;
    let cognitive_switching = cognitive_switching_for_user(pool, user_id).await?;

    Ok(Some(UserBurnoutSignals {
        overload_streak_days: overload_streak,
        stalled_assigned_max_days: stalled_streak,
        window_days: STREAK_WINDOW_DAYS,
        estimation_drift,
        cognitive_switching,
    }))
}

/// Length of the most recent consecutive-overload streak: number
/// of newest-first snapshots where `over_capacity = 1` until the
/// first `over_capacity = 0` (or until the window runs out).
///
/// The "consecutive" property is what makes this a streak rather
/// than a count: 5 separate days over capacity within two weeks
/// may be normal life; 5 days in a row is a pattern.
async fn consecutive_overload_days(pool: &Pool, user_id: &str) -> StorageResult<i64> {
    let snaps = user_metrics_snapshots::recent_for_user(pool, user_id, STREAK_WINDOW_DAYS).await?;
    if snaps.is_empty() {
        return Ok(0);
    }
    // Walk newest-first, count the leading run of over_capacity = true.
    let mut streak = 0_i64;
    for s in snaps.iter().rev() {
        if s.over_capacity {
            streak += 1;
        } else {
            break;
        }
    }
    Ok(streak)
}

/// Days since the most-recent status_changed event for the user's
/// oldest in-flight assigned issue. `0` if the user has no
/// in-flight assignments.
///
/// Uses `COALESCE(latest event time, updated_at)` so legacy
/// (pre-0.8.0) issues don't disappear from the query — same
/// fallback pattern as `project_health`.
async fn oldest_assigned_stalled_days(pool: &Pool, user_id: &str) -> StorageResult<i64> {
    let row: Option<(Option<f64>,)> = sqlx::query_as(
        r#"
        SELECT MAX(
            julianday('now') - julianday(
                COALESCE(
                    (SELECT MAX(e.occurred_at)
                     FROM issue_events e
                     WHERE e.issue_id = i.id
                       AND e.event_type = 'status_changed'),
                    i.updated_at
                )
            )
        )
        FROM issues i
        WHERE i.assignee_id = ?1
          AND i.status IN ('open', 'in_progress')
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row
        .and_then(|(d,)| d)
        .map(|d| d.floor().max(0.0) as i64)
        .unwrap_or(0))
}

/// Estimation drift trend over [`DRIFT_WINDOW_DAYS`] days, split
/// into a recent half (newer) and an older half (earlier).
///
/// "Drift" here is `(recent_median - older_median) / older_median`.
/// The classification into Up / Down / Steady uses
/// [`DRIFT_STEADY_THRESHOLD_RATIO`].
///
/// Returns `None` when either half has zero qualifying issues —
/// we can't compute a comparison without two endpoints. The UI
/// hides the chip in that case.
///
/// ## Algorithm
///
/// For each `done` issue with `effort > 0` whose update fell into
/// the window, compute the same per-issue dwell-time-per-point
/// number that `personal_metrics::active_estimation_skew` uses.
/// Bucket each into recent / older half by the *event time* of
/// its `done` transition (or, for legacy issues with no events,
/// by `updated_at`). Take the median per half, divide.
///
/// We use the median rather than the mean for symmetry with the
/// project trend (`peisear-core::project_health::classify_trend`)
/// — outlier issues do not warp the comparison.
async fn estimation_drift_for_user(
    pool: &Pool,
    user_id: &str,
) -> StorageResult<Option<EstimationDriftTrend>> {
    // Pull each candidate issue's id, effort, and the closing
    // timestamp. We compute dwell time per issue using the same
    // event-walking helper that personal_metrics uses, then bucket
    // by the closing timestamp.
    //
    // The closing timestamp is the most recent status_changed
    // event whose new_value is 'done', or `updated_at` as a
    // fallback. The COALESCE keeps legacy issues in the query
    // rather than dropping them.
    let rows: Vec<(String, i64, String)> = sqlx::query_as(
        r#"
        SELECT
            i.id,
            i.effort,
            COALESCE(
                (SELECT MAX(e.occurred_at)
                 FROM issue_events e
                 WHERE e.issue_id = i.id
                   AND e.event_type = 'status_changed'
                   AND e.new_value = 'done'),
                i.updated_at
            ) AS closed_at
        FROM issues i
        WHERE i.assignee_id = ?1
          AND i.status = 'done'
          AND i.effort IS NOT NULL
          AND i.effort > 0
          AND i.updated_at >= datetime('now', ?2)
        "#,
    )
    .bind(user_id)
    .bind(format!("-{} days", DRIFT_WINDOW_DAYS))
    .fetch_all(pool)
    .await?;

    if rows.is_empty() {
        return Ok(None);
    }

    let half_window = DRIFT_WINDOW_DAYS / 2;
    let cutoff = chrono::Utc::now().naive_utc() - chrono::Duration::days(half_window);

    let mut recent_values: Vec<f64> = Vec::new();
    let mut older_values: Vec<f64> = Vec::new();

    for (issue_id, effort, closed_at_str) in rows {
        // Days-per-point from the event log; falls back to
        // calendar approximation for legacy issues.
        let dwell_seconds =
            crate::issue_events::in_progress_seconds_for_issue(pool, &issue_id).await?;
        let dpp = match dwell_seconds {
            Some(s) if s > 0.0 => s / 86_400.0 / effort as f64,
            _ => continue, // legacy issue without enough data; skip rather than guess
        };

        // Bucket by closed_at.
        let closed_at =
            chrono::NaiveDateTime::parse_from_str(&closed_at_str, "%Y-%m-%d %H:%M:%S").ok();
        let Some(closed_at) = closed_at else { continue };

        if closed_at >= cutoff {
            recent_values.push(dpp);
        } else {
            older_values.push(dpp);
        }
    }

    // Need both halves to compute a comparison.
    if recent_values.is_empty() || older_values.is_empty() {
        return Ok(None);
    }

    let recent_median = median(&mut recent_values);
    let older_median = median(&mut older_values);

    // Direction: if older is essentially zero (shouldn't happen
    // since we filter dpp > 0 above, but defensive), call it Steady.
    let direction = if older_median <= 0.0 {
        DriftDirection::Steady
    } else {
        let ratio = (recent_median - older_median) / older_median;
        if ratio.abs() < DRIFT_STEADY_THRESHOLD_RATIO {
            DriftDirection::Steady
        } else if ratio > 0.0 {
            DriftDirection::Up
        } else {
            DriftDirection::Down
        }
    };

    Ok(Some(EstimationDriftTrend {
        recent_median_days_per_point: recent_median,
        older_median_days_per_point: older_median,
        direction,
        window_days: DRIFT_WINDOW_DAYS,
    }))
}

/// Median of a list of f64s. Mutates the input (sorts in place).
/// We accept the mutation rather than clone because both call
/// sites here already own the vec.
fn median(xs: &mut [f64]) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = xs.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    }
}

/// Cognitive switching: median count of `-> in_progress` events
/// per active day for this user, over [`SWITCHING_WINDOW_DAYS`]
/// days.
///
/// Returns `None` when the total event count over the window is
/// below [`SWITCHING_MIN_EVENTS`] — a handful of pickups across
/// two weeks doesn't characterise a "rhythm", and reporting the
/// number anyway would be more noise than signal.
async fn cognitive_switching_for_user(
    pool: &Pool,
    user_id: &str,
) -> StorageResult<Option<CognitiveSwitchingPattern>> {
    // For each issue currently or formerly assigned to this user,
    // every event whose new_value transitions to in_progress
    // counts as one "pickup". Group by date.
    //
    // We take the actor as the assignee proxy: the user-burnout
    // module is about *the user*'s rhythm, and switches are
    // attributed to whoever moved the issue. The actor field on
    // events is exactly that.
    let rows: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT
            DATE(e.occurred_at) AS day,
            COUNT(*) AS switches_on_day
        FROM issue_events e
        WHERE e.actor_id = ?1
          AND e.event_type = 'status_changed'
          AND e.new_value = 'in_progress'
          AND e.occurred_at >= datetime('now', ?2)
        GROUP BY day
        ORDER BY day ASC
        "#,
    )
    .bind(user_id)
    .bind(format!("-{} days", SWITCHING_WINDOW_DAYS))
    .fetch_all(pool)
    .await?;

    let total_events: i64 = rows.iter().map(|(_, n)| n).sum();
    if total_events < SWITCHING_MIN_EVENTS {
        return Ok(None);
    }

    // Median over active days only — quiet days don't dilute.
    // A user who works in bursts (3 days on, 4 days off) should
    // see their rhythm represented as the active-day median, not
    // as the calendar-week mean.
    let mut counts: Vec<f64> = rows.into_iter().map(|(_, n)| n as f64).collect();
    let median_per_day = median(&mut counts);

    Ok(Some(CognitiveSwitchingPattern {
        switches_per_day_median: median_per_day,
        total_events_observed: total_events,
        window_days: SWITCHING_WINDOW_DAYS,
    }))
}
