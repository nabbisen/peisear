-- 0002_issue_effort.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0002_issue_effort.sql. No porting notes
-- -- this statement is already standard SQL.

ALTER TABLE issues
    ADD COLUMN effort INTEGER NULL CHECK (effort IS NULL OR effort > 0);
