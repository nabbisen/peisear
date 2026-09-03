//! DEV-002 (RFC 007) — keyboard-operable status control on the
//! kanban board.
//!
//! The board's only status-change path used to be a mouse drag
//! (`board.js`). `FR-DM-002` (P0) requires a keyboard equivalent
//! producing the identical effect, with no mouse-only action
//! remaining. Each card now also carries a plain `<form method="post">`
//! with one submit button per reachable target status, sharing the
//! same `apply_status_change` lock check the drag path uses
//! (DEV-001).

mod common;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use common::assertion;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::{TestApp, ensure_distinct_timestamp};

#[tokio::test]
async fn board_renders_a_form_based_status_control_per_card() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}?view=board");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let expected_action = format!("/projects/{project_id}/issues/{issue_id}/status/board");
    assert!(
        body.contains(&expected_action),
        "expected a form posting to the keyboard status endpoint; body: {body}"
    );
    assert!(
        body.contains(r#"name="client_updated_at""#),
        "the keyboard status form must carry the lock token as a hidden field"
    );
    // A fresh issue defaults to "open"; the reachable targets are
    // the other two statuses, so both should appear as submit
    // buttons.
    assert!(
        body.contains(r#"name="status" value="in_progress""#),
        "missing a submit control moving to In Progress"
    );
    assert!(
        body.contains(r#"name="status" value="done""#),
        "missing a submit control moving to Done"
    );
    // The current status ("open") must not itself be offered as a
    // target — moving to the status you're already in isn't a
    // reachable transition.
    assert!(
        !body.contains(r#"name="status" value="open""#),
        "the current status must not be offered as a target"
    );
}

#[tokio::test]
async fn keyboard_status_change_with_valid_token_succeeds_and_redirects() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let t0 = read_issue_updated_at(&app, &issue_id).await;
    let resp = post_status_change_form(&app, &project_id, &issue_id, "in_progress", &t0).await;

    assert_eq!(
        resp.status_code(),
        StatusCode::SEE_OTHER,
        "a valid keyboard status change must redirect (PRG), got {}",
        resp.status_code()
    );
    let location = resp
        .headers()
        .get("location")
        .expect("redirect must carry a Location header")
        .to_str()
        .unwrap();
    assert!(
        location.contains(&format!("/projects/{project_id}")) && location.contains("view=board"),
        "expected the redirect to land back on the board, got {location}"
    );

    let status_after = read_issue_status(&app, &issue_id).await;
    assert_eq!(status_after, "in_progress", "status must have changed");
}

#[tokio::test]
async fn keyboard_status_change_with_missing_token_is_rejected() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}/status/board");
    let resp = app
        .server
        .post(&url)
        .form(&[("status", "in_progress")])
        .await;

    assert_eq!(
        resp.status_code(),
        StatusCode::BAD_REQUEST,
        "missing client_updated_at must be rejected with 400, got {}",
        resp.status_code()
    );

    let status_after = read_issue_status(&app, &issue_id).await;
    assert_eq!(
        status_after, "open",
        "a rejected keyboard status change must not mutate the row"
    );
}

#[tokio::test]
async fn keyboard_status_change_with_stale_token_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let t0 = read_issue_updated_at(&app, &issue_id).await;

    // First change succeeds and advances updated_at. SQLite's
    // whole-second `updated_at` precision means two writes inside
    // the same wall-clock second would collide, so force the clock
    // forward the same way optimistic_lock.rs does.
    ensure_distinct_timestamp().await;
    let resp = post_status_change_form(&app, &project_id, &issue_id, "in_progress", &t0).await;
    assert_eq!(resp.status_code(), StatusCode::SEE_OTHER);

    // Re-using the now-stale t0 must 409, leaving status unchanged.
    let resp = post_status_change_form(&app, &project_id, &issue_id, "done", &t0).await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on the keyboard path must 409, got {}",
        resp.status_code()
    );

    let status_after = read_issue_status(&app, &issue_id).await;
    assert_eq!(
        status_after, "in_progress",
        "a 409 must leave the row at its post-first-change state"
    );
}

