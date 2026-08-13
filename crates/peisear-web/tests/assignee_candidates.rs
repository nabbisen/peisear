//! `TEAM-001` / RFC 009 requirements 1-4: the assignee-candidate set
//! for a team-owned project is its team's membership (`admin`/
//! `member`, not `viewer`) plus the owner, and `project_workload`
//! cannot disagree with it. Regression guard first, per this
//! project's established discipline (`I18N-007`, `QA-001`): test 1
//! is written to fail on unmodified code, demonstrated failing
//! before the fix, then landed alongside it.

mod common;

use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project, create_team_project};
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_storage::{issues, teams};

/// Test 1 -- the regression guard. A team member must be a valid
/// assignee for an issue in their team's project. Fails on
/// unmodified code: `list_assignee_candidates` joins on
/// `p.owner_id = u.id` alone, so a non-owner member is rejected
/// with 400 regardless of team membership.
#[tokio::test]
async fn team_member_is_a_valid_assignee_in_a_team_project() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;

    let member = TestUser::new("bob");
    let member_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(
        &app.db,
        &member_id,
        &member.email,
        "x",
        &member.display_name,
    )
    .await
    .expect("insert member user");

    let team_id = common::fixture::create_team_with_admin(&app.db, &admin_id, "Team").await;
    teams::add_member(&app.db, &team_id, &member_id, TeamRole::Member)
        .await
        .expect("add member");

    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;

    let resp = app
        .server
        .post(&format!("/projects/{project_id}/issues/new"))
        .form(&[
            ("title", "Assigned to a team member"),
            ("description", ""),
            ("status", "open"),
            ("priority", "medium"),
            ("effort", ""),
            ("assignee_id", member_id.as_str()),
        ])
        .await;

    assert_eq!(
        resp.status_code(),
        axum::http::StatusCode::SEE_OTHER,
        "expected the issue to be created and the request redirected; got {} with body {}",
        resp.status_code(),
        resp.text()
    );

    let issues_in_project = issues::list_in_project(&app.db, &project_id)
        .await
        .expect("list issues");
    let created = issues_in_project
        .iter()
        .find(|i| i.title == "Assigned to a team member")
        .expect("created issue present");
    assert_eq!(created.assignee_id.as_deref(), Some(member_id.as_str()));
}

/// Test 2 -- a personal project (`team_id IS NULL`) yields exactly
/// the owner, unchanged.
#[tokio::test]
async fn personal_project_candidates_are_exactly_the_owner() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Proj").await;

    let candidates = issues::list_assignee_candidates(&app.db, &project_id)
        .await
        .expect("list candidates");

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].id, user_id);
}

/// Test 3 -- the owner is a candidate even when the owner holds no
/// `team_memberships` row for the project's team (RFC 009
/// requirement 3: "an owner who cannot be assigned their own issue
/// is a worse defect than the one being fixed").
#[tokio::test]
async fn owner_is_a_candidate_even_when_not_a_team_member() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("teamlead");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = common::fixture::create_team_with_admin(&app.db, &admin_id, "Team").await;

    // A second user owns a project under this team without ever
    // being added to team_memberships for it.
    let owner = TestUser::new("owner");
    let owner_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(&app.db, &owner_id, &owner.email, "x", &owner.display_name)
        .await
        .expect("insert owner user");
    let project_id = create_team_project(&app.db, &owner_id, &team_id, "Proj").await;

    let candidates = issues::list_assignee_candidates(&app.db, &project_id)
        .await
        .expect("list candidates");

    assert!(
        candidates.iter().any(|c| c.id == owner_id),
        "owner missing from candidates: {candidates:?}"
    );
}

/// Test 4 -- a user with no relationship to the project (not the
/// owner, not a team member) is rejected with 400, not silently
/// unassigned.
#[tokio::test]
async fn unrelated_user_is_rejected_as_assignee() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = common::fixture::create_team_with_admin(&app.db, &admin_id, "Team").await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;

    let stranger_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(
        &app.db,
        &stranger_id,
        "stranger@example.com",
        "x",
        "Stranger",
    )
    .await
    .expect("insert stranger user");

    let resp = app
        .server
        .post(&format!("/projects/{project_id}/issues/new"))
        .form(&[
            ("title", "Should be rejected"),
            ("description", ""),
            ("status", "open"),
            ("priority", "medium"),
            ("effort", ""),
            ("assignee_id", stranger_id.as_str()),
        ])
        .await;

    assert_eq!(resp.status_code(), axum::http::StatusCode::BAD_REQUEST);
}

