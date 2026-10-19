//! The dialect seam (`DIA-002`, `RFC 018` P2) — the one place a
//! second storage backend would attach, built with **no PostgreSQL
//! code**: not a query, not a migration, not a connection string.
//!
//! `sqlite` is this crate's default feature and the only one with
//! real code behind it; `postgres` is additive, off by default, and
//! exists so the crate compiles against `sqlx`'s own PostgreSQL
//! driver with nothing wired to it yet. Measured 2026-10-17: one
//! `Pool` alias (below), zero compile-time `sqlx::query!` macros
//! anywhere under `src/` (every query in this crate is a runtime
//! string, which is the only reason a dialect seam is feasible at
//! all — a macro checks its SQL against one database at compile
//! time, and none of this crate's query call sites do), and **131**
//! runtime `sqlx::query`/`query_as`/`query_scalar` call sites in
//! production code (`src/`, excluding `tests.rs` and `src/**/tests/`)
//! — the figure that matters for what a dialect module would need to
//! abstract, since test helpers are not the seam. **176** including
//! this crate's own tests, for completeness; that is not the number
//! above, and the two should never be quoted interchangeably (round 2,
//! `DIA-002` §9 — an earlier draft of this sentence said 148, which
//! was neither population, found only because the report this module
//! shipped beside had already measured both correctly).
//!
//! **What this module owns, and why each piece is here rather than
//! left where it was:**
//!
//! - **[`Pool`]** — the one alias every query function already took
//!   by reference; swapping it is the entire point of a seam.
//! - **The trigger error-code translation** (`DIA-001`) — genuinely
//!   backend-specific: SQLite's `RAISE` carries no machine-readable
//!   code, so the code travels inside the message as a bracketed
//!   prefix; PostgreSQL's `RAISE EXCEPTION … USING ERRCODE = '…'`
//!   would carry a real one, needing a different translation
//!   function entirely, not a parameter on this one.
//!
//! **What this module deliberately does not own — left unabstracted
//! for `P3`, not forgotten (§3's own warning: a guess written into a
//! trait is harder to remove than one left out):**
//!
//! - **Placeholder style.** SQLite's runtime queries use `?`;
//!   PostgreSQL needs `$1, $2, …`. Abstracting this for real would
//!   mean touching every one of the 131 production call sites named
//!   above — `DIA-002` §7's own stop condition — so it is named here
//!   and left exactly as it is. `P3` is where a second real
//!   implementation exists to abstract *against*, not guess at.
//! - **Every runtime query itself.** `sqlx::query(…)`'s runtime API
//!   does not validate SQL text against a specific database at
//!   compile time, so SQLite-flavoured SQL compiles unchanged
//!   regardless of which feature is active. `pool::migrate` (`PG-002`)
//!   now applies a real, ported PostgreSQL schema — `?` placeholders
//!   and SQLite date functions are `PG-004`'s, 295 occurrences across
//!   20 files — so every one of those queries now fails at *runtime*
//!   against the real server `pool::connect` (`PG-001`) reaches,
//!   rather than never reaching a point where failing was possible.
//!
//! **No `#[cfg(feature = "postgres")]` anywhere outside this module
//! and `pool.rs`.** Five files (`lib.rs`, `teams.rs`, `sprints.rs`,
//! `user_capacities.rs`, `issue_events.rs`) do reference [`Database`],
//! [`Tx`] or [`Connection`] in place of a hard-coded `sqlx::Sqlite` /
//! `sqlx::SqliteConnection` — a changed *type name*, found in two
//! passes by building with `postgres` selected and reading what
//! still failed, not a conditional-compilation block of their own.
//! None of the fourteen occurrences carries a `#[cfg]`.

#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!(
    "peisear-storage: `sqlite` and `postgres` are alternatives, not additions. Build with \
     `--no-default-features --features postgres` to select PostgreSQL."
);

