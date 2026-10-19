-- 0006_issue_events.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0006_issue_events.sql. Porting note:
-- `DATETIME` -> `TIMESTAMPTZ`, same reason as `0001`.

CREATE TABLE issue_events (
    id             TEXT PRIMARY KEY,
    issue_id       TEXT REFERENCES issues(id) ON DELETE SET NULL,
    project_id     TEXT NOT NULL,
    actor_id       TEXT REFERENCES users(id) ON DELETE SET NULL,
    event_type     TEXT NOT NULL CHECK (event_type IN (
        'created',
        'status_changed',
        'assignee_changed',
        'effort_changed',
        'deleted'
    )),
    previous_value TEXT,
    new_value      TEXT,
    occurred_at    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_issue_events_issue ON issue_events(issue_id, occurred_at);
CREATE INDEX idx_issue_events_project ON issue_events(project_id, occurred_at);
CREATE INDEX idx_issue_events_actor ON issue_events(actor_id, occurred_at);
