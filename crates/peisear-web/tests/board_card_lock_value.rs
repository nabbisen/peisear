//! `LOCK-001` (`DEC-052`, `RFC 011` step 3's replacement) — asserts the
//! guarantee `board.js` has been covering for without a test.
//!
//! `board.js`'s drag handler refuses to submit a status change when
//! the dragged card carries no `data-updated-at`
//! (`!clientUpdatedAt`), reverting the drop and announcing a reload
//! message instead. That is `NFR-CONC-001`'s optimistic lock — the
//! attribute is how the board participates in it. Nothing in
//! `crates/peisear-web/tests/` asserted the server actually emits it
//! before this handoff; a refactor that stopped emitting the
//! attribute would have failed silently — the board would just quietly
//! stop accepting drags, on a page that reports nothing wrong.
//!
//! **Two shapes this test must not become**, both named in the
//! handoff because this project has been bitten by each:
//!
//! - Not `body.contains("data-updated-at")` — passes when *one* card
//!   has it. [`card_updated_at`] scopes to one named card's own tag,
//!   the same discipline `§10.17`'s eleven decayed assertions are
//!   the cautionary example for.
//! - Not a presence check — `!clientUpdatedAt` is `false` for `""`,
//!   so an empty attribute is exactly the state the JavaScript
//!   guards against. The assertion below is `!value.is_empty()`, not
//!   `body.contains(...)`.
//!
//! Two issues, in two different board columns (Open and In Progress)
//! — a single-card board can't distinguish "every card" from "a
//! card".

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::create_personal_project;
use common::server::TestApp;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::issues;

/// The `data-updated-at` attribute's value on the one board card
/// whose `data-issue-id` is `issue_id` — scoped to that card's own
/// `<div ...>` tag, not a search over the whole body, so a neighbour
/// card carrying the attribute can't make this pass for a card that
/// doesn't (`§10.17`'s own lesson).
fn card_updated_at<'a>(body: &'a str, issue_id: &str, label: &str) -> &'a str {
    let id_marker = format!(r#"data-issue-id="{issue_id}""#);
    let id_pos = body.find(&id_marker).unwrap_or_else(|| {
        panic!("no card for {label:?} (issue {issue_id}) found in body: {body}")
    });
    let tag_start = body[..id_pos]
        .rfind("<div")
        .unwrap_or_else(|| panic!("no <div precedes data-issue-id for {label:?}: {body}"));
    let tag_end = body[tag_start..]
        .find('>')
        .map(|e| tag_start + e)
        .unwrap_or_else(|| panic!("card <div> for {label:?} has no closing '>': {body}"));
    let tag = &body[tag_start..tag_end];
    let dua_marker = "data-updated-at=\"";
    let dua_start = tag.find(dua_marker).unwrap_or_else(|| {
        panic!("card for {label:?} (issue {issue_id}) carries no data-updated-at attribute at all: {tag}")
    });
    let value_start = dua_start + dua_marker.len();
    let value_end = tag[value_start..]
        .find('"')
        .map(|e| value_start + e)
        .unwrap_or_else(|| {
            panic!("data-updated-at attribute unterminated on {label:?}'s card: {tag}")
        });
    &tag[value_start..value_end]
}

#[tokio::test]
async fn every_board_card_carries_a_non_empty_lock_value() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Lock value fixture").await;

    // Two cards, two columns -- a one-card board can't distinguish
    // "every card" from "a card" (the handoff's own §3).
    let open_id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        &app.db,
        &open_id,
        &project_id,
        &user_id,
        issues::IssueFields {
            title: "Open card",
            description: "Fixture issue in the Open column.",
            status: IssueStatus::Open,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert open issue");

    let in_progress_id = uuid::Uuid::new_v4().to_string();
    issues::insert(
        &app.db,
        &in_progress_id,
        &project_id,
        &user_id,
        issues::IssueFields {
            title: "In-progress card",
            description: "Fixture issue in the In Progress column.",
            status: IssueStatus::InProgress,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert in-progress issue");

    let url = format!("/projects/{project_id}?view=board");
    let resp = app.server.get(&url).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    for (issue_id, label) in [
        (&open_id, "Open card"),
        (&in_progress_id, "In-progress card"),
    ] {
        let value = card_updated_at(&body, issue_id, label);
        assert!(
            !value.is_empty(),
            "the {label:?} card (issue {issue_id}) carries an empty data-updated-at -- \
             board.js's `!clientUpdatedAt` guard treats this exactly like a missing \
             attribute, silently refusing every drag on this card; body: {body}"
        );
    }
}
