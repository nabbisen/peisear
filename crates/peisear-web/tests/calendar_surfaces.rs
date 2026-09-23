//! `CAL-002` (RFC 002): the two calendar surfaces. `CAL-001` created
//! `tests/calendar.rs` for the storage-level (migration/trigger/
//! window-query) tests, including RFC 002's own test-plan item 1
//! (`migration_0016_adds_planned_columns_and_existing_rows_are_null`)
//! — handoff §2.1: do not re-add it here. This file is route-shaped:
//! every test drives a real HTTP request through the router.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_personal_project, create_team_project, create_team_with_admin};
use common::server::TestApp;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::{issues, sprints};

fn today() -> chrono::NaiveDate {
    chrono::Utc::now().date_naive()
}

fn utc_hms(d: chrono::NaiveDate, h: u32, m: u32) -> chrono::DateTime<chrono::Utc> {
    d.and_hms_opt(h, m, 0).unwrap().and_utc()
}

#[allow(clippy::too_many_arguments)]
async fn insert_planned_issue(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    title: &str,
    assignee_id: Option<&str>,
    planned_start_at: Option<chrono::DateTime<chrono::Utc>>,
    planned_end_at: Option<chrono::DateTime<chrono::Utc>>,
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
            priority: Priority::Medium,
            effort: None,
            assignee_id,
            planned_start_at,
            planned_end_at,
        },
    )
    .await
    .expect("insert planned issue");
    id
}

/// Test 1 -- `/today/calendar` renders only the viewer's assigned
/// issues.
#[tokio::test]
async fn personal_calendar_renders_only_my_issues() {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let project_id = create_personal_project(&app.db, &alice_id, "Test").await;

    let bob_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(&app.db, &bob_id, "bob@example.com", "x", "Bob")
        .await
        .expect("insert bob");

    let d = today();
    insert_planned_issue(
        &app,
        &project_id,
        &alice_id,
        "Alice's item",
        Some(&alice_id),
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;
    insert_planned_issue(
        &app,
        &project_id,
        &alice_id,
        "Bob's item",
        Some(&bob_id),
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;

    let resp = app.server.get("/today/calendar?view=day").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(body.contains("Alice's item"));
    assert!(
        !body.contains("Bob's item"),
        "another user's issue must not appear: {body}"
    );
}

/// Test 2 -- `/projects/{id}/calendar` requires project read access.
#[tokio::test]
async fn project_calendar_requires_access() {
    let app = TestApp::spawn().await;
    let owner = TestUser::new("alice");
    let owner_id = register_and_login(&app, &owner).await;
    let project_id = create_personal_project(&app.db, &owner_id, "Test").await;

    let stranger = TestUser::new("stranger");
    register_and_login(&app, &stranger).await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar"))
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::NOT_FOUND,
        "a non-member must get exactly what find_accessible gives elsewhere (404), got {}",
        resp.status_code()
    );
}

/// Test 3 -- each `?view=` renders; an unknown value falls back to
/// week rather than erroring.
#[tokio::test]
async fn view_param_renders_and_falls_back_to_week() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    for view in ["day", "week", "month", "not-a-real-view"] {
        let resp = app
            .server
            .get(&format!("/today/calendar?view={view}"))
            .await;
        assert_eq!(
            resp.status_code(),
            StatusCode::OK,
            "?view={view} must render 200, got {}",
            resp.status_code()
        );
    }

    let resp = app.server.get("/today/calendar?view=not-a-real-view").await;
    let body = resp.text();
    assert!(
        body.contains(">Week</a>") || body.contains("btn-primary\">Week"),
        "an unknown view value must fall back to week: {body}"
    );
}

/// Test 4 -- `?date=` anchors the window; prev/next links carry the
/// adjusted date and the current view.
#[tokio::test]
async fn date_param_anchors_window_and_nav_links_advance() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app
        .server
        .get("/today/calendar?view=day&date=2026-06-15")
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains("date=2026-06-14") && body.contains("date=2026-06-16"),
        "day view's prev/next links must be the anchor date ±1 day, carrying view=day: {body}"
    );
    assert!(
        body.contains("view=day"),
        "nav links must preserve the current view: {body}"
    );
}

