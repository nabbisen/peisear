//! Ordering — what order each list, pick and series comes back in, and
//! what a tie on a one-second timestamp does to it.
//!
//! `created_at` / `updated_at` are `CURRENT_TIMESTAMP` (one-second
//! resolution) and SQLite returns tied rows oldest first, so an
//! ordering with no tiebreak reads backwards inside a tie, and a
//! `LIMIT 1` pick can choose the superseded row. `ORD-001` found it on
//! the issue list; `ORD-002` made every timestamp ordering in
//! `peisear-storage` end in a `rowid` tiebreak; `ORD-003` gave these
//! tests their own file. Every tied fixture pins its timestamp
//! explicitly, and every assertion about a rendered page is by relative
//! byte offset, never `body.contains`.

mod common;

use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;

/// Byte offset of `needle`'s first occurrence, panicking with the body
/// if it is absent -- an absent row would otherwise read as "ordered".
fn offset_of(body: &str, needle: &str) -> usize {
    body.find(needle)
        .unwrap_or_else(|| panic!("{needle:?} not found in body: {body}"))
}

/// Same-second creations. `created_at` ties at one-second resolution
/// and a bare `ORDER BY created_at DESC` then returns tied rows in
/// scan order -- **creation order, i.e. oldest first**, the opposite of
/// what the ordering says (measured against the linked SQLite: four
/// issues created back to back came out `first, second, third,
/// fourth`). The `rowid DESC` tiebreak is what makes "newest first"
/// true for a burst, and this is the assertion that fails without it.
#[tokio::test]
async fn issues_created_in_the_same_second_list_newest_first() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Burst").await;

    for title in ["BURST-first", "BURST-second", "BURST-third", "BURST-fourth"] {
        create_issue(&app.db, &project_id, &user_id, title).await;
    }
    let distinct: i64 = sqlx::query_scalar("SELECT COUNT(DISTINCT created_at) FROM issues")
        .fetch_one(&app.db)
        .await
        .expect("count distinct created_at");
    assert_eq!(
        distinct, 1,
        "fixture assumption: all four rows share one created_at second -- if this \
         fails the test no longer exercises the tiebreak"
    );

    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=list"))
        .await;
    let body = resp.text();
    let offsets: Vec<usize> = ["BURST-fourth", "BURST-third", "BURST-second", "BURST-first"]
        .iter()
        .map(|t| offset_of(&body, t))
        .collect();
    assert!(
        offsets.windows(2).all(|w| w[0] < w[1]),
        "same-second creations must list newest first (fourth, third, second, first): {offsets:?}"
    );
}

// ──────────────────────────────────────────────────────────────
// ORD-002 — a tie on a one-second timestamp must not pick the wrong row.
//
// `created_at` / `updated_at` are `CURRENT_TIMESTAMP`, so rows written
// in the same second tie, and SQLite returns a tie oldest-first. Every
// fixture below pins the timestamp explicitly rather than inserting
// quickly and hoping the burst lands inside one second: a burst that
// straddled a second boundary would pass on the unfixed code.
// ──────────────────────────────────────────────────────────────

/// The one second every tied row in these tests shares.
const TIED: &str = "2026-03-01 09:00:00";

/// Write a `user_capacities` row directly, bypassing `insert`'s
/// overlap check, so two open-ended rows can share a `created_at`.
/// Rows are inserted in the order the caller lists them, which is
/// what gives them ascending `rowid`s.
async fn insert_capacity_row(app: &TestApp, user_id: &str, points: i64) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO user_capacities (id, user_id, points, created_at) VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(points)
    .bind(TIED)
    .execute(&app.db)
    .await
    .expect("insert capacity row");
    id
}

/// A user whose capacity was saved twice in one second: 10 first,
/// then 99. The later save is meant to win.
async fn user_with_capacity_saved_twice() -> (TestApp, String) {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    insert_capacity_row(&app, &user_id, 10).await;
    insert_capacity_row(&app, &user_id, 99).await;
    (app, user_id)
}

#[tokio::test]
async fn capacity_saved_twice_in_one_second_resolves_to_the_later_row() {
    let (app, user_id) = user_with_capacity_saved_twice().await;
    let row = peisear_storage::user_capacities::effective_row_for_user(&app.db, &user_id)
        .await
        .expect("effective row")
        .expect("a row applies today");
    assert_eq!(row.points, 99, "the superseded 10 won the tie");
}