#[cfg(not(any(feature = "sqlite", feature = "postgres")))]
compile_error!(
    "peisear-storage: no storage backend selected. Enable exactly one of the `sqlite` \
     (default) or `postgres` features."
);

/// The active backend's pool type. Every query function in this crate
/// takes `&Pool` (or `Pool`) rather than a concrete `SqlitePool` —
/// this alias is the entire reason that was never a cosmetic choice.
#[cfg(feature = "sqlite")]
pub type Pool = sqlx::SqlitePool;

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub type Pool = sqlx::PgPool;

/// The active backend's `sqlx::Database` marker type.
///
/// **Found while proving the seam, not designed in advance**: a
/// handful of functions across the crate (`lib.rs`, `teams.rs`,
/// `sprints.rs`, `user_capacities.rs`, `issue_events.rs`) are generic over
/// `E: sqlx::Executor<'e, Database = sqlx::Sqlite>` or take
/// `tx: &mut sqlx::Transaction<'_, Sqlite>` directly, hard-coding
/// `Sqlite` the same way the old, un-aliased `Pool` field would have.
/// Every one of the eleven occurrences is a type bound or a parameter
/// annotation — no SQL text, no query, no migration — so repointing
/// them at [`Database`]/[`Tx`] is the same class of fix as [`Pool`]
/// itself, not a second abstraction invented for this round. Found by
/// building with `postgres` selected and reading the 82 resulting
/// errors, not by searching for the pattern first — `cargo check`
/// found this gap before any manual read would have.
#[cfg(feature = "sqlite")]
pub type Database = sqlx::Sqlite;

#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub type Database = sqlx::Postgres;

/// The active backend's transaction type, parameterised the same way
/// [`sqlx::Transaction`] itself is.
pub type Tx<'c> = sqlx::Transaction<'c, Database>;

/// The active backend's raw connection type. `sprints.rs` took three
/// functions a `&mut sqlx::SqliteConnection` directly (not through a
/// pool or a transaction) — the same hard-coding [`Database`]/[`Tx`]
/// above fix, found the same way: building with `postgres` selected
/// and reading what still failed after those two landed.
pub type Connection = <Database as sqlx::Database>::Connection;

/// `DIA-001`: the bracketed code every known trigger `RAISE` message
/// carries, paired with the `MessageKey` that renders the human
/// sentence alongside it. Also the registry
/// [`every_raise_code_is_known`] and
/// [`every_known_code_appears_in_a_live_trigger`] both check
/// against, so there is exactly one list to keep in sync — with
/// whatever `sqlite_master` says a fully-migrated database's triggers
/// actually raise (`DIA-001` round 3), not with any one migration
/// file. `0022` is where these five currently live, having dropped
/// and re-created `0015`/`0016`'s own four (those two are frozen
/// byte-for-byte at their `0.53.0` content and carry the original
/// prose forever) — but a future migration that touches these
/// triggers again needs no change here or in the guard, only in this
/// list and in whichever migration it lands in.
///
/// **SQLite-only** (`PG-003`): PostgreSQL carries the same five
/// `MessageKey`s in `TRIGGER_ERROR_CODES_POSTGRES` (not a doc-linkable
/// item here — it only exists under the `postgres` feature, the
/// mirror image of this `const`), a separate table rather than a
/// shared one — the two backends' own codes are different mechanisms
/// (a bracket inside the message text here, an out-of-band `ERRCODE`
/// there), not two spellings of the same identifier, so merging them
/// into one struct would be the "leaky abstraction" `dialect.rs`'s
/// own module doc already declined for the both-directions guard.
///
/// **No mechanical drift guard between the two tables is possible**:
/// the `sqlite`/`postgres` features are mutually exclusive (this
/// file's own `compile_error!`s), so no test can reference both
/// `const`s at once to compare them. Kept in sync by hand instead —
/// both lists are five entries long, both ordered
/// depth / project-mismatch / self-parent / demote / planned-date,
/// so a reviewer can diff them by eye at either file. If a sixth
/// validation ever needs a code, add it to both in the same change.
#[cfg(feature = "sqlite")]
pub(crate) const TRIGGER_ERROR_CODES: &[(&str, peisear_i18n::MessageKey)] = &[
    (
        "E_SUB_ISSUE_DEPTH",
        peisear_i18n::MessageKey::SubIssueCannotHaveSubIssueMessage,
    ),
    (
        "E_SUB_ISSUE_PROJECT_MISMATCH",
        peisear_i18n::MessageKey::SubIssueMustShareProjectMessage,
    ),
    (
        "E_ISSUE_SELF_PARENT",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ),
    (
        "E_ISSUE_DEMOTE_HAS_SUBISSUES",
        peisear_i18n::MessageKey::CannotDemoteIssueWithSubIssuesMessage,
    ),
    (
        "E_ISSUE_PLANNED_DATE_ORDER",
        peisear_i18n::MessageKey::IssuePlannedEndBeforeStartMessage,
    ),
];

