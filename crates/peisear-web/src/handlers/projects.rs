//! Project CRUD handlers.

use axum::{
    Form,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use peisear_storage::projects;
use serde::Deserialize;
use validator::Validate;

use crate::{
    AppError, AppResult, AppState,
    components,
    extractors::AuthUser,
};

#[derive(Debug, Deserialize)]
pub struct FlashQuery {
    pub flash: Option<String>,
}

pub async fn list_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let projects = projects::list_for_user(&state.db, &user.id).await?;
    Ok(components::projects::render_projects_list(
        user, projects, q.flash,
    ))
}

pub async fn new_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    // Load the user's teams (any role) so the form can offer
    // them as project assignment targets. Members and admins
    // can put projects in their teams; viewers cannot
    // (filtered at the post-submit access-control check).
    let user_teams = peisear_storage::teams::teams_for_user(&state.db, &user.id).await?;
    let writable_teams: Vec<(peisear_core::teams::Team, peisear_core::teams::TeamRole)> =
        user_teams
            .into_iter()
            .filter(|(_, role)| role.can_write())
            .collect();
    Ok(components::projects::render_project_new(user, writable_teams, None))
}

#[derive(Debug, Deserialize, Validate)]
pub struct ProjectForm {
    #[validate(length(min = 1, max = 120, message = "Name is required (max 120 chars)."))]
    pub name: String,
    #[validate(length(max = 4000, message = "Description must be under 4000 chars."))]
    pub description: String,
    /// Optional team assignment. The form sends an empty string
    /// for "personal project"; we treat that as `None`.
    #[serde(default)]
    pub team_id: String,
    /// RFC3339 timestamp captured when the edit form was rendered.
    /// Validated against the project's current `updated_at` per
    /// peisear-feature-spec-v2.1 §21.4 (Phase A Step 5). Empty
    /// (i.e. omitted) is rejected as a 400 by `check_optimistic_lock`
    /// — we never silently bypass the lock. Default for create
    /// (which doesn't supply it).
    #[serde(default)]
    pub client_updated_at: String,
}

pub async fn create(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<ProjectForm>,
) -> AppResult<Redirect> {
    form.validate()
        .map_err(|e| AppError::Validation(super::format_validation(&e)))?;

    let team_id_input = form.team_id.trim();
    let team_id_opt: Option<&str> = if team_id_input.is_empty() {
        None
    } else {
        Some(team_id_input)
    };

    // If a team was selected, verify the user is a member with
    // write capability. Without this guard, any user could
    // assign a project to any team they know the id of.
    if let Some(tid) = team_id_opt {
        let role = peisear_storage::teams::role_for(&state.db, tid, &user.id).await?;
        let Some(role) = role else {
            return Err(AppError::Forbidden);
        };
        if !role.can_write() {
            return Err(AppError::Forbidden);
        }
    }

    let id = uuid::Uuid::new_v4().to_string();
    projects::insert(
        &state.db,
        &id,
        &user.id,
        form.name.trim(),
        form.description.trim(),
        team_id_opt,
    )
    .await?;

    Ok(Redirect::to(&format!("/projects/{id}")))
}

pub async fn edit_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let project = projects::find_accessible(&state.db, &project_id, &user.id).await?;
    Ok(components::projects::render_project_edit(user, project, None))
}

pub async fn update(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Form(form): Form<ProjectForm>,
) -> AppResult<Redirect> {
    form.validate()
        .map_err(|e| AppError::Validation(super::format_validation(&e)))?;

    // Optimistic-lock check (peisear-feature-spec-v2.1 §21.4).
    // Read the current project row; both for the access check
    // and for its `updated_at` to compare against the form.
    let current = projects::find_accessible(&state.db, &project_id, &user.id).await?;
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        current.updated_at,
        "project",
        &project_id,
    )?;

    projects::update(
        &state.db,
        &project_id,
        &user.id,
        form.name.trim(),
        form.description.trim(),
    )
    .await?;
    Ok(Redirect::to(&format!("/projects/{project_id}")))
}

pub async fn delete(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> AppResult<Redirect> {
    projects::delete(&state.db, &project_id, &user.id).await?;
    Ok(Redirect::to("/projects?flash=Project+deleted"))
}
