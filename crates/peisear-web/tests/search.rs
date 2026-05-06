//! Search endpoint tests (Phase A Step 4, peisear-feature-spec
//! v2.1 §4.5).
//!
//! Two endpoints under test:
//!
//! - `GET /api/search?q=...` — JSON typeahead for the navbar.
//! - `GET /search?q=...&page=N` — HTML results page.
//!
//! Coverage focuses on the **scope and authorization invariants**
//! the spec spells out:
//!
//! - matches in projects the user can access
//! - matches in **open** issues only (status != done)
//! - LIKE meta-character escaping (a search for "100%"
//!   matches the literal `%`, not as a wildcard)
//! - the JSON shape includes URL fields the typeahead navigates to
//!
//! Cross-user isolation tests live in `auth_boundary.rs` once
//! the `/api/users/{id}/...` endpoints land in Phase B; for
//! search, the v2.1 invariant is that a user only sees their
//! own data, which we enforce via the access predicate in the
//! storage layer. Sharing a project between two users requires
//! teams (Phase C scope), so the bare minimum here is "Bob's
//! personal project doesn't appear in Alice's results."

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;

#[tokio::test]
async fn typeahead_returns_empty_for_blank_query() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/api/search?q=").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    // Empty `q` → empty arrays, not 400. The client polls this
    // endpoint on every keystroke, including the moment the
    // user clears the box.
    assert!(body.contains(r#""projects":[]"#));
    assert!(body.contains(r#""issues":[]"#));
}

#[tokio::test]
async fn typeahead_finds_project_by_name() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let _project_id =
        create_personal_project(&app.db, &user_id, "Customer Portal").await;

    let resp = app.server.get("/api/search?q=Customer").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains(r#""name":"Customer Portal""#),
        "expected project hit in typeahead JSON: {body}"
    );
    // Should also include the URL the typeahead row links to.
    assert!(
        body.contains(r#""url":"/projects/"#),
        "typeahead missing URL field: {body}"
    );
}

#[tokio::test]
async fn typeahead_finds_open_issue_by_title() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id =
        create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let _issue_id =
        create_issue(&app.db, &project_id, &user_id, "Login error on submit").await;

    let resp = app.server.get("/api/search?q=Login").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains(r#""title":"Login error on submit""#),
        "expected issue hit: {body}"
    );
    // The carried-over project name lets the dropdown show
    // "Open issue · Customer Portal" without a second round
    // trip per result.
    assert!(
        body.contains(r#""project_name":"Customer Portal""#),
        "issue hit missing project_name: {body}"
    );
}

#[tokio::test]
async fn typeahead_excludes_done_issues() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id =
        create_personal_project(&app.db, &user_id, "Project X").await;
    let _open = create_issue(&app.db, &project_id, &user_id, "alpha open").await;
    let done = create_issue(&app.db, &project_id, &user_id, "alpha done").await;
    sqlx::query(r#"UPDATE issues SET status = 'done' WHERE id = ?1"#)
        .bind(&done)
        .execute(&app.db)
        .await
        .expect("mark done");

    let resp = app.server.get("/api/search?q=alpha").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(body.contains("alpha open"), "open issue should appear");
    assert!(
        !body.contains("alpha done"),
        "done issue must be excluded per spec §4.5 (search scope = open)"
    );
}

#[tokio::test]
async fn typeahead_does_not_leak_other_users_projects() {
    // Alice and Bob have their own personal projects with
    // similar names. Alice's search must not return Bob's.
    // The scope predicate in `peisear-storage::search` is
    // exercised here.
    let alice_app = TestApp::spawn().await;
    let alice = TestUser::new("alice");
    let alice_id = register_and_login(&alice_app, &alice).await;
    let _alice_project =
        create_personal_project(&alice_app.db, &alice_id, "Alice's Portal").await;

    // Bob has a personal project. We need Bob to actually exist
    // in the DB shared by alice_app, so we register Bob via the
    // same app's HTTP surface (which goes through the same DB
    // pool). We log out first so the cookie jar doesn't carry
    // Alice's session into Bob's registration.
    common::auth::logout(&alice_app).await;
    let bob = TestUser::new("bob");
    let bob_id = register_and_login(&alice_app, &bob).await;
    let _bob_project =
        create_personal_project(&alice_app.db, &bob_id, "Bob's Portal").await;

    // Re-login as Alice for the search. axum-test's saved-cookie
    // jar carries the latest session; since Bob just logged in,
    // we need to log Alice back in.
    common::auth::logout(&alice_app).await;
    common::auth::login(&alice_app, &alice).await;

    let resp = alice_app.server.get("/api/search?q=Portal").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains("Alice's Portal"),
        "Alice should see her own project"
    );
    assert!(
        !body.contains("Bob's Portal"),
        "Alice must NOT see Bob's personal project in search results"
    );
}

#[tokio::test]
async fn typeahead_handles_like_meta_characters() {
    // A search containing `%` must match the literal `%`, not
    // act as a wildcard. Without escaping, a search for "100%"
    // would `%100%%` as the pattern and match anything
    // containing "100".
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id =
        create_personal_project(&app.db, &user_id, "Project X").await;
    // One issue with literal "%", one without.
    let _with_pct = create_issue(&app.db, &project_id, &user_id, "Done 100% test").await;
    let _without = create_issue(&app.db, &project_id, &user_id, "Done 100 test").await;

    let resp = app.server.get("/api/search?q=100%25").await; // %25 = %
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(
        body.contains("Done 100% test"),
        "should match literal %"
    );
    assert!(
        !body.contains("Done 100 test"),
        "must NOT match without %; that would mean LIKE meta wasn't escaped"
    );
}

#[tokio::test]
async fn results_page_renders_with_query() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id =
        create_personal_project(&app.db, &user_id, "Customer Portal").await;
    let _issue_id =
        create_issue(&app.db, &project_id, &user_id, "Login error").await;

    let resp = app.server.get("/search?q=Login").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    assert!(body.contains("Login error"), "results page missing issue");
    // Heading echoes the query.
    assert!(
        body.contains("Login"),
        "results page should echo query"
    );
    // Section headings present.
    assert!(body.contains("Projects"), "missing Projects section");
    assert!(body.contains("Open issues"), "missing Open issues section");
}

#[tokio::test]
async fn results_page_with_blank_query_renders_help_text() {
    // Direct navigation to /search with no query: render the
    // page so the user has a search box to type into. Don't
    // 400 — that would be hostile to anyone who bookmarks
    // /search.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/search").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    // The page should render the empty-state help text and the
    // form itself.
    assert!(
        body.contains("Enter a search term"),
        "empty-query results page missing help text"
    );
    assert!(
        body.contains(r#"action="/search""#),
        "empty-query results page missing form"
    );
}

#[tokio::test]
async fn search_endpoints_require_authentication() {
    let app = TestApp::spawn().await;
    // No login.
    // /api/search uses ApiAuthUser → returns 401 + JSON
    // (Phase B PR2). The /search HTML page still uses
    // AuthUser → 303 redirect to /login.
    let resp = app.server.get("/api/search?q=foo").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::UNAUTHORIZED,
        "/api/search unauth should be 401, got {}",
        resp.status_code()
    );

    let resp = app.server.get("/search?q=foo").await;
    assert_eq!(
        resp.status_code(),
        StatusCode::SEE_OTHER,
        "expected redirect for unauthed /search, got {}",
        resp.status_code()
    );
}
