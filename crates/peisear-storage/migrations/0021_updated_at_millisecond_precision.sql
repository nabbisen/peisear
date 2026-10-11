-- 0021_updated_at_millisecond_precision.sql
--
-- `STORE-001` (`RFC 0018` P0): `NFR-CONC-001` is P0 and has a window in
-- which it cannot do its job. SQLite's `CURRENT_TIMESTAMP` is
-- whole-second; `NFR-CONC-002` reuses `updated_at` as the optimistic-lock
-- token; so two writes to the same row inside one second leave the token
-- unchanged, and a client holding a stale token compares equal and is
-- admitted. Demonstrated through the handlers, not only against SQLite
-- directly, in `crates/peisear-web/tests/optimistic_lock.rs`'s
-- `issue_edited_twice_in_the_same_second_with_the_same_token_must_409` --
-- it fails before this migration and passes after.
--
-- ## The fix: millisecond precision, triggers only
--
-- `strftime('%Y-%m-%d %H:%M:%f', 'now')` in place of `CURRENT_TIMESTAMP`
-- in every `updated_at`-bump trigger's `SET` clause -- SQLite produces
-- `YYYY-MM-DD HH:MM:SS.SSS`, a fixed three fractional digits, which
-- `sqlx-sqlite`'s own `chrono` decode already parses first in its format
-- list (`"%F %T%.f"`) and preserves into the `DateTime<Utc>` the
-- application compares with. No row's shape changes; no application code
-- changes.
--
-- SQLite cannot `ALTER TRIGGER`, so each of the seven triggers below is
-- dropped and recreated whole. **Exactly seven, all of them timestamp
-- bumps** -- the other four triggers this schema carries are constraint
-- triggers (`RAISE(ABORT)` guards) and are untouched:
--
-- | From       | Triggers |
-- |------------|----------|
-- | `0014`     | `sprints_updated_at`, `teams_updated_at`, `team_memberships_updated_at`, `user_capacities_updated_at` |
-- | `0017`     | `issues_updated_at`, `projects_updated_at`, `user_view_states_updated_at` |
--
-- The `WHEN OLD.updated_at = NEW.updated_at` guard on every trigger is
-- unchanged from `0014`/`0017` -- it exists because the application once
-- wrote the column itself (`QA-018`, `QA-019`), a separate question with
-- its own history, not this migration's to revisit.
--
-- ## What this migration deliberately does not touch
--
-- `DEFAULT CURRENT_TIMESTAMP` on the `updated_at` columns themselves
-- (19 occurrences across ten migrations) is untouched. The default
-- applies at INSERT; the lock token is spent at UPDATE. A row inserted
-- at second precision whose first UPDATE fires one of the triggers above
-- moves to millisecond precision immediately -- which differs from
-- whatever a client read before that first update, so a stale write
-- against a never-yet-updated row is still correctly rejected on the
-- token's own terms (it no longer matches once the trigger has fired
-- once, which is unconditionally before any second write can land).
-- Changing the defaults would mean twelve table rebuilds (SQLite cannot
-- alter a column default without one) for a precision window that is
-- already closed by the first UPDATE either way.

-- ──────────────────────────────────────────────────────────────
-- 0014's four triggers
-- ──────────────────────────────────────────────────────────────

DROP TRIGGER sprints_updated_at;
CREATE TRIGGER sprints_updated_at
    AFTER UPDATE ON sprints
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE sprints
       SET updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now')
     WHERE id = NEW.id;
END;

DROP TRIGGER teams_updated_at;
CREATE TRIGGER teams_updated_at
    AFTER UPDATE ON teams
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE teams
       SET updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now')
     WHERE id = NEW.id;
END;

DROP TRIGGER team_memberships_updated_at;
CREATE TRIGGER team_memberships_updated_at
    AFTER UPDATE ON team_memberships
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE team_memberships
       SET updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now')
     WHERE team_id = NEW.team_id AND user_id = NEW.user_id;
END;

DROP TRIGGER user_capacities_updated_at;
CREATE TRIGGER user_capacities_updated_at
    AFTER UPDATE ON user_capacities
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE user_capacities
       SET updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now')
     WHERE id = NEW.id;
END;

-- ──────────────────────────────────────────────────────────────
-- 0017's three triggers
-- ──────────────────────────────────────────────────────────────

DROP TRIGGER issues_updated_at;
CREATE TRIGGER issues_updated_at
    AFTER UPDATE ON issues
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE issues
       SET updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now')
     WHERE id = NEW.id;
END;

DROP TRIGGER projects_updated_at;
CREATE TRIGGER projects_updated_at
    AFTER UPDATE ON projects
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE projects
       SET updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now')
     WHERE id = NEW.id;
END;

DROP TRIGGER user_view_states_updated_at;
CREATE TRIGGER user_view_states_updated_at
    AFTER UPDATE ON user_view_states
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE user_view_states
       SET updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now')
     WHERE user_id = NEW.user_id AND view_key = NEW.view_key;
END;
