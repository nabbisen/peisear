//! `SPRINT-004` (RFC 0013, `DEC-054`, `DEC-053`) -- the record of a
//! completed sprint is **captured at completion and discarded on reopen**.
//!
//! A completed sprint's figures and burndown used to be computed live from
//! four inputs: its membership and each member issue's `status`, `effort`
//! and `updated_at`. So they drifted with no change to the sprint at all --
//! carry an unfinished issue into the next sprint and finish it later, and
//! the *completed* sprint's completed figure rose. `complete` now writes what
//! the sprint reported; a completed sprint reads it; `reopen` deletes it.
//!
//! Every comparison of "the same figures" is by the full `Debug` output of
//! `SprintSummary` and of the burndown series (neither type has `PartialEq`),
//! taken through the public `sprints::summary` / `sprints::burndown` the pages
//! use.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, login, register};
use common::fixture::{create_issue, create_team_project, create_team_with_admin};
use common::server::TestApp;
use peisear_core::teams::TeamRole;
use peisear_i18n::MessageKey;
use peisear_storage::{StorageError, sprints, teams};
use std::sync::Arc;
use tokio::sync::Barrier;

fn day() -> chrono::NaiveDate {
    chrono::Utc::now().date_naive()
}

/// A team whose admin is logged in, with a team project, ready for sprints.
struct World {
    app: TestApp,
    admin_id: String,
    team_id: String,
    slug: String,
    project_id: String,
}

async fn world() -> World {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let admin_id = register(&app, &user).await;
    let team_id = create_team_with_admin(&app.db, &admin_id, "Team").await;
    let slug = teams::find_by_id(&app.db, &team_id)
        .await
        .unwrap()
        .unwrap()
        .slug;
    let project_id = create_team_project(&app.db, &admin_id, &team_id, "Proj").await;
    login(&app, &user).await;
    World {
        app,
        admin_id,
        team_id,
        slug,
        project_id,
    }
}

impl World {
    async fn planned_sprint(&self, name: &str) -> String {
        sprints::insert(
            &self.app.db,
            &self.team_id,
            name,
            None,
            day(),
            day() + chrono::Duration::days(14),
        )
        .await
        .expect("insert sprint")
    }

    async fn issue(&self, title: &str, effort: i64, status: &str) -> String {
        let id = create_issue(&self.app.db, &self.project_id, &self.admin_id, title).await;
        sqlx::query("UPDATE issues SET effort = ?2, status = ?3 WHERE id = ?1")
            .bind(&id)
            .bind(effort)
            .bind(status)
            .execute(&self.app.db)
            .await
            .expect("set effort and status");
        id
    }

    /// The sprint completed with one finished issue (3 points) and one
    /// unfinished (5). Returns (sprint id, finished id, unfinished id).
    async fn completed_sprint(&self, name: &str) -> (String, String, String) {
        let sprint = self.planned_sprint(name).await;
        let done = self.issue(&format!("{name}-done"), 3, "done").await;
        let open = self.issue(&format!("{name}-open"), 5, "open").await;
        for id in [&done, &open] {
            sprints::add_issue(&self.app.db, &sprint, id).await.unwrap();
        }
        sprints::start(&self.app.db, &sprint).await.unwrap();
        sprints::complete(&self.app.db, &sprint).await.unwrap();
        (sprint, done, open)
    }

    /// What the sprint's pages read, as one comparable string.
    async fn record(&self, sprint: &str) -> String {
        format!(
            "{:?} | {:?}",
            sprints::summary(&self.app.db, sprint).await.unwrap(),
            sprints::burndown(&self.app.db, sprint).await.unwrap()
        )
    }

    async fn stamp(&self, sprint: &str) -> String {
        let (t,): (chrono::DateTime<chrono::Utc>,) =
            sqlx::query_as("SELECT updated_at FROM sprints WHERE id = ?1")
                .bind(sprint)
                .fetch_one(&self.app.db)
                .await
                .unwrap();
        t.to_rfc3339()
    }

