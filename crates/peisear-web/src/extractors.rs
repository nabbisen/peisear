//! Axum request extractors that resolve the current user from the
//! session cookie.

use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use peisear_auth::jwt::{self, RequesterId};
use peisear_core::CurrentUser;
use peisear_storage::users;

use crate::{ApiAppError, AppError, AppState};

/// Name of the auth cookie holding the JWT.
pub const AUTH_COOKIE: &str = "it_session";

/// Shared by [`AuthUser`] and [`Requester`]: read the session cookie and
/// return a verified [`RequesterId`] -- the only way either extractor (or
/// anything else in the workspace) can obtain one, per `jwt`'s own module
/// doc comment.
async fn verified_requester_id<S>(parts: &mut Parts, state: &S) -> Result<RequesterId, AppError>
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    let State(app): State<AppState> = State::from_request_parts(parts, state)
        .await
        .map_err(|_| AppError::Internal("failed to extract state".into()))?;
    let jar = CookieJar::from_request_parts(parts, state)
        .await
        .map_err(|_| AppError::Internal("failed to extract cookies".into()))?;

    let token = jar
        .get(AUTH_COOKIE)
        .ok_or(AppError::Unauthorized)?
        .value()
        .to_owned();
    jwt::verify(&token, &app.jwt_secret).map_err(|_| AppError::Unauthorized)
}

/// Extractor that requires an authenticated user. Returns
/// [`AppError::Unauthorized`] (which the error handler turns into a
/// redirect to `/login`) when no valid session is present.
pub struct AuthUser(pub CurrentUser);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let requester_id = verified_requester_id(parts, state).await?;
        let State(app): State<AppState> = State::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Internal("failed to extract state".into()))?;

        // Re-hydrate the user from the DB so deleted or altered accounts
        // are immediately invalidated rather than waiting for the JWT to
        // expire.
        let user = users::find_by_id(&app.db, requester_id.as_str())
            .await?
            .ok_or(AppError::Unauthorized)?;
        Ok(AuthUser(user.into()))
    }
}

/// Extractor for handlers that call personal-data storage (`PRIV-002`,
/// `DEC-058`). Carries only a verified [`RequesterId`] -- no DB
/// round-trip, unlike [`AuthUser`], since nothing here needs the email or
/// display name. A handler that also needs those extracts both; the JWT
/// is decoded twice per such request, which is cheap (no I/O) next to the
/// guarantee: `RequesterId` cannot be constructed from anything but a
/// token that just passed signature and expiry verification.
pub struct Requester(pub RequesterId);

impl<S> FromRequestParts<S> for Requester
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Requester(verified_requester_id(parts, state).await?))
    }
}

/// `/api/*` form of [`Requester`], rejecting with [`ApiAppError`] (JSON)
/// rather than [`AppError`] (redirect) -- same pairing as
/// [`ApiAuthUser`]/[`AuthUser`].
pub struct ApiRequester(pub RequesterId);

impl<S> FromRequestParts<S> for ApiRequester
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiAppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Requester::from_request_parts(parts, state).await {
            Ok(Requester(rid)) => Ok(ApiRequester(rid)),
            Err(AppError::Unauthorized) => Err(ApiAppError::Unauthorized),
            Err(AppError::Internal(msg)) => Err(ApiAppError::Internal(msg)),
            Err(other) => {
                tracing::error!(?other, "unexpected AppError from Requester in ApiRequester");
                Err(ApiAppError::Internal("unexpected auth error".into()))
            }
        }
    }
}

/// Extractor that optionally provides the authenticated user — used on
/// the landing route to decide whether to redirect to `/login` or
/// `/projects`. Returns `None` when no session is present, propagates
/// only genuine internal errors.
pub struct MaybeAuthUser(pub Option<CurrentUser>);

impl<S> FromRequestParts<S> for MaybeAuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(AuthUser(u)) => Ok(MaybeAuthUser(Some(u))),
            Err(AppError::Unauthorized) => Ok(MaybeAuthUser(None)),
            Err(e) => Err(e),
        }
    }
}

/// Extractor for `/api/*` routes. Identical authentication
/// logic to [`AuthUser`] but rejects with [`ApiAppError`]
/// instead of [`AppError`], so the error response is JSON
/// (with status 401) rather than a 303 redirect to `/login`.
///
/// JS callers expect to handle 401 themselves (typically by
/// prompting login or redirecting through their own router);
/// the server's job here is just to fail with a parseable
/// shape.
pub struct ApiAuthUser(pub CurrentUser);

impl<S> FromRequestParts<S> for ApiAuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiAppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Reuse the existing AuthUser logic by attempting it
        // and translating its rejection. This keeps a single
        // source of truth for the auth path; if the cookie
        // extraction or JWT verification logic changes, both
        // extractors pick it up.
        match AuthUser::from_request_parts(parts, state).await {
            Ok(AuthUser(u)) => Ok(ApiAuthUser(u)),
            Err(AppError::Unauthorized) => Err(ApiAppError::Unauthorized),
            Err(AppError::Internal(msg)) => Err(ApiAppError::Internal(msg)),
            // The remaining AppError variants (Forbidden,
            // NotFound, Validation, etc.) shouldn't arise from
            // an extractor that only does authentication; if
            // one ever does, surface it as Internal so we
            // notice in logs rather than papering over with a
            // misleading error code.
            Err(other) => {
                tracing::error!(?other, "unexpected AppError from AuthUser in ApiAuthUser");
                Err(ApiAppError::Internal("unexpected auth error".into()))
            }
        }
    }
}
