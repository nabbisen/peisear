-- 0012_sprints.sql
-- Sprint feature (0.15.0).
--
-- Phase 1 design: Jira-style variable-length sprints. Admins
-- create one with start_on / end_on dates; sprints run their
-- course; then either auto-finalise (we'll skip auto-finalise
-- in Phase 1) or admin marks them complete. Auto-rolling
-- cadence (Linear style) is deferred to Phase 2.
--
-- ## Why team_id-scoped, not project_id-scoped
--
-- A sprint is a *team's* time-boxed unit of planning. Issues
-- are committed to a sprint regardless of which project they
-- belong to (within the sprint's team). This matches Jira's
-- model and works correctly when a team has multiple projects.
--
-- Cross-team sprints (one sprint spanning multiple teams) are
-- not supported in Phase 1; the schema reserves room only for
-- single-team sprints. If cross-team work becomes a real need,
-- a `sprint_team_assignments` join would be the addition.
--
-- ## Why sprints don't extend `metrics_snapshots`
--
-- We considered storing per-snapshot sprint progress as a
-- column on `metrics_snapshots`. Decided against: snapshots
-- are project-scoped and sprints are team-scoped, mixing the
-- two would couple two unrelated subsystems. The burndown is
-- computed live from `issues` + `issue_events` (status_changed
-- events) — this is the same source-of-truth that powers the
-- 0.8.0 dwell-time math, so it's already trustworthy.
--
-- ## Sprint status transitions
--
-- - `planned` → admin can edit dates, add/remove issues
-- - `active` → in flight; admin starts the sprint when ready
-- - `completed` → done; sprint review numbers are final
--
-- Transitions are admin actions (no auto-promotion based on
-- date). This is deliberate: an admin starting / completing a
-- sprint is the **explicit event** that V2.1 §4.4 calls for.
-- Auto-promotion would be convenient but elides the moment of
-- decision.

CREATE TABLE sprints (
    id          TEXT PRIMARY KEY,
    team_id     TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,

    -- Display name. Free-form; common patterns are "Sprint 1",
    -- "2026-W17", "Q2 sprint 3", "Auth refresh sprint".
    name        TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 120),

    -- Optional sprint goal (free text).
    goal        TEXT,

    -- Inclusive date bounds. Stored as DATE (no time component).
    -- Both required at create time — a sprint without dates
    -- doesn't make sense as a time-boxed unit. The table-level
    -- CHECK below ensures end is on or after start.
    starts_on   DATE NOT NULL,
    ends_on     DATE NOT NULL,

    -- Lifecycle state. Application transitions explicitly.
    status      TEXT NOT NULL DEFAULT 'planned'
                CHECK (status IN ('planned', 'active', 'completed')),

    -- Set when admin starts the sprint (status -> active).
    started_at  DATETIME,
    -- Set when admin completes the sprint (status -> completed).
    completed_at DATETIME,

    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Table-level constraints come after all columns. SQLite's
    -- parser doesn't accept table constraints interleaved with
    -- column definitions.
    CHECK (starts_on <= ends_on)
);

-- Sprint listing for a team is the most common access path:
-- "show me this team's sprints, newest first / by status".
CREATE INDEX idx_sprints_team_status ON sprints(team_id, status, starts_on DESC);

-- Sprint-by-date queries (e.g., "what sprint covers today?")
-- are uncommon but useful enough to index.
CREATE INDEX idx_sprints_team_dates ON sprints(team_id, starts_on, ends_on);


-- Issue ↔ Sprint membership.
--
-- One issue belongs to at most one sprint per the natural
-- mental model ("this work is in this sprint, period"). The
-- PRIMARY KEY on issue_id alone enforces this.
--
-- An issue can be moved between sprints (handler updates the
-- row's sprint_id; carry-over flow uses this). Removal from a
-- sprint deletes the row entirely.
--
-- assigned_at provides ordering for the "when did this come in"
-- question on the sprint detail page; not used for analytics
-- yet but cheap to record.
CREATE TABLE sprint_issues (
    issue_id    TEXT PRIMARY KEY REFERENCES issues(id) ON DELETE CASCADE,
    sprint_id   TEXT NOT NULL REFERENCES sprints(id) ON DELETE CASCADE,
    assigned_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Reverse lookup: "issues in this sprint".
CREATE INDEX idx_sprint_issues_sprint ON sprint_issues(sprint_id, assigned_at);
