-- 0007_metrics_snapshots.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0007_metrics_snapshots.sql. Porting
-- note: `DATETIME` -> `TIMESTAMPTZ`, same reason as `0001`.

CREATE TABLE metrics_snapshots (
    id           TEXT PRIMARY KEY,
    project_id   TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    total_issues                  INTEGER NOT NULL,
    done_issues                   INTEGER NOT NULL,
    oldest_in_flight_age_days     INTEGER,
    recent_activity_count         INTEGER NOT NULL,
    in_flight_issues              INTEGER NOT NULL,
    top_assignee_in_flight_issues INTEGER NOT NULL,
    long_stale_in_flight_issues   INTEGER NOT NULL,
    wip_violators                 INTEGER NOT NULL,
    active_assignees              INTEGER NOT NULL,

    score_value  INTEGER NOT NULL CHECK (score_value BETWEEN 0 AND 100),

    captured_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_metrics_snapshots_project_time
    ON metrics_snapshots(project_id, captured_at);