#[tokio::test]
async fn capacity_on_a_date_saved_twice_in_one_second_resolves_to_the_later_row() {
    let (app, user_id) = user_with_capacity_saved_twice().await;
    let day = chrono::NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
    let points =
        peisear_storage::user_capacities::effective_for_user_on_date(&app.db, &user_id, day)
            .await
            .expect("effective on date");
    assert_eq!(points, Some(99), "the superseded 10 won the tie");
}

/// The third pick is a correlated sub-select inside `project_workload`,
/// which feeds the workload strip, WIP and the health indicators.
#[tokio::test]
async fn workload_capacity_saved_twice_in_one_second_is_the_later_value() {
    let (app, user_id) = user_with_capacity_saved_twice().await;
    let project_id = create_personal_project(&app.db, &user_id, "Workload").await;
    let loads = peisear_storage::issues::project_workload(&app.db, &project_id)
        .await
        .expect("project workload");
    let mine = loads
        .iter()
        .find(|l| l.user_id == user_id)
        .expect("the owner is a candidate");
    assert_eq!(
        mine.capacity_points,
        Some(99),
        "the superseded 10 won the tie"
    );
}

/// `ASC` site, pinned: `overlaps_existing` names the *earliest* of the
/// rows it collides with. Correct today by scan order alone; the
/// explicit `rowid ASC` makes it a stated property, and this must not move.
#[tokio::test]
async fn overlap_conflict_names_the_earliest_of_tied_rows() {
    let (app, user_id) = user_with_capacity_saved_twice().await;
    let conflict =
        peisear_storage::user_capacities::overlaps_existing(&app.db, &user_id, None, None, None)
            .await
            .expect("overlap check")
            .expect("both open-ended rows overlap");
    assert_eq!(
        conflict.points, 10,
        "the first-written row is the one named"
    );
}

/// The realistic inbox case: one action writes several notifications
/// in the same second, and the inbox must show the last one first.
#[tokio::test]
async fn notifications_from_one_second_list_newest_first() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    for title in ["INBOX-first", "INBOX-second", "INBOX-third", "INBOX-fourth"] {
        peisear_storage::notifications::insert(
            &app.db,
            &user_id,
            peisear_storage::notifications::NewNotification {
                kind: "test",
                severity: peisear_core::notifications::Severity::Info,
                title,
                body: "burst",
                payload_json: None,
                dispatched_via: &[],
            },
        )
        .await
        .expect("insert notification");
    }
    sqlx::query("UPDATE notifications SET created_at = ?1")
        .bind(TIED)
        .execute(&app.db)
        .await
        .expect("tie the burst");

    let got = peisear_storage::notifications::recent_for_user(&app.db, &user_id, 10)
        .await
        .expect("recent")
        .into_iter()
        .map(|n| n.title)
        .collect::<Vec<_>>();
    assert_eq!(
        got,
        ["INBOX-fourth", "INBOX-third", "INBOX-second", "INBOX-first"]
    );

    let body = app.server.get("/inbox").await.text();
    let offsets: Vec<usize> = ["INBOX-fourth", "INBOX-third", "INBOX-second", "INBOX-first"]
        .iter()
        .map(|t| offset_of(&body, t))
        .collect();
    assert!(
        offsets.windows(2).all(|w| w[0] < w[1]),
        "the rendered inbox must read newest first: {offsets:?}"
    );
}

/// `ASC` site, pinned: sub-issues written in one second list in
/// creation order.
#[tokio::test]
async fn sub_issues_from_one_second_list_in_creation_order() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Subs").await;
    let parent_id = create_issue(&app.db, &project_id, &user_id, "Parent").await;
    for title in ["SUB-a", "SUB-b", "SUB-c"] {
        let id = uuid::Uuid::new_v4().to_string();
        peisear_storage::issues::insert_sub_issue(
            &app.db,
            &id,
            &project_id,
            &parent_id,
            &user_id,
            title,
            "",
            peisear_core::IssueStatus::Open,
            peisear_core::Priority::Medium,
            None,
            None,
        )
        .await
        .expect("insert sub-issue");
    }
    sqlx::query("UPDATE issues SET created_at = ?1 WHERE parent_issue_id = ?2")
        .bind(TIED)
        .bind(&parent_id)
        .execute(&app.db)
        .await
        .expect("tie the burst");

    let got = peisear_storage::issues::list_sub_issues_of(&app.db, &parent_id)
        .await
        .expect("sub-issues")
        .into_iter()
        .map(|i| i.title)
        .collect::<Vec<_>>();
    assert_eq!(got, ["SUB-a", "SUB-b", "SUB-c"], "must not move");
}

