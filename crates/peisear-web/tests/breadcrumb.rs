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

/// `COV-001` §4 (`FR-NAV-003`, `§10.35`). Sprint detail renders its
/// trail through the same shared `render_breadcrumb`/`render_back_link`
/// project/issue detail already use (`components/sprints.rs:684`,
/// `:693`), so this is the structural case: covered by inspection,
/// untested by assertion until now.
#[tokio::test]
async fn sprint_detail_breadcrumb_full_chain() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("carol");
    let user_id = register_and_login(&app, &user).await;
    let team_id = common::fixture::create_team_with_admin(&app.db, &user_id, "Eng").await;
    let team = peisear_storage::teams::find_by_id(&app.db, &team_id)
        .await
        .expect("find team")
        .expect("team exists");
    let sprint_id = common::fixture::create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;

    let url = format!("/teams/{}/sprints/{sprint_id}", team.slug);
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let breadcrumb = breadcrumb_nav(&body);
    assert!(
        breadcrumb.contains(r#"href="/today""#),
        "sprint detail breadcrumb missing /today entry-point link: {breadcrumb}"
    );
    assert!(
        breadcrumb.contains(r#"href="/teams""#),
        "sprint detail breadcrumb missing Teams link: {breadcrumb}"
    );
    let team_link = format!(r#"href="/teams/{}""#, team.slug);
    assert!(
        breadcrumb.contains(&team_link),
        "sprint detail breadcrumb missing parent-team link {team_link}: {breadcrumb}"
    );
    let sprints_link = format!(r#"href="/teams/{}/sprints""#, team.slug);
    assert!(
        breadcrumb.contains(&sprints_link),
        "sprint detail breadcrumb missing Sprints ancestor link {sprints_link}: {breadcrumb}"
    );
    assert!(
        body.contains(r#"aria-current="page""#),
        "sprint detail breadcrumb missing aria-current=\"page\" on terminal node"
    );
    assert!(
        body.contains("Back to sprints"),
        "sprint detail page missing 'Back to sprints' affordance"
    );
}

/// `NAV-001` (`FR-NAV-003`, `§10.35`). `COV-001` found team detail's
/// trail hand-rolled (`components/teams.rs:306`, before this handoff's
/// fix): no `/today` root, no `aria-current`, and -- the actual
/// defect, not just an uncovered one -- no back link anywhere on the
/// page. Written against what the requirement needs, the same way the
/// sprint detail test above is, not against what the page emitted
/// before the fix.
#[tokio::test]
async fn team_detail_breadcrumb_full_chain() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("dana");
    let user_id = register_and_login(&app, &user).await;
    let team_id = common::fixture::create_team_with_admin(&app.db, &user_id, "Design").await;
    let team = peisear_storage::teams::find_by_id(&app.db, &team_id)
        .await
        .expect("find team")
        .expect("team exists");

    let url = format!("/teams/{}", team.slug);
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let breadcrumb = breadcrumb_nav(&body);
    assert!(
        breadcrumb.contains(r#"href="/today""#),
        "team detail breadcrumb missing /today entry-point link: {breadcrumb}"
    );
    assert!(
        breadcrumb.contains(r#"href="/teams""#),
        "team detail breadcrumb missing Teams ancestor link: {breadcrumb}"
    );
    assert!(
        body.contains(r#"aria-current="page""#),
        "team detail breadcrumb missing aria-current=\"page\" on terminal node"
    );
    // The conjunct that was actually missing: a back link, present
    // and targeting the parent context (the teams list).
    assert!(
        body.contains(r#"href="/teams""#) && body.contains("Back to teams"),
        "team detail page missing a 'Back to teams' affordance"
    );
}
