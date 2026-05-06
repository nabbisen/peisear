-- 0015_sub_issues.sql
--
-- Adds the `parent_issue_id` column to the `issues` table to
-- introduce the sub-issue hierarchy (peisear-feature-spec-v2.1
-- §8.3 / §8.4, Phase C PR1).
--
-- ## Design recap (spec §8.3)
--
-- Sub-issues use the **parent_id approach** (Linear-style):
-- one `issues` table, an extra nullable column on each row
-- pointing at its parent. Promotion ("this is actually its own
-- issue") and demotion ("nest this under another one") are
-- single-column updates rather than table moves.
--
-- The constraints from §8.3:
--
-- - **One level only** — sub-issues cannot have sub-issues.
--   Enforced by trigger below; SQLite has no recursive CHECK
--   that can dereference a foreign key, so a BEFORE INSERT/
--   UPDATE trigger reads the parent row and aborts if it
--   already has a parent.
-- - **Same project** — parent and child must share `project_id`.
--   Enforced in the trigger as well.
-- - **assignee independent** — sub-issue's assignee can
--   differ from parent's. No constraint needed; existing
--   columns work as is.
-- - **status independent** — explicit by §8.6: "全 sub-issue
--   が done でも parent は自動的に done にならない." No
--   constraint needed; future "suggest completion" UX (§8.6
--   future-extension) lives in the application layer if at
--   all.
--
-- ## Indexing strategy
--
-- Two complementary indices:
--
-- 1. `idx_issues_parent` for "give me the children of issue X"
--    queries (rendered on issue detail). Partial index on the
--    non-NULL rows keeps it small — the median issue has zero
--    children, so the full table version would be 95% wasted
--    space.
-- 2. `idx_issues_top_level` for "give me the top-level issues
--    of project P with status S" queries (rendered on the issue
--    list, kanban, and project board). Partial index on
--    `parent_issue_id IS NULL` rows lets the planner skip
--    sub-issue rows entirely without a separate WHERE-clause
--    cost.
--
-- ## ON DELETE CASCADE
--
-- If the parent issue is deleted, its sub-issues are deleted
-- with it. This matches user expectations ("delete the parent,
-- everything under it goes too") and avoids dangling
-- references. If a user wants to keep the children but remove
-- the parent, they should promote the children first
-- (`parent_issue_id = NULL`) and then delete the parent.
--
-- ## Note on existing data
--
-- The column is nullable, so existing rows acquire
-- `parent_issue_id = NULL` automatically. They are top-level
-- issues by definition. No data backfill needed.

ALTER TABLE issues
    ADD COLUMN parent_issue_id TEXT
        REFERENCES issues(id) ON DELETE CASCADE;

CREATE INDEX idx_issues_parent
    ON issues(parent_issue_id)
    WHERE parent_issue_id IS NOT NULL;

CREATE INDEX idx_issues_top_level
    ON issues(project_id, status)
    WHERE parent_issue_id IS NULL;

-- Trigger 1: prevent two-level nesting (sub-issue of a sub-
-- issue) and enforce same-project on INSERT.
CREATE TRIGGER prevent_sub_issue_nesting_insert
BEFORE INSERT ON issues
FOR EACH ROW
WHEN NEW.parent_issue_id IS NOT NULL
BEGIN
    SELECT RAISE(ABORT, 'sub-issue cannot have a sub-issue (1-level only)')
    WHERE EXISTS (
        SELECT 1 FROM issues
        WHERE id = NEW.parent_issue_id
          AND parent_issue_id IS NOT NULL
    );
    SELECT RAISE(ABORT, 'sub-issue must share project with its parent')
    WHERE NOT EXISTS (
        SELECT 1 FROM issues
        WHERE id = NEW.parent_issue_id
          AND project_id = NEW.project_id
    );
END;

-- Trigger 2: same checks on UPDATE. This handles "promote/
-- demote" workflows where parent_issue_id changes after the
-- row exists.
CREATE TRIGGER prevent_sub_issue_nesting_update
BEFORE UPDATE OF parent_issue_id ON issues
FOR EACH ROW
WHEN NEW.parent_issue_id IS NOT NULL
BEGIN
    SELECT RAISE(ABORT, 'sub-issue cannot have a sub-issue (1-level only)')
    WHERE EXISTS (
        SELECT 1 FROM issues
        WHERE id = NEW.parent_issue_id
          AND parent_issue_id IS NOT NULL
    );
    -- An issue cannot be its own parent, even via a chain.
    -- Since we restrict to 1 level, the only loop possible is
    -- self-reference; check for it.
    SELECT RAISE(ABORT, 'an issue cannot be its own parent')
    WHERE NEW.parent_issue_id = NEW.id;
    SELECT RAISE(ABORT, 'sub-issue must share project with its parent')
    WHERE NOT EXISTS (
        SELECT 1 FROM issues
        WHERE id = NEW.parent_issue_id
          AND project_id = NEW.project_id
    );
    -- Cannot demote an issue that already has children — that
    -- would create a 2-level chain. Promote the children first.
    SELECT RAISE(ABORT, 'cannot demote an issue that has its own sub-issues; promote them first')
    WHERE EXISTS (
        SELECT 1 FROM issues
        WHERE parent_issue_id = NEW.id
    );
END;