/// The one list query with a `DISTINCT` (§6 named that as a reason
/// to stop if `rowid` were refused): projects created in one second
/// list newest first, and the join fan-out still collapses.
#[tokio::test]
async fn projects_created_in_one_second_list_newest_first() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    for name in ["PROJ-first", "PROJ-second", "PROJ-third"] {
        create_personal_project(&app.db, &user_id, name).await;
    }
    sqlx::query("UPDATE projects SET created_at = ?1, updated_at = ?1")
        .bind(TIED)
        .execute(&app.db)
        .await
        .expect("tie the burst");

    let got = peisear_storage::projects::list_for_user(&app.db, &user_id)
        .await
        .expect("projects")
        .into_iter()
        .map(|p| p.name)
        .collect::<Vec<_>>();
    assert_eq!(got, ["PROJ-third", "PROJ-second", "PROJ-first"]);
}

// ──────────────────────────────────────────────────────────────
// ORD-003 — a sprint's issues read Open, In progress, Done.
//
// `issues_in_sprint` used to order `i.status ASC`, and `status` is
// TEXT, so that was alphabetical: `done, in_progress, open` -- Done
// first, on both surfaces that render the list. The status term is now
// applied in Rust with `IssueStatus::lifecycle_rank` (a *stable* sort,
// so the query's `assigned_at` order survives inside each status).
// ──────────────────────────────────────────────────────────────

/// (title, status, effort), listed in **assignment order**. Two Open
/// and two Done so "assignment order inside a status" has something to
/// say; one In progress between them so the bands are not adjacent.
const SPRINT_ITEMS: [(&str, peisear_core::IssueStatus, i64); 5] = [
    ("SPR-done-1", peisear_core::IssueStatus::Done, 3),
    ("SPR-open-1", peisear_core::IssueStatus::Open, 5),
    ("SPR-prog-1", peisear_core::IssueStatus::InProgress, 2),
    ("SPR-done-2", peisear_core::IssueStatus::Done, 1),
    ("SPR-open-2", peisear_core::IssueStatus::Open, 4),
];

/// A planned sprint holding `SPRINT_ITEMS`. The issues are *created* in
/// the reverse of assignment order and `assigned_at` is pinned to one
/// distinct minute each, so neither creation order nor a one-second
/// tie can stand in for "assignment order". Returns (app, team slug,
/// sprint id).
async fn sprint_with_items_in_every_status() -> (TestApp, String, String) {
    let app = TestApp::spawn().await;
    let admin = TestUser::new("alice");
    let admin_id = register_and_login(&app, &admin).await;
    let team_id = common::fixture::create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = peisear_storage::teams::find_by_id(&app.db, &team_id)
        .await
        .expect("find team")
        .expect("team exists")
        .slug;
    let project_id =
        common::fixture::create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    let sprint_id = common::fixture::create_planned_sprint(&app.db, &team_id, "Sprint 1").await;

    let mut ids = Vec::new();
    for (title, status, effort) in SPRINT_ITEMS.iter().rev() {
        let id = uuid::Uuid::new_v4().to_string();
        peisear_storage::issues::insert(
            &app.db,
            &id,
            &project_id,
            &admin_id,
            peisear_storage::issues::IssueFields {
                title,
                description: "",
                status: *status,
                priority: peisear_core::Priority::Medium,
                effort: Some(*effort),
                assignee_id: None,
                planned_start_at: None,
                planned_end_at: None,
            },
        )
        .await
        .expect("insert issue");
        ids.push((*title, id));
    }
    // Assign in SPRINT_ITEMS order, minute by minute.
    for (minute, (title, _, _)) in SPRINT_ITEMS.iter().enumerate() {
        let id = &ids.iter().find(|(t, _)| t == title).expect("created").1;
        peisear_storage::sprints::add_issue(&app.db, &sprint_id, id)
            .await
            .expect("add to sprint");
        sqlx::query("UPDATE sprint_issues SET assigned_at = ?1 WHERE issue_id = ?2")
            .bind(format!("2026-03-01 09:{minute:02}:00"))
            .bind(id)
            .execute(&app.db)
            .await
            .expect("pin assigned_at");
    }
    (app, slug, sprint_id)
}

