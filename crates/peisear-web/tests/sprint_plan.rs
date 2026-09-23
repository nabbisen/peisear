//! `PLAN-001` / RFC 001: the sprint planning page. A two-column
//! bulk-assign surface — team-wide backlog on the left, the
//! sprint's committed items on the right, button-driven moves
//! between them. RFC 001's own seven tests, with test 6 corrected
//! (404 not 403, handoff §2.1) and two added (§2.2's `viewer` case,
//! and the filter round-trip test 9 names as the surface's own
//! "works when written, silently breaks later" risk).
//!
//! Two more added per `PLAN-001-review.md` §3.1: the two
//! defense-in-depth guards reported in the original review request
//! (§5) had no test holding them in place — the same shape as
//! `TEAM-001`'s role-filter correction one round earlier. Both new
//! tests were demonstrated failing with their guard removed before
//! landing with it restored, same discipline.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, login, register_and_login};
use common::fixture::{create_planned_sprint, create_team_project, create_team_with_admin};
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::{issues, sprints};

fn plan_url(slug: &str, sprint_id: &str) -> String {
    format!("/teams/{slug}/sprints/{sprint_id}/plan")
}

async fn slug_for(app: &TestApp, team_id: &str) -> String {
    peisear_storage::teams::find_by_id(&app.db, team_id)
        .await
        .expect("find team")
        .expect("team exists")
        .slug
}

async fn insert_open_issue(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    title: &str,
    priority: Priority,
    effort: Option<i64>,
) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        &app.db,
        &id,
        project_id,
        author_id,
        issues::IssueFields {
            title,
            description: "Test issue body.",
            status: IssueStatus::Open,
            priority,
            effort,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert issue");
    id
}

/// Test 1 -- a planned sprint renders both columns (both headings
/// present) with a form per movable row, for a member who can write.
#[tokio::test]
async fn plan_page_renders_two_columns_for_planned_sprint() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let backlog_issue = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Backlog candidate",
        Priority::Medium,
        Some(3),
    )
    .await;
    let committed_issue = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Already committed",
        Priority::Medium,
        Some(5),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &committed_issue)
        .await
        .expect("add issue to sprint");

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("id=\"backlog-heading\""),
        "backlog heading missing: {body}"
    );
    assert!(
        body.contains("id=\"sprint-items-heading\""),
        "sprint items heading missing: {body}"
    );
    assert!(
        body.contains(&backlog_issue),
        "backlog row's move form should carry its issue id"
    );
    assert!(body.contains("Backlog candidate"));
    assert!(body.contains("Already committed"));
    // Scoped to each `<section aria-labelledby="...">` block, not two
    // independent whole-page checks -- the doc string's own claim
    // ("in the backlog column"/"in the sprint items column") is
    // stronger than what an unscoped `body.contains(...)` pair proves.
    // Confirmed by planting (`TT-003` §5): swapping which action each
    // column's form actually uses left the unscoped version passing.
    let backlog_start = body
        .find(r#"aria-labelledby="backlog-heading""#)
        .expect("backlog section present");
    let sprint_items_start = body
        .find(r#"aria-labelledby="sprint-items-heading""#)
        .expect("sprint items section present");
    assert!(
        backlog_start < sprint_items_start,
        "expected the backlog section to precede the sprint items section in \
         document order, so slicing between their start markers scopes each \
         correctly: {body}"
    );
    let backlog_section = &body[backlog_start..sprint_items_start];
    let sprint_items_section = &body[sprint_items_start..];
    assert!(
        backlog_section.contains("/plan/add"),
        "expected a move form (action targeting /plan/add) in the backlog \
         column specifically: {backlog_section}"
    );
    assert!(
        sprint_items_section.contains("/plan/remove"),
        "expected a move form (action targeting /plan/remove) in the sprint \
         items column specifically: {sprint_items_section}"
    );
}

/// Test 2 -- `POST /plan/add` moves a backlog issue into the sprint.
#[tokio::test]
async fn add_to_sprint_via_button_succeeds() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let issue_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Move me",
        Priority::Medium,
        Some(3),
    )
    .await;

    let resp = app
        .server
        .post(&format!("{}/add", plan_url(&slug, &sprint_id)))
        .form(&[
            ("issue_id", issue_id.as_str()),
            ("project_id", project_id.as_str()),
        ])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let sprint_for_issue = sprints::sprint_for_issue(&app.db, &issue_id)
        .await
        .expect("query sprint_for_issue");
    assert_eq!(sprint_for_issue.as_deref(), Some(sprint_id.as_str()));

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    let body = resp.text();
    assert!(
        body.contains("Move me"),
        "moved issue should render on the follow-up GET"
    );
}

