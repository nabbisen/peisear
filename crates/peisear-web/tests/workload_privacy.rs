//! DEV-003 (RFC 007) — capacity privacy on workload chips.
//!
//! `NFR-PRIV-001` (P0) lists capacity and WIP limit as visible only
//! to their subject. `WorkloadStrip` (project detail) and
//! `WorkloadHint` (issue create/edit) used to render another
//! member's capacity value, over-capacity annotation, and a
//! capacity-derived danger badge — all fixed to show only in-flight
//! load, which `NFR-PRIV-002` permits sharing.
//!
//! ## Fixture shape (per the `ISSUE-003` ruling)
//!
//! `project_workload` (the sole data source for these three
//! surfaces) **used to** return only the **project owner's** row —
//! it joined on `p.owner_id = u.id`, so a team member who wasn't the
//! owner never appeared at all. `TEAM-001` (RFC 009) fixed that: the
//! result now includes every assignee candidate (owner plus the
//! team's `admin`/`member`s), which is a separate, independently
//! reviewed change — this suite's own assertions do not depend on
//! the row count, so they hold before and after. The disclosure this
//! suite guards is: a team member who is *not* the project's owner,
//! but who has access to the project via team membership, must not
//! see the owner's capacity when viewing that project — while the
//! owner's in-flight load (a permitted signal, `DEV-003`/`ISSUE-003`
//! — ruled to apply "regardless of how many members a surface
//! lists") remains visible. Fixture below still asserts against
//! Bob's row specifically; it does not assert Alice's own (now also
//! visible, zero-valued) row absent or present either way.
//!
//! Fixture: Alice (team member, viewer) and Bob (team member,
//! project owner). Bob sets a 5pt capacity and is assigned a 10pt
//! issue in his own team-scoped project — over capacity. Alice
//! views the project, the issue-create form, and the issue-edit
//! form. Bob's own `/today` and `/settings` are also checked, as a
//! guard against over-correction.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, login, logout, register_and_login};
use common::fixture::create_team_with_admin;
use common::server::TestApp;

