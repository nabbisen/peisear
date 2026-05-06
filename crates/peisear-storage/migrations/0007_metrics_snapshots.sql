-- 0007_metrics_snapshots.sql
-- Periodic snapshots of project-level health metrics, written by a
-- background tokio task. The point is to give project pages a
-- "compared to last week" trend indicator without re-deriving past
-- state from raw events on every render.
--
-- ## Why these specific columns
--
-- The nine metric columns mirror `peisear_core::project_health::
-- ProjectHealthRaw` exactly. Storing the raw inputs (rather than
-- only the composite score) lets future indicator additions
-- recompute their own historical trend without a backfill — the
-- struct is what we already serialise out of `for_project()`.
--
-- `score_value` is the composite 0–100 we computed at the time of
-- the snapshot. It's denormalised on purpose: when `HealthWeights`
-- changes in some future release, today's snapshot still tells the
-- story of "the score we showed people back then". Trend
-- comparisons should reflect the user's lived experience, not a
-- retroactive re-scoring.
--
-- ## What we deliberately don't store
--
-- - No per-user data. This table is project-level aggregation and
--   that's the privacy boundary. Per-V2.1 brief §0.2 ("常時監視を
--   目的としない") and §2.5 ("集計と個別を混同しない"), per-user
--   trend lives in a future user-burnout module with explicit
--   scoping rules, not here.
--
-- - No identification of WIP-violating users. `wip_violators` is a
--   count, not a list — same reasoning.
--
-- ## Lifecycle
--
-- Background task in `peisear` runs every 6 hours and inserts one
-- row per project with at least one issue. Idle projects (no
-- issues) get nothing — there's no signal to capture. The
-- `ON DELETE CASCADE` ensures snapshots vanish if the project is
-- deleted, since standalone project snapshots have no meaning.

CREATE TABLE metrics_snapshots (
    id           TEXT PRIMARY KEY,
    project_id   TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    -- ProjectHealthRaw fields, in the same order as the struct.
    total_issues                  INTEGER NOT NULL,
    done_issues                   INTEGER NOT NULL,
    oldest_in_flight_age_days     INTEGER,
    recent_activity_count         INTEGER NOT NULL,
    in_flight_issues              INTEGER NOT NULL,
    top_assignee_in_flight_issues INTEGER NOT NULL,
    long_stale_in_flight_issues   INTEGER NOT NULL,
    wip_violators                 INTEGER NOT NULL,
    active_assignees              INTEGER NOT NULL,

    -- Denormalised composite, captured at write time.
    score_value  INTEGER NOT NULL CHECK (score_value BETWEEN 0 AND 100),

    captured_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Trend queries fetch the project's recent snapshots ordered by
-- captured_at; the index supports both "all in window" and "most
-- recent N" patterns.
CREATE INDEX idx_metrics_snapshots_project_time
    ON metrics_snapshots(project_id, captured_at);
