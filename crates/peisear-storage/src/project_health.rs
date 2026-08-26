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
/// `HLT-001` (RFC 008 §1) rewrote this from pure-SQL aggregation
/// (`SUM`/`MAX` producing counts alone) to a **fetch plus a fold**:
/// the in-flight and recent-activity queries below select the
/// *rows* an indicator's count is about, and the counts, the oldest
/// issue, and the long-stale subset are all derived from those same
/// fetched rows in Rust. This is what lets [`ProjectHealthRaw`]
/// carry membership (which issues, not just how many) from the
/// *same* evaluation as the count — a second query re-running the
/// same `WHERE` would be two homes for one fact and could disagree
/// with the count it's supposed to explain.
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
    // In-flight issues, one row each: id, assignee, age since
    // creation (staleness's clock), age since last meaningful
    // touch (long-stale's event-aware clock). Folded below into
    // every in-flight-derived count *and* its membership, from
    // this one evaluation.
    let in_flight_rows: Vec<(String, Option<String>, f64, f64)> = sqlx::query_as(
        r#"
        SELECT
            id,
            assignee_id,
            julianday('now') - julianday(created_at) AS age_days,
            julianday('now') - julianday(
                COALESCE(
                    (SELECT MAX(e.occurred_at)
                     FROM issue_events e
                     WHERE e.issue_id = issues.id
                       AND e.event_type = 'status_changed'),
                    updated_at
                )
            ) AS staleness_days
        FROM issues
        WHERE project_id = ?1
          AND status IN ('open', 'in_progress')
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    let in_flight_issues = in_flight_rows.len() as i64;
    let in_flight_issue_ids: Vec<String> =
        in_flight_rows.iter().map(|(id, ..)| id.clone()).collect();

    let mut oldest_in_flight_age_days: Option<i64> = None;
    let mut oldest_in_flight_issue_id: Option<String> = None;
    let mut long_stale_issue_ids: Vec<String> = Vec::new();
    let mut per_assignee_counts: std::collections::HashMap<&str, i64> =
        std::collections::HashMap::new();

    for (id, assignee_id, age_days, staleness_days) in &in_flight_rows {
        let age_floor = age_days.floor() as i64;
        if oldest_in_flight_age_days.is_none_or(|max| age_floor > max) {
            oldest_in_flight_age_days = Some(age_floor);
            oldest_in_flight_issue_id = Some(id.clone());
        }
        if *staleness_days >= LONG_STALE_THRESHOLD_DAYS as f64 {
            long_stale_issue_ids.push(id.clone());
        }
        if let Some(a) = assignee_id.as_deref() {
            *per_assignee_counts.entry(a).or_insert(0) += 1;
        }
    }
    let long_stale_in_flight_issues = long_stale_issue_ids.len() as i64;
    let top_assignee_in_flight_issues = per_assignee_counts.values().copied().max().unwrap_or(0);

    // Done issues: the membership behind `done_issues` and
    // throughput's numerator. `total_issues` is derived below from
    // this plus `in_flight_issues` rather than a third `COUNT(*)` —
    // `IssueStatus` is exhaustively open/in_progress/done, so every
    // issue is in exactly one of the two sets already fetched.
    let done_issue_ids: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT id FROM issues
        WHERE project_id = ?1 AND status = 'done'
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    let done_issues = done_issue_ids.len() as i64;
    let total_issues = in_flight_issues + done_issues;

    // Recent activity: created in the window, or moved to done
    // within it. Not a subset of either set above (a done issue
    // updated recently is in `done_issue_ids` but may or may not be
    // "recent"; a freshly-created in-flight issue is in
    // `in_flight_issue_ids` but only "recent" if young enough) —
    // its own fetch, matching the original `OR` condition exactly.
    let recent_activity_issue_ids: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT id FROM issues
        WHERE project_id = ?1
          AND (
              created_at >= datetime('now', ?2)
              OR (status = 'done' AND updated_at >= datetime('now', ?2))
          )
        "#,
    )
    .bind(project_id)
    .bind(format!("-{} days", ACTIVITY_WINDOW_DAYS))
    .fetch_all(pool)
    .await?;
    let recent_activity_count = recent_activity_issue_ids.len() as i64;

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
        oldest_in_flight_age_days,
        recent_activity_count,
        in_flight_issues,
        top_assignee_in_flight_issues,
        long_stale_in_flight_issues,
        wip_violators,
        active_assignees,
        done_issue_ids,
        oldest_in_flight_issue_id,
        in_flight_issue_ids,
        recent_activity_issue_ids,
        long_stale_issue_ids,
    })
}
