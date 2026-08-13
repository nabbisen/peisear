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
    assert!(
        body.contains("/plan/add") && body.contains("/plan/remove"),
        "expected a move form (action targeting /plan/add) in the backlog column and one \
         (targeting /plan/remove) in the sprint items column: {body}"
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