/// Shared fixture: a team with Alice (admin) and Bob (member), a
/// team-scoped project owned by Bob, a 5pt capacity for Bob, and
/// one issue in that project assigned to Bob with 10pt effort
/// (over capacity). Leaves Alice logged in. Returns
/// `(project_id, issue_id, bob_id, bob)` — `bob` (the `TestUser`
/// credentials) is returned rather than reconstructed via
/// `TestUser::new("bob")`, which would mint a *different* random
/// user; callers that need to log back in as Bob must reuse this
/// value.
async fn over_capacity_owner_fixture(app: &TestApp) -> (String, String, String, TestUser) {
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(app, &alice).await;

    let bob = TestUser::new("bob");
    let bob_id = register_and_login(app, &bob).await;
    logout(app).await;
    login(app, &alice).await;

    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    peisear_storage::teams::add_member(
        &app.db,
        &team_id,
        &bob_id,
        peisear_core::teams::TeamRole::Member,
    )
    .await
    .expect("add bob to team");

    // Bob's capacity: 5pt. Self-only per NFR-PRIV-001.
    peisear_storage::user_capacities::insert(&app.db, &bob_id, 5, None, None, None)
        .await
        .expect("insert bob's capacity");

    // A team-scoped project owned by Bob. Alice reaches it via
    // team membership, not ownership.
    let project_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::projects::insert(
        &app.db,
        &project_id,
        &bob_id,
        "Team Project",
        "",
        Some(&team_id),
    )
    .await
    .expect("insert team project");

    // One issue, assigned to Bob, effort 10 — over his 5pt capacity.
    let issue_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &issue_id,
        &project_id,
        &bob_id,
        peisear_storage::issues::IssueFields {
            title: "Bob's overloaded issue",
            description: "Test issue body.",
            status: peisear_core::IssueStatus::Open,
            priority: peisear_core::Priority::Medium,
            effort: Some(10),
            assignee_id: Some(&bob_id),
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert issue assigned to bob");

    // Fixture setup runs as Alice; she stays logged in for the
    // caller's requests.
    (project_id, issue_id, bob_id, bob)
}

fn assert_no_capacity_leak(body: &str, context: &str) {
    assert!(
        !body.contains("5/5 pt") && !body.contains("5 / 5 pt") && !body.contains("/5 pt"),
        "{context}: must not render Bob's capacity denominator; body: {body}"
    );
    assert!(
        !body.to_lowercase().contains("over capacity"),
        "{context}: must not disclose Bob's over-capacity state"
    );
    assert!(
        !body.to_lowercase().contains("strained"),
        "{context}: must not disclose Bob's strained state"
    );
    assert!(
        !body.contains("badge-error"),
        "{context}: must not use danger colouring derived from Bob's capacity"
    );
}

#[tokio::test]
async fn project_detail_does_not_disclose_owners_capacity() {
    let app = TestApp::spawn().await;
    let (project_id, _issue_id, bob_id, _bob) = over_capacity_owner_fixture(&app).await;

    let url = format!("/projects/{project_id}");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert_no_capacity_leak(&body, "project detail");

    // The permitted signal — Bob's in-flight load — remains
    // visible regardless of how many other rows TEAM-001 added
    // to the strip (`DEV-003`/`ISSUE-003` ruling); his 10pt
    // in-flight load must still appear.
    assert!(
        body.contains("Workload"),
        "the workload strip must still render — it has a real signal to show \
         (Bob's in-flight load), not nothing; body: {body}"
    );
    assert!(
        body.contains("10 pt"),
        "Bob's in-flight point total must remain visible; body: {body}"
    );
    let _ = bob_id;
}

#[tokio::test]
async fn issue_create_form_does_not_disclose_owners_capacity() {
    let app = TestApp::spawn().await;
    let (project_id, _issue_id, _bob_id, _bob) = over_capacity_owner_fixture(&app).await;

    let url = format!("/projects/{project_id}/issues/new");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert_no_capacity_leak(&body, "issue create form");
}

#[tokio::test]
async fn issue_edit_form_does_not_disclose_owners_capacity() {
    let app = TestApp::spawn().await;
    let (project_id, issue_id, _bob_id, _bob) = over_capacity_owner_fixture(&app).await;

    let url = format!("/projects/{project_id}/issues/{issue_id}/edit");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert_no_capacity_leak(&body, "issue edit form");
}

#[tokio::test]
async fn subjects_own_today_and_settings_still_show_capacity() {
    // Guard against over-correction (handoff §7 test 5): the
    // subject seeing their own capacity is correct and required
    // (FR-PER-002, FR-PER-003) and must be unaffected by the
    // fix on the shared surfaces above.
    let app = TestApp::spawn().await;
    let (_project_id, _issue_id, _bob_id, bob) = over_capacity_owner_fixture(&app).await;

    // Fixture leaves Alice logged in; switch to Bob to check his
    // own views.
    logout(&app).await;
    login(&app, &bob).await;

    // Not a bare `contains("5") && contains("pt")` -- every page loads
    // Tailwind from a CDN URL containing "3.4.15" (`components/layout.rs`),
    // which already satisfies both halves regardless of whether the
    // capacity value itself rendered at all (`TT-003` §5, confirmed by
    // planting: hardcoding the rendered capacity to 0 left this check
    // passing). `" / 5 pt"` is `LoadWithCapacityValue`'s own literal
    // format (`en.rs`: `"{in_flight}/{capacity} pt"`) with the known
    // fixture capacity, not a bare digit.
    let resp = app.server.get("/today").await;
    resp.assert_status(StatusCode::OK);
    let today_body = resp.text();
    assert!(
        today_body.contains(" / 5 pt"),
        "Bob's own /today must still show his capacity; body: {today_body}"
    );

    let resp = app.server.get("/settings").await;
    resp.assert_status(StatusCode::OK);
    let settings_body = resp.text();
    // `CapacityRowAriaLabel`'s own literal format
    // (`en.rs`: `"Capacity {points} points, period {from} to {to}."`),
    // not a bare digit -- same reasoning as the /today check above.
    assert!(
        settings_body.contains("Capacity 5 points"),
        "Bob's own /settings must still show his capacity value; body: {settings_body}"
    );
}
