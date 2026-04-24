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
use serde::Deserialize;
use time::Duration as TimeDuration;
use validator::Validate;

use crate::{
    AppState,
    auth::{AUTH_COOKIE, jwt, password},
    error::{AppError, AppResult},
    views::{LoginPage, RegisterPage},
};

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    pub flash: Option<String>,
}

pub async fn login_page(Query(q): Query<AuthQuery>) -> impl IntoResponse {
    LoginPage {
        flash: q.flash,
        email: String::new(),
    }
}

pub async fn register_page(Query(q): Query<AuthQuery>) -> impl IntoResponse {
    RegisterPage {
        flash: q.flash,
        email: String::new(),
        display_name: String::new(),
    }
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
    #[validate(length(min = 8, max = 128, message = "Password must be at least 8 characters."))]
    pub password: String,
}

pub async fn register_submit(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<RegisterForm>,
) -> AppResult<(CookieJar, Redirect)> {
    form.validate()
        .map_err(|e| AppError::Validation(format_validation(&e)))?;

    let email = form.email.trim().to_lowercase();
    let display_name = form.display_name.trim().to_string();

    if crate::db::users::find_by_email(&state.db, &email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict(
            "An account with this email already exists.".into(),
        ));
    }

    let hash = password::hash(&form.password)?;
    let id = uuid::Uuid::new_v4().to_string();
    crate::db::users::insert(&state.db, &id, &email, &hash, &display_name).await?;

    let token = jwt::issue(&id, &email, &state.jwt_secret)?;
    let jar = jar.add(build_session_cookie(token));

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

    let user = crate::db::users::find_by_email(&state.db, &email).await?;
    // Constant-time-ish: always run verify even when user is absent, using
    // a dummy hash. This mitigates user-enumeration via timing.
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
        return Err(AppError::Validation("Invalid email or password.".into()));
    }
    let user = user.expect("ok implies user present");

    let token = jwt::issue(&user.id, &user.email, &state.jwt_secret)?;
    let jar = jar.add(build_session_cookie(token));

    Ok((jar, Redirect::to("/projects")))
}

pub async fn logout(jar: CookieJar) -> (CookieJar, Redirect) {
    let mut removal = Cookie::new(AUTH_COOKIE, "");
    removal.set_path("/");
    let jar = jar.remove(removal);
    (jar, Redirect::to("/login"))
}

fn build_session_cookie(token: String) -> Cookie<'static> {
    let secure =
        std::env::var("COOKIE_SECURE").map_or(false, |v| v == "1" || v.eq_ignore_ascii_case("true"));
    let mut cookie = Cookie::new(AUTH_COOKIE, token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_secure(secure);
    cookie.set_path("/");
    cookie.set_max_age(TimeDuration::seconds(jwt::SESSION_TTL_SECS));
    cookie
}

fn format_validation(errors: &validator::ValidationErrors) -> String {
    super::format_validation(errors)
}
