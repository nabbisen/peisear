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
use peisear_core::Priority;
use peisear_core::sprints::SprintStatus;
use peisear_i18n::{Field, Locale, MessageKey};
use peisear_storage::{issues, notifications as notif_store, projects, sprints, teams};
use serde::Deserialize;

use crate::{AppError, AppResult, AppState, components, components::t, extractors::AuthUser};

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

    // `QA-017` (`NFR-PRIV-007`): the predicate is distinct
    // contributors *across the whole window*, not per sprint — five
    // solo sprints by the same person is still one person, so this
    // counts over the union of every sprint id in `velocity_data`, not
    // once per sprint. The bars themselves (already-visible per-sprint
    // totals) are unaffected either way; only the median line is
    // gated.
    let velocity_sprint_ids: Vec<String> =
        velocity_data.iter().map(|(s, _)| s.id.clone()).collect();
    let velocity_contributors =
        sprints::distinct_contributors(&state.db, &velocity_sprint_ids).await?;
    let show_median = matches!(velocity_contributors, Some(n) if n >= 2);

    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::sprints::render_list(
        user,
        team,
        role,
        sprint_summaries,
        velocity_data,
        show_median,
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

fn parse_date_required(raw: &str, field: Field) -> AppResult<NaiveDate> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(AppError::Validation(t(MessageKey::FieldRequired { field })));
    }
    NaiveDate::parse_from_str(trimmed, "%Y-%m-%d")
        .map_err(|_| AppError::Validation(t(MessageKey::FieldMustBeDateFormat { field })))
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
        return Err(AppError::Validation(t(
            MessageKey::SprintNameRequiredMessage,
        )));
    }
    let goal = if form.goal.trim().is_empty() {
        None
    } else {
        Some(form.goal.trim())
    };
    let starts_on = parse_date_required(&form.starts_on, Field::StartDate)?;
    let ends_on = parse_date_required(&form.ends_on, Field::EndDate)?;

    match sprints::insert(&state.db, &team.id, name, goal, starts_on, ends_on).await {
        Ok(id) => {
            let flash = super::percent_encode_query(
                &Locale::English.render(MessageKey::SprintCreatedFlash),
            );
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{id}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(t(msg))),
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
    // `QA-017` (`NFR-PRIV-007`): the trajectory is a per-person
    // disclosure below two distinct contributors; the sprint-end
    // totals (`summary`, above) are unaffected either way.
    let contributors =
        sprints::distinct_contributors(&state.db, std::slice::from_ref(&sprint.id)).await?;
    let show_trajectory = matches!(contributors, Some(n) if n >= 2);
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::sprints::render_detail(
        user,
        team,
        role,
        sprint,
        summary,
        issues,
        burndown,
        show_trajectory,
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
        peisear_i18n::EntityKind::Sprint,
        &sprint_id,
    )?;

    let name = form.name.trim();
    if name.is_empty() {
        return Err(AppError::Validation(t(
            MessageKey::SprintNameRequiredMessage,
        )));
    }
    let goal = if form.goal.trim().is_empty() {
        None
    } else {
        Some(form.goal.trim())
    };
    let starts_on = parse_date_required(&form.starts_on, Field::StartDate)?;
    let ends_on = parse_date_required(&form.ends_on, Field::EndDate)?;

    match sprints::update(&state.db, &sprint.id, name, goal, starts_on, ends_on).await {
        Ok(()) => {
            let flash = super::percent_encode_query(
                &Locale::English.render(MessageKey::SprintUpdatedFlash),
            );
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{sprint_id}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(t(msg))),
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
        peisear_i18n::EntityKind::Sprint,
        &sprint_id,
    )?;
    match sprints::start(&state.db, &sprint.id).await {
        Ok(()) => {
            let flash = super::percent_encode_query(
                &Locale::English.render(MessageKey::SprintStartedFlash),
            );
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{sprint_id}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Conflict(msg)) => {
            let encoded = super::percent_encode_query(&t(msg));
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{sprint_id}?error={encoded}"
            )))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(t(msg))),
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
        peisear_i18n::EntityKind::Sprint,
        &sprint_id,
    )?;
    match sprints::complete(&state.db, &sprint.id).await {
        Ok(()) => {
            let flash = super::percent_encode_query(
                &Locale::English.render(MessageKey::SprintCompletedFlash),
            );
            Ok(Redirect::to(&format!(
                "/teams/{slug}/sprints/{sprint_id}?flash={flash}"
            )))
        }
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(t(msg))),
        Err(e) => Err(e.into()),
    }
}