/// Test 3 -- `POST /plan/remove` is the symmetric move back.
#[tokio::test]
async fn remove_from_sprint_via_button_succeeds() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let issue_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Move me back",
        Priority::Medium,
        Some(3),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &issue_id)
        .await
        .expect("add issue to sprint");

    let resp = app
        .server
        .post(&format!("{}/remove", plan_url(&slug, &sprint_id)))
        .form(&[("issue_id", issue_id.as_str())])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let sprint_for_issue = sprints::sprint_for_issue(&app.db, &issue_id)
        .await
        .expect("query sprint_for_issue");
    assert_eq!(
        sprint_for_issue, None,
        "issue should no longer be in any sprint"
    );

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    let body = resp.text();
    assert!(
        body.contains("Move me back"),
        "issue should reappear as a backlog row"
    );
}

/// Test 4 -- sub-issues never appear in either column.
#[tokio::test]
async fn sub_issues_do_not_appear_in_either_column() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let parent_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Parent issue",
        Priority::Medium,
        Some(5),
    )
    .await;
    let sub_id = uuid::Uuid::new_v4().to_string();
    issues::insert_sub_issue(
        &app.db,
        &sub_id,
        &project_id,
        &parent_id,
        &admin_id,
        "A sub-issue",
        "Test sub-issue body.",
        IssueStatus::Open,
        Priority::Medium,
        Some(2),
        None,
    )
    .await
    .expect("insert sub-issue");

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    let body = resp.text();
    assert!(body.contains("Parent issue"));
    assert!(
        !body.contains("A sub-issue"),
        "a sub-issue must not render as its own row: {body}"
    );
}

/// Test 5 -- a completed sprint's plan hides the backlog column
/// entirely (review correction PLAN-001-review.md §3.2 -- "re-opening
/// a completed sprint to add issues is not a flow we support") and
/// has no move `<form>`s anywhere on the page.
#[tokio::test]
async fn completed_sprint_plan_is_read_only() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let issue_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Historical item",
        Priority::Medium,
        Some(3),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &issue_id)
        .await
        .expect("add issue to sprint");
    sprints::start(&app.db, &sprint_id)
        .await
        .expect("start sprint");
    sprints::complete(&app.db, &sprint_id)
        .await
        .expect("complete sprint");

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        !body.contains("/plan/add") && !body.contains("/plan/remove"),
        "a completed sprint's plan must have no move forms: {body}"
    );
    assert!(
        !body.contains("id=\"backlog-heading\""),
        "a completed sprint's plan must hide the backlog column entirely: {body}"
    );
    assert!(
        body.contains("id=\"sprint-items-heading\""),
        "the sprint items column must still render: {body}"
    );
    assert!(
        body.contains("Historical item"),
        "the sprint's own items must still render, just without a move form"
    );
}

/// Test 6 -- corrected per handoff §2.1: a non-member gets 404, not
/// 403, matching `resolve_team_membership`'s existing posture.
#[tokio::test]
async fn non_team_member_gets_404() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let stranger = TestUser::new("stranger");
    register_and_login(&app, &stranger).await;
    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::NOT_FOUND);
}

/// Test 7 -- the committed total sums effort across the sprint's
/// items: two issues at 5 and 8 points render "13 pt" (invariant
/// singular unit, matching `PointsUnitSuffix`/`PointsValue`
/// elsewhere -- review correction PLAN-001-review.md §3.3).
#[tokio::test]
async fn committed_total_matches_sum_of_effort() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let a = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Five",
        Priority::Medium,
        Some(5),
    )
    .await;
    let b = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Eight",
        Priority::Medium,
        Some(8),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &a)
        .await
        .expect("add a");
    sprints::add_issue(&app.db, &sprint_id, &b)
        .await
        .expect("add b");

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    let body = resp.text();
    assert!(
        body.contains("13 pt"),
        "expected the committed total to read 13 pt: {body}"
    );
}

