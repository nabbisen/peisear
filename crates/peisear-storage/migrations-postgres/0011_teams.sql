-- 0011_teams.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0011_teams.sql. Porting note:
-- `DATETIME` -> `TIMESTAMPTZ`, same reason as `0001`. `length()`
-- and `lower()` are both standard functions PostgreSQL provides
-- identically.

CREATE TABLE teams (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    slug        TEXT NOT NULL UNIQUE
                CHECK (length(slug) BETWEEN 1 AND 64
                   AND slug = lower(slug)),
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE team_memberships (
    team_id     TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role        TEXT NOT NULL CHECK (role IN ('admin', 'member', 'viewer')),
    joined_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (team_id, user_id)
);

CREATE INDEX idx_team_memberships_user ON team_memberships(user_id);

ALTER TABLE projects ADD COLUMN team_id TEXT REFERENCES teams(id) ON DELETE SET NULL;
CREATE INDEX idx_projects_team ON projects(team_id);
