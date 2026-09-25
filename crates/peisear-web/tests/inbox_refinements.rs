//! `INBOX-001` (RFC 003) — the three inbox refinements.
//!
//! 1. The silence-resume banner, triggered on
//!    `all_kinds_silenced`, never `global_acknowledged`. Test 2
//!    is this RFC's own regression guard — its history (see
//!    `rfcs/handoffs/003-inbox-refinements/README.md`) is that
//!    an earlier version of this RFC triggered the banner on the
//!    wrong predicate, and a test written from that RFC would
//!    have agreed with it and been wrong too. Written first and
//!    demonstrated failing against a deliberate
//!    `global_acknowledged`-triggered implementation before the
//!    real one landed.
//! 2. The email opt-in prompt, moved to `/inbox`, gated on the
//!    user having received at least one notification.
//! 3. Sub-issue search results naming their parent, added to the
//!    same query via a `LEFT JOIN` rather than a second batched
//!    fetch.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::fixture::{create_issue, create_personal_project};
use common::server::TestApp;
use peisear_core::notifications::{Severity, kind};
use peisear_i18n::{Locale, MessageKey};
use peisear_notify::dispatch::DispatchContext;
use peisear_notify::{DispatchEvent, DispatchTx, dispatch_loop};
use peisear_storage::notifications as notif_store;
use std::time::Duration;
use tokio::sync::mpsc;

/// Insert a sub-issue under an existing parent via storage. Same
/// helper as `sub_issues.rs`'s `insert_sub_issue_via_storage` —
/// kept local rather than shared, matching that file's own
/// precedent.
async fn insert_sub_issue_via_storage(
    app: &TestApp,
    project_id: &str,
    parent_id: &str,
    author_id: &str,
    title: &str,
) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    peisear_storage::issues::insert_sub_issue(
        &app.db,
        &id,
        project_id,
        parent_id,
        author_id,
        title,
        "",
        peisear_core::IssueStatus::Open,
        peisear_core::Priority::Medium,
        None,
        None,
    )
    .await
    .expect("insert sub-issue");
    id
}

/// Insert a notification row directly, bypassing dispatch. Tests
/// that only care about "the user has received a notification"
/// (the email prompt's gate) don't need the dispatch pipeline in
/// the loop.
async fn insert_notification(app: &TestApp, user_id: &str) {
    notif_store::insert(
        &app.db,
        user_id,
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

/// Spawn a real dispatch loop against `app`'s pool. Same pattern
/// as `peisear-notify/tests/dispatch_integration.rs`; test 4 needs
/// the real pipeline (preference resolution + cooldown), not a
/// storage-level shortcut, since it's asserting on what dispatch
/// actually does with a silenced vs. resumed user.
fn spawn_dispatch(app: &TestApp) -> DispatchTx {
    let ctx = DispatchContext {
        db: app.db.clone(),
        smtp: None,
    };
    let (tx, rx) = mpsc::channel::<DispatchEvent>(8);
    tokio::spawn(dispatch_loop(ctx, rx));
    tx
}

fn make_event(user_id: &str) -> DispatchEvent {
    DispatchEvent {
        user_id: user_id.to_string(),
        kind: kind::BURNOUT_OVERLOAD.to_string(),
        severity: Severity::Watch,
        title: "Sustained over-capacity streak".to_string(),
        body: "Test body for integration test.".to_string(),
        payload_json: None,
    }
}

async fn count_notifications(app: &TestApp, user_id: &str) -> usize {
    notif_store::recent_for_user(&app.db, user_id, 10)
        .await
        .expect("query notifications")
        .len()
}

async fn wait_until_notification_count(app: &TestApp, user_id: &str, expected: usize) {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        if count_notifications(app, user_id).await == expected {
            return;
        }
        if std::time::Instant::now() > deadline {
            panic!(
                "notification count for {user_id} never reached {expected} within 10s (last: {})",
                count_notifications(app, user_id).await
            );
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::test]
async fn banner_absent_by_default_present_after_silence_absent_after_resume() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resume_button = Locale::English.render(MessageKey::ResumeNotificationsButton);

    let resp = app.server.get("/inbox").await;
    resp.assert_status(StatusCode::OK);
    assert!(
        !resp.text().contains(&resume_button),
        "banner should be absent by default"
    );

    let resp = app.server.post("/settings/notifications/silence-all").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let resp = app.server.get("/inbox").await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains(&resume_button),
        "banner should appear once every user-facing kind is silenced"
    );

    let resp = app.server.post("/inbox/resume").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let resp = app.server.get("/inbox").await;
    resp.assert_status(StatusCode::OK);
    assert!(
        !resp.text().contains(&resume_button),
        "banner should disappear after resume"
    );
}

/// Test 2, this RFC's regression guard: a user who has answered
/// the email prompt (`set_global_acknowledged`) and silenced
/// nothing must not see the resume banner. The old RFC text
/// would have triggered on `global_acknowledged` here and failed
/// this exact test — see this file's module doc.
#[tokio::test]
async fn banner_does_not_trigger_on_global_acknowledged_alone() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    notif_store::set_global_acknowledged(&app.db, &user_id, true)
        .await
        .expect("set global acknowledged");

    let resume_button = Locale::English.render(MessageKey::ResumeNotificationsButton);
    let resp = app.server.get("/inbox").await;
    resp.assert_status(StatusCode::OK);
    assert!(
        !resp.text().contains(&resume_button),
        "a user who answered the email prompt and silenced nothing must not see the resume banner"
    );
}

