//! `HLT-002` (RFC 008 §5, `NFR-A11Y-003`) — chart equivalence. Both
//! sprint charts gain a two-to-three sentence textual summary and a
//! `<details>`-hidden `<table>` of the exact plotted values, and the
//! velocity chart's accessible name is rewritten to describe its data
//! rather than its type.
//!
//! Both additions inherit `QA-017`'s (`NFR-PRIV-007`) suppression:
//! the burndown's summary and table are built inside `render_burndown`
//! itself, so they cannot render without the chart, which is already
//! gated by `show_trajectory`; the velocity table keeps its bars
//! unconditionally but loses its median row under the same
//! `show_median` predicate the reference line uses. Checks 3, 4, 5
//! below are the privacy tests HLT-002 §7 asks to be written first and
//! planted — see the round-2 review package for both plant
//! transcripts.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_planned_sprint, create_team_project, create_team_with_admin};
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::issues::IssueFields;

/// Insert a `done` issue with a specific effort value and link it to
/// `sprint_id`. Effort is a parameter (unlike `aggregate_privacy.rs`'s
/// fixed `Some(3)`) so a single fixture can produce distinguishable
/// per-row table values.
async fn insert_done_issue(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    sprint_id: &str,
    assignee_id: &str,
    title: &str,
    effort: i64,
) {
    let id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &id,
        project_id,
        author_id,
        IssueFields {
            title,
            description: "Test issue body.",
            status: IssueStatus::Done,
            priority: Priority::Medium,
            effort: Some(effort),
            assignee_id: Some(assignee_id),
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert done issue");
    peisear_storage::sprints::add_issue(&app.db, sprint_id, &id)
        .await
        .expect("add issue to sprint");
}

/// Team with Alice (admin) and Bob (member), a team-scoped project,
/// and an active sprint. `create_planned_sprint` always sets
/// `starts_on = today`, so the burndown window is always exactly one
/// day (today) -- convenient for asserting on a single, known row.
/// Returns `(app, project_id, sprint_id, alice_id, bob_id)`, logged
/// in as Alice.
async fn team_with_active_sprint() -> (TestApp, String, String, String, String) {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&app, &bob).await;
    common::auth::logout(&app).await;
    common::auth::login(&app, &alice).await;

    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    peisear_storage::teams::add_member(&app.db, &team_id, &bob_id, TeamRole::Member)
        .await
        .expect("add bob");
    let project_id = create_team_project(&app.db, &alice_id, &team_id, "Project").await;
    let sprint_id = create_planned_sprint(&app.db, &team_id, "Sprint 1").await;
    peisear_storage::sprints::start(&app.db, &sprint_id)
        .await
        .expect("start sprint");

    (app, project_id, sprint_id, alice_id, bob_id)
}

/// Two completed sprints, solo-owned by `owner_id`, with the given
/// per-sprint completed-point effort values -- the one-contributor
/// velocity fixture, generalised over effort so table cells are
/// individually identifiable.
async fn two_solo_completed_sprints(
    app: &TestApp,
    team_id: &str,
    project_id: &str,
    owner_id: &str,
    efforts: [i64; 2],
) {
    for (i, effort) in efforts.into_iter().enumerate() {
        let sprint_id = create_planned_sprint(&app.db, team_id, &format!("Sprint {i}")).await;
        peisear_storage::sprints::start(&app.db, &sprint_id)
            .await
            .expect("start sprint");
        insert_done_issue(
            app,
            project_id,
            owner_id,
            &sprint_id,
            owner_id,
            "Solo work",
            effort,
        )
        .await;
        peisear_storage::sprints::complete(&app.db, &sprint_id)
            .await
            .expect("complete sprint");
    }
}

