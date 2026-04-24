//! Project CRUD handlers.

use axum::{
    Form,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    AppState,
    auth::AuthUser,
    error::{AppError, AppResult},
    views::{ProjectEditPage, ProjectNewPage, ProjectsListPage},
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
    let projects = crate::db::projects::list_for_user(&state.db, &user.id).await?;
    Ok(ProjectsListPage {
        user,
        projects,
        flash: q.flash,
    })
}

pub async fn new_page(AuthUser(user): AuthUser) -> impl IntoResponse {
    ProjectNewPage { user, flash: None }
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
    crate::db::projects::insert(
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
    let project = crate::db::projects::find_accessible(&state.db, &project_id, &user.id).await?;
    Ok(ProjectEditPage {
        user,
        project,
        flash: None,
    })
}

pub async fn update(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Form(form): Form<ProjectForm>,
) -> AppResult<Redirect> {
    form.validate()
        .map_err(|e| AppError::Validation(super::format_validation(&e)))?;
    crate::db::projects::update(
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
    crate::db::projects::delete(&state.db, &project_id, &user.id).await?;
    Ok(Redirect::to("/projects?flash=Project+deleted"))
}
