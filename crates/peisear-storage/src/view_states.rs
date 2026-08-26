//! Per-user, per-view UI state.
//!
//! Backs the URL-primary / server-default-secondary scheme for
//! list-page filter and sort persistence introduced in Phase A
//! Step 3 (peisear-feature-spec-v2.1 §4.4 / 0013 migration).
//!
//! The `view_key` namespace convention:
//!
//! - `project_issues:{project_id}` — issue list on the project
//!   detail page.
//!
//! As more list views land, add their key shapes here. The
//! application layer should only construct keys via helper
//! functions in this module so the namespace stays
//! centrally documented.

use crate::{Pool, StorageResult};

/// Build the canonical view key for a project's issue list.
///
/// Centralised so handlers don't accidentally mint two different
/// keys for the same view (`project_issues:{id}` vs
/// `issues_list:{id}`, etc).
pub fn project_issues_key(project_id: &str) -> String {
    format!("project_issues:{project_id}")
}

/// Read the persisted state JSON for `(user, view)`. Returns
/// `None` if the user has never explicitly set a default for this
/// view yet.
///
/// The caller parses the JSON. We deliberately don't deserialise
/// here: the shape is per-view, and the storage layer doesn't
/// own those types. Returning the raw string also leaves
/// migration room — a future view definition that adds a field
/// can read old JSON without losing data.
pub async fn get(pool: &Pool, user_id: &str, view_key: &str) -> StorageResult<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT state_json
        FROM user_view_states
        WHERE user_id = ?1 AND view_key = ?2
        "#,
    )
    .bind(user_id)
    .bind(view_key)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(s,)| s))
}

/// Upsert the state JSON for `(user, view)`. `updated_at` is left to
/// the column's own `DEFAULT` on insert and to `user_view_states_
/// updated_at` (`0017`) on update — `NFR-CONC-003`: the application
/// does not write this column, so a future "show recently used view
/// defaults" feature reading it gets a value with one authority.
///
/// `state_json` is opaque to this layer. The caller is
/// responsible for serialising a sensible shape; see the per-view
/// key documentation at the top of this module.
pub async fn upsert(
    pool: &Pool,
    user_id: &str,
    view_key: &str,
    state_json: &str,
) -> StorageResult<()> {
    sqlx::query(
        r#"
        INSERT INTO user_view_states (user_id, view_key, state_json)
        VALUES (?1, ?2, ?3)
        ON CONFLICT (user_id, view_key) DO UPDATE
        SET state_json = excluded.state_json
        "#,
    )
    .bind(user_id)
    .bind(view_key)
    .bind(state_json)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete `(user, view)` state. Used for "reset to factory
/// default" UX. Idempotent — deleting a row that doesn't exist
/// is not an error.
#[allow(dead_code)]
pub async fn delete(pool: &Pool, user_id: &str, view_key: &str) -> StorageResult<()> {
    sqlx::query(
        r#"
        DELETE FROM user_view_states
        WHERE user_id = ?1 AND view_key = ?2
        "#,
    )
    .bind(user_id)
    .bind(view_key)
    .execute(pool)
    .await?;
    Ok(())
}
