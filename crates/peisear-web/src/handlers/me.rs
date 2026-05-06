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
use peisear_storage::{personal_metrics, user_burnout, user_capacities};
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

    // Phase 2 burnout signals (0.10.0). Returned as
    // `Option<UserBurnoutSignals>` — `None` only if the user
    // doesn't exist (which can't happen here because the auth
    // extractor already loaded them) but we propagate the option
    // for symmetry. The component decides whether to render the
    // panel based on whether there are any meaningful values.
    let burnout = user_burnout::for_user(&state.db, &user.id).await?;

    // 0.12.0: figure out whether today's effective capacity comes
    // from a period-bounded row, so the Load chip can render a
    // small "(this period)" hint. We pull the row (not just the
    // points) and check whether either bound is set.
    let capacity_row = user_capacities::effective_row_for_user(&state.db, &user.id).await?;
    let capacity_is_period_bounded = capacity_row
        .as_ref()
        .map(|r| r.period_start.is_some() || r.period_end.is_some())
        .unwrap_or(false);

    Ok(components::me::render_dashboard(
        user,
        metrics,
        burnout,
        capacity_is_period_bounded,
        q.flash,
    ))
}