/// Test 5 -- a `planned_end_at IS NULL` issue appears as an anchor
/// block, not dropped.
#[tokio::test]
async fn half_open_issue_renders_as_anchor_block() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let d = today();
    insert_planned_issue(
        &app,
        &project_id,
        &user_id,
        "Anchor only",
        Some(&user_id),
        Some(utc_hms(d, 9, 0)),
        None,
    )
    .await;

    let resp = app.server.get("/today/calendar?view=day").await;
    let body = resp.text();
    assert!(
        body.contains("Anchor only"),
        "a start-only issue must still render: {body}"
    );
}

/// Test 6 -- the sprint band renders on the project axis for an
/// overlapping **active** sprint, and not for a planned or completed
/// one.
#[tokio::test]
async fn sprint_band_only_for_active_sprint() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;

    let d = today();
    let starts = d - chrono::Duration::days(2);
    let ends = d + chrono::Duration::days(5);
    let sprint_id = sprints::insert(&app.db, &team_id, "Sprint 1", None, starts, ends)
        .await
        .expect("insert sprint");

    // Planned: no band yet.
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar"))
        .await;
    let body = resp.text();
    assert!(
        !body.contains("Sprint 1"),
        "a planned sprint must not show a band: {body}"
    );

    // Active: band appears.
    sprints::start(&app.db, &sprint_id)
        .await
        .expect("start sprint");
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar"))
        .await;
    let body = resp.text();
    assert!(
        body.contains("Sprint 1"),
        "an active, overlapping sprint must show a band: {body}"
    );

    // Completed: band disappears again.
    sprints::complete(&app.db, &sprint_id)
        .await
        .expect("complete sprint");
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar"))
        .await;
    let body = resp.text();
    assert!(
        !body.contains("Sprint 1"),
        "a completed sprint must not show a band: {body}"
    );
}

/// Test 7 -- §2.4: a project calendar with two users' issues
/// contains neither display name. The viewer is a third user
/// (assigned nothing), since the viewer's own name legitimately
/// appears in the navbar chrome on every page — this isolates the
/// assertion to per-block disclosure specifically.
#[tokio::test]
async fn project_calendar_never_names_the_assignee() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("carol");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;

    let alice = TestUser::new("alice");
    let alice_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(&app.db, &alice_id, &alice.email, "x", "Alice")
        .await
        .expect("insert alice");
    let bob_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::users::insert(&app.db, &bob_id, "bob@example.com", "x", "Bob")
        .await
        .expect("insert bob");

    // Titles deliberately don't embed either assignee's name --
    // otherwise the assertion below would trip on legitimate title
    // text rather than on an actual assignee-name disclosure.
    let d = today();
    insert_planned_issue(
        &app,
        &project_id,
        &admin_id,
        "First scheduled item",
        Some(&alice_id),
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;
    insert_planned_issue(
        &app,
        &project_id,
        &admin_id,
        "Second scheduled item",
        Some(&bob_id),
        Some(utc_hms(d, 11, 0)),
        Some(utc_hms(d, 12, 0)),
    )
    .await;

    // carol (admin, the current session) is not assigned either
    // issue and never appears as an assignee anywhere.
    let resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar?view=day"))
        .await;
    let body = resp.text();
    assert!(body.contains("First scheduled item") && body.contains("Second scheduled item"));
    assert!(
        !body.contains("Alice"),
        "no assignee display name may appear: {body}"
    );
    assert!(
        !body.contains("Bob"),
        "no assignee display name may appear: {body}"
    );
}

/// Test 8 -- §16.6: no percentage, no "of", no ratio between a count
/// and the threshold; the crowding chip, if present, carries only
/// its state word. This is the test that "will look pointless until
/// someone adds a helpful number" (handoff §5) -- it fails the moment
/// one is.
#[tokio::test]
async fn no_efficiency_metric_and_crowding_chip_carries_no_quantity() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    // Five overlapping blocks on one day -- one more than
    // CROWDING_WATCH_THRESHOLD (4), so the chip should appear.
    let d = today();
    for i in 0..5 {
        insert_planned_issue(
            &app,
            &project_id,
            &user_id,
            &format!("Crowded {i}"),
            Some(&user_id),
            Some(utc_hms(d, 9, 0)),
            Some(utc_hms(d, 10, 0)),
        )
        .await;
    }

    let resp = app.server.get("/today/calendar?view=month").await;
    let body = resp.text();
    assert!(
        body.contains("badge-warning"),
        "five overlapping blocks must trip the Watch crowding chip: {body}"
    );
    // No quantity anywhere on the page: no percentage sign, no "of "
    // (as in "5 of 4"), no literal threshold/count pairing.
    assert!(!body.contains('%'), "no percentage anywhere: {body}");
    assert!(
        !body.to_lowercase().contains(" of "),
        "no ratio phrasing: {body}"
    );
    assert!(
        !body.contains("5/4") && !body.contains("5 / 4"),
        "no count-vs-threshold ratio: {body}"
    );
}

