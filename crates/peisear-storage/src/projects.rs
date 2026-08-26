//! Project table queries.

use chrono::{DateTime, Utc};
use peisear_core::Project;
use sqlx::FromRow;

use crate::{Pool, StorageError, StorageResult};

#[derive(FromRow)]
struct ProjectRow {
    id: String,
    owner_id: String,
    name: String,
    description: String,
    wip_limit_default: Option<i64>,
    team_id: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ProjectRow> for Project {
    fn from(r: ProjectRow) -> Self {
        Project {
            id: r.id,
            owner_id: r.owner_id,
            name: r.name,
            description: r.description,
            wip_limit_default: r.wip_limit_default,
            team_id: r.team_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

pub async fn list_for_user(pool: &Pool, user_id: &str) -> StorageResult<Vec<Project>> {
    // Returns the user's *personal* (team_id IS NULL) projects
    // plus team projects of any team they belong to.
    // Personal project ownership is unchanged by 0.14.0:
    // `owner_id = user_id`. Team projects are reached via the
    // membership join below.
    let rows = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT DISTINCT p.id, p.owner_id, p.name, p.description,
            p.wip_limit_default, p.team_id, p.created_at, p.updated_at
        FROM projects p
        LEFT JOIN team_memberships m
               ON m.team_id = p.team_id AND m.user_id = ?1
        WHERE
            (p.team_id IS NULL AND p.owner_id = ?1)
         OR (p.team_id IS NOT NULL AND m.user_id = ?1)
        ORDER BY p.updated_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

/// All projects belonging to a given team.
pub async fn list_for_team(pool: &Pool, team_id: &str) -> StorageResult<Vec<Project>> {
    let rows = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT id, owner_id, name, description,
            wip_limit_default, team_id, created_at, updated_at
        FROM projects
        WHERE team_id = ?1
        ORDER BY updated_at DESC
        "#,
    )
    .bind(team_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn find_accessible(
    pool: &Pool,
    project_id: &str,
    user_id: &str,
) -> StorageResult<Project> {
    let row = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT p.id, p.owner_id, p.name, p.description,
            p.wip_limit_default, p.team_id, p.created_at, p.updated_at
        FROM projects p
        LEFT JOIN team_memberships m
               ON m.team_id = p.team_id AND m.user_id = ?2
        WHERE p.id = ?1
          AND (
                (p.team_id IS NULL AND p.owner_id = ?2)
             OR (p.team_id IS NOT NULL AND m.user_id = ?2)
          )
        "#,
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    row.map(Into::into).ok_or(StorageError::NotFound)
}

pub async fn insert(
    pool: &Pool,
    id: &str,
    owner_id: &str,
    name: &str,
    description: &str,
    team_id: Option<&str>,
) -> StorageResult<()> {
    sqlx::query(
        r#"
        INSERT INTO projects (id, owner_id, name, description, team_id)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .bind(name)
    .bind(description)
    .bind(team_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update(
    pool: &Pool,
    id: &str,
    owner_id: &str,
    name: &str,
    description: &str,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE projects
        SET name = ?3, description = ?4
        WHERE id = ?1 AND owner_id = ?2
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .bind(name)
    .bind(description)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}

/// `WHERE owner_id = ?2` is the authorisation, not a defensive extra:
/// a caller who names a project id they don't own affects zero rows,
/// and `rows_affected() == 0 → NotFound` turns that into "no such
/// project for you" — the concealment behaviour external design §9
/// asks for — rather than a distinguishable "found it, but you can't
/// touch it" response. `QA-002-review.md` §4.1: a handler-level
/// ownership check was added on top of this, on a misdiagnosis that
/// this function reported success on a zero-row delete; it does not,
/// and the handler check was reverted. This comment exists so the
/// next reader sees why the row count is deliberate before mistaking
/// it for a gap the way that review did.
pub async fn delete(pool: &Pool, id: &str, owner_id: &str) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        DELETE FROM projects
        WHERE id = ?1 AND owner_id = ?2
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .execute(pool)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }
    Ok(())
}
