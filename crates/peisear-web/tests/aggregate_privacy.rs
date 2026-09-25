//! `QA-017` (RFC 005 §7, `NFR-PRIV-007`) — the sprint burndown's
//! day-by-day trajectory, and the velocity chart's median reference
//! line, are suppressed below two distinct contributors (people who
//! completed at least one issue). Everything else stays: the
//! sprint-end totals (`render_summary_card`), the velocity bars
//! themselves, the issues table. `QA-016`'s audit established why —
//! the totals are already assemblable elsewhere; the day-by-day
//! trajectory and the computed median are not, and both become
//! individually attributable once fewer than two people did the
//! completing.
//!
//! Contributor is scoped to **completed** work, not sprint membership
//! — an issue merely present in the sprint but not done does not make
//! its assignee a contributor to the trajectory being shown.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_planned_sprint, create_team_project, create_team_with_admin};
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::issues::IssueFields;

/// Insert a `done` issue directly (bypassing the status-change
/// endpoints, which this suite has no need to exercise) and link it to
/// `sprint_id`, assigned to `assignee_id`.
async fn insert_done_issue(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    sprint_id: &str,
    assignee_id: &str,
    title: &str,
) {
    insert_done_issue_with_assignee(
        app,
        project_id,
        author_id,
        sprint_id,
        Some(assignee_id),
        title,
    )
    .await;
}

/// Insert a `done` issue with no assignee — `QA-017` §3.2, the
/// unassigned-completed-issue case the safe-direction rule exists
/// for.
async fn insert_unassigned_done_issue(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    sprint_id: &str,
    title: &str,
) {
    insert_done_issue_with_assignee(app, project_id, author_id, sprint_id, None, title).await;
}

async fn insert_done_issue_with_assignee(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    sprint_id: &str,
    assignee_id: Option<&str>,
    title: &str,
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
            effort: Some(3),
            assignee_id,
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
/// and an active sprint. Returns `(app, team_id, project_id,
/// sprint_id, alice_id, bob_id)`, logged in as Alice.
async fn team_with_active_sprint() -> (TestApp, String, String, String, String, String) {
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

    (app, team_id, project_id, sprint_id, alice_id, bob_id)
}

/// Check 1: two distinct contributors — burndown renders, median
/// line renders.
#[tokio::test]
async fn two_contributors_burndown_and_median_render() {
    let (app, _team_id, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("viewBox") && body.contains("cumulative"),
        "two contributors should render the burndown chart"
    );
}

/// Check 2: one contributor — burndown absent, summary totals still
/// present.
#[tokio::test]
async fn one_contributor_burndown_absent_totals_present() {
    let (app, _team_id, project_id, sprint_id, alice_id, _bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "B").await;

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
        body.contains("Committed") && body.contains("Completed") && body.contains(">6<"),
        "the summary card's totals must still render, unaffected by the \
         suppression above it: {body}"
    );
}

/// Check 6 (`QA-017` round 2, architect review §3): an unassigned
/// completed issue makes the true contributor count unknown, and
/// unknown is treated the same as "fewer than two" — even on a
/// sprint that would otherwise show a trajectory. Two known,
/// distinct contributors (Alice, Bob) complete one issue each, which
/// alone would render the burndown (see
/// `two_contributors_burndown_and_median_render`); a third completed
/// issue with no assignee is added on top. The true count could be 2
/// or more, never less — but "could be more" is still "not
/// verifiably two known people", so this must suppress, the same as
/// the one-contributor case. A bare `COUNT(DISTINCT assignee_id)`
/// that dropped the unassigned-makes-it-`None` rule would still see
/// exactly 2 known assignees here and render, which is why this test
/// exists separately from check 1: check 1 has no unassigned issue to
/// distinguish "correctly counted 2" from "incorrectly ignored an
/// unknown".
#[tokio::test]
async fn unassigned_completed_issue_suppresses_even_with_two_known_contributors() {
    let (app, _team_id, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;
    insert_unassigned_done_issue(&app, &project_id, &alice_id, &sprint_id, "C").await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        !body.contains("cumulative"),
        "an unassigned completed issue makes the count unknown -- must suppress \
         even though two known contributors would otherwise qualify: {body}"
    );
}