/// Test 9 -- both footers render, byte-identical to their keys.
#[tokio::test]
async fn both_footers_render_byte_identically() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;

    let resp = app.server.get("/today/calendar").await;
    let body = resp.text();
    assert!(
        body.contains(
            &peisear_i18n::Locale::English
                .render(peisear_i18n::MessageKey::PersonalCalendarPrivacyFootnote)
        ),
        "personal footer missing or altered: {body}"
    );

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar"))
        .await;
    let body = resp.text();
    assert!(
        body.contains(
            &peisear_i18n::Locale::English
                .render(peisear_i18n::MessageKey::ProjectCalendarPrivacyFootnote)
        ),
        "project footer missing or altered: {body}"
    );
}

/// Test 10 -- sub-issues appear on neither axis. Also the regression
/// guard for the `planned_for_user` gap found while implementing this
/// test (see `peisear-storage/src/issues.rs`'s doc comment on that
/// function) -- demonstrated failing on the personal axis with the
/// fix reverted, per this project's established discipline.
#[tokio::test]
async fn sub_issues_appear_on_neither_axis() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;

    let d = today();
    let parent_id = insert_planned_issue(
        &app,
        &project_id,
        &admin_id,
        "Parent item",
        Some(&admin_id),
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;

    let sub_id = uuid::Uuid::new_v4().to_string();
    issues::insert_sub_issue(
        &app.db,
        &sub_id,
        &project_id,
        &parent_id,
        &admin_id,
        "Sub item",
        "Test sub-issue body.",
        IssueStatus::Open,
        Priority::Medium,
        None,
        Some(&admin_id),
    )
    .await
    .expect("insert sub-issue");
    // The trigger only fires on planned_start_at/planned_end_at
    // columns, which insert_sub_issue's fixed column list doesn't
    // set -- give the sub-issue a plan date via the same update path
    // the edit form uses, so it has one to (wrongly) appear by.
    issues::update(
        &app.db,
        &sub_id,
        &project_id,
        &admin_id,
        issues::IssueFields {
            title: "Sub item",
            description: "",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: Some(&admin_id),
            planned_start_at: Some(utc_hms(d, 9, 0)),
            planned_end_at: Some(utc_hms(d, 10, 0)),
        },
    )
    .await
    .expect("give the sub-issue a plan date");

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar?view=day"))
        .await;
    let body = resp.text();
    assert!(body.contains("Parent item"));
    assert!(
        !body.contains("Sub item"),
        "a sub-issue must not appear on the project axis: {body}"
    );

    let resp = app.server.get("/today/calendar?view=day").await;
    let body = resp.text();
    assert!(body.contains("Parent item"));
    assert!(
        !body.contains("Sub item"),
        "a sub-issue must not appear on the personal axis: {body}"
    );
}

// ──────────────────────────────────────────────────────────────
// `CAL-003` (RFC 004d D-3) — drag a day-view block to reschedule it
// ──────────────────────────────────────────────────────────────

