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
use sqlx::FromRow;
use uuid::Uuid;

use crate::{Pool, StorageError, StorageResult};

/// Raw `teams` row as returned by sqlx. Kept private — the public
/// API returns [`peisear_core::teams::Team`].
#[derive(FromRow)]
struct TeamRow {
    id: String,
    name: String,
    slug: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<TeamRow> for Team {
    fn from(r: TeamRow) -> Self {
        Team {
            id: r.id,
            name: r.name,
            slug: r.slug,
            description: r.description,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

/// `teams` row joined with the querying user's `role` in that
/// team. Kept private, same reasoning as [`TeamRow`].
#[derive(FromRow)]
struct TeamRoleRow {
    id: String,
    name: String,
    slug: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    role: String,
}

/// Find a team by its id. None if missing.
pub async fn find_by_id(pool: &Pool, id: &str) -> StorageResult<Option<Team>> {
    let row = sqlx::query_as::<_, TeamRow>(
        r#"
            SELECT id, name, slug, description, created_at, updated_at
            FROM teams
            WHERE id = ?1
            "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Team::from))
}

/// Find a team by URL slug. The hot lookup for `/teams/{slug}`.
pub async fn find_by_slug(pool: &Pool, slug: &str) -> StorageResult<Option<Team>> {
    let row = sqlx::query_as::<_, TeamRow>(
        r#"
            SELECT id, name, slug, description, created_at, updated_at
            FROM teams
            WHERE slug = ?1
            "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Team::from))
}

/// Teams the user is a member of, alphabetised by name. Used
/// for the user nav and `/teams` page.
pub async fn teams_for_user(pool: &Pool, user_id: &str) -> StorageResult<Vec<(Team, TeamRole)>> {
    let rows = sqlx::query_as::<_, TeamRoleRow>(
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
        .filter_map(|r| {
            // Unknown role values are skipped; in practice the
            // CHECK constraint prevents these, but we'd rather
            // hide a row than panic on an unrecognised future
            // role string.
            TeamRole::from_storage_str(&r.role).map(|role| {
                (
                    Team {
                        id: r.id,
                        name: r.name,
                        slug: r.slug,
                        description: r.description,
                        created_at: r.created_at,
                        updated_at: r.updated_at,
                    },
                    role,
                )
            })
        })
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
///
/// **A point-in-time read, and the window that follows it is accepted
/// (`RACE-003`).** Handlers call this to learn the *actor's* role, test
/// `can_manage_team()` or `can_write()`, and then write; nothing holds the
/// database lock across those steps. A role change committed between the
/// read and the write -- an admin demoted or removed in that gap -- is not
/// seen by the action already in flight, which still completes. That
/// window exists in every handler that authorises this way. It is accepted,
/// not closed: closing it would mean holding the write lock across each
/// handler's whole body, and the change takes effect on the actor's next
/// request either way. It does not touch [`update_role`] and
/// [`remove_member`]'s rule that a team keeps an admin, which reads its
/// own state again under its own lock.
pub async fn role_for<'e, E>(
    executor: E,
    team_id: &str,
    user_id: &str,
) -> StorageResult<Option<TeamRole>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT role FROM team_memberships
        WHERE team_id = ?1 AND user_id = ?2
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .fetch_optional(executor)
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
            peisear_i18n::MessageKey::TeamSlugCannotBeEmptyMessage,
        ));
    }

    // Pre-check for slug collision so we can return a clean
    // Conflict rather than the SQLite error string.
    if find_by_slug(pool, slug).await?.is_some() {
        return Err(StorageError::Conflict(
            peisear_i18n::MessageKey::TeamSlugAlreadyExistsMessage {
                slug: slug.to_string(),
            },
        ));
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
///
/// **Concurrent duplicates (`RACE-003`).** The check below is a fast
/// path, not the guarantee: two simultaneous adds both pass it. The
/// guarantee is the table's `PRIMARY KEY (team_id, user_id)`, which
/// refuses the second `INSERT` -- so the *state* was always right, and
/// what the loser used to get was the driver's raw error. A unique
/// violation is now mapped onto the same
/// `UserAlreadyTeamMemberMessage` the check returns. No transaction is
/// needed: nothing here is wrong except the words.
pub async fn add_member(
    pool: &Pool,
    team_id: &str,
    user_id: &str,
    role: TeamRole,
) -> StorageResult<()> {
    if role_for(pool, team_id, user_id).await?.is_some() {
        return Err(StorageError::Conflict(
            peisear_i18n::MessageKey::UserAlreadyTeamMemberMessage {
                user_id: user_id.to_string(),
            },
        ));
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
    .await
    .map_err(|e| match e {
        // `sqlx` reports both SQLite's UNIQUE and PRIMARY KEY constraint
        // codes as `UniqueViolation`; this table has no other unique key,
        // so on this statement it can only be the duplicate membership.
        sqlx::Error::Database(db) if db.kind() == sqlx::error::ErrorKind::UniqueViolation => {
            StorageError::Conflict(peisear_i18n::MessageKey::UserAlreadyTeamMemberMessage {
                user_id: user_id.to_string(),
            })
        }
        other => other.into(),
    })?;
    Ok(())
}

/// Update an existing member's role. Errors with `NotFound` if
/// the user is not currently a member.
///
/// **A team must keep an administrator (`RACE-001`).** Changing an
/// admin's role to anything else while they are the team's last admin
/// is refused with [`StorageError::Conflict`]
/// (`MessageKey::LastAdminDemotionError`). The check and the write are
/// one `BEGIN IMMEDIATE` transaction, as in `user_capacities::insert`
/// (`CAP-001`; the reasoning for `IMMEDIATE` over a deferred `BEGIN`
/// is in that module's docs): when this check ran on the pool ahead of
/// the write, two admins demoting each other at once both saw two
/// admins and both proceeded, leaving a team no remaining member could
/// repair -- appointing an admin requires being one.
pub async fn update_role(
    pool: &Pool,
    team_id: &str,
    user_id: &str,
    new_role: TeamRole,
) -> StorageResult<()> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    if !matches!(new_role, TeamRole::Admin)
        && matches!(
            role_for(&mut *tx, team_id, user_id).await?,
            Some(TeamRole::Admin)
        )
        && admin_count(&mut *tx, team_id).await? <= 1
    {
        return Err(StorageError::Conflict(
            peisear_i18n::MessageKey::LastAdminDemotionError,
        ));
    }
    let res = sqlx::query(
        r#"
        UPDATE team_memberships SET role = ?3
        WHERE team_id = ?1 AND user_id = ?2
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .bind(new_role.as_str())
    .execute(&mut *tx)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    tx.commit().await?;
    Ok(())
}

/// Remove a member. Errors with `NotFound` if not a member.
///
/// **A team must keep an administrator (`RACE-001`).** Removing the
/// team's last admin -- including an admin removing themselves -- is
/// refused with [`StorageError::Conflict`]
/// (`MessageKey::LastAdminRemovalError`), checked and written in one
/// `BEGIN IMMEDIATE` transaction; see [`update_role`]. The schema does
/// not enforce this (it would need a trigger); this function is where
/// the rule lives, so a caller cannot forget it.
pub async fn remove_member(pool: &Pool, team_id: &str, user_id: &str) -> StorageResult<()> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    if matches!(
        role_for(&mut *tx, team_id, user_id).await?,
        Some(TeamRole::Admin)
    ) && admin_count(&mut *tx, team_id).await? <= 1
    {
        return Err(StorageError::Conflict(
            peisear_i18n::MessageKey::LastAdminRemovalError,
        ));
    }
    let res = sqlx::query(
        r#"
        DELETE FROM team_memberships
        WHERE team_id = ?1 AND user_id = ?2
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    tx.commit().await?;
    Ok(())
}

/// Count of admins in a team. [`update_role`] and [`remove_member`]
/// use it, inside their transaction, to refuse removing the last
/// admin; a caller does not need to check first (and must not: a check
/// made outside the transaction is the race `RACE-001` closed).
pub async fn admin_count<'e, E>(executor: E, team_id: &str) -> StorageResult<i64>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let n: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM team_memberships
        WHERE team_id = ?1 AND role = 'admin'
        "#,
    )
    .bind(team_id)
    .fetch_one(executor)
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
