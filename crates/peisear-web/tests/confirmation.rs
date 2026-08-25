//! `CONF-001` (RFC 010) — the confirmation interstitial.
//!
//! Four destructive actions (project delete, issue delete,
//! planned-sprint delete, completed-sprint delete) move from a
//! `confirm()` dialog that silently disappears with JavaScript off,
//! to a server-rendered `GET` page. Five reversible actions (leave
//! team, remove member, detach project, remove capacity row, silence
//! all) are untouched.
//!
//! Coverage, matching the handoff's seven checks exactly:
//! 1. `GET` on each route renders an interstitial naming the entity.
//! 2. The project interstitial states the cascade to its issues.
//! 3. No-JS path: `GET` then `POST` the interstitial's own form
//!    deletes the entity.
//! 4. Regression guard: none of the four originating controls
//!    carries an `onsubmit` confirmation or is a form any more.
//! 5. Cancel targets the entity's parent; a `?return_to=` is ignored.
//! 6. The five reversible dialogs are untouched.
//! 7. Authorisation: a user who may not delete may not see the
//!    interstitial, matching what the `POST` already gives.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{
    create_issue, create_personal_project, create_planned_sprint, create_team_project,
    create_team_with_admin,
};
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_storage::{sprints, teams, user_capacities, users};

#[tokio::test]
async fn get_renders_an_interstitial_naming_the_specific_entity() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains("Delete Customer Portal?"),
        "project interstitial should name the project"
    );

    let issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains("Delete Fix login bug?"),
        "issue interstitial should name the issue"
    );

    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;
    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains("Delete Sprint Alpha?"),
        "sprint interstitial should name the sprint"
    );
}

#[tokio::test]
async fn project_interstitial_states_the_cascade_to_its_issues() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains("All its issues will be deleted too."),
        "project interstitial should state the cascade to its issues"
    );
}

/// `QA-006` §3: the issue interstitial names the cascade to its own
/// sub-issues too, once it has any — `issues.parent_issue_id` is
/// `ON DELETE CASCADE`, confirmed firing by `sub_issues::deleting_a_
/// parent_issue_cascades_to_its_sub_issues` before this copy was
/// written.
#[tokio::test]
async fn issue_interstitial_states_the_cascade_to_its_sub_issues() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let parent_id = create_issue(&app.db, &project_id, &user_id, "Parent issue").await;

    peisear_storage::issues::insert_sub_issue(
        &app.db,
        &uuid::Uuid::new_v4().to_string(),
        &project_id,
        &parent_id,
        &user_id,
        "Sub 1",
        "",
        peisear_core::IssueStatus::Open,
        peisear_core::Priority::Medium,
        None,
        None,
    )
    .await
    .expect("insert sub-issue");
    peisear_storage::issues::insert_sub_issue(
        &app.db,
        &uuid::Uuid::new_v4().to_string(),
        &project_id,
        &parent_id,
        &user_id,
        "Sub 2",
        "",
        peisear_core::IssueStatus::Open,
        peisear_core::Priority::Medium,
        None,
        None,
    )
    .await
    .expect("insert sub-issue");

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{parent_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains("This issue has 2 sub-issues. Deleting it deletes all of them too."),
        "issue interstitial should state the cascade to its sub-issues, naming the count: {body}"
    );
}

/// The unchanged case: an issue with no sub-issues still gets the
/// plain irreversibility note, not the cascade wording.
#[tokio::test]
async fn childless_issue_interstitial_is_unchanged() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Childless issue").await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains("This cannot be undone.") && !body.contains("sub-issue"),
        "a childless issue must keep the plain note, not the cascade wording: {body}"
    );
}