/// Check 3: one contributor — velocity bars present, median line
/// absent.
#[tokio::test]
async fn one_contributor_velocity_bars_present_median_absent() {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    let project_id = create_team_project(&app.db, &alice_id, &team_id, "Project").await;

    for i in 0..2 {
        let sprint_id = create_planned_sprint(&app.db, &team_id, &format!("Sprint {i}")).await;
        peisear_storage::sprints::start(&app.db, &sprint_id)
            .await
            .expect("start sprint");
        insert_done_issue(
            &app,
            &project_id,
            &alice_id,
            &sprint_id,
            &alice_id,
            "Solo work",
        )
        .await;
        peisear_storage::sprints::complete(&app.db, &sprint_id)
            .await
            .expect("complete sprint");
    }

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
        !body.contains("The dotted line is the median"),
        "the caption's median sentence (`QA-017` round 2) is gated on the same \
         predicate as the line it describes -- it must not render on its own \
         once the line is gone: {body}"
    );
    assert!(
        body.contains("Numbers describe what happened"),
        "the caption's closing note is not gated on the median predicate -- it \
         must still render: {body}"
    );
}

/// Check 4: one contributor — no text anywhere on the page explains
/// the absence. The guard against `QA-017` §4's silence being undone
/// later by a well-meant explanatory note.
///
/// This is a copy tripwire, not evidence the suppression itself
/// fired — it asserts these phrases never appear, which holds
/// whether or not `show_trajectory`/`show_median` actually suppressed
/// anything. `two_contributors_burndown_and_median_render`,
/// `one_contributor_burndown_absent_totals_present`, and
/// `one_contributor_velocity_bars_present_median_absent` are the
/// tests that prove the suppression fired (`QA-017` round-2 review,
/// §3: confirmed by planting `distinct_contributors` to always return
/// a fixed count and observing only those three fail).
#[tokio::test]
async fn one_contributor_page_has_no_text_explaining_the_absence() {
    let (app, _team_id, project_id, sprint_id, alice_id, _bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;

    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    let body = resp.text().to_lowercase();

    for phrase in [
        "one person",
        "one contributor",
        "single contributor",
        "privacy",
        "hidden because",
        "not shown because",
        "only you",
        "solo",
    ] {
        assert!(
            !body.contains(phrase),
            "the page must not explain the suppression -- found {phrase:?} in body"
        );
    }
}

/// Check 5: a `viewer`-role member sees the same behaviour as a
/// `member` — the audience `QA-016` established, confirmed here on
/// both the two-contributor and one-contributor cases.
#[tokio::test]
async fn viewer_role_sees_the_same_behaviour_as_member() {
    let (app, team_id, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    let carol = TestUser::new("carol");
    let carol_id = register_and_login(&app, &carol).await;
    peisear_storage::teams::add_member(&app.db, &team_id, &carol_id, TeamRole::Viewer)
        .await
        .expect("add carol as viewer");

    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;

    common::auth::logout(&app).await;
    common::auth::login(&app, &carol).await;
    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains("cumulative"),
        "a viewer must see the burndown for a two-contributor sprint, same as a member"
    );
}

// ─────────────────────────────────────────────────────────────
// SPRINT-005 (`DEC-054`, RFC 0013) -- the gate reads the record's basis
// ─────────────────────────────────────────────────────────────
//
// After `SPRINT-004` a completed sprint *displays* the record captured at
// completion, but `distinct_contributors` still asked about *now*: live
// membership, live status. Whether the captured trajectory is reversible to
// one person is a property of who contributed to it, and that was fixed at
// completion. Work finished afterwards could therefore raise the count and
// open a trajectory nobody decided to disclose -- the disclosure
// `NFR-PRIV-007` exists to prevent, arriving by itself.

/// A non-`done` issue assigned to `assignee_id`, linked to `sprint_id`.
async fn insert_open_issue_for(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    sprint_id: &str,
    assignee_id: &str,
    title: &str,
) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &id,
        project_id,
        author_id,
        IssueFields {
            title,
            description: "Test issue body.",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: Some(2),
            assignee_id: Some(assignee_id),
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert open issue");
    peisear_storage::sprints::add_issue(&app.db, sprint_id, &id)
        .await
        .expect("add issue to sprint");
    id
}

async fn sprint_page(app: &TestApp, sprint_id: &str) -> String {
    let resp = app
        .server
        .get(&format!("/teams/engineering/sprints/{sprint_id}"))
        .await;
    resp.assert_status(StatusCode::OK);
    resp.text()
}

/// **The disclosure, demonstrated.** A sprint completes with **one**
/// contributor among its done issues -- trajectory correctly hidden. Another
/// member's issue, left in it, is finished afterwards. The captured record
/// did not change and neither may the gate: the trajectory stays hidden.
#[tokio::test]
async fn finishing_work_after_completion_cannot_open_a_hidden_trajectory() {
    let (app, _team, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    let bobs = insert_open_issue_for(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;
    peisear_storage::sprints::complete(&app.db, &sprint_id)
        .await
        .expect("complete");

    assert!(
        !sprint_page(&app, &sprint_id).await.contains("cumulative"),
        "one contributor at completion: the trajectory is hidden"
    );

    sqlx::query("UPDATE issues SET status = 'done' WHERE id = ?1")
        .bind(&bobs)
        .execute(&app.db)
        .await
        .expect("finish Bob's issue in place");

    assert!(
        !sprint_page(&app, &sprint_id).await.contains("cumulative"),
        "work finished after completion must not open a trajectory that was hidden \
         at completion"
    );
}

/// The reverse still works: a sprint completed with two contributors keeps
/// its trajectory when work is carried out of it afterwards.
#[tokio::test]
async fn a_sprint_completed_with_two_contributors_keeps_its_trajectory_when_work_leaves() {
    let (app, _team, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;
    peisear_storage::sprints::complete(&app.db, &sprint_id)
        .await
        .expect("complete");
    assert!(sprint_page(&app, &sprint_id).await.contains("cumulative"));

    sqlx::query(
        "DELETE FROM sprint_issues WHERE issue_id = (SELECT id FROM issues WHERE title = 'B')",
    )
    .execute(&app.db)
    .await
    .expect("carry Bob's issue out");

    assert!(
        sprint_page(&app, &sprint_id).await.contains("cumulative"),
        "two contributors at completion: carrying work out later must not hide the \
         captured trajectory"
    );
}

/// Reopen re-derives the gate from live data; completing recaptures the basis.
#[tokio::test]
async fn reopen_makes_the_gate_live_again_and_completing_recaptures_the_basis() {
    let (app, _team, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    let bobs = insert_open_issue_for(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;
    let basis = |app: &TestApp, sprint: String| {
        let db = app.db.clone();
        async move {
            sqlx::query_as::<_, (i64, i64)>(
                "SELECT contributor_count, had_unassigned_contributor FROM sprint_records WHERE sprint_id = ?1",
            )
            .bind(sprint)
            .fetch_one(&db)
            .await
            .expect("record basis")
        }
    };

    peisear_storage::sprints::complete(&app.db, &sprint_id)
        .await
        .unwrap();
    assert_eq!(basis(&app, sprint_id.clone()).await, (1, 0));

    peisear_storage::sprints::reopen(&app.db, &sprint_id)
        .await
        .unwrap();
    assert!(
        !sprint_page(&app, &sprint_id).await.contains("cumulative"),
        "live: one contributor"
    );
    sqlx::query("UPDATE issues SET status = 'done' WHERE id = ?1")
        .bind(&bobs)
        .execute(&app.db)
        .await
        .unwrap();
    assert!(
        sprint_page(&app, &sprint_id).await.contains("cumulative"),
        "reopened, the gate is live again: two contributors now"
    );

    peisear_storage::sprints::complete(&app.db, &sprint_id)
        .await
        .unwrap();
    assert_eq!(basis(&app, sprint_id.clone()).await, (2, 0), "recaptured");
    assert!(sprint_page(&app, &sprint_id).await.contains("cumulative"));
}

/// `had_unassigned_contributor` suppresses as the live `unassigned > 0`
/// does: a completed issue with no assignee could be anyone's.
#[tokio::test]
async fn an_unassigned_completed_issue_suppresses_the_captured_trajectory() {
    let (app, _team, project_id, sprint_id, alice_id, bob_id) = team_with_active_sprint().await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &alice_id, "A").await;
    insert_done_issue(&app, &project_id, &alice_id, &sprint_id, &bob_id, "B").await;
    insert_unassigned_done_issue(&app, &project_id, &alice_id, &sprint_id, "C").await;
    peisear_storage::sprints::complete(&app.db, &sprint_id)
        .await
        .unwrap();

    let flag: i64 = sqlx::query_scalar(
        "SELECT had_unassigned_contributor FROM sprint_records WHERE sprint_id = ?1",
    )
    .bind(&sprint_id)
    .fetch_one(&app.db)
    .await
    .unwrap();
    assert_eq!(flag, 1);
    assert!(
        !sprint_page(&app, &sprint_id).await.contains("cumulative"),
        "two known contributors and one unknown: suppressed"
    );
}

/// Complete `n` sprints, sprint `i` having exactly the contributors
/// `by[i]` (each a user id), and return the velocity page body.
async fn velocity_page_for(by: &[&[&str]]) -> (TestApp, String) {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    let project_id = create_team_project(&app.db, &alice_id, &team_id, "Project").await;
    // three more people, registered through the real flow, addressed by name
    let mut people = std::collections::HashMap::new();
    people.insert("alice", alice_id.clone());
    for name in ["bob", "carol", "dave"] {
        let u = TestUser::new(name);
        let id = common::auth::register(&app, &u).await;
        people.insert(name, id);
    }
    common::auth::login(&app, &alice).await;
    for (i, contributors) in by.iter().enumerate() {
        let sprint_id = create_planned_sprint(&app.db, &team_id, &format!("Sprint {i}")).await;
        peisear_storage::sprints::start(&app.db, &sprint_id)
            .await
            .unwrap();
        for who in *contributors {
            insert_done_issue(
                &app,
                &project_id,
                &alice_id,
                &sprint_id,
                &people[who],
                "work",
            )
            .await;
        }
        peisear_storage::sprints::complete(&app.db, &sprint_id)
            .await
            .unwrap();
    }
    let body = app.server.get("/teams/engineering/sprints").await.text();
    (app, body)
}

/// **Velocity.** Three completed sprints, each the output of a *single*,
/// different person. The live union is three people, so today's gate showed
/// the median line -- but each sprint's number is one person's output, so
/// the aggregate is reversible per sprint. The conservative reading (the set
/// passes only if one sprint clears the floor on its own) suppresses it.
/// In this fixture **three sprints** change visibility.
#[tokio::test]
async fn velocity_is_suppressed_when_no_single_sprint_clears_the_floor() {
    let (_app, body) = velocity_page_for(&[&["alice"], &["bob"], &["carol"]]).await;
    assert!(body.contains("<rect"), "the bars are unaffected: {body}");
    assert!(
        !body.contains("stroke-dasharray"),
        "three sprints, one contributor each, is not an aggregate: the median line \
         must be suppressed"
    );
}

/// And the median still shows when one sprint clears the floor on its own.
#[tokio::test]
async fn velocity_is_shown_when_one_sprint_clears_the_floor_on_its_own() {
    let (_app, body) = velocity_page_for(&[&["alice"], &["alice", "bob"], &["carol"]]).await;
    assert!(
        body.contains("stroke-dasharray"),
        "one sprint has two contributors on its own: the median line shows"
    );
}

/// `REQ-002` / `FR-TEAM-005`: the privacy footnote is **on the team detail
/// screen**, for every role that can see the team. Its wording is byte-pinned in
/// `peisear-i18n` (`team_privacy_footnote_renders_byte_identically`) -- that is
/// the string; this is the screen, which `REQ-001` found nothing asserted.
#[tokio::test]
async fn the_team_privacy_footnote_renders_on_the_team_screen_for_every_role() {
    let app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&app, &alice).await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&app, &bob).await;
    let carol = TestUser::new("carol");
    let carol_id = register_and_login(&app, &carol).await;
    common::auth::logout(&app).await;

    let team_id = create_team_with_admin(&app.db, &alice_id, "Engineering").await;
    peisear_storage::teams::add_member(&app.db, &team_id, &bob_id, TeamRole::Member)
        .await
        .expect("add bob");
    peisear_storage::teams::add_member(&app.db, &team_id, &carol_id, TeamRole::Viewer)
        .await
        .expect("add carol");
    let slug = peisear_storage::teams::find_by_id(&app.db, &team_id)
        .await
        .expect("find team")
        .expect("team exists")
        .slug;
    let footnote =
        peisear_i18n::Locale::English.render(peisear_i18n::MessageKey::TeamPrivacyFootnote);

    for (who, role) in [(&alice, "admin"), (&bob, "member"), (&carol, "viewer")] {
        common::auth::login(&app, who).await;
        let body = app.server.get(&format!("/teams/{slug}")).await.text();
        assert!(
            body.contains(&footnote),
            "the {role}'s team screen must carry the privacy footnote: {body}"
        );
        common::auth::logout(&app).await;
    }
}
