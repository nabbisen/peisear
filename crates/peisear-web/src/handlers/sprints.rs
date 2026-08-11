//! Sprint handlers (0.15.0).
//!
//! Routes mounted under `/teams/{slug}/sprints`. Listing and
//! detail are visible to all team members; create/edit/start/
//! complete/delete are admin-only.
//!
//! All sprint operations enforce team membership via
//! [`teams::role_for`]; the returned `404` rather than `403` for
//! non-members keeps the privacy posture from 0.14.0.

use axum::{
    Form,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use chrono::NaiveDate;
use peisear_core::sprints::SprintStatus;
use peisear_i18n::{Locale, MessageKey};
use peisear_storage::{notifications as notif_store, sprints, teams};
use serde::Deserialize;

use crate::{AppError, AppResult, AppState, components, extractors::AuthUser};

#[derive(Debug, Deserialize)]
pub struct FlashQuery {
    pub flash: Option<String>,
    pub error: Option<String>,
}

/// Helper: resolve team by slug, verify the user is a member,
/// and return (team, role). Errors with 404 for non-members.
async fn resolve_team_membership(
    state: &AppState,
    user_id: &str,
    slug: &str,
) -> AppResult<(peisear_core::teams::Team, peisear_core::teams::TeamRole)> {
    let team = teams::find_by_slug(&state.db, slug)
        .await?
        .ok_or(AppError::NotFound)?;
    let role = teams::role_for(&state.db, &team.id, user_id).await?;
    let Some(role) = role else {
        return Err(AppError::NotFound);
    };
    Ok((team, role))
}

pub async fn list_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    let all_sprints = sprints::list_for_team(&state.db, &team.id).await?;

    // Compute summaries for the listing. Cheap (one COUNT/SUM
    // per sprint).
    let mut sprint_summaries = Vec::with_capacity(all_sprints.len());
    for s in all_sprints {
        let sum = sprints::summary(&state.db, &s.id).await?;
        sprint_summaries.push((s, sum));
    }

    // Velocity chart data: most recent completed sprints,
    // oldest-first for left-to-right reading.
    let velocity_window = peisear_core::sprints::VELOCITY_MEDIAN_WINDOW as i64;
    let mut velocity_data =
        sprints::recent_completed_for_team(&state.db, &team.id, velocity_window).await?;
    velocity_data.reverse();

    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::sprints::render_list(
        user,
        team,
        role,
        sprint_summaries,
        velocity_data,
        unread_count,
        q.flash,
        q.error,
    ))
}

pub async fn new_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;
    Ok(components::sprints::render_new(
        user,
        team,
        unread_count,
        q.error,
    ))
}

#[derive(Debug, Deserialize)]
pub struct SprintForm {
    pub name: String,
    #[serde(default)]
    pub goal: String,
    pub starts_on: String,
    pub ends_on: String,
    /// RFC3339 timestamp captured at form render. Validated
    /// against the sprint's current `updated_at` per
    /// peisear-feature-spec-v2.1 §21.4. Default is empty string
    /// for the create flow (which doesn't have an existing row
    /// to lock against); the update handler rejects an empty
    /// value as a 400 (malformed RFC3339) so the
    /// no-hidden-input case fails closed.
    #[serde(default)]
    pub client_updated_at: String,
}

fn parse_date_required(raw: &str, field: &str) -> AppResult<NaiveDate> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(AppError::Validation(format!("{field} is required.")));
    }
    NaiveDate::parse_from_str(trimmed, "%Y-%m-%d")
        .map_err(|_| AppError::Validation(format!("{field} must be in YYYY-MM-DD format.")))
}

