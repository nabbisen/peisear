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

/// `QA-006` finding 1: the issue delete route now locks too.
#[tokio::test]
async fn issue_delete_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Original title").await;

    let t0 = read_issue_updated_at(&app, &issue_id).await;

    ensure_distinct_timestamp().await;
    post_issue_update(&app, &project_id, &issue_id, &t0, "Renamed").await;

    let resp = app
        .server
        .post(&format!("/projects/{project_id}/issues/{issue_id}/delete"))
        .form(&[("client_updated_at", t0.as_str())])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "delete with a stale client_updated_at must 409, got {}",
        resp.status_code()
    );

    let still_there = peisear_storage::issues::find(&app.db, &issue_id, &project_id).await;
    assert!(
        still_there.is_ok(),
        "a rejected delete must not remove the issue"
    );
}

/// `CAL-001` §2.4 (RFC 002): `planned_start_at`/`planned_end_at` join
/// the existing issue `UPDATE`'s `SET` clause rather than getting a
/// statement of their own. This test exists specifically because
/// `issue_update_with_stale_timestamp_returns_409` above never
/// touches the two new columns at all — it cannot catch a regression
/// where *only* a planned-dates edit escapes the lock.
///
/// **Not demonstrated failing first, unlike this project's usual
/// discipline for a guard correction — reported rather than forced.**
/// Splitting the two columns into a second `UPDATE` *inside this same
/// function* does not fail this test: `check_optimistic_lock` compares
/// the submitted timestamp against the row's live `updated_at`
/// *before any write runs*, and every request through this endpoint
/// resubmits title/status/etc. unconditionally, so the co-located
/// write's own `updated_at` touch still advances the clock on every
/// save regardless of whether the date columns share its statement.
/// Verified this empirically (a naive split still passes) before
/// writing this doc comment, rather than assuming it from the code.
///
/// The failure `§2.4` actually describes needs a genuinely decoupled
/// write path — one that does not go through this endpoint's lock
/// check at all (e.g. a future direct-write function CAL-002's
/// eventual drag-and-drop reschedule might add). Built a throwaway
/// function shaped exactly like that, called it directly to simulate
/// an out-of-band planned-date write, then submitted a normal edit
/// holding the pre-write timestamp: it returned 303 (silent success),
/// confirming the risk is real for *that* shape of mistake — evidence
/// captured, then the throwaway function deleted (see the review
/// request). Joining the columns into this statement is what makes
/// that shape of mistake structurally unreachable through the shipped
/// app today: there is no legitimate call site that writes planned
/// dates without going through this lock-checked statement. This test
/// still guards a real, narrower property — planned-date submissions
/// go through the same locked endpoint as everything else — even
/// though it has no naive-but-broken state reachable via this form to
/// demonstrate against.
#[tokio::test]
async fn issue_planned_dates_only_edit_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Plan me").await;

    let t0 = read_issue_updated_at(&app, &issue_id).await;

    ensure_distinct_timestamp().await;
    let resp =
        post_issue_planned_dates_update(&app, &project_id, &issue_id, &t0, "2026-09-01T09:00")
            .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::SEE_OTHER,
        "first planned-dates-only edit should redirect on success, got {}",
        resp.status_code()
    );

    let resp =
        post_issue_planned_dates_update(&app, &project_id, &issue_id, &t0, "2026-09-02T09:00")
            .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "second planned-dates-only edit with stale client_updated_at must 409, got {}",
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
        resp.status_code() == StatusCode::BAD_REQUEST || resp.status_code() == StatusCode::CONFLICT,
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
        // `STATUS-002` §3: the endpoint now returns 200 + the new
        // `updated_at`, not a bare 204 -- `board.js` only ever
        // checked `res.status === 409`/`!res.ok`, so this was
        // confirmed not to be a behaviour change for that surface.
        StatusCode::OK,
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

#[tokio::test]
async fn issue_status_change_with_missing_client_updated_at_is_rejected() {
    // Regression test for §10.6: the kanban status endpoint used to
    // accept a request with no client_updated_at at all and apply the
    // mutation silently (Phase A rollout bypass that outlived Phase A
    // by three releases). A missing lock value must be rejected like
    // any other mutation path (`NFR-CONC-005`), and the row must be
    // left unchanged.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}/status");
    let resp = app
        .server
        .post(&url)
        .json(&serde_json::json!({ "status": "in_progress" }))
        .await;

    assert_eq!(
        resp.status_code(),
        StatusCode::BAD_REQUEST,
        "missing client_updated_at on status change must be rejected with 400, got {}",
        resp.status_code()
    );

    let body = resp.text();
    assert!(
        !body.contains("Failed") && !body.contains("Error:"),
        "rejection message must not use failure vocabulary, got: {body}"
    );

    let status_after = read_issue_status(&app, &issue_id).await;
    assert_eq!(
        status_after, "open",
        "a rejected status change must not mutate the row"
    );
}

