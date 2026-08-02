//! Per-user personal metrics queries.
//!
//! Sibling to [`crate::project_health`]. The module shape is
//! deliberately the same so a future `user_burnout` module fits
//! alongside without a refactor.
//!
//! ## Phase 2 (0.8.0): event-aware where possible
//!
//! `long_stale_count` and `estimation_skew_days_per_point` were
//! coarse calendar-time approximations in 0.7.0. 0.8.0 uses event
//! data when present (i.e., transitions that happened from this
//! release onward) and falls back to the 0.7.0 approximation when
//! no events exist for an issue (i.e., legacy data).
//!
//! - `long_stale_count`: "in-flight issues whose status hasn't
//!   moved in N days". Event-based: latest `status_changed`
//!   `occurred_at` per issue. Fallback: `updated_at`.
//! - `estimation_skew`: Event-based — the helper
//!   [`active_estimation_skew`] sums in_progress dwell time per
//!   recently-done issue and divides by effort. Fallback: average
//!   `(updated_at - created_at) / effort`. The active-time number
//!   is the more honest reflection signal.
//!
//! ## Privacy
//!
//! These queries return data for a single user at a time. The web
//! layer must enforce that the requesting user is the same as the
//! `user_id` parameter — see `peisear-web::handlers::me` for the
//! check. Future manager / neutral-third-party roles will lift
//! that constraint with explicit permission checks.

use peisear_core::personal_metrics::{
    DEFAULT_WIP_LIMIT, PERSONAL_ACTIVITY_WINDOW_DAYS, PersonalMetrics,
};

use crate::{Pool, StorageResult};

/// Compute the event-aware estimation skew for a user across the
/// given scope. Walks the user's recently-done issues, computes
/// per-issue in_progress dwell time from event log when at least
/// one `status_changed` event exists, otherwise falls back to
/// calendar time `(updated_at - created_at)`. Averages
/// `dwell_seconds / (effort * 86400)` to get days-per-point.
///
/// Returns `None` when no recently-done estimated issues exist.
async fn active_estimation_skew(
    pool: &Pool,
    user_id: &str,
    project_filter: Option<&str>,
    window_days: i64,
) -> StorageResult<Option<f64>> {
    // Collect (issue_id, effort, created_at, updated_at) for
    // recently-done estimated issues. We then walk events per
    // issue in Rust because reconstructing dwell time across
    // multiple windows in pure SQL would be hard to reason about.
    let project_clause = if project_filter.is_some() {
        "AND project_id = ?3"
    } else {
        ""
    };
    let sql = format!(
        r#"
        SELECT id, effort, created_at, updated_at
        FROM issues
        WHERE assignee_id = ?1
          AND status = 'done'
          AND effort IS NOT NULL
          AND effort > 0
          AND updated_at >= datetime('now', ?2)
          {project_clause}
        "#
    );
    let mut q = sqlx::query_as::<_, (String, i64, String, String)>(&sql)
        .bind(user_id)
        .bind(format!("-{} days", window_days));
    if let Some(p) = project_filter {
        q = q.bind(p);
    }
    let rows: Vec<(String, i64, String, String)> = q.fetch_all(pool).await?;

    if rows.is_empty() {
        return Ok(None);
    }

    let mut total_days_per_point = 0.0_f64;
    let mut count = 0_u32;

    for (issue_id, effort, created_at, updated_at) in rows {
        // Try event-based dwell time first.
        let dwell = crate::issue_events::in_progress_seconds_for_issue(pool, &issue_id)
            .await?
            // Convert seconds to days.
            .map(|secs| secs / 86_400.0);

        let days = match dwell {
            Some(d) if d > 0.0 => d,
            // Fallback to calendar-time approximation. This
            // matches the 0.7.0 semantics exactly, so legacy
            // issues continue to contribute the same number to
            // the average.
            _ => calendar_time_days(&created_at, &updated_at).unwrap_or(0.0),
        };

        if days > 0.0 {
            total_days_per_point += days / effort as f64;
            count += 1;
        }
    }

    if count == 0 {
        return Ok(None);
    }
    Ok(Some(total_days_per_point / count as f64))
}

/// Calendar time between created_at and updated_at, in days.
/// Returns `None` if the strings can't be parsed.
fn calendar_time_days(created_at: &str, updated_at: &str) -> Option<f64> {
    let parse = |s: &str| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok();
    let start = parse(created_at)?;
    let end = parse(updated_at)?;
    let secs = (end - start).num_seconds().max(0) as f64;
    Some(secs / 86_400.0)
}

