-- 0004_user_capacity.sql
-- Add a per-user global capacity (story points) for workload-fairness.
--
-- This is the demand/supply pair to the per-issue effort field shipped
-- in 0.3.0 and the per-issue assignee shipped in 0.4.0:
--
--   demand  = Σ effort over my assigned, in-flight issues
--   supply  = my capacity_points
--   warning = demand > supply
--
-- Nullable on purpose. NULL means "not set, do not show a warning" —
-- the user has not yet opted in to capacity management. A non-NULL
-- value is the user's stated commitment for what they can carry at
-- once.
--
-- Future direction (period support):
--
-- This is intentionally a single global integer, not a periodised
-- table. When a future release introduces sprint / week / month
-- periods, the migration path is:
--
--   CREATE TABLE user_capacities (
--       user_id       TEXT NOT NULL,
--       period_start  TEXT NULL,   -- NULL = global / "current"
--       period_end    TEXT NULL,
--       points_limit  INTEGER NOT NULL CHECK (points_limit > 0),
--       PRIMARY KEY (user_id, period_start, period_end)
--   );
--   INSERT INTO user_capacities (user_id, period_start, period_end, points_limit)
--   SELECT id, NULL, NULL, capacity_points FROM users WHERE capacity_points IS NOT NULL;
--   ALTER TABLE users DROP COLUMN capacity_points;
--
-- Until that day, callers see a single Option<i64> on the user.

ALTER TABLE users
    ADD COLUMN capacity_points INTEGER NULL
        CHECK (capacity_points IS NULL OR capacity_points > 0);
