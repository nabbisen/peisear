-- 0005_personal_limits.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0005_personal_limits.sql. No porting
-- notes -- already standard SQL.
--
-- `INTEGER` -> `BIGINT` for both columns (`CON-006`, amended in
-- place): not named in that round's own §3 file list, found by its
-- own predicate instead -- see `0001`'s porting note for the
-- general reason, and that round's own completion package for this
-- gap between what §3 named and what §2's own "every one" required.

ALTER TABLE users
    ADD COLUMN wip_limit BIGINT NULL
        CHECK (wip_limit IS NULL OR wip_limit > 0);

ALTER TABLE projects
    ADD COLUMN wip_limit_default BIGINT NULL
        CHECK (wip_limit_default IS NULL OR wip_limit_default > 0);
