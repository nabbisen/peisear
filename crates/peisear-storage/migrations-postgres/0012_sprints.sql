-- 0012_sprints.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0012_sprints.sql. Porting note:
-- `DATETIME` -> `TIMESTAMPTZ`, `DATE` unchanged, same reasons as
-- `0001`. Table-level `CHECK` after all columns is standard SQL in
-- both dialects.

CREATE TABLE sprints (
    id          TEXT PRIMARY KEY,
    team_id     TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name        TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 120),
    goal        TEXT,
    starts_on   DATE NOT NULL,
    ends_on     DATE NOT NULL,
    status      TEXT NOT NULL DEFAULT 'planned'
                CHECK (status IN ('planned', 'active', 'completed')),
    started_at  TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CHECK (starts_on <= ends_on)
);

CREATE INDEX idx_sprints_team_status ON sprints(team_id, status, starts_on DESC);
CREATE INDEX idx_sprints_team_dates ON sprints(team_id, starts_on, ends_on);

CREATE TABLE sprint_issues (
    issue_id    TEXT PRIMARY KEY REFERENCES issues(id) ON DELETE CASCADE,
    sprint_id   TEXT NOT NULL REFERENCES sprints(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_sprint_issues_sprint ON sprint_issues(sprint_id, assigned_at);
