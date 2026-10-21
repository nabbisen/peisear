-- 0002_issue_effort.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0002_issue_effort.sql. No porting notes
-- -- this statement is already standard SQL.
--
-- `INTEGER` -> `BIGINT` (`CON-006`, amended in place): this column
-- is selected by `find_by_id` and every issue list, so it is the
-- live instance of the class that round found -- see `0001`'s own
-- porting note for the general reason.

ALTER TABLE issues
    ADD COLUMN effort BIGINT NULL CHECK (effort IS NULL OR effort > 0);