/// `CONF-001`: the confirmation interstitial, `GET`. Same
/// authorisation as [`delete_sprint`]'s `POST` —
/// `role.can_manage_team()` plus the sprint belonging to this team.
///
/// One route serves both `Planned` and `Completed` sprints; the
/// difference is the consequence copy, not the check above.
///
/// `QA-002` item 1: an `Active` sprint refuses here too, before
/// rendering anything. `CONF-001`'s review found this route (and
/// `delete_sprint`'s `POST`) live for an active sprint despite the UI
/// offering no delete control for one — rendering "you are about to
/// delete *X*" for a team's running sprint and then letting the
/// `POST` do it. At most one sprint per team is active; deleting it
/// is not equivalent to deleting a planned one.
pub async fn delete_confirm(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
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
    if matches!(sprint.status, SprintStatus::Active) {
        return Err(AppError::Validation(
            Locale::English.render(MessageKey::SprintActiveCannotBeDeletedMessage),
        ));
    }

    let consequence_key = match sprint.status {
        SprintStatus::Planned => MessageKey::ConfirmDeleteSprintPlannedNote,
        SprintStatus::Completed => MessageKey::ConfirmDeleteSprintCompletedNote,
        // Unreachable today — the guard above already returned for
        // `Active` — but `QA-002-review.md` §4.3: the failure mode
        // matters more than today's reachability. Returning the same
        // refusal here (rather than `unreachable!`) means a future
        // reorder of this function degrades to "still refuses", not
        // "panics into a 500". Defence in depth: removing either this
        // arm or the guard above leaves the route correct.
        SprintStatus::Active => {
            return Err(AppError::Validation(
                Locale::English.render(MessageKey::SprintActiveCannotBeDeletedMessage),
            ));
        }
    };
    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::confirmation::render_delete_confirmation(
        user,
        sprint.name,
        Locale::English.render(consequence_key),
        format!("/teams/{slug}/sprints/{sprint_id}/delete"),
        format!("/teams/{slug}/sprints"),
        vec![(
            "client_updated_at".to_string(),
            sprint.updated_at.to_rfc3339(),
        )],
        unread_count,
    ))
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
    // `QA-002` item 1: a state constraint, not an authorisation
    // failure — 400, not 403. The caller may well be a team admin
    // perfectly entitled to delete a *different* sprint.
    if matches!(sprint.status, SprintStatus::Active) {
        return Err(AppError::Validation(
            Locale::English.render(MessageKey::SprintActiveCannotBeDeletedMessage),
        ));
    }
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        sprint.updated_at,
        peisear_i18n::EntityKind::Sprint,
        &sprint_id,
    )?;
    sprints::delete(&state.db, &sprint.id).await?;
    let flash =
        super::percent_encode_query(&Locale::English.render(MessageKey::SprintDeletedFlash));
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
        return Err(AppError::Validation(t(
            MessageKey::SubIssueFollowsParentSprintMessage,
        )));
    }

    // Personal projects (team_id None) can't have sprints
    // assigned, since sprints are team-scoped.
    let team_id = project
        .team_id
        .clone()
        .ok_or_else(|| AppError::Validation(t(MessageKey::SprintsPersonalProjectMessage)))?;

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
            return Err(AppError::Validation(t(
                MessageKey::SprintProjectTeamMismatchMessage,
            )));
        }
        // Refuse to assign to a completed sprint — historical
        // sprint summaries should remain stable.
        if matches!(sprint.status, SprintStatus::Completed) {
            return Err(AppError::Validation(t(
                MessageKey::CannotAssignToCompletedSprintMessage,
            )));
        }
        sprints::add_issue(&state.db, &sprint.id, &issue_id).await?;
    }
    let flash = super::percent_encode_query(
        &Locale::English.render(MessageKey::SprintAssignmentSavedFlash),
    );
    Ok(Redirect::to(&format!(
        "/projects/{project_id}/issues/{issue_id}?flash={flash}"
    )))
}

// ──────────────────────────────────────────────────────────────
// Sprint planning page (`PLAN-001` / RFC 001)
// ──────────────────────────────────────────────────────────────

/// Backlog filter, mirrored from the URL query string. `Some("")`
/// (a dropdown's "no constraint" option submitting an empty value)
/// is treated as `None` at the point this is turned into a
/// [`sprints::BacklogFilter`] — same convention `ProjectViewQuery`
/// uses elsewhere in this crate.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct PlanQuery {
    pub project: Option<String>,
    pub priority: Option<String>,
    pub assignee: Option<String>,
}