/// Test 5 -- requirement 2: the candidate set is a subset of the
/// workload set, over a fixture covering personal, team-owned, and
/// removed-member cases. Written so it fails if the two queries
/// ever diverge again.
#[tokio::test]
async fn candidate_set_is_a_subset_of_the_workload_set() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;

    // Personal project.
    let personal_project = create_personal_project(&app.db, &admin_id, "Personal").await;

    // Team-owned project with a member who is later removed but
    // still holds an in-flight issue.
    let team_id = common::fixture::create_team_with_admin(&app.db, &admin_id, "Team").await;
    let member = TestUser::new("bob");
    let member_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(
        &app.db,
        &member_id,
        &member.email,
        "x",
        &member.display_name,
    )
    .await
    .expect("insert member");
    teams::add_member(&app.db, &team_id, &member_id, TeamRole::Member)
        .await
        .expect("add member");
    let team_project = create_team_project(&app.db, &admin_id, &team_id, "TeamProj").await;
    let issue_id = create_issue(&app.db, &team_project, &admin_id, "Held by bob").await;
    issues::update(
        &app.db,
        &issue_id,
        &team_project,
        &admin_id,
        issues::IssueFields {
            title: "Held by bob",
            description: "",
            status: peisear_core::IssueStatus::Open,
            priority: peisear_core::Priority::Medium,
            effort: None,
            assignee_id: Some(&member_id),
        },
    )
    .await
    .expect("assign issue to bob");
    teams::remove_member(&app.db, &team_id, &member_id)
        .await
        .expect("remove bob from team");

    for project_id in [&personal_project, &team_project] {
        let candidates = issues::list_assignee_candidates(&app.db, project_id)
            .await
            .expect("list candidates");
        let workload = issues::project_workload(&app.db, project_id)
            .await
            .expect("list workload");
        let workload_ids: std::collections::HashSet<_> =
            workload.iter().map(|w| w.user_id.clone()).collect();

        for candidate in &candidates {
            assert!(
                workload_ids.contains(&candidate.id),
                "candidate {} (project {project_id}) missing from workload set {workload_ids:?}",
                candidate.id
            );
        }
    }
}

/// Test 6 -- RFC 009 §D3: a removed member holding an in-flight
/// issue appears in the workload set and not in the candidate set.
/// The report describes reality; the form describes policy.
#[tokio::test]
async fn removed_member_with_in_flight_issue_is_in_workload_not_candidates() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = common::fixture::create_team_with_admin(&app.db, &admin_id, "Team").await;

    let member = TestUser::new("bob");
    let member_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(
        &app.db,
        &member_id,
        &member.email,
        "x",
        &member.display_name,
    )
    .await
    .expect("insert member");
    teams::add_member(&app.db, &team_id, &member_id, TeamRole::Member)
        .await
        .expect("add member");

    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let issue_id = create_issue(&app.db, &project_id, &admin_id, "Held by bob").await;
    issues::update(
        &app.db,
        &issue_id,
        &project_id,
        &admin_id,
        issues::IssueFields {
            title: "Held by bob",
            description: "",
            status: peisear_core::IssueStatus::InProgress,
            priority: peisear_core::Priority::Medium,
            effort: Some(3),
            assignee_id: Some(&member_id),
        },
    )
    .await
    .expect("assign issue to bob");

    teams::remove_member(&app.db, &team_id, &member_id)
        .await
        .expect("remove bob from team");

    let candidates = issues::list_assignee_candidates(&app.db, &project_id)
        .await
        .expect("list candidates");
    assert!(
        !candidates.iter().any(|c| c.id == member_id),
        "removed member bob should not be a candidate: {candidates:?}"
    );

    let workload = issues::project_workload(&app.db, &project_id)
        .await
        .expect("list workload");
    let bob_row = workload
        .iter()
        .find(|w| w.user_id == member_id)
        .expect("removed member bob should still appear in the workload report");
    assert_eq!(bob_row.in_flight_issues, 1);
    assert_eq!(bob_row.in_flight_points, 3);
}

/// Test 7 -- requirement 4: assignment is not authorisation. A user
/// who is a valid assignee candidate for one project has no read
/// access to an unrelated project just because they're a candidate
/// somewhere. Guards specifically against a candidate query that
/// forgot its project filter.
#[tokio::test]
async fn being_a_valid_assignee_grants_no_read_access_to_other_projects() {
    let app_a = TestApp::spawn().await;
    // Both users share one DB (TestApp::spawn's pool), but drive
    // requests through separate authenticated sessions.
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app_a, &admin).await;
    let team_id = common::fixture::create_team_with_admin(&app_a.db, &admin_id, "TeamA").await;

    let member = TestUser::new("bob");
    let member_id = register_and_login(&app_a, &member).await;
    teams::add_member(&app_a.db, &team_id, &member_id, TeamRole::Member)
        .await
        .expect("add member");
    let _project_a = create_team_project(&app_a.db, &admin_id, &team_id, "ProjA").await;

    // A wholly unrelated project, owned by a third user with no
    // connection to bob or team A.
    let stranger = TestUser::new("carol");
    let stranger_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(
        &app_a.db,
        &stranger_id,
        &stranger.email,
        "x",
        &stranger.display_name,
    )
    .await
    .expect("insert stranger");
    let project_b = create_personal_project(&app_a.db, &stranger_id, "ProjB").await;

    // bob is logged in on app_a's server (shared cookie jar with
    // alice's session would be wrong -- register_and_login for bob
    // above already switched app_a's cookie jar to bob's session).
    let resp = app_a.server.get(&format!("/projects/{project_b}")).await;
    assert_eq!(
        resp.status_code(),
        axum::http::StatusCode::NOT_FOUND,
        "a valid assignee for team A's project must not be able to read an unrelated project"
    );
}
