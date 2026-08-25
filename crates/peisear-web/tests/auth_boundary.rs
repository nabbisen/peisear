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
//! ## Coverage note (RFC 007 / DEV-005 item B)
//!
//! This file used to hold a placeholder `#[ignore]`d test,
//! `cross_user_settings_post_returns_403`, awaiting a user-scoped
//! POST endpoint (`/api/users/{user_id}/wip-limit` or similar,
//! `FR-API-006`) that has never been scheduled. It was withdrawn
//! rather than left `#[ignore]`d — an ignored test on a privacy
//! boundary reads as coverage that does not exist. Settings
//! mutations (`/settings/wip-limit`, `/settings/capacity/*`) are
//! session-scoped, not addressed by `user_id` in the path, so
//! "cross-user POST" isn't expressible against them today. If
//! `FR-API-006` ever lands a user-scoped POST surface, reinstate an
//! equivalent test against it.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, logout, new_authed_app, register_and_login};
use common::fixture::{create_issue, create_personal_project};
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
async fn team_admin_cannot_read_member_personal_data() {
    // V2.1 §11.5.2 / §2.5: admin is a management role, NOT an
    // oversight role. An admin querying /api/users/{member_id}/
    // burnout must still receive 403. The boundary is "self
    // access only" — admin status doesn't bypass it.
    //
    // Both users live in the same DB so the team membership is
    // real (cross-DB tests can't add a membership row that's
    // visible to both apps' connection pools).
    let app = TestApp::spawn().await;

    // Register Alice (will become team admin) and stay logged in.
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;

    // Register Bob in the same DB. axum-test's cookie jar gets
    // overwritten by the registration (Bob's session replaces
    // Alice's), so we re-login as Alice afterward.
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&app, &bob).await;
    common::auth::logout(&app).await;
    common::auth::login(&app, &alice).await;

    // Make Alice the admin of a team and add Bob as a member.
    let team_id = common::fixture::create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    peisear_storage::teams::add_member(
        &app.db,
        &team_id,
        &bob_id,
        peisear_core::teams::TeamRole::Member,
    )
    .await
    .expect("add Bob to team");

    // Alice (team admin) tries to read Bob's burnout. Must 403.
    let url = format!("/api/users/{}/burnout", bob_id);
    let resp = app.server.get(&url).await;
    assert_eq!(
        resp.status_code(),
        StatusCode::FORBIDDEN,
        "team admin must NOT bypass the §11.5 self-access boundary; got {}",
        resp.status_code()
    );

    // Same for capacity and notifications — admin is not
    // oversight on any of these.
    let url = format!("/api/users/{}/capacity", bob_id);
    let resp = app.server.get(&url).await;
    assert_eq!(resp.status_code(), StatusCode::FORBIDDEN);

    let url = format!("/api/users/{}/notifications", bob_id);
    let resp = app.server.get(&url).await;
    assert_eq!(resp.status_code(), StatusCode::FORBIDDEN);
}

// -------------------------------------------------------------------
// Login-failure indistinguishability — I18N-005e §7 / FR-AUTH-002
// -------------------------------------------------------------------

/// `FR-AUTH-002`: a failed login must not disclose which field was
/// wrong. `handlers/auth.rs::login_submit` already converges
/// "unknown account" and "wrong password" onto the same code path
/// and the same `MessageKey::InvalidCredentialsMessage` — this test
/// is the assertion `I18N-005e` §7 explicitly asks for, so a future
/// change that reintroduces per-field wording fails a test instead
/// of only a code review.
#[tokio::test]
async fn login_failure_message_is_identical_for_unknown_account_and_wrong_password() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    // Register normally, but don't keep the session — we want two
    // fresh unauthenticated login attempts against the same server.
    common::auth::register(&app, &user).await;
    common::auth::logout(&app).await;

    // Case 1: unknown account entirely.
    let unknown_email = format!("no-such-user-{}@example.com", user.email);
    let resp_unknown = app
        .server
        .post("/login")
        .form(&[
            ("email", unknown_email.as_str()),
            ("password", "wrong-password-entirely"),
        ])
        .await;

    // Case 2: real account, wrong password.
    let resp_wrong_password = app
        .server
        .post("/login")
        .form(&[
            ("email", user.email.as_str()),
            ("password", "wrong-password-entirely"),
        ])
        .await;

    assert_eq!(
        resp_unknown.status_code(),
        resp_wrong_password.status_code(),
        "unknown-account and wrong-password login failures must return the same status"
    );
    assert_eq!(
        resp_unknown.text(),
        resp_wrong_password.text(),
        "unknown-account and wrong-password login failures must render byte-identical \
         bodies — a difference here would let an attacker enumerate valid accounts"
    );
    assert!(
        resp_unknown.text().contains("Invalid email or password."),
        "expected the neutral InvalidCredentialsMessage text in the error response; got: {}",
        resp_unknown.text()
    );
}

