//! Phase C PR1 sub-issue hierarchy tests.
//!
//! Coverage:
//!
//! 1. `list_in_project` returns top-level only — sub-issues
//!    don't appear on the project's main board / list (§8.5).
//! 2. The issue detail page renders the Sub-issues section
//!    for a top-level issue, and not for a sub-issue.
//! 3. Creating a sub-issue links it to the parent and the
//!    parent's detail page lists it.
//! 4. The 1-level constraint is enforced — POSTing to a
//!    sub-issue's `/sub-issues/new` form returns a 400 with
//!    a clear validation message.
//! 5. Sub-issues follow the parent's sprint —
//!    `sprint_for_issue` for a sub-issue returns the parent's
//!    sprint id even though the sub-issue has no
//!    `sprint_issues` row.
//! 6. The sprint-assignment endpoint refuses to assign a
//!    sprint directly to a sub-issue (POST returns 400 with
//!    "follow the parent's sprint" message).
//!
//! These together establish that sub-issues exist, are
//! reachable through the right URLs, and can't be made to
//! violate the spec's hierarchy/sprint rules.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;

/// Insert a sub-issue under an existing parent via storage.
/// Returns the new sub-issue's id. Used by tests that need a
/// sub-issue but don't want to exercise the form-post path.
async fn insert_sub_issue_via_storage(
    app: &TestApp,
    project_id: &str,
    parent_id: &str,
    author_id: &str,
    title: &str,
) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert_sub_issue(
        &app.db,
        &id,
        project_id,
        parent_id,
        author_id,
        title,
        "",
        peisear_core::IssueStatus::Open,
        peisear_core::Priority::Medium,
        None,
        None,
    )
    .await
    .expect("insert sub-issue");
    id
}

#[tokio::test]
async fn list_in_project_returns_top_level_only() {
    // Project with 2 top-level issues and 2 sub-issues. The
    // list should return 2.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Proj").await;

    let parent1 = create_issue(&app.db, &project_id, &user_id, "Top 1").await;
    let _parent2 = create_issue(&app.db, &project_id, &user_id, "Top 2").await;
    let _sub1 = insert_sub_issue_via_storage(&app, &project_id, &parent1, &user_id, "Sub a").await;
    let _sub2 = insert_sub_issue_via_storage(&app, &project_id, &parent1, &user_id, "Sub b").await;

    let issues = peisear_storage::issues::list_in_project(&app.db, &project_id)
        .await
        .expect("list_in_project");
    assert_eq!(
        issues.len(),
        2,
        "list_in_project should return only top-level issues; got {} rows",
        issues.len()
    );
    for issue in &issues {
        assert!(
            issue.is_top_level(),
            "all returned issues must be top-level; got sub-issue {}",
            issue.id
        );
    }

    // Sanity check the sub-issue helper.
    let subs = peisear_storage::issues::list_sub_issues_of(&app.db, &parent1)
        .await
        .expect("list_sub_issues_of");
    assert_eq!(subs.len(), 2, "parent should have 2 sub-issues");
}

#[tokio::test]
async fn detail_page_renders_sub_issues_section_for_top_level() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Proj").await;
    let parent_id = create_issue(&app.db, &project_id, &user_id, "Big task").await;
    let _sub_id =
        insert_sub_issue_via_storage(&app, &project_id, &parent_id, &user_id, "Small step").await;

    let url = format!("/projects/{project_id}/issues/{parent_id}");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains(r#"aria-label="Sub-issues""#),
        "Sub-issues section missing on top-level issue detail"
    );
    assert!(
        body.contains("Small step"),
        "child sub-issue title missing from parent detail page"
    );
    assert!(
        body.contains("+ Add sub-issue"),
        "Add sub-issue affordance missing"
    );
}

#[tokio::test]
async fn detail_page_omits_sub_issues_section_for_sub_issue() {
    // A sub-issue's own detail page should NOT display the
    // Sub-issues section (one-level rule means it can't have
    // children).
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Proj").await;
    let parent_id = create_issue(&app.db, &project_id, &user_id, "Big task").await;
    let sub_id =
        insert_sub_issue_via_storage(&app, &project_id, &parent_id, &user_id, "Step").await;

    let url = format!("/projects/{project_id}/issues/{sub_id}");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        !body.contains(r#"aria-label="Sub-issues""#),
        "Sub-issues section must not appear on a sub-issue's own detail page"
    );
    // Breadcrumb should include the parent title.
    assert!(
        body.contains("Big task"),
        "parent title missing from sub-issue's breadcrumb"
    );
}

