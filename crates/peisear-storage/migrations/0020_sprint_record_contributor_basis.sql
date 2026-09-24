-- 0020_sprint_record_contributor_basis.sql
--
-- `SPRINT-005` (RFC 0013, `DEC-054`, `NFR-PRIV-007`): the privacy gate reads the
-- basis captured with the record, not live data.
--
-- ## Why
--
-- `sprints::distinct_contributors` decides whether a sprint's burndown
-- trajectory and the velocity aggregate are shown: below two distinct people
-- who completed work, they would be reversible to an individual. It read *live*
-- membership and *live* status. After `0019`, what a completed sprint displays
-- is its captured record, and whether that record is reversible to one person is
-- a property of who contributed to it -- fixed at completion. Asking about *now*
-- meant work finished after completion could raise the count and open a
-- trajectory that was correctly hidden at completion, and carrying a done issue
-- out could hide one that was correctly shown.
--
-- ## Schema
--
-- Two columns on `sprint_records`: **the basis, not the decision.** The
-- two-contributor floor is policy and belongs in code where it can change; a
-- stored verdict would freeze today's policy into old rows. No contributor
-- identities are stored: copying person-identifying rows into a second table to
-- decide a privacy gate would be the wrong trade.
--
-- * `contributor_count` -- `COUNT(DISTINCT assignee_id)` over the sprint's
--   `done` issues at capture.
-- * `had_unassigned_contributor` -- 1 if any of those `done` issues had no
--   assignee: an unassigned completion could be anyone's, so the count is
--   unknown and the gate suppresses (`QA-017`'s safe direction).
ALTER TABLE sprint_records ADD COLUMN contributor_count INTEGER NOT NULL DEFAULT 0;
ALTER TABLE sprint_records
    ADD COLUMN had_unassigned_contributor INTEGER NOT NULL DEFAULT 0
    CHECK (had_unassigned_contributor IN (0, 1));

-- ## Backfill -- correct as of this migration, not as of completion
--
-- Every existing record gets both columns from the live data, by the same query
-- the gate used. **For a sprint whose issues were finished or moved after it
-- completed, that is not who contributed at completion** -- the drift `0019`'s
-- header already states, on a second axis, accepted for the same reason: a mixed
-- estate (some sprints captured, some still computing live) is worse than a
-- consistent one.
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
