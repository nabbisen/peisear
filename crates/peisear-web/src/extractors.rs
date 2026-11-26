//! Axum request extractors that resolve the current user from the
//! session cookie.

use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use peisear_auth::jwt::{self, RequesterId};
use peisear_core::{CurrentUser, User};
use peisear_storage::users;

use crate::{ApiAppError, AppError, AppState};

/// Name of the auth cookie holding the JWT.
pub const AUTH_COOKIE: &str = "it_session";

/// `SES-001` (`RFC 024` §4.3): [`verified_requester_id`]'s own result,
/// memoised in `parts.extensions` so a handler whose signature extracts
/// both [`AuthUser`] and [`Requester`]/[`ApiRequester`] (29 of them) pays
/// for the JWT decode and the user-row read once per request, not twice
/// — the double decode already happened before this round (`Requester`'s
/// own doc comment called it cheap because it was I/O-free); the row
/// read is the new cost this cache removes a second time over.
#[derive(Clone)]
struct CachedAuth {
    requester_id: RequesterId,
    user: User,
}

/// Shared by [`AuthUser`] and [`Requester`]: read the session cookie,
/// verify the token, and check it against the subject's current session
/// generation -- the only way either extractor (or anything else in the
/// workspace) can obtain a [`RequesterId`], per `jwt`'s own module doc
/// comment, and now also the one and only place `RFC 024` §4's
/// revocation check is made (§4.2: "nowhere else" -- a rule placed in
/// both extractors is a rule a sixth one skips).
///
/// Returns the user row too, not only the id: this function already
/// needs it to read `session_generation`, so [`AuthUser`] no longer
/// makes its own second `users::find_by_id` -- the row is fetched once
/// and handed to whichever caller asked.
async fn verified_requester_id<S>(
    parts: &mut Parts,
    state: &S,
) -> Result<(RequesterId, User), AppError>
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    if let Some(cached) = parts.extensions.get::<CachedAuth>() {
        return Ok((cached.requester_id.clone(), cached.user.clone()));
    }

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
    let (requester_id, token_generation) =
        jwt::verify(&token, &app.jwt_secret).map_err(|_| AppError::Unauthorized)?;

    // Re-hydrate the user from the DB so deleted or altered accounts
    // are immediately invalidated rather than waiting for the JWT to
    // expire -- unchanged from before this round, just moved here from
    // `AuthUser` since this function now needs the row regardless.
    let user = users::find_by_id(&app.db, requester_id.as_str())
        .await?
        .ok_or(AppError::Unauthorized)?;

    // `RFC 024` §3: a credential is valid only if it was issued under
    // the subject's *current* session generation. A mismatch gets its
    // own error variant, not a bare `Unauthorized` -- the HTML path
    // uses it to show the other device *why* it landed back at
    // `/login` (`RFC 024` §5) rather than a bare, unexplained redirect.
    if user.session_generation != token_generation {
        return Err(AppError::SessionRevoked);
    }

    parts.extensions.insert(CachedAuth {
        requester_id: requester_id.clone(),
        user: user.clone(),
    });
    Ok((requester_id, user))
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
        let (_, user) = verified_requester_id(parts, state).await?;
        Ok(AuthUser(user.into()))
    }
}

/// Extractor for handlers that call personal-data storage (`PRIV-002`,
/// `DEC-058`). Carries only a verified [`RequesterId`]. `SES-001`:
/// **no longer no-I/O** -- `verified_requester_id` now reads the user
/// row on every request (to check the session generation), so this
/// extractor pays that read too, unlike before. A handler that also
/// needs the email or display name extracts [`AuthUser`] as well; both
/// extractors funnel through the same choke point, which memoises the
/// row (`CachedAuth`) so that costs one read, not two.
pub struct Requester(pub RequesterId);

impl<S> FromRequestParts<S> for Requester
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let (requester_id, _) = verified_requester_id(parts, state).await?;
        Ok(Requester(requester_id))
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
            Err(AppError::Unauthorized) | Err(AppError::SessionRevoked) => {
                Err(ApiAppError::Unauthorized)
            }
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
            Err(AppError::Unauthorized) | Err(AppError::SessionRevoked) => Ok(MaybeAuthUser(None)),
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
            Err(AppError::Unauthorized) | Err(AppError::SessionRevoked) => {
                Err(ApiAppError::Unauthorized)
            }
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
