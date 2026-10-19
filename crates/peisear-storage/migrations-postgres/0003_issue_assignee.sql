-- 0003_issue_assignee.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0003_issue_assignee.sql. No porting
-- notes -- already standard SQL.

ALTER TABLE issues
    ADD COLUMN assignee_id TEXT NULL
        REFERENCES users (id) ON DELETE SET NULL;

CREATE INDEX idx_issues_assignee ON issues (assignee_id);
