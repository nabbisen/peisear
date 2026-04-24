-- 0001_initial.sql
-- Enforce foreign keys for data integrity.
-- Note: SQLite requires PRAGMA foreign_keys = ON per connection (set in pool.rs).

CREATE TABLE users (
    id           TEXT PRIMARY KEY NOT NULL,
    email        TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    display_name TEXT NOT NULL,
    created_at   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users (email);

CREATE TABLE projects (
    id          TEXT PRIMARY KEY NOT NULL,
    owner_id    TEXT NOT NULL,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX idx_projects_owner ON projects (owner_id);

-- status: 'open' | 'in_progress' | 'done'
-- priority: 'low' | 'medium' | 'high' | 'urgent'
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
    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    FOREIGN KEY (author_id)  REFERENCES users (id)    ON DELETE CASCADE
);

CREATE INDEX idx_issues_project ON issues (project_id);
CREATE INDEX idx_issues_status  ON issues (project_id, status);
