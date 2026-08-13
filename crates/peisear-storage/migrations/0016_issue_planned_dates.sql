-- 0016_issue_planned_dates.sql
--
-- Adds `planned_start_at` / `planned_end_at` to `issues`
-- (peisear-feature-spec-v2.1 §16, RFC 002, `CAL-001`). The data
-- layer for the calendar surfaces CAL-002 renders later -- this
-- migration ships alone and nothing reads these columns yet.
--
-- ## Two columns, not the spec's four
--
-- The spec (§38.1 task 2) lists `start_date`, `due_date`,
-- `planned_start_at`, `planned_end_at`. RFC 002's own background
-- section narrows this to the two here: `start_date` would mirror
-- `planned_start_at::date` for most uses, and `due_date` already
-- lives in the user's mental model as "a hard ceiling on
-- `planned_end_at`" -- carrying both invites a "which one is the
-- real deadline?" bug class. If a real distinction emerges later,
-- add it then.
--
-- ## Why a trigger, not CHECK
--
-- SQLite `CHECK` can't reference NULL semantics the way this
-- constraint needs (both columns nullable independently; the
-- constraint only applies when *both* are set) -- a `BEFORE
-- INSERT`/`BEFORE UPDATE OF` trigger pair, same shape as 0015's
-- sub-issue nesting checks.
--
-- ## The RAISE text is user-facing copy (`CAL-001` §2.3)
--
-- Per `DEC-011`, `translate_trigger_error`
-- (`peisear-storage/src/issues.rs`) matches this string as a
-- needle and returns the `MessageKey`
-- (`IssuePlannedEndBeforeStartMessage`) that renders it to the
-- user -- so this sentence, not a later web-layer rewording, is
-- where the wording gets decided. It intentionally does not name
-- the database columns (`planned_start_at`/`planned_end_at`) --
-- COPY-001 fixed exactly that shape three days before this
-- migration was written, on the capacity-period form
-- ("period_start must be on or before period_end" ->
-- "The From date must be on or before the To date."). The two
-- issue-edit-form field labels this text matches are "Planned
-- start date" and "Planned end date"
-- (`peisear_i18n::Field::PlannedStartDate`/`PlannedEndDate`).
--
-- ## Both triggers, insert and update
--
-- A constraint enforced on only one path is a constraint that
-- holds until someone uses the other -- same reasoning as 0015.
-- `issues::insert` did not previously route its `sqlx::Error`
-- through `translate_trigger_error` (nothing before this migration
-- could trigger on a plain top-level insert); it now does, so this
-- trigger's violations translate the same way on both paths.
--
-- ## Partial index
--
-- Most issues in a real project will never have plan dates --
-- `WHERE planned_start_at IS NOT NULL` keeps the index small and
-- lets the calendar window query (CAL-002) skip unplanned rows
-- entirely.

ALTER TABLE issues
    ADD COLUMN planned_start_at TIMESTAMP;

ALTER TABLE issues
    ADD COLUMN planned_end_at TIMESTAMP;

CREATE TRIGGER issues_planned_range_check_insert
BEFORE INSERT ON issues
FOR EACH ROW
WHEN NEW.planned_start_at IS NOT NULL
 AND NEW.planned_end_at IS NOT NULL
 AND NEW.planned_end_at < NEW.planned_start_at
BEGIN
    SELECT RAISE(ABORT, 'planned end date must be on or after planned start date');
END;

CREATE TRIGGER issues_planned_range_check_update
BEFORE UPDATE OF planned_start_at, planned_end_at ON issues
FOR EACH ROW
WHEN NEW.planned_start_at IS NOT NULL
 AND NEW.planned_end_at IS NOT NULL
 AND NEW.planned_end_at < NEW.planned_start_at
BEGIN
    SELECT RAISE(ABORT, 'planned end date must be on or after planned start date');
END;

CREATE INDEX idx_issues_planned_window
    ON issues(planned_start_at)
    WHERE planned_start_at IS NOT NULL;
