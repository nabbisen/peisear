-- 0019_sprint_records.sql
--
-- `SPRINT-004` (RFC 0013, `DEC-054`): the record of a completed sprint is
-- captured at completion, not computed.
--
-- ## Why
--
-- A completed sprint's figures (`sprints::summary`) and burndown
-- (`sprints::burndown`) were computed *live* from four inputs: its
-- membership (`sprint_issues`) and each member issue's `status`, `effort`
-- and `updated_at`. So a completed sprint's record drifted with no change
-- to the sprint at all -- carry an unfinished issue into the next sprint
-- and finish it a month later, and the *completed* sprint's completed
-- figure rose and its carried-over figure fell. A comment in `summary` said
-- the figures "capture the moment"; they never did.
--
-- `complete` now writes what the sprint reported into the two tables below,
-- inside the transaction that sets the status; a `completed` sprint reads
-- them; `reopen` deletes them. Membership and issues stay freely editable,
-- which is what carry-over needs.
--
-- ## Schema
--
-- Scalar columns, following `metrics_snapshots` / `user_metrics_snapshots`
-- -- not JSON. **Carried-over is not stored**: it is committed minus
-- completed, and a stored copy invites the two to disagree.
--
-- Both tables cascade from `sprints`: deleting a sprint deletes its record.
CREATE TABLE sprint_records (
    sprint_id        TEXT PRIMARY KEY REFERENCES sprints(id) ON DELETE CASCADE,
    committed_points INTEGER NOT NULL,
    completed_points INTEGER NOT NULL,
    committed_count  INTEGER NOT NULL,
    completed_count  INTEGER NOT NULL,
    captured_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- One row per sprint per day: `peisear_core::sprints::BurndownPoint` as it
-- already exists. A burndown is a series and a series is rows.
CREATE TABLE sprint_burndown_points (
    sprint_id            TEXT NOT NULL REFERENCES sprints(id) ON DELETE CASCADE,
    day                  DATE NOT NULL,
    cumulative_committed INTEGER NOT NULL,
    cumulative_completed INTEGER NOT NULL,
    PRIMARY KEY (sprint_id, day)
);

-- ## Backfill -- faithful to what 0.38.0 displayed, and wrong on purpose
--
-- Every sprint that is already `completed` gets a record, computed here from
-- the live data exactly as `summary` and `burndown` computed it. **It is
-- wrong by exactly the drift that has already happened** -- a sprint whose
-- issues were carried over and finished since it completed is captured with
-- today's figures, not the ones it reported -- and that is accepted: a sprint
-- reporting a drifted figure consistently forever is better than one that
-- keeps drifting, and leaving them computing live would make two classes of
-- completed sprint with nothing to tell them apart.
--
-- The computation is expressible in SQL, so it is done here rather than
-- approximated. What it reproduces:
--
-- * summary: SUM of effort (NULL = 0) over current members, the same over
--   members whose status is 'done', and the two counts -- `sprints::summary`'s
--   query verbatim. An empty sprint captures four zeros.
-- * burndown, per member issue: `assigned_day` = the date of its
--   `sprint_issues.assigned_at`; `done_day` = the date of the *latest*
--   `status_changed -> done` event for the issue, else -- for an issue whose
--   current status is 'done' with no such event (the 0.7.0-era fallback) --
--   the date of its `updated_at`. One row per day from `starts_on` to
--   min(`ends_on`, date(`completed_at`), today), where `cumulative_committed`
--   on day D is the effort of members with `assigned_day <= D` and
--   `cumulative_completed` the effort of members with `done_day <= D`. A
--   sprint with no members has no series (`burndown` returned an empty
--   vector for it), and a window that ends before it starts has none either.
--
-- Nothing else is read.
INSERT INTO sprint_records
    (sprint_id, committed_points, completed_points, committed_count, completed_count)
SELECT
    s.id,
    COALESCE(SUM(COALESCE(i.effort, 0)), 0),
    COALESCE(SUM(CASE WHEN i.status = 'done' THEN COALESCE(i.effort, 0) ELSE 0 END), 0),
    COUNT(i.id),
    COALESCE(SUM(CASE WHEN i.status = 'done' THEN 1 ELSE 0 END), 0)
FROM sprints s
LEFT JOIN sprint_issues si ON si.sprint_id = s.id
LEFT JOIN issues i ON i.id = si.issue_id
WHERE s.status = 'completed'
GROUP BY s.id;

WITH RECURSIVE
members AS (
    SELECT
        s.id AS sprint_id,
        COALESCE(i.effort, 0) AS effort,
        date(si.assigned_at) AS assigned_day,
        COALESCE(
            (SELECT date(MAX(e.occurred_at))
               FROM issue_events e
              WHERE e.issue_id = i.id
                AND e.event_type = 'status_changed'
                AND e.new_value = 'done'),
            CASE WHEN i.status = 'done' THEN date(i.updated_at) END
        ) AS done_day
    FROM sprints s
    JOIN sprint_issues si ON si.sprint_id = s.id
    JOIN issues i ON i.id = si.issue_id
    WHERE s.status = 'completed'
),
bounds AS (
    SELECT
        s.id AS sprint_id,
        s.starts_on AS start_day,
        MIN(s.ends_on, date(COALESCE(s.completed_at, 'now')), date('now')) AS end_day
    FROM sprints s
    WHERE s.status = 'completed'
      AND EXISTS (SELECT 1 FROM sprint_issues si WHERE si.sprint_id = s.id)
),
days(sprint_id, day, end_day) AS (
    SELECT sprint_id, start_day, end_day FROM bounds WHERE start_day <= end_day
    UNION ALL
    SELECT sprint_id, date(day, '+1 day'), end_day FROM days WHERE day < end_day
)
INSERT INTO sprint_burndown_points
    (sprint_id, day, cumulative_committed, cumulative_completed)
SELECT
    d.sprint_id,
    d.day,
    COALESCE(SUM(CASE WHEN m.assigned_day <= d.day THEN m.effort ELSE 0 END), 0),
    COALESCE(SUM(CASE WHEN m.done_day IS NOT NULL AND m.done_day <= d.day THEN m.effort ELSE 0 END), 0)
FROM days d
JOIN members m ON m.sprint_id = d.sprint_id
GROUP BY d.sprint_id, d.day;
