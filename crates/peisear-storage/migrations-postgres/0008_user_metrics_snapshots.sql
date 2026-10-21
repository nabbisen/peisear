-- 0008_user_metrics_snapshots.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0008_user_metrics_snapshots.sql.
-- Porting note: `DATETIME` -> `TIMESTAMPTZ`, same reason as `0001`.
-- The two boolean-shaped flags stay `CHECK (x IN (0, 1))` rather
-- than becoming native `BOOLEAN` -- the query layer this schema
-- serves is not ported until `PG-004`, and that round decides
-- whether the representation itself changes, not this one.
--
-- `INTEGER` -> `BIGINT` throughout (`CON-006`, amended in place),
-- including the two 0/1 flags' own column type -- their *shape*
-- (`CHECK (x IN (0, 1))`, not native `BOOLEAN`) is a separate
-- question from their *width*, and is raised, not decided, by that
-- round (its own §5). See `0001`'s porting note for the general
-- reason.

CREATE TABLE user_metrics_snapshots (
    id            TEXT PRIMARY KEY,
    user_id       TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    current_wip            BIGINT NOT NULL,
    in_flight_points       BIGINT NOT NULL,
    capacity_points        BIGINT,
    over_capacity          BIGINT NOT NULL CHECK (over_capacity IN (0, 1)),

    effective_wip_limit    BIGINT NOT NULL,
    over_wip_limit         BIGINT NOT NULL CHECK (over_wip_limit IN (0, 1)),

    captured_at            TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_metrics_snapshots_user_time
    ON user_metrics_snapshots(user_id, captured_at);
