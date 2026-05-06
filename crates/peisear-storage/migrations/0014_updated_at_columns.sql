-- 0014_updated_at_columns.sql
--
-- Adds `updated_at` columns and bump triggers to entities that
-- will participate in the optimistic-lock contract (Phase A
-- Step 5 schema preparation, peisear-feature-spec-v2.1 §21.4).
--
-- The contract requires every mutation endpoint to compare a
-- client-supplied `client_updated_at` against the row's
-- canonical `updated_at` and reject (409) on mismatch. That
-- check needs an `updated_at` column on every entity. v0.16.0
-- had it on `projects` and `issues` (since 0001) but not on:
--
-- - sprints
-- - teams
-- - team_memberships  (role changes need it)
-- - user_capacities   (period rows are mutable)
--
-- ## Scope of this migration
--
-- 0.17.0 (Phase A) ships:
--
-- 1. The columns and triggers below (this file).
-- 2. Application-layer optimistic-lock checks for `issues` and
--    `projects` mutations (the entities whose `updated_at`
--    column already existed).
--
-- 0.18.0+ (Phase B) ships:
--
-- 3. Rust-struct field additions for Sprint, Team,
--    TeamMembership, CapacityRow to surface `updated_at` to
--    the application layer.
-- 4. Storage SELECT widenings to actually fetch the column.
-- 5. Handler-level lock checks for sprint / team / membership /
--    capacity mutations.
--
-- The split is pragmatic: shipping the schema in 0.17.0 means
-- live data accumulates correct `updated_at` values from day
-- one (via the triggers below), so when Phase B turns on the
-- handler-level checks, even rows untouched in Phase A have a
-- meaningful "last changed" value to compare against.
--
-- Triggers (rather than relying on every storage function to
-- remember to `SET updated_at = CURRENT_TIMESTAMP`) are the
-- safer pattern: we can't accidentally forget to bump.
--
-- ## Why initial value = COALESCE(latest event, created_at)
--
-- For rows that already exist when this migration runs,
-- `updated_at` is set to whatever the latest meaningful event
-- timestamp on the row is. Reasoning: the row's "last meaningful
-- change" is the latest of the timestamps the row already
-- carries. Using CURRENT_TIMESTAMP for backfill would invent a
-- fictional update event and look weird in any future "what
-- changed when" surface.
--
-- The sprint table has both `created_at` and `started_at` /
-- `completed_at`; we COALESCE to the latest non-NULL of these.
-- Other tables only have `created_at` (or `joined_at` for
-- memberships), so the backfill is straightforward.
--
-- ## Why ALTER TABLE ... ADD COLUMN, not table-rebuild
--
-- SQLite supports adding NOT NULL columns with a constant
-- DEFAULT in one shot. We use `CURRENT_TIMESTAMP` as the
-- DEFAULT so newly-inserted rows get a sensible value
-- automatically; the immediate UPDATE below overwrites the
-- backfill on existing rows with the more-meaningful event
-- timestamp.

-- ──────────────────────────────────────────────────────────────
-- sprints.updated_at
-- ──────────────────────────────────────────────────────────────

ALTER TABLE sprints
    ADD COLUMN updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP;

-- Backfill: for sprints, the most-recent meaningful event is
-- whichever of created_at / started_at / completed_at is
-- latest. COALESCE picks the latest non-NULL value.
UPDATE sprints SET updated_at = COALESCE(
    completed_at,
    started_at,
    created_at
);

-- Trigger: on any UPDATE, bump updated_at to now. The
-- `WHEN OLD.updated_at = NEW.updated_at` guard prevents the
-- trigger from firing recursively when the application layer
-- explicitly sets updated_at as part of its own UPDATE
-- (which we never do, but the guard documents the intent).
CREATE TRIGGER sprints_updated_at
    AFTER UPDATE ON sprints
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE sprints
       SET updated_at = CURRENT_TIMESTAMP
     WHERE id = NEW.id;
END;

-- ──────────────────────────────────────────────────────────────
-- teams.updated_at
-- ──────────────────────────────────────────────────────────────

ALTER TABLE teams
    ADD COLUMN updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP;

UPDATE teams SET updated_at = created_at;

CREATE TRIGGER teams_updated_at
    AFTER UPDATE ON teams
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE teams
       SET updated_at = CURRENT_TIMESTAMP
     WHERE id = NEW.id;
END;

-- ──────────────────────────────────────────────────────────────
-- team_memberships.updated_at
-- ──────────────────────────────────────────────────────────────
--
-- Role changes are the mutating action here. The membership
-- row's primary key is (team_id, user_id) so the trigger keys
-- on both.

ALTER TABLE team_memberships
    ADD COLUMN updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP;

UPDATE team_memberships SET updated_at = joined_at;

CREATE TRIGGER team_memberships_updated_at
    AFTER UPDATE ON team_memberships
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE team_memberships
       SET updated_at = CURRENT_TIMESTAMP
     WHERE team_id = NEW.team_id AND user_id = NEW.user_id;
END;

-- ──────────────────────────────────────────────────────────────
-- user_capacities.updated_at
-- ──────────────────────────────────────────────────────────────

ALTER TABLE user_capacities
    ADD COLUMN updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP;

UPDATE user_capacities SET updated_at = created_at;

CREATE TRIGGER user_capacities_updated_at
    AFTER UPDATE ON user_capacities
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE user_capacities
       SET updated_at = CURRENT_TIMESTAMP
     WHERE id = NEW.id;
END;
