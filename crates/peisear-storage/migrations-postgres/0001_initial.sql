-- 0001_initial.sql (PostgreSQL port, PG-002)
--
-- Schema and rationale: see migrations/0001_initial.sql. Porting
-- notes for this file only -- see dialect.rs's own module doc for
-- the general rules this whole directory follows.
--
-- `DATETIME` -> `TIMESTAMPTZ`: every one of these columns stores an
-- instant (`chrono::DateTime<Utc>` in the Rust layer); PostgreSQL's
-- own `CURRENT_TIMESTAMP` already returns `timestamptz`, so the
-- column type and the default agree with no translation needed.
--
-- PostgreSQL has no per-connection `PRAGMA foreign_keys` -- foreign
-- key enforcement is unconditional and always on, so the comment in
-- the SQLite original about enabling it in `pool.rs` has nothing to
-- port.

CREATE TABLE users (
    id           TEXT PRIMARY KEY NOT NULL,
    email        TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    display_name TEXT NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users (email);

CREATE TABLE projects (
    id          TEXT PRIMARY KEY NOT NULL,
    owner_id    TEXT NOT NULL,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX idx_projects_owner ON projects (owner_id);

-- status: 'open' | 'in_progress' | 'done'
-- priority: 'low' | 'medium' | 'high' | 'urgent'
--
-- `position` is ported here and dropped in `0018`, matching the
-- SQLite set's own history byte-for-byte in shape -- this directory
-- mirrors version numbers and intent, not just end states, which is
-- what the drift guard checks (`§2`).
CREATE TABLE issues (
    id          TEXT PRIMARY KEY NOT NULL,
    project_id  TEXT NOT NULL,
    author_id   TEXT NOT NULL,
    title       TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status      TEXT NOT NULL DEFAULT 'open'
                  CHECK (status IN ('open', 'in_progress', 'done')),
    priority    TEXT NOT NULL DEFAULT 'medium'
                  CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    position    INTEGER NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    FOREIGN KEY (author_id)  REFERENCES users (id)    ON DELETE CASCADE
);

CREATE INDEX idx_issues_project ON issues (project_id);
CREATE INDEX idx_issues_status  ON issues (project_id, status);
