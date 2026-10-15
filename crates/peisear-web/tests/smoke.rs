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

/// `QA-003`: `search.js`'s tag (`components/layout.rs:72`, inside
/// `AppShell`) had no test at all -- unlike `dm.js`/`board.js`, its
/// blast radius is every authenticated page, not one surface, since
/// every such page renders through `AppShell`. `smoke` is the right
/// home for this claim: it's about the shell, not about search.
///
/// `GET /static/search.js` itself is `every_static_asset_is_actually_
/// servable`, below.
#[tokio::test]
async fn search_js_is_referenced_in_the_app_shell() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("dana");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    let body = resp.text();
    assert!(
        body.contains(r#"<script src="/static/search.js" defer"#),
        "an authenticated page must reference search.js with defer: {body}"
    );
}

/// `FIV-005`: `static/` is embedded into the binary at compile time
/// (`static_assets.rs`) rather than resolved against the process's
/// working directory at request time, which closes a gap the three
/// reference tests above and in `status_control.rs` used to name in
/// their own doc comments -- `ServeDir::new("static")` resolved
/// against `cargo test`'s crate-root working directory, not the
/// workspace root `static/` lives under, so an HTTP-level "is it
/// actually served" assertion would have 404'd regardless of whether
/// the file was really reachable from a real deployment. The embedded
/// set has no working-directory dependency, so this now tests the
/// real thing.
#[tokio::test]
async fn every_static_asset_is_actually_servable() {
    let app = TestApp::spawn().await;
    for name in [
        "app.css",
        "tailwind.css",
        "daisyui.min.css",
        "search.js",
        "board.js",
        "calendar.js",
        "plan.js",
        "dm.js",
    ] {
        let resp = app.server.get(&format!("/static/{name}")).await;
        assert_eq!(
            resp.status_code(),
            StatusCode::OK,
            "GET /static/{name} must be 200, found {}",
            resp.status_code()
        );
    }

    let resp = app.server.get("/static/does-not-exist.js").await;
    assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
}

/// `FIV-005` round 2 `§10`: the installed-binary walk re-run, at the
/// router level this time -- round 1 proved the page renders once;
/// this asserts a second navigation does not pay for the body again.
/// A real `GET` returns an `ETag`; echoing it back as `If-None-Match`
/// must answer `304` with no body, the way a browser's own second
/// navigation would.
#[tokio::test]
async fn a_second_navigation_does_not_refetch_the_asset_body() {
    let app = TestApp::spawn().await;

    let first = app.server.get("/static/daisyui.min.css").await;
    assert_eq!(first.status_code(), StatusCode::OK);
    let etag = first
        .header(axum::http::header::ETAG)
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        !first.as_bytes().is_empty(),
        "the first navigation must still carry the real body"
    );

    let second = app
        .server
        .get("/static/daisyui.min.css")
        .add_header(
            axum::http::header::IF_NONE_MATCH,
            etag.parse::<axum::http::HeaderValue>().unwrap(),
        )
        .await;
    assert_eq!(
        second.status_code(),
        StatusCode::NOT_MODIFIED,
        "a second navigation with the first response's own ETag must be 304, not a second \
         2.9 MB body"
    );
    assert!(
        second.as_bytes().is_empty(),
        "a 304 must carry no body -- wire bytes on the second navigation: {}",
        second.as_bytes().len()
    );
}
