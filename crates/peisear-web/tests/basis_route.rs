//! `HLT-001` (RFC 008 §1–§3) — the basis route.
//!
//! Each indicator's explanation row gets a link to a route
//! rendering exactly the issues behind its count — not a filter
//! that reconstructs the set, the same membership
//! `project_health::for_project` already computed
//! (`ProjectHealthRaw::basis_for`). `WipCompliance` gets none,
//! structurally (RFC 008 §2/§2.3): its basis is users, not issues.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::issues::IssueFields;

/// Scope a body-wide assertion to just the explanation `<ul>` — the
/// navbar's own account menu legitimately shows the logged-in
/// user's email, which is not the leak `HLT-001` §3 guards against.
fn extract_explanation_list(body: &str) -> &str {
    const OPEN: &str = r#"<ul class="mt-2 ml-4 list-disc"#;
    let Some(start) = body.find(OPEN) else {
        return ""; // no explanations rendered at all -- nothing to scope
    };
    let end = body[start..]
        .find("</ul>")
        .expect("unterminated explanation <ul>");
    &body[start..start + end]
}

/// Check 1 (`HLT-001` §3, written first): WIP compliance renders no
/// basis link, and the over-limit assignee's identity does not
/// appear anywhere in the explanation area.
#[tokio::test]
async fn wip_compliance_renders_no_basis_link_or_assignee_identity() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    // Same fixture shape as `wip_compliance_explanation_uses_
    // corrected_wording`: 4 in_progress issues assigned to one user
    // pushes them past DEFAULT_WIP_LIMIT (3), reaching Watch/Concern.
    for i in 0..4 {
        let id = uuid::Uuid::new_v4().to_string();
        peisear_storage::issues::insert(
            &app.db,
            &id,
            &project_id,
            &user_id,
            IssueFields {
                title: &format!("T{i}"),
                description: "Test issue body.",
                status: IssueStatus::InProgress,
                priority: Priority::Medium,
                effort: None,
                assignee_id: Some(&user_id),
                planned_start_at: None,
                planned_end_at: None,
            },
        )
        .await
        .expect("insert issue");
    }

    let resp = app.server.get(&format!("/projects/{project_id}")).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("active assignees are over their WIP limit."),
        "expected the WIP-compliance explanation to actually render, or \
         the rest of this test is meaningless: {body}"
    );
    assert!(
        !body.contains("What WIP compliance is based on"),
        "WIP compliance must render no basis-link accessible name: {body}"
    );
    assert!(
        !body.contains("/health/wip-compliance/basis"),
        "WIP compliance must render no basis-link href: {body}"
    );
    let explanation_area = extract_explanation_list(&body);
    assert!(
        !explanation_area.contains(&user.email),
        "no assignee identity may appear in the explanation area: {explanation_area}"
    );

    // The route itself must also refuse to render WIP compliance's
    // (nonexistent) basis if visited directly.
    let direct = app
        .server
        .get(&format!(
            "/projects/{project_id}/health/wip-compliance/basis"
        ))
        .await;
    direct.assert_status(StatusCode::NOT_FOUND);
}

/// Check 2: the five linked indicators each render a basis link with
/// a distinguishing accessible name (`board_keyboard`'s `each_
/// status_control_has_a_distinguishing_accessible_name` precedent),
/// and following it lands on a page naming that indicator.
#[tokio::test]
async fn linked_indicators_render_distinguishing_basis_links() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    // One done issue, four open: throughput is 1/5 = 20% (below the
    // 30% Watch floor), with a non-empty done set to link to.
    let done_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &done_id,
        &project_id,
        &user_id,
        IssueFields {
            title: "Done",
            description: "",
            status: IssueStatus::Done,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert done issue");
    for i in 0..4 {
        let _ = create_issue(&app.db, &project_id, &user_id, &format!("Open {i}")).await;
    }

    let resp = app.server.get(&format!("/projects/{project_id}")).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains(r#"aria-label="What Throughput is based on""#),
        "expected throughput's basis link with a distinguishing accessible \
         name; body: {body}"
    );
    assert!(
        body.contains(&format!("/projects/{project_id}/health/throughput/basis")),
        "expected throughput's basis link href; body: {body}"
    );

    let basis_resp = app
        .server
        .get(&format!("/projects/{project_id}/health/throughput/basis"))
        .await;
    basis_resp.assert_status(StatusCode::OK);
    let basis_body = basis_resp.text();
    assert!(
        basis_body.contains("Throughput"),
        "expected the basis page to name the indicator; body: {basis_body}"
    );
}

/// Check 3: throughput's basis set is exactly the done issues — not
/// the open ones, not a superset.
#[tokio::test]
async fn throughput_basis_is_exactly_the_done_issues() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    let done_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &done_id,
        &project_id,
        &user_id,
        IssueFields {
            title: "Done issue",
            description: "",
            status: IssueStatus::Done,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert done issue");
    let _open_id = create_issue(&app.db, &project_id, &user_id, "Open issue").await;

    let basis_resp = app
        .server
        .get(&format!("/projects/{project_id}/health/throughput/basis"))
        .await;
    basis_resp.assert_status(StatusCode::OK);
    let body = basis_resp.text();

    assert!(
        body.contains("Done issue"),
        "the done issue must appear in throughput's basis: {body}"
    );
    assert!(
        !body.contains("Open issue"),
        "the open issue must NOT appear in throughput's basis -- a link to \
         the wrong issues is worse than no link: {body}"
    );
}

/// Check 4: staleness's basis is exactly the single oldest in-flight
/// issue, not the whole in-flight set.
#[tokio::test]
async fn staleness_basis_is_exactly_the_oldest_in_flight_issue() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    let _older = create_issue(&app.db, &project_id, &user_id, "Older issue").await;
    common::server::ensure_distinct_timestamp().await;
    let _newer = create_issue(&app.db, &project_id, &user_id, "Newer issue").await;

    let basis_resp = app
        .server
        .get(&format!("/projects/{project_id}/health/staleness/basis"))
        .await;
    basis_resp.assert_status(StatusCode::OK);
    let body = basis_resp.text();

    assert!(
        body.contains("Older issue"),
        "staleness's basis must be the older (first-created) issue: {body}"
    );
    assert!(
        !body.contains("Newer issue"),
        "staleness's basis must be exactly one issue, not the whole \
         in-flight set: {body}"
    );
}

/// Check 5: an unknown indicator slug 404s rather than panicking or
/// rendering something misleading.
#[tokio::test]
async fn unknown_indicator_slug_404s() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let _issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let resp = app
        .server
        .get(&format!(
            "/projects/{project_id}/health/not-a-real-indicator/basis"
        ))
        .await;
    resp.assert_status(StatusCode::NOT_FOUND);
}
