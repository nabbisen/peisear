//! Optimistic-lock conflict tests — verify that mutation
//! endpoints reject stale `client_updated_at` with 409 Conflict
//! per peisear-feature-spec-v2.1 §21.4 / Appendix E.3.
//!
//! ## Pattern
//!
//! Each test follows the same shape:
//!
//! 1. Create an entity (issue, sprint, project, capacity).
//! 2. Read its `updated_at` — call this `t0`.
//! 3. Make a successful update — entity's `updated_at` advances
//!    to `t1`.
//! 4. Try a second update sending `client_updated_at = t0` —
//!    this is "stale" because the actual current value is `t1`.
//! 5. Assert 409 Conflict.
//!
//! ## Why we read updated_at directly from the DB in tests
//!
//! The production code path that exposes `updated_at` to the
//! browser is the form's hidden input — i.e. an HTML-rendered
//! string. Parsing HTML in tests would be brittle and add a
//! dep. Reading the DB directly is the test's privilege: we
//! own both ends of the wire.

mod common;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::{TestApp, ensure_distinct_timestamp};

// -------------------------------------------------------------------
// Issue updates
// -------------------------------------------------------------------

#[tokio::test]
async fn issue_update_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Original title").await;

    // 1. Read t0
    let t0 = read_issue_updated_at(&app, &issue_id).await;

    // 2. Successful first update advances updated_at to t1.
    ensure_distinct_timestamp().await;
    let resp = post_issue_update(&app, &project_id, &issue_id, &t0, "First edit").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::SEE_OTHER,
        "first update should redirect on success, got {}",
        resp.status_code()
    );

    // 3. Second update with the now-stale t0 must 409.
    let resp = post_issue_update(&app, &project_id, &issue_id, &t0, "Second edit").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "second update with stale client_updated_at must return 409, got {}",
        resp.status_code()
    );
}

#[tokio::test]
async fn issue_update_with_missing_client_updated_at_is_rejected() {
    // Submitting without any client_updated_at must not silently
    // bypass the lock. The handler returns 400 (validation
    // error: not a valid RFC3339 string) for an empty value —
    // either 400 or 409 is acceptable here as long as it's NOT
    // a 303 (silent success).
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let resp = post_issue_update(&app, &project_id, &issue_id, "", "edited").await;
    assert!(
        resp.status_code() == StatusCode::BAD_REQUEST
            || resp.status_code() == StatusCode::CONFLICT,
        "missing client_updated_at must be rejected (got {})",
        resp.status_code()
    );
}

#[tokio::test]
async fn issue_status_change_with_stale_timestamp_returns_409() {
    // The kanban DnD JSON endpoint accepts client_updated_at in
    // its JSON body (Phase A Step 5). Stale value → 409.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let t0 = read_issue_updated_at(&app, &issue_id).await;

    // Move to in_progress with valid t0 to advance updated_at.
    ensure_distinct_timestamp().await;
    let resp = post_status_change(&app, &project_id, &issue_id, &t0, "in_progress").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::NO_CONTENT,
        "first status change should succeed (got {})",
        resp.status_code()
    );

    // Now try to move to done with stale t0 — must 409.
    let resp = post_status_change(&app, &project_id, &issue_id, &t0, "done").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on status change must 409, got {}",
        resp.status_code()
    );
}

// -------------------------------------------------------------------
// Project updates
// -------------------------------------------------------------------

#[tokio::test]
async fn project_update_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Original").await;

    let t0 = read_project_updated_at(&app, &project_id).await;

    ensure_distinct_timestamp().await;
    let resp = post_project_update(&app, &project_id, &t0, "First rename", "desc1").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::SEE_OTHER,
        "first project rename should redirect on success, got {}",
        resp.status_code()
    );

    let resp = post_project_update(&app, &project_id, &t0, "Second rename", "desc2").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "second project rename with stale timestamp must 409, got {}",
        resp.status_code()
    );
}

