//! Phase B PR3 (B-1) `/today` panel collapsing + "what to read
//! first" callout tests.
//!
//! What the page should look like after PR3:
//!
//! 1. The "Right now" section (WIP / Load) is always visible —
//!    most-actionable, smallest surface.
//! 2. The "Rhythm" section (Throughput / Long-stale / Pace) is
//!    folded inside `<details>` and closed by default.
//! 3. The "Sustainability" / burnout panel keeps its existing
//!    self-folding behaviour (auto-opens when there's a watch
//!    signal).
//! 4. A callout — "what to read first" — appears above the
//!    Right now section ONLY when the priority chain in
//!    `compute_read_first` matches:
//!    - sustained burnout; or
//!    - WIP over the user's effective limit; or
//!    - long-stale issues count > 0.
//!    On a baseline empty/healthy account no callout renders.
//!
//! For a freshly-registered user with no issues and no
//! capacity, the callout should not appear (long_stale_count =
//! 0, current_wip = 0). This is the default-quiet behaviour we
//! want.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::server::TestApp;

#[tokio::test]
async fn today_renders_with_right_now_panel_visible() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // "Right now" section is always-visible — its heading is
    // outside any <details> wrapper. Spot-check via the heading
    // text + aria-label.
    assert!(
        body.contains(r#"aria-label="Current load""#),
        "Right now section missing"
    );
    assert!(body.contains(">Right now<"), "Right now heading missing");
}

#[tokio::test]
async fn today_folds_rhythm_panel_by_default() {
    // Rhythm is inside a <details> with no `open` attribute.
    // Browsers render <details> closed by default, so the
    // section's labels (Throughput / Long-stale / Pace) are
    // present in the HTML (so screen readers can find them
    // and the user can click to expand) but the visual surface
    // starts collapsed.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    // Locate the Rhythm <summary> — distinctive aria-label.
    assert!(
        body.contains("Rhythm — open to see throughput"),
        "Rhythm summary aria-label missing"
    );

    // Inside Rhythm: Throughput is one of the chips. Confirms
    // the content rendered (just hidden visually).
    assert!(
        body.contains("Throughput"),
        "Throughput chip absent from Rhythm"
    );

    // Confirm `<details>` doesn't have `open` attribute on
    // the Rhythm section. We look for the specific summary
    // aria-label and check the surrounding <details> tag.
    // A simple scan: there's a `<details ` immediately
    // preceding the summary; if that opening tag carries
    // `open`, the test fails.
    if let Some(idx) = body.find("Rhythm — open to see throughput") {
        // Walk backwards to find the most recent `<details`.
        let prefix = &body[..idx];
        let last_details = prefix.rfind("<details").expect("details tag before Rhythm summary");
        // The full opening tag ends at the next `>`.
        let tag_end = prefix[last_details..]
            .find('>')
            .map(|n| last_details + n)
            .expect("details tag should close");
        let opening_tag = &prefix[last_details..tag_end];
        assert!(
            !opening_tag.contains(" open"),
            "Rhythm <details> should be closed by default; found open attribute: {opening_tag:?}"
        );
    }
}

#[tokio::test]
async fn today_renders_no_callout_for_fresh_user() {
    // A fresh user — no issues, no capacity, no burnout
    // history — should see the dashboard without a "what to
    // read first" callout. This is the default-quiet
    // behaviour from V2.1 §0.3.
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        !body.contains(r#"aria-label="What to read first""#),
        "fresh user should not see a 'what to read first' callout"
    );
}