    async fn reopen(&self, sprint: &str) -> axum_test::TestResponse {
        let stamp = self.stamp(sprint).await;
        self.app
            .server
            .post(&format!("/teams/{}/sprints/{sprint}/reopen", self.slug))
            .form(&[("client_updated_at", stamp.as_str())])
            .await
    }

    async fn status_of(&self, sprint: &str) -> (String, Option<String>) {
        sqlx::query_as("SELECT status, CAST(completed_at AS TEXT) FROM sprints WHERE id = ?1")
            .bind(sprint)
            .fetch_one(&self.app.db)
            .await
            .unwrap()
    }
}

// ─────────────────────────────────────────────────────────────
// The drift that started this, fixed
// ─────────────────────────────────────────────────────────────

/// **The test the whole RFC exists for.** Complete a sprint with one finished
/// and one unfinished issue and record what it reports; **carry the
/// unfinished issue to the next sprint through the planning page -- which
/// must succeed**; then finish it. The completed sprint's summary and
/// burndown are identical at all three points.
#[tokio::test]
async fn carrying_an_issue_over_and_finishing_it_does_not_move_the_completed_sprint() {
    let w = world().await;
    let (sprint, _done, open) = w.completed_sprint("S1").await;
    let next = w.planned_sprint("S2").await;
    let reported = w.record(&sprint).await;
    assert!(
        reported.contains("committed_points: 8")
            && reported.contains("completed_points: 3")
            && reported.contains("carried_over_points: 5"),
        "fixture: 8 committed, 3 completed, 5 carried over -- {reported}"
    );

    let resp = w
        .app
        .server
        .post(&format!("/teams/{}/sprints/{next}/plan/add", w.slug))
        .form(&[
            ("issue_id", open.as_str()),
            ("project_id", w.project_id.as_str()),
        ])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    assert_eq!(
        sprints::issues_in_sprint(&w.app.db, &next)
            .await
            .unwrap()
            .len(),
        1,
        "the carry-over itself must succeed"
    );
    assert_eq!(
        w.record(&sprint).await,
        reported,
        "carried over: the completed sprint's record must not move"
    );

    sqlx::query("UPDATE issues SET status = 'done' WHERE id = ?1")
        .bind(&open)
        .execute(&w.app.db)
        .await
        .unwrap();
    assert_eq!(
        w.record(&sprint).await,
        reported,
        "finished later: the completed sprint's record must not move"
    );
}

/// Carry-over from the **issue form's** sprint field too -- the route
/// `SPRINT-001` briefly refused -- and unassigning.
#[tokio::test]
async fn carry_over_works_from_the_issue_form_and_leaves_the_record_alone() {
    let w = world().await;
    let (sprint, done, open) = w.completed_sprint("S1").await;
    let next = w.planned_sprint("S2").await;
    let reported = w.record(&sprint).await;

    let resp = w
        .app
        .server
        .post(&format!("/projects/{}/issues/{open}/sprint", w.project_id))
        .form(&[("sprint_id", next.as_str())])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    assert_eq!(
        sprints::issues_in_sprint(&w.app.db, &next)
            .await
            .unwrap()
            .len(),
        1
    );

    let resp = w
        .app
        .server
        .post(&format!("/projects/{}/issues/{done}/sprint", w.project_id))
        .form(&[("sprint_id", "")])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);

    assert_eq!(
        sprints::issues_in_sprint(&w.app.db, &sprint)
            .await
            .unwrap()
            .len(),
        0,
        "both left the completed sprint's live list"
    );
    assert_eq!(
        w.record(&sprint).await,
        reported,
        "and its record did not move"
    );
}

