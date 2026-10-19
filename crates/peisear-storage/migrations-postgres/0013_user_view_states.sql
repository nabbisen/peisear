-- 0013_user_view_states.sql (PostgreSQL port, PG-002)
-- Rationale: see migrations/0013_user_view_states.sql.
--
-- Porting note, stated rather than silently made: the SQLite
-- original types `updated_at` as `TEXT DEFAULT (CURRENT_TIMESTAMP)`,
-- not `DATETIME` like every sibling column -- nothing in the Rust
-- layer decodes it as a structured value today, so SQLite's own
-- loose type affinity made the difference invisible there. `0017`
-- adds a bump trigger on this column with the *same* pattern the
-- other six `updated_at` triggers use, which is the real intent:
-- an instant, like its siblings, not a string this table happens to
-- store. Ported as `TIMESTAMPTZ` for that reason -- consistent with
-- every other `updated_at` column in this directory, and so `PG-003`
-- can write this trigger with `NOW()` the same way as the other six
-- rather than a text-formatting special case for one column.

CREATE TABLE user_view_states (
    user_id     TEXT NOT NULL,
    view_key    TEXT NOT NULL,
    state_json  TEXT NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (user_id, view_key),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