/// Test 8 -- handoff §2.2: `viewer` may read the plan (200, backlog
/// still shown per PLAN-001-review.md §3.2's corrected table -- a
/// viewer is reading a live plan and needs to see what isn't
/// committed yet -- but no move forms) and gets 403 attempting to
/// POST `/plan/add`.
#[tokio::test]
async fn viewer_gets_read_only_plan_and_403_on_post() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let issue_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Backlog item",
        Priority::Medium,
        Some(3),
    )
    .await;

    let viewer = TestUser::new("vic");
    let viewer_id = register_and_login(&app, &viewer).await;
    peisear_storage::teams::add_member(&app.db, &team_id, &viewer_id, TeamRole::Viewer)
        .await
        .expect("add viewer");

    // `register_and_login` above switched app's cookie jar to the
    // viewer's session.
    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        !body.contains("/plan/add") && !body.contains("/plan/remove"),
        "a viewer must see no move forms: {body}"
    );
    assert!(
        body.contains("id=\"backlog-heading\"") && body.contains("Backlog item"),
        "a viewer on a planned sprint must still see the backlog: {body}"
    );

    let resp = app
        .server
        .post(&format!("{}/add", plan_url(&slug, &sprint_id)))
        .form(&[
            ("issue_id", issue_id.as_str()),
            ("project_id", project_id.as_str()),
        ])
        .await;
    resp.assert_status(StatusCode::FORBIDDEN);
}

/// Test 9 -- the backlog filter narrows by priority and the 303
/// after a move preserves the same filter query.
#[tokio::test]
async fn filter_round_trip_narrows_backlog_and_survives_move() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let high_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Urgent fix",
        Priority::High,
        Some(2),
    )
    .await;
    let _low_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Someday maybe",
        Priority::Low,
        Some(1),
    )
    .await;

    let resp = app
        .server
        .get(&format!("{}?priority=high", plan_url(&slug, &sprint_id)))
        .await;
    let body = resp.text();
    assert!(
        body.contains("Urgent fix"),
        "the matching priority should still show"
    );
    assert!(
        !body.contains("Someday maybe"),
        "a non-matching priority must be filtered out: {body}"
    );

    let resp = app
        .server
        .post(&format!("{}/add", plan_url(&slug, &sprint_id)))
        .form(&[
            ("issue_id", high_id.as_str()),
            ("project_id", project_id.as_str()),
            ("priority", "high"),
        ])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get("location")
        .expect("redirect must carry a Location header")
        .to_str()
        .unwrap();
    assert!(
        location.contains("priority=high"),
        "the redirect must preserve the active filter, got {location}"
    );
}

/// Correction (`PLAN-001-review.md` §3.1 / §5.2): `plan_remove`
/// deletes by `issue_id` alone with no sprint scoping at the storage
/// layer, so the handler's own check -- the issue must currently be
/// in *this* sprint -- is the only thing standing between a member
/// of team A and removing an issue from a completely different
/// team's sprint. A member of team A, with no relationship to team B
/// at all, forges a POST naming an issue that actually lives in team
/// B's active sprint.
#[tokio::test]
async fn plan_remove_rejects_an_issue_belonging_to_another_teams_sprint() {
    let app = TestApp::spawn().await;

    let admin_a = TestUser::new("alice");
    let admin_a_id = register_and_login(&app, &admin_a).await;
    let team_a_id = create_team_with_admin(&app.db, &admin_a_id, "Team A").await;
    let slug_a = slug_for(&app, &team_a_id).await;
    let sprint_a_id = create_planned_sprint(&app.db, &team_a_id, "Sprint A").await;

    // A wholly unrelated team, with its own active sprint holding an
    // issue -- the forged removal's target.
    let admin_b = TestUser::new("bob");
    let admin_b_id = register_and_login(&app, &admin_b).await;
    let team_b_id = create_team_with_admin(&app.db, &admin_b_id, "Team B").await;
    let project_b_id = create_team_project(&app.db, &admin_b_id, &team_b_id, "Proj B").await;
    let sprint_b_id = create_planned_sprint(&app.db, &team_b_id, "Sprint B").await;
    let issue_b_id = insert_open_issue(
        &app,
        &project_b_id,
        &admin_b_id,
        "Team B's committed item",
        Priority::Medium,
        Some(5),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_b_id, &issue_b_id)
        .await
        .expect("add to sprint B");
    sprints::start(&app.db, &sprint_b_id)
        .await
        .expect("start sprint B");

    // `register_and_login` for bob switched the shared cookie jar;
    // switch back to alice's session before forging the request.
    login(&app, &admin_a).await;

    let resp = app
        .server
        .post(&format!("{}/remove", plan_url(&slug_a, &sprint_a_id)))
        .form(&[("issue_id", issue_b_id.as_str())])
        .await;
    resp.assert_status(StatusCode::NOT_FOUND);

    let still_in_b = sprints::sprint_for_issue(&app.db, &issue_b_id)
        .await
        .expect("query sprint_for_issue");
    assert_eq!(
        still_in_b.as_deref(),
        Some(sprint_b_id.as_str()),
        "the issue must still be in team B's sprint after the rejected removal"
    );
}

