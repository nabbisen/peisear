-- 0008_user_metrics_snapshots.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0008_user_metrics_snapshots.sql.
-- Porting note: `DATETIME` -> `TIMESTAMPTZ`, same reason as `0001`.
-- The two boolean-shaped flags stay `INTEGER CHECK (x IN (0, 1))`
-- rather than becoming native `BOOLEAN` -- the query layer this
-- schema serves is not ported until `PG-004`, and that round
-- decides whether the representation itself changes, not this one.

CREATE TABLE user_metrics_snapshots (
    id            TEXT PRIMARY KEY,
    user_id       TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    current_wip            INTEGER NOT NULL,
    in_flight_points       INTEGER NOT NULL,
    capacity_points        INTEGER,
    over_capacity          INTEGER NOT NULL CHECK (over_capacity IN (0, 1)),

    effective_wip_limit    INTEGER NOT NULL,
    over_wip_limit         INTEGER NOT NULL CHECK (over_wip_limit IN (0, 1)),

    captured_at            TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_metrics_snapshots_user_time
    ON user_metrics_snapshots(user_id, captured_at);