/// Builds the `?project=&priority=&assignee=` suffix the two POST
/// handlers redirect back to (RFC 001: "Both POSTs redirect (303)
/// back to the GET with the same filter query so the planner stays
/// in context after each move"). Values here are always UUIDs, the
/// literal `"unassigned"`, or a `Priority::as_str()` word, so this
/// was one of the three raw-interpolation redirect sites `QA-020`
/// found safe today only because of that — not because anything
/// enforced it. Routed through `percent_encode_query` anyway, same
/// as the other 28 sites: the point of one encoder everywhere is
/// that a site's safety stops depending on what its argument
/// happens to be shaped like today.
fn plan_query_string(
    project: &Option<String>,
    priority: &Option<String>,
    assignee: &Option<String>,
) -> String {
    let mut parts = Vec::new();
    if let Some(v) = project.as_deref().filter(|s| !s.is_empty()) {
        parts.push(format!("project={}", super::percent_encode_query(v)));
    }
    if let Some(v) = priority.as_deref().filter(|s| !s.is_empty()) {
        parts.push(format!("priority={}", super::percent_encode_query(v)));
    }
    if let Some(v) = assignee.as_deref().filter(|s| !s.is_empty()) {
        parts.push(format!("assignee={}", super::percent_encode_query(v)));
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!("?{}", parts.join("&"))
    }
}

pub async fn plan_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Query(q): Query<PlanQuery>,
) -> AppResult<impl IntoResponse> {
    // GET is open to any team member, including `viewer` (handoff
    // §2.2) — no `can_write()` gate here, unlike every mutating
    // route below.
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }

    // Review correction (PLAN-001-review §3.2): three shapes, not
    // two. `can_move` (move forms in either column) requires both
    // `can_write()` and a planned sprint — a viewer or an active
    // sprint both suppress it, for different reasons. `show_backlog`
    // is a separate axis: the backlog stays visible on an active
    // sprint (a member is still reading a live plan and needs to see
    // what isn't committed yet) and only disappears once the sprint
    // is completed ("re-opening a completed sprint to add issues is
    // not a flow we support" — RFC 001's own reasoning, which the
    // first cut of this handler wrongly folded into the same flag as
    // `can_move`).
    let can_move = role.can_write() && matches!(sprint.status, SprintStatus::Planned);
    let show_backlog = !matches!(sprint.status, SprintStatus::Completed);

    let summary = sprints::summary(&state.db, &sprint.id).await?;
    let sprint_items = sprints::issues_in_sprint(&state.db, &sprint.id).await?;
    let sprint_item_ids: std::collections::HashSet<&str> =
        sprint_items.iter().map(|(id, ..)| id.as_str()).collect();

    // Backlog scope: RFC 001 open question 1's default is
    // team-scoped projects only (`backlog_for_team` already applies
    // that). This page additionally subtracts the current sprint's
    // own items — they're already shown on the right, and
    // `backlog_for_team`'s own "not in any active sprint" rule (see
    // its doc comment) doesn't know about *this* sprint specifically
    // when it isn't active yet.
    let filter = sprints::BacklogFilter {
        project_id: q.project.clone().filter(|s| !s.is_empty()),
        priority: q.priority.as_deref().and_then(Priority::parse),
        assignee_id: q.assignee.clone().filter(|s| !s.is_empty()),
    };
    let backlog: Vec<_> = sprints::backlog_for_team(&state.db, &team.id, filter)
        .await?
        .into_iter()
        .filter(|row| !sprint_item_ids.contains(row.issue.id.as_str()))
        .collect();

    // Filter dropdown data. The assignee list comes from the same
    // place the issue form's does (handoff §2.5) — `CANDIDATE_SET_CTE`
    // is project-scoped (RFC 009 §D1), so a team-wide list is the
    // union across the team's projects, deduplicated by id, rather
    // than a new team-scoped query.
    let team_projects = projects::list_for_team(&state.db, &team.id).await?;
    let mut assignees: Vec<peisear_core::AssigneeOption> = Vec::new();
    let mut seen_assignee_ids = std::collections::HashSet::new();
    for p in &team_projects {
        for a in issues::list_assignee_candidates(&state.db, &p.id).await? {
            if seen_assignee_ids.insert(a.id.clone()) {
                assignees.push(a);
            }
        }
    }
    assignees.sort_by(|a, b| a.display_name.cmp(&b.display_name));

    let unread_count = notif_store::unread_count_for_user(&state.db, &user.id).await?;

    Ok(components::sprint_plan::render_plan(
        user,
        team,
        sprint,
        summary,
        backlog,
        sprint_items,
        team_projects,
        assignees,
        q.project.unwrap_or_default(),
        q.priority.unwrap_or_default(),
        q.assignee.unwrap_or_default(),
        can_move,
        show_backlog,
        unread_count,
    ))
}

