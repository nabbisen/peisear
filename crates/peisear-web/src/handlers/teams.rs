//! Teams handlers (0.14.0).
//!
//! Routes:
//! - `GET /teams` — list user's teams
//! - `GET /teams/new` — create form
//! - `POST /teams` — create
//! - `GET /teams/{slug}` — team detail
//! - `GET /teams/{slug}/edit` — edit form (admin)
//! - `POST /teams/{slug}/edit` — submit edit (admin)
//! - `POST /teams/{slug}/members` — add member (admin)
//! - `POST /teams/{slug}/members/{user_id}/role` — change role (admin)
//! - `POST /teams/{slug}/members/{user_id}/remove` — remove member (admin)
//! - `POST /teams/{slug}/projects/{project_id}/unassign` — detach project from team (admin)
//!
//! Access control posture (V2.1 §2.5):
//!
//! - Any authenticated user can navigate to `/teams/{slug}` if
//!   they are a member of the team.
//! - Non-members are redirected away rather than told the team
//!   exists. Slug-typing fishing is harmless; we don't make it
//!   useful by leaking team existence.
//! - Admin-only operations check `role.can_manage_team()` and
//!   return `AppError::Forbidden` to non-admins.

use axum::{
    Form,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use peisear_core::teams::{TeamRole, slugify};
use peisear_i18n::{Locale, MessageKey};
use peisear_storage::{notifications as notif_store, projects, teams, users};
use serde::Deserialize;

use crate::{AppError, AppResult, AppState, components, components::t, extractors::AuthUser};

#[derive(Debug, Deserialize)]
pub struct FlashQuery {
    pub flash: Option<String>,
    pub error: Option<String>,
}

pub async fn list_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let user_teams = teams::teams_for_user(&state.db, &user.id).await?;
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;
    Ok(components::teams::render_list(
        user,
        user_teams,
        unread_count,
        q.flash,
        q.error,
    ))
}

pub async fn new_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;
    Ok(components::teams::render_new(user, unread_count, q.error))
}

#[derive(Debug, Deserialize)]
pub struct CreateForm {
    pub name: String,
    /// Optional explicit slug; if blank, generated from name.
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub description: String,
}