/// Correction (`PLAN-001-review.md` §3.1 / §5.1): `plan_add` rejects
/// a non-planned sprint at the handler, not just by omitting the
/// button. A completed sprint is targeted directly with an issue
/// that was never in it.
#[tokio::test]
async fn plan_add_rejects_a_completed_sprint() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let seed_issue = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Seed",
        Priority::Medium,
        Some(1),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &seed_issue)
        .await
        .expect("seed the sprint so it has something to complete with");
    sprints::start(&app.db, &sprint_id).await.expect("start");
    sprints::complete(&app.db, &sprint_id)
        .await
        .expect("complete");

    let target_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Late arrival",
        Priority::Medium,
        Some(2),
    )
    .await;

    let resp = app
        .server
        .post(&format!("{}/add", plan_url(&slug, &sprint_id)))
        .form(&[
            ("issue_id", target_id.as_str()),
            ("project_id", project_id.as_str()),
        ])
        .await;
    resp.assert_status(StatusCode::BAD_REQUEST);

    let sprint_for_target = sprints::sprint_for_issue(&app.db, &target_id)
        .await
        .expect("query sprint_for_issue");
    assert_eq!(
        sprint_for_target, None,
        "the issue must not have been added to the completed sprint"
    );
}

/// `QA-002` item 1, test 1: `delete_sprint`'s `POST` had no status
/// check and would delete a team's running sprint. 400, not the
/// generic 403/404 shape, since this is a state constraint on an
/// otherwise-authorised admin, not an authorisation failure.
#[tokio::test]
async fn delete_refuses_an_active_sprint() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Engineering").await;
    let slug = slug_for(&app, &team_id).await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;
    sprints::start(&app.db, &sprint_id).await.expect("start");

    let sprint = sprints::find_by_id(&app.db, &sprint_id)
        .await
        .expect("query sprint")
        .expect("sprint exists");
    let resp = app
        .server
        .post(&format!("/teams/{slug}/sprints/{sprint_id}/delete"))
        .form(&[("client_updated_at", sprint.updated_at.to_rfc3339().as_str())])
        .await;
    resp.assert_status(StatusCode::BAD_REQUEST);

    let still_there = sprints::find_by_id(&app.db, &sprint_id)
        .await
        .expect("query sprint");
    assert!(
        still_there.is_some(),
        "an active sprint must survive a delete attempt"
    );
}

// ──────────────────────────────────────────────────────────────
// `PLAN-002` (RFC 004c D-4) — drag between the backlog and the
// sprint
// ──────────────────────────────────────────────────────────────