// -------------------------------------------------------------------
// Sprint lifecycle (Step 5.2 pending)
// -------------------------------------------------------------------

#[tokio::test]
#[ignore = "client_updated_at plumbing for sprint endpoints — Phase A Step 5.2 pending"]
async fn sprint_start_with_stale_timestamp_returns_409() {
    // Activated in Step 5.2 once sprint mutations carry
    // client_updated_at and the team fixture helper lands.
    todo!("Step 5.2: write once sprint endpoints accept client_updated_at");
}

// -------------------------------------------------------------------
// Capacity period edits (Step 5.3 pending)
// -------------------------------------------------------------------

#[tokio::test]
#[ignore = "client_updated_at plumbing for capacity endpoints — Phase A Step 5.3 pending"]
async fn capacity_period_edit_with_stale_timestamp_returns_409() {
    todo!("Step 5.3: write once capacity endpoints accept client_updated_at");
}

// -------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------

/// Read the issue's current `updated_at` directly from the DB
/// and format it as RFC3339. This matches the string the
/// production handler writes into the form's hidden input via
/// `chrono::DateTime::to_rfc3339()`.
async fn read_issue_updated_at(app: &TestApp, issue_id: &str) -> String {
    let (updated_at,): (DateTime<Utc>,) =
        sqlx::query_as(r#"SELECT updated_at FROM issues WHERE id = ?1"#)
            .bind(issue_id)
            .fetch_one(&app.db)
            .await
            .expect("read updated_at");
    updated_at.to_rfc3339()
}

/// Read project's current updated_at as RFC3339, parallel to
/// `read_issue_updated_at`.
async fn read_project_updated_at(app: &TestApp, project_id: &str) -> String {
    let (updated_at,): (DateTime<Utc>,) =
        sqlx::query_as(r#"SELECT updated_at FROM projects WHERE id = ?1"#)
            .bind(project_id)
            .fetch_one(&app.db)
            .await
            .expect("read project updated_at");
    updated_at.to_rfc3339()
}

/// POST `/projects/{project_id}/edit` with a given
/// `client_updated_at` and a new name+description.
async fn post_project_update(
    app: &TestApp,
    project_id: &str,
    client_updated_at: &str,
    new_name: &str,
    new_description: &str,
) -> axum_test::TestResponse {
    let url = format!("/projects/{project_id}/edit");
    app.server
        .post(&url)
        .form(&[
            ("name", new_name),
            ("description", new_description),
            ("team_id", ""),
            ("client_updated_at", client_updated_at),
        ])
        .await
}

/// POST `/projects/{project_id}/issues/{issue_id}` with the
/// minimum form fields needed to satisfy validation. Title is
/// the only field the test changes; everything else is held
/// constant so the test asserts the lock check, not other
/// behaviour.
async fn post_issue_update(
    app: &TestApp,
    project_id: &str,
    issue_id: &str,
    client_updated_at: &str,
    new_title: &str,
) -> axum_test::TestResponse {
    let url = format!("/projects/{project_id}/issues/{issue_id}");
    app.server
        .post(&url)
        .form(&[
            ("title", new_title),
            ("description", "test body"),
            ("status", "open"),
            ("priority", "medium"),
            ("effort", ""),
            ("assignee_id", ""),
            ("client_updated_at", client_updated_at),
        ])
        .await
}

/// POST the kanban status-change JSON endpoint with a given
/// client_updated_at. Returns the response so the caller
/// asserts on its status code.
async fn post_status_change(
    app: &TestApp,
    project_id: &str,
    issue_id: &str,
    client_updated_at: &str,
    new_status: &str,
) -> axum_test::TestResponse {
    let url = format!("/projects/{project_id}/issues/{issue_id}/status");
    app.server
        .post(&url)
        .json(&serde_json::json!({
            "status": new_status,
            "client_updated_at": client_updated_at,
        }))
        .await
}
