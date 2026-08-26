//! `QA-019` (RFC 005 §8, `NFR-CONC-003`) — `updated_at` has exactly
//! one authority: database triggers, not application code. `0017`
//! adds triggers for `issues`, `projects`, and `user_view_states`,
//! the three tables `QA-018`'s audit found the application still
//! writing directly; the four application `SET`/`VALUES` clauses
//! that used to do it are gone.
//!
//! Checks 1-3 confirm the new triggers actually fire: an `UPDATE`
//! that does not touch `updated_at` in its own `SET` clause (true of
//! every call site now, since none of them write it) still advances
//! the column. Check 4 confirms the migration didn't disturb the
//! four tables `0014` already covered. Check 5 — "the lock's own
//! behaviour survives the change of authority" — is **not**
//! duplicated here: it's `optimistic_lock.rs`'s existing
//! `issue_update_with_stale_timestamp_returns_409` and
//! `project_update_with_stale_timestamp_returns_409`, both of which
//! pass **unmodified** by this handoff, per its own requirement.

mod common;

use chrono::{DateTime, NaiveDate, Utc};
use common::auth::{TestUser, register_and_login};
use common::fixture::{
    create_issue, create_personal_project, create_planned_sprint, create_team_with_admin,
};
use common::server::{TestApp, ensure_distinct_timestamp};
use peisear_core::{IssueStatus, Priority};
use peisear_storage::issues::IssueFields;
use peisear_storage::{issues, projects, sprints, view_states};

async fn read_updated_at(app: &TestApp, table: &str, id_column: &str, id: &str) -> DateTime<Utc> {
    let query = format!("SELECT updated_at FROM {table} WHERE {id_column} = ?1");
    let (updated_at,): (DateTime<Utc>,) = sqlx::query_as(&query)
        .bind(id)
        .fetch_one(&app.db)
        .await
        .expect("read updated_at");
    updated_at
}

/// Check 1: `issues` — an `UPDATE` that does not touch `updated_at`
/// (true of every call site post-`0017`) still advances it.
#[tokio::test]
async fn issues_update_advances_updated_at_via_trigger() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Project").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "Original title").await;

    let before = read_updated_at(&app, "issues", "id", &issue_id).await;
    ensure_distinct_timestamp().await;

    issues::update(
        &app.db,
        &issue_id,
        &project_id,
        &user_id,
        IssueFields {
            title: "Renamed title",
            description: "",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("update issue");

    let after = read_updated_at(&app, "issues", "id", &issue_id).await;
    assert!(
        after > before,
        "issues_updated_at trigger must advance updated_at: before={before}, after={after}"
    );
}

/// Check 2: `projects` — same.
#[tokio::test]
async fn projects_update_advances_updated_at_via_trigger() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Original name").await;

    let before = read_updated_at(&app, "projects", "id", &project_id).await;
    ensure_distinct_timestamp().await;

    projects::update(&app.db, &project_id, &user_id, "Renamed", "New description")
        .await
        .expect("update project");

    let after = read_updated_at(&app, "projects", "id", &project_id).await;
    assert!(
        after > before,
        "projects_updated_at trigger must advance updated_at: before={before}, after={after}"
    );
}

/// Check 3: `user_view_states` — same. Not lock-participating
/// (`QA-019` §4), so this is the uniformity property, not a safety
/// one — but the requirement has no carve-out, and the trigger must
/// still fire.
#[tokio::test]
async fn user_view_states_upsert_advances_updated_at_via_trigger() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let view_key = "project_issues:some-project";

    view_states::upsert(&app.db, &user_id, view_key, r#"{"status":"open"}"#)
        .await
        .expect("initial upsert");
    let before = read_updated_at(&app, "user_view_states", "user_id", &user_id).await;
    ensure_distinct_timestamp().await;

    view_states::upsert(&app.db, &user_id, view_key, r#"{"status":"done"}"#)
        .await
        .expect("second upsert");

    let after = read_updated_at(&app, "user_view_states", "user_id", &user_id).await;
    assert!(
        after > before,
        "user_view_states_updated_at trigger must advance updated_at: before={before}, after={after}"
    );
}

/// Check 4: the four tables `0014` already covered
/// (`sprints`/`teams`/`team_memberships`/`user_capacities`) are
/// unaffected by `0017`. One representative is enough to prove
/// `0017` didn't disturb the pre-existing trigger machinery (a name
/// collision, a syntax error elsewhere in the same migration file
/// aborting the whole transaction, etc.) — the other three are
/// exercised the same way by the existing, unmodified
/// `optimistic_lock` suite (`sprint_edit_with_stale_timestamp_
/// returns_409` and friends), which still passes.
#[tokio::test]
async fn sprints_update_still_advances_updated_at_via_pre_existing_trigger() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &user_id, "Engineering").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let before = read_updated_at(&app, "sprints", "id", &sprint_id).await;
    ensure_distinct_timestamp().await;

    sprints::update(
        &app.db,
        &sprint_id,
        "Sprint 1 (renamed)",
        None,
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 1, 14).unwrap(),
    )
    .await
    .expect("update sprint");

    let after = read_updated_at(&app, "sprints", "id", &sprint_id).await;
    assert!(
        after > before,
        "the pre-existing sprints_updated_at trigger (0014) must still fire \
         after 0017 is applied: before={before}, after={after}"
    );
}
