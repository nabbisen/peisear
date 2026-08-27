//! `JS-003` (RFC 011 step 2) — one authority for response
//! classification. `dm.js`'s and `board.js`'s copy islands now carry
//! an `outcomes` object (`conflict`/`unavailable`/`unconfirmed`, each
//! with a `message` and a `reload` flag, plus `conflictStatus`)
//! instead of the classification being written three times across the
//! two scripts. These tests check the *data* the islands render, not
//! the scripts' own consumption of it — `§10.15` still holds that.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;

/// Extract a `<script type="application/json" id="{id}">…</script>`
/// island's JSON content from a rendered page body.
fn extract_island(body: &str, id: &str) -> serde_json::Value {
    let marker = format!(r#"id="{id}""#);
    let start = body
        .find(&marker)
        .unwrap_or_else(|| panic!("island id={id:?} not found in body: {body}"));
    let after_marker = &body[start..];
    let content_start = after_marker
        .find('>')
        .expect("island opening tag closes with `>`")
        + 1;
    let content = &after_marker[content_start..];
    let end = content
        .find("</script>")
        .expect("island script tag has a closing </script>");
    serde_json::from_str(&content[..end]).unwrap_or_else(|e| {
        panic!(
            "island id={id:?} is not valid JSON: {e}\ncontent: {}",
            &content[..end]
        )
    })
}

/// Check 1: both islands carry `outcomes` with all three keys, each
/// with a non-empty `message`, and a numeric `conflictStatus`.
#[tokio::test]
async fn both_islands_carry_outcomes_with_all_three_keys() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let detail_resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}"))
        .await;
    detail_resp.assert_status(StatusCode::OK);
    let dm_island = extract_island(&detail_resp.text(), "status-enhancement-copy");
    assert_outcomes_shape(&dm_island, "dm.js");

    let board_resp = app
        .server
        .get(&format!("/projects/{project_id}?view=board"))
        .await;
    board_resp.assert_status(StatusCode::OK);
    let board_island = extract_island(&board_resp.text(), "board-copy");
    assert_outcomes_shape(&board_island, "board.js");
}

fn assert_outcomes_shape(island: &serde_json::Value, surface: &str) {
    assert!(
        island["outcomes"]["conflictStatus"].as_u64().is_some(),
        "{surface}'s island must carry a numeric outcomes.conflictStatus: {island}"
    );
    for key in ["conflict", "unavailable", "unconfirmed"] {
        let message = island["outcomes"][key]["message"].as_str();
        assert!(
            message.is_some_and(|m| !m.is_empty()),
            "{surface}'s island must carry a non-empty outcomes.{key}.message: {island}"
        );
        assert!(
            island["outcomes"][key]["reload"].as_bool().is_some(),
            "{surface}'s island must carry a boolean outcomes.{key}.reload: {island}"
        );
    }
}

/// Check 2: `outcomes.conflictStatus` equals the status a real
/// optimistic-lock conflict actually produces. Routed through the
/// same path a real response takes — a genuine stale
/// `client_updated_at` against the live status-change endpoint — not
/// a hand-constructed `AppError` compared against itself.
#[tokio::test]
async fn conflict_status_matches_a_real_409() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let resp = app
        .server
        .post(&format!("/projects/{project_id}/issues/{issue_id}/status"))
        .json(&serde_json::json!({
            "status": "in_progress",
            "client_updated_at": "1970-01-01T00:00:00Z",
        }))
        .await;
    resp.assert_status(StatusCode::CONFLICT);
    let real_status = resp.status_code().as_u16();

    let page = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}"))
        .await;
    let island = extract_island(&page.text(), "status-enhancement-copy");
    let declared = island["outcomes"]["conflictStatus"]
        .as_u64()
        .expect("conflictStatus is a number") as u16;

    assert_eq!(
        declared, real_status,
        "the island's declared conflictStatus must equal what a real optimistic-lock \
         conflict actually returns, not a value that merely happens to match today"
    );
}

/// Check 3 (the assertion that would have failed on `board.js`'s
/// pre-`JS-003` silent `return`): the board island's `unconfirmed`
/// message is non-empty.
#[tokio::test]
async fn board_unconfirmed_message_is_non_empty() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let _issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let resp = app
        .server
        .get(&format!("/projects/{project_id}?view=board"))
        .await;
    resp.assert_status(StatusCode::OK);
    let island = extract_island(&resp.text(), "board-copy");
    let message = island["outcomes"]["unconfirmed"]["message"]
        .as_str()
        .unwrap_or("");

    assert!(
        !message.trim().is_empty(),
        "board.js's unconfirmed outcome must carry a real message -- board.js's \
         pre-JS-003 silent `return` at lines 135/220 announced nothing at all: {island}"
    );
}

/// Fetch both islands for a fresh fixture project. Shared by checks 4
/// and 5 below, which both need to inspect `reload` on every island.
async fn both_islands() -> (serde_json::Value, serde_json::Value) {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let detail_resp = app
        .server
        .get(&format!("/projects/{project_id}/issues/{issue_id}"))
        .await;
    let dm_island = extract_island(&detail_resp.text(), "status-enhancement-copy");

    let board_resp = app
        .server
        .get(&format!("/projects/{project_id}?view=board"))
        .await;
    let board_island = extract_island(&board_resp.text(), "board-copy");

    (dm_island, board_island)
}

/// Check 4 (round 2, `JS-003-review.md` §2): `conflict.reload` is
/// `true` on every island. A `409` means the client's lock value is
/// stale by definition; continuing without re-fetching guarantees the
/// next write conflicts too.
#[tokio::test]
async fn conflict_outcome_always_reloads() {
    let (dm_island, board_island) = both_islands().await;
    for (island, surface) in [(&dm_island, "dm.js"), (&board_island, "board.js")] {
        assert_eq!(
            island["outcomes"]["conflict"]["reload"].as_bool(),
            Some(true),
            "{surface}'s conflict outcome must reload -- a stale lock value \
             guarantees the next write conflicts too: {island}"
        );
    }
}

/// Check 5 (round 2, `JS-003-review.md` §2): `unconfirmed.reload` is
/// `true` on every island. The mutation's outcome is genuinely
/// unknown; re-fetching is the only way to learn it -- not reloading
/// leaves the interface asserting a state it does not know to be
/// true.
///
/// `unavailable.reload` is deliberately **not** asserted anywhere in
/// this file: it is legitimately per-surface (`board.js` can revert
/// its card in place and skip the reload; `dm.js`'s undo has neither
/// a card nor a form left to fall back to, so it always reloads).
/// Pinning it would restate the code, not check an invariant --
/// `conflict` and `unconfirmed` both carry a reason for `reload=true`
/// that holds independently of what either script happens to do
/// today; `unavailable` does not.
#[tokio::test]
async fn unconfirmed_outcome_always_reloads() {
    let (dm_island, board_island) = both_islands().await;
    for (island, surface) in [(&dm_island, "dm.js"), (&board_island, "board.js")] {
        assert_eq!(
            island["outcomes"]["unconfirmed"]["reload"].as_bool(),
            Some(true),
            "{surface}'s unconfirmed outcome must reload -- the mutation's fate is \
             unknown, and re-fetching is the only way to learn it: {island}"
        );
    }
}