/// Check 1 and 2: two contributors -- the burndown's table renders
/// beside its card, and its one row's cells equal the values the
/// chart plots. `create_planned_sprint`'s window is exactly today, so
/// the table has exactly one row; its full ISO date (`%Y-%m-%d`) is
/// distinctive -- the chart's own axis labels only ever render the
/// short `%m-%d` form, so finding the long form proves the table, not
/// the chart, produced it.
#[tokio::test]
async fn two_contributors_burndown_table_renders_with_matching_cells() {
    let (app, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A", 4).await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B", 4).await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let today_iso = chrono::Utc::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    assert!(
        body.contains("cumulative"),
        "two contributors should still render the burndown chart: {body}"
    );
    assert!(
        body.contains("Burndown values"),
        "the burndown's table (aria-label \"Burndown values\") must render \
         beside the chart it belongs to: {body}"
    );
    assert!(
        body.contains(&today_iso),
        "the table's one row must show today's full date -- the chart's own \
         axis labels never render the long form, so this proves the table \
         rendered its own row rather than the chart's date labels leaking \
         through: {body}"
    );
    // Scoped to the "Burndown values" table and counted, not a bare
    // `contains(">8<")` -- both facts were checked independently. Two
    // real gaps found by planting (`TT-003` §5): (1) `contains` only
    // proves ">8<" appears *somewhere*, so breaking the completed
    // cell's own value (while committed stayed correct) still left the
    // check passing; (2) the burndown SVG's y-axis tick labels are
    // `<text>` elements on the same page that can independently render
    // the literal text "8", so even a table-wide count needs to be
    // scoped to the table itself, not the whole body.
    let table_start = body
        .find(r#"aria-label="Burndown values""#)
        .expect("Burndown values table present");
    let table_end = body[table_start..]
        .find("</table>")
        .map(|i| table_start + i)
        .expect("Burndown values table has a closing </table>");
    let table_markup = &body[table_start..table_end];
    let eight_count = table_markup.matches(">8<").count();
    assert_eq!(
        eight_count, 2,
        "both issues are effort 4, so the table's committed and completed \
         cells for today should both read 8 (found {eight_count} in the \
         table, not the whole page); table: {table_markup}"
    );
}

/// Check 3 (privacy, written first): one contributor -- the
/// burndown's table must be absent along with the chart itself, not
/// just visually hidden. Planted (see round-2 review package) by
/// temporarily removing `show_trajectory` from `burndown_card`'s
/// gate, reproducing the exact defect this test exists to catch.
#[tokio::test]
async fn one_contributor_burndown_table_is_absent_with_the_chart() {
    let (app, project_id, sprint_id, alice_id, _bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A", 4).await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "B", 4).await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        !body.contains("cumulative"),
        "one contributor must not render the burndown chart: {body}"
    );
    assert!(
        !body.contains("Burndown values"),
        "one contributor must not render the burndown's table either -- a \
         table of the same numbers the chart withholds would disclose \
         exactly what the suppression is for: {body}"
    );
}

/// Check 4 (privacy, written first): one contributor across the
/// velocity window -- the table renders (it keeps its bars, §3.2),
/// but has no median row. Planted by temporarily removing
/// `show_median` from the median row's gate.
#[tokio::test]
async fn one_contributor_velocity_table_renders_with_no_median_row() {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    let project_id = create_team_project(&app.db, &alice_id, &team_id, "Project").await;
    two_solo_completed_sprints(&app, &team_id, &project_id, &alice_id, [4, 8]).await;

    let resp = app.server.get("/teams/engineering/sprints").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("<rect"),
        "solo velocity bars must still render: {body}"
    );
    assert!(
        !body.contains("stroke-dasharray"),
        "one contributor across the whole window must not render the median line: {body}"
    );
    assert!(
        body.contains("Completed sprint values"),
        "the velocity table (aria-label \"Completed sprint values\") must \
         still render -- unlike the burndown, the velocity chart and its \
         table are never suppressed as a whole: {body}"
    );
    assert!(
        !body.contains(">Median<"),
        "the table's median row must be absent when the reference line is: {body}"
    );
}

/// Check 5 (privacy, written first): one contributor -- the summary's
/// completed-points sentence still renders (it discloses nothing the
/// bars don't already show), but no sentence states the median.
/// Planted by temporarily removing `show_median` from the summary's
/// median clause.
#[tokio::test]
async fn one_contributor_summary_has_no_median_sentence() {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    let project_id = create_team_project(&app.db, &alice_id, &team_id, "Project").await;
    two_solo_completed_sprints(&app, &team_id, &project_id, &alice_id, [4, 8]).await;

    let resp = app.server.get("/teams/engineering/sprints").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("Completed points across the last"),
        "the points-list sentence must still render -- it states only what \
         the bars already show: {body}"
    );
    assert!(
        !body.contains("The median is"),
        "no sentence may state the median once its line and table row are \
         both suppressed: {body}"
    );
}

/// Check 6: the bar chart's accessible name describes its data (a
/// completed-points range), not its type. The old static label named
/// the chart type only ("Bar chart of recent sprint outcomes") --
/// `NFR-A11Y-003` asks for a summary, and an accessible name is not
/// one.
#[tokio::test]
async fn bar_chart_accessible_name_describes_data_not_chart_type() {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    let project_id = create_team_project(&app.db, &alice_id, &team_id, "Project").await;
    two_solo_completed_sprints(&app, &team_id, &project_id, &alice_id, [4, 8]).await;

    let resp = app.server.get("/teams/engineering/sprints").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        !body.contains("Bar chart of recent sprint outcomes"),
        "the old chart-type-only label must be gone: {body}"
    );
    assert!(
        body.contains("completed points from 4 to 8"),
        "the new label must describe the data's own range: {body}"
    );
}
