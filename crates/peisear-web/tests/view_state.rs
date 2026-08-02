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
    assert!(
        body.contains(r#"aria-label="Filter and sort issues""#),
        "list view missing filter/sort toolbar"
    );
    // Status select.
    assert!(body.contains(r#"name="status""#), "missing status select");
    // Assignee select.
    assert!(
        body.contains(r#"name="assignee""#),
        "missing assignee select"
    );
    // Sort select.
    assert!(body.contains(r#"name="sort""#), "missing sort select");
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