/// Extract a `<script type="application/json" id="{id}">…</script>`
/// island's JSON content from a rendered page body. Same shape as
/// `response_outcomes.rs`'s own `extract_island` — duplicated rather
/// than shared, same reasoning `static_js_scan.rs`'s module doc gives
/// for its own small duplicated helpers.
fn extract_island(body: &str, id: &str) -> serde_json::Value {
    let marker = format!(r#"id="{id}""#);
    let start = body
        .find(&marker)
        .unwrap_or_else(|| panic!("island id={id:?} not found in body: {body}"));
    let after_marker = &body[start..];
    let content_start = after_marker
        .find('>')
        .expect("island opening tag closes with `>`")
        + 1;
    let content = &after_marker[content_start..];
    let end = content
        .find("</script>")
        .expect("island script tag has a closing </script>");
    serde_json::from_str(&content[..end]).unwrap_or_else(|e| {
        panic!(
            "island id={id:?} is not valid JSON: {e}\ncontent: {}",
            &content[..end]
        )
    })
}

/// §6's most important new test: the drag markers, `plan.js`'s tag
/// and its copy island are all attached for an admin on a `Planned`
/// sprint — the one cell in the module doc's four-shape table where
/// `can_move` is true.
#[tokio::test]
async fn drag_markers_attached_for_admin_on_planned_sprint() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let backlog_issue = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Backlog candidate",
        Priority::Medium,
        Some(3),
    )
    .await;
    let committed_issue = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Committed item",
        Priority::Medium,
        Some(5),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &committed_issue)
        .await
        .expect("add committed issue to sprint");

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains(r#"data-plan-move="add""#),
        "a backlog row must carry data-plan-move=\"add\": {body}"
    );
    assert!(
        body.contains(r#"data-plan-move="remove""#),
        "a sprint item row must carry data-plan-move=\"remove\": {body}"
    );
    assert!(
        body.contains(r#"data-plan-drop="add""#),
        "the sprint column must carry data-plan-drop=\"add\": {body}"
    );
    assert!(
        body.contains(r#"data-plan-drop="remove""#),
        "the backlog column must carry data-plan-drop=\"remove\": {body}"
    );
    // Round 2 (`PLAN-002-review.md` §6a/§6e): the value a row *sitting
    // in* each column carries -- the opposite of that column's own
    // `data-plan-drop` -- rendered server-side so the script never
    // inverts one to get the other.
    //
    // `PLAN-002-round2-review.md` §4: checked as one substring per
    // column, pinning `data-plan-drop` to its own `data-plan-row-move`
    // rather than two independent unscoped checks -- both strings
    // appear on the page regardless of which column carries which, so
    // the unscoped pair would still pass with the two columns'
    // attributes swapped, which is exactly the association this round
    // exists to get right.
    assert!(
        body.contains(r#"data-plan-drop="add" data-plan-row-move="remove""#),
        "the sprint column must pair data-plan-drop=\"add\" with its own \
         data-plan-row-move=\"remove\" (a row sitting there moves away by \
         a remove): {body}"
    );
    assert!(
        body.contains(r#"data-plan-drop="remove" data-plan-row-move="add""#),
        "the backlog column must pair data-plan-drop=\"remove\" with its own \
         data-plan-row-move=\"add\" (a row sitting there moves away by an \
         add): {body}"
    );
    assert!(
        body.contains(r#"draggable="true""#) && body.contains(r#"draggable="false""#),
        "a draggable row and its inner link's draggable=\"false\" must both be present \
         (§3.2 — the nested-drag-source trap board.js already paid for): {body}"
    );
    assert!(
        body.contains(&format!(r#"data-plan-issue-id="{backlog_issue}""#)),
        "the backlog row must carry its own issue id: {body}"
    );
    assert!(
        body.contains(r#"<script src="/static/plan.js" defer"#),
        "plan.js must be referenced with defer when drag is attached: {body}"
    );
    assert!(
        body.contains(r#"id="plan-copy""#),
        "the copy island must be present when drag is attached: {body}"
    );

    let island = extract_island(&body, "plan-copy");
    for key in [
        "movedToSprint",
        "movedToBacklog",
        "undoLabel",
        "noBacklogIssuesMessage",
        "noSprintItemsMessage",
        "undoUnavailableMessage",
        // Round 2 (`PLAN-002-review.md` §6c): the two move-button
        // labels, so `syncRowForm` can relabel a moved row's own
        // button without authoring a string.
        "sprintButtonLabel",
        "backlogButtonLabel",
    ] {
        let value = island[key].as_str();
        assert!(
            value.is_some_and(|v| !v.is_empty()),
            "plan-copy must carry a non-empty {key}: {island}"
        );
    }
    assert!(
        island.get("outcomes").is_none(),
        "plan-copy must carry no outcomes block (§3.5 — there is no lock to \
         conflict over): {island}"
    );
}

/// The three shapes where drag is *not* attached (module doc's
/// table): a viewer on a planned sprint, any role on an active
/// sprint, any role on a completed sprint. One test per shape, same
/// granularity the rest of this file already uses for role/status
/// combinations.
#[tokio::test]
async fn drag_markers_absent_for_viewer_on_planned_sprint() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Backlog item",
        Priority::Medium,
        Some(3),
    )
    .await;

    let viewer = TestUser::new("vic");
    let viewer_id = register_and_login(&app, &viewer).await;
    peisear_storage::teams::add_member(&app.db, &team_id, &viewer_id, TeamRole::Viewer)
        .await
        .expect("add viewer");

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert_no_drag_markers(&body);
}

#[tokio::test]
async fn drag_markers_absent_on_active_sprint() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let issue_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Committed item",
        Priority::Medium,
        Some(3),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &issue_id)
        .await
        .expect("add issue to sprint");
    sprints::start(&app.db, &sprint_id).await.expect("start");

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert_no_drag_markers(&body);
}

#[tokio::test]
async fn drag_markers_absent_on_completed_sprint() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let issue_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Historical item",
        Priority::Medium,
        Some(3),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &issue_id)
        .await
        .expect("add issue to sprint");
    sprints::start(&app.db, &sprint_id).await.expect("start");
    sprints::complete(&app.db, &sprint_id)
        .await
        .expect("complete");

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert_no_drag_markers(&body);
}

fn assert_no_drag_markers(body: &str) {
    for needle in [
        "data-plan-move",
        "data-plan-drop",
        "data-plan-row-move",
        "plan-copy",
        "/static/plan.js",
    ] {
        assert!(
            !body.contains(needle),
            "drag must not be attached here -- found {needle:?}: {body}"
        );
    }
}

/// §4.6: the remove form omitted the three filter fields the add
/// form already carries (`§2c`), so a button-driven remove silently
/// dropped the active backlog filter from the redirect. Same shape
/// as `filter_round_trip_narrows_backlog_and_survives_move` (test 9)
/// above, but for `/plan/remove`.
#[tokio::test]
async fn plan_remove_preserves_the_active_filter_like_add_does() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let committed_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Committed item",
        Priority::High,
        Some(2),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &committed_id)
        .await
        .expect("add committed issue");

    let resp = app
        .server
        .post(&format!("{}/remove", plan_url(&slug, &sprint_id)))
        .form(&[("issue_id", committed_id.as_str()), ("priority", "high")])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get("location")
        .expect("redirect must carry a Location header")
        .to_str()
        .unwrap();
    assert!(
        location.contains("priority=high"),
        "a button-driven remove must preserve the active filter, got {location}"
    );

    let now_in_backlog = sprints::sprint_for_issue(&app.db, &committed_id)
        .await
        .expect("query sprint_for_issue");
    assert_eq!(
        now_in_backlog, None,
        "the issue must have actually moved to the backlog"
    );
}

/// The rendered remove form itself carries the three hidden filter
/// inputs, mirroring the add form's own markup (§2c/§4.6) — the
/// no-JavaScript, button-driven path this proves works even without
/// a POST, by inspecting what a click would submit.
#[tokio::test]
async fn remove_form_markup_carries_the_same_filter_fields_as_add() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let committed_id = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "Committed item",
        Priority::Medium,
        Some(3),
    )
    .await;
    sprints::add_issue(&app.db, &sprint_id, &committed_id)
        .await
        .expect("add committed issue");

    let resp = app
        .server
        .get(&format!("{}?priority=medium", plan_url(&slug, &sprint_id)))
        .await;
    let body = resp.text();
    let remove_form_start = body
        .find("/plan/remove")
        .expect("a remove form must be present");
    let form_slice = &body[remove_form_start..];
    let form_end = form_slice.find("</form>").expect("form has a close tag");
    let form_slice = &form_slice[..form_end];
    for name in ["project", "priority", "assignee"] {
        assert!(
            form_slice.contains(&format!(r#"name="{name}""#)),
            "the remove form must carry a hidden {name} input, matching the add form: {form_slice}"
        );
    }
}

// ──────────────────────────────────────────────────────────────
// `PLAN-003` — the backlog column's order.
//
// `backlog_for_team` used to `ORDER BY p.name ASC, i.priority DESC,
// i.created_at DESC`. `priority` is TEXT, so `DESC` is alphabetical:
// `urgent medium low high` -- `high` last, below `low`. Everything
// above asserts membership, never sequence, which is why this
// survived from `PLAN-001`. Every assertion below is by **relative
// byte offset in the rendered body**, not `body.contains`, which
// passes on any order (`PLAN-002` round 2's first attempt failed
// review for exactly that).
// ──────────────────────────────────────────────────────────────

/// Pin `created_at` explicitly: it is a one-second `CURRENT_TIMESTAMP`
/// and these tests insert rows in microseconds, so "which is newer"
/// would otherwise be a coin flip.
async fn pin_created_at(app: &TestApp, issue_id: &str, created_at: &str) {
    sqlx::query("UPDATE issues SET created_at = ?1 WHERE id = ?2")
        .bind(created_at)
        .bind(issue_id)
        .execute(&app.db)
        .await
        .expect("pin created_at");
}

fn offset_of(body: &str, needle: &str) -> usize {
    body.find(needle)
        .unwrap_or_else(|| panic!("{needle:?} not found in body: {body}"))
}

/// Byte offsets of `titles` in `body`, in the order given -- so a
/// caller asserts `offsets.windows(2).all(|w| w[0] < w[1])` to say
/// "these appear in this sequence".
fn offsets_of(body: &str, titles: &[&str]) -> Vec<usize> {
    titles.iter().map(|t| offset_of(body, t)).collect()
}

/// The backlog reads urgent, high, medium, low. Created in the
/// *reverse* of severity, so neither insertion order nor a bare
/// `created_at` sort can produce the expected sequence by accident.
#[tokio::test]
async fn backlog_reads_urgent_high_medium_low() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    for (title, priority) in [
        ("PLAN3-low", Priority::Low),
        ("PLAN3-medium", Priority::Medium),
        ("PLAN3-high", Priority::High),
        ("PLAN3-urgent", Priority::Urgent),
    ] {
        insert_open_issue(&app, &project_id, &admin_id, title, priority, Some(1)).await;
    }

    let resp = app.server.get(&plan_url(&slug, &sprint_id)).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    let offsets = offsets_of(
        &body,
        &["PLAN3-urgent", "PLAN3-high", "PLAN3-medium", "PLAN3-low"],
    );
    assert!(
        offsets.windows(2).all(|w| w[0] < w[1]),
        "the backlog must read urgent, high, medium, low -- the old TEXT `DESC` \
         gave urgent, medium, low, high: {offsets:?}"
    );
    assert!(
        offset_of(&body, "PLAN3-high") < offset_of(&body, "PLAN3-low"),
        "`high` must no longer sort below `low`"
    );
}

