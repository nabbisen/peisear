-- 0018_remove_issue_position.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0018_remove_issue_position.sql. No
-- porting notes -- `ALTER TABLE ... DROP COLUMN` is standard SQL,
-- supported identically; PostgreSQL has had it since long before
-- this project's own SQLite-version floor (3.35) needed it.

ALTER TABLE issues DROP COLUMN position;
