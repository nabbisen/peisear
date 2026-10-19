-- 0019_sprint_records.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0019_sprint_records.sql. The backfill's
-- own reasoning (faithful to what 0.38.0 displayed, wrong by
-- exactly the drift that has already happened) is unchanged; this
-- file's notes are about syntax only.
--
-- Porting notes:
-- - `DATETIME`/`DATE` -> `TIMESTAMPTZ`/`DATE`, same reasons as
--   `0001`/`0012`.
-- - `date(x)` (SQLite's date-truncation function) -> `x::date`
--   (PostgreSQL cast). Three call sites: the assignment day, the
--   latest `status_changed` event's day, and the fallback
--   `updated_at` day.
-- - `date(COALESCE(s.completed_at, 'now'))` -> `COALESCE(s.completed_at::date,
--   CURRENT_DATE)`. SQLite's `date()` recognises the bare string
--   `'now'` as a modifier keyword meaning "today"; PostgreSQL has
--   no equivalent overload, so the `COALESCE` moves outside the
--   cast and `CURRENT_DATE` stands in for `date('now')` directly.
-- - `MIN(a, b, c)` (SQLite's scalar, multi-argument `MIN` -- not
--   the aggregate) -> `LEAST(a, b, c)`, PostgreSQL's own name for
--   the same operation; PostgreSQL's own `MIN`/`MAX` are
--   aggregates only and do not take multiple scalar arguments.
-- - `date(day, '+1 day')` -> `day + 1`. PostgreSQL defines
--   `date + integer` as "add N days", returning `date` directly --
--   simpler than `+ INTERVAL '1 day'`, which would return
--   `timestamp` and need casting back.
-- - On a database migrating from empty (this round's own gate),
--   `sprints` has no `status = 'completed'` rows, so both
--   backfill statements run as before: real SQL, zero rows
--   affected.

CREATE TABLE sprint_records (
    sprint_id        TEXT PRIMARY KEY REFERENCES sprints(id) ON DELETE CASCADE,
    committed_points INTEGER NOT NULL,
    completed_points INTEGER NOT NULL,
    committed_count  INTEGER NOT NULL,
    completed_count  INTEGER NOT NULL,
    captured_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sprint_burndown_points (
    sprint_id            TEXT NOT NULL REFERENCES sprints(id) ON DELETE CASCADE,
    day                  DATE NOT NULL,
    cumulative_committed INTEGER NOT NULL,
    cumulative_completed INTEGER NOT NULL,
    PRIMARY KEY (sprint_id, day)
);

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
        si.assigned_at::date AS assigned_day,
        COALESCE(
            (SELECT MAX(e.occurred_at)::date
               FROM issue_events e
              WHERE e.issue_id = i.id
                AND e.event_type = 'status_changed'
                AND e.new_value = 'done'),
            CASE WHEN i.status = 'done' THEN i.updated_at::date END
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
        LEAST(s.ends_on, COALESCE(s.completed_at::date, CURRENT_DATE), CURRENT_DATE) AS end_day
    FROM sprints s
    WHERE s.status = 'completed'
      AND EXISTS (SELECT 1 FROM sprint_issues si WHERE si.sprint_id = s.id)
),
days(sprint_id, day, end_day) AS (
    SELECT sprint_id, start_day, end_day FROM bounds WHERE start_day <= end_day
    UNION ALL
    SELECT sprint_id, day + 1, end_day FROM days WHERE day < end_day
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
