-- 0007_metrics_snapshots.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0007_metrics_snapshots.sql. Porting
-- note: `DATETIME` -> `TIMESTAMPTZ`, same reason as `0001`.
--
-- `INTEGER` -> `BIGINT` throughout (`CON-006`, amended in place):
-- see `0001`'s own porting note for the general reason.

CREATE TABLE metrics_snapshots (
    id           TEXT PRIMARY KEY,
    project_id   TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    total_issues                  BIGINT NOT NULL,
    done_issues                   BIGINT NOT NULL,
    oldest_in_flight_age_days     BIGINT,
    recent_activity_count         BIGINT NOT NULL,
    in_flight_issues              BIGINT NOT NULL,
    top_assignee_in_flight_issues BIGINT NOT NULL,
    long_stale_in_flight_issues   BIGINT NOT NULL,
    wip_violators                 BIGINT NOT NULL,
    active_assignees              BIGINT NOT NULL,

    score_value  BIGINT NOT NULL CHECK (score_value BETWEEN 0 AND 100),

    captured_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_metrics_snapshots_project_time
    ON metrics_snapshots(project_id, captured_at);
