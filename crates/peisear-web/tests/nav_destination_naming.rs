//! `FLOW-002` (`059`, unit two) — "the word on the door is the name
//! inside": for all five of `FR-NAV-001`'s destinations, the primary
//! nav's own word and the page the reader actually lands on must
//! answer to the same name.
//!
//! **Checked against the real rendered `<title>`, not an `<h1>` and
//! not a hand-copied pairing table.** `<h1>` was considered and
//! rejected: `/search`'s own heading is conditional on a query and
//! absent on arrival with none, so an `<h1>`-scan would have nothing
//! to read on the one destination that most needs checking. `<title>`
//! always exists, and after `FLOW-002`'s own Rule 1
//! (`page_title_scan.rs`) every title's content is the bare page name
//! plus the product suffix stripped off here before comparing.
//!
//! **A pairing table was tried first and rejected by its own plant**:
//! an earlier version of this check lived in `page_title_scan.rs` as
//! a `const` array of `MessageKey` pairs I chose myself by reading
//! each component file once. Reverting `notifications.rs`'s own fix
//! during this round's plant left it green, because it never read
//! `notifications.rs` at all — "two literals that happen to match
//! today," one level removed, exactly what the handoff's own
//! acceptance warned against. This reads the live page instead.
//!
//! `Today` was excluded here and pinned separately, as a pure
//! `MessageKey` comparison in `page_title_scan.rs`, until `NAME-001`
//! (`060`) closed the gap: the nav word is now `Me`, which
//! `/today`'s own title already answers to, so the pending pin is
//! deleted and this file gains the fifth case instead.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::server::TestApp;
use peisear_i18n::{Locale, MessageKey};

fn rendered_title(body: &str) -> &str {
    let start = body
        .find("<title>")
        .map(|i| i + "<title>".len())
        .expect("<title> present");
    let end = body[start..]
        .find("</title>")
        .map(|i| start + i)
        .expect("<title> has a closing </title>");
    body[start..end].trim()
}

/// Strips `FLOW-002`'s own product suffix, so the comparison is
/// against the bare page name only -- the property under test is
/// "does the destination answer to the nav word," not "does it also
/// carry the brand," which `page_title_scan.rs` already covers.
fn without_brand_suffix(title: &str) -> &str {
    let brand = Locale::English.render(MessageKey::AppBrandName);
    let suffix = format!(" — {brand}");
    title
        .strip_suffix(suffix.as_str())
        .unwrap_or_else(|| panic!("title {title:?} does not carry the product suffix {suffix:?}"))
}

async fn assert_destination_title_matches_nav_word(path: &str, nav_key: MessageKey) {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let _user_id = register_and_login(&app, &user).await;

    let resp = app.server.get(path).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();
    let title = without_brand_suffix(rendered_title(&body));
    let expected = Locale::English.render(nav_key);

    assert_eq!(
        title, expected,
        "{path}'s own <title> ({title:?}) does not match the primary nav's own word ({expected:?})"
    );
}

#[tokio::test]
async fn inbox_answers_to_its_own_nav_word() {
    assert_destination_title_matches_nav_word("/inbox", MessageKey::NavLinkInbox).await;
}

#[tokio::test]
async fn projects_answers_to_its_own_nav_word() {
    assert_destination_title_matches_nav_word("/projects", MessageKey::NavLinkProjects).await;
}

#[tokio::test]
async fn teams_answers_to_its_own_nav_word() {
    assert_destination_title_matches_nav_word("/teams", MessageKey::NavLinkTeams).await;
}

/// Landed on with no query -- the state a reader actually reaches by
/// clicking `Search` in the primary nav, as opposed to arriving with
/// `?q=...` already filled in from the typeahead or a bookmark.
#[tokio::test]
async fn search_answers_to_its_own_nav_word_on_arrival_with_no_query() {
    assert_destination_title_matches_nav_word("/search", MessageKey::NavLinkSearch).await;
}

/// `NAME-001` (`060`): the fifth destination, closing the gap
/// `page_title_scan.rs`'s own deleted pending pin left open. The nav
/// word moved to `Me`; `/today`'s own title was already
/// `PersonalDashboardTitle`, which this round also renders `Me` --
/// the live page, not a pairing table, is what proves they agree.
#[tokio::test]
async fn today_answers_to_its_own_nav_word() {
    assert_destination_title_matches_nav_word("/today", MessageKey::NavLinkToday).await;
}
