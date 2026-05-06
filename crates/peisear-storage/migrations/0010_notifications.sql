-- 0010_notifications.sql
-- Two new tables for the notification subsystem (0.13.0).
--
-- ## Why two tables, not one
--
-- - `notifications` is the **inbox + audit log**. Every dispatched
--   notification appears here, even if the user is not subscribed
--   to in-app delivery (in which case the row exists only as an
--   audit trail of what was sent via other channels). This makes
--   "did the system send me anything yesterday?" answerable by a
--   single SELECT per user.
--
-- - `notification_preferences` records the user's per-kind delivery
--   choice. A row exists if the user has explicitly configured
--   that kind; absent rows fall back to the system defaults
--   defined in `peisear-core::notifications::DEFAULT_PREFERENCES`.
--   We do not pre-populate one row per (user, kind) on user
--   creation; that would couple table size to the number of
--   notification kinds we currently ship and turn every new
--   kind into an O(users) backfill.
--
-- ## Cooldown semantics live in `notifications`, not preferences
--
-- The 24-hour cooldown rule (per V2.1 §1.4 frequency control) is
-- enforced at dispatch time by querying `notifications` for the
-- most recent row of the same `(user_id, kind)`. No "last_sent_at"
-- column on the preferences table — that would be tracking state
-- in the wrong place (preferences should describe intent, not
-- history) and would make the audit story harder.

CREATE TABLE notifications (
    id              TEXT PRIMARY KEY,
    user_id         TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Identifies the rule that produced this notification.
    -- Kept as a free-form TEXT (not an enum) because new kinds
    -- ship continuously and a CHECK list would be brittle.
    -- The application owns the kind vocabulary; orphan kinds
    -- (no longer recognised by code) display generically.
    kind            TEXT NOT NULL,

    -- Severity drives both UI palette (info → ghost, watch →
    -- warning) and the per-kind preference filter (a user can
    -- subscribe only to severity ≥ watch).
    severity        TEXT NOT NULL CHECK (severity IN ('info', 'watch')),

    title           TEXT NOT NULL,
    body            TEXT NOT NULL,

    -- Free-form JSON for in-app rendering. Keeps the row schema
    -- stable as new kinds add structured payloads (e.g. a link
    -- to /me, a project_id, a delta number). Application
    -- decodes per-kind.
    payload_json    TEXT,

    created_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Read tracking is in-app only. Email and webhook channels
    -- have no notion of "read" so we don't try to model it.
    -- NULL = unread.
    read_at         DATETIME,

    -- Comma-separated list of channel ids that successfully
    -- delivered this notification. "in_app,email" / "in_app" /
    -- "" (failed everywhere). Application splits on comma.
    -- We use TEXT rather than a separate dispatch_attempts
    -- table because the channel list is small (≤3 today) and
    -- we never query "which notifications were dispatched via
    -- email"; the audit need is per-row.
    dispatched_via  TEXT NOT NULL DEFAULT ''
);

-- Cooldown lookup: "find the latest notification of this kind
-- for this user". Both columns appear in the WHERE clause, and
-- created_at is the ORDER BY, so the index is composite.
CREATE INDEX idx_notifications_user_kind_created
    ON notifications(user_id, kind, created_at DESC);

-- Inbox display: "list a user's recent notifications, newest
-- first". user_id is the primary filter; created_at is the sort.
-- This index also serves the unread-count badge (with a
-- read_at IS NULL filter on top, which the planner handles fine).
CREATE INDEX idx_notifications_user_created
    ON notifications(user_id, created_at DESC);


CREATE TABLE notification_preferences (
    user_id        TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Same vocabulary as notifications.kind. Foreign-key to
    -- nothing (no kinds table); the (user_id, kind) composite
    -- key is enough.
    kind           TEXT NOT NULL,

    -- Which channels deliver this kind for this user.
    -- Comma-separated channel ids; "" means silent (the
    -- notification is still recorded as an audit row, but no
    -- channels actually deliver). Application normalises by
    -- sorting + lowercasing on write.
    channels       TEXT NOT NULL DEFAULT 'in_app',

    -- Minimum severity that triggers a send. 'info' = all
    -- severities, 'watch' = watch only (skip info-level
    -- notifications). The default for most kinds is 'info'
    -- because we ship few info-level ones today; users who
    -- explicitly want the higher threshold can opt in.
    min_severity   TEXT NOT NULL DEFAULT 'info'
                   CHECK (min_severity IN ('info', 'watch')),

    PRIMARY KEY (user_id, kind)
);

-- The first-login email opt-in (Q3=A in design discussion) is
-- recorded by a sentinel row (kind = '_global', channels =
-- 'in_app' or 'in_app,email') so we don't need a third column on
-- users. Absence of the row = the user has not yet been prompted.
-- See peisear-core::notifications::GLOBAL_KIND.
