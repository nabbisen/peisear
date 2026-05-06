//! Background tokio tasks that run alongside the web server.
//!
//! Today this is one task — the metrics snapshot writer. Future
//! tasks (per-user burnout snapshots in 0.10.0, optional cleanup of
//! old data, optional notification dispatch) live as siblings in
//! this module. The shape is deliberately uniform: each task is
//! `async fn`, owns its own loop, and is spawned from
//! [`spawn_all`] at startup.
//!
//! ## Design constraints
//!
//! - **Lightweight.** Each tick must do work proportional to the
//!   active fleet (projects with any issues), and a single tick
//!   must not block the request path. The snapshot task today does
//!   one SELECT to find candidates plus N small queries (one per
//!   project), each on the millisecond scale.
//!
//! - **Failure-tolerant.** A failed snapshot for one project must
//!   not stop snapshotting other projects, and the task itself
//!   must not exit on a transient SQL error. Errors are logged at
//!   `tracing::error` level and the loop continues.
//!
//! - **Cooperative shutdown.** Tasks are spawned with `JoinHandle`s
//!   so the caller can choose to await them. They use
//!   `tokio::select!` against a shutdown signal so the next tick
//!   doesn't block process exit.
//!
//! - **No panics.** Anywhere a `?` would propagate from the inner
//!   work, we wrap it in a `match` and log instead. The task
//!   should die only on programmer error.

use std::time::Duration;

use peisear_core::project_health::compute_report;
use peisear_notify::{
    DISPATCH_CHANNEL_BUFFER, DispatchEvent, DispatchTx, detect_burnout_overload_edge,
    detect_burnout_stalled_edge, dispatch_loop,
};
use peisear_notify::config::SmtpConfig;
use peisear_notify::dispatch::DispatchContext;
use peisear_storage::{
    Pool, metrics_snapshots, personal_metrics, project_health, user_burnout,
    user_metrics_snapshots,
};
use tokio::sync::{mpsc, oneshot};

/// How often the snapshot loop wakes up. Six hours is short
/// enough that "compared to last week" trends have several data
/// points to median over, long enough that the database doesn't
/// grow noticeably from snapshot rows alone (~1 KB / project /
/// week).
const SNAPSHOT_INTERVAL: Duration = Duration::from_secs(6 * 60 * 60);

/// Spawn all background jobs. Returns one oneshot sender per
/// stoppable task; dropping or sending on each triggers
/// cooperative shutdown.
///
/// The notification dispatcher exits when its mpsc channel is
/// closed (the snapshot loop drops its `DispatchTx` on exit),
/// so it's not in the returned vec — its lifetime is tied to
/// the snapshot loop's.
///
/// In `main`, this is called after the DB pool is built and
/// before `axum::serve`. The returned senders can be held by the
/// caller; on shutdown they're dropped and the tasks exit.
///
/// `smtp` is the SMTP configuration read from environment by
/// the binary. `None` means email is unconfigured; the
/// dispatcher will skip the email channel and the in-app
/// channel continues to work — see Q4 of 0.16.0 design notes.
pub fn spawn_all(db: Pool, smtp: Option<SmtpConfig>) -> Vec<oneshot::Sender<()>> {
    let (snapshot_tx, snapshot_rx) = oneshot::channel();
    let (dispatch_tx, dispatch_rx) = mpsc::channel::<DispatchEvent>(DISPATCH_CHANNEL_BUFFER);

    let ctx = DispatchContext {
        db: db.clone(),
        smtp,
    };

    // Dispatcher is spawned first so the channel has a consumer
    // before the snapshot loop starts producing.
    tokio::spawn(dispatch_loop(ctx, dispatch_rx));
    tokio::spawn(snapshot_loop(db, snapshot_rx, dispatch_tx));

    vec![snapshot_tx]
}

/// Snapshot writer loop. Each tick:
///   1. Captures one project-level snapshot row per project with
///      any issues (drives the `<HealthStrip>` trend chip).
///   2. Captures one user-level snapshot row per user with any
///      in-flight assigned issues (drives the burnout streak
///      detector at `/me`).
///
/// Both pieces share the same tick rhythm so the two histories
/// stay roughly synchronised; the streak math elsewhere assumes
/// per-user snapshots are taken at roughly `SNAPSHOT_INTERVAL`.
async fn snapshot_loop(
    db: Pool,
    mut shutdown: oneshot::Receiver<()>,
    dispatch_tx: DispatchTx,
) {
    tracing::info!("snapshot_loop started");

    // Take an initial snapshot on startup so the first project
    // page after a long downtime has something recent to compare
    // against. From then on, run on schedule.
    capture_all(&db).await;
    capture_all_users(&db, &dispatch_tx).await;

    loop {
        tokio::select! {
            _ = tokio::time::sleep(SNAPSHOT_INTERVAL) => {
                capture_all(&db).await;
                capture_all_users(&db, &dispatch_tx).await;
            }
            _ = &mut shutdown => {
                tracing::info!("snapshot_loop shutting down");
                // Dropping `dispatch_tx` here closes the
                // dispatch channel, which lets the dispatch
                // loop drain and exit cleanly.
                return;
            }
        }
    }
}

