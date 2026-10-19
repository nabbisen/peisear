-- 0021_updated_at_millisecond_precision.sql (PostgreSQL port, PG-003)
--
-- **The version numbers do not divide the way SQLite's did, stated
-- rather than forced** (`PG-002` §6b's own closing instruction).
-- SQLite's `0014`/`0017` created seven `updated_at`-bump triggers at
-- whole-second precision, and this file (`0021`) later `DROP`s and
-- re-`CREATE`s all seven at millisecond precision -- a real defect
-- fix, because `CURRENT_TIMESTAMP` was the problem. PostgreSQL's
-- `TIMESTAMPTZ`/`clock_timestamp()` never had that problem
-- (`PG-002`'s own measurement: microsecond precision from the
-- column type alone) — **there is no whole-second defect for this
-- file to fix**, so all seven triggers are created directly, once,
-- here. `migrations-postgres/0014_…` and `0017_…` stay exactly what
-- `PG-002` left them: permanent placeholders, not pending anything
-- further. Nothing in this project ever edits a migration after
-- real installs can run it (`DIA-001` round 2's own lesson), so a
-- placeholder that stays empty forever is exactly as safe as one
-- that was never there.
--
-- **One function, not seven** — genuinely possible here in a way it
-- is not in the SQLite original: each SQLite trigger fires a
-- *second* `UPDATE` statement naming its own table
-- (`UPDATE sprints SET updated_at = … WHERE id = NEW.id`), so each
-- needs its own trigger body. This design instead mutates `NEW`
-- directly inside a `BEFORE UPDATE` trigger and returns it — no
-- second statement, no table name needed inside the function body,
-- so one function serves all seven tables. `team_memberships` and
-- `user_view_states` have no single `id` column (composite primary
-- keys); the SQLite trigger keyed its second `UPDATE`'s `WHERE`
-- clause on both columns for exactly that reason, and this design
-- does not need to, since it never issues a second statement at
-- all.
--
-- `clock_timestamp()`, not `CURRENT_TIMESTAMP`/`NOW()` — measured
-- distinction, not a style preference: `CURRENT_TIMESTAMP` returns
-- the *transaction's* start time, frozen for every call inside one
-- transaction, where `clock_timestamp()` returns the actual wall
-- clock at the moment it runs. Two writes to the same row inside one
-- transaction would share a `CURRENT_TIMESTAMP` value — reproducing
-- `NFR-CONC-001`'s own defect at the transaction level instead of
-- SQLite's whole-second level — and `clock_timestamp()` is what
-- `pool::tests::two_writes_in_one_postgres_transaction_still_advance_the_lock_token`
-- demonstrates against a real server, not assumed from documentation.
--
-- The `WHEN (OLD.updated_at = NEW.updated_at)` guard is carried over
-- unchanged from the SQLite original — it exists so an application
-- write that already set `updated_at` itself is not double-bumped
-- (`NFR-CONC-003`'s own history, `0014`/`0017`'s comments).

CREATE FUNCTION bump_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := clock_timestamp();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER sprints_updated_at
    BEFORE UPDATE ON sprints
    FOR EACH ROW
    WHEN (OLD.updated_at = NEW.updated_at)
    EXECUTE FUNCTION bump_updated_at();

CREATE TRIGGER teams_updated_at
    BEFORE UPDATE ON teams
    FOR EACH ROW
    WHEN (OLD.updated_at = NEW.updated_at)
    EXECUTE FUNCTION bump_updated_at();

CREATE TRIGGER team_memberships_updated_at
    BEFORE UPDATE ON team_memberships
    FOR EACH ROW
    WHEN (OLD.updated_at = NEW.updated_at)
    EXECUTE FUNCTION bump_updated_at();

CREATE TRIGGER user_capacities_updated_at
    BEFORE UPDATE ON user_capacities
    FOR EACH ROW
    WHEN (OLD.updated_at = NEW.updated_at)
    EXECUTE FUNCTION bump_updated_at();

CREATE TRIGGER issues_updated_at
    BEFORE UPDATE ON issues
    FOR EACH ROW
    WHEN (OLD.updated_at = NEW.updated_at)
    EXECUTE FUNCTION bump_updated_at();

CREATE TRIGGER projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    WHEN (OLD.updated_at = NEW.updated_at)
    EXECUTE FUNCTION bump_updated_at();

CREATE TRIGGER user_view_states_updated_at
    BEFORE UPDATE ON user_view_states
    FOR EACH ROW
    WHEN (OLD.updated_at = NEW.updated_at)
    EXECUTE FUNCTION bump_updated_at();
