//! Shared assertions for cross-cutting v2.1 spec invariants.
//!
//! These helpers exist so that authorization and optimistic-lock
//! tests across `tests/auth_boundary.rs` and
//! `tests/optimistic_lock.rs` (and future suites) make their
//! intent explicit at the call site:
//!
//! ```ignore
//! assertion::expects_403_for_other_user(&alice_app, &alice,
//!     &format!("/api/users/{}/burnout", bob_id)).await;
//! ```
//!
//! reads more clearly than the same pattern open-coded in each
//! test.

use axum::http::StatusCode;
use axum_test::TestServer;
use peisear_web::components::TOUCH_TARGET;

/// The URL is a peisear endpoint that exposes personal data of
/// `target_user_id`. Asserts:
///
/// 1. With no auth, the URL returns 401 Unauthorized.
/// 2. With auth as a *different* user, the URL returns 403
///    Forbidden — never 200, never 404 (the latter would leak
///    "this user_id doesn't exist" information; per spec §11.5.2
///    we use 403 even for non-existent ids).
///
/// `unauthed_server` is a fresh `TestServer` without cookies —
/// typically `TestApp::spawn().await.server`.
/// `other_user_server` is a `TestServer` whose cookie jar holds
/// a session for some user *other* than `target_user_id`.
pub async fn personal_data_endpoint_is_walled_off(
    unauthed_server: &TestServer,
    other_user_server: &TestServer,
    url: &str,
) {
    let unauthed = unauthed_server.get(url).await;
    assert_eq!(
        unauthed.status_code(),
        StatusCode::UNAUTHORIZED,
        "expected 401 Unauthorized for unauthenticated GET {url}, got {}",
        unauthed.status_code()
    );

    let other_user = other_user_server.get(url).await;
    assert_eq!(
        other_user.status_code(),
        StatusCode::FORBIDDEN,
        "expected 403 Forbidden for cross-user GET {url}, got {}",
        other_user.status_code()
    );
}

/// Asserts a mutation endpoint enforces optimistic locking via
/// the `client_updated_at` field.
///
/// Phase A (when this helper is first used) the production code
/// may not yet implement the check on every endpoint. Tests that
/// exercise un-instrumented endpoints should be marked with
/// `#[ignore]` until the corresponding endpoint is migrated.
///
/// `app` should already have an authenticated session.
/// `mutation_url` is the POST endpoint to hit.
/// `valid_body_with_stale_timestamp` is a urlencoded body whose
/// `client_updated_at` is older than the entity's current value.
///
/// Asserts the response is 409 Conflict. (Body shape per
/// peisear-feature-spec-v2.1 §21.4.5 / Appendix E.3.3 is checked
/// elsewhere; this helper asserts only the status code, since
/// many endpoints will hand-roll the conflict response shape
/// during incremental rollout.)
pub async fn stale_update_returns_409(
    server: &TestServer,
    mutation_url: &str,
    valid_body_with_stale_timestamp: &str,
) {
    let resp = server
        .post(mutation_url)
        .add_header(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .text(valid_body_with_stale_timestamp.to_string())
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "expected 409 Conflict for POST {mutation_url} with stale \
         client_updated_at, got {} (body: {})",
        resp.status_code(),
        resp.text()
    );
}

/// Asserts `markup` carries every utility in
/// [`peisear_web::components::TOUCH_TARGET`] — checked one axis at a
/// time (`TOUCH_TARGET.split_whitespace()`), so a failure names which
/// axis is missing rather than only that the pair as a whole isn't
/// present.
///
/// `TT-003` (`§10.15`/`TT-002-round2-review.md` §4): `board_keyboard.rs`
/// and `confirmation.rs` used to assert the literal `"min-h-11"` /
/// `"min-w-11"` directly — `grow()`'s premise (one home for the 44px
/// fact) undone one layer up, in test code rather than production.
/// Reading the constant here instead means a future change to
/// `TOUCH_TARGET` is what these tests track, not a frozen copy of its
/// value at the time they were written.
pub fn meets_the_touch_target_minimum(markup: &str, what: &str) {
    for utility in TOUCH_TARGET.split_whitespace() {
        assert!(
            markup.contains(utility),
            "{what} must carry {utility:?} (part of components::TOUCH_TARGET, \
             {TOUCH_TARGET:?}) -- NFR-A11Y-007's 44px floor; markup: {markup}"
        );
    }
}
