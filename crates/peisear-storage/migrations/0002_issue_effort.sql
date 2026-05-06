-- 0002_issue_effort.sql
-- Add a per-issue effort estimate (story points).
--
-- Nullable on purpose: estimation is gradual, and forcing every existing
-- issue to have an effort would be a UX failure on roll-out. Issues that
-- have not been estimated render as "—" in the UI.
--
-- The CHECK constraint enforces a positive value; UI presets are 1, 2, 3,
-- 5, 8, 13 (Fibonacci-ish), but any positive integer is valid for users
-- who want a different scale.

ALTER TABLE issues
    ADD COLUMN effort INTEGER NULL CHECK (effort IS NULL OR effort > 0);