// -------------------------------------------------------------------
// Positive cases — self access on /api/users/{user_id}/* succeeds
// -------------------------------------------------------------------

#[tokio::test]
async fn self_can_read_own_burnout() {
    // The negative tests above prove the boundary; this confirms
    // a legitimate self-read returns 200 with a JSON body
    // shaped per the API spec.
    let (app, _user, user_id) = new_authed_app("alice").await;

    let url = format!("/api/users/{}/burnout", user_id);
    let resp = app.server.get(&url).await;
    assert_eq!(
        resp.status_code(),
        StatusCode::OK,
        "self-access should succeed; got {}",
        resp.status_code()
    );
    // Spot-check one stable field rather than the whole shape —
    // shape iteration is allowed during Phase B (decision B-E3).
    let body = resp.text();
    assert!(
        body.contains("\"user_id\""),
        "burnout response missing user_id field: {body}"
    );
    assert!(
        body.contains("\"indicator\""),
        "burnout response missing indicator field: {body}"
    );
}

#[tokio::test]
async fn self_can_read_own_capacity() {
    let (app, _user, user_id) = new_authed_app("alice").await;
    let url = format!("/api/users/{}/capacity", user_id);
    let resp = app.server.get(&url).await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body = resp.text();
    assert!(body.contains("\"effective_today\""));
    assert!(body.contains("\"rows\""));
}

#[tokio::test]
async fn self_can_read_own_notifications() {
    let (app, _user, user_id) = new_authed_app("alice").await;
    let url = format!("/api/users/{}/notifications", user_id);
    let resp = app.server.get(&url).await;
    assert_eq!(resp.status_code(), StatusCode::OK);
    let body = resp.text();
    assert!(body.contains("\"unread_count\""));
    assert!(body.contains("\"items\""));
}

#[tokio::test]
async fn unauthed_api_users_returns_401_not_redirect() {
    // Critical UX difference between AppError and ApiAppError:
    // browser AppError unauth → 303 redirect to /login.
    // /api/* ApiAppError unauth → 401 with JSON body, no
    // redirect. The JSON client (typeahead JS, future
    // dashboards) handles auth state itself; redirecting it to
    // a login page would just leak HTML into the JSON parser.
    let app = TestApp::spawn().await;
    // No login on this app.
    let resp = app.server.get("/api/users/some-id/burnout").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::UNAUTHORIZED,
        "/api/* unauth must be 401, not redirect; got {}",
        resp.status_code()
    );
    // Body must be JSON, not the login HTML.
    let body = resp.text();
    assert!(
        body.contains("\"error\""),
        "/api/* unauth response should be JSON; got: {body}"
    );
    assert!(
        body.contains("\"unauthorized\""),
        "/api/* unauth body should carry the 'unauthorized' code; got: {body}"
    );
}

// -------------------------------------------------------------------
// `QA-007` (RFC 005 §1) — the two `STATUS-001` form routes, added in
// 0.25.0 and never independently audited. Same `find_accessible`
// check `apply_status_change` gives every status-change entry point;
// posted straight to the route with no prior `GET`, matching
// `confirmation::authorisation_matches_the_corresponding_post_per_
// route`'s shape for the three delete routes.
// -------------------------------------------------------------------

#[tokio::test]
async fn status_detail_post_walls_off_a_user_with_no_project_access() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;
    let project_id = create_personal_project(&app.db, &owner_id, "Private Project").await;
    let issue_id = create_issue(&app.db, &project_id, &owner_id, "Fix login bug").await;

    logout(&app).await;
    let bob = TestUser::new("bob");
    register_and_login(&app, &bob).await;

    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{issue_id}/status/detail"
        ))
        .form(&[
            ("status", "in_progress"),
            ("client_updated_at", "irrelevant"),
        ])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::NOT_FOUND,
        "a user with no access to the project must not be able to change status via /status/detail; got {}",
        resp.status_code()
    );
}

#[tokio::test]
async fn status_list_post_walls_off_a_user_with_no_project_access() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;
    let project_id = create_personal_project(&app.db, &owner_id, "Private Project").await;
    let issue_id = create_issue(&app.db, &project_id, &owner_id, "Fix login bug").await;

    logout(&app).await;
    let bob = TestUser::new("bob");
    register_and_login(&app, &bob).await;

    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{issue_id}/status/list"
        ))
        .form(&[
            ("status", "in_progress"),
            ("client_updated_at", "irrelevant"),
            ("filter_status", ""),
            ("filter_assignee", ""),
            ("sort", ""),
        ])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::NOT_FOUND,
        "a user with no access to the project must not be able to change status via /status/list; got {}",
        resp.status_code()
    );
}

// -------------------------------------------------------------------
// `QA-007` §4 — `/inbox/{id}/read` was in neither RFC 005 §1's table
// nor this file, despite mutating a single row that (unlike the
// session-scoped routes above) genuinely has a resource id in the
// path. `notif_store::mark_read`'s own query is `WHERE id = ?1 AND
// user_id = ?2` — storage-layer scoping, not merely "no user_id in
// the URL" — but that claim had never been exercised against a real
// cross-user attempt before this test.
// -------------------------------------------------------------------

