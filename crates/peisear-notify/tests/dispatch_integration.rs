//! Integration test for the notification dispatch pipeline
//! (peisear 0.16.0, Q5 = "env var diff verification").
//!
//! ## What this test covers
//!
//! - The end-to-end shape of the dispatch_loop: an event sent
//!   on the channel results in a `notifications` row written to
//!   the audit table.
//! - The Q4 graceful-degradation contract: when SMTP is *not*
//!   configured (`DispatchContext::smtp = None`), the email
//!   channel is skipped (not failed); the in-app channel still
//!   delivers; the audit row's `dispatched_via` records only
//!   the channels that actually succeeded.
//! - The Q4 graceful-degradation contract under failure: when
//!   SMTP is configured but unreachable (a synthetic invalid
//!   host that won't connect), the email channel fails at send
//!   time and `dispatched_via` doesn't include `email`. The
//!   in-app row is still recorded.
//!
//! ## What this test does NOT cover
//!
//! Real SMTP delivery on the wire. Per Q5, that's operator
//! territory — verifying it would require a docker mailpit
//! container in CI, which we explicitly chose to avoid.
//! End-to-end SMTP correctness is the operator's call when
//! they configure their own server.
//!
//! ## How the test forces a dispatch
//!
//! The test sets up a user with email-channel preferences for
//! one notification kind, sends a `DispatchEvent` directly
//! through the dispatch channel, and reads the resulting
//! `notifications` row. This bypasses the snapshot loop —
//! whose timing is irrelevant to dispatching correctness.

use std::time::Duration;

use peisear_core::notifications::{Severity, channel as channel_id, kind as kind_id};
use peisear_notify::{DispatchEvent, DispatchTx, dispatch_loop};
use peisear_notify::config::{SmtpConfig, TlsMode};
use peisear_notify::dispatch::DispatchContext;
use peisear_storage::{Pool, notifications as notif_store, pool, users};
use tokio::sync::mpsc;

/// Per-test fresh DB. Uses peisear-storage's connect+migrate
/// helpers rather than reaching into sqlx directly — avoids
/// the dev-dep feature-resolution recompile that would
/// otherwise hit when this test is built.
async fn fresh_pool() -> Pool {
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = format!("/tmp/peisear-notify-it-{}", suffix);
    std::fs::create_dir_all(&path).unwrap();
    let url = format!("sqlite://{}/test.db", path);
    let p = pool::connect(&url).await.unwrap();
    pool::migrate(&p).await.unwrap();
    p
}

/// Set up a user with an active email-channel preference for
/// `BURNOUT_OVERLOAD`. Returns the user id.
async fn create_user_subscribed_to_email(pool: &Pool) -> String {
    // Use timestamp-derived id to avoid pulling in `uuid` as a
    // dev-dep (which would expand the test's compile graph).
    let user_id = format!(
        "user-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    users::insert(
        pool,
        &user_id,
        &format!("{}@example.com", user_id),
        "x", // password hash placeholder; we don't authenticate
        "TestUser",
    )
    .await
    .unwrap();

    notif_store::upsert_preference(
        pool,
        &user_id,
        kind_id::BURNOUT_OVERLOAD,
        &[channel_id::IN_APP, channel_id::EMAIL],
        Severity::Info,
    )
    .await
    .unwrap();

    user_id
}

fn spawn_dispatch(ctx: DispatchContext) -> DispatchTx {
    let (tx, rx) = mpsc::channel::<DispatchEvent>(8);
    tokio::spawn(dispatch_loop(ctx, rx));
    tx
}

fn make_event(user_id: &str) -> DispatchEvent {
    DispatchEvent {
        user_id: user_id.to_string(),
        kind: kind_id::BURNOUT_OVERLOAD.to_string(),
        severity: Severity::Watch,
        title: "Sustained over-capacity streak".to_string(),
        body: "Test body for integration test.".to_string(),
        payload_json: None,
    }
}

async fn wait_for_notification(
    pool: &Pool,
    user_id: &str,
) -> peisear_core::notifications::Notification {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let rows = notif_store::recent_for_user(pool, user_id, 10)
            .await
            .unwrap();
        if let Some(row) = rows.into_iter().next() {
            return row;
        }
        if std::time::Instant::now() > deadline {
            panic!("notification didn't arrive within 10s");
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::test]
async fn smtp_unconfigured_records_in_app_only_in_dispatched_via() {
    let pool = fresh_pool().await;
    let user_id = create_user_subscribed_to_email(&pool).await;

    let ctx = DispatchContext {
        db: pool.clone(),
        smtp: None, // <-- Q4 graceful: no SMTP config
    };
    let tx = spawn_dispatch(ctx);

    tx.send(make_event(&user_id)).await.unwrap();
    let row = wait_for_notification(&pool, &user_id).await;

    // The audit row should record only the channels that
    // succeeded. With SMTP unconfigured, email is skipped at
    // the channel layer, so dispatched_via is just ["in_app"].
    assert_eq!(
        row.dispatched_via,
        vec!["in_app".to_string()],
        "expected only in_app delivery; got: {:?}",
        row.dispatched_via
    );
    assert_eq!(row.kind, kind_id::BURNOUT_OVERLOAD);
}

#[tokio::test]
async fn smtp_configured_but_unreachable_records_in_app_only() {
    let pool = fresh_pool().await;
    let user_id = create_user_subscribed_to_email(&pool).await;

    // Synthetic SMTP config pointing at a host that won't
    // accept connections. Port 1 is reserved for tcpmux; no
    // service listens. The TCP connect fails fast with a
    // transport error — no real SMTP traffic. The point of
    // this test is to verify that a *configured-but-broken*
    // SMTP doesn't block in-app delivery.
    let unreachable = SmtpConfig {
        host: "127.0.0.1".to_string(),
        port: 1,
        tls_mode: TlsMode::Implicit,
        username: "test@example.com".to_string(),
        password: "x".to_string(),
        from_address: "test@example.com".to_string(),
        from_name: None,
    };

    let ctx = DispatchContext {
        db: pool.clone(),
        smtp: Some(unreachable),
    };
    let tx = spawn_dispatch(ctx);

    tx.send(make_event(&user_id)).await.unwrap();
    let row = wait_for_notification(&pool, &user_id).await;

    assert_eq!(
        row.dispatched_via,
        vec!["in_app".to_string()],
        "expected only in_app delivery (email send fails); \
         got: {:?}",
        row.dispatched_via
    );
}

#[tokio::test]
async fn cooldown_suppresses_second_dispatch_within_window() {
    // Sanity: existing cooldown filter still works after the
    // dispatch pipeline moved to its own crate.
    let pool = fresh_pool().await;
    let user_id = create_user_subscribed_to_email(&pool).await;

    let ctx = DispatchContext {
        db: pool.clone(),
        smtp: None,
    };
    let tx = spawn_dispatch(ctx);

    tx.send(make_event(&user_id)).await.unwrap();
    let _first = wait_for_notification(&pool, &user_id).await;

    tx.send(make_event(&user_id)).await.unwrap();
    tokio::time::sleep(Duration::from_millis(200)).await;

    let rows = notif_store::recent_for_user(&pool, &user_id, 10)
        .await
        .unwrap();
    assert_eq!(
        rows.len(),
        1,
        "cooldown should suppress the 2nd notification within \
         24h; got {} rows",
        rows.len()
    );
}
