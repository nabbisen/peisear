//! Persisted history of per-user personal-load metrics.
//!
//! Sibling to [`crate::metrics_snapshots`]. The two tables exist
//! separately because their privacy boundaries differ — see the
//! rationale comment in `migrations/0008_user_metrics_snapshots.sql`.
//!
//! Read API today exposes streak-counting queries used by
//! [`crate::user_burnout`]. Write API is one function called from
//! the background job for each user with active assigned work.

use sqlx::FromRow;
use uuid::Uuid;

use crate::{Pool, StorageResult};

/// Compact view of one snapshot row, used by streak / trend
/// queries. Mirrors the table columns minus storage metadata.
#[derive(Debug, Clone)]
pub struct UserSnapshot {
    pub user_id: String,
    pub current_wip: i64,
    pub in_flight_points: i64,
    pub capacity_points: Option<i64>,
    pub over_capacity: bool,
    pub effective_wip_limit: i64,
    pub over_wip_limit: bool,
    pub captured_at: chrono::DateTime<chrono::Utc>,
}

/// The measured values for one user-metrics snapshot. `user_id`
/// is kept as [`insert`]'s own argument (the routing key); this
/// bundles the rest — the load figures the background job
/// computed for that user on this tick.
///
/// `over_capacity` and `over_wip_limit` are passed in by the
/// caller rather than computed here; the caller has already
/// computed them as part of building a `PersonalMetrics` value
/// and we want a single source of truth for the boolean
/// definition.
pub struct NewUserSnapshot {
    pub current_wip: i64,
    pub in_flight_points: i64,
    pub capacity_points: Option<i64>,
    pub over_capacity: bool,
    pub effective_wip_limit: i64,
    pub over_wip_limit: bool,
}

/// Insert one user-metrics snapshot row. Called by the
/// background job tick.
pub async fn insert(pool: &Pool, user_id: &str, snapshot: NewUserSnapshot) -> StorageResult<()> {
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO user_metrics_snapshots (
            id, user_id,
            current_wip, in_flight_points, capacity_points, over_capacity,
            effective_wip_limit, over_wip_limit
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(snapshot.current_wip)
    .bind(snapshot.in_flight_points)
    .bind(snapshot.capacity_points)
    .bind(snapshot.over_capacity as i64)
    .bind(snapshot.effective_wip_limit)
    .bind(snapshot.over_wip_limit as i64)
    .execute(pool)
    .await?;
    Ok(())
}

/// Raw `user_metrics_snapshots` row as returned by sqlx. Kept
/// private — the public API returns [`UserSnapshot`], whose
/// `bool` fields this converts from the stored `0`/`1` integers.
#[derive(FromRow)]
struct UserSnapshotRow {
    user_id: String,
    current_wip: i64,
    in_flight_points: i64,
    capacity_points: Option<i64>,
    over_capacity: i64,
    effective_wip_limit: i64,
    over_wip_limit: i64,
    captured_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserSnapshotRow> for UserSnapshot {
    fn from(r: UserSnapshotRow) -> Self {
        UserSnapshot {
            user_id: r.user_id,
            current_wip: r.current_wip,
            in_flight_points: r.in_flight_points,
            capacity_points: r.capacity_points,
            over_capacity: r.over_capacity != 0,
            effective_wip_limit: r.effective_wip_limit,
            over_wip_limit: r.over_wip_limit != 0,
            captured_at: r.captured_at,
        }
    }
}

/// Recent snapshots for one user, ordered oldest → newest. Used
/// by streak detection in [`crate::user_burnout`]. The window is
/// expressed in days so the query layer doesn't need to know
/// about clock format details.
pub async fn recent_for_user(
    pool: &Pool,
    user_id: &str,
    window_days: i64,
) -> StorageResult<Vec<UserSnapshot>> {
    let rows = sqlx::query_as::<_, UserSnapshotRow>(
        r#"
        SELECT
            user_id, current_wip, in_flight_points, capacity_points,
            over_capacity, effective_wip_limit, over_wip_limit,
            captured_at
        FROM user_metrics_snapshots
        WHERE user_id = ?1
          AND captured_at >= datetime('now', ?2)
        ORDER BY captured_at ASC
        "#,
    )
    .bind(user_id)
    .bind(format!("-{} days", window_days))
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(UserSnapshot::from).collect())
}

/// Users with at least one in-flight assigned issue, used by the
/// background tick to choose which users to snapshot. Idle users
/// (nothing assigned) have no signal to capture, so we save the
/// row and the privacy footprint by skipping them.
pub async fn users_with_active_assignments(pool: &Pool) -> StorageResult<Vec<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT DISTINCT assignee_id
        FROM issues
        WHERE assignee_id IS NOT NULL
          AND status IN ('open', 'in_progress')
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}
