//! Teams + memberships persistence (0.14.0).
//!
//! ## API shape
//!
//! Reads:
//! - [`find_by_id`] / [`find_by_slug`] — lookup
//! - [`teams_for_user`] — "what teams am I in?"
//! - [`members_of_team`] — "who's in this team?"
//! - [`role_for`] — "what's my role in this team?" (None if not a member)
//! - [`projects_in_team`] — projects with `team_id = ?`
//!
//! Writes:
//! - [`insert`] — create a new team. The first member is
//!   added as Admin in the same transaction.
//! - [`add_member`] / [`remove_member`] / [`update_role`] — membership
//! - [`update_team`] — rename / re-describe (slug is immutable post-create)
//! - [`assign_project_to_team`] / [`unassign_project`] — project↔team
//!
//! ## Slug uniqueness
//!
//! `INSERT` against the UNIQUE constraint will fail with a
//! `Database` error containing the SQLite UNIQUE message; we
//! could special-case this into [`StorageError::Conflict`] but
//! the web layer's pre-check (look for an existing team by slug
//! before insert) makes the race window tiny in practice, and
//! the conflict message is human-readable enough as-is.

use chrono::{DateTime, Utc};
use peisear_core::teams::{Team, TeamMembership, TeamRole};
use uuid::Uuid;

use crate::{Pool, StorageError, StorageResult};

/// Find a team by its id. None if missing.
pub async fn find_by_id(pool: &Pool, id: &str) -> StorageResult<Option<Team>> {
    let row: Option<(
        String,
        String,
        String,
        Option<String>,
        DateTime<Utc>,
        DateTime<Utc>,
    )> = sqlx::query_as(
        r#"
            SELECT id, name, slug, description, created_at, updated_at
            FROM teams
            WHERE id = ?1
            "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(
        |(id, name, slug, description, created_at, updated_at)| Team {
            id,
            name,
            slug,
            description,
            created_at,
            updated_at,
        },
    ))
}

/// Find a team by URL slug. The hot lookup for `/teams/{slug}`.
pub async fn find_by_slug(pool: &Pool, slug: &str) -> StorageResult<Option<Team>> {
    let row: Option<(
        String,
        String,
        String,
        Option<String>,
        DateTime<Utc>,
        DateTime<Utc>,
    )> = sqlx::query_as(
        r#"
            SELECT id, name, slug, description, created_at, updated_at
            FROM teams
            WHERE slug = ?1
            "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(
        |(id, name, slug, description, created_at, updated_at)| Team {
            id,
            name,
            slug,
            description,
            created_at,
            updated_at,
        },
    ))
}

/// Teams the user is a member of, alphabetised by name. Used
/// for the user nav and `/teams` page.
pub async fn teams_for_user(pool: &Pool, user_id: &str) -> StorageResult<Vec<(Team, TeamRole)>> {
    let rows: Vec<(
        String,
        String,
        String,
        Option<String>,
        DateTime<Utc>,
        DateTime<Utc>,
        String,
    )> = sqlx::query_as(
        r#"
        SELECT t.id, t.name, t.slug, t.description, t.created_at, t.updated_at, m.role
        FROM teams t
        JOIN team_memberships m ON m.team_id = t.id
        WHERE m.user_id = ?1
        ORDER BY t.name COLLATE NOCASE ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .filter_map(
            |(id, name, slug, description, created_at, updated_at, role_str)| {
                // Unknown role values are skipped; in practice the
                // CHECK constraint prevents these, but we'd rather
                // hide a row than panic on an unrecognised future
                // role string.
                TeamRole::from_storage_str(&role_str).map(|role| {
                    (
                        Team {
                            id,
                            name,
                            slug,
                            description,
                            created_at,
                            updated_at,
                        },
                        role,
                    )
                })
            },
        )
        .collect())
}

/// Members of a team with their roles. Used for the team page
/// member list.
pub async fn members_of_team(
    pool: &Pool,
    team_id: &str,
) -> StorageResult<Vec<(String, String, String, TeamRole, DateTime<Utc>)>> {
    // Returns (user_id, display_name, email, role, joined_at).
    let rows: Vec<(String, String, String, String, DateTime<Utc>)> = sqlx::query_as(
        r#"
        SELECT u.id, u.display_name, u.email, m.role, m.joined_at
        FROM team_memberships m
        JOIN users u ON u.id = m.user_id
        WHERE m.team_id = ?1
        ORDER BY
            CASE m.role
                WHEN 'admin' THEN 0
                WHEN 'member' THEN 1
                WHEN 'viewer' THEN 2
                ELSE 3
            END,
            u.display_name COLLATE NOCASE ASC
        "#,
    )
    .bind(team_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .filter_map(|(uid, name, email, role_str, joined)| {
            TeamRole::from_storage_str(&role_str).map(|role| (uid, name, email, role, joined))
        })
        .collect())
}

