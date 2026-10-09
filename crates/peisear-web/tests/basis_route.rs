//! `HLT-001` (RFC 008 §1–§3) — the basis route.
//!
//! Each indicator's explanation row gets a link to a route
//! rendering exactly the issues behind its count — not a filter
//! that reconstructs the set, the same membership
//! `project_health::for_project` already computed
//! (`ProjectHealthRaw::basis_for`). `WipCompliance` gets none,
//! structurally (RFC 008 §2/§2.3): its basis is users, not issues.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;
use peisear_core::{IssueStatus, Priority};
use peisear_storage::issues::IssueFields;

/// Scope a body-wide assertion to just the explanation `<ul>` — the
/// navbar's own account menu legitimately shows the logged-in
/// user's email, which is not the leak `HLT-001` §3 guards against.
fn extract_explanation_list(body: &str) -> &str {
    const OPEN: &str = r#"<ul class="mt-2 ml-4 list-disc"#;
    let Some(start) = body.find(OPEN) else {
        return ""; // no explanations rendered at all -- nothing to scope
    };
    let end = body[start..]
        .find("</ul>")
        .expect("unterminated explanation <ul>");
    &body[start..start + end]
}

/// Check 1 (`HLT-001` §3, written first): WIP compliance renders no
/// basis link, and the over-limit assignee's identity does not
/// appear anywhere in the explanation area.
#[tokio::test]
async fn wip_compliance_renders_no_basis_link_or_assignee_identity() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    // Same fixture shape as `wip_compliance_explanation_uses_
    // corrected_wording`: 4 in_progress issues assigned to one user
    // pushes them past DEFAULT_WIP_LIMIT (3), reaching Watch/Concern.
    for i in 0..4 {
        let id = uuid::Uuid::new_v4().to_string();
        peisear_storage::issues::insert(
            &app.db,
            &id,
            &project_id,
            &user_id,
            IssueFields {
                title: &format!("T{i}"),
                description: "Test issue body.",
                status: IssueStatus::InProgress,
                priority: Priority::Medium,
                effort: None,
                assignee_id: Some(&user_id),
                planned_start_at: None,
                planned_end_at: None,
            },
        )
        .await
        .expect("insert issue");
    }

    let resp = app.server.get(&format!("/projects/{project_id}")).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("active assignees are over their WIP limit."),
        "expected the WIP-compliance explanation to actually render, or \
         the rest of this test is meaningless: {body}"
    );
    assert!(
        !body.contains("What WIP compliance is based on"),
        "WIP compliance must render no basis-link accessible name: {body}"
    );
    assert!(
        !body.contains("/health/wip-compliance/basis"),
        "WIP compliance must render no basis-link href: {body}"
    );
    let explanation_area = extract_explanation_list(&body);
    assert!(
        !explanation_area.contains(&user.email),
        "no assignee identity may appear in the explanation area: {explanation_area}"
    );

    // The route itself must also refuse to render WIP compliance's
    // (nonexistent) basis if visited directly.
    let direct = app
        .server
        .get(&format!(
            "/projects/{project_id}/health/wip-compliance/basis"
        ))
        .await;
    direct.assert_status(StatusCode::NOT_FOUND);
}