/// The code inside a `[CODE]` bracket anywhere in `msg`, if one is
/// there. **Not anchored to the start of `msg`** — measured directly
/// against a real trigger failure before trusting this: `sqlx::Error`'s
/// own `Display` is `"error returned from database: {0}"`
/// (`sqlx-core`'s `error.rs`), so the bracket this function is
/// actually asked to find never opens at position 0, only
/// `msg.contains` would have found it. `None` for any message with no
/// `[…]` at all, which is every `sqlx::Error` this function was never
/// meant to translate (a constraint violation with no trigger
/// involved, a connection error, and so on).
///
/// **SQLite-only** (`PG-003`): PostgreSQL's `RAISE EXCEPTION …
/// USING ERRCODE = '…'` carries a real out-of-band code, so the
/// postgres branch of [`translate_trigger_error`] below never calls
/// this — there is no bracket to find in a message PostgreSQL never
/// puts one in.
#[cfg(feature = "sqlite")]
pub(crate) fn trigger_error_code(msg: &str) -> Option<&str> {
    let idx = msg.find('[')?;
    let rest = &msg[idx + 1..];
    let end = rest.find(']')?;
    Some(&rest[..end])
}

/// Translate a raw `sqlx::Error` into the closest
/// [`crate::StorageError`] for sub-issue and planned-date trigger
/// violations.
///
/// **Matches a code, never prose** (`DIA-001`, 0.54.0). Before that
/// round, this function matched `msg.contains(english sentence)` —
/// brittle twice over: two of the five needles were *prefixes* of the
/// real message rather than the whole of it, so a reword landing
/// after the matched prefix would have kept passing silently, and the
/// coupling meant the English wording could never change without
/// also touching this function. SQLite's `RAISE` has no
/// machine-readable error code the way a real constraint violation
/// does, so the code is carried *inside* the message text instead, as
/// a `[E_CODE]` prefix no ordinary English sentence could be mistaken
/// for — chosen over a bare word for exactly that reason.
///
/// **The code and the live trigger both live in
/// `0022_trigger_error_codes.sql`, not in `0015`/`0016`** (round 2:
/// an in-place edit to those two broke every existing `0.53.0`
/// install, since `sqlx` refuses to re-run a migration whose
/// checksum has changed — demonstrated, not assumed, at that round's
/// review). `0022` drops and re-creates the five triggers this
/// function translates, with the identical sentence
/// [`peisear_i18n::MessageKey`] renders, now carrying the code ahead
/// of it. `0015`/`0016` keep the original prose-only text forever,
/// unreachable once `0022` has run, kept only because changing a
/// byte of either breaks every database that already applied them.
///
/// **PostgreSQL has its own body below**, not a parameter on this
/// one — `RAISE EXCEPTION … USING ERRCODE = '…'` carries a real
/// out-of-band code, so there is no bracket to parse at all on that
/// backend, which is exactly why the SQLite-only names chosen here
/// (`E_SUB_ISSUE_DEPTH`, not a number) carry no SQLite-specific
/// meaning of their own — `TRIGGER_ERROR_CODES_POSTGRES` reuses the
/// same `MessageKey`s under PostgreSQL's own `ERRCODE` strings.
#[cfg(feature = "sqlite")]
pub(crate) fn translate_trigger_error(e: sqlx::Error) -> crate::StorageError {
    let msg = e.to_string();
    if let Some(code) = trigger_error_code(&msg) {
        for (known_code, key) in TRIGGER_ERROR_CODES {
            if *known_code == code {
                return crate::StorageError::Validation(key.clone());
            }
        }
    }
    e.into()
}

