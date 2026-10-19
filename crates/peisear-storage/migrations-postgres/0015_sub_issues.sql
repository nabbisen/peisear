-- 0015_sub_issues.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0015_sub_issues.sql.
--
-- **Triggers omitted, deliberately** — same note as `0014`.
-- `prevent_sub_issue_nesting_insert`/`_update` are two of the five
-- validation triggers `PG-003` ports with `ERRCODE` rather than
-- `DIA-001`'s bracket (`RFC 018` §5). **`PG-003` update**: both
-- triggers landed in `migrations-postgres/0022_…`, same version
-- number as the SQLite set's own re-creation of them — this file
-- stays a permanent placeholder for the trigger logic, the column
-- and its indexes being the part that belongs at this version.
--
-- Porting note: partial indexes (`CREATE INDEX ... WHERE ...`) are
-- supported identically in PostgreSQL; no translation needed.

ALTER TABLE issues
    ADD COLUMN parent_issue_id TEXT
        REFERENCES issues(id) ON DELETE CASCADE;

CREATE INDEX idx_issues_parent
    ON issues(parent_issue_id)
    WHERE parent_issue_id IS NOT NULL;

CREATE INDEX idx_issues_top_level
    ON issues(project_id, status)
    WHERE parent_issue_id IS NULL;
