//! Breadcrumb / back-link rendering tests for v2.1 spec §4.4.
//!
//! These verify the **structural** invariants:
//!
//! - Every detail page begins its breadcrumb with the v2.1 entry
//!   point (a link to `/today`).
//! - The terminal node carries `aria-current="page"`.
//! - A `Back to ...` link is rendered beneath the breadcrumb.
//!
//! We grep the rendered HTML rather than parse the DOM. Two
//! reasons: (1) it keeps the test crate free of an HTML-parser
//! dep; (2) the assertions express the contract literally —
//! "the substring `aria-current=\"page\"` must appear in the
//! response body" is what a screen reader will look for too.
//!
//! When Phase B reworks visual styling, these substring checks
//! continue to pass as long as the ARIA contract holds, which
//! is exactly the right level of coupling.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;

#[tokio::test]
async fn project_detail_breadcrumb_starts_with_today() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id =
        create_personal_project(&app.db, &user_id, "Customer Portal").await;

    let resp = app.server.get(&format!("/projects/{project_id}")).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // 1. Leading entry: a link to /today labelled "Today".
    assert!(
        body.contains(r#"href="/today""#),
        "project detail breadcrumb missing /today entry-point link"
    );
    // 2. The Projects ancestor link.
    assert!(
        body.contains(r#"href="/projects""#),
        "project detail breadcrumb missing Projects link"
    );
    // 3. Terminal node carries aria-current="page".
    assert!(
        body.contains(r#"aria-current="page""#),
        "project detail breadcrumb missing aria-current=\"page\" on \
         terminal node"
    );
    // 4. Back link to projects list.
    assert!(
        body.contains("Back to Projects"),
        "project detail page missing 'Back to Projects' affordance"
    );
}

#[tokio::test]
async fn issue_detail_breadcrumb_full_chain() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("bob");
    let user_id = register_and_login(&app, &user).await;
    let project_id =
        create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let issue_id =
        create_issue(&app.db, &project_id, &user_id, "Login error").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // Today entry point.
    assert!(body.contains(r#"href="/today""#), "missing /today link");
    // Projects ancestor.
    assert!(
        body.contains(r#"href="/projects""#),
        "missing /projects link"
    );
    // Project ancestor (link to project detail).
    let project_link = format!(r#"href="/projects/{project_id}""#);
    assert!(
        body.contains(&project_link),
        "missing parent-project link {project_link}"
    );
    // Terminal aria-current.
    assert!(
        body.contains(r#"aria-current="page""#),
        "missing aria-current on terminal node"
    );
    // Back link should target the parent project (where the issue
    // list lives), not /projects.
    assert!(
        body.contains("Back to issues"),
        "issue detail page missing 'Back to issues' link"
    );
}
