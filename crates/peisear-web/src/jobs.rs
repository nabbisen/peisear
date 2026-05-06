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
use peisear_storage::{Pool, metrics_snapshots, project_health};
use tokio::sync::oneshot;

/// How often the snapshot loop wakes up. Six hours is short
/// enough that "compared to last week" trends have several data
/// points to median over, long enough that the database doesn't
/// grow noticeably from snapshot rows alone (~1 KB / project /
/// week).
const SNAPSHOT_INTERVAL: Duration = Duration::from_secs(6 * 60 * 60);

/// Spawn all background jobs. Returns one oneshot sender per task;
/// dropping or sending on it triggers cooperative shutdown.
///
/// In `main`, this is called after the DB pool is built and
/// before `axum::serve`. The returned senders can be held by the
/// caller; on shutdown they're dropped and the tasks exit.
pub fn spawn_all(db: Pool) -> Vec<oneshot::Sender<()>> {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(snapshot_loop(db, rx));
    vec![tx]
}

/// Snapshot writer loop. Picks projects with any issues, computes
/// their health report, writes one snapshot row per project. Then
/// sleeps until the next tick.
async fn snapshot_loop(db: Pool, mut shutdown: oneshot::Receiver<()>) {
    tracing::info!("snapshot_loop started");

    // Take an initial snapshot on startup so the first project
    // page after a long downtime has something recent to compare
    // against. From then on, run on schedule.
    capture_all(&db).await;

    loop {
        tokio::select! {
            _ = tokio::time::sleep(SNAPSHOT_INTERVAL) => {
                capture_all(&db).await;
            }
            _ = &mut shutdown => {
                tracing::info!("snapshot_loop shutting down");
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
        tracing::debug!(succeeded, total, "snapshot pass complete");
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