#[tokio::test]
async fn resume_deletes_the_row_for_every_user_facing_kind() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    let resp = app.server.post("/settings/notifications/silence-all").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    for k in kind::all_user_facing() {
        let pref = notif_store::preference_for_user_kind(&app.db, &user_id, k)
            .await
            .expect("query preference");
        assert!(
            pref.is_some(),
            "silence_all should have written a row for {k}"
        );
    }

    let resp = app.server.post("/inbox/resume").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    for k in kind::all_user_facing() {
        let pref = notif_store::preference_for_user_kind(&app.db, &user_id, k)
            .await
            .expect("query preference");
        assert!(
            pref.is_none(),
            "resume should delete the row for {k}, found {pref:?}"
        );
    }
}

#[tokio::test]
async fn resumed_user_receives_a_dispatch_a_silenced_user_does_not() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    for k in kind::all_user_facing() {
        notif_store::upsert_preference(&app.db, &user_id, k, &[], Severity::Info)
            .await
            .expect("silence kind");
    }

    let tx = spawn_dispatch(&app);
    tx.send(make_event(&user_id)).await.unwrap();
    // Give the dispatch loop a moment to process; since channels
    // are empty, process_event returns "skipped" before writing a
    // row, so there is nothing to poll for arriving — only that
    // nothing arrives.
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(
        count_notifications(&app, &user_id).await,
        0,
        "a fully silenced user must not receive a dispatch"
    );

    let resp = app.server.post("/inbox/resume").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    tx.send(make_event(&user_id)).await.unwrap();
    wait_until_notification_count(&app, &user_id, 1).await;
}

#[tokio::test]
async fn email_prompt_shown_with_a_notification_absent_without() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;

    let prompt_heading = Locale::English.render(MessageKey::EmailNotificationsHeading);

    let resp = app.server.get("/inbox").await;
    resp.assert_status(StatusCode::OK);
    assert!(
        !resp.text().contains(&prompt_heading),
        "prompt must not show for a never-prompted user with zero notifications"
    );

    insert_notification(&app, &user_id).await;

    let resp = app.server.get("/inbox").await;
    resp.assert_status(StatusCode::OK);
    assert!(
        resp.text().contains(&prompt_heading),
        "prompt should show once the user has received at least one notification"
    );
}

#[tokio::test]
async fn either_email_opt_in_answer_records_it_and_the_prompt_does_not_return() {
    let prompt_heading = Locale::English.render(MessageKey::EmailNotificationsHeading);

    for answer in ["yes", "no"] {
        let app = TestApp::spawn().await;
        let user = TestUser::new("alice");
        let user_id = register_and_login(&app, &user).await;
        insert_notification(&app, &user_id).await;

        let resp = app.server.get("/inbox").await;
        assert!(
            resp.text().contains(&prompt_heading),
            "prompt should show before any answer (answer={answer})"
        );

        let resp = app
            .server
            .post("/inbox/email-opt-in")
            .form(&[("email_opt_in", answer)])
            .await;
        resp.assert_status(StatusCode::SEE_OTHER);

        assert!(
            notif_store::global_acknowledged(&app.db, &user_id)
                .await
                .expect("query global_acknowledged"),
            "global_acknowledged should be true after answering '{answer}'"
        );

        let resp = app.server.get("/inbox").await;
        resp.assert_status(StatusCode::OK);
        assert!(
            !resp.text().contains(&prompt_heading),
            "prompt should not return after answering '{answer}'"
        );
    }
}

#[tokio::test]
async fn search_result_shows_parent_for_sub_issue_and_omits_for_top_level() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let project_id = create_personal_project(&app.db, &user_id, "Customer Portal").await;

    let parent_id = create_issue(&app.db, &project_id, &user_id, "Fix auth flow").await;
    let _sub_id = insert_sub_issue_via_storage(
        &app,
        &project_id,
        &parent_id,
        &user_id,
        "Login redirect loop",
    )
    .await;
    let _top_level_id =
        create_issue(&app.db, &project_id, &user_id, "Standalone redirect fix").await;

    let resp = app.server.get("/search?q=redirect").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains("Login redirect loop"),
        "sub-issue result should still render its own title"
    );
    assert!(
        body.contains("Standalone redirect fix"),
        "a top-level result must not be dropped by the LEFT JOIN"
    );

    let sub_issue_caption = Locale::English.render(MessageKey::SubIssueHitTypePrefix {
        project_name: "Customer Portal".to_string(),
        parent_title: "Fix auth flow".to_string(),
    });
    assert!(
        body.contains(&sub_issue_caption),
        "sub-issue result should render its parent's title: {body}"
    );

    let open_issue_caption = Locale::English.render(MessageKey::OpenIssueHitTypePrefix {
        project_name: "Customer Portal".to_string(),
    });
    assert!(
        body.contains(&open_issue_caption),
        "top-level result should render without a parent breadcrumb"
    );
}