/// No-JS path, exercised via the sprint case: the interstitial's own
/// hidden `client_updated_at` field must round-trip correctly for
/// the `POST` to succeed. `QA-006` finding 1: the project and issue
/// interstitials now carry the same hidden field —
/// `owner_post_project_delete_still_works` and the cascade tests
/// above exercise those two; this one is kept as the sprint case,
/// the interstitial that had it first.
#[tokio::test]
async fn following_the_link_and_posting_the_form_deletes_the_entity_no_js() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    let client_updated_at = extract_hidden_field(&body, "client_updated_at")
        .expect("interstitial carries the hidden field");

    let resp = app
        .server
        .post(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .form(&[("client_updated_at", client_updated_at.as_str())])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let remaining = sprints::find_by_id(&app.db, &sprint_id)
        .await
        .expect("query sprint");
    assert!(remaining.is_none(), "sprint should be deleted");
}

#[tokio::test]
async fn none_of_the_four_originating_controls_carries_onsubmit_or_is_a_form() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/edit"))
        .await;
    let body = resp.text();
    assert!(
        !body.contains("onsubmit="),
        "project edit page should carry no onsubmit confirm() any more"
    );
    assert!(
        body.contains(&format!(r#"<a href="/projects/{project_id}/delete""#)),
        "project delete control should be a plain link: {body}"
    );

    let issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}"))
        .await;
    let body = resp.text();
    assert!(
        !body.contains("onsubmit="),
        "issue detail page should carry no onsubmit confirm() any more"
    );
    assert!(
        body.contains(&format!(
            r#"<a href="/projects/{project_id}/issues/{issue_id}/delete""#
        )),
        "issue delete control should be a plain link: {body}"
    );

    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let planned_id = create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;
    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{planned_id}"))
        .await;
    let body = resp.text();
    assert!(
        !body.contains("onsubmit="),
        "planned-sprint detail page should carry no onsubmit confirm() any more"
    );
    assert!(
        body.contains(&format!(
            r#"<a href="/teams/engineering/sprints/{planned_id}/delete""#
        )),
        "planned-sprint delete control should be a plain link: {body}"
    );

    let completed_id = create_planned_sprint(&app.db, &team_id, "Sprint Beta").await;
    sprints::start(&app.db, &completed_id).await.unwrap();
    sprints::complete(&app.db, &completed_id).await.unwrap();
    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{completed_id}"))
        .await;
    let body = resp.text();
    assert!(
        !body.contains("onsubmit="),
        "completed-sprint detail page should carry no onsubmit confirm() any more"
    );
    assert!(
        body.contains(&format!(
            r#"<a href="/teams/engineering/sprints/{completed_id}/delete""#
        )),
        "completed-sprint delete control should be a plain link: {body}"
    );
}

#[tokio::test]
async fn cancel_targets_the_parent_and_return_to_is_ignored() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let resp = app
        .server
        .get(&format!(
            "/projects/{project_id}/delete?return_to=https://evil.example"
        ))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains(r#"<a href="/projects""#),
        "project cancel should target the project list, ignoring return_to"
    );
    assert!(
        !resp.text().contains("evil.example"),
        "a caller-supplied return_to must not be reflected anywhere"
    );

    let issue_id = create_issue(&app.db, &project_id, &user_id, "Fix login bug").await;
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text()
            .contains(&format!(r#"<a href="/projects/{project_id}""#)),
        "issue cancel should target the project detail page"
    );

    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;
    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text()
            .contains(r#"<a href="/teams/engineering/sprints""#),
        "sprint cancel should target the team's sprint list"
    );
}

#[tokio::test]
async fn five_reversible_dialogs_are_untouched() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let _project_id = create_team_project(&app.db, &user_id, &team_id, "Shared Project").await;

    let bob_id = uuid::Uuid::new_v4().to_string();
    users::insert(&app.db, &bob_id, "bob@example.com", "x", "Bob")
        .await
        .expect("insert bob");
    teams::add_member(&app.db, &team_id, &bob_id, TeamRole::Member)
        .await
        .expect("add bob to team");

    let resp = app.server.get("/teams/engineering").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains("Detach this project from the team?"),
        "detach dialog should be untouched"
    );
    assert!(
        body.contains("Leave this team?"),
        "leave dialog should be untouched"
    );
    assert!(
        body.contains("Remove this member from the team?"),
        "remove-member dialog should be untouched"
    );

    user_capacities::insert(&app.db, &user_id, 5, None, None, None)
        .await
        .expect("insert capacity row");
    let resp = app.server.get("/settings").await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains("Remove this capacity row?"),
        "remove-capacity-row dialog should be untouched"
    );

    let resp = app.server.get("/settings/notifications").await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains("Silence all notification kinds?"),
        "silence-all dialog should be untouched"
    );
}

