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
use common::fixture::{
    create_issue, create_personal_project, create_planned_sprint, create_team_with_admin,
};
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
// Sprint lifecycle
// -------------------------------------------------------------------

#[tokio::test]
async fn sprint_start_with_stale_timestamp_returns_409() {
    // Workflow:
    // 1. Create team + planned sprint, read its `updated_at` (t0).
    // 2. Edit the sprint (e.g. rename) so its `updated_at`
    //    advances to t1 — this simulates a concurrent edit
    //    landing between page render and form submit.
    // 3. POST `/start` with `client_updated_at = t0`. Must 409.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let team_slug = read_team_slug(&app, &team_id).await;

    let t0 = read_sprint_updated_at(&app, &sprint_id).await;

    // Concurrent edit: bump updated_at by editing the sprint.
    // Going through the storage layer is enough — we don't need
    // to exercise the form path here.
    ensure_distinct_timestamp().await;
    let today = chrono::Utc::now().date_naive();
    let ends = today + chrono::Duration::days(14);
    peisear_storage::sprints::update(
        &app.db,
        &sprint_id,
        "Sprint 1 (renamed)",
        None,
        today,
        ends,
    )
    .await
    .expect("rename sprint");

    // Now try to start with the stale t0.
    let url = format!("/teams/{team_slug}/sprints/{sprint_id}/start");
    let resp = app.server.post(&url).form(&[("client_updated_at", t0.as_str())]).await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on /start must 409 (got {})",
        resp.status_code()
    );
}

// -------------------------------------------------------------------
// Capacity period edits
// -------------------------------------------------------------------

#[tokio::test]
async fn capacity_period_edit_with_stale_timestamp_returns_409() {
    // Parallel to issue_update test: create row, edit once with
    // valid t0, edit again with the now-stale t0 → 409.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let row_id = create_capacity_row(&app, &user_id, 8).await;

    let t0 = read_capacity_updated_at(&app, &row_id).await;

    ensure_distinct_timestamp().await;
    let resp = post_capacity_update(&app, &row_id, &t0, 10).await;
    assert_eq!(
        resp.status_code(),
        StatusCode::SEE_OTHER,
        "first capacity update should redirect on success, got {}",
        resp.status_code()
    );

    // Second update with the now-stale t0 must 409.
    let resp = post_capacity_update(&app, &row_id, &t0, 12).await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "second capacity update with stale client_updated_at must return 409, got {}",
        resp.status_code()
    );
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

// ── Sprint helpers ─────────────────────────────────────────

/// Read a sprint's `updated_at` directly from the DB and
/// format as RFC3339, matching what the production handler
/// puts in form hidden inputs.
async fn read_sprint_updated_at(app: &TestApp, sprint_id: &str) -> String {
    let (updated_at,): (DateTime<Utc>,) =
        sqlx::query_as(r#"SELECT updated_at FROM sprints WHERE id = ?1"#)
            .bind(sprint_id)
            .fetch_one(&app.db)
            .await
            .expect("read sprint updated_at");
    updated_at.to_rfc3339()
}

/// Read a team's slug from the DB. Tests that create a team
/// only know its id; the URL needs the slug, and computing it
/// from the team name is fragile (special chars, dedup), so
/// we just look it up.
async fn read_team_slug(app: &TestApp, team_id: &str) -> String {
    let (slug,): (String,) = sqlx::query_as(r#"SELECT slug FROM teams WHERE id = ?1"#)
        .bind(team_id)
        .fetch_one(&app.db)
        .await
        .expect("read team slug");
    slug
}

// ── Capacity helpers ───────────────────────────────────────

/// Insert a capacity period row directly via storage. Used
/// over the form path because we don't need to exercise form
/// validation here — just need a row to lock against.
async fn create_capacity_row(app: &TestApp, user_id: &str, points: i64) -> String {
    peisear_storage::user_capacities::insert(
        &app.db, user_id, points, None, None, None,
    )
    .await
    .expect("insert capacity row")
}

/// Read a capacity row's `updated_at`, RFC3339-formatted.
async fn read_capacity_updated_at(app: &TestApp, row_id: &str) -> String {
    let (updated_at,): (DateTime<Utc>,) =
        sqlx::query_as(r#"SELECT updated_at FROM user_capacities WHERE id = ?1"#)
            .bind(row_id)
            .fetch_one(&app.db)
            .await
            .expect("read capacity updated_at");
    updated_at.to_rfc3339()
}

/// POST `/settings/capacity/{row_id}` with a given
/// client_updated_at and a new points value. Holds period and
/// note constant — only points changes — so the test asserts
/// on the lock check rather than other validation.
async fn post_capacity_update(
    app: &TestApp,
    row_id: &str,
    client_updated_at: &str,
    new_points: i64,
) -> axum_test::TestResponse {
    let url = format!("/settings/capacity/{row_id}");
    let points_str = new_points.to_string();
    app.server
        .post(&url)
        .form(&[
            ("points", points_str.as_str()),
            ("period_start", ""),
            ("period_end", ""),
            ("note", ""),
            ("client_updated_at", client_updated_at),
        ])
        .await
}
