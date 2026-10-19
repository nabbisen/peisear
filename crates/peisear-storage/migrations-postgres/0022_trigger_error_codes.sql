-- 0022_trigger_error_codes.sql (PostgreSQL port, PG-003)
--
-- The four validation triggers `migrations/0015_…`/`0016_…` first
-- created and `migrations/0022_…` re-created with SQLite's bracketed
-- `[E_CODE]` convention. PostgreSQL carries the same five codes
-- **out of band**, via `RAISE EXCEPTION … USING ERRCODE = '…'` —
-- `dialect.rs`'s own module doc named this as the reason SQLite's
-- bracket trick is not a PostgreSQL shape at all, not a smaller
-- version of it. `peisear-storage::dialect::translate_trigger_error`
-- reads this build's `sqlx::Error::Database(..).code()` directly;
-- there is no message text to parse on this backend.
--
-- **Five custom `ERRCODE`s, `PE001`–`PE005`** — PostgreSQL does not
-- reserve a registry for application-defined codes; any unused
-- 5-character string is valid. `PE` ("peisear error") is chosen to
-- be visibly not one of PostgreSQL's own built-in classes (which are
-- two-digit numeric or alphanumeric prefixes documented in the
-- manual's error-codes appendix), so a reader never mistakes one for
-- a constraint violation or a connection error.
--
-- **Two functions, not four** — `issues_planned_range_check_insert`
-- and `_update` share identical logic in the SQLite original (the
-- same condition, the same message, differing only in which
-- statement triggers it); here they share one function, attached to
-- two triggers, so the logic exists once. The sub-issue nesting pair
-- does not share logic (the update body carries two additional
-- checks the insert body cannot reach: self-parent and demote
-- below), so it stays two functions, matching the original's own
-- asymmetry rather than forcing a false shared shape onto it.
--
-- **Four trigger objects, same count as the SQLite original** —
-- `prevent_sub_issue_nesting_insert`, `prevent_sub_issue_nesting_update`,
-- `issues_planned_range_check_insert`, `issues_planned_range_check_update`.
-- Combined with `0021`'s seven, this schema now carries eleven
-- triggers on PostgreSQL, the same count `PG-003`'s own measurement
-- found on SQLite.

CREATE FUNCTION fn_prevent_sub_issue_nesting_insert() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_issue_id IS NOT NULL THEN
        IF EXISTS (
            SELECT 1 FROM issues
             WHERE id = NEW.parent_issue_id
               AND parent_issue_id IS NOT NULL
        ) THEN
            RAISE EXCEPTION 'sub-issue cannot have a sub-issue (1-level only)'
                USING ERRCODE = 'PE001';
        END IF;
        IF NOT EXISTS (
            SELECT 1 FROM issues
             WHERE id = NEW.parent_issue_id
               AND project_id = NEW.project_id
        ) THEN
            RAISE EXCEPTION 'sub-issue must share project with its parent'
                USING ERRCODE = 'PE002';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_sub_issue_nesting_insert
    BEFORE INSERT ON issues
    FOR EACH ROW
    EXECUTE FUNCTION fn_prevent_sub_issue_nesting_insert();

CREATE FUNCTION fn_prevent_sub_issue_nesting_update() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_issue_id IS NOT NULL THEN
        IF EXISTS (
            SELECT 1 FROM issues
             WHERE id = NEW.parent_issue_id
               AND parent_issue_id IS NOT NULL
        ) THEN
            RAISE EXCEPTION 'sub-issue cannot have a sub-issue (1-level only)'
                USING ERRCODE = 'PE001';
        END IF;
        IF NEW.parent_issue_id = NEW.id THEN
            RAISE EXCEPTION 'an issue cannot be its own parent'
                USING ERRCODE = 'PE003';
        END IF;
        IF NOT EXISTS (
            SELECT 1 FROM issues
             WHERE id = NEW.parent_issue_id
               AND project_id = NEW.project_id
        ) THEN
            RAISE EXCEPTION 'sub-issue must share project with its parent'
                USING ERRCODE = 'PE002';
        END IF;
        IF EXISTS (SELECT 1 FROM issues WHERE parent_issue_id = NEW.id) THEN
            RAISE EXCEPTION 'cannot demote an issue that has its own sub-issues; promote them first'
                USING ERRCODE = 'PE004';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_sub_issue_nesting_update
    BEFORE UPDATE OF parent_issue_id ON issues
    FOR EACH ROW
    EXECUTE FUNCTION fn_prevent_sub_issue_nesting_update();

CREATE FUNCTION fn_issues_planned_range_check() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.planned_start_at IS NOT NULL
       AND NEW.planned_end_at IS NOT NULL
       AND NEW.planned_end_at < NEW.planned_start_at THEN
        RAISE EXCEPTION 'Planned end date must be on or after planned start date.'
            USING ERRCODE = 'PE005';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER issues_planned_range_check_insert
    BEFORE INSERT ON issues
    FOR EACH ROW
    EXECUTE FUNCTION fn_issues_planned_range_check();

CREATE TRIGGER issues_planned_range_check_update
    BEFORE UPDATE OF planned_start_at, planned_end_at ON issues
    FOR EACH ROW
    EXECUTE FUNCTION fn_issues_planned_range_check();