#[tokio::test]
async fn authorisation_matches_the_corresponding_post_per_route() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;
    let team_id = create_team_with_admin(&app.db, &owner_id, "Engineering").await;
    let project_id = create_team_project(&app.db, &owner_id, &team_id, "Shared Project").await;
    let issue_id = create_issue(&app.db, &project_id, &owner_id, "Fix login bug").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;

    // -- Project: bob is a team admin (so `find_accessible` succeeds)
    // but is not the project's owner, which is the POST's actual
    // gate (`projects::delete` scopes on `owner_id`, not team
    // membership). `register_and_login` switches the shared cookie
    // jar to the newly registered user.
    common::auth::logout(&app).await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&app, &bob).await;
    teams::add_member(&app.db, &team_id, &bob_id, TeamRole::Admin)
        .await
        .expect("add bob to team as admin");

    let get_resp = app
        .server
        .get(&format!("/projects/{project_id}/delete"))
        .await;
    assert_eq!(
        get_resp.status_code(),
        StatusCode::NOT_FOUND,
        "bob is a team admin but not the project's owner_id -- GET should 404 like the POST does"
    );
    let post_resp = app
        .server
        .post(&format!("/projects/{project_id}/delete"))
        .form(&[("client_updated_at", "irrelevant")])
        .await;
    assert_eq!(
        post_resp.status_code(),
        StatusCode::NOT_FOUND,
        "POST confirms the same 404 for the same actor"
    );

    // -- Issue: carol has no relationship to the team/project at
    // all -- `find_accessible` itself should deny her.
    common::auth::logout(&app).await;
    let carol = TestUser::new("carol");
    register_and_login(&app, &carol).await;

    let get_resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}/delete"))
        .await;
    assert_eq!(
        get_resp.status_code(),
        StatusCode::NOT_FOUND,
        "carol has no project access -- GET should 404 like the POST does"
    );
    let post_resp = app
        .server
        .post(&format!("/projects/{project_id}/issues/{issue_id}/delete"))
        .form(&[("client_updated_at", "irrelevant")])
        .await;
    assert_eq!(
        post_resp.status_code(),
        StatusCode::NOT_FOUND,
        "POST confirms the same 404 for the same actor"
    );

    // -- Sprint: dave is a plain team Member -- team access without
    // `can_manage_team()`, the POST's actual gate.
    common::auth::logout(&app).await;
    let dave = TestUser::new("dave");
    let dave_id = register_and_login(&app, &dave).await;
    teams::add_member(&app.db, &team_id, &dave_id, TeamRole::Member)
        .await
        .expect("add dave to team as member");

    let get_resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .await;
    assert_eq!(
        get_resp.status_code(),
        StatusCode::FORBIDDEN,
        "dave is a member but not an admin -- GET should 403 like the POST does"
    );
    let post_resp = app
        .server
        .post(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .form(&[("client_updated_at", "irrelevant")])
        .await;
    assert_eq!(
        post_resp.status_code(),
        StatusCode::FORBIDDEN,
        "POST confirms the same 403 for the same actor"
    );
}

