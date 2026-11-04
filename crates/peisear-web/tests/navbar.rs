//! `NAV-001` (`FR-NAV-001`): the primary navigation's own five
//! required entries, asserted by their *rendered* `MessageKey` text,
//! scoped to the primary nav's own `<nav aria-label="Primary">` block
//! -- not the whole page.
//!
//! **Render the keys, do not quote the words** (`REQ-005`'s own
//! finding): a literal `"Today"` in this file would still pass if the
//! shipped copy drifted to something else; rendering
//! `MessageKey::NavLinkToday` and checking for *that* output fails
//! the moment the two disagree.
//!
//! **Scoped, not whole-body** (`breadcrumb.rs`'s own `TT-003` §5
//! finding, the same shape here): the collapsed `Menu` dropdown
//! duplicates all five words and hrefs for narrow viewports, so an
//! unscoped `body.contains(...)` would stay green with one of the
//! five removed from the primary row alone, as long as it survived in
//! the duplicate. Isolating the primary `<nav>`'s own block is what
//! makes the plant below actually test the row this requirement is
//! about.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::server::TestApp;
use peisear_core::notifications::{Severity, kind};
use peisear_i18n::{Locale, MessageKey};
use peisear_storage::notifications as notif_store;

/// The `<nav aria-label="Primary">...</nav>` block's own markup.
fn primary_nav(body: &str) -> &str {
    let marker = r#"aria-label="Primary""#;
    let marker_at = body.find(marker).expect("primary nav aria-label present");
    let nav_start = body[..marker_at]
        .rfind("<nav")
        .expect("a <nav tag precedes the primary nav aria-label");
    let nav_end = body[nav_start..]
        .find("</nav>")
        .map(|i| nav_start + i)
        .expect("primary nav has a closing </nav>");
    &body[nav_start..nav_end]
}

/// The bell `<a href="/inbox">`'s own `aria-label` value, within an
/// already-scoped `nav` slice (`primary_nav`'s own output). Skips
/// past the `<nav>` landmark's own `aria-label="Primary"` first, so
/// this finds the *next* one -- the bell's.
fn bell_aria_label(nav: &str) -> &str {
    let landmark = r#"aria-label="Primary""#;
    let after_landmark = nav.find(landmark).map(|i| i + landmark.len()).unwrap_or(0);
    let marker = r#"aria-label=""#;
    let rel = nav[after_landmark..]
        .find(marker)
        .expect("bell aria-label present after the nav's own landmark label");
    let start = after_landmark + rel + marker.len();
    let end = nav[start..]
        .find('"')
        .map(|i| start + i)
        .expect("aria-label has a closing quote");
    &nav[start..end]
}

/// `NAV-001` round 2: insert a notification row directly, bypassing
/// dispatch -- this test only cares that the unread count is nonzero
/// (same shape as `inbox_refinements.rs`'s own `insert_notification`).
async fn insert_notification(app: &TestApp, user_id: &str) {
    notif_store::insert(
        &app.db,
        &common::auth::subject_id_for(user_id),
        notif_store::NewNotification {
            kind: kind::BURNOUT_OVERLOAD,
            severity: Severity::Watch,
            title: "Sustained over-capacity streak",
            body: "Test body.",
            payload_json: None,
            dispatched_via: &["in_app"],
        },
    )
    .await
    .expect("insert notification");
}

/// `FR-NAV-001`'s own five, rendered rather than quoted.
fn required_entries() -> [String; 5] {
    [
        Locale::English.render(MessageKey::NavLinkToday),
        Locale::English.render(MessageKey::NavLinkInbox),
        Locale::English.render(MessageKey::NavLinkProjects),
        Locale::English.render(MessageKey::NavLinkTeams),
        Locale::English.render(MessageKey::NavLinkSearch),
    ]
}

#[tokio::test]
async fn the_five_required_entries_are_all_visible_in_the_primary_nav() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let _user_id = register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    let nav = primary_nav(&body);

    for entry in required_entries() {
        assert!(
            nav.contains(&entry),
            "the primary nav must carry {entry:?}: {nav}"
        );
    }
}

/// `FR-NAV-001` names exactly five; `Settings` is not one of them and
/// `NAV-001` `§0` moves it to the account menu specifically so it is
/// not read as a sixth entry in this row.
#[tokio::test]
async fn settings_is_not_among_the_five() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let _user_id = register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    let body = resp.text();
    let nav = primary_nav(&body);

    let settings = Locale::English.render(MessageKey::NavLinkSettings);
    assert!(
        !nav.contains(&settings),
        "Settings must not appear in the primary nav: {nav}"
    );
}

/// `NAV-001` round 2, `WCAG 2.5.3` (Label in Name): the bell's
/// accessible name must *contain* its own visible label
/// (`NavLinkInbox`, "Inbox") -- round 1 gave the bell a visible word
/// without checking that the name already in place still agreed with
/// it. No unread notifications: `NavBellLabelNone`.
#[tokio::test]
async fn the_bells_accessible_name_contains_its_visible_label_with_no_unread() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let _user_id = register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    let body = resp.text();
    let nav = primary_nav(&body);

    let visible = Locale::English.render(MessageKey::NavLinkInbox);
    let accessible = bell_aria_label(nav);
    assert!(
        accessible.contains(&visible),
        "the bell's accessible name {accessible:?} must contain its visible label {visible:?}"
    );
}

/// Same contract, the other reachable shape: `NavBellLabelUnread`,
/// which also names the count -- a reader who cannot see the badge
/// still needs it.
///
/// **`/teams`, not `/today`**: `handlers::me::page` (the `/today`
/// handler) never calls `unread_count_for_user` at all and always
/// passes `AppShell`'s own `unread_count` default of `0` -- found
/// while writing this test, not something this round introduced or
/// is fixing. `/teams`'s own `list_page` does thread it, correctly,
/// so this test exercises the shape it names rather than a page that
/// cannot produce it. Reported, not silently worked around; the
/// standing exception (`058`'s own README) is why it earns a round
/// rather than a fix folded in here.
#[tokio::test]
async fn the_bells_accessible_name_contains_its_visible_label_with_unread() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    insert_notification(&app, &user_id).await;

    let resp = app.server.get("/teams").await;
    let body = resp.text();
    let nav = primary_nav(&body);

    let visible = Locale::English.render(MessageKey::NavLinkInbox);
    let accessible = bell_aria_label(nav);
    assert!(
        accessible.contains(&visible),
        "the bell's accessible name {accessible:?} must contain its visible label {visible:?}"
    );
    assert!(
        accessible.contains("unread"),
        "the unread shape must still name the count for a reader who cannot see the badge: {accessible:?}"
    );
}
