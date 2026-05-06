//! Issue CRUD handlers including the board view with drag‑and‑drop
//! status updates.

use axum::{
    Form, Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use peisear_core::{IssueStatus, Priority};
use peisear_storage::{issues, metrics_snapshots, project_health, projects, view_states};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppError, AppResult, AppState,
    components::{self, Column},
    extractors::AuthUser,
};

/// Query parameters for the project detail page.
///
/// `view` selects board vs list view (existing). `status`,
/// `assignee`, and `sort` are Phase A Step 3 additions: they
/// drive filter and sort on the list view, persist as the
/// user's server-side default for that project, and round-trip
/// through breadcrumb back-links so the user returns to the
/// same filtered view.
///
/// All filter/sort fields are `Option<String>`. The interpretation
/// is:
///
/// - `None` (parameter not in URL) → fall back to the server-
///   stored default; if no default is stored, fall back to the
///   factory default declared in `factory_default()` below.
/// - `Some("")` (parameter present but empty, e.g. user picked
///   "all" from a dropdown that submits an empty value) → treat
///   as "no constraint" / "factory default".
/// - `Some(value)` → that value is the active filter/sort.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProjectViewQuery {
    pub view: Option<String>,
    pub status: Option<String>,
    pub assignee: Option<String>,
    pub sort: Option<String>,
    pub flash: Option<String>,
}

impl ProjectViewQuery {
    /// True if any filter/sort parameter (i.e. anything other than
    /// `view` and `flash`) was explicitly provided in the URL.
    /// When this is true we save the state as the user's new
    /// server-side default; when false we *load* the default and
    /// don't touch it.
    fn has_explicit_filter_or_sort(&self) -> bool {
        self.status.is_some() || self.assignee.is_some() || self.sort.is_some()
    }

    /// Merge query parameters with a previously-saved state.
    /// URL parameters always win where present; missing
    /// parameters inherit from `saved`. This is the
    /// "URL primary, server default secondary" merge described in
    /// peisear-feature-spec-v2.1 §4.4 / appendix decision A-3 = C.
    fn merge_with_saved(self, saved: ProjectViewQuery) -> ProjectViewQuery {
        ProjectViewQuery {
            view: self.view.or(saved.view),
            status: self.status.or(saved.status),
            assignee: self.assignee.or(saved.assignee),
            sort: self.sort.or(saved.sort),
            // `flash` is intentionally NOT inherited — it's a
            // one-shot UI message, not user preference.
            flash: self.flash,
        }
    }

    /// The shape we serialise into `user_view_states.state_json`.
    /// Only the persistence-worthy fields. `flash` and `view` are
    /// omitted: `flash` is one-shot, and `view` (board vs list)
    /// is already remembered by the user on a per-link basis via
    /// the breadcrumbs / nav buttons.
    fn to_persisted_json(&self) -> String {
        // Use a tiny anonymous type so we never accidentally
        // persist `flash` or `view`.
        #[derive(Serialize)]
        struct Persisted<'a> {
            status: Option<&'a str>,
            assignee: Option<&'a str>,
            sort: Option<&'a str>,
        }
        let p = Persisted {
            status: self.status.as_deref(),
            assignee: self.assignee.as_deref(),
            sort: self.sort.as_deref(),
        };
        // serde_json::to_string can't fail for a struct of
        // Option<&str>, but we still default defensively to "{}"
        // so a future schema bump doesn't crash live traffic.
        serde_json::to_string(&p).unwrap_or_else(|_| "{}".to_string())
    }

    /// Parse JSON we pulled from `user_view_states.state_json`.
    /// Returns a default-empty state if parsing fails — a corrupt
    /// row should not break the page render. The render will then
    /// fall back to factory defaults via `merge_with_saved`.
    fn from_persisted_json(json: &str) -> ProjectViewQuery {
        #[derive(Deserialize)]
        struct Persisted {
            #[serde(default)]
            status: Option<String>,
            #[serde(default)]
            assignee: Option<String>,
            #[serde(default)]
            sort: Option<String>,
        }
        match serde_json::from_str::<Persisted>(json) {
            Ok(p) => ProjectViewQuery {
                view: None,
                status: p.status,
                assignee: p.assignee,
                sort: p.sort,
                flash: None,
            },
            Err(_) => ProjectViewQuery::default(),
        }
    }
}

