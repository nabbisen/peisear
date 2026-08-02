//! Smoke tests — verify the app boots and the most common
//! happy paths produce the expected status codes / redirects.
//!
//! These intentionally don't assert HTML content. Phase A's
//! information architecture changes (e.g. `/me` → `/today`)
//! will exercise these via redirect status. UI rendering is
//! validated by Phase B via separate component-focused tests.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::server::TestApp;

#[tokio::test]
async fn health_endpoint_returns_200() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/health").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn login_page_renders() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/login").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn register_page_renders() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/register").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn register_then_redirected_to_projects() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let _user_id = register_and_login(&app, &user).await;
    // After register, the production handler 303s to /projects.
    // Following that, /projects should now render successfully
    // (we have a session cookie in the saved jar).
    let resp = app.server.get("/projects").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn unauthenticated_projects_redirects_to_login() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/projects").await;
    // Per existing behaviour: protected routes redirect
    // unauthenticated users to /login.
    assert_eq!(resp.status_code(), StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("redirect Location header")
        .to_str()
        .expect("ASCII Location header");
    assert!(
        location.starts_with("/login"),
        "expected redirect to /login, got: {location}"
    );
}

#[tokio::test]
async fn today_dashboard_loads_for_authenticated_user() {
    // /today is the canonical path as of v0.17.0 (renamed from /me).
    let app = TestApp::spawn().await;
    let user = TestUser::new("bob");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn me_redirects_to_today_with_308() {
    // Legacy /me must 308-redirect to /today, preserving the
    // request method. The redirect itself does not require auth
    // — auth is enforced at /today after the redirect.
    let app = TestApp::spawn().await;
    let resp = app.server.get("/me").await;

    assert_eq!(
        resp.status_code(),
        StatusCode::PERMANENT_REDIRECT,
        "expected 308 from /me, got {}",
        resp.status_code()
    );
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location header present")
        .to_str()
        .expect("Location is ASCII");
    assert_eq!(location, "/today");
}

#[tokio::test]
async fn notifications_redirects_to_inbox_with_308() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/notifications").await;

    assert_eq!(resp.status_code(), StatusCode::PERMANENT_REDIRECT);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location header present")
        .to_str()
        .expect("Location is ASCII");
    assert_eq!(location, "/inbox");
}

#[tokio::test]
async fn inbox_loads_for_authenticated_user() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("frank");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/inbox").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn legacy_notifications_post_endpoints_redirect_with_308() {
    // The 308 redirect must preserve POST method on its way to
    // the new /inbox/* endpoints. The 308 status code is what
    // makes the difference vs. 301: a 301 would (at least
    // historically) let the client downgrade to GET.
    let app = TestApp::spawn().await;

    // mark-all-read
    let resp = app.server.post("/notifications/mark-all-read").await;
    assert_eq!(resp.status_code(), StatusCode::PERMANENT_REDIRECT);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location present")
        .to_str()
        .unwrap();
    assert_eq!(location, "/inbox/mark-all-read");

    // mark one as read — preserves the {id} path parameter
    let resp = app.server.post("/notifications/some-notif-id/read").await;
    assert_eq!(resp.status_code(), StatusCode::PERMANENT_REDIRECT);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location present")
        .to_str()
        .unwrap();
    assert_eq!(location, "/inbox/some-notif-id/read");
}

#[tokio::test]
async fn logout_clears_session_and_redirects() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("carol");
    register_and_login(&app, &user).await;

    // Verify authenticated first.
    let resp = app.server.get("/projects").await;
    resp.assert_status_ok();

    // Log out.
    let resp = app.server.post("/logout").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    // Now /projects should redirect to /login again.
    let resp = app.server.get("/projects").await;
    assert_eq!(resp.status_code(), StatusCode::SEE_OTHER);
}
