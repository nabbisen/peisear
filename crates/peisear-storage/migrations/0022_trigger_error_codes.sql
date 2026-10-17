-- 0022_trigger_error_codes.sql
--
-- `DIA-001` round 2 (0.54.0). `0015_sub_issues.sql` and
-- `0016_issue_planned_dates.sql` are restored byte-for-byte to their
-- `0.53.0` content (`git show 0.53.0:<path>`, confirmed by sha256 and
-- by an empty `git diff 0.53.0 --`), because both shipped in that
-- release and `sqlx` refuses to run a migration whose checksum no
-- longer matches what a real database already applied
-- (`sqlx-core-0.8.6/src/migrate/migrator.rs:176`) -- demonstrated at
-- round 1's own review: a database created under 0.53.0's migration
-- text, then opened by a build carrying round 1's in-place edit,
-- refused to start. Round 1's "no production use yet" precedent
-- (2026-10-14) carried its own expiry, "at the first real
-- deployment" -- which 0.53.0 was, three days before round 1 cited
-- it.
--
-- This migration is additive instead: it drops the four triggers
-- `0015`/`0016` created and re-creates them with the identical
-- logic, differing only in the `RAISE` text, which now carries the
-- `[E_CODE]` prefix `translate_trigger_error`
-- (`peisear-storage/src/issues.rs`) matches on. A database that has
-- already applied `0015`/`0016` gets here by running this one
-- migration; a fresh database gets the same five triggers by running
-- `0015`, `0016`, then this one -- same end state, two routes, both
-- exercised directly rather than assumed equivalent (see the round's
-- own report for the two walks).
--
-- `DROP TRIGGER` has no `IF EXISTS`-shaped ambiguity to resolve
-- here: all four names are exactly what `0015`/`0016` created, and
-- this migration only ever runs once per database, after both of
-- those have (migrations apply in order, and `sqlx::migrate!` never
-- re-runs one that already succeeded).

DROP TRIGGER prevent_sub_issue_nesting_insert;
DROP TRIGGER prevent_sub_issue_nesting_update;
DROP TRIGGER issues_planned_range_check_insert;
DROP TRIGGER issues_planned_range_check_update;

CREATE TRIGGER prevent_sub_issue_nesting_insert
BEFORE INSERT ON issues
FOR EACH ROW
WHEN NEW.parent_issue_id IS NOT NULL
BEGIN
    SELECT RAISE(ABORT, '[E_SUB_ISSUE_DEPTH] sub-issue cannot have a sub-issue (1-level only)')
    WHERE EXISTS (
        SELECT 1 FROM issues
        WHERE id = NEW.parent_issue_id
          AND parent_issue_id IS NOT NULL
    );
    SELECT RAISE(ABORT, '[E_SUB_ISSUE_PROJECT_MISMATCH] sub-issue must share project with its parent')
    WHERE NOT EXISTS (
        SELECT 1 FROM issues
        WHERE id = NEW.parent_issue_id
          AND project_id = NEW.project_id
    );
END;

CREATE TRIGGER prevent_sub_issue_nesting_update
BEFORE UPDATE OF parent_issue_id ON issues
FOR EACH ROW
WHEN NEW.parent_issue_id IS NOT NULL
BEGIN
    SELECT RAISE(ABORT, '[E_SUB_ISSUE_DEPTH] sub-issue cannot have a sub-issue (1-level only)')
    WHERE EXISTS (
        SELECT 1 FROM issues
        WHERE id = NEW.parent_issue_id
          AND parent_issue_id IS NOT NULL
    );
    SELECT RAISE(ABORT, '[E_ISSUE_SELF_PARENT] an issue cannot be its own parent')
    WHERE NEW.parent_issue_id = NEW.id;
    SELECT RAISE(ABORT, '[E_SUB_ISSUE_PROJECT_MISMATCH] sub-issue must share project with its parent')
    WHERE NOT EXISTS (
        SELECT 1 FROM issues
        WHERE id = NEW.parent_issue_id
          AND project_id = NEW.project_id
    );
    SELECT RAISE(ABORT, '[E_ISSUE_DEMOTE_HAS_SUBISSUES] cannot demote an issue that has its own sub-issues; promote them first')
    WHERE EXISTS (
        SELECT 1 FROM issues
        WHERE parent_issue_id = NEW.id
    );
END;

CREATE TRIGGER issues_planned_range_check_insert
BEFORE INSERT ON issues
FOR EACH ROW
WHEN NEW.planned_start_at IS NOT NULL
 AND NEW.planned_end_at IS NOT NULL
 AND NEW.planned_end_at < NEW.planned_start_at
BEGIN
    SELECT RAISE(ABORT, '[E_ISSUE_PLANNED_DATE_ORDER] Planned end date must be on or after planned start date.');
END;

CREATE TRIGGER issues_planned_range_check_update
BEFORE UPDATE OF planned_start_at, planned_end_at ON issues
FOR EACH ROW
WHEN NEW.planned_start_at IS NOT NULL
 AND NEW.planned_end_at IS NOT NULL
 AND NEW.planned_end_at < NEW.planned_start_at
BEGIN
    SELECT RAISE(ABORT, '[E_ISSUE_PLANNED_DATE_ORDER] Planned end date must be on or after planned start date.');
END;
