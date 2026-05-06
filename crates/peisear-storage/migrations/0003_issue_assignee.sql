-- 0003_issue_assignee.sql
-- Add a per-issue assignee.
--
-- Nullable on purpose: unassigned issues are a normal, common state
-- (a triage backlog where ownership has not yet been decided). Forcing
-- an assignee at creation time would be presumptive.
--
-- ON DELETE SET NULL on the FK: if a user is removed from the system,
-- their issues should remain visible and re-assignable, not vanish.
-- Cascading would silently delete work that the team probably still
-- cares about. Defaulting to NULL means "ownership returns to the
-- pool" — operators can then re-assign at leisure.
--
-- The candidate set for the assignee selector is, in this single-tenant
-- model, the project owner only. When the team / organisation feature
-- lands (see ROADMAP medium-term), the candidate set will expand to
-- include team members; the schema stays the same.

ALTER TABLE issues
    ADD COLUMN assignee_id TEXT NULL
        REFERENCES users (id) ON DELETE SET NULL;

CREATE INDEX idx_issues_assignee ON issues (assignee_id);