#[tokio::test]
async fn create_sub_issue_via_form_links_to_parent() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Proj").await;
    let parent_id = create_issue(&app.db, &project_id, &user_id, "Top").await;

    let url = format!("/projects/{project_id}/issues/{parent_id}/sub-issues/new");
    let resp = app
        .server
        .post(&url)
        .form(&[
            ("title", "Form-created sub"),
            ("description", "Some text"),
            ("status", "open"),
            ("priority", "medium"),
            ("effort", ""),
            ("assignee_id", ""),
        ])
        .await;
    // Successful form submission redirects (303 SEE_OTHER) back
    // to the parent detail.
    assert_eq!(
        resp.status_code(),
        StatusCode::SEE_OTHER,
        "create_sub_issue should redirect on success; got {}",
        resp.status_code()
    );

    // Verify parent now lists the new child.
    let parent_url = format!("/projects/{project_id}/issues/{parent_id}");
    let parent_resp = app.server.get(&parent_url).await;
    parent_resp.assert_status(StatusCode::OK);
    let parent_body = parent_resp.text();
    assert!(
        parent_body.contains("Form-created sub"),
        "newly created sub-issue not visible on parent detail page"
    );
}

#[tokio::test]
async fn cannot_create_sub_issue_under_a_sub_issue() {
    // The 1-level rule. Try POSTing to /sub-issues/new under
    // an issue that's already a sub-issue. Handler short-
    // circuits with a Validation error before SQL.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Proj").await;
    let parent_id = create_issue(&app.db, &project_id, &user_id, "Top").await;
    let sub_id = insert_sub_issue_via_storage(&app, &project_id, &parent_id, &user_id, "Sub").await;

    let url = format!("/projects/{project_id}/issues/{sub_id}/sub-issues/new");
    let resp = app
        .server
        .post(&url)
        .form(&[
            ("title", "Grandchild"),
            ("description", ""),
            ("status", "open"),
            ("priority", "medium"),
            ("effort", ""),
            ("assignee_id", ""),
        ])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::BAD_REQUEST,
        "trying to nest sub-issues should be rejected; got {}",
        resp.status_code()
    );
}

#[tokio::test]
async fn sub_issue_inherits_parent_sprint() {
    // Parent in sprint S → sub-issue's `sprint_for_issue`
    // returns S even though there's no sprint_issues row for
    // the sub-issue.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    // Sprint requires team. Create team + planned sprint.
    let team_id = common::fixture::create_team_with_admin(&app.db, &user_id, "Eng").await;
    let sprint_id = common::fixture::create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    // Need a team-scoped project, not personal. Create
    // directly via storage since fixture::create_personal
    // makes personal projects only.
    let project_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::projects::insert(
        &app.db,
        &project_id,
        &user_id,
        "Team proj",
        "team-scoped",
        Some(&team_id),
    )
    .await
    .expect("insert team project");

    let parent_id = create_issue(&app.db, &project_id, &user_id, "Parent").await;
    let sub_id =
        insert_sub_issue_via_storage(&app, &project_id, &parent_id, &user_id, "Child").await;

    // Add parent to the sprint.
    peisear_storage::sprints::add_issue(&app.db, &sprint_id, &parent_id)
        .await
        .expect("add parent to sprint");

    // Sub-issue's sprint should be the parent's sprint.
    let resolved = peisear_storage::sprints::sprint_for_issue(&app.db, &sub_id)
        .await
        .expect("sprint_for_issue");
    assert_eq!(
        resolved.as_deref(),
        Some(sprint_id.as_str()),
        "sub-issue should follow parent's sprint; got {:?}",
        resolved
    );
}

#[tokio::test]
async fn cannot_assign_sprint_directly_to_sub_issue() {
    // POST to the sprint-assignment endpoint on a sub-issue's
    // URL should return 400 with the "follow the parent's
    // sprint" message.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    let team_id = common::fixture::create_team_with_admin(&app.db, &user_id, "Eng").await;
    let sprint_id = common::fixture::create_planned_sprint(&app.db, &team_id, "S1").await;

    let project_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::projects::insert(&app.db, &project_id, &user_id, "TP", "tp", Some(&team_id))
        .await
        .expect("project insert");

    let parent_id = create_issue(&app.db, &project_id, &user_id, "P").await;
    let sub_id = insert_sub_issue_via_storage(&app, &project_id, &parent_id, &user_id, "C").await;

    let url = format!("/projects/{project_id}/issues/{sub_id}/sprint");
    let resp = app
        .server
        .post(&url)
        .form(&[("sprint_id", sprint_id.as_str())])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::BAD_REQUEST,
        "direct sprint assignment to sub-issue should be 400; got {}",
        resp.status_code()
    );
}
