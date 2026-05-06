//! Project-level health aggregations.
//!
//! Read-only queries that compute the inputs to
//! [`peisear_core::project_health::ProjectHealthRaw`]. No new tables —
//! everything is aggregated from `issues`, `users`, and `projects`.
//!
//! ## Phase 1 (0.7.0) limitations
//!
//! These queries derive everything from `issues.created_at`,
//! `issues.updated_at`, and `issues.status`. That means:
//!
//! - "long-stale" detection uses `updated_at`, which is overwritten
//!   every time the row changes (priority bump, etc.) — it's a
//!   close-but-imperfect proxy for "last meaningful change".
//! - "Throughput" counts `status = 'done'` rows but cannot tell us
//!   *when* they reached done; recent_activity uses `updated_at`
//!   as a proxy for the done-transition time.
//! - There is no per-status-segment time accounting, so cycle time
//!   ("how long was this issue actively in progress?") is not
//!   computable.
//!
//! Phase 2 plans an `issue_events` table that records each
//! transition, lifting these limitations. The function signatures
//! here are stable across that change — only the SQL bodies move.
//!
//! ## Future direction: user_burnout
//!
//! A planned sibling module will compute per-user fatigue / burnout
//! signals (sustained overload, stalled assigned work, unbalanced
//! load distribution). It will live as `user_burnout` in this same
//! crate, share the [`peisear_core::HealthIndicator`] palette, and
//! reuse the same activity-window concept defined in
//! `peisear_core::project_health::ACTIVITY_WINDOW_DAYS`.

use peisear_core::project_health::{
    ACTIVITY_WINDOW_DAYS, LONG_STALE_THRESHOLD_DAYS, ProjectHealthRaw,
};

use crate::{Pool, StorageResult};

/// Compute the raw health snapshot for a project.
///
/// One round-trip with conditional `SUM`s plus a follow-up small
/// query for the assignee-distribution numbers. SQLite's
/// `julianday()` lets us subtract dates as fractional days.
///
/// ## Phase 2 (0.8.0): event-aware long-stale detection
///
/// Long-stale uses the latest `status_changed` event's
/// `occurred_at` when one exists, falling back to `updated_at`
/// otherwise. This means:
///
/// - For issues created or transitioned in 0.8.0+, "long-stale"
///   tracks status-only changes; priority bumps and other edits
///   no longer reset the staleness clock.
/// - For pre-0.8.0 issues with no event log, behaviour is
///   identical to 0.7.0 (the documented limitation).
pub async fn for_project(pool: &Pool, project_id: &str) -> StorageResult<ProjectHealthRaw> {
    // Aggregate over the issues table.
    //
    // The long_stale_in_flight_issues subquery uses a correlated
    // SELECT to find the most recent status_changed event for each
    // in-flight issue. COALESCE picks the event timestamp if one
    // exists; otherwise it falls back to updated_at (Phase 1
    // behaviour for legacy issues).
    let row: (i64, i64, Option<f64>, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COUNT(*) AS total_issues,
            SUM(CASE WHEN status = 'done' THEN 1 ELSE 0 END) AS done_issues,
            MAX(CASE
                WHEN status IN ('open', 'in_progress')
                THEN julianday('now') - julianday(created_at)
                ELSE NULL
            END) AS oldest_in_flight_days,
            SUM(CASE
                WHEN created_at >= datetime('now', ?2)
                  OR (status = 'done' AND updated_at >= datetime('now', ?2))
                THEN 1 ELSE 0
            END) AS recent_activity_count,
            SUM(CASE
                WHEN status IN ('open', 'in_progress') THEN 1 ELSE 0
            END) AS in_flight_issues,
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
            END) AS long_stale_in_flight_issues
        FROM issues
        WHERE project_id = ?1
        "#,
    )
    .bind(project_id)
    .bind(format!("-{} days", ACTIVITY_WINDOW_DAYS))
    .bind(LONG_STALE_THRESHOLD_DAYS)
    .fetch_one(pool)
    .await?;

    let (
        total_issues,
        done_issues,
        oldest_days,
        recent_activity_count,
        in_flight_issues,
        long_stale_in_flight_issues,
    ) = row;

    // Top-assignee concentration: how many in-flight issues sit on
    // the single most-loaded user.
    let top_assignee_in_flight_issues: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(MAX(per_user_count), 0) FROM (
            SELECT COUNT(*) AS per_user_count
            FROM issues
            WHERE project_id = ?1
              AND status IN ('open', 'in_progress')
              AND assignee_id IS NOT NULL
            GROUP BY assignee_id
        )
        "#,
    )
    .bind(project_id)
    .fetch_one(pool)
    .await?;

    let active_assignees: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT assignee_id)
        FROM issues
        WHERE project_id = ?1
          AND status IN ('open', 'in_progress')
          AND assignee_id IS NOT NULL
        "#,
    )
    .bind(project_id)
    .fetch_one(pool)
    .await?;

    // WIP violators: count of distinct users whose current
    // in_progress count exceeds their effective WIP limit.
    //
    // The "effective limit" is resolved per-user inside the SQL:
    // COALESCE(user.wip_limit, project.wip_limit_default,
    // DEFAULT_WIP_LIMIT). The system default is read from core
    // and bound as a parameter so the SQL stays in sync if the
    // const changes.
    let wip_violators: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM (
            SELECT
                u.id,
                COALESCE(u.wip_limit, p.wip_limit_default, ?2) AS effective_limit,
                SUM(CASE WHEN i.status = 'in_progress' THEN 1 ELSE 0 END)
                    AS current_wip
            FROM users u
            JOIN projects p ON p.id = ?1
            LEFT JOIN issues i
                ON i.assignee_id = u.id
               AND i.project_id = ?1
            GROUP BY u.id, effective_limit
            HAVING current_wip > effective_limit
        )
        "#,
    )
    .bind(project_id)
    .bind(peisear_core::personal_metrics::DEFAULT_WIP_LIMIT)
    .fetch_one(pool)
    .await?;

    Ok(ProjectHealthRaw {
        total_issues,
        done_issues,
        oldest_in_flight_age_days: oldest_days.map(|d| d.floor() as i64),
        recent_activity_count,
        in_flight_issues,
        top_assignee_in_flight_issues,
        long_stale_in_flight_issues,
        wip_violators,
        active_assignees,
    })
}