/// Check 2: the five linked indicators each render a basis link with
/// a distinguishing accessible name (`board_keyboard`'s `each_
/// status_control_has_a_distinguishing_accessible_name` precedent),
/// and following it lands on a page naming that indicator.
///
/// `HLT-003`: widened from Throughput alone. The plural name promised
/// all five and the body checked one -- `§10.17`'s shape in a test
/// name, the same defect `DM-TEST-002` renamed `undo_dom_order` for.
///
/// **The basis link needs two things, not one**: `render_explanation_row`
/// (`components/issues.rs`) only renders a basis link on an
/// *explanation row*, and an indicator contributes one only when it is
/// *not* at `Good` (`human_explanation`) -- a non-empty `basis_for` set
/// alone is not enough, as the first draft of this fixture found by
/// running it and reading the body: staleness and activity both had
/// non-empty bases (one in-flight issue; four recently-created ones)
/// but no explanation row, because both were `Good`.
///
/// So the fixture pushes all five into `Watch` from one combined
/// change: one of the four open issues gets *both* `created_at` and
/// `updated_at` back-dated past the activity window / long-stale
/// threshold (they are numerically the same constant, `ACTIVITY_WINDOW_DAYS`
/// == `LONG_STALE_THRESHOLD_DAYS`). That single issue becomes
/// simultaneously: the oldest in-flight issue (staleness reaches
/// Watch at 14+ days), long-stale on its own (updated_at past the
/// threshold), and absent from the activity window -- dropping
/// `recent_activity_count` from 5 to 4, which crosses Activity's own
/// Watch floor. Throughput (one done of five) and bus factor (one
/// assignee) were already `Watch` in the un-widened fixture. `WipCompliance`
/// stays excluded, structurally (`basis_for` returns `None` for it)
/// and covered by its own test above.
#[tokio::test]
async fn linked_indicators_render_distinguishing_basis_links() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    // One done issue, four open: throughput is 1/5 = 20% (below the
    // 30% Watch floor), with a non-empty done set to link to. The
    // four open issues are in-flight, so bus factor's and staleness's
    // bases are already non-empty; they are also freshly created, so
    // activity's basis is non-empty too.
    let done_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &done_id,
        &project_id,
        &user_id,
        IssueFields {
            title: "Done",
            description: "",
            status: IssueStatus::Done,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert done issue");
    let mut open_ids = Vec::new();
    for i in 0..4 {
        open_ids.push(create_issue(&app.db, &project_id, &user_id, &format!("Open {i}")).await);
    }
    // Back-date one open issue's `created_at` AND `updated_at` past
    // the activity window / long-stale threshold. Neither column
    // carries a maintaining trigger that would overwrite an explicit
    // `UPDATE` (`updated_at`'s, per `NFR-CONC-003`/`COV-001`, only
    // fires `WHEN OLD.updated_at = NEW.updated_at`; `created_at` has
    // no trigger at all). This single change pushes staleness,
    // activity and long-stale into `Watch` together -- see the doc
    // comment above for why all three, not one at a time.
    let past_threshold = chrono::Utc::now()
        - chrono::Duration::days(peisear_core::project_health::LONG_STALE_THRESHOLD_DAYS + 1);
    sqlx::query("UPDATE issues SET created_at = ?1, updated_at = ?1 WHERE id = ?2")
        .bind(past_threshold)
        .bind(&open_ids[0])
        .execute(&app.db)
        .await
        .expect("back-date created_at and updated_at");

    let resp = app.server.get(&format!("/projects/{project_id}")).await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    for (slug, label) in [
        ("throughput", "Throughput"),
        ("staleness", "Oldest in-flight"),
        ("activity", "Activity (14d)"),
        ("bus-factor", "Bus factor"),
        ("long-stale", "Long-stale"),
    ] {
        let aria = format!(r#"aria-label="What {label} is based on""#);
        assert!(
            body.contains(&aria),
            "expected {label}'s basis link with a distinguishing accessible \
             name ({aria:?}); body: {body}"
        );
        let href = format!("/projects/{project_id}/health/{slug}/basis");
        assert!(
            body.contains(&href),
            "expected {label}'s basis link href ({href:?}); body: {body}"
        );

        let basis_resp = app.server.get(&href).await;
        basis_resp.assert_status(StatusCode::OK);
        let basis_body = basis_resp.text();
        assert!(
            basis_body.contains(label),
            "expected the basis page to name {label}; body: {basis_body}"
        );
    }
}

/// Check 3: throughput's basis set is exactly the done issues — not
/// the open ones, not a superset.
#[tokio::test]
async fn throughput_basis_is_exactly_the_done_issues() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    let done_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &done_id,
        &project_id,
        &user_id,
        IssueFields {
            title: "Done issue",
            description: "",
            status: IssueStatus::Done,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert done issue");
    let _open_id = create_issue(&app.db, &project_id, &user_id, "Open issue").await;

    let basis_resp = app
        .server
        .get(&format!("/projects/{project_id}/health/throughput/basis"))
        .await;
    basis_resp.assert_status(StatusCode::OK);
    let body = basis_resp.text();

    assert!(
        body.contains("Done issue"),
        "the done issue must appear in throughput's basis: {body}"
    );
    assert!(
        !body.contains("Open issue"),
        "the open issue must NOT appear in throughput's basis -- a link to \
         the wrong issues is worse than no link: {body}"
    );
}

/// Check 4: staleness's basis is exactly the single oldest in-flight
/// issue, not the whole in-flight set.
#[tokio::test]
async fn staleness_basis_is_exactly_the_oldest_in_flight_issue() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    let _older = create_issue(&app.db, &project_id, &user_id, "Older issue").await;
    common::server::ensure_distinct_timestamp().await;
    let _newer = create_issue(&app.db, &project_id, &user_id, "Newer issue").await;

    let basis_resp = app
        .server
        .get(&format!("/projects/{project_id}/health/staleness/basis"))
        .await;
    basis_resp.assert_status(StatusCode::OK);
    let body = basis_resp.text();

    assert!(
        body.contains("Older issue"),
        "staleness's basis must be the older (first-created) issue: {body}"
    );
    assert!(
        !body.contains("Newer issue"),
        "staleness's basis must be exactly one issue, not the whole \
         in-flight set: {body}"
    );
}

