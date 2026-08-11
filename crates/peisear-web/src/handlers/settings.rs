//! User settings: personal WIP limit + period-aware capacity rows.
//!
//! ## 0.12.0 changes
//!
//! Replaced the single capacity field with CRUD over
//! `user_capacities` rows. The capacity model is now:
//!
//! - The user can have any number of `user_capacities` rows, each
//!   with an optional `period_start` and `period_end`.
//! - Periods may not overlap (enforced in storage; surfaced as
//!   `AppError::Conflict` when violated).
//! - Adding the first row when the user has none takes the place
//!   of the old "set capacity" flow.
//! - Editing involves updating an existing row or closing one and
//!   adding a new one.
//!
//! WIP limit is unchanged: still a single integer field on
//! `users`.

use axum::{
    Form,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use chrono::NaiveDate;
use peisear_i18n::{Locale, MessageKey};
use peisear_storage::{user_capacities, users};
use serde::Deserialize;

use crate::{AppError, AppResult, AppState, components, extractors::AuthUser};

#[derive(Debug, Deserialize)]
pub struct FlashQuery {
    pub flash: Option<String>,
    /// Error message surfaced after a redirect — used for the
    /// capacity-overlap case so the form can render the conflict
    /// without forcing the user to re-enter their values.
    pub error: Option<String>,
}

pub async fn page(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let full = users::find_by_id(&state.db, &user.id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let rows = user_capacities::list_for_user(&state.db, &user.id).await?;
    // Today's effective capacity, surfaced in the page header so
    // the user can see "this is what's in effect right now".
    let effective_today = user_capacities::effective_for_user(&state.db, &user.id).await?;

    Ok(components::settings::render_settings(
        user,
        full.wip_limit,
        rows,
        effective_today,
        q.flash,
        q.error,
    ))
}

#[derive(Debug, Deserialize)]
pub struct WipLimitForm {
    #[serde(default)]
    pub wip_limit: String,
}

#[derive(Debug, Deserialize)]
pub struct CapacityForm {
    pub points: String,
    #[serde(default)]
    pub period_start: String,
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub note: String,
}

fn parse_positive_int(raw: &str, field_label: &str) -> Result<Option<i64>, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let n: i64 = trimmed
        .parse()
        .map_err(|_| AppError::Validation(format!("{field_label} must be a positive integer.")))?;
    if n <= 0 {
        return Err(AppError::Validation(format!(
            "{field_label} must be a positive integer."
        )));
    }
    Ok(Some(n))
}

fn parse_date(raw: &str, field_label: &str) -> Result<Option<NaiveDate>, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    NaiveDate::parse_from_str(trimmed, "%Y-%m-%d")
        .map(Some)
        .map_err(|_| AppError::Validation(format!("{field_label} must be in YYYY-MM-DD format.")))
}

/// Translate a `Conflict` from storage into a redirect with the
/// error in the query string. We don't 4xx the response because
/// the form is on a settings page that we'd like to re-render
/// with the conflict explained, not a stark error page.
fn redirect_with_conflict(message: &str) -> Redirect {
    let encoded = percent_encode_for_query(message);
    Redirect::to(&format!("/settings?error={}", encoded))
}

/// Minimal percent-encoder for the small subset of characters
/// that appear in our error strings. Avoids pulling in a full
/// urlencoding dep for one call site.
fn percent_encode_for_query(s: &str) -> String {
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

pub async fn update_wip_limit(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<WipLimitForm>,
) -> AppResult<Redirect> {
    let wip = parse_positive_int(&form.wip_limit, "WIP limit")?;
    users::set_wip_limit(&state.db, &user.id, wip).await?;
    let flash = Locale::English
        .render(MessageKey::WipLimitSavedFlash)
        .replace(' ', "+");
    Ok(Redirect::to(&format!("/settings?flash={flash}")))
}

/// Insert a new capacity row.
pub async fn insert_capacity(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<CapacityForm>,
) -> AppResult<Redirect> {
    let points = parse_positive_int(&form.points, "Capacity points")?
        .ok_or_else(|| AppError::Validation("Capacity points are required.".into()))?;
    let period_start = parse_date(&form.period_start, "Period start")?;
    let period_end = parse_date(&form.period_end, "Period end")?;
    let note = if form.note.trim().is_empty() {
        None
    } else {
        Some(form.note.trim())
    };

    match user_capacities::insert(&state.db, &user.id, points, period_start, period_end, note).await
    {
        Ok(_) => {
            let flash = Locale::English
                .render(MessageKey::CapacityRowAddedFlash)
                .replace(' ', "+");
            Ok(Redirect::to(&format!("/settings?flash={flash}")))
        }
        Err(peisear_storage::StorageError::Conflict(msg)) => Ok(redirect_with_conflict(&msg)),
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(msg)),
        Err(e) => Err(e.into()),
    }
}

