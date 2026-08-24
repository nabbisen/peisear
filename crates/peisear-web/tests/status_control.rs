//! `STATUS-001` (RFC 004a step 1) — a status control that works
//! without JavaScript, on the issue detail page's segment and on
//! each issue-list row. The board's own per-card control (DEV-002)
//! is untouched.
//!
//! Route shape: (b) from the handoff's §5 — one form-`POST` route
//! per surface (`/status/detail`, `/status/list`), mirroring the
//! existing `/status/board` route, rather than (a)'s single
//! generalised route. The three surfaces' redirect targets aren't
//! uniformly derivable from `project_id`/`issue_id` alone — the
//! list's needs to carry its filter/sort state forward, which the
//! detail page has none of — so (b) was the smaller change here.
//!
//! Coverage, matching the handoff's seven checks:
//! 1. `POST` the detail form directly → status changes, redirect
//!    lands on the detail page.
//! 2. Same for a list row → redirect lands on the list, preserving
//!    view parameters.
//! 3. A stale `client_updated_at` on either → 409.
//! 4. `aria-pressed` still marks the current status on the detail
//!    segment.
//! 5. The detail segments are keyboard-reachable — no `tabindex="-1"`.
//! 6. Regression guard: neither surface's control depends on script.
//! 7. The board's per-card control renders unchanged.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;
use peisear_core::IssueStatus;
use peisear_storage::issues;

fn location_of(resp: &axum_test::TestResponse) -> String {
    resp.headers()
        .get(axum::http::header::LOCATION)
        .expect("Location header present")
        .to_str()
        .expect("Location is ASCII")
        .to_string()
}

/// Pull `value="..."` from `<input type="hidden" name="{field}"
/// value="...">` in rendered HTML. Same minimal shape as
/// `confirmation.rs`'s own copy — duplicated rather than shared, the
/// same call `QA-002` made for `strip_line_comments`: small enough
/// that sharing costs more than it saves.
fn extract_hidden_field(body: &str, field: &str) -> Option<String> {
    let marker = format!(r#"name="{field}" value=""#);
    let start = body.find(&marker)? + marker.len();
    let end = body[start..].find('"')? + start;
    Some(body[start..end].to_string())
}

#[tokio::test]
async fn post_detail_form_changes_status_and_redirects_to_detail() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;
    let issue = issues::find(&app.db, &issue_id, &project_id)
        .await
        .expect("query issue");
    assert_eq!(issue.status, IssueStatus::Open, "fixture default");

    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{issue_id}/status/detail"
        ))
        .form(&[
            ("status", "in_progress"),
            ("client_updated_at", issue.updated_at.to_rfc3339().as_str()),
        ])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    assert_eq!(
        location_of(&resp),
        format!("/projects/{project_id}/issues/{issue_id}"),
        "detail status change should redirect back to the detail page"
    );

    let updated = issues::find(&app.db, &issue_id, &project_id)
        .await
        .expect("query issue");
    assert_eq!(updated.status, IssueStatus::InProgress);
}

#[tokio::test]
async fn post_list_row_form_changes_status_and_redirects_preserving_view_params() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;

    // Land on the filtered list view first, and read the row's own
    // hidden fields back rather than assuming their shape.
    let resp = app
        .server
        .get(&format!(
            "/projects/{project_id}?view=list&status=open&sort=priority"
        ))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    let client_updated_at = extract_hidden_field(&body, "client_updated_at")
        .expect("row carries the hidden client_updated_at field");
    let filter_status =
        extract_hidden_field(&body, "filter_status").expect("row carries filter_status");
    let filter_sort = extract_hidden_field(&body, "sort").expect("row carries sort");
    assert_eq!(filter_status, "open");
    assert_eq!(filter_sort, "priority");

    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{issue_id}/status/list"
        ))
        .form(&[
            ("status", "done"),
            ("client_updated_at", client_updated_at.as_str()),
            ("filter_status", filter_status.as_str()),
            ("filter_assignee", ""),
            ("sort", filter_sort.as_str()),
        ])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    let location = location_of(&resp);
    assert!(
        location.starts_with(&format!("/projects/{project_id}?view=list")),
        "list status change should redirect back to the list view: {location}"
    );
    assert!(
        location.contains("status=open") && location.contains("sort=priority"),
        "the view's filter/sort parameters should survive the round trip: {location}"
    );

    let updated = issues::find(&app.db, &issue_id, &project_id)
        .await
        .expect("query issue");
    assert_eq!(updated.status, IssueStatus::Done);
}