/// `p.name ASC` still groups first; severity orders *within* a
/// project. Beta's urgent issue is more severe than everything in
/// Alpha and must still come after all of it, and Alpha's `high` must
/// come before Alpha's `low` (alphabetically the old order had them the
/// other way round).
#[tokio::test]
async fn backlog_groups_by_project_name_then_severity() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    // Created Beta first, so neither project creation order nor id order
    // is what puts Alpha first.
    let beta = create_team_project(&app.db, &admin_id, &team_id, "Beta").await;
    let alpha = create_team_project(&app.db, &admin_id, &team_id, "Alpha").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    insert_open_issue(
        &app,
        &beta,
        &admin_id,
        "GRP-beta-urgent",
        Priority::Urgent,
        Some(1),
    )
    .await;
    insert_open_issue(
        &app,
        &beta,
        &admin_id,
        "GRP-beta-medium",
        Priority::Medium,
        Some(1),
    )
    .await;
    insert_open_issue(
        &app,
        &alpha,
        &admin_id,
        "GRP-alpha-low",
        Priority::Low,
        Some(1),
    )
    .await;
    insert_open_issue(
        &app,
        &alpha,
        &admin_id,
        "GRP-alpha-high",
        Priority::High,
        Some(1),
    )
    .await;

    let body = app.server.get(&plan_url(&slug, &sprint_id)).await.text();
    let offsets = offsets_of(
        &body,
        &[
            "GRP-alpha-high",
            "GRP-alpha-low",
            "GRP-beta-urgent",
            "GRP-beta-medium",
        ],
    );
    assert!(
        offsets.windows(2).all(|w| w[0] < w[1]),
        "expected Alpha (high, low) then Beta (urgent, medium): {offsets:?}"
    );
}

