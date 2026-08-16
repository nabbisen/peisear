//! Global search handlers (Phase A Step 4, peisear-feature-spec
//! v2.1 §4.5).
//!
//! Two endpoints share one search core:
//!
//! - `GET /search?q=...&page=N` — HTML results page. Used by
//!   form submission from the navbar input box, by direct URL
//!   navigation, and by the user pressing Enter in the
//!   typeahead.
//! - `GET /api/search?q=...` — JSON typeahead. Used by the
//!   navbar input box's `input` event handler (vanilla JS,
//!   `static/search.js`).
//!
//! Both honour the spec's authorization invariants: only
//! projects the requesting user can access, and only open
//! issues within those projects, are returned.
//!
//! ## Result counts
//!
//! Per A-7 in the v2.1 session record:
//!
//! - typeahead: 8 hits total, balanced 4 projects + 4 issues.
//!   When one category has fewer matches, the other category
//!   takes the slack up to 8 total.
//! - results page: 50 per page, paginated.

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use peisear_storage::search;
use serde::{Deserialize, Serialize};

use crate::{
    ApiAppResult, AppResult, AppState, components,
    extractors::{ApiAuthUser, AuthUser},
};

/// Query parameters for both `/search` and `/api/search`.
///
/// `q` is the literal user input. Empty / whitespace-only `q`
/// produces empty result sets (we don't try to "show all" — the
/// list views are the right surface for that).
///
/// `page` is 1-indexed, defaulting to 1. Bounded server-side to
/// prevent absurd offsets exhausting the DB.
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub page: Option<u32>,
}

/// Number of hits returned by typeahead — small budget across
/// projects + issues, since this is a popover that has to fit
/// on screen without scrolling.
const TYPEAHEAD_TOTAL: i64 = 8;

/// Per-page count on the results page.
const RESULTS_PER_PAGE: i64 = 50;

/// Hard cap on `page` so a malicious or buggy caller can't
/// trigger an arbitrarily large `OFFSET` scan.
const MAX_PAGE: u32 = 200;

/// JSON shape for typeahead. Lean — only the fields needed for
/// the dropdown row + click target.
#[derive(Debug, Serialize)]
pub struct TypeaheadResponse {
    pub projects: Vec<TypeaheadProject>,
    pub issues: Vec<TypeaheadIssue>,
    /// Echo the query back so the client can ignore stale
    /// responses if the user has since typed more characters
    /// (race-condition guard for the typeahead).
    pub q: String,
}

