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

/// The `<nav aria-label="Breadcrumb">...</nav>` block's own markup,
/// not the whole page. `TT-003` §5, confirmed by planting: the
/// navbar's account-dropdown menu also links to `href="/today"`
/// (`components/layout.rs`), and the navbar's brand/logo link also
/// points to `href="/projects"` -- both render on every authenticated
/// page, independent of whatever the breadcrumb component itself
/// produces, so unscoped checks for either stayed green with the
/// breadcrumb's own entries removed.
fn breadcrumb_nav(body: &str) -> &str {
    let marker = r#"aria-label="Breadcrumb""#;
    let marker_at = body
        .find(marker)
        .expect("breadcrumb nav aria-label present");
    let nav_start = body[..marker_at]
        .rfind("<nav")
        .expect("a <nav tag precedes the breadcrumb aria-label");
    let nav_end = body[nav_start..]
        .find("</nav>")
        .map(|i| nav_start + i)
        .expect("breadcrumb nav has a closing </nav>");
    &body[nav_start..nav_end]
}

#[tokio::test]
async fn project_detail_breadcrumb_starts_with_today() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;

    let resp = app.server.get(&format!("/projects/{project_id}")).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let breadcrumb = breadcrumb_nav(&body);
    // 1. Leading entry: a link to /today labelled "Today".
    assert!(
        breadcrumb.contains(r#"href="/today""#),
        "project detail breadcrumb missing /today entry-point link: {breadcrumb}"
    );
    // 2. The Projects ancestor link.
    assert!(
        breadcrumb.contains(r#"href="/projects""#),
        "project detail breadcrumb missing Projects link: {breadcrumb}"
    );
    // 3. Terminal node carries aria-current="page".
    assert!(
        body.contains(r#"aria-current="page""#),
        "project detail breadcrumb missing aria-current=\"page\" on \
         terminal node"
    );
    // 4. Back link to projects list.
    assert!(
        body.contains("Back to projects"),
        "project detail page missing 'Back to projects' affordance"
    );
}

#[tokio::test]
async fn issue_detail_breadcrumb_full_chain() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("bob");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Login error").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let breadcrumb = breadcrumb_nav(&body);
    // Today entry point.
    assert!(
        breadcrumb.contains(r#"href="/today""#),
        "missing /today link: {breadcrumb}"
    );
    // Projects ancestor.
    assert!(
        breadcrumb.contains(r#"href="/projects""#),
        "missing /projects link: {breadcrumb}"
    );
    // Project ancestor (link to project detail).
    let project_link = format!(r#"href="/projects/{project_id}""#);
    assert!(
        breadcrumb.contains(&project_link),
        "missing parent-project link {project_link}: {breadcrumb}"
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