/// The `data-updated-at` attribute's value on the one day-view block
/// whose `data-issue-id` is `issue_id` — scoped to that block's own
/// wrapper `<div ...>` tag, same discipline
/// `board_card_lock_value.rs`'s `card_updated_at` uses (`§10.17`'s
/// lesson: not a whole-body search).
fn block_attr<'a>(body: &'a str, issue_id: &str, attr: &str, label: &str) -> &'a str {
    let id_marker = format!(r#"data-issue-id="{issue_id}""#);
    let id_pos = body.find(&id_marker).unwrap_or_else(|| {
        panic!("no block for {label:?} (issue {issue_id}) found in body: {body}")
    });
    let tag_start = body[..id_pos]
        .rfind("<div")
        .unwrap_or_else(|| panic!("no <div precedes data-issue-id for {label:?}: {body}"));
    let tag_end = body[tag_start..]
        .find('>')
        .map(|e| tag_start + e)
        .unwrap_or_else(|| panic!("block <div> for {label:?} has no closing '>': {body}"));
    let tag = &body[tag_start..tag_end];
    let marker = format!(r#"{attr}=""#);
    let attr_start = tag.find(&marker).unwrap_or_else(|| {
        panic!("block for {label:?} (issue {issue_id}) carries no {attr} attribute at all: {tag}")
    });
    let value_start = attr_start + marker.len();
    let value_end = tag[value_start..]
        .find('"')
        .map(|e| value_start + e)
        .unwrap_or_else(|| panic!("{attr} attribute unterminated on {label:?}'s block: {tag}"));
    &tag[value_start..value_end]
}

/// `LOCK-001`'s guarantee (`DEC-052`), extended per handoff §4.3 to
/// the day view's own drag identity: every block renders a non-empty
/// `data-updated-at`, the optimistic lock `calendar.js` participates
/// in. Two blocks, same shape `board_card_lock_value.rs` uses (a
/// one-block page can't distinguish "every block" from "a block").
#[tokio::test]
async fn every_day_view_block_carries_a_non_empty_lock_value() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let project_id = create_personal_project(&app.db, &admin_id, "Lock value fixture").await;
    let d = today();

    let first_id = insert_planned_issue(
        &app,
        &project_id,
        &admin_id,
        "First block",
        None,
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;
    let second_id = insert_planned_issue(
        &app,
        &project_id,
        &admin_id,
        "Second block",
        None,
        Some(utc_hms(d, 13, 0)),
        Some(utc_hms(d, 14, 0)),
    )
    .await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar?view=day"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    for (issue_id, label) in [(&first_id, "First block"), (&second_id, "Second block")] {
        let value = block_attr(&body, issue_id, "data-updated-at", label);
        assert!(
            !value.is_empty(),
            "the {label:?} block (issue {issue_id}) carries an empty data-updated-at -- \
             calendar.js has no way to send a lock value it never received: {body}"
        );
    }
}

/// §4.1: a stale `client_updated_at` against `/schedule` returns the
/// conflict status through `check_optimistic_lock`, the same shared
/// function every mutation path uses — not a hand-written 409, and no
/// second lock check introduced for this endpoint.
#[tokio::test]
async fn change_schedule_rejects_a_stale_lock() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let project_id = create_personal_project(&app.db, &admin_id, "P").await;
    let d = today();
    let issue_id = insert_planned_issue(
        &app,
        &project_id,
        &admin_id,
        "Stale lock fixture",
        None,
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;

    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{issue_id}/schedule"
        ))
        .json(&serde_json::json!({
            "planned_start_at": format!("{}T09:30", d.format("%Y-%m-%d")),
            "planned_end_at": format!("{}T10:30", d.format("%Y-%m-%d")),
            "client_updated_at": "1970-01-01T00:00:00Z",
        }))
        .await;
    resp.assert_status(StatusCode::CONFLICT);

    let unchanged = issues::find(&app.db, &issue_id, &project_id)
        .await
        .expect("find issue")
        .planned_start_at
        .expect("still planned");
    assert_eq!(
        unchanged,
        utc_hms(d, 9, 0),
        "a rejected reschedule must not have moved the issue"
    );
}

/// §4.1's success case: both timestamps move by the same delta, the
/// response carries the new lock value, and `time_label`/
/// `announcement` are both present and non-empty.
#[tokio::test]
async fn change_schedule_moves_both_timestamps_and_returns_the_new_lock() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let project_id = create_personal_project(&app.db, &admin_id, "P").await;
    let d = today();
    let issue_id = insert_planned_issue(
        &app,
        &project_id,
        &admin_id,
        "Reschedule fixture",
        None,
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;

    let before = issues::find(&app.db, &issue_id, &project_id)
        .await
        .expect("find issue")
        .updated_at;

    let resp = app
        .server
        .post(&format!(
            "/projects/{project_id}/issues/{issue_id}/schedule"
        ))
        .json(&serde_json::json!({
            "planned_start_at": format!("{}T11:15", d.format("%Y-%m-%d")),
            "planned_end_at": format!("{}T12:15", d.format("%Y-%m-%d")),
            "client_updated_at": before.to_rfc3339(),
        }))
        .await;
    resp.assert_status(StatusCode::OK);
    let json: serde_json::Value = resp.json();
    assert!(
        json["updated_at"].as_str().is_some_and(|v| !v.is_empty()),
        "response must carry a non-empty updated_at: {json}"
    );
    assert_eq!(
        json["time_label"].as_str(),
        Some("11:15–12:15"),
        "response must carry the server-formatted time label: {json}"
    );
    assert!(
        json["announcement"]
            .as_str()
            .is_some_and(|a| a.contains("11:15–12:15")),
        "response's announcement must name the new time: {json}"
    );

    let after = issues::find(&app.db, &issue_id, &project_id)
        .await
        .expect("find issue");
    assert_eq!(after.planned_start_at, Some(utc_hms(d, 11, 15)));
    assert_eq!(after.planned_end_at, Some(utc_hms(d, 12, 15)));
    // Not `assert_ne!(after.updated_at, before)` -- `CURRENT_TIMESTAMP`
    // (`0017`'s trigger) is second-resolution, and this test's two
    // writes can land in the same wall-clock second, same shape
    // `json_status_change_returns_the_new_updated_at` (`status_control.rs`)
    // already avoids for the same reason. The response's own value
    // matching what the row actually holds is the meaningful check.
    assert_eq!(
        json["updated_at"].as_str(),
        Some(after.updated_at.to_rfc3339().as_str()),
        "the response's updated_at must equal what the row actually holds"
    );
}