#[tokio::test]
async fn issue_status_change_with_empty_client_updated_at_is_rejected() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let resp = post_status_change(&app, &project_id, &issue_id, "", "in_progress").await;

    assert_eq!(
        resp.status_code(),
        StatusCode::BAD_REQUEST,
        "empty client_updated_at on status change must be rejected with 400, got {}",
        resp.status_code()
    );

    let body = resp.text();
    assert!(
        !body.contains("Failed") && !body.contains("Error:"),
        "rejection message must not use failure vocabulary, got: {body}"
    );

    let status_after = read_issue_status(&app, &issue_id).await;
    assert_eq!(
        status_after, "open",
        "a rejected status change must not mutate the row"
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

/// `QA-006` finding 1: the project delete route now locks too.
#[tokio::test]
async fn project_delete_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Original").await;

    let t0 = read_project_updated_at(&app, &project_id).await;

    ensure_distinct_timestamp().await;
    post_project_update(&app, &project_id, &t0, "Renamed", "desc").await;

    let resp = app
        .server
        .post(&format!("/projects/{project_id}/delete"))
        .form(&[("client_updated_at", t0.as_str())])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "delete with a stale client_updated_at must 409, got {}",
        resp.status_code()
    );

    let still_there =
        peisear_storage::projects::find_accessible(&app.db, &project_id, &user_id).await;
    assert!(
        still_there.is_ok(),
        "a rejected delete must not remove the project"
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
    peisear_storage::sprints::update(&app.db, &sprint_id, "Sprint 1 (renamed)", None, today, ends)
        .await
        .expect("rename sprint");

    // Now try to start with the stale t0.
    let url = format!("/teams/{team_slug}/sprints/{sprint_id}/start");
    let resp = app
        .server
        .post(&url)
        .form(&[("client_updated_at", t0.as_str())])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on /start must 409 (got {})",
        resp.status_code()
    );
}

/// `QA-006` §4: `/edit` and `/complete` share `/start`'s lock check
/// (all three call `check_optimistic_lock` against the same sprint
/// row) but neither had a test naming its own route — RFC 005 §2's
/// table recorded `/start`'s test as covering `/edit` "analogously",
/// which is coverage of the requirement, not of the entry point
/// (`NFR-CONC-005`'s own gap, restated: a shared code path is not the
/// same claim as a tested route).
#[tokio::test]
async fn sprint_edit_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let team_slug = read_team_slug(&app, &team_id).await;

    let t0 = read_sprint_updated_at(&app, &sprint_id).await;

    ensure_distinct_timestamp().await;
    let today = chrono::Utc::now().date_naive();
    let ends = today + chrono::Duration::days(14);
    peisear_storage::sprints::update(&app.db, &sprint_id, "Sprint 1 (renamed)", None, today, ends)
        .await
        .expect("rename sprint");

    let url = format!("/teams/{team_slug}/sprints/{sprint_id}/edit");
    let resp = app
        .server
        .post(&url)
        .form(&[
            ("name", "Sprint 1 (edited again)"),
            ("starts_on", &today.to_string()),
            ("ends_on", &ends.to_string()),
            ("client_updated_at", t0.as_str()),
        ])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on /edit must 409 (got {})",
        resp.status_code()
    );
}

/// See `sprint_edit_with_stale_timestamp_returns_409` — same gap,
/// `/complete` half.
#[tokio::test]
async fn sprint_complete_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let team_slug = read_team_slug(&app, &team_id).await;
    peisear_storage::sprints::start(&app.db, &sprint_id)
        .await
        .expect("start sprint");

    let t0 = read_sprint_updated_at(&app, &sprint_id).await;

    ensure_distinct_timestamp().await;
    let today = chrono::Utc::now().date_naive();
    let ends = today + chrono::Duration::days(14);
    peisear_storage::sprints::update(&app.db, &sprint_id, "Sprint 1 (renamed)", None, today, ends)
        .await
        .expect("rename sprint");

    let url = format!("/teams/{team_slug}/sprints/{sprint_id}/complete");
    let resp = app
        .server
        .post(&url)
        .form(&[("client_updated_at", t0.as_str())])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on /complete must 409 (got {})",
        resp.status_code()
    );
}

