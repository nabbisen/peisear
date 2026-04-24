//! Authentication primitives: password hashing, JWT, and a request
//! extractor that returns the current user.

pub mod jwt;
pub mod password;

use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;

use crate::{AppState, error::AppError, models::CurrentUser};

/// Name of the auth cookie holding the JWT.
pub const AUTH_COOKIE: &str = "it_session";

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
        let claims =
            jwt::verify(&token, &app.jwt_secret).map_err(|_| AppError::Unauthorized)?;

        // Re-hydrate the user from the DB so deleted or altered accounts are
        // immediately invalidated rather than waiting for the JWT to expire.
        let user = crate::db::users::find_by_id(&app.db, &claims.sub)
            .await?
            .ok_or(AppError::Unauthorized)?;
        Ok(AuthUser(user.into()))
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