/// The set of valid sort orderings on the list view. Centralised
/// so the handler, component, and tests share the same vocabulary.
const SORT_PRIORITY: &str = "priority";
const SORT_CREATED: &str = "created";
const SORT_UPDATED: &str = "updated";

/// Apply `query.status`, `query.assignee`, and `query.sort` to a
/// raw issue list pulled from the DB. Returns a new Vec —
/// allocation cost is negligible at the team sizes peisear targets
/// (5–30 members), and an in-memory pass keeps the storage layer
/// out of UI concerns.
///
/// `status = ""` or unset → no status filter.
/// `assignee = "unassigned"` → only issues with no assignee.
/// `assignee = "<id>"` → only that user's issues.
/// `assignee = ""` or unset → no assignee filter.
/// `sort = "priority"` → urgent first, then high, medium, low,
///   stable on prior order within ties (so the existing
///   `status ASC, position ASC` remains as a tiebreaker).
/// `sort = "created"` → newest first.
/// `sort = "updated"` → most-recently-updated first.
/// Unknown / missing → preserve the storage-layer default
///   ordering (status ASC, position ASC, created_at DESC).
fn apply_filter_and_sort(
    mut issues: Vec<peisear_core::Issue>,
    query: &ProjectViewQuery,
) -> Vec<peisear_core::Issue> {
    if let Some(s) = query.status.as_deref().filter(|s| !s.is_empty()) {
        issues.retain(|i| i.status.as_str() == s);
    }
    if let Some(a) = query.assignee.as_deref().filter(|s| !s.is_empty()) {
        if a == "unassigned" {
            issues.retain(|i| i.assignee_id.is_none());
        } else {
            let a_owned = a.to_string();
            issues.retain(|i| i.assignee_id.as_deref() == Some(&a_owned));
        }
    }
    match query.sort.as_deref() {
        Some(SORT_PRIORITY) => {
            // Stable sort preserves the storage-default order
            // among issues with equal priority.
            issues.sort_by_key(|i| match i.priority {
                Priority::Urgent => 0u8,
                Priority::High => 1,
                Priority::Medium => 2,
                Priority::Low => 3,
            });
        }
        Some(SORT_CREATED) => {
            issues.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        }
        Some(SORT_UPDATED) => {
            issues.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        }
        _ => {} // unknown/none → keep storage-layer order
    }
    issues
}