/// PostgreSQL's own five validation codes (`PG-003`), the same
/// `MessageKey`s `TRIGGER_ERROR_CODES` carries for SQLite (not a
/// doc-linkable item here — `sqlite`-only), keyed by the `ERRCODE`
/// the triggers in `migrations-postgres/0022_…` raise
/// via `RAISE EXCEPTION … USING ERRCODE = '…'` rather than a bracket
/// in the message. `PE001`–`PE005`: PostgreSQL reserves no registry
/// for application-defined codes, and `PE` ("peisear error") is
/// chosen to be visibly distinct from any of PostgreSQL's own
/// built-in classes, documented in that migration's own header.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub(crate) const TRIGGER_ERROR_CODES_POSTGRES: &[(&str, peisear_i18n::MessageKey)] = &[
    (
        "PE001",
        peisear_i18n::MessageKey::SubIssueCannotHaveSubIssueMessage,
    ),
    (
        "PE002",
        peisear_i18n::MessageKey::SubIssueMustShareProjectMessage,
    ),
    (
        "PE003",
        peisear_i18n::MessageKey::IssueCannotBeOwnParentMessage,
    ),
    (
        "PE004",
        peisear_i18n::MessageKey::CannotDemoteIssueWithSubIssuesMessage,
    ),
    (
        "PE005",
        peisear_i18n::MessageKey::IssuePlannedEndBeforeStartMessage,
    ),
];

/// PostgreSQL sibling of the SQLite [`translate_trigger_error`]
/// above (`PG-003`). Reads `sqlx::Error::Database`'s own `.code()` —
/// PostgreSQL's out-of-band `ERRCODE`, the `SQLSTATE` the wire
/// protocol itself carries — rather than parsing anything out of the
/// message text, since there is no bracket to find on this backend.
#[cfg(all(feature = "postgres", not(feature = "sqlite")))]
pub(crate) fn translate_trigger_error(e: sqlx::Error) -> crate::StorageError {
    if let sqlx::Error::Database(ref db_err) = e
        && let Some(code) = db_err.code()
    {
        for (known_code, key) in TRIGGER_ERROR_CODES_POSTGRES {
            if *known_code == code.as_ref() {
                return crate::StorageError::Validation(key.clone());
            }
        }
    }
    e.into()
}