/// Within one (project, priority) group the order is still
/// `created_at DESC` -- newest first -- because the Rust re-sort is
/// *stable*; only the priority term's behaviour changed. The two
/// `medium` issues are separated in time by a `high` one, so the
/// group is not simply adjacent in creation order.
#[tokio::test]
async fn backlog_keeps_newest_first_within_a_priority() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let older = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "REC-medium-older",
        Priority::Medium,
        Some(1),
    )
    .await;
    let mid = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "REC-high",
        Priority::High,
        Some(1),
    )
    .await;
    let newer = insert_open_issue(
        &app,
        &project_id,
        &admin_id,
        "REC-medium-newer",
        Priority::Medium,
        Some(1),
    )
    .await;
    pin_created_at(&app, &older, "2026-01-01 09:00:00").await;
    pin_created_at(&app, &mid, "2026-01-02 09:00:00").await;
    pin_created_at(&app, &newer, "2026-01-03 09:00:00").await;

    let body = app.server.get(&plan_url(&slug, &sprint_id)).await.text();
    let offsets = offsets_of(&body, &["REC-high", "REC-medium-newer", "REC-medium-older"]);
    assert!(
        offsets.windows(2).all(|w| w[0] < w[1]),
        "expected high, then the two mediums newest first: {offsets:?}"
    );
}

