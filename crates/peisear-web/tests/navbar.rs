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
use peisear_i18n::{Locale, MessageKey};

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