/// `HLT-003` (`FR-HLT-007`, `§10.35`). `Activity`, `BusFactor` and
/// `LongStale` had zero test references -- `REQ-005` reported this
/// with the wrong needle (underscored slugs; the real ones are
/// hyphenated, `peisear-core/src/lib.rs:902`), and re-checking with the
/// right one leaves the conclusion unchanged. `basis_for` forks per
/// kind (`:713`) -- a different field for each -- so these are three
/// separate untested computations, not one shared path sampled once
/// the way `FR-SUB-006` was.
///
/// Each test follows `throughput_basis_is_exactly_the_done_issues`'s
/// shape: assert the matching issue is present *and* the non-matching
/// one is absent, because a basis route that lists every issue in the
/// project would pass a presence-only check.
///
/// Check 6: activity's basis is exactly the recently-touched issues --
/// not an issue whose own `created_at` falls outside the window.
/// `created_at` carries no maintaining trigger (unlike `updated_at`,
/// `NFR-CONC-003`), so a direct `UPDATE` back-dating it is not
/// overwritten by anything.
#[tokio::test]
async fn activity_basis_is_exactly_the_recently_touched_issues() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    let _recent_id = create_issue(&app.db, &project_id, &user_id, "Recent issue").await;
    let old_id = create_issue(&app.db, &project_id, &user_id, "Old issue").await;
    let before_window = chrono::Utc::now()
        - chrono::Duration::days(peisear_core::project_health::ACTIVITY_WINDOW_DAYS + 1);
    sqlx::query("UPDATE issues SET created_at = ?1 WHERE id = ?2")
        .bind(before_window)
        .bind(&old_id)
        .execute(&app.db)
        .await
        .expect("back-date created_at");

    let basis_resp = app
        .server
        .get(&format!("/projects/{project_id}/health/activity/basis"))
        .await;
    basis_resp.assert_status(StatusCode::OK);
    let body = basis_resp.text();

    assert!(
        body.contains("Recent issue"),
        "activity's basis must include the recently-created issue: {body}"
    );
    assert!(
        !body.contains("Old issue"),
        "activity's basis must NOT include an issue created outside the \
         window -- a link to the wrong issues is worse than no link: {body}"
    );
}

/// Check 7: bus factor's basis is exactly the in-flight issues -- not
/// a done one. `basis_for(BusFactor)` returns `in_flight_issue_ids`
/// directly, the same set `project_health::for_project` fetches once
/// for every in-flight-derived count.
#[tokio::test]
async fn bus_factor_basis_is_exactly_the_in_flight_issues() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    let _in_flight_id = create_issue(&app.db, &project_id, &user_id, "In-flight issue").await;
    let done_id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert(
        &app.db,
        &done_id,
        &project_id,
        &user_id,
        IssueFields {
            title: "Done issue",
            description: "",
            status: IssueStatus::Done,
            priority: Priority::Medium,
            effort: None,
            assignee_id: None,
            planned_start_at: None,
            planned_end_at: None,
        },
    )
    .await
    .expect("insert done issue");

    let basis_resp = app
        .server
        .get(&format!("/projects/{project_id}/health/bus-factor/basis"))
        .await;
    basis_resp.assert_status(StatusCode::OK);
    let body = basis_resp.text();

    assert!(
        body.contains("In-flight issue"),
        "bus factor's basis must include the in-flight issue: {body}"
    );
    assert!(
        !body.contains("Done issue"),
        "bus factor's basis must NOT include a done issue -- it is not \
         in-flight work: {body}"
    );
}

/// Check 8: long-stale's basis is exactly the issues whose staleness
/// clock has crossed the threshold -- not every in-flight issue.
/// Reuses `COV-001`'s documented back-door for `updated_at`
/// (`0017_updated_at_single_authority.sql`'s trigger fires only `WHEN
/// OLD.updated_at = NEW.updated_at`, so a direct `UPDATE` that sets it
/// explicitly is not immediately overwritten) rather than rediscovering it.
#[tokio::test]
async fn long_stale_basis_is_exactly_the_long_stale_issues() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;

    let stale_id = create_issue(&app.db, &project_id, &user_id, "Stale issue").await;
    let _fresh_id = create_issue(&app.db, &project_id, &user_id, "Fresh issue").await;
    let past_threshold = chrono::Utc::now()
        - chrono::Duration::days(peisear_core::project_health::LONG_STALE_THRESHOLD_DAYS + 1);
    sqlx::query("UPDATE issues SET updated_at = ?1 WHERE id = ?2")
        .bind(past_threshold)
        .bind(&stale_id)
        .execute(&app.db)
        .await
        .expect("back-date updated_at");

    let basis_resp = app
        .server
        .get(&format!("/projects/{project_id}/health/long-stale/basis"))
        .await;
    basis_resp.assert_status(StatusCode::OK);
    let body = basis_resp.text();

    assert!(
        body.contains("Stale issue"),
        "long-stale's basis must include the issue past the threshold: {body}"
    );
    assert!(
        !body.contains("Fresh issue"),
        "long-stale's basis must NOT include an in-flight issue that has \
         not gone stale: {body}"
    );
}

/// Check 5: an unknown indicator slug 404s rather than panicking or
/// rendering something misleading.
#[tokio::test]
async fn unknown_indicator_slug_404s() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "P").await;
    let _issue_id = create_issue(&app.db, &project_id, &user_id, "T").await;

    let resp = app
        .server
        .get(&format!(
            "/projects/{project_id}/health/not-a-real-indicator/basis"
        ))
        .await;
    resp.assert_status(StatusCode::NOT_FOUND);
}
