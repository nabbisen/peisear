-- 0006_issue_events.sql
-- Issue event log: an append-only record of what happened to each
-- issue, written in the same transaction as the issue mutation
-- itself. This is the foundation of Phase 2 of the Health & Burnout
-- extension.
--
-- ## Why have it at all
--
-- Phase 1 (0.7.0) derived signals from `issues.{updated_at, status}`.
-- That left several limitations documented in
-- `peisear-storage/src/project_health.rs`:
--
-- - "long-stale" detection used `updated_at`, which is overwritten
--   by *any* row update (priority bump, etc.), not just status
--   movement.
-- - Cycle time ("how long was this issue actively in progress?")
--   was uncomputable.
-- - Personal estimation skew was calendar-time-per-point, including
--   weekends and time the issue sat untouched.
--
-- An event log lifts all three: status transitions are first-class
-- events, so we can compute "minutes spent in each status" and
-- "elapsed since last status change" cleanly.
--
-- ## Schema rationale
--
-- - `id`: UUID primary key. Events never get rewritten, so a stable
--   identifier is plenty; we don't need ordering by id.
--
-- - `issue_id`: nullable + ON DELETE SET NULL. Per the design
--   discussion, the event log is a monotonically appendable record;
--   deleting the issue should not vaporise its history. Future
--   audit-trail uses depend on this. The 'deleted' event itself is
--   the last record before the issue_id goes NULL via cascade.
--
-- - `project_id`: denormalised from issues so per-project event
--   queries don't need a join. Set to the project at event-write
--   time and never updated. Survives issue deletion.
--
-- - `actor_id`: who did this. Nullable + ON DELETE SET NULL so
--   deleting the user doesn't lose the event; the event simply
--   becomes "someone (now departed) did X". 0.8.0 has owner=self,
--   so actor_id always equals the project owner today, but the
--   field exists from the start to avoid a breaking migration when
--   manager / neutral-third-party roles arrive.
--
-- - `event_type`: enum-encoded as TEXT (matching the
--   `IssueStatus`/`Priority` precedent in the schema). Values used
--   in 0.8.0:
--     'created'           — issue first inserted
--     'status_changed'    — IssueStatus value transitioned
--     'assignee_changed'  — assignee_id changed (incl. to/from NULL)
--     'effort_changed'    — effort changed (incl. to/from NULL)
--     'deleted'           — issue removed (last event before
--                            issue_id goes NULL via cascade SET NULL)
--   Future event types ('priority_changed', 'title_changed', etc.)
--   land additively without schema change.
--
-- - `previous_value` / `new_value`: TEXT, nullable. JSON-encoded
--   when the value is structured, otherwise the bare scalar. For
--   `status_changed` these are the IssueStatus discriminant strings
--   ('open', 'in_progress', 'done'); for `effort_changed` they are
--   numeric strings or NULL; for `assignee_changed` they are user
--   UUIDs or NULL.
--
-- - `occurred_at`: server-clock timestamp at the moment of the
--   transaction. Indexed for time-range queries.
--
-- ## Compatibility with pre-0.8.0 data
--
-- Issues that existed before this migration have no events. The
-- query layer is written with that in mind: it falls back to the
-- 0.7.0 calendar-time approximation when no event log exists for an
-- issue. New events get written from this release onward, so the
-- precision improves over time without backfilling.
--
-- We do NOT backfill synthetic events for existing issues. Two
-- reasons:
--
--   1. We don't know when the historical status transitions
--      actually happened — only `created_at` is real. Inventing
--      'status_changed' events at created_at would be a lie that
--      makes some indicators worse, not better.
--   2. The Phase 1 fallback is documented and bounded. Users will
--      see precision improve naturally as they work, with no
--      mysterious data manufactured behind their back.

CREATE TABLE issue_events (
    id             TEXT PRIMARY KEY,
    issue_id       TEXT REFERENCES issues(id) ON DELETE SET NULL,
    project_id     TEXT NOT NULL,
    actor_id       TEXT REFERENCES users(id) ON DELETE SET NULL,
    event_type     TEXT NOT NULL CHECK (event_type IN (
        'created',
        'status_changed',
        'assignee_changed',
        'effort_changed',
        'deleted'
    )),
    previous_value TEXT,
    new_value      TEXT,
    occurred_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Per-issue history queries: "all events for this issue in order".
-- The most-asked variant ("most recent status_changed event") uses
-- this index with a `WHERE event_type = 'status_changed'` filter.
CREATE INDEX idx_issue_events_issue ON issue_events(issue_id, occurred_at);

-- Per-project audit queries: "all events in this project in time
-- order". Used by the Phase 2 long-stale detection (latest status
-- change per in-flight issue) and by the planned audit log view.
CREATE INDEX idx_issue_events_project ON issue_events(project_id, occurred_at);

-- Per-user activity: "what did this person do recently". Useful
-- for the planned manager / neutral-third-party views, and for
-- per-user burnout signals in 0.10.0.
CREATE INDEX idx_issue_events_actor ON issue_events(actor_id, occurred_at);
