-- 0011_teams.sql
-- Phase 1 of the team / organisation model (0.14.0).
--
-- ## Design summary
--
-- Linear-style flat teams: optional, one level deep. Users can
-- belong to multiple teams; projects can either belong to a
-- team or remain personal (`team_id IS NULL`). Admins are a
-- political role (manage membership, edit team settings); they
-- are *not* a surveillance role — V2.1 §2.5 keeps individual
-- signals (burnout, personal dashboard) private to the user.
--
-- ## What ships in 0.14.0
--
-- - `teams` table with name, slug, optional description.
-- - `team_memberships` join table with role.
-- - `projects.team_id` (nullable FK to teams).
-- - Existing personal projects continue to function unchanged
--   (their `team_id` stays NULL).
--
-- ## What does *not* ship
--
-- - Sub-teams (parent_team_id): deferred to Phase 2.
-- - Per-team workflow / cycle / label settings.
-- - Custom roles. The three values are fixed strings checked at
--   the schema level. Adding more values later means changing
--   the CHECK constraint (a non-breaking migration); adding a
--   `team_roles` table for custom-named roles is a Phase 2
--   schema change.
-- - Per-team privacy controls beyond the V2.1 floor. See ROADMAP
--   "Privacy & access control evolution" for what's parked.

CREATE TABLE teams (
    id          TEXT PRIMARY KEY,

    -- Display name. Renamable (no slug coupling).
    name        TEXT NOT NULL,

    -- URL slug. Lowercased, hyphenated, unique. The application
    -- generates this from `name` on create but treats them as
    -- independent thereafter (renaming `name` does not change
    -- the slug — same posture as GitHub orgs).
    slug        TEXT NOT NULL UNIQUE
                CHECK (length(slug) BETWEEN 1 AND 64
                   AND slug = lower(slug)),

    -- Free-form description shown on the team page. Optional.
    description TEXT,

    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Slug lookup is the hot path (URL → team). Already covered by
-- the UNIQUE constraint, but index name made explicit for
-- readability when reviewing schema dumps.
-- (SQLite auto-creates an index for UNIQUE; this is documentation.)


CREATE TABLE team_memberships (
    team_id     TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Role string. Stored as TEXT (not an enum) so future role
    -- additions don't require a migration; the CHECK constraint
    -- is the only place the vocabulary is enforced.
    --
    -- - admin: manage team members and settings; can move
    --   team-owned projects in/out; assign or change roles.
    -- - member: full project participation (create, edit issues;
    --   be assigned).
    -- - viewer: read-only on team projects (no issue create,
    --   no assignment, no edit).
    role        TEXT NOT NULL CHECK (role IN ('admin', 'member', 'viewer')),

    joined_at   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (team_id, user_id)
);

-- "What teams am I in?" — primary lookup for the user-facing
-- nav and dashboard. Composite primary key already covers
-- (team_id, user_id) lookups; the reverse direction needs its
-- own index.
CREATE INDEX idx_team_memberships_user ON team_memberships(user_id);


-- Add team_id to projects. Nullable; existing projects stay
-- personal (NULL). Indexed because team-scoped queries
-- ("projects in this team") are the common reverse lookup.
ALTER TABLE projects ADD COLUMN team_id TEXT REFERENCES teams(id) ON DELETE SET NULL;
CREATE INDEX idx_projects_team ON projects(team_id);
