//! The identity type for personal-data storage (`PRIV-002`/`PRIV-003`,
//! `DEC-058`).
//!
//! `PRIV-004`: moved here from `user_metrics_snapshots`, unchanged in
//! behaviour — that placement named identity after one of its consumers
//! (a metrics module) rather than after what it is. The seal itself did
//! not move: [`SubjectId`]'s field is private to this module, not to
//! `peisear-storage`, so nothing outside it — including the rest of
//! `peisear-storage` — can construct one by struct-literal syntax. The
//! mint ([`users_with_active_assignments`]) had to move with the type,
//! or the seal would have weakened from module-private to crate-private;
//! it is not `pub(crate)`, by design — that would let any module in this
//! crate mint an identity.
//!
//! Two legitimate construction paths, no others:
//!
//! - `From<&RequesterId>` below — a requester is always the subject of
//!   their own data, the handler path, total for every caller that has
//!   one.
//! - This module's own enumeration, `users_with_active_assignments` —
//!   the snapshot job's path, where no `RequesterId` exists at all.

use peisear_auth::jwt::RequesterId;

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
