//! Issue table queries.
//!
//! All mutations are scoped against `(project_id, owner_id)` to enforce
//! access control at the query level as a defense-in-depth measure on
//! top of the handler checks.

use chrono::{DateTime, Utc};
use peisear_core::{Issue, IssueStatus, Priority};
use sqlx::FromRow;

use crate::{Guarded, Pool, StorageError, StorageResult, issue_events, stamp_matches};

/// Raw row as returned by sqlx. Kept private — the public API returns
/// [`peisear_core::Issue`] with parsed enum fields.
#[derive(FromRow)]
struct IssueRow {
    id: String,
    project_id: String,
    author_id: String,
    title: String,
    description: String,
    status: String,
    priority: String,
    effort: Option<i64>,
    assignee_id: Option<String>,
    /// Sub-issue parent reference (Phase C PR1, migration
    /// 0015). NULL for top-level issues; FK to `issues(id)`
    /// for sub-issues. The 1-level constraint is enforced by
    /// trigger.
    parent_issue_id: Option<String>,
    planned_start_at: Option<DateTime<Utc>>,
    planned_end_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl IssueRow {
    fn into_issue(self) -> StorageResult<Issue> {
        let status = IssueStatus::parse(&self.status)
            .ok_or_else(|| StorageError::InvalidData(format!("status={}", self.status)))?;
        let priority = Priority::parse(&self.priority)
            .ok_or_else(|| StorageError::InvalidData(format!("priority={}", self.priority)))?;
        Ok(Issue {
            id: self.id,
            project_id: self.project_id,
            author_id: self.author_id,
            title: self.title,
            description: self.description,
            status,
            priority,
            effort: self.effort,
            assignee_id: self.assignee_id,
            parent_issue_id: self.parent_issue_id,
            planned_start_at: self.planned_start_at,
            planned_end_at: self.planned_end_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

/// List top-level issues in a project. Per
/// peisear-feature-spec-v2.1 §8.5, the project board / list /
/// kanban surface only the top-level rows; sub-issues are
/// rendered inline in their parent's detail page. This
/// function applies that filter — callers that want every row
/// (e.g. analytics, project_workload, health computation)
/// should use [`list_all_in_project`].
///
/// **Ordering (`ORD-001`): newest first, and nothing else** --
/// `created_at DESC, rowid DESC`. There is deliberately no `status`
/// term (it is TEXT, so `ASC` was alphabetical and put `done` first;
/// the board gets its column order from `IssueStatus::all()`) and no
/// stored manual order (`position` was removed by migration `0018`).
/// **`rowid DESC` is load-bearing, not redundant**: `created_at` is a
/// one-second `CURRENT_TIMESTAMP`, so issues created in the same second
/// tie, and SQLite returns ties in scan order -- oldest first -- which
/// would make a burst of creations read backwards.
/// `view_state::issues_created_in_the_same_second_list_newest_first`
/// fails without it. What it guarantees is a **stable, deterministic**
/// order within a tie, and creation order in the ordinary case -- not
/// creation order unconditionally: SQLite reuses the rowid of a deleted
/// highest row (measured), so deleting the newest issue and creating
/// another within the same second can invert that pair. Every
/// alternative tiebreak is arbitrary in *all* cases, so this is never
/// worse; do not build anything on it that needs more.
/// [`list_all_in_project`] orders identically.
pub async fn list_in_project(pool: &Pool, project_id: &str) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE project_id = ?1
          AND parent_issue_id IS NULL
        ORDER BY created_at DESC, rowid DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}

/// List **every** issue in a project, top-level and sub-issue
/// alike. Use for analytics surfaces (workload, health,
/// project_health) where excluding sub-issues would skew the
/// numbers — sub-issues represent real assigned work even if
/// they're not shown on the kanban.
pub async fn list_all_in_project(pool: &Pool, project_id: &str) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE project_id = ?1
        ORDER BY created_at DESC, rowid DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}

/// List the sub-issues of a given parent. Returns them in
/// creation order (oldest first) — this is the order they
/// were defined in, which usually reads as "natural reading
/// order" in the parent's detail panel.
///
/// Empty vec if the parent has no children. The parent itself
/// is not included.
pub async fn list_sub_issues_of(pool: &Pool, parent_issue_id: &str) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE parent_issue_id = ?1
        ORDER BY created_at ASC, rowid ASC
        "#,
    )
    .bind(parent_issue_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}

pub async fn find(pool: &Pool, issue_id: &str, project_id: &str) -> StorageResult<Issue> {
    let row = sqlx::query_as::<_, IssueRow>(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(issue_id)
    .bind(project_id)
    .fetch_optional(pool)
    .await?;
    row.ok_or(StorageError::NotFound)
        .and_then(IssueRow::into_issue)
}

/// The mutable content fields of an issue — everything a create
/// or edit form supplies, as opposed to the id/project/actor
/// coordinates that route the write. Shared between [`insert`]
/// and [`update`] so both take the same shape.
pub struct IssueFields<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub status: IssueStatus,
    pub priority: Priority,
    pub effort: Option<i64>,
    pub assignee_id: Option<&'a str>,
    /// `CAL-001` / RFC 002. Only the issue edit form's inputs set
    /// these to `Some` — the create form does not carry them
    /// (handoff §1: "the issue edit form's date inputs", not the
    /// create form's), so every `create`/`create_sub_issue` call
    /// site passes `None` for both.
    pub planned_start_at: Option<DateTime<Utc>>,
    pub planned_end_at: Option<DateTime<Utc>>,
}

pub async fn insert(
    pool: &Pool,
    id: &str,
    project_id: &str,
    author_id: &str,
    fields: IssueFields<'_>,
) -> StorageResult<()> {
    // Open a transaction so the issue insert and the 'created'
    // event row land atomically. If anything fails, neither
    // commits.
    let mut tx = pool.begin().await?;

    let res = sqlx::query(
        r#"
        INSERT INTO issues
            (id, project_id, author_id, title, description, status, priority,
             effort, assignee_id, planned_start_at, planned_end_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(author_id)
    .bind(fields.title)
    .bind(fields.description)
    .bind(fields.status.as_str())
    .bind(fields.priority.as_str())
    .bind(fields.effort)
    .bind(fields.assignee_id)
    .bind(fields.planned_start_at)
    .bind(fields.planned_end_at)
    .execute(&mut *tx)
    .await;

    // Migration 0016's insert trigger can now fire here (it could
    // not before — nothing on this path set planned dates until
    // CAL-001). Translate the same way insert_sub_issue already
    // does, per DEC-011.
    if let Err(e) = res {
        return Err(translate_trigger_error(e));
    }

    // The 'created' event records the initial state. Storing
    // status as `new_value` lets later analysis answer "what
    // status did this issue start in?" without a join.
    issue_events::insert_event(
        &mut tx,
        id,
        project_id,
        Some(author_id),
        issue_events::kind::CREATED,
        None,
        Some(fields.status.as_str()),
    )
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Insert a sub-issue under an existing parent. Same shape as
/// `insert` but with an additional `parent_issue_id` argument.
///
/// The 1-level constraint (a sub-issue can't have its own
/// sub-issue) and the same-project constraint are both
/// enforced by triggers in migration 0015 — this function
/// surfaces those as `StorageError::Validation` rather than
/// raw SQL errors so the caller can render them to the user.
///
/// Sub-issues are listed under their parent in creation order
/// (see `list_sub_issues_of`), not interspersed in the project's
/// main kanban.
#[allow(clippy::too_many_arguments)]
pub async fn insert_sub_issue(
    pool: &Pool,
    id: &str,
    project_id: &str,
    parent_issue_id: &str,
    author_id: &str,
    title: &str,
    description: &str,
    status: IssueStatus,
    priority: Priority,
    effort: Option<i64>,
    assignee_id: Option<&str>,
) -> StorageResult<()> {
    let mut tx = pool.begin().await?;

    let res = sqlx::query(
        r#"
        INSERT INTO issues
            (id, project_id, author_id, title, description, status, priority,
             effort, assignee_id, parent_issue_id)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(author_id)
    .bind(title)
    .bind(description)
    .bind(status.as_str())
    .bind(priority.as_str())
    .bind(effort)
    .bind(assignee_id)
    .bind(parent_issue_id)
    .execute(&mut *tx)
    .await;

    // Trigger violations show up as raw sqlx errors with
    // SQLite's RAISE message inside. Translate them to
    // Validation so the handler can render them as 400 instead
    // of 500.
    if let Err(e) = res {
        return Err(translate_trigger_error(e));
    }

    issue_events::insert_event(
        &mut tx,
        id,
        project_id,
        Some(author_id),
        issue_events::kind::CREATED,
        None,
        Some(status.as_str()),
    )
    .await?;

    tx.commit().await?;
    Ok(())
}

/// Promote a sub-issue back to top-level (sets
/// `parent_issue_id = NULL`). Per spec §8.3 this is a single
/// column update — the assignee, status, sprint membership
/// etc. all stay as they were.
///
/// No-op if the issue is already top-level.
pub async fn promote_to_top_level(pool: &Pool, id: &str, project_id: &str) -> StorageResult<()> {
    sqlx::query(
        r#"
        UPDATE issues
        SET parent_issue_id = NULL
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Demote a top-level issue to be a sub-issue of `new_parent_id`.
/// Validates that the move doesn't create a 2-level chain
/// (existing children must be promoted first) — that check is
/// done in the trigger.
pub async fn demote_to_sub_issue(
    pool: &Pool,
    id: &str,
    project_id: &str,
    new_parent_id: &str,
) -> StorageResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE issues
        SET parent_issue_id = ?3
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(new_parent_id)
    .execute(pool)
    .await;

    if let Err(e) = res {
        return Err(translate_trigger_error(e));
    }
    Ok(())
}

/// `DIA-001`: the bracketed code every known trigger `RAISE` message
/// carries, paired with the `MessageKey` that renders the human
/// sentence alongside it. Also the registry
/// [`every_raise_code_is_known`] and
/// [`every_known_code_appears_in_a_live_trigger`] both check against,
/// so there is exactly one list to keep in sync — with whatever
/// `sqlite_master` says a fully-migrated database's triggers actually
/// raise (`DIA-001` round 3), not with any one migration file. `0022`
/// is where these five currently live, having dropped and re-created
/// `0015`/`0016`'s own four (those two are frozen byte-for-byte at
/// their `0.53.0` content and carry the original prose forever) — but
/// a future migration that touches these triggers again needs no
/// change here or in the guard, only in this list and in whichever
/// migration it lands in.
const TRIGGER_ERROR_CODES: &[(&str, peisear_i18n::MessageKey)] = &[
    (
        "E_SUB_ISSUE_DEPTH",
        peisear_i18n::MessageKey::SubIssueCannotHaveSubIssueMessage,
    ),
    (
        "E_SUB_ISSUE_PROJECT_MISMATCH",
        peisear_i18n::MessageKey::SubIssueMustShareProjectMessage,
    ),
    (
        "E_ISSUE_SELF_PARENT",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ),
    (
        "E_ISSUE_DEMOTE_HAS_SUBISSUES",
        peisear_i18n::MessageKey::CannotDemoteIssueWithSubIssuesMessage,
    ),
    (
        "E_ISSUE_PLANNED_DATE_ORDER",
        peisear_i18n::MessageKey::IssuePlannedEndBeforeStartMessage,
    ),
];

/// The code inside a `[CODE]` bracket anywhere in `msg`, if one is
/// there. **Not anchored to the start of `msg`** — measured directly
/// against a real trigger failure before trusting this: `sqlx::Error`'s
/// own `Display` is `"error returned from database: {0}"`
/// (`sqlx-core`'s `error.rs`), so the bracket this function is
/// actually asked to find never opens at position 0, only
/// `msg.contains` would have found it. `None` for any message with no
/// `[…]` at all, which is every `sqlx::Error` this function was never
/// meant to translate (a constraint violation with no trigger
/// involved, a connection error, and so on).
fn trigger_error_code(msg: &str) -> Option<&str> {
    let idx = msg.find('[')?;
    let rest = &msg[idx + 1..];
    let end = rest.find(']')?;
    Some(&rest[..end])
}

/// Translate a raw `sqlx::Error` into the closest `StorageError` for
/// sub-issue and planned-date trigger violations.
///
/// **Matches a code, never prose** (`DIA-001`, 0.54.0). Before this
/// round, this function matched `msg.contains(english sentence)` —
/// brittle twice over: two of the five needles were *prefixes* of the
/// real message rather than the whole of it, so a reword landing
/// after the matched prefix would have kept passing silently, and the
/// coupling meant the English wording could never change without
/// also touching this function. SQLite's `RAISE` has no
/// machine-readable error code the way a real constraint violation
/// does, so the code is carried *inside* the message text instead, as
/// a `[E_CODE]` prefix no ordinary English sentence could be mistaken
/// for — chosen over a bare word for exactly that reason.
///
/// **The code and the live trigger both live in
/// `0022_trigger_error_codes.sql`, not in `0015`/`0016`** (round 2:
/// an in-place edit to those two broke every existing `0.53.0`
/// install, since `sqlx` refuses to re-run a migration whose
/// checksum has changed — demonstrated, not assumed, at that round's
/// review). `0022` drops and re-creates the five triggers this
/// function translates, with the identical sentence
/// [`peisear_i18n::MessageKey`] renders, now carrying the code ahead
/// of it. `0015`/`0016` keep the original prose-only text forever,
/// unreachable once `0022` has run, kept only because changing a
/// byte of either breaks every database that already applied them.
///
/// **Not a PostgreSQL shape.** `RAISE EXCEPTION … USING ERRCODE =
/// '…'` carries a real out-of-band code, so a future `P3` port does
/// not need this bracket trick at all — it needs [`TRIGGER_ERROR_CODES`]'s
/// own code strings moved to the `ERRCODE` values the Postgres
/// triggers raise, which is exactly why the SQLite-only names chosen
/// here (`E_SUB_ISSUE_DEPTH`, not a number) carry no SQLite-specific
/// meaning of their own.
fn translate_trigger_error(e: sqlx::Error) -> StorageError {
    let msg = e.to_string();
    if let Some(code) = trigger_error_code(&msg) {
        for (known_code, key) in TRIGGER_ERROR_CODES {
            if *known_code == code {
                return StorageError::Validation(key.clone());
            }
        }
    }
    e.into()
}

/// Every `RAISE(ABORT, '…')` message's full quoted text carried by a
/// **live trigger** in `pool`'s own `sqlite_master` — not a migration
/// file, after `DIA-001` round 3 found the file-scoped version of
/// this check (round 2's own `all_raise_messages`, reading
/// `0022_trigger_error_codes.sql` alone) could not survive the next
/// migration: a future `0023` adding an uncoded `RAISE` would reach
/// users as a raw, untyped error while this check kept reading `0022`
/// and staying green. `sqlite_master` is what a real, fully-migrated
/// database actually has, regardless of which file most recently
/// defined it, so a later migration is covered the day it lands, with
/// nothing here to update. The full message, not only a bracketed
/// prefix, so a `RAISE` carrying no code at all is still part of this
/// population (`§10.39`'s own class: a needle that can only find what
/// already matches its own shape).
#[cfg(test)]
async fn live_trigger_raise_messages(pool: &Pool) -> Vec<String> {
    let marker = "RAISE(ABORT, '";
    let sqls: Vec<String> =
        sqlx::query_scalar("SELECT sql FROM sqlite_master WHERE type = 'trigger'")
            .fetch_all(pool)
            .await
            .expect("read sqlite_master");
    let mut out = Vec::new();
    for sql in &sqls {
        let mut rest = sql.as_str();
        while let Some(idx) = rest.find(marker) {
            let after = &rest[idx + marker.len()..];
            let Some(end) = after.find("')") else { break };
            out.push(after[..end].to_string());
            rest = &after[end..];
        }
    }
    out
}

/// `DIA-001` §3/§13, direction 1: every `RAISE` message found carries
/// a code `known` recognises. Takes `known` as a parameter, not
/// [`TRIGGER_ERROR_CODES`] directly, so a test can plant a shrunk
/// list against the real, live triggers without touching a committed
/// file (the handoff's own "remove a code the translation knows"
/// plant).
#[cfg(test)]
fn every_raise_code_is_known(
    messages: &[String],
    known: &[(&str, peisear_i18n::MessageKey)],
) -> Vec<String> {
    let mut out = Vec::new();
    for msg in messages {
        match trigger_error_code(msg) {
            Some(code) if known.iter().any(|(c, _)| *c == code) => {}
            Some(code) => out.push(format!(
                "a live trigger RAISEs '{msg}', code {code}, which translate_trigger_error does not know"
            )),
            None => out.push(format!(
                "a live trigger RAISEs '{msg}', which carries no [CODE] prefix"
            )),
        }
    }
    out
}

/// `DIA-001` §3/§13, direction 2: every code `known` carries appears
/// in at least one live `RAISE` message found. This is the direction
/// that direction 1 cannot catch on its own — a code
/// `TRIGGER_ERROR_CODES` still lists after the trigger that used to
/// raise it is dropped (and never re-created) would pass direction 1
/// forever, silently dead.
#[cfg(test)]
fn every_known_code_appears_in_a_live_trigger(
    messages: &[String],
    known: &[(&str, peisear_i18n::MessageKey)],
) -> Vec<String> {
    let mut out = Vec::new();
    for (code, _) in known {
        if !messages
            .iter()
            .any(|m| trigger_error_code(m) == Some(*code))
        {
            out.push(format!(
                "translate_trigger_error knows code {code}, which no live trigger RAISEs"
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests;

pub async fn update(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
    fields: IssueFields<'_>,
) -> StorageResult<()> {
    update_guarded(pool, id, project_id, actor_id, fields, None)
        .await
        .map(Guarded::unguarded)
}

/// What [`update_guarded`] reads before writing: status, effort, assignee (to
/// diff into events) and the stamp (to compare).
type PriorIssueState = (String, Option<i64>, Option<String>, DateTime<Utc>);

/// [`update`], applied only if the issue still carries `expected` as its
/// `updated_at` (`RACE-002`).
///
/// **All four guarded issue writes open `BEGIN IMMEDIATE`, not the
/// deferred `BEGIN` they used to.** A deferred transaction takes its
/// read snapshot at the first `SELECT` and only tries for the write lock
/// at the first write; if another writer committed in between, SQLite
/// fails the upgrade at once (`SQLITE_BUSY_SNAPSHOT`, which the busy
/// timeout does not retry) and the request became a `500`. So two saves
/// holding one stamp did not silently overwrite each other here -- the
/// loser was answered with a server error instead of the conflict it was
/// owed. Taking the write lock first makes the loser wait, read the
/// winner's stamp, and be refused as `Stale`.
pub async fn update_guarded(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
    fields: IssueFields<'_>,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<()>> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    // Read the previous values inside the same transaction so we
    // can diff and emit one event per changed field. SELECT
    // first, UPDATE second, COMMIT once — the previous read is
    // self-consistent with the write, and (`RACE-002`) so is the stamp.
    let prev: Option<PriorIssueState> = sqlx::query_as(
        r#"
        SELECT status, effort, assignee_id, updated_at
        FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some((prev_status, prev_effort, prev_assignee, current_updated_at)) = prev else {
        return Err(StorageError::NotFound);
    };
    if !stamp_matches(expected, current_updated_at) {
        return Ok(Guarded::Stale { current_updated_at });
    }

    // CAL-001 §2.4: planned_start_at/planned_end_at join this
    // existing UPDATE's SET clause rather than getting a statement
    // of their own. `updated_at` is not written here — `issues_
    // updated_at` (`0017`, `QA-019`/`NFR-CONC-003`) advances it for
    // any UPDATE that doesn't touch the column itself, so combining
    // every field into one statement is no longer load-bearing for
    // the timestamp the way it once was; it stays combined because
    // splitting it would still cost an extra event-diff read for no
    // benefit.
    let res = sqlx::query(
        r#"
        UPDATE issues
        SET title = ?3, description = ?4, status = ?5, priority = ?6,
            effort = ?7,
            assignee_id = ?8,
            planned_start_at = ?9,
            planned_end_at = ?10
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(fields.title)
    .bind(fields.description)
    .bind(fields.status.as_str())
    .bind(fields.priority.as_str())
    .bind(fields.effort)
    .bind(fields.assignee_id)
    .bind(fields.planned_start_at)
    .bind(fields.planned_end_at)
    .execute(&mut *tx)
    .await;

    let res = match res {
        Ok(r) => r,
        Err(e) => return Err(translate_trigger_error(e)),
    };
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }

    // One event per field that actually moved. We want
    // status_changed events to be the single source of truth for
    // the in-progress dwell-time analysis, so emitting them only
    // on real status moves is important.
    if prev_status != fields.status.as_str() {
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::STATUS_CHANGED,
            Some(&prev_status),
            Some(fields.status.as_str()),
        )
        .await?;
    }

    if prev_effort != fields.effort {
        let prev_str = prev_effort.map(|n| n.to_string());
        let new_str = fields.effort.map(|n| n.to_string());
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::EFFORT_CHANGED,
            prev_str.as_deref(),
            new_str.as_deref(),
        )
        .await?;
    }

    let new_assignee_owned = fields.assignee_id.map(|s| s.to_string());
    if prev_assignee != new_assignee_owned {
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::ASSIGNEE_CHANGED,
            prev_assignee.as_deref(),
            fields.assignee_id,
        )
        .await?;
    }

    tx.commit().await?;
    Ok(Guarded::Written(()))
}

pub async fn update_status(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
    status: IssueStatus,
) -> StorageResult<DateTime<Utc>> {
    update_status_guarded(pool, id, project_id, actor_id, status, None)
        .await
        .map(Guarded::unguarded)
}

/// [`update_status`], applied only if the issue still carries `expected`
/// as its `updated_at` (`RACE-002`; see [`update_guarded`] for why the
/// transaction is `BEGIN IMMEDIATE`). On success returns the *new* stamp,
/// as before -- the caller's next lock value.
pub async fn update_status_guarded(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
    status: IssueStatus,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<DateTime<Utc>>> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    let prev: Option<(String, DateTime<Utc>)> = sqlx::query_as(
        r#"
        SELECT status, updated_at FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some((prev_status, current_updated_at)) = prev else {
        return Err(StorageError::NotFound);
    };
    if !stamp_matches(expected, current_updated_at) {
        return Ok(Guarded::Stale { current_updated_at });
    }

    sqlx::query(
        r#"
        UPDATE issues
        SET status = ?3
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(status.as_str())
    .execute(&mut *tx)
    .await?;

    // `updated_at` is not written above — `issues_updated_at`
    // (`0017`, `QA-019`/`NFR-CONC-003`) advances it. `RETURNING`
    // would have handed back the row as modified by this
    // statement, which is *before* an `AFTER` trigger's own nested
    // `UPDATE` runs (verified directly in `sqlite3`; not documented
    // behaviour this codebase used to rely on), so it would return
    // a value the row has already moved past. A same-transaction
    // read after the trigger has fired sees the real value —
    // STATUS-002 needs that value handed back to the caller as the
    // new lock. The `prev_status` fetch above already establishes
    // the row exists; this `NotFound` path is defensive, matching
    // the shape `RETURNING` had.
    let new_updated_at: DateTime<Utc> = sqlx::query_scalar(
        r#"
        SELECT updated_at FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(StorageError::NotFound)?;

    if prev_status != status.as_str() {
        issue_events::insert_event(
            &mut tx,
            id,
            project_id,
            Some(actor_id),
            issue_events::kind::STATUS_CHANGED,
            Some(&prev_status),
            Some(status.as_str()),
        )
        .await?;
    }

    tx.commit().await?;
    Ok(Guarded::Written(new_updated_at))
}

/// `CAL-003` (RFC 004d D-3): moves both planned timestamps together —
/// a reschedule, not a general edit. Two fields where [`update`] has
/// ten; same shape as [`update_status`] otherwise, including the
/// same-transaction `updated_at` read after the trigger fires (see
/// that function's own comment for why `RETURNING` doesn't work
/// here). No ordering rule between the two — the handoff's §2c found
/// none exists anywhere in this codebase, and a reschedule moves both
/// timestamps by the same delta, so an ordering that was valid stays
/// valid by construction; inventing one here would be new, unasked-for
/// behaviour.
///
/// No event is emitted for the change — [`update`] (the ten-field
/// form) doesn't diff `planned_start_at`/`planned_end_at` into an
/// event either (only status/effort/assignee), so this doesn't
/// introduce an inconsistency; it follows the existing precedent.
pub async fn update_schedule(
    pool: &Pool,
    id: &str,
    project_id: &str,
    planned_start_at: Option<DateTime<Utc>>,
    planned_end_at: Option<DateTime<Utc>>,
) -> StorageResult<DateTime<Utc>> {
    update_schedule_guarded(pool, id, project_id, planned_start_at, planned_end_at, None)
        .await
        .map(Guarded::unguarded)
}

/// [`update_schedule`], applied only if the issue still carries
/// `expected` as its `updated_at` (`RACE-002`; see [`update_guarded`]).
/// Returns the new stamp on success.
pub async fn update_schedule_guarded(
    pool: &Pool,
    id: &str,
    project_id: &str,
    planned_start_at: Option<DateTime<Utc>>,
    planned_end_at: Option<DateTime<Utc>>,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<DateTime<Utc>>> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    let current_updated_at: DateTime<Utc> = sqlx::query_scalar(
        r#"
        SELECT updated_at FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(StorageError::NotFound)?;
    if !stamp_matches(expected, current_updated_at) {
        return Ok(Guarded::Stale { current_updated_at });
    }

    let res = sqlx::query(
        r#"
        UPDATE issues
        SET planned_start_at = ?3, planned_end_at = ?4
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(planned_start_at)
    .bind(planned_end_at)
    .execute(&mut *tx)
    .await;

    let res = match res {
        Ok(r) => r,
        Err(e) => return Err(translate_trigger_error(e)),
    };
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }

    // Same reasoning as `update_status`: a same-transaction read
    // after the `AFTER` trigger has advanced `updated_at` is the one
    // way to hand the caller the real new lock value.
    let new_updated_at: DateTime<Utc> = sqlx::query_scalar(
        r#"
        SELECT updated_at FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(StorageError::NotFound)?;

    tx.commit().await?;
    Ok(Guarded::Written(new_updated_at))
}

pub async fn delete(pool: &Pool, id: &str, project_id: &str, actor_id: &str) -> StorageResult<()> {
    delete_guarded(pool, id, project_id, actor_id, None)
        .await
        .map(Guarded::unguarded)
}

/// [`delete`], applied only if the issue still carries `expected` as its
/// `updated_at` (`RACE-002`; see [`update_guarded`]). A delete after a
/// concurrent edit is refused, so the confirmation's sub-issue count
/// (`QA-006`) is still the one the delete acts on.
pub async fn delete_guarded(
    pool: &Pool,
    id: &str,
    project_id: &str,
    actor_id: &str,
    expected: Option<DateTime<Utc>>,
) -> StorageResult<Guarded<()>> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    // Read the previous status / current state so we can record
    // it in the deletion event. After the DELETE, the cascade
    // SET NULL on issue_id loses the link, but the project_id and
    // event metadata still tell the story.
    let prev: Option<(String, DateTime<Utc>)> = sqlx::query_as(
        r#"SELECT status, updated_at FROM issues WHERE id = ?1 AND project_id = ?2"#,
    )
    .bind(id)
    .bind(project_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some((prev_status, current_updated_at)) = prev else {
        return Err(StorageError::NotFound);
    };
    if !stamp_matches(expected, current_updated_at) {
        return Ok(Guarded::Stale { current_updated_at });
    }
    let prev_status = Some(prev_status);

    // Write the deletion event BEFORE the actual delete, while
    // the issue_id reference is still valid. The CASCADE then
    // sets issue_id to NULL on this freshly-written row, leaving
    // it standalone in the log. previous_value carries the
    // last-known status as a small affordance.
    issue_events::insert_event(
        &mut tx,
        id,
        project_id,
        Some(actor_id),
        issue_events::kind::DELETED,
        prev_status.as_deref(),
        None,
    )
    .await?;

    let res = sqlx::query(
        r#"
        DELETE FROM issues
        WHERE id = ?1 AND project_id = ?2
        "#,
    )
    .bind(id)
    .bind(project_id)
    .execute(&mut *tx)
    .await?;
    if res.rows_affected() == 0 {
        return Err(StorageError::NotFound);
    }

    tx.commit().await?;
    Ok(Guarded::Written(()))
}

/// The assignee-candidate set for a project, as a `WITH` CTE
/// fragment: the project's owner, plus any `admin`/`member` (never
/// `viewer`) of the project's team, if it has one.
///
/// `TEAM-001` / RFC 009 §D1: `list_assignee_candidates` and
/// `project_workload` used to be two independently-written queries
/// that both joined `projects p ON p.owner_id = u.id` — a join that
/// can only ever produce the owner, regardless of team membership.
/// They diverged from the same wrong text because they were written
/// twice; the fix is one definition both derive from, not two
/// corrected copies.
///
/// **Mechanism chosen**: a `WITH` CTE, embedded as a shared Rust
/// `&str` constant in each query, rather than a SQL view or the
/// `query_as!` compile-time macro. A view is a migration and a
/// bigger commitment than this fix needs; `peisear-storage` has no
/// `query_as!`/`query!` call anywhere (confirmed by grep) and no
/// `DATABASE_URL`/offline-cache machinery to support one, so
/// introducing the macro here would be a first for the crate, not a
/// fit with this fix's scope. The runtime-checked `query_as::<_,
/// T>()` form every other query in this file uses is preserved.
///
/// `?1` is the project id, bound once and reused — this project's
/// existing convention for repeated positional SQLite params (see
/// `projects::list_for_user`'s `?1` reused three times with one
/// `.bind()`).
///
/// `viewer` is excluded in the `LEFT JOIN`'s `ON` clause, not a
/// `WHERE` filter: a `WHERE tm.role IN (...)` would still be
/// correct here, since it only ever narrows which `tm` rows can
/// satisfy `u.id = tm.user_id`, but filtering in the join condition
/// makes the exclusion visible at the point the membership rows are
/// selected, alongside the team-id join, rather than downstream of
/// it — `0011_teams.sql`'s own comment: a viewer is read-only, no
/// assignment. RFC 009 §D1's own sample SQL has no role filter at
/// all (it would incorrectly admit viewers); this fixes that gap
/// too, per handoff §4's explicit requirement that only `admin`/
/// `member` are candidates.
///
/// The `LEFT JOIN` cross-matches every user against every
/// membership row for the project's team (it is not correlated to
/// `u` in the join condition), so a user who is both the owner and
/// a team member can produce more than one matching row before
/// `GROUP BY`/`DISTINCT` collapses it — same shape RFC 009 §D1's own
/// sample query has, and why it (and this one) end in `GROUP BY`/
/// `SELECT DISTINCT`.
const CANDIDATE_SET_CTE: &str = r#"
    candidates AS (
        SELECT DISTINCT u.id, u.display_name
        FROM users u
        JOIN projects p ON p.id = ?1
        LEFT JOIN team_memberships tm
               ON tm.team_id = p.team_id AND tm.role IN ('admin', 'member')
        WHERE u.id = p.owner_id OR u.id = tm.user_id
    )
"#;

/// List the users who are valid assignee candidates for issues in a
/// given project: the project's owner, plus any `admin`/`member` of
/// the project's team. A personal project (`team_id IS NULL`) yields
/// exactly the owner, since the `LEFT JOIN` in [`CANDIDATE_SET_CTE`]
/// then contributes nothing.
pub async fn list_assignee_candidates(
    pool: &Pool,
    project_id: &str,
) -> StorageResult<Vec<peisear_core::AssigneeOption>> {
    sqlx::query_as::<_, (String, String)>(&format!(
        r#"
        WITH {CANDIDATE_SET_CTE}
        SELECT id, display_name FROM candidates
        ORDER BY display_name ASC
        "#,
    ))
    .bind(project_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, display_name)| Ok(peisear_core::AssigneeOption { id, display_name }))
    .collect()
}

/// Per-user workload report for a project.
///
/// Returns one [`peisear_core::UserLoad`] per assignee candidate ([`CANDIDATE_SET_CTE`]),
/// **plus** any user holding an in-flight issue in the project even if
/// they are no longer a candidate (RFC 009 §D3, settled: a user
/// removed from a team keeps issues already assigned to them, so the
/// candidate set is a subset of the workload set, not equal to it —
/// the form describes policy going forward, the report describes
/// reality). Users with no in-flight issues still appear in the
/// result with zero counts so the UI can show their (empty) chip.
///
/// Today this is a project-level report. The query intentionally
/// keeps a `project_id` filter even though the user-level capacity
/// is global — this means the chip strip on a project page reflects
/// "what this team has on its plate from this project". The full
/// global view (across all projects a user is on) is a future
/// addition that takes the same shape with `project_id = NULL`.
///
/// When period support lands, this query gains a period filter on
/// the issue side without changing the [`peisear_core::UserLoad`] result shape.
pub async fn project_workload(
    pool: &Pool,
    project_id: &str,
) -> StorageResult<Vec<peisear_core::UserLoad>> {
    // 0.12.0: capacity comes from `user_capacities`, resolved as
    // "the row whose period covers today, most recently created".
    // The correlated sub-select keeps the per-user join tractable
    // even with the period filter.
    sqlx::query_as::<_, (String, String, Option<i64>, Option<i64>, i64)>(&format!(
        r#"
        WITH {CANDIDATE_SET_CTE}
        SELECT
            u.id,
            u.display_name,
            (SELECT uc.points
             FROM user_capacities uc
             WHERE uc.user_id = u.id
               AND (uc.period_start IS NULL OR uc.period_start <= date('now'))
               AND (uc.period_end   IS NULL OR uc.period_end   >= date('now'))
             ORDER BY uc.created_at DESC, uc.rowid DESC
             LIMIT 1
            ) AS capacity_points,
            COALESCE(SUM(CASE
                WHEN i.status IN ('open', 'in_progress') THEN i.effort
                ELSE NULL
            END), 0) AS in_flight_points,
            COALESCE(SUM(CASE
                WHEN i.status IN ('open', 'in_progress') THEN 1
                ELSE 0
            END), 0) AS in_flight_issues
        FROM users u
        LEFT JOIN issues i ON i.assignee_id = u.id AND i.project_id = ?1
        WHERE u.id IN (SELECT id FROM candidates)
           OR EXISTS (
                SELECT 1 FROM issues i2
                WHERE i2.assignee_id = u.id
                  AND i2.project_id = ?1
                  AND i2.status IN ('open', 'in_progress')
              )
        GROUP BY u.id, u.display_name
        ORDER BY u.display_name ASC
        "#,
    ))
    .bind(project_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(
        |(user_id, display_name, capacity_points, in_flight_points, in_flight_issues)| {
            Ok(peisear_core::UserLoad {
                user_id,
                display_name,
                capacity_points,
                in_flight_points: in_flight_points.unwrap_or(0),
                in_flight_issues,
            })
        },
    )
    .collect()
}

// ──────────────────────────────────────────────────────────────
// Calendar window queries (`CAL-001` / RFC 002)
// ──────────────────────────────────────────────────────────────

/// The overlap predicate RFC 002 §Design specifies, as a `WHERE`
/// fragment shared by [`planned_for_user`] and [`planned_for_project`]
/// — one definition, not two independently-written copies of the
/// same three-way NULL-handling logic. `?2`/`?3` (`from`/`to`) are
/// hardcoded because both callers bind their scope value first (`?1`
/// — assignee or project) and `from`/`to` second and third, in that
/// order, so the positions are identical in both queries; this is
/// the same `?N`-reuse-via-shared-const-`&str`-embedded-with-`format!`
/// mechanism `TEAM-001`'s `CANDIDATE_SET_CTE` established.
///
/// A `NULL` `planned_end_at` is treated as a half-hour anchor at
/// `planned_start_at` (must-have 5) — the second arm of the `OR`
/// keeps such a row in the overlap set using `planned_start_at`
/// itself as the stand-in end bound, so it appears whenever the
/// window covers its start instant.
const PLANNED_WINDOW_OVERLAP_PREDICATE: &str = r#"
    planned_start_at IS NOT NULL
    AND (
        (planned_end_at IS NOT NULL AND planned_end_at >= ?2)
        OR (planned_end_at IS NULL AND planned_start_at >= ?2)
    )
    AND planned_start_at <= ?3
"#;

/// Issues planned to overlap `[from, to]` for the given assignee.
/// Personal axis (`/today/calendar`, CAL-002) — self-only by
/// construction (`§11.5`): the caller supplies `user_id`, there is no
/// path to another user's planned issues through this function.
///
/// Top-level only — `CAL-002` §5 test 10 ("sub-issues appear on
/// neither axis"). This filter was missing when `CAL-001` shipped
/// this function: RFC 002 must-have 6 states "top-level" explicitly
/// for the project axis but only implies it for the personal one
/// ("sub-issues follow parent; they don't appear separately on the
/// calendar" reads as a general rule, not a project-axis-only one),
/// and `planned_for_project` already had the filter while this one
/// didn't. Found while implementing CAL-002's test 10, fixed here
/// rather than worked around at the render layer.
pub async fn planned_for_user(
    pool: &Pool,
    user_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(&format!(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE assignee_id = ?1
          AND parent_issue_id IS NULL
          AND {PLANNED_WINDOW_OVERLAP_PREDICATE}
        ORDER BY planned_start_at ASC, rowid ASC
        "#,
    ))
    .bind(user_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}

/// Issues planned to overlap `[from, to]` in the given project.
/// Top-level only — sub-issues inherit their parent's position on
/// the calendar by way of the parent appearing in the result (RFC
/// 002 §Design), matching [`list_in_project`]'s existing
/// top-level-only filter.
pub async fn planned_for_project(
    pool: &Pool,
    project_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> StorageResult<Vec<Issue>> {
    let rows = sqlx::query_as::<_, IssueRow>(&format!(
        r#"
        SELECT id, project_id, author_id, title, description,
               status, priority, effort, assignee_id, parent_issue_id,
               planned_start_at, planned_end_at,
               created_at, updated_at
        FROM issues
        WHERE project_id = ?1
          AND parent_issue_id IS NULL
          AND {PLANNED_WINDOW_OVERLAP_PREDICATE}
        ORDER BY planned_start_at ASC, rowid ASC
        "#,
    ))
    .bind(project_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(IssueRow::into_issue).collect()
}