/// §4.1: a forged `project_id` (an issue that actually lives in a
/// different project) is refused the same way the form path already
/// refuses one — `issues::find` is scoped by `(issue_id, project_id)`
/// together, so this needs no defense-in-depth check of its own.
#[tokio::test]
async fn change_schedule_rejects_a_forged_project_id() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let real_project_id = create_personal_project(&app.db, &admin_id, "Real project").await;
    let other_project_id = create_personal_project(&app.db, &admin_id, "Other project").await;
    let d = today();
    let issue_id = insert_planned_issue(
        &app,
        &real_project_id,
        &admin_id,
        "Forged id fixture",
        None,
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;

    let resp = app
        .server
        .post(&format!(
            "/projects/{other_project_id}/issues/{issue_id}/schedule"
        ))
        .json(&serde_json::json!({
            "planned_start_at": format!("{}T09:30", d.format("%Y-%m-%d")),
            "planned_end_at": format!("{}T10:30", d.format("%Y-%m-%d")),
            "client_updated_at": "2020-01-01T00:00:00Z",
        }))
        .await;
    resp.assert_status(StatusCode::NOT_FOUND);

    let unchanged = issues::find(&app.db, &issue_id, &real_project_id)
        .await
        .expect("find issue")
        .planned_start_at
        .expect("still planned");
    assert_eq!(unchanged, utc_hms(d, 9, 0));
}

/// §4.7: `calendar.js` is referenced with `defer` on day view and
/// absent on week/month view — the other two shapes have no
/// `data-issue-id`/`draggable` markers for it to attach to (`CAL-003`
/// §6, mirroring `dm_js_is_served_with_defer_on_both_surfaces`'s
/// negative-case shape).
#[tokio::test]
async fn calendar_js_is_referenced_only_on_day_view() {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let project_id = create_personal_project(&app.db, &admin_id, "P").await;
    let d = today();
    insert_planned_issue(
        &app,
        &project_id,
        &admin_id,
        "Script tag fixture",
        None,
        Some(utc_hms(d, 9, 0)),
        Some(utc_hms(d, 10, 0)),
    )
    .await;

    let day_resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar?view=day"))
        .await;
    let day_body = day_resp.text();
    assert!(
        day_body.contains(r#"<script src="/static/calendar.js" defer"#),
        "day view must reference calendar.js with defer: {day_body}"
    );

    let week_resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar?view=week"))
        .await;
    let week_body = week_resp.text();
    assert!(
        !week_body.contains("/static/calendar.js"),
        "week view must not load calendar.js -- it has nothing there to enhance: {week_body}"
    );

    let month_resp = app
        .server
        .get(&format!("/projects/{project_id}/calendar?view=month"))
        .await;
    let month_body = month_resp.text();
    assert!(
        !month_body.contains("/static/calendar.js"),
        "month view must not load calendar.js either: {month_body}"
    );

    // The personal axis shares the same gate (`view == Day`), so one
    // check there is enough to confirm it isn't a project-axis-only
    // condition.
    let personal_day_resp = app.server.get("/today/calendar?view=day").await;
    let personal_day_body = personal_day_resp.text();
    assert!(
        personal_day_body.contains(r#"<script src="/static/calendar.js" defer"#),
        "the personal axis's day view must also reference calendar.js: {personal_day_body}"
    );
}
