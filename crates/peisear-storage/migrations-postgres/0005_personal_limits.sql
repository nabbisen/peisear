-- 0005_personal_limits.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0005_personal_limits.sql. No porting
-- notes -- already standard SQL.

ALTER TABLE users
    ADD COLUMN wip_limit INTEGER NULL
        CHECK (wip_limit IS NULL OR wip_limit > 0);

ALTER TABLE projects
    ADD COLUMN wip_limit_default INTEGER NULL
        CHECK (wip_limit_default IS NULL OR wip_limit_default > 0);
