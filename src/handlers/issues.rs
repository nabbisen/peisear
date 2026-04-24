//! Issue CRUD handlers including the board view with drag-and-drop
//! status updates.

use axum::{
    Form, Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    AppState,
    auth::AuthUser,
    error::{AppError, AppResult},
    models::{IssueStatus, Priority},
    views::{Column, IssueDetailPage, IssueNewPage, ProjectDetailPage},
};

#[derive(Debug, Deserialize)]
pub struct ProjectViewQuery {
    pub view: Option<String>,
    pub flash: Option<String>,
}

/// Project detail page: renders either board (kanban) or list view.
pub async fn project_detail(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Query(q): Query<ProjectViewQuery>,
) -> AppResult<impl IntoResponse> {
    let project = crate::db::projects::find_accessible(&state.db, &project_id, &user.id).await?;
    let all_issues = crate::db::issues::list_in_project(&state.db, &project_id).await?;

    let mut columns: Vec<Column> = IssueStatus::all()
        .into_iter()
        .map(|s| Column {
            status: s,
            issues: Vec::new(),
        })
        .collect();
    for issue in &all_issues {
        if let Some(col) = columns.iter_mut().find(|c| c.status == issue.status) {
            col.issues.push(issue.clone());
        }
    }

    let view_mode = match q.view.as_deref() {
        Some("list") => "list".to_string(),
        _ => "board".to_string(),
    };

    Ok(ProjectDetailPage {
        user,
        project,
        columns,
        view_mode,
        all_issues,
        flash: q.flash,
    })
}

pub async fn new_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let project = crate::db::projects::find_accessible(&state.db, &project_id, &user.id).await?;
    Ok(IssueNewPage {
        user,
        project,
        priorities: Priority::all().to_vec(),
        statuses: IssueStatus::all().to_vec(),
        flash: None,
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct IssueForm {
    #[validate(length(min = 1, max = 200, message = "Title is required (max 200 chars)."))]
    pub title: String,
    #[validate(length(max = 10_000, message = "Description too long (max 10,000 chars)."))]
    pub description: String,
    pub status: String,
    pub priority: String,
}

pub async fn create(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Form(form): Form<IssueForm>,
) -> AppResult<Redirect> {
    form.validate()
        .map_err(|e| AppError::Validation(super::format_validation(&e)))?;

    // Enforce access to the project.
    let _project =
        crate::db::projects::find_accessible(&state.db, &project_id, &user.id).await?;

    let status = IssueStatus::parse(&form.status)
        .ok_or_else(|| AppError::Validation("Invalid status".into()))?;
    let priority = Priority::parse(&form.priority)
        .ok_or_else(|| AppError::Validation("Invalid priority".into()))?;

    let id = uuid::Uuid::new_v4().to_string();
    crate::db::issues::insert(
        &state.db,
        &id,
        &project_id,
        &user.id,
        form.title.trim(),
        form.description.trim(),
        status,
        priority,
    )
    .await?;
    Ok(Redirect::to(&format!("/projects/{project_id}/issues/{id}")))
}

#[derive(Debug, Deserialize)]
pub struct EditFlag {
    pub edit: Option<u8>,
    pub flash: Option<String>,
}

pub async fn detail_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((project_id, issue_id)): Path<(String, String)>,
    Query(q): Query<EditFlag>,
) -> AppResult<impl IntoResponse> {
    let project = crate::db::projects::find_accessible(&state.db, &project_id, &user.id).await?;
    let issue = crate::db::issues::find(&state.db, &issue_id, &project_id).await?;
    Ok(IssueDetailPage {
        user,
        project,
        issue,
        priorities: Priority::all().to_vec(),
        statuses: IssueStatus::all().to_vec(),
        flash: q.flash,
        editing: q.edit == Some(1),
    })
}

pub async fn update(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((project_id, issue_id)): Path<(String, String)>,
    Form(form): Form<IssueForm>,
) -> AppResult<Redirect> {
    form.validate()
        .map_err(|e| AppError::Validation(super::format_validation(&e)))?;

    // Access check.
    let _project =
        crate::db::projects::find_accessible(&state.db, &project_id, &user.id).await?;

    let status = IssueStatus::parse(&form.status)
        .ok_or_else(|| AppError::Validation("Invalid status".into()))?;
    let priority = Priority::parse(&form.priority)
        .ok_or_else(|| AppError::Validation("Invalid priority".into()))?;

    crate::db::issues::update(
        &state.db,
        &issue_id,
        &project_id,
        form.title.trim(),
        form.description.trim(),
        status,
        priority,
    )
    .await?;
    Ok(Redirect::to(&format!(
        "/projects/{project_id}/issues/{issue_id}"
    )))
}

pub async fn delete(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((project_id, issue_id)): Path<(String, String)>,
) -> AppResult<Redirect> {
    // Access check.
    let _project =
        crate::db::projects::find_accessible(&state.db, &project_id, &user.id).await?;
    crate::db::issues::delete(&state.db, &issue_id, &project_id).await?;
    Ok(Redirect::to(&format!(
        "/projects/{project_id}?flash=Issue+deleted"
    )))
}

// --- JSON endpoints for the kanban drag-and-drop UI ---

#[derive(Debug, Deserialize)]
pub struct StatusChange {
    pub status: String,
}

pub async fn change_status(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((project_id, issue_id)): Path<(String, String)>,
    Json(body): Json<StatusChange>,
) -> AppResult<StatusCode> {
    let _project =
        crate::db::projects::find_accessible(&state.db, &project_id, &user.id).await?;
    let status = IssueStatus::parse(&body.status)
        .ok_or_else(|| AppError::Validation("Invalid status".into()))?;
    crate::db::issues::update_status(&state.db, &issue_id, &project_id, status).await?;
    Ok(StatusCode::NO_CONTENT)
}
