//! Authentication helpers. Wraps the production register/login
//! flow so tests can produce authenticated `TestServer` sessions
//! with one call.
//!
//! Auth in peisear is via HTTP-only cookie. `axum-test`'s
//! `save_cookies` (configured in `server.rs`) means: after
//! `register` or `login`, the same `TestServer` will carry the
//! auth cookie on subsequent requests automatically. Tests don't
//! need to thread the cookie value through manually.
//!
//! For tests that need *two* logged-in users (e.g. authorization
//! tests where Alice and Bob are distinct), use [`new_authed_app`]
//! to spin up a separate `TestApp` per user — each `TestApp` has
//! its own cookie jar.

use peisear_storage::{Pool, users};

use super::server::TestApp;

/// Convenience credentials struct so tests can name their users
/// without re-writing the same struct everywhere.
#[derive(Debug, Clone)]
pub struct TestUser {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

impl TestUser {
    /// Construct credentials for a uniquely-named user. The email
    /// is suffixed with a nanosecond timestamp so multiple
    /// `TestUser::new` calls in the same test get distinct emails.
    pub fn new(name_hint: &str) -> Self {
        let suffix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock not before unix epoch")
            .as_nanos();
        Self {
            email: format!("{}-{}@example.com", name_hint, suffix),
            password: "test-password-1234".into(),
            display_name: name_hint.into(),
        }
    }
}

/// Register `user` against `app`'s server, completing the full
/// production flow: form POST to `/register`, server issues JWT,
/// cookie is saved on the TestServer's cookie jar.
///
/// Asserts the registration succeeded (303 redirect to /projects).
/// Returns the resolved user id (read from the DB after the
/// register handler inserts the row).
pub async fn register(app: &TestApp, user: &TestUser) -> String {
    let resp = app
        .server
        .post("/register")
        .form(&[
            ("email", user.email.as_str()),
            ("password", user.password.as_str()),
            ("display_name", user.display_name.as_str()),
        ])
        .await;
    resp.assert_status(axum::http::StatusCode::SEE_OTHER);

    user_id_for_email(&app.db, &user.email).await
}

/// Log `user` in against `app`'s server. The cookie is saved on
/// the TestServer's cookie jar.
pub async fn login(app: &TestApp, user: &TestUser) {
    let resp = app
        .server
        .post("/login")
        .form(&[
            ("email", user.email.as_str()),
            ("password", user.password.as_str()),
        ])
        .await;
    resp.assert_status(axum::http::StatusCode::SEE_OTHER);
}

/// Combined register + login flow for tests that don't need to
/// separate the two events: returns the resolved user id with
/// cookies set so subsequent requests on `app.server` are
/// authenticated.
pub async fn register_and_login(app: &TestApp, user: &TestUser) -> String {
    let user_id = register(app, user).await;
    // register_submit already issues a session cookie and a
    // 303 to /projects. No need for a second login call.
    user_id
}

/// Spin up a fresh app and register-then-login a fresh user.
/// Returns `(TestApp, TestUser, user_id)` so the test can use
/// the credentials and id without re-deriving them.
pub async fn new_authed_app(name_hint: &str) -> (TestApp, TestUser, String) {
    let app = TestApp::spawn().await;
    let user = TestUser::new(name_hint);
    let user_id = register_and_login(&app, &user).await;
    (app, user, user_id)
}

/// Manually log out the current session. Equivalent to a user
/// clicking the logout button in the UI.
pub async fn logout(app: &TestApp) {
    let resp = app.server.post("/logout").await;
    resp.assert_status(axum::http::StatusCode::SEE_OTHER);
}

/// Read the user id from the DB given an email. Useful when a
/// test creates a user via the production register flow and
/// later needs the id (e.g. to construct authorization-test URLs
/// like `/api/users/{user_id}/burnout`).
pub async fn user_id_for_email(db: &Pool, email: &str) -> String {
    users::find_by_email(db, email)
        .await
        .expect("query users by email")
        .expect("user not found in DB after register")
        .id
}
