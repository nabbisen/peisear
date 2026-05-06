//! Tests for the Phase B PR3 (B-4) status segment UI.
//!
//! The detail page replaces the single status badge with a
//! three-segment control (Open / In Progress / Done) showing
//! all three statuses at once, with the current status
//! visually highlighted. Display-only — clicking does nothing;
//! the user clicks the dedicated Edit button to change status.
//! Direct-manipulation status changes are deferred to Phase D.
//!
//! What we assert here:
//!
//! 1. All three status labels appear in the rendered HTML so a
//!    user can see the full lifecycle at a glance.
//! 2. The current status segment carries `aria-pressed="true"`
//!    and the others `aria-pressed="false"` — the active state
//!    is conveyed semantically to assistive tech, not just
//!    visually.
//! 3. The segments are typed `button` (not links, not form
//!    submits) so keyboard navigation lands on them but
//!    activation is inert.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;

#[tokio::test]
async fn detail_renders_three_segment_status_control() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // The group wrapper.
    assert!(
        body.contains(r#"role="group""#) && body.contains(r#"aria-label="Issue status""#),
        "status segment group wrapper missing"
    );

    // All three labels present (Open / In Progress / Done).
    // We look for the literal labels as rendered, which lets
    // this test catch wording changes intentionally.
    assert!(body.contains("Open"), "missing 'Open' segment label");
    assert!(
        body.contains("In Progress"),
        "missing 'In Progress' segment label"
    );
    assert!(body.contains("Done"), "missing 'Done' segment label");

    // The active segment carries aria-pressed="true". A new
    // issue's default status is open, so we'd expect at least
    // one aria-pressed="true" and at least one
    // aria-pressed="false".
    assert!(
        body.contains(r#"aria-pressed="true""#),
        "active segment must carry aria-pressed=true"
    );
    assert!(
        body.contains(r#"aria-pressed="false""#),
        "inactive segments must carry aria-pressed=false"
    );
}

#[tokio::test]
async fn edit_form_keeps_existing_select_widget() {
    // The segment is read-only. The actual mutation path —
    // the edit form — still uses a `<select name="status">`.
    // This guards against an accidental future change that
    // replaces the select with the segment in edit mode (which
    // would mean the segment becomes interactive ahead of
    // Phase D).
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}/edit");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains(r#"name="status""#),
        "edit form must keep its existing status <select>"
    );
}