/// `REQ-002` / `FR-NTF-005`: *mark all read* is offered **only while something
/// is unread**. Absent on an empty inbox, present with one unread notification,
/// absent again once everything is read. The route's redirect and its isolation
/// between users are tested elsewhere; this is the limb the record said nothing
/// pinned.
#[tokio::test]
async fn mark_all_read_is_offered_only_while_something_is_unread() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let control = "action=\"/inbox/mark-all-read\"";

    let empty = app.server.get("/inbox").await.text();
    assert!(
        !empty.contains(control),
        "an empty inbox must not offer mark-all-read: {empty}"
    );

    insert_notification(&app, &user_id).await;
    let unread = app.server.get("/inbox").await.text();
    assert!(
        unread.contains(control),
        "with an unread notification the control must be offered: {unread}"
    );

    let resp = app.server.post("/inbox/mark-all-read").await;
    resp.assert_status(StatusCode::SEE_OTHER);
    let read = app.server.get("/inbox").await.text();
    assert!(
        !read.contains(control),
        "once everything is read the control must be gone again: {read}"
    );
}

// ---------------------------------------------------------------------------
// `NTF-001`: `project_trend_decline` has no emitter, so it must not be offered
// as a preference -- and must not be required to be silenced before the
// product will say everything is. `all_kinds_silenced` reads
// `kind::all_user_facing()`, and it gates the pinned banner (`FR-NTF-006`).
// ---------------------------------------------------------------------------

/// The defect, asserted first: with **both kinds that can arrive** silenced and
/// **no row at all** for the one that cannot, the banner must appear. Before
/// the change it did not, because a phantom kind had to be silenced too.
#[tokio::test]
async fn banner_appears_when_the_two_kinds_that_can_arrive_are_silenced() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let resume_button = Locale::English.render(MessageKey::ResumeNotificationsButton);

    for k in [kind::BURNOUT_OVERLOAD, kind::BURNOUT_STALLED] {
        notif_store::upsert_preference(&app.db, &user_id, k, &[], Severity::Info)
            .await
            .expect("silence a real kind");
    }

    let body = app.server.get("/inbox").await.text();
    assert!(
        body.contains(&resume_button),
        "everything that can arrive is silenced, so the banner must say so: {body}"
    );
}

/// A database that already holds a stored preference row for the unlisted kind
/// -- here one that is **not** silenced -- behaves: the settings page renders,
/// the banner still answers on the two real kinds, and the row is left alone
/// (no migration deletes it; it is what the kind needs if it is ever built).
#[tokio::test]
async fn a_stored_preference_row_for_the_unlisted_kind_is_harmless() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let user_id = register_and_login(&app, &user).await;
    let resume_button = Locale::English.render(MessageKey::ResumeNotificationsButton);

    notif_store::upsert_preference(
        &app.db,
        &user_id,
        kind::PROJECT_TREND_DECLINE,
        &["in_app"],
        Severity::Info,
    )
    .await
    .expect("a row an older build could have saved");
    for k in [kind::BURNOUT_OVERLOAD, kind::BURNOUT_STALLED] {
        notif_store::upsert_preference(&app.db, &user_id, k, &[], Severity::Info)
            .await
            .expect("silence a real kind");
    }

    let page = app.server.get("/settings/notifications").await;
    page.assert_status(StatusCode::OK);
    let inbox = app.server.get("/inbox").await.text();
    assert!(
        inbox.contains(&resume_button),
        "a stored row for a kind that cannot arrive must not hold the banner back: {inbox}"
    );
    assert!(
        notif_store::preference_for_user_kind(&app.db, &user_id, kind::PROJECT_TREND_DECLINE)
            .await
            .expect("query")
            .is_some(),
        "the stored row must survive: nothing deletes it"
    );
    assert!(
        notif_store::all_kinds_silenced(&app.db, &user_id)
            .await
            .expect("query"),
        "all_kinds_silenced answers on the two kinds that can arrive"
    );
}

/// The preferences page offers exactly the kinds that can arrive, named -- and
/// not the one that cannot.
#[tokio::test]
async fn the_preferences_page_offers_only_notifications_that_can_arrive() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let body = app.server.get("/settings/notifications").await.text();
    for label in [
        peisear_i18n::NotificationKindLabel::BurnoutOverload,
        peisear_i18n::NotificationKindLabel::BurnoutStalled,
    ] {
        let name = Locale::English.render(MessageKey::NotificationKindName { kind: label });
        assert!(
            body.contains(&name),
            "a preference row for {name:?} is expected: {body}"
        );
    }
    let phantom = Locale::English.render(MessageKey::NotificationKindName {
        kind: peisear_i18n::NotificationKindLabel::ProjectTrendDecline,
    });
    assert!(
        !body.contains(&phantom),
        "no preference row for a notification that can never arrive ({phantom:?}): {body}"
    );
}