pub async fn create(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<CreateForm>,
) -> AppResult<Redirect> {
    let name = form.name.trim();
    if name.is_empty() {
        return Err(AppError::Validation(t(MessageKey::TeamNameRequiredMessage)));
    }
    let slug_candidate = if form.slug.trim().is_empty() {
        slugify(name)
    } else {
        slugify(&form.slug)
    };
    if slug_candidate.is_empty() {
        return Err(AppError::Validation(t(
            MessageKey::SlugDerivationFailedMessage,
        )));
    }
    let description = if form.description.trim().is_empty() {
        None
    } else {
        Some(form.description.trim())
    };

    match teams::insert(&state.db, name, &slug_candidate, description, &user.id).await {
        Ok(_) => {
            let flash =
                super::percent_encode_query(&Locale::English.render(MessageKey::TeamCreatedFlash));
            Ok(Redirect::to(&format!(
                "/teams/{slug_candidate}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Conflict(msg)) => {
            // Re-render the form with the conflict in the
            // error query param. The form is on `/teams/new`,
            // so redirect there.
            let encoded = super::percent_encode_query(&t(msg));
            Ok(Redirect::to(&format!("/teams/new?error={encoded}")))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(t(msg))),
        Err(e) => Err(e.into()),
    }
}

pub async fn detail(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let team = teams::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;

    let role = teams::role_for(&state.db, &team.id, &user.id).await?;
    let Some(role) = role else {
        // Non-member. Treat as not-found rather than forbidden:
        // we don't want slug-typing to leak which teams exist.
        return Err(AppError::NotFound);
    };

    let members = teams::members_of_team(&state.db, &team.id).await?;
    let team_projects = projects::list_for_team(&state.db, &team.id).await?;
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::teams::render_detail(
        user,
        team,
        role,
        members,
        team_projects,
        unread_count,
        q.flash,
        q.error,
    ))
}

pub async fn edit_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let team = teams::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;
    let role = teams::role_for(&state.db, &team.id, &user.id).await?;
    let Some(role) = role else {
        return Err(AppError::NotFound);
    };
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;
    Ok(components::teams::render_edit(
        user,
        team,
        unread_count,
        q.error,
    ))
}

#[derive(Debug, Deserialize)]
pub struct EditForm {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

pub async fn update(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Form(form): Form<EditForm>,
) -> AppResult<Redirect> {
    let team = teams::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;
    let role = teams::role_for(&state.db, &team.id, &user.id).await?;
    let Some(role) = role else {
        return Err(AppError::NotFound);
    };
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let name = form.name.trim();
    if name.is_empty() {
        return Err(AppError::Validation(t(MessageKey::TeamNameRequiredMessage)));
    }
    let description = if form.description.trim().is_empty() {
        None
    } else {
        Some(form.description.trim())
    };
    teams::update_team(&state.db, &team.id, name, description).await?;
    let flash = super::percent_encode_query(&Locale::English.render(MessageKey::TeamUpdatedFlash));
    Ok(Redirect::to(&format!("/teams/{slug}?flash={flash}")))
}

#[derive(Debug, Deserialize)]
pub struct AddMemberForm {
    pub email: String,
    /// `admin` / `member` / `viewer`
    pub role: String,
}

pub async fn add_member(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Form(form): Form<AddMemberForm>,
) -> AppResult<Redirect> {
    let team = teams::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;
    let actor_role = teams::role_for(&state.db, &team.id, &user.id).await?;
    let Some(actor_role) = actor_role else {
        return Err(AppError::NotFound);
    };
    if !actor_role.can_manage_team() {
        return Err(AppError::Forbidden);
    }

    let email = form.email.trim();
    let new_role = TeamRole::from_storage_str(form.role.trim())
        .ok_or_else(|| AppError::Validation(t(MessageKey::InvalidRoleMessage)))?;

    let target = users::find_by_email(&state.db, email).await?;
    let Some(target) = target else {
        let encoded = super::percent_encode_query(&Locale::English.render(
            MessageKey::NoUserWithEmailFound {
                email: email.to_string(),
            },
        ));
        return Ok(Redirect::to(&format!("/teams/{slug}?error={encoded}")));
    };

    match teams::add_member(&state.db, &team.id, &target.id, new_role).await {
        Ok(()) => {
            let flash =
                super::percent_encode_query(&Locale::English.render(MessageKey::MemberAddedFlash));
            Ok(Redirect::to(&format!("/teams/{slug}?flash={flash}")))
        }
        Err(peisear_storage::StorageError::Conflict(msg)) => {
            let encoded = super::percent_encode_query(&t(msg));
            Ok(Redirect::to(&format!("/teams/{slug}?error={encoded}")))
        }
        Err(e) => Err(e.into()),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleForm {
    pub role: String,
}

pub async fn update_member_role(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, target_user_id)): Path<(String, String)>,
    Form(form): Form<UpdateRoleForm>,
) -> AppResult<Redirect> {
    let team = teams::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;
    let actor_role = teams::role_for(&state.db, &team.id, &user.id).await?;
    let Some(actor_role) = actor_role else {
        return Err(AppError::NotFound);
    };
    if !actor_role.can_manage_team() {
        return Err(AppError::Forbidden);
    }

    let new_role = TeamRole::from_storage_str(form.role.trim())
        .ok_or_else(|| AppError::Validation(t(MessageKey::InvalidRoleMessage)))?;

    // Refuse to demote the last admin.
    if !matches!(new_role, TeamRole::Admin) {
        let current = teams::role_for(&state.db, &team.id, &target_user_id).await?;
        if matches!(current, Some(TeamRole::Admin)) {
            let admins = teams::admin_count(&state.db, &team.id).await?;
            if admins <= 1 {
                let encoded = super::percent_encode_query(
                    &Locale::English.render(MessageKey::LastAdminDemotionError),
                );
                return Ok(Redirect::to(&format!("/teams/{slug}?error={encoded}")));
            }
        }
    }

    teams::update_role(&state.db, &team.id, &target_user_id, new_role).await?;
    let flash = super::percent_encode_query(&Locale::English.render(MessageKey::RoleUpdatedFlash));
    Ok(Redirect::to(&format!("/teams/{slug}?flash={flash}")))
}

pub async fn remove_member(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, target_user_id)): Path<(String, String)>,
) -> AppResult<Redirect> {
    let team = teams::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;
    let actor_role = teams::role_for(&state.db, &team.id, &user.id).await?;
    let Some(actor_role) = actor_role else {
        return Err(AppError::NotFound);
    };

    // Self-removal is allowed regardless of admin status (a
    // non-admin member can leave a team at any time). However,
    // an admin removing themselves still has to leave at least
    // one admin behind — same rule as update_member_role.
    let is_self_removal = user.id == target_user_id;
    if !is_self_removal && !actor_role.can_manage_team() {
        return Err(AppError::Forbidden);
    }

    let target_role = teams::role_for(&state.db, &team.id, &target_user_id).await?;
    if matches!(target_role, Some(TeamRole::Admin)) {
        let admins = teams::admin_count(&state.db, &team.id).await?;
        if admins <= 1 {
            let encoded = super::percent_encode_query(
                &Locale::English.render(MessageKey::LastAdminRemovalError),
            );
            return Ok(Redirect::to(&format!("/teams/{slug}?error={encoded}")));
        }
    }

    teams::remove_member(&state.db, &team.id, &target_user_id).await?;

    if is_self_removal {
        let flash =
            super::percent_encode_query(&Locale::English.render(MessageKey::YouLeftTeamFlash));
        Ok(Redirect::to(&format!("/teams?flash={flash}")))
    } else {
        let flash =
            super::percent_encode_query(&Locale::English.render(MessageKey::MemberRemovedFlash));
        Ok(Redirect::to(&format!("/teams/{slug}?flash={flash}")))
    }
}

pub async fn unassign_project(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, project_id)): Path<(String, String)>,
) -> AppResult<Redirect> {
    let team = teams::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(AppError::NotFound)?;
    let role = teams::role_for(&state.db, &team.id, &user.id).await?;
    let Some(role) = role else {
        return Err(AppError::NotFound);
    };
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    teams::unassign_project(&state.db, &project_id).await?;
    let flash =
        super::percent_encode_query(&Locale::English.render(MessageKey::ProjectDetachedFlash));
    Ok(Redirect::to(&format!("/teams/{slug}?flash={flash}")))
}
