-- 0016_issue_planned_dates.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0016_issue_planned_dates.sql.
--
-- **Triggers omitted, deliberately** — same note as `0014`/`0015`.
-- `issues_planned_range_check_insert`/`_update` are the remaining
-- two of the five validation triggers `PG-003` ports. **Until
-- `PG-003` lands, a planned end date before a planned start date is
-- not rejected on the PostgreSQL side.**
--
-- Porting note: the SQLite original types these columns `TIMESTAMP`
-- (not `DATETIME`) — SQLite treats both as the same affinity, but
-- PostgreSQL distinguishes `TIMESTAMP` (no time zone) from
-- `TIMESTAMPTZ`. Ported as `TIMESTAMPTZ` for the same reason as
-- every other instant-valued column in this directory (`0001`):
-- these are `chrono::DateTime<Utc>` in the Rust layer, and
-- `TIMESTAMPTZ` is what agrees with `CURRENT_TIMESTAMP`'s own
-- return type without a cast.

ALTER TABLE issues
    ADD COLUMN planned_start_at TIMESTAMPTZ;

ALTER TABLE issues
    ADD COLUMN planned_end_at TIMESTAMPTZ;

CREATE INDEX idx_issues_planned_window
    ON issues(planned_start_at)
    WHERE planned_start_at IS NOT NULL;
