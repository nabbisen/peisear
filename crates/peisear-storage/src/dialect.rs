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
//! - **Connection options.** `pool::connect`'s SQLite branch tunes
//!   WAL mode, synchronous level and busy timeout with types that
//!   have no PostgreSQL equivalent (`SqliteConnectOptions`,
//!   `SqliteJournalMode`). A PostgreSQL branch would need its own
//!   tuning, decided against a real server, not invented here.
//! - **Migration syntax.** `0001`–`0022` are SQLite SQL. PostgreSQL's
//!   own migration set is `P3`'s to write, against real differences
//!   (`SERIAL` vs SQLite's `INTEGER PRIMARY KEY`, trigger syntax, and
//!   so on) rather than this round's guesses.
//! - **Every runtime query itself.** `sqlx::query(…)`'s runtime API
//!   does not validate SQL text against a specific database at
//!   compile time, so SQLite-flavoured SQL compiles unchanged
//!   regardless of which feature is active; it would only fail at
//!   *runtime*, against a real PostgreSQL server, which `pool::connect`'s
//!   `unimplemented!()` postgres branch (below) never lets a caller
//!   reach.
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
/// **Not a PostgreSQL shape.** `RAISE EXCEPTION … USING ERRCODE =
/// '…'` carries a real out-of-band code, so a future `P3` port does
/// not need this bracket trick at all — it needs [`TRIGGER_ERROR_CODES`]'s
/// own code strings moved to the `ERRCODE` values the Postgres
/// triggers raise, which is exactly why the SQLite-only names chosen
/// here (`E_SUB_ISSUE_DEPTH`, not a number) carry no SQLite-specific
/// meaning of their own.
///
/// **Compiled regardless of which backend feature is active.** This
/// function is plain string parsing — it needs no SQLite-specific
/// crate feature of its own — so it stays unconditional rather than
/// gated, which is what keeps every caller in `issues.rs` from
/// needing a `#[cfg]` of its own. Under `postgres`, it simply never
/// runs, since [`crate::pool::connect`]'s own postgres branch never
/// reaches a point where a trigger could fire.
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
#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
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

#[cfg(test)]
mod tests;