/// `QA-002` item 1, test 2: the confirmation `GET` refuses an
/// `Active` sprint too, rather than rendering "you are about to
/// delete *X*" for a team's running sprint.
#[tokio::test]
async fn get_confirmation_refuses_an_active_sprint() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;
    sprints::start(&app.db, &sprint_id).await.expect("start");

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::BAD_REQUEST,
        "GET should refuse an active sprint, not render a confirmation for it"
    );
    assert!(
        !resp.text().contains("Delete Sprint Alpha?"),
        "no interstitial should render for an active sprint"
    );
}

/// `QA-002` item 1, test 3: planned and completed sprints still
/// delete via both halves (`GET` then `POST`) after the `Active`
/// refusal landed. `following_the_link_and_posting_the_form_deletes_the_entity_no_js`
/// already covers the planned case; this covers completed, the other
/// status the shared route serves.
#[tokio::test]
async fn completed_sprint_still_deletes_via_both_halves() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint Alpha").await;
    sprints::start(&app.db, &sprint_id).await.expect("start");
    sprints::complete(&app.db, &sprint_id)
        .await
        .expect("complete");

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    let client_updated_at = extract_hidden_field(&body, "client_updated_at")
        .expect("interstitial carries the hidden field");

    let resp = app
        .server
        .post(&format!("/teams/engineering/sprints/{sprint_id}/delete"))
        .form(&[("client_updated_at", client_updated_at.as_str())])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let remaining = sprints::find_by_id(&app.db, &sprint_id)
        .await
        .expect("query sprint");
    assert!(remaining.is_none(), "completed sprint should be deleted");
}

/// `QA-002` item 2, test 4: a non-owner's `POST` to the project
/// delete route must not report success. Reproduced against
/// unmodified code before the fix landed (see the review package);
/// this is that same scenario, held in place going forward.
#[tokio::test]
async fn non_owner_post_project_delete_returns_404_and_project_survives() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;
    let team_id = create_team_with_admin(&app.db, &owner_id, "Engineering").await;
    let project_id = create_team_project(&app.db, &owner_id, &team_id, "Shared Project").await;

    common::auth::logout(&app).await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&app, &bob).await;
    teams::add_member(&app.db, &team_id, &bob_id, TeamRole::Admin)
        .await
        .expect("add bob to team as admin");

    let resp = app
        .server
        .post(&format!("/projects/{project_id}/delete"))
        .form(&[("client_updated_at", "irrelevant")])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::NOT_FOUND,
        "a non-owner's delete must not report success"
    );

    let still_there =
        peisear_storage::projects::find_accessible(&app.db, &project_id, &owner_id).await;
    assert!(
        still_there.is_ok(),
        "the project must survive a non-owner's delete attempt"
    );
}

/// `QA-002-review.md` §4.2: a genuine gap the review found — nothing
/// previously exercised a *successful* owner-initiated project
/// delete via `POST` anywhere in the suite.
#[tokio::test]
async fn owner_post_project_delete_still_works() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;
    let project_id = create_personal_project(&app.db, &owner_id, "Customer Portal").await;

    let get_resp = app
        .server
        .get(&format!("/projects/{project_id}/delete"))
        .await;
    get_resp.assert_status(StatusCode::OK);
    let client_updated_at = extract_hidden_field(&get_resp.text(), "client_updated_at")
        .expect("interstitial carries the hidden field");

    let resp = app
        .server
        .post(&format!("/projects/{project_id}/delete"))
        .form(&[("client_updated_at", client_updated_at.as_str())])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let still_there =
        peisear_storage::projects::find_accessible(&app.db, &project_id, &owner_id).await;
    assert!(
        still_there.is_err(),
        "the project should be gone after the owner's own delete"
    );
}

/// Pull `value="..."` from `<input type="hidden" name="{field}"
/// value="...">` in rendered HTML. Minimal, not a general HTML
/// parser -- fine for this test's one known shape.
fn extract_hidden_field(body: &str, field: &str) -> Option<String> {
    let marker = format!(r#"name="{field}" value=""#);
    let start = body.find(&marker)? + marker.len();
    let end = body[start..].find('"')? + start;
    Some(body[start..end].to_string())
}
