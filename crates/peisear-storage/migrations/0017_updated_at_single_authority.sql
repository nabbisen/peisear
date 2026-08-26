-- 0017_updated_at_single_authority.sql
--
-- `NFR-CONC-003`: the application MUST NOT write `updated_at`;
-- database triggers MUST maintain it, so the lock value has
-- exactly one authority. `QA-018`'s audit found two authorities on
-- three tables: `issues`, `projects`, and `user_view_states` all
-- had application code writing `updated_at` directly, with no
-- trigger backing them the way `0014_updated_at_columns.sql` backs
-- `sprints`/`teams`/`team_memberships`/`user_capacities`.
--
-- `issues` and `projects` already carried `updated_at` from 0001,
-- before the trigger convention existed (0014); `0014` only
-- retrofitted the four tables that were gaining the column for the
-- first time. This migration closes that gap for the three
-- remaining tables (`QA-019`).
--
-- Trigger shape copied verbatim from `0014`, including the `WHEN`
-- clause: the trigger fires only when the `UPDATE` did not itself
-- change `updated_at`, so it is inert for one release while the
-- application still sets the column explicitly (this migration
-- ships before the application-layer clauses are removed, in a
-- separate commit) — no double-bump, no window where nothing
-- maintains the column.
--
-- `user_view_states` is not lock-participating (`QA-019` §4): its
-- `upsert` takes no client-supplied lock value and is called from a
-- `GET` handler as a fire-and-forget side effect of visiting a
-- filtered URL. It gets a trigger for uniformity with the
-- requirement's blanket wording, not because a stale value here has
-- any safety consequence.

-- ──────────────────────────────────────────────────────────────
-- issues.updated_at
-- ──────────────────────────────────────────────────────────────

CREATE TRIGGER issues_updated_at
    AFTER UPDATE ON issues
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE issues
       SET updated_at = CURRENT_TIMESTAMP
     WHERE id = NEW.id;
END;

-- ──────────────────────────────────────────────────────────────
-- projects.updated_at
-- ──────────────────────────────────────────────────────────────

CREATE TRIGGER projects_updated_at
    AFTER UPDATE ON projects
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE projects
       SET updated_at = CURRENT_TIMESTAMP
     WHERE id = NEW.id;
END;

-- ──────────────────────────────────────────────────────────────
-- user_view_states.updated_at
-- ──────────────────────────────────────────────────────────────
--
-- Primary key is (user_id, view_key), not a single `id` column, so
-- the trigger keys on both — same shape `0014` used for
-- `team_memberships`.

CREATE TRIGGER user_view_states_updated_at
    AFTER UPDATE ON user_view_states
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE user_view_states
       SET updated_at = CURRENT_TIMESTAMP
     WHERE user_id = NEW.user_id AND view_key = NEW.view_key;
END;