/// See `sprint_edit_with_stale_timestamp_returns_409` — same gap,
/// `/delete` half. `sprint_plan.rs::delete_refuses_an_active_sprint`
/// posts to this same route but asserts the `Active`-status 400, not
/// a stale-lock 409 — a different requirement, not this one.
#[tokio::test]
async fn sprint_delete_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    let team_slug = read_team_slug(&app, &team_id).await;

    let t0 = read_sprint_updated_at(&app, &sprint_id).await;

    ensure_distinct_timestamp().await;
    let today = chrono::Utc::now().date_naive();
    let ends = today + chrono::Duration::days(14);
    peisear_storage::sprints::update(&app.db, &sprint_id, "Sprint 1 (renamed)", None, today, ends)
        .await
        .expect("rename sprint");

    let url = format!("/teams/{team_slug}/sprints/{sprint_id}/delete");
    let resp = app
        .server
        .post(&url)
        .form(&[("client_updated_at", t0.as_str())])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on /delete must 409 (got {})",
        resp.status_code()
    );

    let still_there = peisear_storage::sprints::find_by_id(&app.db, &sprint_id)
        .await
        .expect("query sprint");
    assert!(
        still_there.is_some(),
        "a rejected delete must not remove the sprint"
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

/// See `sprint_edit_with_stale_timestamp_returns_409` — same gap,
/// `settings::delete_capacity`: locks, has an existing 409 sibling
/// (`capacity_period_edit_with_stale_timestamp_returns_409`) on the
/// update route, but its own `/delete` route had no test naming it.
#[tokio::test]
async fn capacity_delete_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let row_id = create_capacity_row(&app, &user_id, 8).await;

    let t0 = read_capacity_updated_at(&app, &row_id).await;

    ensure_distinct_timestamp().await;
    let resp = post_capacity_update(&app, &row_id, &t0, 10).await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let resp = app
        .server
        .post(&format!("/settings/capacity/{row_id}/delete"))
        .form(&[("client_updated_at", t0.as_str())])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on /delete must 409, got {}",
        resp.status_code()
    );

    let still_there = peisear_storage::user_capacities::find(&app.db, &user_id, &row_id)
        .await
        .expect("query capacity row");
    assert!(
        still_there.is_some(),
        "a rejected delete must not remove the capacity row"
    );
}

/// `QA-006` §4: `POST /settings/capacity/{id}/close` locks
/// (`settings::close_capacity`) but was in neither RFC 005 §2's
/// table nor the test suite — a genuinely missing row and a missing
/// test, not the `/edit`/`/complete` shape (which had a lock and a
/// sibling-route test, just not one naming their own route).
#[tokio::test]
async fn capacity_close_with_stale_timestamp_returns_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let row_id = create_capacity_row(&app, &user_id, 8).await;

    let t0 = read_capacity_updated_at(&app, &row_id).await;

    ensure_distinct_timestamp().await;
    let resp = post_capacity_update(&app, &row_id, &t0, 10).await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let resp = app
        .server
        .post(&format!("/settings/capacity/{row_id}/close"))
        .form(&[
            ("period_end", "2026-12-31"),
            ("client_updated_at", t0.as_str()),
        ])
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::CONFLICT,
        "stale client_updated_at on /close must 409, got {}",
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

/// Read an issue's current `status` column directly from the DB,
/// so a rejection test can assert the mutation did not apply.
async fn read_issue_status(app: &TestApp, issue_id: &str) -> String {
    let (status,): (String,) = sqlx::query_as(r#"SELECT status FROM issues WHERE id = ?1"#)
        .bind(issue_id)
        .fetch_one(&app.db)
        .await
        .expect("read issue status");
    status
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

/// POST `/projects/{project_id}/issues/{issue_id}` changing only
/// `planned_start_at`, holding every other field constant. Title
/// matches `create_issue`'s fixture value so this is genuinely a
/// planned-dates-only edit, not a disguised title change.
async fn post_issue_planned_dates_update(
    app: &TestApp,
    project_id: &str,
    issue_id: &str,
    client_updated_at: &str,
    planned_start_at: &str,
) -> axum_test::TestResponse {
    let url = format!("/projects/{project_id}/issues/{issue_id}");
    app.server
        .post(&url)
        .form(&[
            ("title", "Plan me"),
            ("description", "test body"),
            ("status", "open"),
            ("priority", "medium"),
            ("effort", ""),
            ("assignee_id", ""),
            ("planned_start_at", planned_start_at),
            ("planned_end_at", ""),
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
    peisear_storage::user_capacities::insert(&app.db, user_id, points, None, None, None)
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