/// Look up a single membership row. None if the user is not a
/// member of this team. Used by access-control helpers.
pub async fn role_for(
    pool: &Pool,
    team_id: &str,
    user_id: &str,
) -> StorageResult<Option<TeamRole>> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT role FROM team_memberships
        WHERE team_id = ?1 AND user_id = ?2
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|(s,)| TeamRole::from_storage_str(&s)))
}

/// Find one membership in full. None if the user is not a
/// member.
pub async fn membership(
    pool: &Pool,
    team_id: &str,
    user_id: &str,
) -> StorageResult<Option<TeamMembership>> {
    let row: Option<(String, DateTime<Utc>, DateTime<Utc>)> = sqlx::query_as(
        r#"
        SELECT role, joined_at, updated_at FROM team_memberships
        WHERE team_id = ?1 AND user_id = ?2
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|(role_str, joined_at, updated_at)| {
        TeamRole::from_storage_str(&role_str).map(|role| TeamMembership {
            team_id: team_id.to_string(),
            user_id: user_id.to_string(),
            role,
            joined_at,
            updated_at,
        })
    }))
}

/// Create a new team with the given creator as its first
/// member (with role Admin). Both inserts in one transaction
/// so partial failures don't leave a team without admins.
///
/// Returns the new team's id.
///
/// Slug collisions surface as `StorageError::Conflict` after
/// translating the SQLite UNIQUE error.
pub async fn insert(
    pool: &Pool,
    name: &str,
    slug: &str,
    description: Option<&str>,
    creator_user_id: &str,
) -> StorageResult<String> {
    if slug.is_empty() {
        return Err(StorageError::Validation(
            "Team URL slug cannot be empty.".into(),
        ));
    }

    // Pre-check for slug collision so we can return a clean
    // Conflict rather than the SQLite error string.
    if find_by_slug(pool, slug).await?.is_some() {
        return Err(StorageError::Conflict(format!(
            "A team with slug '{slug}' already exists."
        )));
    }

    let id = Uuid::new_v4().to_string();
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO teams (id, name, slug, description)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(name)
    .bind(slug)
    .bind(description)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO team_memberships (team_id, user_id, role)
        VALUES (?, ?, 'admin')
        "#,
    )
    .bind(&id)
    .bind(creator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(id)
}

pub async fn update_team(
    pool: &Pool,
    team_id: &str,
    name: &str,
    description: Option<&str>,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE teams SET name = ?2, description = ?3
        WHERE id = ?1
        "#,
    )
    .bind(team_id)
    .bind(name)
    .bind(description)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// Add a member with the given role. If the user is already a
/// member, returns `StorageError::Conflict` rather than
/// silently updating the role (use `update_role` for that).
pub async fn add_member(
    pool: &Pool,
    team_id: &str,
    user_id: &str,
    role: TeamRole,
) -> StorageResult<()> {
    if let Some(_) = role_for(pool, team_id, user_id).await? {
        return Err(StorageError::Conflict(format!(
            "User {user_id} is already a member of this team."
        )));
    }
    sqlx::query(
        r#"
        INSERT INTO team_memberships (team_id, user_id, role)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .bind(role.as_str())
    .execute(pool)
    .await?;
    Ok(())
}

/// Update an existing member's role. Errors with `NotFound` if
/// the user is not currently a member.
pub async fn update_role(
    pool: &Pool,
    team_id: &str,
    user_id: &str,
    new_role: TeamRole,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE team_memberships SET role = ?3
        WHERE team_id = ?1 AND user_id = ?2
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .bind(new_role.as_str())
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// Remove a member. Errors with `NotFound` if not a member.
///
/// Removing the last admin would orphan the team — application
/// layer guards against that. The schema does not (we'd need a
/// trigger for that and the application check is tractable).
pub async fn remove_member(pool: &Pool, team_id: &str, user_id: &str) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        DELETE FROM team_memberships
        WHERE team_id = ?1 AND user_id = ?2
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// Count of admins in a team. Used by `remove_member` /
/// `update_role` callers to refuse "demote / remove the last
/// admin" operations before issuing the SQL.
pub async fn admin_count(pool: &Pool, team_id: &str) -> StorageResult<i64> {
    let n: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM team_memberships
        WHERE team_id = ?1 AND role = 'admin'
        "#,
    )
    .bind(team_id)
    .fetch_one(pool)
    .await?;
    Ok(n)
}

/// Move a project into a team. Caller must verify both that the
/// project's owner allows the move and that the actor has admin
/// rights on the destination team.
pub async fn assign_project_to_team(
    pool: &Pool,
    project_id: &str,
    team_id: &str,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE projects SET team_id = ?2 WHERE id = ?1
        "#,
    )
    .bind(project_id)
    .bind(team_id)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// Detach a project from its team — turn it back into a
/// personal project (`team_id = NULL`).
pub async fn unassign_project(pool: &Pool, project_id: &str) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE projects SET team_id = NULL WHERE id = ?1
        "#,
    )
    .bind(project_id)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}
