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
use axum_extra::extract::CookieJar;
use chrono::NaiveDate;
use peisear_auth::{jwt, password};
use peisear_i18n::{Field, Locale, MessageKey};
use peisear_storage::{notifications as notif_store, subjects::SubjectId, user_capacities, users};
use serde::Deserialize;
use validator::Validate;

use crate::{
    AppError, AppResult, AppState, components,
    components::t,
    extractors::{AuthUser, Requester},
    handlers::auth::{build_session_cookie, validate_password_strength},
};

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
    Requester(rid): Requester,
    State(state): State<AppState>,
    Query(q): Query<FlashQuery>,
) -> AppResult<impl IntoResponse> {
    let full = users::find_by_id(&state.db, &user.id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let rows = user_capacities::list_for_user(&state.db, &rid).await?;
    // Today's effective capacity, surfaced in the page header so
    // the user can see "this is what's in effect right now".
    let effective_today =
        user_capacities::effective_for_user(&state.db, &SubjectId::from(&rid)).await?;
    let unread_count = notif_store::unread_count_for_user(&state.db, &rid).await?;

    Ok(components::settings::render_settings(
        user,
        full.wip_limit,
        rows,
        effective_today,
        q.flash,
        q.error,
        unread_count,
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

fn parse_positive_int(
    raw: &str,
    invalid_message: impl Fn() -> String,
) -> Result<Option<i64>, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let n: i64 = trimmed
        .parse()
        .map_err(|_| AppError::Validation(invalid_message()))?;
    if n <= 0 {
        return Err(AppError::Validation(invalid_message()));
    }
    Ok(Some(n))
}

fn parse_date(
    raw: &str,
    invalid_message: impl Fn() -> String,
) -> Result<Option<NaiveDate>, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    NaiveDate::parse_from_str(trimmed, "%Y-%m-%d")
        .map(Some)
        .map_err(|_| AppError::Validation(invalid_message()))
}

/// Translate a `Conflict` from storage into a redirect with the
/// error in the query string. We don't 4xx the response because
/// the form is on a settings page that we'd like to re-render
/// with the conflict explained, not a stark error page.
///
/// Used a same-shaped, differently-named local copy of the encoder
/// below (`percent_encode_for_query`) until `QA-020` — a fourth
/// hand-rolled implementation the handoff's own name-based search
/// for `percent_encode_query` didn't find, discovered while routing
/// this file's other sites through the single copy in
/// `super::percent_encode_query`.
fn redirect_with_conflict(message: &str) -> Redirect {
    let encoded = super::percent_encode_query(message);
    Redirect::to(&format!("/settings?error={}", encoded))
}

pub async fn update_wip_limit(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Form(form): Form<WipLimitForm>,
) -> AppResult<Redirect> {
    let wip = parse_positive_int(&form.wip_limit, || {
        t(MessageKey::WipLimitMustBePositiveIntegerMessage)
    })?;
    users::set_wip_limit(&state.db, &user.id, wip).await?;
    let flash =
        super::percent_encode_query(&Locale::English.render(MessageKey::WipLimitSavedFlash));
    Ok(Redirect::to(&format!("/settings?flash={flash}")))
}

/// `PWD-001` (`RFC 025` §4.1): `new_password` shares its strength rule
/// with `handlers::auth::RegisterForm::password` via
/// [`validate_password_strength`] rather than carrying a second
/// `#[validate(length(...))]` copy.
///
/// `confirm_password` carries no derive attribute: `validator`'s own
/// `must_match` splices its `message` as a literal at compile time
/// (`quote!(err.message = Some(Cow::from(#m)))`), which cannot call
/// into `peisear_i18n::t` — every other message in this product goes
/// through a `MessageKey`, so the comparison is a plain `!=` in
/// [`change_password`] below instead of a derive rule that would have
/// to bypass the one text source this crate otherwise never bypasses.
///
/// `current_password` carries no `#[validate(...)]` attribute either:
/// an empty or wrong value is rejected by [`password::verify`] below
/// like any other wrong value, which is also why `§4` item 4's "a
/// wrong current password changes nothing" holds for an empty
/// submission too, with no special case.
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordForm {
    pub current_password: String,
    #[validate(custom(function = "validate_password_strength"))]
    pub new_password: String,
    pub confirm_password: String,
}

/// `PWD-001` (`RFC 025` §4.1, `§3` of the handoff): the first handler
/// in this product to change a stored credential.
///
/// Order matters here, and is the order `§3` gives: validate the new
/// password's shape first (cheap, no I/O): then load the row and
/// verify the *current* password (the one check a convenience-minded
/// form would be tempted to skip, and the one this RFC says a
/// security property wins over without argument); only then write.
/// `§3` step 5's own instruction — "take the generation from the
/// value you just wrote, not from a re-read, and not from the stale
/// row you loaded" — is `new_generation` below: computed from the row
/// already in hand plus the one the storage layer's single `UPDATE`
/// is guaranteed to have applied, never a second `SELECT`.
pub async fn change_password(
    Requester(rid): Requester,
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<ChangePasswordForm>,
) -> AppResult<(CookieJar, Redirect)> {
    form.validate()
        .map_err(|e| AppError::Validation(super::format_validation(&e)))?;

    if form.confirm_password != form.new_password {
        return Err(AppError::Validation(t(
            MessageKey::NewPasswordConfirmationMismatchMessage,
        )));
    }

    let user = users::find_by_id(&state.db, rid.as_str())
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !password::verify(&form.current_password, &user.password_hash)? {
        return Err(AppError::Validation(t(
            MessageKey::CurrentPasswordIncorrectMessage,
        )));
    }

    let new_hash = password::hash(&form.new_password)?;
    users::change_password(&state.db, &user.id, &new_hash).await?;
    let new_generation = user.session_generation + 1;

    let token = jwt::issue(&user.id, &user.email, new_generation, &state.jwt_secret)?;
    let jar = jar.add(build_session_cookie(token, state.cookie_secure));

    let flash =
        super::percent_encode_query(&Locale::English.render(MessageKey::PasswordChangedFlash));
    Ok((jar, Redirect::to(&format!("/settings?flash={flash}"))))
}

/// Insert a new capacity row.
pub async fn insert_capacity(
    Requester(rid): Requester,
    State(state): State<AppState>,
    Form(form): Form<CapacityForm>,
) -> AppResult<Redirect> {
    let points = parse_positive_int(&form.points, || {
        t(MessageKey::FieldMustBePositiveInteger {
            field: Field::CapacityPoints,
        })
    })?
    .ok_or_else(|| AppError::Validation(t(MessageKey::CapacityPointsRequiredMessage)))?;
    let period_start = parse_date(&form.period_start, || {
        t(MessageKey::PeriodStartMustBeDateFormatMessage)
    })?;
    let period_end = parse_date(&form.period_end, || {
        t(MessageKey::PeriodEndMustBeDateFormatMessage)
    })?;
    let note = if form.note.trim().is_empty() {
        None
    } else {
        Some(form.note.trim())
    };

    match user_capacities::insert(&state.db, &rid, points, period_start, period_end, note).await {
        Ok(_) => {
            let flash = super::percent_encode_query(
                &Locale::English.render(MessageKey::CapacityRowAddedFlash),
            );
            Ok(Redirect::to(&format!("/settings?flash={flash}")))
        }
        Err(peisear_storage::StorageError::Conflict(msg)) => Ok(redirect_with_conflict(&t(msg))),
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(t(msg))),
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
    Requester(rid): Requester,
    State(state): State<AppState>,
    Path(row_id): Path<String>,
    Form(form): Form<CapacityUpdateForm>,
) -> AppResult<Redirect> {
    // Optimistic-lock check (peisear-feature-spec-v2.1 §21.4).
    // Re-read the row to get the canonical `updated_at` —
    // also doubles as a 404 guard if the row was deleted
    // between page render and form submit.
    let current = user_capacities::find(&state.db, &rid, &row_id)
        .await?
        .ok_or(AppError::NotFound)?;
    let stamp = crate::error::check_optimistic_lock(
        &form.client_updated_at,
        current.updated_at,
        peisear_i18n::EntityKind::CapacityPeriod,
        &row_id,
    )?;

    let points = parse_positive_int(&form.points, || {
        t(MessageKey::FieldMustBePositiveInteger {
            field: Field::CapacityPoints,
        })
    })?
    .ok_or_else(|| AppError::Validation(t(MessageKey::CapacityPointsRequiredMessage)))?;
    let period_start = parse_date(&form.period_start, || {
        t(MessageKey::PeriodStartMustBeDateFormatMessage)
    })?;
    let period_end = parse_date(&form.period_end, || {
        t(MessageKey::PeriodEndMustBeDateFormatMessage)
    })?;
    let note = if form.note.trim().is_empty() {
        None
    } else {
        Some(form.note.trim())
    };

    match user_capacities::update_guarded(
        &state.db,
        &rid,
        &row_id,
        points,
        period_start,
        period_end,
        note,
        Some(stamp),
    )
    .await
    {
        Ok(peisear_storage::Guarded::Stale { current_updated_at }) => {
            Err(crate::error::stale_conflict(
                peisear_i18n::EntityKind::CapacityPeriod,
                &row_id,
                current_updated_at,
            ))
        }
        Ok(peisear_storage::Guarded::Written(())) => {
            let flash = super::percent_encode_query(
                &Locale::English.render(MessageKey::CapacityRowUpdatedFlash),
            );
            Ok(Redirect::to(&format!("/settings?flash={flash}")))
        }
        Err(peisear_storage::StorageError::Conflict(msg)) => Ok(redirect_with_conflict(&t(msg))),
        Err(peisear_storage::StorageError::Validation(msg)) => Err(AppError::Validation(t(msg))),
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
    Requester(rid): Requester,
    State(state): State<AppState>,
    Path(row_id): Path<String>,
    Form(form): Form<CapacityDeleteForm>,
) -> AppResult<Redirect> {
    let current = user_capacities::find(&state.db, &rid, &row_id)
        .await?
        .ok_or(AppError::NotFound)?;
    let stamp = crate::error::check_optimistic_lock(
        &form.client_updated_at,
        current.updated_at,
        peisear_i18n::EntityKind::CapacityPeriod,
        &row_id,
    )?;

    crate::error::lock_outcome(
        user_capacities::delete_guarded(&state.db, &rid, &row_id, Some(stamp)).await?,
        peisear_i18n::EntityKind::CapacityPeriod,
        &row_id,
    )?;
    let flash =
        super::percent_encode_query(&Locale::English.render(MessageKey::CapacityRowRemovedFlash));
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
    Requester(rid): Requester,
    State(state): State<AppState>,
    Path(row_id): Path<String>,
    Form(form): Form<CloseForm>,
) -> AppResult<Redirect> {
    let current = user_capacities::find(&state.db, &rid, &row_id)
        .await?
        .ok_or(AppError::NotFound)?;
    let stamp = crate::error::check_optimistic_lock(
        &form.client_updated_at,
        current.updated_at,
        peisear_i18n::EntityKind::CapacityPeriod,
        &row_id,
    )?;

    let period_end = parse_date(&form.period_end, || {
        t(MessageKey::FieldMustBeDateFormat {
            field: Field::CloseDate,
        })
    })?
    .ok_or_else(|| {
        AppError::Validation(t(MessageKey::FieldRequired {
            field: Field::CloseDate,
        }))
    })?;
    crate::error::lock_outcome(
        user_capacities::close_at_guarded(&state.db, &rid, &row_id, period_end, Some(stamp))
            .await?,
        peisear_i18n::EntityKind::CapacityPeriod,
        &row_id,
    )?;
    let flash = super::percent_encode_query(&Locale::English.render(MessageKey::RowClosedFlash));
    Ok(Redirect::to(&format!("/settings?flash={flash}")))
}