#[derive(Debug, Deserialize)]
pub struct PlanAddForm {
    pub issue_id: String,
    /// The issue's project id. Carried explicitly (rather than
    /// re-derived) so the handler can verify it names one of *this*
    /// team's own projects before trusting `issue_id` at all — a
    /// forged `project_id` from another team fails that check and
    /// never reaches [`issues::find`].
    pub project_id: String,
    #[serde(default)]
    pub project: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub assignee: Option<String>,
}

/// Move one issue from the backlog into the sprint being planned.
/// `can_write()` only (handoff §2.2); `viewer` gets 403.
pub async fn plan_add(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Form(form): Form<PlanAddForm>,
) -> AppResult<Redirect> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_write() {
        return Err(AppError::Forbidden);
    }
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }
    if !matches!(sprint.status, SprintStatus::Planned) {
        return Err(AppError::Validation(t(
            MessageKey::SprintPlanNotEditableMessage,
        )));
    }

    // Defense in depth (this crate's established convention, see
    // this file's module doc and TEAM-001): confirm `project_id`
    // genuinely names one of this team's own projects before
    // trusting the issue lookup at all.
    let team_projects = projects::list_for_team(&state.db, &team.id).await?;
    if !team_projects.iter().any(|p| p.id == form.project_id) {
        return Err(AppError::NotFound);
    }
    let issue = issues::find(&state.db, &form.issue_id, &form.project_id).await?;

    // Phase C PR1 (peisear-feature-spec-v2.1 §8.5): sub-issues
    // follow the parent's sprint and never get their own
    // `sprint_issues` row. `backlog_for_team` already excludes them
    // (`parent_issue_id IS NULL`); this rejects a forged POST that
    // names one directly, same guard `assign_issue` above applies.
    if issue.is_sub_issue() {
        return Err(AppError::Validation(t(
            MessageKey::SubIssueFollowsParentSprintMessage,
        )));
    }

    sprints::add_issue(&state.db, &sprint.id, &issue.id).await?;
    let qs = plan_query_string(&form.project, &form.priority, &form.assignee);
    Ok(Redirect::to(&format!(
        "/teams/{slug}/sprints/{sprint_id}/plan{qs}"
    )))
}

#[derive(Debug, Deserialize)]
pub struct PlanRemoveForm {
    pub issue_id: String,
    #[serde(default)]
    pub project: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub assignee: Option<String>,
}

/// Move one issue from the sprint being planned back to the
/// backlog. `can_write()` only, same as [`plan_add`].
pub async fn plan_remove(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((slug, sprint_id)): Path<(String, String)>,
    Form(form): Form<PlanRemoveForm>,
) -> AppResult<Redirect> {
    let (team, role) = resolve_team_membership(&state, &user.id, &slug).await?;
    if !role.can_write() {
        return Err(AppError::Forbidden);
    }
    let sprint = sprints::find_by_id(&state.db, &sprint_id)
        .await?
        .ok_or(AppError::NotFound)?;
    if sprint.team_id != team.id {
        return Err(AppError::NotFound);
    }
    if !matches!(sprint.status, SprintStatus::Planned) {
        return Err(AppError::Validation(t(
            MessageKey::SprintPlanNotEditableMessage,
        )));
    }

    // Defense in depth: `sprints::remove_issue` deletes by
    // `issue_id` alone with no sprint scoping, so without this check
    // a forged `issue_id` belonging to a *different* sprint (this
    // team's or another team's entirely) would be removed from
    // wherever it actually lives. Only remove if the issue is
    // currently in *this* sprint.
    let current = sprints::sprint_for_issue(&state.db, &form.issue_id).await?;
    if current.as_deref() != Some(sprint.id.as_str()) {
        return Err(AppError::NotFound);
    }

    sprints::remove_issue(&state.db, &form.issue_id).await?;
    let qs = plan_query_string(&form.project, &form.priority, &form.assignee);
    Ok(Redirect::to(&format!(
        "/teams/{slug}/sprints/{sprint_id}/plan{qs}"
    )))
}
