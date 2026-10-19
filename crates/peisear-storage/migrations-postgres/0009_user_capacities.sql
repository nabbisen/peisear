-- 0009_user_capacities.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0009_user_capacities.sql.
--
-- Porting notes:
-- - `DATE`/`DATETIME` -> `DATE`/`TIMESTAMPTZ`, same reasons as
--   `0001`.
-- - **UUID generation**: SQLite's `lower(hex(randomblob(16)))`
--   builds a 32-character lowercase hex string with no application
--   involvement, in pure SQL. PostgreSQL's equivalent built-in (no
--   extension required, core since PostgreSQL 13) is
--   `gen_random_uuid()`, which returns a *formatted* UUID
--   (`8-4-4-4-12` hyphenated); `replace(..., '-', '')` strips the
--   hyphens to match the same bare-hex-string shape the application
--   layer already expects from the SQLite column (`id TEXT`, not a
--   native `uuid` type -- the column type is unchanged, so the
--   value shape must match what callers already parse).
-- - On a database migrating from empty (this round's own gate),
--   `users` has no rows with `capacity_points IS NOT NULL`, so this
--   `INSERT ... SELECT` and the following `ALTER TABLE ... DROP
--   COLUMN` both run as before: a no-op `INSERT` (zero rows
--   selected) and a real `DROP COLUMN` on a column with no data to
--   lose either way.

CREATE TABLE user_capacities (
    id            TEXT PRIMARY KEY,
    user_id       TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    points        INTEGER NOT NULL CHECK (points >= 0),
    period_start  DATE,
    period_end    DATE,
    note          TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CHECK (period_start IS NULL OR period_end IS NULL OR period_start <= period_end)
);

CREATE INDEX idx_user_capacities_user_period
    ON user_capacities(user_id, period_start, period_end);

INSERT INTO user_capacities (id, user_id, points, period_start, period_end, note)
SELECT
    replace(gen_random_uuid()::text, '-', ''),
    id,
    capacity_points,
    NULL,
    NULL,
    'migrated from 0.11.0 users.capacity_points'
FROM users
WHERE capacity_points IS NOT NULL;

ALTER TABLE users DROP COLUMN capacity_points;