#[derive(Debug, Deserialize)]
pub struct CapacityUpdateForm {
    pub points: String,
    #[serde(default)]
    pub period_start: String,
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub note: String,
    /// RFC3339 timestamp captured at form render. Validated
    /// against the row's current `updated_at` per
    /// peisear-feature-spec-v2.1 §21.4. Default empty means
    /// "no lock value provided" — `check_optimistic_lock`
    /// returns 400 (validation) on parse failure rather than
    /// silently bypassing.
    #[serde(default)]
    pub client_updated_at: String,
}

/// Update an existing capacity row.
pub async fn update_capacity(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(row_id): Path<String>,
    Form(form): Form<CapacityUpdateForm>,
) -> AppResult<Redirect> {
    // Optimistic-lock check (peisear-feature-spec-v2.1 §21.4).
    // Re-read the row to get the canonical `updated_at` —
    // also doubles as a 404 guard if the row was deleted
    // between page render and form submit.
    let current = user_capacities::find(&state.db, &user.id, &row_id)
        .await?
        .ok_or(AppError::NotFound)?;
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        current.updated_at,
        "capacity_period",
        &row_id,
    )?;

    let points = parse_positive_int(&form.points, "Capacity points")?
        .ok_or_else(|| AppError::Validation("Capacity points are required.".into()))?;
    let period_start = parse_date(&form.period_start, "Period start")?;
    let period_end = parse_date(&form.period_end, "Period end")?;
    let note = if form.note.trim().is_empty() {
        None
    } else {
        Some(form.note.trim())
    };

    match user_capacities::update(
        &state.db,
        &user.id,
        &row_id,
        points,
        period_start,
        period_end,
        note,
    )
    .await
    {
        Ok(()) => {
            let flash = Locale::English
                .render(MessageKey::CapacityRowUpdatedFlash)
                .replace(' ', "+");
            Ok(Redirect::to(&format!("/settings?flash={flash}")))
        }
        Err(peisear_storage::StorageError::Conflict(msg)) => Ok(redirect_with_conflict(&msg)),
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(msg)),
        Err(e) => Err(e.into()),
    }
}

/// Body for the delete form. Carries the lock value only.
#[derive(Debug, Deserialize, Default)]
pub struct CapacityDeleteForm {
    #[serde(default)]
    pub client_updated_at: String,
}

pub async fn delete_capacity(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(row_id): Path<String>,
    Form(form): Form<CapacityDeleteForm>,
) -> AppResult<Redirect> {
    let current = user_capacities::find(&state.db, &user.id, &row_id)
        .await?
        .ok_or(AppError::NotFound)?;
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        current.updated_at,
        "capacity_period",
        &row_id,
    )?;

    user_capacities::delete(&state.db, &user.id, &row_id).await?;
    let flash = Locale::English
        .render(MessageKey::CapacityRowRemovedFlash)
        .replace(' ', "+");
    Ok(Redirect::to(&format!("/settings?flash={flash}")))
}

#[derive(Debug, Deserialize)]
pub struct CloseForm {
    pub period_end: String,
    #[serde(default)]
    pub client_updated_at: String,
}

/// Helper endpoint: set the `period_end` of an existing row to a
/// specific date. Useful when adding a new period that would
/// otherwise overlap with an open-ended one.
pub async fn close_capacity(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Path(row_id): Path<String>,
    Form(form): Form<CloseForm>,
) -> AppResult<Redirect> {
    let current = user_capacities::find(&state.db, &user.id, &row_id)
        .await?
        .ok_or(AppError::NotFound)?;
    crate::error::check_optimistic_lock(
        &form.client_updated_at,
        current.updated_at,
        "capacity_period",
        &row_id,
    )?;

    let period_end = parse_date(&form.period_end, "Close date")?
        .ok_or_else(|| AppError::Validation("Close date is required.".into()))?;
    user_capacities::close_at(&state.db, &user.id, &row_id, period_end).await?;
    let flash = Locale::English
        .render(MessageKey::RowClosedFlash)
        .replace(' ', "+");
    Ok(Redirect::to(&format!("/settings?flash={flash}")))
}
