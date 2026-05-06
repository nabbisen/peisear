//! Issue CRUD handlers including the board view with drag‑and‑drop
//! status updates.

use axum::{
    Form, Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use peisear_core::{IssueStatus, Priority};
use peisear_storage::{issues, metrics_snapshots, project_health, projects};
use serde::Deserialize;
use validator::Validate;

use crate::{
    AppError, AppResult, AppState,
    components::{self, Column},
    extractors::AuthUser,
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
    let project = projects::find_accessible(&state.db, &project_id, &user.id).await?;
    let all_issues = issues::list_in_project(&state.db, &project_id).await?;
    let assignees = issues::list_assignee_candidates(&state.db, &project_id).await?;
    let workload = issues::project_workload(&state.db, &project_id).await?;
    let raw_health = project_health::for_project(&state.db, &project_id).await?;
    // Phase 2 trend window: 7-14 days before now. We fetch
    // snapshots in this window, take their median score as the
    // "past baseline", and report Up / Down / Flat against today.
    // An empty list (no snapshots yet — first time the project is
    // viewed, or a fresh install) yields Trend::Unavailable, which
    // the UI hides.
    let past_snapshots = metrics_snapshots::recent_for_project(
        &state.db,
        &project_id,
        peisear_core::project_health::TREND_PAST_WINDOW_MIN_DAYS,
        peisear_core::project_health::TREND_PAST_WINDOW_MAX_DAYS,
    )
    .await?;
    // The function only needs the past score values, not the
    // full ProjectHealthRaw. The denormalised score column is
    // the right input here per the design rationale (today's
    // weights aren't applied to yesterday's data).
    let past_scores: Vec<u8> = past_snapshots.iter().map(|s| s.score_value).collect();
    let health = peisear_core::project_health::compute_report_with_trend(raw_health, &past_scores);

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

    Ok(components::issues::render_project_detail(
        user, project, columns, view_mode, all_issues, assignees, workload, health, q.flash,
    ))
}

pub async fn new_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let project = projects::find_accessible(&state.db, &project_id, &user.id).await?;
    let assignees = issues::list_assignee_candidates(&state.db, &project_id).await?;
    let workload = issues::project_workload(&state.db, &project_id).await?;
    Ok(components::issues::render_issue_new(
        user,
        project,
        Priority::all().to_vec(),
        IssueStatus::all().to_vec(),
        assignees,
        workload,
        None,
    ))
}

#[derive(Debug, Deserialize, Validate)]
pub struct IssueForm {
    #[validate(length(min = 1, max = 200, message = "Title is required (max 200 chars)."))]
    pub title: String,
    #[validate(length(max = 10_000, message = "Description too long (max 10,000 chars)."))]
    pub description: String,
    pub status: String,
    pub priority: String,
    /// Effort estimate as a string from the form `<select>`. The empty
    /// string means "not estimated" (`None`); any positive integer is
    /// passed through to storage. Validation lives in [`parse_effort`]
    /// rather than `validator` derives so the empty-string case is
    /// handled cleanly.
    #[serde(default)]
    pub effort: String,
    /// User id from the assignee `<select>`. The empty string means
    /// "unassigned" (`None`). Any non-empty value must match a user
    /// who is a valid candidate for this project — see
    /// [`validate_assignee`].
    #[serde(default)]
    pub assignee_id: String,
}

/// Parse an effort string as it arrives from a browser form.
///
/// `""` (the "—" preset) → `None` (not estimated).
/// `"3"` → `Some(3)`. Negative numbers, zero, and non-numeric strings
/// are validation errors.
fn parse_effort(raw: &str) -> Result<Option<i64>, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let n: i64 = trimmed
        .parse()
        .map_err(|_| AppError::Validation("Effort must be a positive integer.".into()))?;
    if n <= 0 {
        return Err(AppError::Validation(
            "Effort must be a positive integer.".into(),
        ));
    }
    Ok(Some(n))
}

