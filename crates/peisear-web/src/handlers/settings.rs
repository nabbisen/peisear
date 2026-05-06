//! User settings: personal capacity (story points) and personal
//! WIP limit (count of in-progress issues).

use axum::{
    Form,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use peisear_storage::users;
use serde::Deserialize;

use crate::{AppError, AppResult, AppState, components, extractors::AuthUser};

#[derive(Debug, Deserialize)]
pub struct FlashQuery {
    pub flash: Option<String>,
}

pub async fn page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    // Re-fetch the full user record so we have the current capacity
    // and WIP limit. The session-cookie extractor only carries
    // id / email / display_name.
    let full = users::find_by_id(&state.db, &user.id)
        .await?
        .ok_or(AppError::Unauthorized)?;
    Ok(components::settings::render_settings(
        user,
        full.capacity_points,
        full.wip_limit,
        q.flash,
    ))
}

#[derive(Debug, Deserialize)]
pub struct CapacityForm {
    /// Capacity as a string from the form `<input type="number">`.
    /// The empty string means "unset / opt out". Any positive integer
    /// is passed through. Validation lives in [`parse_positive_int`]
    /// below so the empty-string case is handled cleanly.
    #[serde(default)]
    pub capacity_points: String,
}

#[derive(Debug, Deserialize)]
pub struct WipLimitForm {
    /// WIP limit as a string from the form. Empty string clears it
    /// (the user falls back to project default → system default).
    #[serde(default)]
    pub wip_limit: String,
}

/// Parse a positive-integer string from a settings form.
///
/// `""` → `None` (the user has opted out of this constraint).
/// `"3"` → `Some(3)`. Negative, zero, or non-numeric → HTTP 400.
fn parse_positive_int(raw: &str, field_label: &str) -> Result<Option<i64>, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let n: i64 = trimmed.parse().map_err(|_| {
        AppError::Validation(format!("{field_label} must be a positive integer."))
    })?;
    if n <= 0 {
        return Err(AppError::Validation(format!(
            "{field_label} must be a positive integer."
        )));
    }
    Ok(Some(n))
}

pub async fn update_capacity(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<CapacityForm>,
) -> AppResult<Redirect> {
    let cap = parse_positive_int(&form.capacity_points, "Capacity")?;
    users::set_capacity(&state.db, &user.id, cap).await?;
    Ok(Redirect::to("/settings?flash=Capacity+saved"))
}

pub async fn update_wip_limit(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<WipLimitForm>,
) -> AppResult<Redirect> {
    let wip = parse_positive_int(&form.wip_limit, "WIP limit")?;
    users::set_wip_limit(&state.db, &user.id, wip).await?;
    Ok(Redirect::to("/settings?flash=WIP+limit+saved"))
}