#[tokio::test]
async fn each_status_control_has_a_distinguishing_accessible_name() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let _issue_a = create_issue(&app.db, &project_id, &user_id, "Fix the login bug").await;
    let _issue_b = create_issue(&app.db, &project_id, &user_id, "Write onboarding docs").await;

    let url = format!("/projects/{project_id}?view=board");
    let resp = app.server.get(&url).await;
    let body = resp.text();

    // Both issue titles must appear inside an aria-label so a
    // screen-reader user can tell the two cards' controls apart —
    // not just "Done" repeated with no context. The renderer
    // HTML-escapes the quotes around the issue title.
    assert!(
        body.contains(r#"aria-label="Move &quot;Fix the login bug&quot; to Done""#),
        "expected an accessible name naming both the issue and the target \
         status for issue A; body: {body}"
    );
    assert!(
        body.contains(r#"aria-label="Move &quot;Write onboarding docs&quot; to Done""#),
        "expected an accessible name naming both the issue and the target \
         status for issue B; body: {body}"
    );
}

/// `QA-014` §4.1 (`NFR-A11Y-007`): the board's per-card status buttons
/// were the first control in `src/components/` to meet the 44px
/// touch-target minimum — `min-h-11 min-w-11` on top of `btn-xs`'s own
/// 24px box, since joined by 136 more (`TT-002`, `tests/touch_target.rs`)
/// composed from the same fact via `components::grow`. Nothing asserted
/// this control specifically until now; the baseline's claim that
/// `board_keyboard` verified `NFR-A11Y-007` was never true.
///
/// **Scoped to each `<button ... name="status" ...>` tag
/// individually**, not a bare `body.contains(...)` — `TT-002-review.md`
/// §1 found this exact test still passing with this button's own
/// `grow()` call removed, because other controls on the same board
/// page (e.g. the per-row status `join` on `issues.rs`) also carry the
/// same classes once `TT-002` shipped, so an unscoped check no longer
/// proves anything about *this* control specifically. Same shape as
/// `touch_target.rs`'s `grown_input_reaches_the_rendered_page` fix.
///
/// **Reads `components::TOUCH_TARGET`** (`common::assertion::
/// meets_the_touch_target_minimum`), not the literal `"min-h-11"`/
/// `"min-w-11"` — `TT-003` §4 collapsed this and `confirmation.rs`'s
/// four sites onto the one production constant.
#[tokio::test]
async fn per_card_status_button_meets_the_touch_target_minimum() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let _issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}?view=board");
    let resp = app.server.get(&url).await;
    let body = resp.text();

    let mut checked = 0;
    let mut rest = body.as_str();
    while let Some(button_start) = rest.find("<button") {
        let tag_end = rest[button_start..]
            .find('>')
            .expect("a <button tag has a closing '>'");
        let tag = &rest[button_start..button_start + tag_end];
        if tag.contains(r#"name="status""#) {
            assertion::meets_the_touch_target_minimum(tag, "a board status-move button");
            checked += 1;
        }
        rest = &rest[button_start + tag_end..];
    }
    assert!(
        checked > 0,
        "expected to find at least one status-move <button name=\"status\"> \
         on the board page; body: {body}"
    );
}

#[tokio::test]
async fn board_contains_no_prohibited_vocabulary() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let _issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}?view=board");
    let resp = app.server.get(&url).await;
    let body = resp.text();

    for bad in ["Failed", "Error:", "achievement", "congrat"] {
        assert!(
            !body.contains(bad),
            "board must not contain prohibited vocabulary; found {bad:?}"
        );
    }
}

// -------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------

async fn read_issue_updated_at(app: &TestApp, issue_id: &str) -> String {
    let (updated_at,): (DateTime<Utc>,) =
        sqlx::query_as(r#"SELECT updated_at FROM issues WHERE id = ?1"#)
            .bind(issue_id)
            .fetch_one(&app.db)
            .await
            .expect("read updated_at");
    updated_at.to_rfc3339()
}

async fn read_issue_status(app: &TestApp, issue_id: &str) -> String {
    let (status,): (String,) = sqlx::query_as(r#"SELECT status FROM issues WHERE id = ?1"#)
        .bind(issue_id)
        .fetch_one(&app.db)
        .await
        .expect("read issue status");
    status
}

async fn post_status_change_form(
    app: &TestApp,
    project_id: &str,
    issue_id: &str,
    status: &str,
    client_updated_at: &str,
) -> axum_test::TestResponse {
    let url = format!("/projects/{project_id}/issues/{issue_id}/status/board");
    app.server
        .post(&url)
        .form(&[("status", status), ("client_updated_at", client_updated_at)])
        .await
}