/// Every `RAISE(ABORT, '…')` message's full quoted text carried by a
/// **live trigger** in `pool`'s own `sqlite_master` — not a migration
/// file, after `DIA-001` round 3 found the file-scoped version of
/// this check (round 2's own `all_raise_messages`, reading
/// `0022_trigger_error_codes.sql` alone) could not survive the next
/// migration: a future `0023` adding an uncoded `RAISE` would reach
/// users as a raw, untyped error while this check kept reading `0022`
/// and staying green. `sqlite_master` is what a real, fully-migrated
/// database actually has, regardless of which file most recently
/// defined it, so a later migration is covered the day it lands, with
/// nothing here to update. The full message, not only a bracketed
/// prefix, so a `RAISE` carrying no code at all is still part of this
/// population (`§10.39`'s own class: a needle that can only find what
/// already matches its own shape).
///
/// **Widened past `ABORT`** (DIA-001 closing review, 2026-10-17): the
/// marker used to be `"RAISE(ABORT, '"` specifically, the same class
/// one level below the one this doc comment already named — SQLite's
/// `RAISE` also takes `FAIL`, `ROLLBACK` and `IGNORE`, and keying on
/// one keyword only would leave a future `RAISE(FAIL, '…')` invisible
/// to this check for exactly the reason this whole function exists to
/// avoid. Demonstrated, not assumed: a planted `RAISE(FAIL, 'planted
/// uncoded message')` passed invisibly under the `ABORT`-only marker
/// and failed, naming the planted message, under this one. `IGNORE`
/// takes no message argument at all, so it can never match either
/// form and needs no special case.
#[cfg(all(test, feature = "sqlite"))]
async fn live_trigger_raise_messages(pool: &Pool) -> Vec<String> {
    let markers = ["RAISE(ABORT, '", "RAISE(FAIL, '", "RAISE(ROLLBACK, '"];
    let sqls: Vec<String> =
        sqlx::query_scalar("SELECT sql FROM sqlite_master WHERE type = 'trigger'")
            .fetch_all(pool)
            .await
            .expect("read sqlite_master");
    let mut out = Vec::new();
    for sql in &sqls {
        for marker in markers {
            let mut rest = sql.as_str();
            while let Some(idx) = rest.find(marker) {
                let after = &rest[idx + marker.len()..];
                let Some(end) = after.find("')") else { break };
                out.push(after[..end].to_string());
                rest = &after[end..];
            }
        }
    }
    out
}

/// `DIA-001` §3/§13, direction 1: every `RAISE` message found carries
/// a code `known` recognises. Takes `known` as a parameter, not
/// [`TRIGGER_ERROR_CODES`] directly, so a test can plant a shrunk
/// list against the real, live triggers without touching a committed
/// file (the handoff's own "remove a code the translation knows"
/// plant).
#[cfg(all(test, feature = "sqlite"))]
fn every_raise_code_is_known(
    messages: &[String],
    known: &[(&str, peisear_i18n::MessageKey)],
) -> Vec<String> {
    let mut out = Vec::new();
    for msg in messages {
        match trigger_error_code(msg) {
            Some(code) if known.iter().any(|(c, _)| *c == code) => {}
            Some(code) => out.push(format!(
                "a live trigger RAISEs '{msg}', code {code}, which translate_trigger_error does not know"
            )),
            None => out.push(format!(
                "a live trigger RAISEs '{msg}', which carries no [CODE] prefix"
            )),
        }
    }
    out
}

/// `DIA-001` §3/§13, direction 2: every code `known` carries appears
/// in at least one live `RAISE` message found. This is the direction
/// that direction 1 cannot catch on its own — a code
/// `TRIGGER_ERROR_CODES` still lists after the trigger that used to
/// raise it is dropped (and never re-created) would pass direction 1
/// forever, silently dead.
#[cfg(all(test, feature = "sqlite"))]
fn every_known_code_appears_in_a_live_trigger(
    messages: &[String],
    known: &[(&str, peisear_i18n::MessageKey)],
) -> Vec<String> {
    let mut out = Vec::new();
    for (code, _) in known {
        if !messages
            .iter()
            .any(|m| trigger_error_code(m) == Some(*code))
        {
            out.push(format!(
                "translate_trigger_error knows code {code}, which no live trigger RAISEs"
            ));
        }
    }
    out
}

