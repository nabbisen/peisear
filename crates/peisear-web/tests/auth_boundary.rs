//! Authorization boundary tests — verify that personal-data
//! endpoints reject cross-user access at the API layer, per
//! peisear-feature-spec-v2.1 §11.5 ("API レベル認可の不変条件").
//!
//! These tests run against the production router. They exercise
//! the request path the same way a malicious browser DevTools
//! request would — bypassing UI checks entirely. The assertion
//! is always:
//!
//! - Unauthenticated: 401 Unauthorized
//! - Authenticated as a *different* user: 403 Forbidden
//! - Authenticated as the *same* user: 200 OK (smoke-checked
//!   in `smoke.rs`, not re-asserted here)
//!
//! ## Phase A status
//!
//! As of Phase A entry, the `/api/users/{user_id}/...` endpoints
//! enumerated in §11.5.1 don't yet exist. This file holds:
//!
//! 1. Tests for endpoints that already exist and should already
//!    enforce the boundary (e.g. `/me`).
//! 2. `#[ignore]`d tests for endpoints that will exist after
//!    Phase B/C, so the test inventory is in place when those
//!    endpoints land.
//!
//! When a Phase B/C PR adds a new personal-data endpoint, the
//! `#[ignore]` is removed in the same PR, and the test confirms
//! the new authorization guard is wired correctly.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, new_authed_app, register_and_login};
use common::server::TestApp;

// -------------------------------------------------------------------
// /today (the personal dashboard) — already exists in v0.17.0,
// must already enforce
// -------------------------------------------------------------------

/// /today has no `{user_id}` path segment — it's implicitly "self".
/// The boundary check here is: an unauthenticated request must
/// not see any data, and gets redirected to /login instead.
///
/// We deliberately target /today (not the legacy /me redirect)
/// because /me is a 308 to /today regardless of authentication —
/// the authorization check happens at the destination, not at
/// the legacy alias.
#[tokio::test]
async fn today_unauthenticated_redirects_to_login() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/today").await;
    // Existing behaviour: AppError::Unauthenticated turns into a
    // 303 redirect to /login (see peisear-web::error). This is
    // HTML-friendly. JSON API endpoints introduced in Phase B+
    // will return 401 instead — those get their own tests below.
    assert_eq!(resp.status_code(), StatusCode::SEE_OTHER);
}

/// Legacy `/me` is a 308 permanent redirect to `/today`. The
/// redirect itself does NOT depend on auth state — that gating
/// happens at the canonical destination. This is the correct
/// layering for two reasons:
///
/// 1. **Operational** (canonical-URL learning): the rule
///    "the URL `/me` moved to `/today`" is true regardless of
///    who's asking. Static redirect rules belong above session
///    checks. If `/me` *did* gate on auth, an unauthenticated
///    browser following an old bookmark would land on `/login`
///    directly without ever learning the canonical URL. After
///    logging in, they'd come back via `/me` again and re-bounce,
///    never arriving at `/today`. Routing the redirect first and
///    gating auth at the destination preserves canonical-URL
///    learning for browser caches and bookmarks.
/// 2. **Privacy boundary** (§11.5): by funneling `/me` through
///    `/today` rather than serving `/me` directly, every personal-
///    data response is forced through the same `AuthUser`
///    extractor. A regression where the legacy alias accidentally
///    rendered data without that extractor would be exactly the
///    kind of cross-cutting bug §11.5 of the spec is meant to
///    prevent. That's why this test lives in `auth_boundary.rs`
///    and not in `smoke.rs` with the other routing tests.
#[tokio::test]
async fn me_unauthenticated_redirects_to_today_not_login() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/me").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::PERMANENT_REDIRECT,
        "expected 308 from /me regardless of auth state, got {}",
        resp.status_code()
    );
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location header")
        .to_str()
        .expect("ASCII Location");
    assert_eq!(location, "/today");
}

// -------------------------------------------------------------------
// /api/users/{user_id}/burnout — Phase B (planned)
// -------------------------------------------------------------------

#[tokio::test]
#[ignore = "endpoint not implemented yet — Phase B"]
async fn burnout_endpoint_walls_off_other_users() {
    let (alice_app, _alice, _alice_id) = new_authed_app("alice").await;

    // Register Bob in a separate app/server so we have a distinct
    // user id but share no cookie jar.
    let bob_app = TestApp::spawn().await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&bob_app, &bob).await;

    // Alice tries to read Bob's burnout panel data.
    let url = format!("/api/users/{}/burnout", bob_id);
    let resp = alice_app.server.get(&url).await;

    assert_eq!(
        resp.status_code(),
        StatusCode::FORBIDDEN,
        "Alice must not read Bob's burnout data; got {}",
        resp.status_code()
    );

    // Unauthenticated request gets 401 (no cookie jar means no
    // session).
    let unauthed = TestApp::spawn().await;
    let resp = unauthed.server.get(&url).await;
    assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
}

// -------------------------------------------------------------------
// /api/users/{user_id}/capacity — Phase B (planned)
// -------------------------------------------------------------------

#[tokio::test]
#[ignore = "endpoint not implemented yet — Phase B"]
async fn capacity_endpoint_walls_off_other_users() {
    let (alice_app, _alice, _alice_id) = new_authed_app("alice").await;
    let bob_app = TestApp::spawn().await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&bob_app, &bob).await;

    let url = format!("/api/users/{}/capacity", bob_id);
    let resp = alice_app.server.get(&url).await;

    assert_eq!(resp.status_code(), StatusCode::FORBIDDEN);
}

// -------------------------------------------------------------------
// /api/users/{user_id}/notifications — Phase B (planned)
// -------------------------------------------------------------------

#[tokio::test]
#[ignore = "endpoint not implemented yet — Phase B"]
async fn notifications_endpoint_walls_off_other_users() {
    let (alice_app, _alice, _alice_id) = new_authed_app("alice").await;
    let bob_app = TestApp::spawn().await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&bob_app, &bob).await;

    let url = format!("/api/users/{}/notifications", bob_id);
    let resp = alice_app.server.get(&url).await;

    assert_eq!(resp.status_code(), StatusCode::FORBIDDEN);
}

// -------------------------------------------------------------------
// admin role does NOT bypass the boundary — Phase B (planned)
// -------------------------------------------------------------------

#[tokio::test]
#[ignore = "team admin role + personal data endpoints not yet co-existing — Phase B"]
async fn team_admin_cannot_read_member_personal_data() {
    // V2.1 §11.5.2 / §2.5: admin is a management role, NOT an
    // oversight role. An admin querying /api/users/{member_id}/
    // burnout must still receive 403.
    //
    // Test outline (to fill in once the endpoint exists):
    // 1. Create team T with admin = Alice
    // 2. Add Bob as team member
    // 3. Alice GETs /api/users/{bob_id}/burnout
    // 4. Assert 403, not 200
    todo!("write once Phase B adds team-membership + endpoint together");
}

// -------------------------------------------------------------------
// Cross-user POST endpoints (preferences, capacity edit) —
// Phase B (planned)
// -------------------------------------------------------------------

#[tokio::test]
#[ignore = "user-scoped POSTs not yet exposed by user_id path — Phase B"]
async fn cross_user_settings_post_returns_403() {
    // Once /api/users/{user_id}/wip-limit (or similar) exists as
    // an explicit user-scoped POST, Alice POSTing to Bob's URL
    // must 403. Currently /settings/wip-limit is implicitly
    // "self" — no need for this test until the explicit shape
    // lands.
    todo!("write once explicit user-scoped POSTs land");
}