/// The priority *filter* is an equality test on the same query and
/// is unaffected: each band returns only itself.
#[tokio::test]
async fn backlog_priority_filter_returns_only_the_chosen_band() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let bands = [
        ("low", "FLT-low", Priority::Low),
        ("medium", "FLT-medium", Priority::Medium),
        ("high", "FLT-high", Priority::High),
        ("urgent", "FLT-urgent", Priority::Urgent),
    ];
    for (_, title, priority) in bands {
        insert_open_issue(&app, &project_id, &admin_id, title, priority, Some(1)).await;
    }

    for (param, wanted, _) in bands {
        let body = app
            .server
            .get(&format!("{}?priority={param}", plan_url(&slug, &sprint_id)))
            .await
            .text();
        for (_, title, _) in bands {
            assert_eq!(
                body.contains(title),
                title == wanted,
                "`?priority={param}` must show {wanted} and nothing else; {title} presence wrong"
            );
        }
    }
}

/// `ORD-002`: the backlog's recency term ties on a one-second
/// timestamp and SQLite returns a tie oldest-first, so four issues
/// written in one burst used to read first, second, third, fourth --
/// the reverse of "newest first". Every issue shares one `created_at`
/// (pinned, so a burst straddling a second boundary cannot hide it).
#[tokio::test]
async fn backlog_reads_a_same_second_burst_newest_first() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = slug_for(&app, &team_id).await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let titles = ["BURST-first", "BURST-second", "BURST-third", "BURST-fourth"];
    for title in titles {
        let id = insert_open_issue(
            &app,
            &project_id,
            &admin_id,
            title,
            Priority::Medium,
            Some(1),
        )
        .await;
        pin_created_at(&app, &id, "2026-03-01 09:00:00").await;
    }

    let body = app.server.get(&plan_url(&slug, &sprint_id)).await.text();
    let offsets = offsets_of(
        &body,
        &["BURST-fourth", "BURST-third", "BURST-second", "BURST-first"],
    );
    assert!(
        offsets.windows(2).all(|w| w[0] < w[1]),
        "a same-second burst must read newest first: {offsets:?}"
    );
}
