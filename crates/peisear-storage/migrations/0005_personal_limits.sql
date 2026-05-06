-- 0005_personal_limits.sql
-- Add personal & project WIP limits (count of in-progress issues
-- per assignee). This is the "件数" half of the workload-fairness
-- model — distinct from `users.capacity_points` (story-point total)
-- shipped in 0.5.0.
--
-- Both halves matter and they are not redundant:
--
--   capacity_points  - "I can carry N points worth of work at once"
--                      → effort budget across all in-flight issues
--   wip_limit        - "I can have N issues open at any one time"
--                      → cognitive load / context-switching budget
--
-- A user might have capacity_points=10 (room for one big or a few
-- small issues) AND wip_limit=3 (no more than 3 things open at once
-- regardless of points). Both constraints can be violated
-- independently, both are surfaced as soft warnings.
--
-- ## Resolution order (web layer)
--
-- For a given user in a given project, the effective WIP limit is:
--   1. `users.wip_limit` if set
--   2. else `projects.wip_limit_default` if set
--   3. else system-wide default of 3 (constant in peisear-core)
--
-- ## Future direction
--
-- The system-wide default `3` lives in `peisear-core::personal_metrics`
-- as a const, not in storage. If product needs a deployment-wide
-- override (e.g. "this Anthropic install defaults to 5"), introduce
-- a `system_settings` key/value table at that point — premature now.
--
-- Roles (manager / neutral observer) that the V2.1 brief mentions
-- arrive with the planned Team / organisation feature; until then,
-- "owner of the project" = "the person being measured", and these
-- limits are personal preferences set by each user.

ALTER TABLE users
    ADD COLUMN wip_limit INTEGER NULL
        CHECK (wip_limit IS NULL OR wip_limit > 0);

ALTER TABLE projects
    ADD COLUMN wip_limit_default INTEGER NULL
        CHECK (wip_limit_default IS NULL OR wip_limit_default > 0);
