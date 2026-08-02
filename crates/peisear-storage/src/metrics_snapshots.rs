//! Persisted history of project-level health metrics.
//!
//! A small, append-only table written by the background task in
//! `peisear`. The query layer here exposes:
//!
//! - [`insert`] for the writer (background task).
//! - [`recent_for_project`] for the trend reader (project detail
//!   page handler).
//! - [`projects_with_recent_issue_activity`] for the writer to
//!   pick which projects to snapshot.
//!
//! See `migrations/0007_metrics_snapshots.sql` for schema rationale.
//!
//! ## Future direction
//!
//! When per-user burnout indicators land (planned 0.10.0), this
//! table is *not* extended with per-user columns — a separate
//! `user_metrics_snapshots` table is the natural shape. The two
//! kinds of snapshot have different privacy boundaries (project
//! aggregates are visible to project members; per-user snapshots
//! are visible only to the user themselves and, eventually, to
//! managers / neutral observers). Keeping them in separate tables
//! keeps the access-control story clear.

use peisear_core::project_health::ProjectHealthRaw;
use uuid::Uuid;

use crate::{Pool, StorageResult};

/// One snapshot row, deserialised from storage. Same shape as
/// what the writer puts in.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub raw: ProjectHealthRaw,
    pub score_value: u8,
    pub captured_at: chrono::DateTime<chrono::Utc>,
}

/// Insert one snapshot row. Called by the background task.
pub async fn insert(
    pool: &Pool,
    project_id: &str,
    raw: &ProjectHealthRaw,
    score_value: u8,
) -> StorageResult<()> {
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO metrics_snapshots (
            id, project_id,
            total_issues, done_issues, oldest_in_flight_age_days,
            recent_activity_count, in_flight_issues,
            top_assignee_in_flight_issues, long_stale_in_flight_issues,
            wip_violators, active_assignees,
            score_value
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(raw.total_issues)
    .bind(raw.done_issues)
    .bind(raw.oldest_in_flight_age_days)
    .bind(raw.recent_activity_count)
    .bind(raw.in_flight_issues)
    .bind(raw.top_assignee_in_flight_issues)
    .bind(raw.long_stale_in_flight_issues)
    .bind(raw.wip_violators)
    .bind(raw.active_assignees)
    .bind(score_value as i64)
    .execute(pool)
    .await?;
    Ok(())
}

/// Snapshots for `project_id` whose `captured_at` falls between
/// `min_days_ago` and `max_days_ago` before now (i.e., older than
/// `min_days_ago` and newer than `max_days_ago`). Ordered oldest →
/// newest.
///
/// The two-sided bound deliberately *excludes* the very-recent
/// past (`captured_at` ≥ now - min): if today's snapshot leaks
/// into the past baseline, the trend collapses to "current vs.
/// current" and never moves. The 7-14 day window gives a stable
/// median that reflects "how things were a week or so ago".
pub async fn recent_for_project(
    pool: &Pool,
    project_id: &str,
    min_days_ago: i64,
    max_days_ago: i64,
) -> StorageResult<Vec<Snapshot>> {
    let rows: Vec<(
        i64,
        i64,
        Option<i64>,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        chrono::DateTime<chrono::Utc>,
    )> = sqlx::query_as(
        r#"
        SELECT
            total_issues, done_issues, oldest_in_flight_age_days,
            recent_activity_count, in_flight_issues,
            top_assignee_in_flight_issues, long_stale_in_flight_issues,
            wip_violators, active_assignees,
            score_value, captured_at
        FROM metrics_snapshots
        WHERE project_id = ?1
          AND captured_at <= datetime('now', ?2)
          AND captured_at >= datetime('now', ?3)
        ORDER BY captured_at ASC
        "#,
    )
    .bind(project_id)
    .bind(format!("-{} days", min_days_ago))
    .bind(format!("-{} days", max_days_ago))
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Snapshot {
            raw: ProjectHealthRaw {
                total_issues: r.0,
                done_issues: r.1,
                oldest_in_flight_age_days: r.2,
                recent_activity_count: r.3,
                in_flight_issues: r.4,
                top_assignee_in_flight_issues: r.5,
                long_stale_in_flight_issues: r.6,
                wip_violators: r.7,
                active_assignees: r.8,
            },
            score_value: r.9.clamp(0, 100) as u8,
            captured_at: r.10,
        })
        .collect())
}

/// Project IDs with at least one issue ever, used by the
/// background task to decide where to write a snapshot. Empty
/// projects don't produce signal so we don't capture them — the
/// HealthStrip already hides itself for empty projects.
///
/// "Activity" here is loose on purpose: any issue means a project
/// is alive enough to track. A more aggressive filter (e.g.
/// "had a status_changed event in the past 30 days") could spare
/// some database writes but at the cost of incomplete trend lines
/// for projects that go quiet for a while. Keep it simple.
pub async fn projects_with_recent_issue_activity(pool: &Pool) -> StorageResult<Vec<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT DISTINCT p.id
        FROM projects p
        WHERE EXISTS (
            SELECT 1 FROM issues i WHERE i.project_id = p.id
        )
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}