#[tokio::test]
async fn stale_client_updated_at_returns_409_on_either_route() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;

    let detail_issue_id = create_issue(&app.db, &project_id, &user_id, "Detail target").await;
    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{detail_issue_id}/status/detail"
        ))
        .form(&[
            ("status", "in_progress"),
            ("client_updated_at", "2020-01-01T00:00:00Z"),
        ])
        .await;
    resp.assert_status(StatusCode::CONFLICT);
    let unchanged = issues::find(&app.db, &detail_issue_id, &project_id)
        .await
        .expect("query issue");
    assert_eq!(
        unchanged.status,
        IssueStatus::Open,
        "a stale lock must not change the status"
    );

    let list_issue_id = create_issue(&app.db, &project_id, &user_id, "List target").await;
    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{list_issue_id}/status/list"
        ))
        .form(&[
            ("status", "in_progress"),
            ("client_updated_at", "2020-01-01T00:00:00Z"),
        ])
        .await;
    resp.assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn aria_pressed_marks_the_current_status_on_the_detail_segment() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains(r#"aria-pressed="true""#),
        "exactly one segment should be pressed (the current status): {body}"
    );
    assert!(
        body.contains(r#"aria-pressed="false""#),
        "the other segments should be unpressed: {body}"
    );
}

#[tokio::test]
async fn detail_segments_are_keyboard_reachable() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        !body.contains(r#"tabindex="-1""#),
        "the status segments must be keyboard-reachable now, not skipped: {body}"
    );
}

/// Test 6, this handoff's own reason for existing. Written so it
/// fails if either control becomes script-only again.
#[tokio::test]
async fn neither_surface_depends_on_script() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;

    let detail_resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}"))
        .await;
    let detail_body = detail_resp.text();
    assert!(
        detail_body.contains(&format!(
            r#"<form method="post" action="/projects/{project_id}/issues/{issue_id}/status/detail">"#
        )),
        "detail status control should be a plain form: {detail_body}"
    );
    assert!(
        !detail_body.contains("onclick=") && !detail_body.contains("onsubmit="),
        "detail status control must not depend on script: {detail_body}"
    );
    // `STATUS-001-review.md` §2: a form being present proves
    // nothing on its own — a `type="button"` segment inside a
    // perfectly good form is still completely inert without
    // JavaScript, which is exactly §17.4's shape and exactly what
    // this handoff exists to remove. The segments must actually be
    // submit controls.
    assert!(
        !detail_body.contains(r#"type="button""#),
        "no detail segment may be type=\"button\" -- that submits nothing: {detail_body}"
    );
    assert!(
        detail_body.contains(r#"type="submit""#) && detail_body.contains(r#"name="status""#),
        "detail segments must be real type=\"submit\" status controls: {detail_body}"
    );

    let list_resp = app
        .server
        .get(&format!("/projects/{project_id}?view=list"))
        .await;
    let list_body = list_resp.text();
    assert!(
        list_body.contains(&format!(
            r#"action="/projects/{project_id}/issues/{issue_id}/status/list""#
        )),
        "list row status control should be a plain form: {list_body}"
    );
    assert!(
        !list_body.contains("onclick=") && !list_body.contains("onsubmit="),
        "list row status control must not depend on script: {list_body}"
    );
    assert!(
        !list_body.contains(r#"type="button""#),
        "no list-row segment may be type=\"button\" -- that submits nothing: {list_body}"
    );
    assert!(
        list_body.contains(r#"type="submit""#) && list_body.contains(r#"name="status""#),
        "list-row segments must be real type=\"submit\" status controls: {list_body}"
    );
}

#[tokio::test]
async fn boards_per_card_control_renders_unchanged() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let _issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=board"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains("/status/board"),
        "board card status control should still post to /status/board: {body}"
    );
    assert!(
        !body.contains("/status/detail") && !body.contains("/status/list"),
        "the board must not pick up either of the new surfaces' routes: {body}"
    );
}
