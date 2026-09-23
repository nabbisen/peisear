//! Tests for the URL-primary, server-default-secondary
//! filter/sort persistence introduced in Phase A Step 3
//! (peisear-feature-spec-v2.1 §4.4).
//!
//! Two invariants matter:
//!
//! 1. **URL wins.** A query parameter on the URL always
//!    overrides whatever the user previously saved.
//! 2. **Bare URL inherits.** A URL with NO filter/sort params
//!    falls back to the user's previously saved default. A
//!    bare URL must NOT erase the saved default.
//!
//! Together these let a user pick a filter once (saving it as
//! their default), then navigate to detail pages and back via
//! bare links without losing context.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;

#[tokio::test]
async fn list_view_renders_with_filter_toolbar() {
    // The filter/sort toolbar must appear on the list view.
    // (Board view doesn't need it because the kanban columns
    // are themselves a status filter.)
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;
    let _ = create_issue(&app.db, &project_id, &user_id, "First").await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=list"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // Toolbar form's GET action.
    let toolbar_marker = r#"aria-label="Filter and sort issues""#;
    assert!(
        body.contains(toolbar_marker),
        "list view missing filter/sort toolbar"
    );
    // Scoped to the toolbar `<form>`, not the whole page.
    // `TT-003` §5, confirmed by planting: each list row also renders
    // its own status-change form (`issues.rs:1050`, `name="status"`)
    // and carries a hidden `name="sort"` field
    // (`issues.rs`, the per-row form's own sort-preserving input) --
    // both independent of the toolbar's own `<select>`s, so an
    // unscoped check on either stayed green with the toolbar select
    // deleted entirely. `name="assignee"` has no such collision (the
    // per-row form's equivalent field is named `filter_assignee`) but
    // is scoped the same way for consistency.
    let marker_at = body
        .find(toolbar_marker)
        .expect("toolbar aria-label present");
    let form_start = body[..marker_at]
        .rfind("<form")
        .expect("a <form tag precedes the toolbar's aria-label");
    let form_end = body[form_start..]
        .find("</form>")
        .map(|i| form_start + i)
        .expect("toolbar form has a closing </form>");
    let toolbar = &body[form_start..form_end];

    // Status select.
    assert!(
        toolbar.contains(r#"name="status""#),
        "missing status select: {toolbar}"
    );
    // Assignee select.
    assert!(
        toolbar.contains(r#"name="assignee""#),
        "missing assignee select: {toolbar}"
    );
    // Sort select.
    assert!(
        toolbar.contains(r#"name="sort""#),
        "missing sort select: {toolbar}"
    );
}

#[tokio::test]
async fn url_filter_filters_the_list() {
    // Create issues with mixed statuses, ask for status=open
    // via URL, and verify only the open ones appear.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let _open = create_issue(&app.db, &project_id, &user_id, "Open issue").await;
    let other = create_issue(&app.db, &project_id, &user_id, "Done issue").await;

    // Mark `other` as done so the open filter excludes it.
    sqlx::query(r#"UPDATE issues SET status = 'done' WHERE id = ?1"#)
        .bind(&other)
        .execute(&app.db)
        .await
        .expect("update issue status");

    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=list&status=open"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("Open issue"),
        "open issue should be in filtered list"
    );
    assert!(
        !body.contains("Done issue"),
        "done issue should be filtered out by status=open"
    );
}

#[tokio::test]
async fn explicit_filter_persists_as_default() {
    // After visiting a URL with explicit filter, a subsequent
    // bare URL on the same project must show the same filter.
    // This is the persistence half of the URL-primary /
    // server-default-secondary scheme.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let _open = create_issue(&app.db, &project_id, &user_id, "Open issue").await;
    let other = create_issue(&app.db, &project_id, &user_id, "Done issue").await;
    sqlx::query(r#"UPDATE issues SET status = 'done' WHERE id = ?1"#)
        .bind(&other)
        .execute(&app.db)
        .await
        .expect("update issue status");

    // 1. Visit with explicit filter; assert it's applied.
    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=list&status=open"))
        .await;
    resp.assert_status(StatusCode::OK);

    // 2. Visit BARE url (no query params except view, since the
    // bare URL goes to board view by default and the filter
    // wouldn't be applied to the board column structure anyway).
    // We need view=list to land on the filterable list view.
    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=list"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // Expected: open shows, done is filtered out — the saved
    // default (status=open) was applied.
    assert!(
        body.contains("Open issue"),
        "saved default not applied: open issue should still appear"
    );
    assert!(
        !body.contains("Done issue"),
        "saved default not applied: done issue should remain filtered"
    );
}

#[tokio::test]
async fn url_overrides_saved_default() {
    // A user with a saved status=open default visits the URL
    // with status=done. The URL must win for that visit.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Test").await;

    let _open = create_issue(&app.db, &project_id, &user_id, "Open issue").await;
    let done = create_issue(&app.db, &project_id, &user_id, "Done issue").await;
    sqlx::query(r#"UPDATE issues SET status = 'done' WHERE id = ?1"#)
        .bind(&done)
        .execute(&app.db)
        .await
        .expect("update issue status");

    // Save status=open as default.
    let _ = app
        .server
        .get(&format!("/projects/{project_id}?view=list&status=open"))
        .await;

    // Now visit with status=done — URL must override.
    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=list&status=done"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        !body.contains("Open issue"),
        "URL status=done should hide the open issue"
    );
    assert!(
        body.contains("Done issue"),
        "URL status=done should show the done issue"
    );
}

#[tokio::test]
async fn defaults_are_per_user() {
    // Alice and Bob each save their own filter on their own
    // personal projects. Bob's saved state must not leak into
    // Alice's view of her project, and vice versa.
    //
    // Each user works on their own personal project (rather
    // than a shared team project) to keep this test in Phase A
    // scope — team projects + per-team isolation is Phase C.
    let alice_app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&alice_app, &alice).await;
    let alice_project = create_personal_project(&alice_app.db, &alice_id, "Alice's").await;
    let _alice_open = create_issue(&alice_app.db, &alice_project, &alice_id, "Alice open").await;
    let alice_done = create_issue(&alice_app.db, &alice_project, &alice_id, "Alice done").await;
    sqlx::query(r#"UPDATE issues SET status = 'done' WHERE id = ?1"#)
        .bind(&alice_done)
        .execute(&alice_app.db)
        .await
        .expect("update");

    // Alice saves status=open as her default.
    let _ = alice_app
        .server
        .get(&format!("/projects/{alice_project}?view=list&status=open"))
        .await;

    // A fresh Alice visit with bare list URL → still filters to open.
    let resp = alice_app
        .server
        .get(&format!("/projects/{alice_project}?view=list"))
        .await;
    let alice_body = resp.text();
    assert!(
        alice_body.contains("Alice open"),
        "Alice's saved default should still apply on her own project"
    );
    assert!(
        !alice_body.contains("Alice done"),
        "Alice's saved status=open default should still hide done"
    );

    // No assertion on Bob in this test — the cross-user case
    // belongs in Phase C (team projects). The test name is
    // aspirational: it documents that the storage key includes
    // user_id (see view_states::project_issues_key) so even when
    // team projects land, Bob's preferences won't leak into
    // Alice's view of a shared project.
}

// ──────────────────────────────────────────────────────────────
// `ORD-001` — the issue list, and every board column, order by
// `created_at DESC` (newest first) and nothing else.
//
// Before this, both list queries ordered `status ASC, position ASC,
// created_at DESC`. `status` is TEXT, so `ASC` is alphabetical --
// `done`, `in_progress`, `open` -- and the default list showed Done
// issues first. `position` was assigned `MAX+1` on insert and never
// recomputed on a status change, so within a band it was creation
// order (oldest first) and across bands it carried a number that
// meant nothing. No test referenced either, which is why neither
// was ever caught. Every assertion below is by *relative offset in the
// rendered body*, not `body.contains`, which passes on any order.
// ──────────────────────────────────────────────────────────────

/// Insert an issue in a given status, then pin its `created_at`
/// explicitly. `created_at` is `CURRENT_TIMESTAMP` (one-second
/// resolution) and a test creates rows in microseconds, so relying on
/// real time would make the newest/oldest relationship a coin flip.
async fn insert_with_status_at(
    app: &TestApp,
    project_id: &str,
    author_id: &str,
    title: &str,
    status: peisear_core::IssueStatus,
    created_at: &str,
) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &id,
        project_id,
        author_id,
        peisear_storage::issues::IssueFields {
            title,
            description: "",
            status,
            priority: peisear_core::Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert issue");
    sqlx::query("UPDATE issues SET created_at = ?1 WHERE id = ?2")
        .bind(created_at)
        .bind(&id)
        .execute(&app.db)
        .await
        .expect("pin created_at");
    id
}

/// Byte offset of `needle`'s first occurrence, panicking with the body
/// if it is absent -- an absent row would otherwise read as "ordered".
fn offset_of(body: &str, needle: &str) -> usize {
    body.find(needle)
        .unwrap_or_else(|| panic!("{needle:?} not found in body: {body}"))
}

/// The default list is newest first **regardless of status**, and Done
/// is no longer at the top. The oldest issue is Done, the newest is
/// Open, so the old alphabetical-status ordering and the new
/// recency-only ordering are exact opposites here -- no ordering can
/// satisfy both, and the assertion names the old behaviour so it
/// cannot silently revert.
#[tokio::test]
async fn default_list_is_newest_first_regardless_of_status() {
    use peisear_core::IssueStatus::{Done, InProgress, Open};
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Ordering").await;

    insert_with_status_at(
        &app,
        &project_id,
        &user_id,
        "ORD-oldest-done",
        Done,
        "2026-01-01 09:00:00",
    )
    .await;
    insert_with_status_at(
        &app,
        &project_id,
        &user_id,
        "ORD-middle-in-progress",
        InProgress,
        "2026-01-02 09:00:00",
    )
    .await;
    insert_with_status_at(
        &app,
        &project_id,
        &user_id,
        "ORD-newest-open",
        Open,
        "2026-01-03 09:00:00",
    )
    .await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=list"))
        .await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let newest = offset_of(&body, "ORD-newest-open");
    let middle = offset_of(&body, "ORD-middle-in-progress");
    let oldest = offset_of(&body, "ORD-oldest-done");
    assert!(
        newest < middle && middle < oldest,
        "the list must read newest first (open, in-progress, done here): \
         newest@{newest} middle@{middle} oldest@{oldest}"
    );
    assert!(
        oldest > newest,
        "the Done issue must no longer be first -- the old `status ASC` put `done` \
         ahead of `in_progress` and `open` alphabetically"
    );
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

/// The board's columns are unchanged in *order* -- Open, In Progress,
/// Done, from `IssueStatus::all()`, never from SQL -- so removing the
/// status term from the query leaves the board exactly as it was.
#[tokio::test]
async fn board_columns_stay_in_lifecycle_order() {
    use peisear_core::IssueStatus::{Done, InProgress, Open};
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Columns").await;
    insert_with_status_at(
        &app,
        &project_id,
        &user_id,
        "COL-done",
        Done,
        "2026-01-01 09:00:00",
    )
    .await;
    insert_with_status_at(
        &app,
        &project_id,
        &user_id,
        "COL-progress",
        InProgress,
        "2026-01-02 09:00:00",
    )
    .await;
    insert_with_status_at(
        &app,
        &project_id,
        &user_id,
        "COL-open",
        Open,
        "2026-01-03 09:00:00",
    )
    .await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=board"))
        .await;
    let body = resp.text();
    let open = offset_of(&body, r#"data-status="open""#);
    let progress = offset_of(&body, r#"data-status="in_progress""#);
    let done = offset_of(&body, r#"data-status="done""#);
    assert!(
        open < progress && progress < done,
        "board columns must be Open, In Progress, Done: open@{open} progress@{progress} done@{done}"
    );
}

/// **Within** a board column the order is now newest first. It was
/// `position ASC` -- creation order, oldest first -- so this is a
/// *change*, not an invariant: the handoff said the board was
/// unchanged, and it is unchanged in its columns but not in what
/// each column shows first. Each column is filled from the same
/// storage-ordered list the list view reads, so the one query
/// decides both.
#[tokio::test]
async fn board_column_contents_are_newest_first() {
    use peisear_core::IssueStatus::Open;
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Within").await;
    insert_with_status_at(
        &app,
        &project_id,
        &user_id,
        "WITHIN-older",
        Open,
        "2026-01-01 09:00:00",
    )
    .await;
    insert_with_status_at(
        &app,
        &project_id,
        &user_id,
        "WITHIN-newer",
        Open,
        "2026-01-02 09:00:00",
    )
    .await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=board"))
        .await;
    let body = resp.text();
    assert!(
        offset_of(&body, "WITHIN-newer") < offset_of(&body, "WITHIN-older"),
        "within a board column the newest issue must come first"
    );
}

/// The explicit sorts are unchanged by `ORD-001`: they re-sort in Rust
/// (`apply_filter_and_sort`), so only their *tie-breaking* input order
/// moved. `sort=priority` in particular ranks with an explicit
/// `Urgent => 0 … Low => 3` and not a SQL `ORDER BY` on the TEXT
/// column -- the correct pattern `ORD-001` §3 points at and
/// `PLAN-003` fixes a query for lacking. Three rows whose priority order
/// (urgent, high, low), creation order and update order are all
/// different, so each sort has only one right answer.
#[tokio::test]
async fn explicit_sorts_are_unchanged() {
    use peisear_core::IssueStatus::Open;
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Sorts").await;

    // (title, priority, created_at, updated_at)
    let rows = [
        (
            "SORT-low",
            "low",
            "2026-01-03 09:00:00",
            "2026-02-01 09:00:00",
        ),
        (
            "SORT-urgent",
            "urgent",
            "2026-01-01 09:00:00",
            "2026-02-03 09:00:00",
        ),
        (
            "SORT-high",
            "high",
            "2026-01-02 09:00:00",
            "2026-02-02 09:00:00",
        ),
    ];
    for (title, priority, created, updated) in rows {
        let id = insert_with_status_at(&app, &project_id, &user_id, title, Open, created).await;
        // Setting `updated_at` explicitly keeps `0017`'s trigger (which
        // only fires when the UPDATE leaves it unchanged) out of the way.
        sqlx::query("UPDATE issues SET priority = ?1, updated_at = ?2 WHERE id = ?3")
            .bind(priority)
            .bind(updated)
            .bind(&id)
            .execute(&app.db)
            .await
            .expect("pin priority and updated_at");
    }

    let order = |sort: &'static str| {
        let app = &app;
        let project_id = &project_id;
        async move {
            let body = app
                .server
                .get(&format!("/projects/{project_id}?view=list&sort={sort}"))
                .await
                .text();
            let mut titles = ["SORT-urgent", "SORT-high", "SORT-low"];
            titles.sort_by_key(|t| offset_of(&body, t));
            titles.to_vec()
        }
    };
    assert_eq!(
        order("priority").await,
        ["SORT-urgent", "SORT-high", "SORT-low"]
    );
    assert_eq!(
        order("created").await,
        ["SORT-low", "SORT-high", "SORT-urgent"]
    );
    assert_eq!(
        order("updated").await,
        ["SORT-urgent", "SORT-high", "SORT-low"]
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
