//! Registration, login, and logout handlers.

use axum::{
    Form,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use peisear_auth::{jwt, password};
use peisear_i18n::MessageKey;
use peisear_storage::users;
use serde::Deserialize;
use time::Duration as TimeDuration;
use validator::Validate;

use crate::{
    AppError, AppResult, AppState, components,
    components::t,
    extractors::{AUTH_COOKIE, Requester},
};

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    pub flash: Option<String>,
}

pub async fn login_page(Query(q): Query<AuthQuery>) -> impl IntoResponse {
    components::auth::render_login(q.flash, String::new())
}

pub async fn register_page(Query(q): Query<AuthQuery>) -> impl IntoResponse {
    components::auth::render_register(q.flash, String::new(), String::new())
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterForm {
    #[validate(email(message = "Please enter a valid email address."))]
    pub email: String,
    #[validate(length(
        min = 1,
        max = 80,
        message = "Display name must be between 1 and 80 characters."
    ))]
    pub display_name: String,
    #[validate(length(
        min = 8,
        max = 128,
        message = "Password must be at least 8 characters."
    ))]
    pub password: String,
}

pub async fn register_submit(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<RegisterForm>,
) -> AppResult<(CookieJar, Redirect)> {
    form.validate()
        .map_err(|e| AppError::Validation(super::format_validation(&e)))?;

    let email = form.email.trim().to_lowercase();
    let display_name = form.display_name.trim().to_string();

    if users::find_by_email(&state.db, &email).await?.is_some() {
        return Err(AppError::Conflict(t(MessageKey::EmailAlreadyExistsMessage)));
    }

    let hash = password::hash(&form.password)?;
    let id = uuid::Uuid::new_v4().to_string();
    users::insert(&state.db, &id, &email, &hash, &display_name).await?;

    // `SES-001`: a brand-new row starts at generation 0 (the column's
    // own default), so the freshly-inserted user has never been read
    // back here — stamping the literal rather than a wasted query.
    let token = jwt::issue(&id, &email, 0, &state.jwt_secret)?;
    let jar = jar.add(build_session_cookie(token, state.cookie_secure));

    Ok((jar, Redirect::to("/projects")))
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn login_submit(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> AppResult<(CookieJar, Redirect)> {
    let email = form.email.trim().to_lowercase();

    let user = users::find_by_email(&state.db, &email).await?;
    // Constant-time-ish: always run verify even when user is absent,
    // using a dummy hash. This mitigates user‑enumeration via timing.
    let (ok, user) = match user {
        Some(u) => {
            let ok = password::verify(&form.password, &u.password_hash)?;
            (ok, Some(u))
        }
        None => {
            // Phantom verification with a fixed (invalid) hash.
            let _ = password::verify(
                &form.password,
                "$argon2id$v=19$m=19456,t=2,p=1$c2FsdHNhbHRzYWx0$aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            );
            (false, None)
        }
    };

    if !ok {
        return Err(AppError::Validation(t(
            MessageKey::InvalidCredentialsMessage,
        )));
    }
    let user = user.expect("ok implies user present");

    // `SES-001`: stamp the row's own current generation into the new
    // token. `user` was just loaded above to check the password, so
    // this is no new read — `RFC 024` §4's own point.
    let token = jwt::issue(
        &user.id,
        &user.email,
        user.session_generation,
        &state.jwt_secret,
    )?;
    let jar = jar.add(build_session_cookie(token, state.cookie_secure));

    Ok((jar, Redirect::to("/projects")))
}

/// `SES-001` (`RFC 024` §3/§4): server-side now, not merely a cookie
/// removal. `Requester` both supplies whose generation to advance and
/// rejects an already-ended session before this body ever runs — the
/// "land politely at `/login`" requirement for that case (`RFC 024`
/// §5's last paragraph) is `AppError`'s own `SessionRevoked` redirect,
/// not a second error page written here.
pub async fn logout(
    State(state): State<AppState>,
    Requester(rid): Requester,
    jar: CookieJar,
) -> AppResult<(CookieJar, Redirect)> {
    users::advance_session_generation(&state.db, rid.as_str()).await?;

    let mut removal = Cookie::new(AUTH_COOKIE, "");
    removal.set_path("/");
    let jar = jar.remove(removal);
    Ok((jar, Redirect::to("/login")))
}

fn build_session_cookie(token: String, secure: bool) -> Cookie<'static> {
    let mut cookie = Cookie::new(AUTH_COOKIE, token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_secure(secure);
    cookie.set_path("/");
    cookie.set_max_age(TimeDuration::seconds(jwt::SESSION_TTL_SECS));
    cookie
}
