-- 0024_session_generation.sql (PostgreSQL port)
--
-- Byte-identical intent to `migrations/0024_session_generation.sql`
-- -- `RFC 024` §4.1's own point, demonstrated rather than merely
-- claimed: an integer column needs no dialect translation at all,
-- which is half the reason the mechanism is a counter and not a
-- stored instant (the other half of that same sentence is the
-- `DATETIME`-versus-`TIMESTAMPTZ` divergence this column never has
-- to resolve). See that file for the column's purpose and the
-- `migration_drift.rs` numbering correction (`0024`, not the
-- handoff's own `0023` -- already spent on this backend by
-- `0023_rowid_surrogate_for_ordering.sql`, which has no SQLite
-- counterpart).

ALTER TABLE users ADD COLUMN session_generation INTEGER NOT NULL DEFAULT 0;