#[derive(Debug, Serialize)]
pub struct TypeaheadProject {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct TypeaheadIssue {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub title: String,
    pub url: String,
}

/// JSON typeahead handler. Returns up to 8 hits total, split
/// roughly evenly between project and issue matches.
///
/// Returns an empty payload (not 400) for empty / whitespace-only
/// queries. This keeps the client-side logic simple: every
/// keystroke goes to the same endpoint, the server returns
/// nothing for "no real query", and the dropdown closes.
pub async fn typeahead(
    ApiAuthUser(user): ApiAuthUser,
    State(state): State<AppState>,
    Query(q): Query<SearchQuery>,
) -> ApiAppResult<axum::Json<TypeaheadResponse>> {
    let raw = q.q.unwrap_or_default();
    let trimmed = raw.trim();

    if trimmed.is_empty() {
        return Ok(axum::Json(TypeaheadResponse {
            projects: Vec::new(),
            issues: Vec::new(),
            q: raw,
        }));
    }

    // Fetch the budget (4) for each category. If one has fewer
    // matches, top up from the other up to 8. This is symmetric
    // and predictable; no ranking is involved (per A-4 = A:
    // "creation/update order, ranking is Phase 2").
    let half = TYPEAHEAD_TOTAL / 2;
    let project_hits =
        search::projects_by_name(&state.db, &user.id, trimmed, TYPEAHEAD_TOTAL).await?;
    let issue_hits =
        search::open_issues_by_title(&state.db, &user.id, trimmed, TYPEAHEAD_TOTAL).await?;

    let project_count = project_hits.len() as i64;
    let issue_count = issue_hits.len() as i64;
    // Each category claims up to `half` slots. Whatever's left
    // goes to the other category as overflow.
    let projects_keep = project_count.min(half);
    let issues_keep = issue_count.min(TYPEAHEAD_TOTAL - projects_keep);
    // Now back-fill: if issues didn't use all their share,
    // give projects more.
    let projects_keep = projects_keep.min(TYPEAHEAD_TOTAL - issues_keep)
        + (project_count - projects_keep)
            .min((TYPEAHEAD_TOTAL - projects_keep - issues_keep).max(0));

    let projects: Vec<TypeaheadProject> = project_hits
        .into_iter()
        .take(projects_keep as usize)
        .filter_map(|h| match h {
            search::SearchHit::Project { id, name } => {
                let url = format!("/projects/{id}");
                Some(TypeaheadProject { id, name, url })
            }
            // Should not happen — projects_by_name returns only
            // Project hits — but the enum is shared so we keep
            // the discriminant explicit.
            search::SearchHit::Issue { .. } => None,
        })
        .collect();

    let issues: Vec<TypeaheadIssue> = issue_hits
        .into_iter()
        .take(issues_keep as usize)
        .filter_map(|h| match h {
            search::SearchHit::Issue {
                id,
                project_id,
                project_name,
                title,
                // Typeahead is a compact popover with no room for
                // breadcrumb context; the parent title is a
                // results-page-only addition (`INBOX-001`).
                parent_title: _,
            } => {
                let url = format!("/projects/{project_id}/issues/{id}");
                Some(TypeaheadIssue {
                    id,
                    project_id,
                    project_name,
                    title,
                    url,
                })
            }
            search::SearchHit::Project { .. } => None,
        })
        .collect();

    Ok(axum::Json(TypeaheadResponse {
        projects,
        issues,
        q: raw,
    }))
}

/// HTML results page. Renders 50 hits per page, paginated.
///
/// We display projects first, then issues. Each section is
/// independently paginated by the `page` query parameter — a
/// future enhancement would split into per-category pages, but
/// at typical scales the user won't have more than one page of
/// either category, and the unified pagination keeps the URL
/// simple.
///
/// Implementation note: we fetch `page * RESULTS_PER_PAGE + 1`
/// rows per query so the renderer can detect "is there a next
/// page" without a second `COUNT(*)` query. The +1 row is
/// dropped before display.
pub async fn results_page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<SearchQuery>,
) -> AppResult<impl IntoResponse> {
    let raw = q.q.clone().unwrap_or_default();
    let trimmed = raw.trim().to_string();
    let page = q.page.unwrap_or(1).clamp(1, MAX_PAGE);

    if trimmed.is_empty() {
        // Empty-query landing — render the page with no hits,
        // ready for the user to type into the navbar input or
        // the on-page form.
        return Ok(components::search::render_results(
            user,
            String::new(),
            Vec::new(),
            Vec::new(),
            page,
            false,
            false,
        ));
    }

    // Fetch one extra row beyond `RESULTS_PER_PAGE * page` so we
    // can answer "is there a next page" without a second query.
    let fetch_limit = (page as i64) * RESULTS_PER_PAGE + 1;

    let project_hits = search::projects_by_name(&state.db, &user.id, &trimmed, fetch_limit).await?;
    let issue_hits =
        search::open_issues_by_title(&state.db, &user.id, &trimmed, fetch_limit).await?;

    // Slice to the requested page window.
    let offset = ((page - 1) as i64 * RESULTS_PER_PAGE) as usize;
    let projects_window: Vec<_> = project_hits
        .iter()
        .skip(offset)
        .take(RESULTS_PER_PAGE as usize)
        .cloned()
        .collect();
    let issues_window: Vec<_> = issue_hits
        .iter()
        .skip(offset)
        .take(RESULTS_PER_PAGE as usize)
        .cloned()
        .collect();
    let has_more_projects = project_hits.len() > offset + projects_window.len();
    let has_more_issues = issue_hits.len() > offset + issues_window.len();

    Ok(components::search::render_results(
        user,
        trimmed,
        projects_window,
        issues_window,
        page,
        has_more_projects,
        has_more_issues,
    ))
}