#[tokio::test]
async fn mark_read_does_not_affect_another_users_notification() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;

    let notif_id = peisear_storage::notifications::insert(
        &app.db,
        &owner_id,
        peisear_storage::notifications::NewNotification {
            kind: peisear_core::notifications::kind::BURNOUT_OVERLOAD,
            severity: peisear_core::notifications::Severity::Watch,
            title: "Sustained over-capacity streak",
            body: "Test body.",
            payload_json: None,
            dispatched_via: &["in_app"],
        },
    )
    .await
    .expect("insert notification");

    logout(&app).await;
    let bob = TestUser::new("bob");
    register_and_login(&app, &bob).await;

    // `notif_store::mark_read`'s own "exists at all" fallback check
    // is *also* scoped to `user_id`, so bob's attempt on alice's
    // notification finds nothing under his id and 404s -- an
    // explicit rejection, not the silent same-user "already read"
    // no-op the handler's doc comment describes. Better than what
    // this test set out to prove was even true.
    let resp = app.server.post(&format!("/inbox/{notif_id}/read")).await;
    resp.assert_status(StatusCode::NOT_FOUND);

    let still_unread = peisear_storage::notifications::recent_for_user(&app.db, &owner_id, 10)
        .await
        .expect("query alice's notifications")
        .into_iter()
        .find(|n| n.id == notif_id)
        .expect("alice's notification still exists");
    assert!(
        still_unread.read_at.is_none(),
        "bob's POST to /inbox/{{id}}/read must not mark alice's notification read"
    );
}

/// `QA-007-review.md` §2: `/inbox/mark-all-read` has no `{user_id}`
/// in its path, but "no `user_id` in the path" only rules out
/// *impersonation* — it says nothing about a **bulk** write, where
/// the cross-user exposure lives in the `WHERE` clause rather than
/// the URL. `mark_all_read`'s query is `WHERE user_id = ?1 AND
/// read_at IS NULL`; that predicate is the only thing standing
/// between "my unread notifications" and "everyone's."
#[tokio::test]
async fn mark_all_read_does_not_affect_another_users_notifications() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;

    peisear_storage::notifications::insert(
        &app.db,
        &owner_id,
        peisear_storage::notifications::NewNotification {
            kind: peisear_core::notifications::kind::BURNOUT_OVERLOAD,
            severity: peisear_core::notifications::Severity::Watch,
            title: "Sustained over-capacity streak",
            body: "Test body.",
            payload_json: None,
            dispatched_via: &["in_app"],
        },
    )
    .await
    .expect("insert notification");

    logout(&app).await;
    let bob = TestUser::new("bob");
    register_and_login(&app, &bob).await;

    let resp = app.server.post("/inbox/mark-all-read").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let alice_notifications =
        peisear_storage::notifications::recent_for_user(&app.db, &owner_id, 10)
            .await
            .expect("query alice's notifications");
    assert!(
        alice_notifications.iter().all(|n| n.read_at.is_none()),
        "bob's POST to /inbox/mark-all-read must not mark alice's notifications read: {alice_notifications:?}"
    );
}

/// The other half of the pair above (`QA-008` §3, RFC 005 §1 "a bulk route
/// needs both assertions"). The cross-user test only asserts what
/// `mark_all_read` must **not** do, and that alone is satisfied by a route
/// that does nothing at all: replacing its predicate with `WHERE user_id =
/// 'nobody'` left the button inert for every user and `cargo test
/// --workspace` at 195 passed, 0 failed. This asserts what it **must** do —
/// alice's own unread notifications actually become read when she posts to
/// her own `/inbox/mark-all-read`.
#[tokio::test]
async fn mark_all_read_marks_the_callers_own_unread_notifications_read() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;

    for _ in 0..2 {
        peisear_storage::notifications::insert(
            &app.db,
            &owner_id,
            peisear_storage::notifications::NewNotification {
                kind: peisear_core::notifications::kind::BURNOUT_OVERLOAD,
                severity: peisear_core::notifications::Severity::Watch,
                title: "Sustained over-capacity streak",
                body: "Test body.",
                payload_json: None,
                dispatched_via: &["in_app"],
            },
        )
        .await
        .expect("insert notification");
    }

    let resp = app.server.post("/inbox/mark-all-read").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let alice_notifications =
        peisear_storage::notifications::recent_for_user(&app.db, &owner_id, 10)
            .await
            .expect("query alice's notifications");
    assert_eq!(
        alice_notifications.len(),
        2,
        "setup should have inserted exactly two notifications: {alice_notifications:?}"
    );
    assert!(
        alice_notifications.iter().all(|n| n.read_at.is_some()),
        "alice's own POST to /inbox/mark-all-read must mark her own unread notifications read: {alice_notifications:?}"
    );
}
