-- 0009_user_capacities.sql
-- Replaces the static `users.capacity_points` field (introduced
-- in 0004_user_capacity.sql) with a period-aware capacity table.
--
-- ## Why this is a breaking change
--
-- 0.5.0 introduced `users.capacity_points` as a single integer per
-- user. This worked for "what's my capacity right now?" but not
-- for any of:
--
-- - **Sprint cadence.** Capacity that legitimately differs across
--   sprints needs a way to say "20 from Aug 1, 30 from Aug 15"
--   without rewriting history.
-- - **Time off.** A week of leave is a period of capacity 0,
--   bracketed cleanly between normal weeks. Writing 0 to the
--   default would be wrong; writing it back afterwards is
--   error-prone.
-- - **Snapshot honesty.** `user_metrics_snapshots` (0.10.0) stores
--   the resolved capacity at write time. Today that resolution is
--   trivial (just the user row); after this migration it's "the
--   row from `user_capacities` that was effective on the
--   captured_at date". Snapshots remain immutable; the new table
--   becomes the source for *current* and *future* values.
--
-- ## The schema decision (no overrides)
--
-- An earlier draft kept `users.capacity_points` as the default and
-- added `user_capacities` as overrides. That meant two sources of
-- truth and the question "why is my capacity 15 today?" required
-- consulting both. Per the design discussion, the cost of having
-- one source clearly outweighs the migration disruption: any user
-- not on 0.12.0 yet hits this migration once and stops thinking
-- about it.
--
-- After this migration, the answer to "what's the capacity?" is
-- always "look at the `user_capacities` row whose period covers
-- the date in question". One source, one answer.
--
-- ## Why CHECK doesn't enforce non-overlap (and what does)
--
-- SQLite's row-level CHECK can compare columns within the same
-- row but cannot reach into other rows of the same table. A
-- table-level "no two rows for the same user_id have overlapping
-- periods" constraint would need either a trigger (tedious to
-- review and easy to bypass) or application-layer enforcement.
--
-- We chose application-layer: every INSERT and UPDATE in
-- `peisear-storage::user_capacities` calls `overlaps_existing`
-- first and refuses on conflict. The schema's CHECK still
-- catches the in-row inconsistency (period_start > period_end),
-- which is the only thing that makes sense at the row level.
--
-- ## Period semantics
--
-- - `period_start IS NULL` means "from the dawn of time".
--   Effectively the row is the default capacity for any date
--   ≤ period_end (or all dates if period_end is also NULL).
-- - `period_end IS NULL` means "until further notice". Useful
--   for the open-ended "this is my capacity now" row.
-- - Both NULL is the "no period at all" row — the migration
--   creates exactly one such row per pre-existing user, and
--   the application's overlap check ensures only one such row
--   exists at a time per user.

CREATE TABLE user_capacities (
    id            TEXT PRIMARY KEY,
    user_id       TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    points        INTEGER NOT NULL CHECK (points >= 0),
    period_start  DATE,
    period_end    DATE,
    note          TEXT,
    created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Row-level coherence: if both bounds exist, start must
    -- precede or equal end. NULLs are permissive on either side.
    CHECK (period_start IS NULL OR period_end IS NULL OR period_start <= period_end)
);

-- Resolution queries are "given user_id and target_date, find the
-- one row whose period covers it". The composite index supports
-- both the user_id filter and the period range.
CREATE INDEX idx_user_capacities_user_period
    ON user_capacities(user_id, period_start, period_end);

-- Migrate existing data: each user with a non-NULL capacity gets
-- one open-ended default row (period_start = period_end = NULL).
-- Note text records provenance for future operators reading
-- backups.
--
-- We use lower(hex(randomblob(16))) to make a UUID-shaped string
-- without depending on the application layer; the migration runs
-- in pure SQL.
INSERT INTO user_capacities (id, user_id, points, period_start, period_end, note)
SELECT
    lower(hex(randomblob(16))),
    id,
    capacity_points,
    NULL,
    NULL,
    'migrated from 0.11.0 users.capacity_points'
FROM users
WHERE capacity_points IS NOT NULL;

-- Drop the old column. SQLite ≥ 3.35 (we target 3.40+) supports
-- ALTER TABLE DROP COLUMN; this is the breaking part of the
-- breaking change.
ALTER TABLE users DROP COLUMN capacity_points;