pub async fn create(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Form(form): Form<SprintForm>,
) -> AppResult<Redirect> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let name = form.name.trim();
    if name.is_empty() {
        return Err(AppError::Validation("Sprint name is required.".into()));
    }
    let goal = if form.goal.trim().is_empty() {
        None
    } else {
        Some(form.goal.trim())
    };
    let starts_on = parse_date_required(&form.starts_on, "Start date")?;
    let ends_on = parse_date_required(&form.ends_on, "End date")?;

    match sprints::insert(&state.db, &team.id, name, goal, starts_on, ends_on).await {
        Ok(id) => {
            let flash = Locale::English
                .render(MessageKey::SprintCreatedFlash)
                .replace(' ', "+");
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{id}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(msg)),
        Err(e) => Err(e.into()),
    }
}

pub async fn detail(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }

    let summary = sprints::summary(&state.db, &sprint.id).await?;
    let issues = sprints::issues_in_sprint(&state.db, &sprint.id).await?;
    let burndown = sprints::burndown(&state.db, &sprint.id).await?;
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::sprints::render_detail(
        user,
        team,
        role,
        sprint,
        summary,
        issues,
        burndown,
        unread_count,
        q.flash,
        q.error,
    ))
}

pub async fn edit_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;
    Ok(components::sprints::render_edit(
        user,
        team,
        sprint,
        unread_count,
        q.error,
    ))
}

pub async fn update(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Form(form): Form<SprintForm>,
) -> AppResult<Redirect> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }

    // Optimistic-lock check (peisear-feature-spec-v2.1 §21.4).
    // The sprint we just fetched carries the canonical
    // `updated_at`; compare it against the form's hidden input
    // before any state-mutating SQL.
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        sprint.updated_at,
        "sprint",
        &sprint_id,
    )?;

    let name = form.name.trim();
    if name.is_empty() {
        return Err(AppError::Validation("Sprint name is required.".into()));
    }
    let goal = if form.goal.trim().is_empty() {
        None
    } else {
        Some(form.goal.trim())
    };
    let starts_on = parse_date_required(&form.starts_on, "Start date")?;
    let ends_on = parse_date_required(&form.ends_on, "End date")?;

    match sprints::update(&state.db, &sprint.id, name, goal, starts_on, ends_on).await {
        Ok(()) => {
            let flash = Locale::English
                .render(MessageKey::SprintUpdatedFlash)
                .replace(' ', "+");
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{sprint_id}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(msg)),
        Err(e) => Err(e.into()),
    }
}

/// Body for non-edit lifecycle actions (start, complete,
/// delete) that need to carry the lock value but don't have
/// other fields. Keeping this as a separate struct from
/// `SprintForm` keeps the validator-derive surface narrow.
#[derive(Debug, Deserialize, Default)]
pub struct LifecycleForm {
    #[serde(default)]
    pub client_updated_at: String,
}

pub async fn start(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Form(form): Form<LifecycleForm>,
) -> AppResult<Redirect> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        sprint.updated_at,
        "sprint",
        &sprint_id,
    )?;
    match sprints::start(&state.db, &sprint.id).await {
        Ok(()) => {
            let flash = Locale::English
                .render(MessageKey::SprintStartedFlash)
                .replace(' ', "+");
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{sprint_id}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Conflict(msg)) => {
            let encoded = percent_encode_query(&msg);
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{sprint_id}?error={encoded}"
            )))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(msg)),
        Err(e) => Err(e.into()),
    }
}

pub async fn complete(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Form(form): Form<LifecycleForm>,
) -> AppResult<Redirect> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        sprint.updated_at,
        "sprint",
        &sprint_id,
    )?;
    match sprints::complete(&state.db, &sprint.id).await {
        Ok(()) => {
            let flash = Locale::English
                .render(MessageKey::SprintCompletedFlash)
                .replace(' ', "+");
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{sprint_id}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(msg)),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_sprint(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Form(form): Form<LifecycleForm>,
) -> AppResult<Redirect> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_manage_team() {
        return Err(AppError::Forbidden);
    }
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        sprint.updated_at,
        "sprint",
        &sprint_id,
    )?;
    sprints::delete(&state.db, &sprint.id).await?;
    let flash = Locale::English
        .render(MessageKey::SprintDeletedFlash)
        .replace(' ', "+");
    Ok(Redirect::to(&format!(
        "/teams/{slug}/sprints?flash={flash}"
    )))
}

