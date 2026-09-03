//! Tests for the Phase B PR3 (B-4) status segment UI.
//!
//! The detail page replaces the single status badge with a
//! three-segment control (Open / In Progress / Done) showing
//! all three statuses at once, with the current status visually
//! highlighted. Originally display-only; `STATUS-001` (RFC 004a
//! step 1) made it a real `<form>` that posts the clicked
//! segment's status, with no script involved
//! (`crates/peisear-web/tests/status_control.rs` covers that
//! behaviour directly). The structural assertions below —
//! group wrapper, all three labels, `aria-pressed` marking the
//! current status — hold regardless of whether the segments
//! are inert or submit; nothing here needed to change.
//!
//! What we assert here:
//!
//! 1. All three status labels appear in the rendered HTML so a
//!    user can see the full lifecycle at a glance.
//! 2. The current status segment carries `aria-pressed="true"`
//!    and the others `aria-pressed="false"` — the active state
//!    is conveyed semantically to assistive tech, not just
//!    visually.

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

    // The group wrapper -- co-location, not two independent facts:
    // the literal rendered tag (Leptos attribute order:
    // `role`, `aria-label`, `class`).
    assert!(
        body.contains(r#"<div role="group" aria-label="Issue status" class="join mb-3">"#),
        "status segment group wrapper missing: {body}"
    );

    // Scoped to the segment container, not the whole page.
    // `TT-003` §5, confirmed by planting: this page also renders a
    // `status-enhancement-copy` JSON island (`JS-003`'s `movedTo`
    // client-side copy: "Moved to Open.", "Moved to In Progress.",
    // "Moved to Done.") -- an unrelated data blob that happens to
    // contain the same three words. Blanking the segment buttons'
    // own labels left the old unscoped checks passing, satisfied
    // entirely by that JSON island.
    let segment_start = body
        .find(r#"<div role="group" aria-label="Issue status" class="join mb-3">"#)
        .expect("segment container present");
    let segment_end = body[segment_start..]
        .find("</div>")
        .map(|i| segment_start + i)
        .expect("segment container has a closing </div>");
    let segment = &body[segment_start..segment_end];

    // All three labels present (Open / In Progress / Done).
    // We look for the literal labels as rendered, which lets
    // this test catch wording changes intentionally.
    assert!(
        segment.contains("Open"),
        "missing 'Open' segment label: {segment}"
    );
    assert!(
        segment.contains("In Progress"),
        "missing 'In Progress' segment label: {segment}"
    );
    assert!(
        segment.contains("Done"),
        "missing 'Done' segment label: {segment}"
    );

    // The active segment carries aria-pressed="true" -- counted, not
    // just `contains`, and scoped to the segment: three buttons
    // total, so exactly one true and exactly two false (same
    // double-pressed gap already found and fixed in
    // `status_control.rs`'s sibling test).
    let true_count = segment.matches(r#"aria-pressed="true""#).count();
    let false_count = segment.matches(r#"aria-pressed="false""#).count();
    assert_eq!(
        true_count, 1,
        "expected exactly one segment pressed, found {true_count}: {segment}"
    );
    assert_eq!(
        false_count, 2,
        "expected exactly two segments unpressed, found {false_count}: {segment}"
    );
}

#[tokio::test]
async fn edit_form_keeps_existing_select_widget() {
    // The segment is now also a mutation path (`STATUS-001`), but
    // the edit form is a separate, older one and still uses a
    // `<select name="status">`. This guards against an accidental
    // future change that collapses the two paths by replacing the
    // select with the segment in edit mode.
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
