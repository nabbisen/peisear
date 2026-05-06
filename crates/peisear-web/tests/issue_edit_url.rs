//! Tests for the Phase B PR3 (B-3) issue detail / edit URL
//! split.
//!
//! The HTTP behaviour we want to enforce:
//!
//! 1. `GET /projects/{id}/issues/{issue_id}` renders read-only.
//! 2. `GET /projects/{id}/issues/{issue_id}/edit` renders the
//!    edit form.
//! 3. `GET /projects/{id}/issues/{issue_id}?edit=1` (legacy)
//!    308-redirects to `.../edit`.
//!
//! 308 (not 301) keeps a hypothetical POST against the legacy
//! URL with `?edit=1` from being downgraded to GET — symmetric
//! with the `/me` → `/today` migration that landed in 0.17.0.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;

#[tokio::test]
async fn detail_url_renders_read_only() {
    // The bare detail URL must NOT show the edit form. The
    // simplest probe is "no `<form action='.../issues/...' method='post'>`
    // at the page level" — read-only renders only the
    // navigation links and the issue body.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // Edit affordance must be a link to /edit, not the form
    // itself. Looking for the IssueEditForm's distinctive
    // `client_updated_at` hidden input is the cleanest probe —
    // it's only present in edit mode.
    assert!(
        !body.contains(r#"name="client_updated_at""#),
        "detail URL must not render the edit form (client_updated_at hidden input present)"
    );
    // The "Edit" link should point to the new explicit URL.
    let expected_edit_link = format!("/projects/{project_id}/issues/{issue_id}/edit");
    assert!(
        body.contains(&expected_edit_link),
        "detail page should link to the explicit /edit URL; \
         expected substring {expected_edit_link:?}"
    );
}

#[tokio::test]
async fn edit_url_renders_edit_form() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}/edit");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // The optimistic-lock hidden input is a stable marker for
    // "edit form is rendered."
    assert!(
        body.contains(r#"name="client_updated_at""#),
        "edit URL must render the edit form; \
         client_updated_at hidden input absent"
    );
}

#[tokio::test]
async fn legacy_edit_query_redirects_with_308() {
    // `?edit=1` → 308 to `/edit`. Bookmarks and external links
    // from before 0.18.0 still work.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let url = format!("/projects/{project_id}/issues/{issue_id}?edit=1");
    let resp = app.server.get(&url).await;
    assert_eq!(
        resp.status_code(),
        StatusCode::PERMANENT_REDIRECT,
        "?edit=1 should 308-redirect to /edit; got {}",
        resp.status_code()
    );

    // Location header points at the new URL (path only — most
    // axum response builders emit a path-only Location for
    // same-origin redirects).
    let location = resp
        .headers()
        .get("location")
        .expect("Location header on 308")
        .to_str()
        .expect("Location header value");
    let expected_target = format!("/projects/{project_id}/issues/{issue_id}/edit");
    assert_eq!(
        location, expected_target,
        "Location header should point at /edit"
    );
}
