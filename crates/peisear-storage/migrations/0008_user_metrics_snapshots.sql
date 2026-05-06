-- 0008_user_metrics_snapshots.sql
-- Per-user point-in-time snapshots of personal load metrics, used
-- by the user_burnout module to detect sustained-overload streaks
-- and similar patterns that the project-level snapshots cannot
-- show.
--
-- ## Why a separate table from `metrics_snapshots`?
--
-- Privacy boundaries differ:
--
-- - `metrics_snapshots` is project-aggregated. Anyone who can see
--   the project page can see those numbers. They are explicitly
--   non-personal — no row identifies a specific user.
--
-- - `user_metrics_snapshots` is per-user. Only the user themselves
--   (and, eventually, manager / neutral-third-party roles arriving
--   with the planned Team feature) can see this data.
--
-- Storing them in the same table would mean either (a) the
-- aggregation table grows per-user rows that bleed past their
-- access boundary, or (b) the privacy logic lives at the row
-- level rather than at the table level. Separate tables keep the
-- access control story tractable: per-V2.1 brief §2.5 ("集計と個別
-- を混同しない"), the boundary is at the schema level.
--
-- ## Schema rationale
--
-- - `user_id` references `users(id)` with `ON DELETE CASCADE`.
--   When a user is deleted, their personal history goes with them.
--   This is different from `issue_events` (which keeps history
--   when an issue is deleted) because event log is operational
--   audit, while user metrics are personal data — the user's
--   right to deletion takes precedence over operational history.
--
-- - The four metric columns mirror the parts of
--   `peisear_core::personal_metrics::PersonalMetrics` that vary
--   over time and matter for streak detection. Static facts
--   (display_name, effective_wip_limit) are not snapshotted —
--   they're cheap to look up at read time and don't tell us
--   anything about *how the day went*.
--
-- - `over_capacity` is a boolean denormalisation of
--   "in_flight_points > capacity_points (when set)". It's
--   computed at write time and stored to make streak detection a
--   simple `COUNT(*) WHERE over_capacity = 1 ORDER BY captured_at`
--   query, rather than re-deriving it from raw inputs every read.
--
-- ## Lifecycle
--
-- The same `peisear-web::jobs::snapshot_loop` that writes
-- `metrics_snapshots` rows also writes `user_metrics_snapshots`,
-- one row per user with at least one in-flight assigned issue per
-- tick. Idle users (no in-flight work) get nothing — they have no
-- streak to track.

CREATE TABLE user_metrics_snapshots (
    id            TEXT PRIMARY KEY,
    user_id       TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Personal-metrics fields that vary over time and feed
    -- streak / trend detection. Names mirror the
    -- `PersonalMetrics` struct; only the time-varying ones live
    -- here.
    current_wip            INTEGER NOT NULL,
    in_flight_points       INTEGER NOT NULL,
    -- Snapshot of the user's capacity at write time. NULL when
    -- the user has not set a capacity. Stored alongside the
    -- in-flight number so a future capacity change does not
    -- retroactively change "was I over capacity that day?".
    capacity_points        INTEGER,
    -- Boolean flag for fast streak detection. Computed at write
    -- time as: capacity_points IS NOT NULL AND
    --          in_flight_points > capacity_points.
    over_capacity          INTEGER NOT NULL CHECK (over_capacity IN (0, 1)),

    -- WIP exceedance is the cognitive-load equivalent of capacity
    -- exceedance. Same denormalisation pattern.
    effective_wip_limit    INTEGER NOT NULL,
    over_wip_limit         INTEGER NOT NULL CHECK (over_wip_limit IN (0, 1)),

    captured_at            DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Streak queries: "for this user, count consecutive days where
-- over_capacity = 1, ordered by captured_at". The composite index
-- supports the time-range filter and the user filter both.
CREATE INDEX idx_user_metrics_snapshots_user_time
    ON user_metrics_snapshots(user_id, captured_at);
