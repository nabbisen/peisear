-- 0010_notifications.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0010_notifications.sql. Porting note:
-- `DATETIME` -> `TIMESTAMPTZ`, same reason as `0001`. `DESC` in an
-- index column list is standard SQL, supported identically.

CREATE TABLE notifications (
    id              TEXT PRIMARY KEY,
    user_id         TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    kind            TEXT NOT NULL,
    severity        TEXT NOT NULL CHECK (severity IN ('info', 'watch')),
    title           TEXT NOT NULL,
    body            TEXT NOT NULL,
    payload_json    TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    read_at         TIMESTAMPTZ,
    dispatched_via  TEXT NOT NULL DEFAULT ''
);

CREATE INDEX idx_notifications_user_kind_created
    ON notifications(user_id, kind, created_at DESC);

CREATE INDEX idx_notifications_user_created
    ON notifications(user_id, created_at DESC);


CREATE TABLE notification_preferences (
    user_id        TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    kind           TEXT NOT NULL,
    channels       TEXT NOT NULL DEFAULT 'in_app',
    min_severity   TEXT NOT NULL DEFAULT 'info'
                   CHECK (min_severity IN ('info', 'watch')),

    PRIMARY KEY (user_id, kind)
);
