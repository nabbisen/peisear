//! Personal dashboard for the authenticated user.
//!
//! Privacy posture: the dashboard renders metrics for the
//! authenticated user only. No URL parameter exposes "any user's
//! /me" — the user identity comes from the session cookie via the
//! [`AuthUser`] extractor and goes directly into the storage query.
//!
//! The V2.1 brief (§0.3 Role-aware Visibility, §2.5 Privacy) calls
//! for distinct view scopes for the user themselves vs. managers vs.
//! a neutral observer. peisear today only has the "self" scope; the
//! manager / observer scopes arrive with the planned Team feature.

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use peisear_storage::personal_metrics;
use serde::Deserialize;

use crate::{AppResult, AppState, components, extractors::AuthUser};

#[derive(Debug, Deserialize)]
pub struct FlashQuery {
    pub flash: Option<String>,
}

pub async fn page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    // Global view (all projects this user is involved in) is the
    // default for /me. A future per-project page (`/projects/{id}/me`)
    // could call `for_user_in_project` instead, but the V2.1 brief
    // does not call for that today.
    let metrics = personal_metrics::for_user_global(&state.db, &user.id).await?;

    Ok(components::me::render_dashboard(user, metrics, q.flash))
}