/// **All four inputs**, because four inputs is the finding: after completion,
/// a member issue's `status`, its `effort`, its `updated_at` (any edit) and the
/// membership itself each leave the record unchanged.
#[tokio::test]
async fn the_record_is_stable_against_all_four_inputs() {
    let w = world().await;
    let (sprint, done, open) = w.completed_sprint("S1").await;
    let reported = w.record(&sprint).await;

    sqlx::query("UPDATE issues SET status = 'done' WHERE id = ?1")
        .bind(&open)
        .execute(&w.app.db)
        .await
        .unwrap();
    assert_eq!(w.record(&sprint).await, reported, "status");

    sqlx::query("UPDATE issues SET effort = 100 WHERE id IN (?1, ?2)")
        .bind(&done)
        .bind(&open)
        .execute(&w.app.db)
        .await
        .unwrap();
    assert_eq!(w.record(&sprint).await, reported, "effort");

    sqlx::query(
        "UPDATE issues SET title = 'edited', updated_at = '2031-01-01 00:00:00' WHERE id = ?1",
    )
    .bind(&done)
    .execute(&w.app.db)
    .await
    .unwrap();
    assert_eq!(w.record(&sprint).await, reported, "updated_at (any edit)");

    sqlx::query("DELETE FROM sprint_issues WHERE sprint_id = ?1")
        .bind(&sprint)
        .execute(&w.app.db)
        .await
        .unwrap();
    assert_eq!(w.record(&sprint).await, reported, "membership");
}

/// A completed sprint with **no** record is a fault, not a reason to compute
/// live: that would be the drift returning with figures that look right.
#[tokio::test]
async fn a_completed_sprint_with_no_record_is_an_error_not_a_live_figure() {
    let w = world().await;
    let (sprint, _done, _open) = w.completed_sprint("S1").await;
    sqlx::query("DELETE FROM sprint_records WHERE sprint_id = ?1")
        .bind(&sprint)
        .execute(&w.app.db)
        .await
        .unwrap();
    assert!(matches!(
        sprints::summary(&w.app.db, &sprint).await,
        Err(StorageError::InvalidData(_))
    ));
    assert!(matches!(
        sprints::burndown(&w.app.db, &sprint).await,
        Err(StorageError::InvalidData(_))
    ));
}

// ─────────────────────────────────────────────────────────────
// Reopen
// ─────────────────────────────────────────────────────────────

/// A completed sprint reopens to `active` with `completed_at` NULL; it leaves
/// the velocity list (`recent_completed_for_team`); its record is discarded and
/// its figures and burndown are computed as for any active sprint -- so
/// membership edits move them again.
#[tokio::test]
async fn reopen_returns_the_sprint_to_active_and_makes_it_live_again() {
    let w = world().await;
    let (sprint, _done, open) = w.completed_sprint("S1").await;
    let in_velocity = |w: &World| {
        let (db, team) = (w.app.db.clone(), w.team_id.clone());
        async move {
            sprints::recent_completed_for_team(&db, &team, 10)
                .await
                .unwrap()
                .len()
        }
    };
    assert_eq!(in_velocity(&w).await, 1, "completed: in the velocity list");

    let resp = w.reopen(&sprint).await;
    resp.assert_status(StatusCode::SEE_OTHER);
    // success flashes, as `start` and `complete` do
    let loc = resp.header("location").to_str().unwrap().to_string();
    let (_, query) = loc.split_once('?').expect("a flash query");
    let q: std::collections::HashMap<String, String> = serde_urlencoded::from_str(query).unwrap();
    assert_eq!(
        q.get("flash").cloned().unwrap_or_default(),
        peisear_i18n::Locale::English.render(MessageKey::SprintReopenedFlash)
    );

    assert_eq!(w.status_of(&sprint).await, ("active".to_string(), None));
    assert_eq!(
        in_velocity(&w).await,
        0,
        "reopened: out of the velocity list"
    );
    let records: i64 = sqlx::query_scalar(
        "SELECT (SELECT COUNT(*) FROM sprint_records WHERE sprint_id = ?1)
              + (SELECT COUNT(*) FROM sprint_burndown_points WHERE sprint_id = ?1)",
    )
    .bind(&sprint)
    .fetch_one(&w.app.db)
    .await
    .unwrap();
    assert_eq!(records, 0, "the capture is discarded");

    let live = format!("{:?}", sprints::summary(&w.app.db, &sprint).await.unwrap());
    assert!(live.contains("committed_points: 8") && live.contains("carried_over_points: 0"));
    sqlx::query("UPDATE issues SET status = 'done' WHERE id = ?1")
        .bind(&open)
        .execute(&w.app.db)
        .await
        .unwrap();
    let live = format!("{:?}", sprints::summary(&w.app.db, &sprint).await.unwrap());
    assert!(
        live.contains("completed_points: 8"),
        "an active sprint follows its issues again: {live}"
    );
}

