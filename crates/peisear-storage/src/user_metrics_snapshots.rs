//! Persisted history of per-user personal-load metrics.
//!
//! Sibling to [`crate::metrics_snapshots`]. The two tables exist
//! separately because their privacy boundaries differ — see the
//! rationale comment in `migrations/0008_user_metrics_snapshots.sql`.
//!
//! Read API today exposes streak-counting queries used by
//! [`crate::user_burnout`]. Write API is one function called from
//! the background job for each user with active assigned work.
//!
//! `PRIV-003` (`DEC-058`): [`SubjectId`] is declared here, not in
//! `peisear-auth` beside [`peisear_auth::jwt::RequesterId`]. Its own
//! trusted origin is a database enumeration
//! ([`users_with_active_assignments`]), not a token, and putting it
//! beside `RequesterId` would need `peisear-auth` to know about rows —
//! a dependency in the wrong direction for a crate whose own doc
//! comment states it has none. The field is private to this module, so
//! nothing outside it — including the rest of `peisear-storage` — can
//! construct one by struct-literal syntax; the only two paths are
//! `From<&RequesterId>` below (a requester is always the subject of
//! their own data) and this module's own enumeration.

use peisear_auth::jwt::RequesterId;
use sqlx::FromRow;
use uuid::Uuid;

use crate::{Pool, StorageResult};

/// See the module doc comment for why this lives here and what the two
/// legitimate construction paths are. No public constructor exists
/// beyond `From<&RequesterId>` and this module's own enumeration query —
/// deliberately: a public `SubjectId::new(&str)` would be the `String`
/// convention again, and would let a handler mint one from a path
/// parameter and reach every function `PRIV-002`/`PRIV-003` sealed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubjectId(String);

impl SubjectId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A requester is always the subject of their own data (`DEC-058`
/// decision 2) — the handler path, and it is total: every handler that
/// has a `RequesterId` can get a `SubjectId` for it with no fallible
/// step.
impl From<&RequesterId> for SubjectId {
    fn from(rid: &RequesterId) -> Self {
        SubjectId(rid.as_str().to_string())
    }
}

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
///
/// `PRIV-003`: takes [`SubjectId`] — the job's own enumeration
/// ([`users_with_active_assignments`]) is one of its two legitimate
/// sources.
pub async fn insert(
    pool: &Pool,
    user_id: &SubjectId,
    snapshot: NewUserSnapshot,
) -> StorageResult<()> {
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
    .bind(user_id.as_str())
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
///
/// `PRIV-003`: takes [`SubjectId`], matching
/// [`crate::user_burnout::for_user`], its only caller.
pub async fn recent_for_user(
    pool: &Pool,
    user_id: &SubjectId,
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
        ORDER BY captured_at ASC, rowid ASC
        "#,
    )
    .bind(user_id.as_str())
    .bind(format!("-{} days", window_days))
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(UserSnapshot::from).collect())
}

/// Users with at least one in-flight assigned issue, used by the
/// background tick to choose which users to snapshot. Idle users
/// (nothing assigned) have no signal to capture, so we save the
/// row and the privacy footprint by skipping them.
pub async fn users_with_active_assignments(pool: &Pool) -> StorageResult<Vec<SubjectId>> {
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
    Ok(rows.into_iter().map(|(id,)| SubjectId(id)).collect())
}