/// One pass: snapshot every project with at least one issue.
async fn capture_all(db: &Pool) {
    let project_ids = match metrics_snapshots::projects_with_recent_issue_activity(db).await {
        Ok(ids) => ids,
        Err(err) => {
            tracing::error!(error = %err, "snapshot_loop: failed to list projects");
            return;
        }
    };

    let total = project_ids.len();
    let mut succeeded = 0_usize;
    for project_id in project_ids {
        match capture_one(db, &project_id).await {
            Ok(()) => succeeded += 1,
            Err(err) => {
                // One project's failure must not abort the others.
                tracing::error!(
                    error = %err,
                    project_id = %project_id,
                    "snapshot_loop: capture_one failed",
                );
            }
        }
    }
    if total > 0 {
        tracing::debug!(succeeded, total, "project snapshot pass complete");
    }
}

/// Snapshot one project. Reads the current `ProjectHealthRaw`,
/// computes the composite score, writes one row.
async fn capture_one(db: &Pool, project_id: &str) -> Result<(), peisear_storage::StorageError> {
    let raw = project_health::for_project(db, project_id).await?;
    let report = compute_report(raw.clone());
    metrics_snapshots::insert(db, project_id, &raw, report.score.value).await?;
    Ok(())
}

/// One pass: snapshot every user with at least one in-flight
/// assigned issue. Idle users are skipped — no signal to capture
/// and no streak to track.
async fn capture_all_users(db: &Pool, dispatch_tx: &DispatchTx) {
    let user_ids = match user_metrics_snapshots::users_with_active_assignments(db).await {
        Ok(ids) => ids,
        Err(err) => {
            tracing::error!(error = %err, "snapshot_loop: failed to list active users");
            return;
        }
    };

    let total = user_ids.len();
    let mut succeeded = 0_usize;
    for user_id in user_ids {
        match capture_one_user(db, &user_id, dispatch_tx).await {
            Ok(()) => succeeded += 1,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    user_id = %user_id,
                    "snapshot_loop: capture_one_user failed",
                );
            }
        }
    }
    if total > 0 {
        tracing::debug!(succeeded, total, "user snapshot pass complete");
    }
}

/// Snapshot one user. Builds the current `PersonalMetrics` value
/// (global, not per-project), derives the over-capacity / over-WIP
/// booleans, writes one row, and emits notification events for
/// any state transitions detected against the prior snapshot.
async fn capture_one_user(
    db: &Pool,
    user_id: &str,
    dispatch_tx: &DispatchTx,
) -> Result<(), peisear_storage::StorageError> {
    let Some(metrics) = personal_metrics::for_user_global(db, user_id).await? else {
        // User vanished between the listing and the per-user
        // query. Treat as a no-op rather than an error.
        return Ok(());
    };

    let over_capacity = match metrics.capacity_points {
        Some(cap) => metrics.in_flight_points > cap,
        None => false,
    };
    let over_wip_limit = metrics.current_wip > metrics.effective_wip_limit;

    // Capture the *prior* burnout state before we write the new
    // snapshot row. The new snapshot may shift the streak count;
    // we want the comparison to reflect the transition introduced
    // by this tick.
    let prior = user_burnout::for_user(db, user_id).await?;

    user_metrics_snapshots::insert(
        db,
        user_id,
        metrics.current_wip,
        metrics.in_flight_points,
        metrics.capacity_points,
        over_capacity,
        metrics.effective_wip_limit,
        over_wip_limit,
    )
    .await?;

    // Compute the new state and look for edges. The
    // notifications subsystem decides whether to actually
    // dispatch — we just submit the candidate events.
    let current = user_burnout::for_user(db, user_id).await?;
    if let (Some(p), Some(c)) = (prior, current) {
        if let Some(event) = detect_burnout_overload_edge(
            user_id,
            p.overload_streak_days,
            c.overload_streak_days,
        ) {
            // try_send: if the dispatch channel is full,
            // drop the event with a warning log rather than
            // back-pressuring the snapshot loop. Edge events
            // are inherently rare; if the channel is full
            // we have a bigger problem than one missed
            // notification.
            if let Err(err) = dispatch_tx.try_send(event) {
                tracing::warn!(
                    user_id = %user_id,
                    error = %err,
                    "snapshot_loop: dispatch channel full or closed (overload edge)",
                );
            }
        }

        if let Some(event) = detect_burnout_stalled_edge(
            user_id,
            p.stalled_assigned_max_days,
            c.stalled_assigned_max_days,
        ) {
            if let Err(err) = dispatch_tx.try_send(event) {
                tracing::warn!(
                    user_id = %user_id,
                    error = %err,
                    "snapshot_loop: dispatch channel full or closed (stalled edge)",
                );
            }
        }
    }

    Ok(())
}