/// **The round trip, which is the point**: complete -> reopen -> correct ->
/// complete, and the second capture differs from the first where the work
/// changed (otherwise the discard is not proven).
#[tokio::test]
async fn complete_reopen_correct_complete_recaptures_afresh() {
    let w = world().await;
    let (sprint, done, open) = w.completed_sprint("S1").await;
    let first = w.record(&sprint).await;

    w.reopen(&sprint).await.assert_status(StatusCode::SEE_OTHER);
    // the correction: the unfinished work was in fact finished, and re-estimated
    sqlx::query("UPDATE issues SET status = 'done', effort = 6 WHERE id = ?1")
        .bind(&open)
        .execute(&w.app.db)
        .await
        .unwrap();
    // and an unassign (which no longer refuses anything)
    w.app
        .server
        .post(&format!("/projects/{}/issues/{done}/sprint", w.project_id))
        .form(&[("sprint_id", "")])
        .await
        .assert_status(StatusCode::SEE_OTHER);

    sprints::complete(&w.app.db, &sprint).await.unwrap();
    let second = w.record(&sprint).await;
    assert_ne!(
        second, first,
        "the second capture must reflect the correction"
    );
    assert!(
        second.contains("committed_points: 6")
            && second.contains("completed_points: 6")
            && second.contains("carried_over_points: 0"),
        "6 committed and completed, nothing carried over: {second}"
    );
}

/// `FR-SPR-002` binds reopen as it binds `start`: refused while another sprint
/// in the team is active, sequentially, in `start`'s words.
#[tokio::test]
async fn reopen_is_refused_while_another_sprint_is_active() {
    let w = world().await;
    let (sprint, _, _) = w.completed_sprint("First").await;
    let other = w.planned_sprint("Second").await;
    sprints::start(&w.app.db, &other).await.unwrap();

    let resp = w.reopen(&sprint).await;
    resp.assert_status(StatusCode::SEE_OTHER);
    let loc = resp.header("location").to_str().unwrap().to_string();
    let (_, query) = loc.split_once('?').expect("an error query");
    let q: std::collections::HashMap<String, String> = serde_urlencoded::from_str(query).unwrap();
    assert_eq!(
        q.get("error").cloned().unwrap_or_default(),
        peisear_i18n::Locale::English.render(MessageKey::OtherSprintActiveInTeamMessage {
            sprint_name: "Second".to_string()
        })
    );
    assert_eq!(w.status_of(&sprint).await.0, "completed", "nothing changed");
}

/// Reopen is refused on a planned and on an active sprint, with its own
/// wrong-state message.
#[tokio::test]
async fn reopen_is_refused_on_a_planned_and_on_an_active_sprint() {
    let w = world().await;
    let planned = w.planned_sprint("Planned").await;
    let want = peisear_i18n::Locale::English.render(MessageKey::SprintNotCompletedMessage);

    let resp = w.reopen(&planned).await;
    resp.assert_status(StatusCode::BAD_REQUEST);
    assert!(resp.text().contains(&want));

    sprints::start(&w.app.db, &planned).await.unwrap();
    let resp = w.reopen(&planned).await;
    resp.assert_status(StatusCode::BAD_REQUEST);
    assert!(resp.text().contains(&want));
}