#[derive(Debug, Deserialize)]
pub struct AssignIssueForm {
    /// Empty string means "unassign from any sprint".
    #[serde(default)]
    pub sprint_id: String,
}

/// Used from the issue detail page to set or clear the sprint
/// for one issue. The form's action target is
/// `/projects/{project_id}/issues/{issue_id}/sprint`. We
/// resolve the project, then the team, then verify membership
/// (write capability required).
pub async fn assign_issue(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((project_id, issue_id)): Path<(String, String)>,
    Form(form): Form<AssignIssueForm>,
) -> AppResult<Redirect> {
    // Verify the issue's project is accessible to the user.
    let project =
        peisear_storage::projects::find_accessible(&state.db, &project_id, &user.id).await?;

    // Phase C PR1 (peisear-feature-spec-v2.1 §8.5): sub-issues
    // follow the parent's sprint. Reject explicit sprint
    // assignment on a sub-issue — the only correct flow is to
    // change the parent's sprint, which propagates via
    // `sprint_for_issue`.
    let issue = peisear_storage::issues::find(&state.db, &issue_id, &project_id).await?;
    if issue.is_sub_issue() {
        return Err(AppError::Validation(
            "Sub-issues follow the parent's sprint. Change the parent's sprint instead."
                .to_string(),
        ));
    }

    // Personal projects (team_id None) can't have sprints
    // assigned, since sprints are team-scoped.
    let team_id = project.team_id.clone().ok_or_else(|| {
        AppError::Validation("Sprints are a team feature; this is a personal project.".into())
    })?;

    let role = teams::role_for(&state.db, &team_id, &user.id).await?;
    let Some(role) = role else {
        return Err(AppError::Forbidden);
    };
    if !role.can_write() {
        return Err(AppError::Forbidden);
    }

    // Optimistic-lock note: this endpoint mutates the
    // `sprint_issues` join table, not the issue or the
    // sprint. Since neither the issue's `updated_at` nor the
    // sprint's `updated_at` reflects this change (the join is
    // separate), there's no natural lock value for the
    // join-row contention pattern. We accept the looser
    // semantics here — concurrent sprint reassignment of the
    // same issue is rare in practice, and the last write
    // wins; the resulting state is a coherent (issue, sprint)
    // pair either way. If this proves problematic, add a
    // `version` or `updated_at` column to `sprint_issues`
    // and check it here.

    let sprint_id_trimmed = form.sprint_id.trim();
    if sprint_id_trimmed.is_empty() {
        // Unassign.
        sprints::remove_issue(&state.db, &issue_id).await?;
    } else {
        // Verify the sprint belongs to this team.
        let sprint = sprints::find_by_id(&state.db, sprint_id_trimmed)
            .await?
            .ok_or(AppError::NotFound)?;
        if sprint.team_id != team_id {
            return Err(AppError::Validation(
                "Sprint and project must belong to the same team.".into(),
            ));
        }
        // Refuse to assign to a completed sprint — historical
        // sprint summaries should remain stable.
        if matches!(sprint.status, SprintStatus::Completed) {
            return Err(AppError::Validation(
                "Cannot assign issues to a completed sprint.".into(),
            ));
        }
        sprints::add_issue(&state.db, &sprint.id, &issue_id).await?;
    }
    let flash = Locale::English
        .render(MessageKey::SprintAssignmentSavedFlash)
        .replace(' ', "+");
    Ok(Redirect::to(&format!(
        "/projects/{project_id}/issues/{issue_id}?flash={flash}"
    )))
}

fn percent_encode_query(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char);
            }
            b' ' => out.push('+'),
            other => out.push_str(&format!("%{:02X}", other)),
        }
    }
    out
}
