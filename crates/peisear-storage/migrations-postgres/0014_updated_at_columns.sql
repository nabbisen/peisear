-- 0014_updated_at_columns.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0014_updated_at_columns.sql.
--
-- **Triggers omitted, deliberately** — this series' own scope line:
-- "Not the triggers. PG-003." The four bump triggers the SQLite
-- original creates here (`sprints_updated_at`, `teams_updated_at`,
-- `team_memberships_updated_at`, `user_capacities_updated_at`) are
-- part of the 11 this schema carries in total, all of them `PG-003`'s
-- — this file ports the columns and the backfill around them and
-- says so, per the handoff's own instruction (`PG-002` §3), rather
-- than silently providing partial trigger logic.
--
-- **Until `PG-003` lands, nothing bumps these columns on
-- `UPDATE`** on the PostgreSQL side — an `UPDATE` changes other
-- columns but leaves `updated_at` as it was. This matches `pool::migrate`'s
-- own honesty about being partial: the schema exists, the lock
-- token's maintenance does not yet.
--
-- Porting note: `DATETIME` -> `TIMESTAMPTZ`, same reason as `0001`.
-- `COALESCE` is standard SQL, unchanged.

ALTER TABLE sprints
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP;

UPDATE sprints SET updated_at = COALESCE(
    completed_at,
    started_at,
    created_at
);

ALTER TABLE teams
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP;

UPDATE teams SET updated_at = created_at;

ALTER TABLE team_memberships
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP;

UPDATE team_memberships SET updated_at = joined_at;

ALTER TABLE user_capacities
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP;

UPDATE user_capacities SET updated_at = created_at;
