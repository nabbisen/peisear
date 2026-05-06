-- 0013_user_view_states.sql
--
-- Per-user, per-view UI state for list views (Phase A Step 3, v2.1
-- spec §4.4). The table holds the *server-side default* — the
-- initial filter/sort that a user sees when navigating fresh to a
-- list page that has no query parameters.
--
-- ## Why a key/value JSON store rather than typed columns
--
-- The set of remembered fields is going to grow as more list
-- screens land (Phase B's calendar settings, Phase C's sprint
-- backlog filter, etc.) and also as the issue-list filters
-- themselves grow richer (priority, due_date, sub-issue
-- visibility — see ROADMAP). A typed schema would force a
-- migration on every addition. A JSON blob keyed by view name
-- lets the application layer evolve freely; the only schema
-- contract is "for this user and this view, here is the
-- preferred default state".
--
-- The trade-off — losing SQL-level filtering/aggregation across
-- view states — does not bite us, because every read here is
-- "fetch the one row for (this user, this view)" and never a
-- cross-user query.
--
-- ## How the application layer uses this table
--
-- 1. The user navigates to a list URL (e.g.
--    `/projects/{id}?view=list&status=open&sort=priority`).
-- 2. The handler reads the query parameters. If present, they
--    win — those are the **active** filter/sort.
-- 3. After successfully rendering with explicit query params, the
--    handler upserts this row so the next *bare* URL (no query
--    params) restores them.
-- 4. If the URL has no query params, the handler reads this row
--    and applies its state as the default. Missing row → fall
--    back to a hard-coded factory default.
--
-- This is the URL-primary, server-default-secondary scheme
-- agreed for Phase A (filter/sort decision A-3 = C in the
-- session record).
--
-- ## Why `view_key` is a string, not an FK
--
-- View keys mix a stable namespace prefix (e.g.
-- `project_issues`) with a variable suffix (the project id).
-- The natural shape is a freeform string. We don't need
-- referential integrity here — if a project is deleted, the
-- orphan view-state row is harmless and gets pruned next time
-- that project id appears (or by a future cleanup job, ROADMAP).

CREATE TABLE user_view_states (
    user_id     TEXT NOT NULL,
    view_key    TEXT NOT NULL,
    -- JSON blob. Application layer parses with serde. The shape
    -- for `project_issues:{project_id}` is documented in
    -- `peisear-storage::view_states`.
    state_json  TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),

    PRIMARY KEY (user_id, view_key),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- The PRIMARY KEY already creates a unique index on
-- (user_id, view_key), which is the only access path we need.
-- No additional indexes.