/// Only an administrator can reopen. A member with write access gets the same
/// `403` `start` and `complete` give (their gate is the same
/// `can_manage_team()`); shape followed: `sprint_plan.rs`'s
/// `viewer_gets_read_only_plan_and_403_on_post`.
#[tokio::test]
async fn only_an_administrator_can_reopen() {
    let w = world().await;
    let (sprint, _, _) = w.completed_sprint("S1").await;
    let member = TestUser::new("mallory");
    let member_id = register(&w.app, &member).await;
    teams::add_member(&w.app.db, &w.team_id, &member_id, TeamRole::Member)
        .await
        .unwrap();
    login(&w.app, &member).await;

    let resp = w.reopen(&sprint).await;
    resp.assert_status(StatusCode::FORBIDDEN);
    assert_eq!(w.status_of(&sprint).await.0, "completed");
}

/// The control is on the completed sprint's page for an administrator, with
/// the lock value, and the page says which figures are captured and which
/// list is current.
#[tokio::test]
async fn the_completed_sprint_page_offers_reopen_and_labels_record_and_list() {
    let w = world().await;
    let (sprint, _, _) = w.completed_sprint("S1").await;
    let body = w
        .app
        .server
        .get(&format!("/teams/{}/sprints/{sprint}", w.slug))
        .await
        .text();
    let reopen_at = body
        .find(&format!("/sprints/{sprint}/reopen"))
        .expect("a reopen form on a completed sprint");
    let tail = &body[reopen_at..];
    assert!(
        tail.contains("client_updated_at"),
        "the form carries the lock value"
    );
    assert!(body.contains("Reopen sprint"));
    let summary = body
        .find("Summary at completion")
        .expect("captured heading");
    let list = body
        .find("Issues in this sprint now")
        .expect("live-list heading");
    assert!(
        summary < list,
        "the captured figures come before the live list"
    );

    // and an active sprint offers none, with the ordinary headings
    let other = w.planned_sprint("Active").await;
    let active_body = w
        .app
        .server
        .get(&format!("/teams/{}/sprints/{other}", w.slug))
        .await
        .text();
    assert!(!active_body.contains("/reopen"));
    assert!(!active_body.contains("Summary at completion"));
}

// ─────────────────────────────────────────────────────────────
// FR-SPR-002, concurrently
// ─────────────────────────────────────────────────────────────

/// `N` completed sprints of one team, all reopened at once: exactly one
/// becomes active, the rest are refused with `OtherSprintActiveInTeamMessage`
/// -- and each refused sprint is still completed with its record intact. A
/// non-atomic reopen (the check on the pool, the write on another connection)
/// leaves several active; that first draft is what this exists to catch.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn simultaneous_reopens_in_one_team_leave_one_active() {
    const N: usize = 12;
    let w = world().await;
    let mut ids = Vec::new();
    for i in 0..N {
        let (s, _, _) = w.completed_sprint(&format!("S{i}")).await;
        ids.push(s);
    }
    let db = w.app.db.clone();
    let barrier = Arc::new(Barrier::new(N));
    let handles: Vec<_> = ids
        .iter()
        .map(|id| {
            let (db, id, barrier) = (db.clone(), id.clone(), barrier.clone());
            tokio::spawn(async move {
                barrier.wait().await;
                sprints::reopen(&db, &id).await
            })
        })
        .collect();
    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }

    let ok = results.iter().filter(|r| r.is_ok()).count();
    let refused = results
        .iter()
        .filter(|r| {
            matches!(
                r,
                Err(StorageError::Conflict(
                    MessageKey::OtherSprintActiveInTeamMessage { .. }
                ))
            )
        })
        .count();
    let active: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM sprints WHERE team_id = ?1 AND status = 'active'")
            .bind(&w.team_id)
            .fetch_one(&db)
            .await
            .unwrap();
    let records: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sprint_records")
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(
        (ok, refused, active, records),
        (1, N - 1, 1, (N - 1) as i64),
        "one reopened, {} refused, one active, and only the reopened sprint's record gone; \
         results {results:?}",
        N - 1
    );
}
