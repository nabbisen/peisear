-- 0023_rowid_surrogate_for_ordering.sql (PostgreSQL only, PG-004)
--
-- Not a port. SQLite gives every table an implicit `rowid`, and nine
-- production queries' own `ORDER BY ..., rowid {ASC,DESC}` lean on it
-- as a tiebreak where the leading column ties -- `issues.rs`'s own
-- doc comment names the exact failure this round found it to exist
-- against (`created_at` is a one-second `CURRENT_TIMESTAMP`; SQLite
-- ties return in scan order, so a `rowid` term keeps a burst of
-- same-second creations from reading backwards). PostgreSQL has no
-- implicit row identifier of its own that is stable across an
-- `UPDATE` or a `VACUUM` (`ctid` is neither), so there is nothing to
-- cast or rename here -- only something to add.
--
-- `GENERATED ALWAYS AS IDENTITY` on each of the nine tables these
-- queries touch, named `rowid` to keep every `ORDER BY` clause in
-- `src/` byte-identical on both backends -- not a mechanical
-- `?` -> `$` rewrite, so this is the one column this round adds
-- rather than translates. `GENERATED ALWAYS` matches the role
-- exactly: nothing in `src/` ever names `rowid` in an `INSERT`
-- column list, on either backend.
--
-- Stronger than the thing it stands in for, not merely equivalent:
-- `issues.rs`'s own doc comment admits SQLite reuses the rowid of a
-- deleted highest row, so a delete immediately followed by a create
-- in the same table can invert that pair. PostgreSQL's identity
-- sequence never reuses a value after a delete (or a rolled-back
-- transaction), so this surrogate carries no equivalent caveat --
-- documented here because the SQLite-side doc comment states one
-- that no longer applies, not because this file needed to prove it.
--
-- `migration_drift.rs`'s own guard would otherwise flag this file as
-- unmatched; it carries a named, reasoned exception for exactly this
-- filename rather than a placeholder on the SQLite side -- there is
-- nothing for `migrations/0023` to port, since SQLite already has
-- what this file adds for free.

ALTER TABLE issues ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
ALTER TABLE projects ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
ALTER TABLE issue_events ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
ALTER TABLE metrics_snapshots ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
ALTER TABLE user_metrics_snapshots ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
ALTER TABLE user_capacities ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
ALTER TABLE notifications ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
ALTER TABLE sprints ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
ALTER TABLE sprint_issues ADD COLUMN rowid BIGINT GENERATED ALWAYS AS IDENTITY;