/// Project detail page: renders either board (kanban) or list view.
///
/// Implements the URL-primary / server-default-secondary
/// filter+sort scheme from peisear-feature-spec-v2.1 §4.4:
///
/// 1. The handler reads `q` from the URL.
/// 2. It loads the user's saved default for this view (if any).
/// 3. It merges them: every URL-supplied field wins; fields the
///    URL omitted inherit from the saved default.
/// 4. It applies the merged filter/sort to the issue list before
///    rendering.
/// 5. If the URL itself supplied any filter/sort field, the
///    merged state is upserted as the user's new saved default.
///    A bare URL (no filter/sort) does NOT overwrite the saved
///    state — that would erase the user's preference every time
///    they navigated via a generic link.
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

    // Step 3: merge URL with server-saved default.
    let view_key = view_states::project_issues_key(&project_id);
    let url_had_explicit = q.has_explicit_filter_or_sort();
    let saved = match view_states::get(&state.db, &user.id, &view_key).await? {
        Some(json) => ProjectViewQuery::from_persisted_json(&json),
        None => ProjectViewQuery::default(),
    };
    let merged = q.merge_with_saved(saved);

    // Persist the merged state IF the URL contributed something.
    // A user navigating via a link without filter/sort must NOT
    // erase their previously chosen default.
    if url_had_explicit {
        let to_save = merged.to_persisted_json();
        view_states::upsert(&state.db, &user.id, &view_key, &to_save).await?;
    }

    let filtered_issues = apply_filter_and_sort(all_issues.clone(), &merged);

    let mut columns: Vec<Column> = IssueStatus::all()
        .into_iter()
        .map(|s| Column {
            status: s,
            issues: Vec::new(),
        })
        .collect();
    // Board view's columns are NOT filtered — the board is the
    // user's mental map of all the work in the project, and
    // hiding columns based on a `status` filter would be
    // surprising. We apply filter only to the list view.
    for issue in &all_issues {
        if let Some(col) = columns.iter_mut().find(|c| c.status == issue.status) {
            col.issues.push(issue.clone());
        }
    }

    let view_mode = match merged.view.as_deref() {
        Some("list") => "list".to_string(),
        _ => "board".to_string(),
    };

    Ok(components::issues::render_project_detail(
        user,
        project,
        columns,
        view_mode,
        filtered_issues,
        assignees,
        workload,
        health,
        merged.flash,
        merged.status.unwrap_or_default(),
        merged.assignee.unwrap_or_default(),
        merged.sort.unwrap_or_default(),
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
    /// RFC3339 timestamp captured when the edit form was rendered.
    /// Validated against the issue's current `updated_at` to detect
    /// concurrent edits per peisear-feature-spec-v2.1 §21.4. Optional
    /// only for backwards compat during the Phase A rollout window —
    /// after Step 5 lands fully, every form template embeds this as
    /// a hidden input. A missing value triggers an explicit conflict
    /// rather than silently bypassing the check.
    #[serde(default)]
    pub client_updated_at: String,
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

    // Optimistic lock check (peisear-feature-spec-v2.1 §21.4).
    // We re-read the issue here for two reasons:
    //
    // 1. To get the canonical `updated_at` against which to
    //    compare the form's `client_updated_at`. Reading after
    //    the access check (rather than reusing a value from the
    //    page-render path) ensures we catch even rapid
    //    edit-edit-edit sequences from a single tab.
    //
    // 2. To produce a 404 if the row was deleted between the
    //    page render and the form submit, before we get to the
    //    update query.
    let issue_now = issues::find(&state.db, &issue_id, &project_id).await?;
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        issue_now.updated_at,
        "issue",
        &issue_id,
    )?;

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
    /// RFC3339 timestamp captured at page render. The kanban
    /// JS reads it from the card's `data-updated-at` attribute
    /// and includes it in the JSON body. Validated against the
    /// issue's current `updated_at` per §21.4. Optional during
    /// the Phase A rollout window — once the JS embeds it for
    /// every card, missing should be a 400.
    #[serde(default)]
    pub client_updated_at: String,
}

pub async fn change_status(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path((project_id, issue_id)): Path<(String, String)>,
    Json(body): Json<StatusChange>,
) -> AppResult<StatusCode> {
    let _project = projects::find_accessible(&state.db, &project_id, &user.id).await?;

    // Optimistic lock check. We accept an empty
    // client_updated_at during the Phase A rollout (kanban JS
    // hasn't been updated yet) but emit a tracing line so we
    // can spot whether real traffic is sending it.
    if body.client_updated_at.is_empty() {
        tracing::debug!(
            %issue_id,
            "kanban status change without client_updated_at \
             (Phase A rollout: tracked, allowed)"
        );
    } else {
        let issue_now = issues::find(&state.db, &issue_id, &project_id).await?;
        crate::error::check_optimistic_lock(
            &body.client_updated_at,
            issue_now.updated_at,
            "issue",
            &issue_id,
        )?;
    }

    let status = IssueStatus::parse(&body.status)
        .ok_or_else(|| AppError::Validation("Invalid status".into()))?;
    issues::update_status(&state.db, &issue_id, &project_id, &user.id, status).await?;
    Ok(StatusCode::NO_CONTENT)
}