/// Both surfaces that render `issues_in_sprint`: the sprint's detail
/// page and the sprint plan's sprint column.
async fn sprint_surfaces(
    app: &TestApp,
    slug: &str,
    sprint_id: &str,
) -> [(&'static str, String); 2] {
    [
        (
            "sprint detail",
            app.server
                .get(&format!("/teams/{slug}/sprints/{sprint_id}"))
                .await
                .text(),
        ),
        (
            "sprint plan",
            app.server
                .get(&format!("/teams/{slug}/sprints/{sprint_id}/plan"))
                .await
                .text(),
        ),
    ]
}

fn band(body: &str, titles: &[&str]) -> (usize, usize) {
    let offsets: Vec<usize> = titles.iter().map(|t| offset_of(body, t)).collect();
    (
        *offsets.iter().min().expect("non-empty band"),
        *offsets.iter().max().expect("non-empty band"),
    )
}

/// Every Open issue above every In progress issue above every Done
/// issue, on both surfaces. Compares whole bands (last of one against
/// first of the next), so it says nothing about the order *inside* a
/// band -- that is a later test's job. Both surfaces are checked
/// before failing, so a failure names each one that is wrong.
#[tokio::test]
async fn sprint_issues_read_open_then_in_progress_then_done() {
    let (app, slug, sprint_id) = sprint_with_items_in_every_status().await;
    let mut wrong = Vec::new();
    for (surface, body) in sprint_surfaces(&app, &slug, &sprint_id).await {
        let open = band(&body, &["SPR-open-1", "SPR-open-2"]);
        let progress = band(&body, &["SPR-prog-1"]);
        let done = band(&body, &["SPR-done-1", "SPR-done-2"]);
        if !(open.1 < progress.0 && progress.1 < done.0) {
            wrong.push(format!(
                "{surface}: open {open:?}, in progress {progress:?}, done {done:?}"
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "expected Open, In progress, Done (byte offsets shown): {wrong:#?}"
    );
}

/// Named so the old behaviour cannot silently return: the alphabetical
/// order put Done first, so the first of the five titles on a page must
/// not be a Done one. Both surfaces are checked before failing.
#[tokio::test]
async fn done_is_no_longer_first_in_a_sprint() {
    let (app, slug, sprint_id) = sprint_with_items_in_every_status().await;
    let mut done_first = Vec::new();
    for (surface, body) in sprint_surfaces(&app, &slug, &sprint_id).await {
        let first = SPRINT_ITEMS
            .iter()
            .min_by_key(|(title, ..)| offset_of(&body, title))
            .expect("items");
        if first.1 == peisear_core::IssueStatus::Done {
            done_first.push(format!("{surface}: {} leads", first.0));
        }
    }
    assert!(
        done_first.is_empty(),
        "the alphabetical `status ASC` put Done first: {done_first:#?}"
    );
}

/// Assignment order survives inside a status: `assigned_at` is the
/// SQL order and the Rust sort is stable. Open-1 was assigned before
/// Open-2 and Done-1 before Done-2 -- the reverse of creation order --
/// so neither creation order nor an unstable sort can pass this.
#[tokio::test]
async fn sprint_issues_keep_assignment_order_within_a_status() {
    let (app, slug, sprint_id) = sprint_with_items_in_every_status().await;
    let mut wrong = Vec::new();
    for (surface, body) in sprint_surfaces(&app, &slug, &sprint_id).await {
        for (earlier, later) in [("SPR-open-1", "SPR-open-2"), ("SPR-done-1", "SPR-done-2")] {
            if offset_of(&body, earlier) >= offset_of(&body, later) {
                wrong.push(format!(
                    "{surface}: {earlier} was assigned before {later} and must stay above it"
                ));
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