/// Validate an assignee submission against the project's candidate set.
///
/// The empty string yields `None` (unassigned). Any non-empty value
/// must appear in the candidate list returned by
/// [`peisear_storage::issues::list_assignee_candidates`] — anything
/// else is a 400, not a silent fallback. Falling back to "unassigned"
/// on an unknown id would lose user-submitted data; rejecting forces
/// the client to refresh and try again.
async fn validate_assignee(
    pool: &peisear_storage::Pool,
    project_id: &str,
    raw: &str,
) -> Result<Option<String>, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let candidates = issues::list_assignee_candidates(pool, project_id).await?;
    if candidates.iter().any(|c| c.id == trimmed) {
        Ok(Some(trimmed.to_string()))
    } else {
        Err(AppError::Validation(
            "Selected user is not a valid assignee for this project.".into(),
        ))
    }
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
    let _project = projects::find_accessible(&state.db, &project_id, &user.id).await?;

    let status = IssueStatus::parse(&form.status)
        .ok_or_else(|| AppError::Validation("Invalid status".into()))?;
    let priority = Priority::parse(&form.priority)
        .ok_or_else(|| AppError::Validation("Invalid priority".into()))?;
    let effort = parse_effort(&form.effort)?;
    let assignee_id = validate_assignee(&state.db, &project_id, &form.assignee_id).await?;

    let id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        &state.db,
        &id,
        &project_id,
        &user.id,
        form.title.trim(),
        form.description.trim(),
        status,
        priority,
        effort,
        assignee_id.as_deref(),
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
    let project = projects::find_accessible(&state.db, &project_id, &user.id).await?;
    let issue = issues::find(&state.db, &issue_id, &project_id).await?;
    let assignees = issues::list_assignee_candidates(&state.db, &project_id).await?;
    let workload = issues::project_workload(&state.db, &project_id).await?;

    // Sprint options: only when the project belongs to a team
    // and that team has planned/active sprints. Personal
    // projects skip this entirely (sprints are a team feature).
    let (sprint_options, current_sprint_id) = if let Some(team_id) = &project.team_id {
        let all = peisear_storage::sprints::list_for_team(&state.db, team_id).await?;
        let opts: Vec<(String, String)> = all
            .into_iter()
            .filter(|s| !matches!(
                s.status,
                peisear_core::sprints::SprintStatus::Completed
            ))
            .map(|s| (s.id, s.name))
            .collect();
        let cur = peisear_storage::sprints::sprint_for_issue(&state.db, &issue_id).await?;
        (opts, cur)
    } else {
        (Vec::new(), None)
    };

    Ok(components::issues::render_issue_detail(
        user,
        project,
        issue,
        Priority::all().to_vec(),
        IssueStatus::all().to_vec(),
        assignees,
        workload,
        sprint_options,
        current_sprint_id,
        q.flash,
        q.edit == Some(1),
    ))
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
    let _project = projects::find_accessible(&state.db, &project_id, &user.id).await?;

    let status = IssueStatus::parse(&form.status)
        .ok_or_else(|| AppError::Validation("Invalid status".into()))?;
    let priority = Priority::parse(&form.priority)
        .ok_or_else(|| AppError::Validation("Invalid priority".into()))?;
    let effort = parse_effort(&form.effort)?;
    let assignee_id = validate_assignee(&state.db, &project_id, &form.assignee_id).await?;

    issues::update(
        &state.db,
        &issue_id,
        &project_id,
        &user.id,
        form.title.trim(),
        form.description.trim(),
        status,
        priority,
        effort,
        assignee_id.as_deref(),
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
    let _project = projects::find_accessible(&state.db, &project_id, &user.id).await?;
    issues::delete(&state.db, &issue_id, &project_id, &user.id).await?;
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
    let _project = projects::find_accessible(&state.db, &project_id, &user.id).await?;
    let status = IssueStatus::parse(&body.status)
        .ok_or_else(|| AppError::Validation("Invalid status".into()))?;
    issues::update_status(&state.db, &issue_id, &project_id, &user.id, status).await?;
    Ok(StatusCode::NO_CONTENT)
}