/// PostgreSQL's own both-directions guard (`PG-003` §2): **two
/// honest functions, not a shared abstraction over SQLite's** — the
/// handoff's own preference, since the introspection mechanism
/// genuinely differs (SQLite reads a trigger's inline SQL text from
/// `sqlite_master`; PostgreSQL's trigger body lives in a separate
/// function, found by joining `pg_trigger` to `pg_proc`), not merely
/// styled differently.
///
/// **`NOT tgisinternal`** is load-bearing, not decoration: PostgreSQL
/// implements every `ON DELETE CASCADE`/`SET NULL` foreign key as an
/// internal trigger of its own, invisible in the SQLite original
/// (which has no such mechanism — cascades are a core engine
/// feature there, not trigger-backed). Measured against this
/// schema's own real foreign keys before trusting the filter: without
/// it, this query returns dozens of rows for referential-integrity
/// triggers this project never wrote and does not own; with it,
/// exactly the ones `migrations-postgres/0021_…`/`0022_…` create.
///
/// Returns every `ERRCODE` found in a trigger function's own source
/// (`pg_proc.prosrc`) — the out-of-band equivalent of
/// `live_trigger_raise_messages`'s bracket scan, searching for the
/// literal `ERRCODE = '…'` this round's own migration writes rather
/// than a bracket no PostgreSQL `RAISE` ever carries.
#[cfg(all(test, feature = "postgres", not(feature = "sqlite")))]
async fn live_trigger_errcodes_postgres(pool: &Pool) -> Vec<String> {
    let marker = "ERRCODE = '";
    let sources: Vec<String> = sqlx::query_scalar(
        "SELECT p.prosrc \
           FROM pg_trigger t \
           JOIN pg_proc p ON p.oid = t.tgfoid \
          WHERE NOT t.tgisinternal",
    )
    .fetch_all(pool)
    .await
    .expect("read pg_trigger/pg_proc");

    let mut out = Vec::new();
    for src in &sources {
        let mut rest = src.as_str();
        while let Some(idx) = rest.find(marker) {
            let after = &rest[idx + marker.len()..];
            let Some(end) = after.find('\'') else { break };
            out.push(after[..end].to_string());
            rest = &after[end..];
        }
    }
    out
}

/// PostgreSQL direction 1 (`PG-003` §2): every `ERRCODE` a live
/// trigger function raises is one `known` recognises. Unlike the
/// SQLite direction 1 above, there is no bracket to fail to find —
/// `live_trigger_errcodes_postgres` already extracts bare codes, so
/// this compares them directly.
#[cfg(all(test, feature = "postgres", not(feature = "sqlite")))]
fn every_errcode_is_known_postgres(
    codes: &[String],
    known: &[(&str, peisear_i18n::MessageKey)],
) -> Vec<String> {
    codes
        .iter()
        .filter(|code| !known.iter().any(|(c, _)| c == code))
        .map(|code| format!("a live trigger raises ERRCODE '{code}', which nothing here knows"))
        .collect()
}

/// PostgreSQL direction 2 (`PG-003` §2): every code `known` carries
/// appears in at least one live trigger's own source.
#[cfg(all(test, feature = "postgres", not(feature = "sqlite")))]
fn every_known_errcode_appears_in_a_live_trigger_postgres(
    codes: &[String],
    known: &[(&str, peisear_i18n::MessageKey)],
) -> Vec<String> {
    known
        .iter()
        .filter(|(code, _)| !codes.iter().any(|c| c == code))
        .map(|(code, _)| {
            format!("translate_trigger_error knows ERRCODE '{code}', which no live trigger raises")
        })
        .collect()
}

/// `DIA-001`'s own tests, all of them against a real SQLite database
/// -- gated here rather than per-test inside the file (`PG-003`),
/// since every one of them is SQLite-specific and none would compile
/// against the postgres-only items above.
#[cfg(all(test, feature = "sqlite"))]
mod tests;

/// `PG-003`'s own tests, against a real PostgreSQL database -- a
/// separate file rather than more `#[cfg]` inside `tests.rs`, since
/// nothing in that file is reachable under `postgres` and mixing the
/// two would need a `#[cfg]` on every item rather than one on the
/// module declaration.
#[cfg(all(test, feature = "postgres", not(feature = "sqlite")))]
mod postgres_tests;