/// Compute personal metrics for `user_id` scoped to `project_id`.
///
/// Returns `None` if the user does not exist.
///
/// **0.12.0**: capacity is resolved through
/// [`crate::user_capacities::effective_for_user`] (period-aware)
/// rather than read from a static `users.capacity_points` field.
pub async fn for_user_in_project(
    pool: &Pool,
    user_id: &str,
    project_id: &str,
) -> StorageResult<Option<PersonalMetrics>> {
    let user_row: Option<(String, Option<i64>)> = sqlx::query_as(
        r#"
        SELECT u.display_name, u.wip_limit
        FROM users u
        WHERE u.id = ?1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let Some((display_name, user_wip_limit)) = user_row else {
        return Ok(None);
    };

    // Today's effective capacity from user_capacities. None if the
    // user has no row covering today (i.e., no capacity set).
    let capacity_points = crate::user_capacities::effective_for_user(pool, user_id).await?;

    let project_default: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT wip_limit_default FROM projects WHERE id = ?1
        "#,
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?
    .flatten();

    let effective_wip_limit = user_wip_limit
        .or(project_default)
        .unwrap_or(DEFAULT_WIP_LIMIT);

    // Aggregate over this user's issues in this project.
    //
    // long_stale_count uses the COALESCE(event_time, updated_at)
    // pattern so legacy issues are treated as 0.7.0 did.
    let row: (i64, Option<i64>, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            SUM(CASE WHEN status = 'in_progress' THEN 1 ELSE 0 END)
                AS current_wip,
            SUM(CASE
                WHEN status IN ('open', 'in_progress') THEN effort
                ELSE NULL
            END) AS in_flight_points,
            SUM(CASE
                WHEN status = 'done' AND updated_at >= datetime('now', ?3)
                THEN 1 ELSE 0
            END) AS recent_done_count,
            SUM(CASE
                WHEN status IN ('open', 'in_progress')
                  AND julianday('now') - julianday(
                      COALESCE(
                          (SELECT MAX(e.occurred_at)
                           FROM issue_events e
                           WHERE e.issue_id = issues.id
                             AND e.event_type = 'status_changed'),
                          updated_at
                      )
                  ) >= ?4
                THEN 1 ELSE 0
            END) AS long_stale_count
        FROM issues
        WHERE assignee_id = ?1 AND project_id = ?2
        "#,
    )
    .bind(user_id)
    .bind(project_id)
    .bind(format!("-{} days", PERSONAL_ACTIVITY_WINDOW_DAYS))
    .bind(PERSONAL_ACTIVITY_WINDOW_DAYS)
    .fetch_one(pool)
    .await?;

    let (current_wip, in_flight_points, recent_done_count, long_stale_count) = row;

    // Event-aware estimation skew. Window is 4× the activity
    // window (i.e., ~8 weeks) so we have enough completed issues
    // to fit a meaningful average.
    let skew = active_estimation_skew(
        pool,
        user_id,
        Some(project_id),
        PERSONAL_ACTIVITY_WINDOW_DAYS * 4,
    )
    .await?;

    Ok(Some(PersonalMetrics {
        user_id: user_id.to_string(),
        display_name,
        effective_wip_limit,
        current_wip,
        in_flight_points: in_flight_points.unwrap_or(0),
        capacity_points,
        recent_done_count,
        long_stale_count,
        estimation_skew_days_per_point: skew,
    }))
}

/// Compute personal metrics for `user_id` aggregated across all
/// projects they are part of (today: own).
///
/// The `/me` page uses this to give a global view rather than one
/// scoped to a single project. Returns `None` if the user does
/// not exist.
pub async fn for_user_global(pool: &Pool, user_id: &str) -> StorageResult<Option<PersonalMetrics>> {
    let user_row: Option<(String, Option<i64>)> = sqlx::query_as(
        r#"
        SELECT u.display_name, u.wip_limit
        FROM users u
        WHERE u.id = ?1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let Some((display_name, user_wip_limit)) = user_row else {
        return Ok(None);
    };

    // Today's effective capacity. See `for_user_in_project` above.
    let capacity_points = crate::user_capacities::effective_for_user(pool, user_id).await?;

    // Global WIP limit ignores per-project defaults; per-project
    // defaults don't compose into a global cap.
    let effective_wip_limit = user_wip_limit.unwrap_or(DEFAULT_WIP_LIMIT);

    let row: (i64, Option<i64>, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            SUM(CASE WHEN status = 'in_progress' THEN 1 ELSE 0 END)
                AS current_wip,
            SUM(CASE
                WHEN status IN ('open', 'in_progress') THEN effort
                ELSE NULL
            END) AS in_flight_points,
            SUM(CASE
                WHEN status = 'done' AND updated_at >= datetime('now', ?2)
                THEN 1 ELSE 0
            END) AS recent_done_count,
            SUM(CASE
                WHEN status IN ('open', 'in_progress')
                  AND julianday('now') - julianday(
                      COALESCE(
                          (SELECT MAX(e.occurred_at)
                           FROM issue_events e
                           WHERE e.issue_id = issues.id
                             AND e.event_type = 'status_changed'),
                          updated_at
                      )
                  ) >= ?3
                THEN 1 ELSE 0
            END) AS long_stale_count
        FROM issues
        WHERE assignee_id = ?1
        "#,
    )
    .bind(user_id)
    .bind(format!("-{} days", PERSONAL_ACTIVITY_WINDOW_DAYS))
    .bind(PERSONAL_ACTIVITY_WINDOW_DAYS)
    .fetch_one(pool)
    .await?;

    let (current_wip, in_flight_points, recent_done_count, long_stale_count) = row;

    let skew =
        active_estimation_skew(pool, user_id, None, PERSONAL_ACTIVITY_WINDOW_DAYS * 4).await?;

    Ok(Some(PersonalMetrics {
        user_id: user_id.to_string(),
        display_name,
        effective_wip_limit,
        current_wip,
        in_flight_points: in_flight_points.unwrap_or(0),
        capacity_points,
        recent_done_count,
        long_stale_count,
        estimation_skew_days_per_point: skew,
    }))
}
