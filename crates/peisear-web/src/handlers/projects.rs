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

pub async fn new_page(AuthUser(user): AuthUser) -> impl IntoResponse {
    components::projects::render_project_new(user, None)
}

#[derive(Debug, Deserialize, Validate)]
pub struct ProjectForm {
    #[validate(length(min = 1, max = 120, message = "Name is required (max 120 chars)."))]
    pub name: String,
    #[validate(length(max = 4000, message = "Description must be under 4000 chars."))]
    pub description: String,
}

pub async fn create(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<ProjectForm>,
) -> AppResult<Redirect> {
    form.validate()
        .map_err(|e| AppError::Validation(super::format_validation(&e)))?;

    let id = uuid::Uuid::new_v4().to_string();
    projects::insert(
        &state.db,
        &id,
        &user.id,
        form.name.trim(),
        form.description.trim(),
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
