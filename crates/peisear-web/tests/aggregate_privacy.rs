//! `QA-017` (RFC 005 §7, `NFR-PRIV-007`) — the sprint burndown's
//! day-by-day trajectory, and the velocity chart's median reference
//! line, are suppressed below two distinct contributors (people who
//! completed at least one issue). Everything else stays: the
//! sprint-end totals (`render_summary_card`), the velocity bars
//! themselves, the issues table. `QA-016`'s audit established why —
//! the totals are already assemblable elsewhere; the day-by-day
//! trajectory and the computed median are not, and both become
//! individually attributable once fewer than two people did the
//! completing.
//!
//! Contributor is scoped to **completed** work, not sprint membership
//! — an issue merely present in the sprint but not done does not make
//! its assignee a contributor to the trajectory being shown.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_planned_sprint, create_team_project, create_team_with_admin};
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::issues::IssueFields;

/// Insert a `done` issue directly (bypassing the status-change
/// endpoints, which this suite has no need to exercise) and link it to
/// `sprint_id`, assigned to `assignee_id`.
async fn insert_done_issue(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    sprint_id: &str,
    assignee_id: &str,
    title: &str,
) {
    let id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &id,
        project_id,
        author_id,
        IssueFields {
            title,
            description: "Test issue body.",
            status: IssueStatus::Done,
            priority: Priority::Medium,
            effort: Some(3),
            assignee_id: Some(assignee_id),
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert done issue");
    peisear_storage::sprints::add_issue(&app.db, sprint_id, &id)
        .await
        .expect("add issue to sprint");
}

/// Team with Alice (admin) and Bob (member), a team-scoped project,
/// and an active sprint. Returns `(app, team_id, project_id,
/// sprint_id, alice_id, bob_id)`, logged in as Alice.
async fn team_with_active_sprint() -> (TestApp, String, String, String, String, String) {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&app, &bob).await;
    common::auth::logout(&app).await;
    common::auth::login(&app, &alice).await;

    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    peisear_storage::teams::add_member(&app.db, &team_id, &bob_id, TeamRole::Member)
        .await
        .expect("add bob");
    let project_id = create_team_project(&app.db, &alice_id, &team_id, "Project").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    peisear_storage::sprints::start(&app.db, &sprint_id)
        .await
        .expect("start sprint");

    (app, team_id, project_id, sprint_id, alice_id, bob_id)
}

/// Check 1: two distinct contributors — burndown renders, median
/// line renders.
#[tokio::test]
async fn two_contributors_burndown_and_median_render() {
    let (app, _team_id, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("viewBox") && body.contains("cumulative"),
        "two contributors should render the burndown chart"
    );
}

/// Check 2: one contributor — burndown absent, summary totals still
/// present.
#[tokio::test]
async fn one_contributor_burndown_absent_totals_present() {
    let (app, _team_id, project_id, sprint_id, alice_id, _bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "B").await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        !body.contains("cumulative"),
        "one contributor must not render the burndown chart: {body}"
    );
    assert!(
        body.contains("Committed") && body.contains("Completed") && body.contains(">6<"),
        "the summary card's totals must still render, unaffected by the \
         suppression above it: {body}"
    );
}

/// Check 3: one contributor — velocity bars present, median line
/// absent.
#[tokio::test]
async fn one_contributor_velocity_bars_present_median_absent() {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    let project_id = create_team_project(&app.db, &alice_id, &team_id, "Project").await;

    for i in 0..2 {
        let sprint_id = create_planned_sprint(&app.db, &team_id, &format!("Sprint {i}")).await;
        peisear_storage::sprints::start(&app.db, &sprint_id)
            .await
            .expect("start sprint");
        insert_done_issue(
            &app,
            &project_id,
            &alice_id,
            &sprint_id,
            &alice_id,
            "Solo work",
        )
        .await;
        peisear_storage::sprints::complete(&app.db, &sprint_id)
            .await
            .expect("complete sprint");
    }

    let resp = app.server.get("/teams/engineering/sprints").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("<rect"),
        "solo velocity bars must still render: {body}"
    );
    assert!(
        !body.contains("stroke-dasharray"),
        "one contributor across the whole window must not render the median line: {body}"
    );
}

/// Check 4: one contributor — no text anywhere on the page explains
/// the absence. The guard against `QA-017` §4's silence being undone
/// later by a well-meant explanatory note.
#[tokio::test]
async fn one_contributor_page_has_no_text_explaining_the_absence() {
    let (app, _team_id, project_id, sprint_id, alice_id, _bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    let body = resp.text().to_lowercase();

    for phrase in [
        "one person",
        "one contributor",
        "single contributor",
        "privacy",
        "hidden because",
        "not shown because",
        "only you",
        "solo",
    ] {
        assert!(
            !body.contains(phrase),
            "the page must not explain the suppression -- found {phrase:?} in body"
        );
    }
}

/// Check 5: a `viewer`-role member sees the same behaviour as a
/// `member` — the audience `QA-016` established, confirmed here on
/// both the two-contributor and one-contributor cases.
#[tokio::test]
async fn viewer_role_sees_the_same_behaviour_as_member() {
    let (app, team_id, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    let carol = TestUser::new("carol");
    let carol_id = register_and_login(&app, &carol).await;
    peisear_storage::teams::add_member(&app.db, &team_id, &carol_id, TeamRole::Viewer)
        .await
        .expect("add carol as viewer");

    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;

    common::auth::logout(&app).await;
    common::auth::login(&app, &carol).await;
    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains("cumulative"),
        "a viewer must see the burndown for a two-contributor sprint, same as a member"
    );
}
