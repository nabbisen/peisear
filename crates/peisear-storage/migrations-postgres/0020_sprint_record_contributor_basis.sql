-- 0020_sprint_record_contributor_basis.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0020_sprint_record_contributor_basis.sql.
-- No porting notes -- every construct here (`ALTER TABLE ... ADD
-- COLUMN ... CHECK`, correlated subqueries, `COUNT(DISTINCT ...)`,
-- `COALESCE`/`CASE`) is standard SQL, unchanged. On a database
-- migrating from empty, `sprint_records` has no rows, so the
-- `UPDATE` below runs as before: real SQL, zero rows affected.

ALTER TABLE sprint_records ADD COLUMN contributor_count INTEGER NOT NULL DEFAULT 0;
ALTER TABLE sprint_records
    ADD COLUMN had_unassigned_contributor INTEGER NOT NULL DEFAULT 0
    CHECK (had_unassigned_contributor IN (0, 1));

UPDATE sprint_records
SET contributor_count = (
        SELECT COUNT(DISTINCT i.assignee_id)
        FROM sprint_issues si
        JOIN issues i ON i.id = si.issue_id
        WHERE si.sprint_id = sprint_records.sprint_id
          AND i.status = 'done'
    ),
    had_unassigned_contributor = (
        SELECT CASE WHEN COALESCE(SUM(CASE WHEN i.assignee_id IS NULL THEN 1 ELSE 0 END), 0) > 0
                    THEN 1 ELSE 0 END
        FROM sprint_issues si
        JOIN issues i ON i.id = si.issue_id
        WHERE si.sprint_id = sprint_records.sprint_id
          AND i.status = 'done'
    );
