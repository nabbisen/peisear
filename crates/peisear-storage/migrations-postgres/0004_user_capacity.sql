-- 0004_user_capacity.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0004_user_capacity.sql. No porting
-- notes -- already standard SQL. (This column is dropped again in
-- `0009`, ported for the same version-parity reason noted in
-- `0001`.)

ALTER TABLE users
    ADD COLUMN capacity_points INTEGER NULL
        CHECK (capacity_points IS NULL OR capacity_points > 0);
