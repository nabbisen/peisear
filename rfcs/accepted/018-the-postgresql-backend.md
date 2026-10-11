# RFC 0018: The PostgreSQL backend — and the lock defect found while scoping it

**Status**: **Accepted** (2026-10-11) — implementation may begin at P0
**Target**: **P0 at 0.50.0**, **P1 at 0.51.0**, the backend across
**0.52.0 – 0.54.0**, after `RFC 0017`
**Raised by**: the owner, 2026-10-11 — *"Is there any actual plan or schedule
to extend peisear-storage for postgresql?"* then *"Make the schedule and
RFC."*
**Answers**: `ROADMAP.md:569-600`, which names two shapes and then says the
work *"needs its own RFC before it is scheduled or promised."* **This is that
RFC.**
**Related requirements**: `NFR-CONC-001`/`-002`/`-003` (P0 optimistic lock),
`FR-SUB-001..003` (database-layer enforcement), `NFR-CMP-003` (single-file
database), `NFR-PERF-001` (target scale), `§11` (RDBMS backend, Phase 3)
**Last updated**: 2026-10-11

## Summary

**The type boundary is ready. The contracts underneath it are not, and one of
them is already broken on SQLite.**

Measured today:

| Ready | Not ready |
|---|---|
| `pub type Pool = sqlx::SqlitePool` — **one alias**, `peisear-storage/src/lib.rs:33` | **11 `CREATE TRIGGER`** statements across four migrations |
| **Zero** SQLite references in `peisear-core`, `-auth`, `-i18n`, `-notify` sources | **8 `RAISE(ABORT)`** messages, string-matched by `translate_trigger_error` at three call sites |
| Two in `peisear-web`, both the default DSN string | `CURRENT_TIMESTAMP` is **whole-second**, and it is the optimistic-lock token |
| **Zero** compile-time `sqlx` macros — all **137** queries are runtime `query_as::<_, Row>` | Migrations are SQLite dialect throughout |

**§3 is the finding that matters most, and it is independent of this RFC's
subject**: the lock token does not advance when two writes land in the same
second. I verified it in SQLite directly. It should be scheduled whether or
not PostgreSQL ever ships.

## §1 — the shape: a feature flag, not a sibling crate

`ROADMAP.md:571-576` offers both. **Recommend the feature flag on
`peisear-storage`**, swapping the `Pool` alias.

**Why not the sibling crate.** `peisear-storage-postgres` would carry
*"identical query function signatures"* — **a second copy of 137 function
signatures to keep in step**, with nothing but discipline holding them
together. This project has ruled against that shape twice already this
month: the documentation screenshots reused the gate's fixture rather than
building *"a second copy of that logic to keep in step"*, and the `grow()`
helper exists so 136 call sites compose one name instead of repeating a pair.

**With a feature flag there is one set of signatures** and the divergence is
confined to where it genuinely exists: SQL text, the trigger layer, and the
pool options.

**Two guards the flag needs**, because cargo features are additive and a
dependency can turn one on behind your back:

- `compile_error!` when both backends are enabled, and when neither is.
- The dialect-specific SQL lives in **one module per backend**, not in
  `#[cfg]` arms scattered through 137 call sites. If a query cannot be
  written once for both, it moves there; if the module grows past a page,
  that is the signal the sibling-crate shape was right after all.

## §2 — the reason, settled by the owner

**Owner, 2026-10-11, superseding this section's original recommendation:**
*"For server stability and security, first. For extensibility, secondly."*

Not the "operator preference" this RFC first proposed, and not scale.
**Recorded as given, and assessed honestly below** — because the reason
decides what the documentation will claim, and a claim that does not hold is
the defect this project keeps finding.

**`docs/operations/scaling.md:84-88` already says two thirds of this.** Its
*"When SQLite isn't enough"* list names HA, size, and *"point-in-time
recovery, replication, or a managed backup story that SQLite can't satisfy"*.
**The owner's first reason is the project's own documented line**, which is a
good sign for it.

### Stability — what PostgreSQL genuinely buys

- **Point-in-time recovery and replication.** SQLite has neither, at any
  configuration. This is the strongest item and it is not reachable any
  other way.
- **Failure isolation.** The database is a separate process with its own
  lifecycle: the application can crash, be upgraded, or be replaced without
  the data file being in the blast radius, and it can be backed up and
  restored without coordinating with the application at all.
- **Concurrent writers.** SQLite admits one writer at a time; the pool holds
  **8 connections** with a **5-second `busy_timeout`**
  (`peisear-storage/src/pool.rs:42-46`), so sustained write contention
  surfaces as a timeout rather than a queue. MVCC removes the mode.

### Stability — what it does **not** buy, and what is available sooner

- **Crash safety is already there.** WAL plus `foreign_keys` is set
  (`pool.rs:28-30`); an application crash does not corrupt the file.
- **Durability on power loss is one line, today.** `synchronous = NORMAL`
  with WAL is safe against an application crash but **can lose recently
  committed transactions on a power cut or OS crash**. `FULL` removes that,
  at a write-throughput cost nobody here has measured. **This is the highest
  stability-per-effort change available and it needs no backend** — scheduled
  as **P0b** below, so the easy half of the owner's first reason is not
  waiting three releases behind the hard half.

### Security — the honest reading, because it runs the other way

**For a single-node self-hosted install, SQLite is the stronger posture, not
the weaker one.** It has no network surface, no credentials, and no second
daemon to patch; access is filesystem permissions. **PostgreSQL adds all
three.**

What PostgreSQL does buy is **separation**: the data stops being a file the
application's own user can copy wholesale, access becomes role-based and
auditable, transport can be TLS, and encryption at rest becomes the
platform's problem rather than the filesystem's.

**So security is a reason to choose PostgreSQL where a hardened database
platform already exists, and not a reason to prefer it on one box.** I have
stated the concern once; the decision stands as the owner made it, and
nothing in the plan changes because of it. **What does change is the
claim**: `scaling.md` and the eventual backend documentation must say
*separation and recoverability*, never *"more secure"*, which for the default
deployment would be false.

### Extensibility — uncontested

Full-text search without `LIKE '%…%'`, `LISTEN`/`NOTIFY` for the notification
dispatcher instead of in-process polling, richer types, and read replicas.
This is also the `§11` *RDBMS backend* row the requirements already carry.

### What this reason does **not** change

**No requirement is amended.** Stability, security and extensibility are not
scale claims, so `NFR-PERF-001` keeps its 5–30 users on a single node and its
exclusion of horizontal scaling, and `§1.4`'s non-intended users stand.
**SQLite stays the default and loses nothing** — a condition of this
acceptance, enforced at every phase gate below.

### One correction this reason exposes

`scaling.md:91-94` tells an operator that the migrations hold *"no
SQLite-specific tricks beyond a few `julianday` calls and `datetime('now',
'-7 days')` modifier strings, both of which have direct PostgreSQL
equivalents."*

**Measured: 24 `julianday` uses and 10 `datetime('now'` modifiers — and
`§5`'s 11 triggers and 8 `RAISE(ABORT)` messages, which the sentence excludes
and which have no direct equivalent at all.** The date arithmetic does
translate; the triggers are a rewrite in PL/pgSQL and the error text matching
does not survive in any form. The same paragraph also points at
`ROADMAP.md`'s *"Pluggable backends beyond relational"* (line 792) when the
section it means is *"PostgreSQL backend"* (line 569) — a different subject.

**Recorded for 0.50.0** in
`rfcs/handoffs/042-documentation-that-drifted/README.md`: an operator reading
`scaling.md` today is told this port is smaller than it is.

## §3 — found while scoping: the lock token does not always advance

**Not a PostgreSQL problem. A SQLite one, today.**

`NFR-CONC-002` reuses `updated_at` as the lock token rather than adding a
version column. `NFR-CONC-003` makes triggers its single authority. The
trigger (`0017_updated_at_single_authority.sql`) sets
`updated_at = CURRENT_TIMESTAMP`, and **SQLite's `CURRENT_TIMESTAMP` is
whole-second** — `YYYY-MM-DD HH:MM:SS`, no fraction.

**Verified directly**, with the trigger shape copied from that migration:

```
after insert    | 2026-10-11 05:29:06
after update A  | 2026-10-11 05:29:06
after update B  | 2026-10-11 05:29:06
```

Three mutations, **one token**. `check_optimistic_lock`
(`peisear-web/src/error.rs:306`) compares parsed timestamps for equality, so
a client holding the token from before update A **compares equal after it**
and its write proceeds. That is the lost update `NFR-CONC-001` (P0) exists to
prevent.

**Bounded, honestly**: I verified the mechanism in SQLite with the real
trigger shape. **I did not demonstrate an end-to-end lost update through the
handlers**, and until someone does, this is a proven mechanism and a probable
defect, not a confirmed one. The window is one whole second on the same row —
narrow, but reachable by a double submit, by the direct-manipulation
surfaces, and by two people acting at once.

**Proposed fix, keeping `NFR-CONC-002`**: store milliseconds —
`strftime('%Y-%m-%d %H:%M:%f','now')` in the triggers and column defaults.
The window narrows by ~1000×, no version column is introduced, and the
requirement's own rationale survives. PostgreSQL gives microseconds for free,
so the backend work would have masked this rather than fixed it — **which is
why it is scheduled first and separately.**

## §4 — the prerequisite that pays for itself: coded errors

`translate_trigger_error` string-matches SQLite `RAISE(ABORT, '…')` **message
text** at three call sites in `issues.rs`. It does not survive a port in any
form, and it is brittle today — a reworded message is a silently broken
validation path.

**Replace prose matching with a small set of application error codes**, raised
by both backends (SQLite encodes the code in the `RAISE` message; PostgreSQL
uses `RAISE EXCEPTION … ERRCODE`), and translate on the code.

**Worth doing even if PostgreSQL never ships**, which is the test this RFC
applies to every prerequisite.

## §5 — the triggers, which are the real work

Eleven triggers carry two requirement families:

| Migration | Triggers | Carries |
|---|---|---|
| `0014`, `0017` | 7 | `NFR-CONC-003` — `updated_at` has exactly one authority |
| `0015` | 2 | `FR-SUB-001..003` — one level, same project, no self-parent, **enforced at the database layer, not only in application code** |
| `0016` | 2 | planned-date validation |

`FR-SUB-001` is explicit that application-level enforcement is **not**
sufficient, so the Rust-side alternative is not available: **the triggers
must be ported, not replaced.** Two implementations of one invariant, on two
engines.

**The only way to know they agree is to run the same tests against both.** So
the backend work brings a standing cost, stated now rather than discovered:
**CI runs the storage-touching suite twice**, against SQLite and against a
PostgreSQL service container. That is the continuous cost the owner asks to
be told about, and it is the honest price of a second backend.

## §6 — the schedule

| Phase | Release | Content | Gate |
|---|---|---|---|
| **P0** | **0.50.0 or 0.51.0** | **§3**: demonstrate the same-second lost update end-to-end, then move the triggers and defaults to millisecond precision | The demonstration fails before the fix and passes after |
| **P0b** | **0.50.0** | **§2**: measure the write cost of `synchronous = FULL` and decide it on the number — the durability half of the owner's first reason, reachable without a backend | A measured figure on both settings, and the choice recorded with it |
| **P1** | with P0 | **§4**: coded errors replacing prose matching | Existing validation tests green, no message text in any match arm |
| **P2** | **0.52.0** | The dialect seam: feature flag, `compile_error!` guards, dialect module. **No PostgreSQL code yet** | Every existing test green on SQLite; the seam compiles with the flag absent |
| **P3** | **0.53.0** | The schema on PostgreSQL: migrations ported, 11 triggers re-implemented, pool options | The migration suite applies cleanly on both |
| **P4** | **0.54.0** | The conformance suite: the same storage and lock tests against both backends in CI; the operations story (backup is no longer `cp` on PostgreSQL); documentation | Both backends green on one suite; `NFR-CMP-003` restated to say *for the SQLite backend* |

**Sequenced after `RFC 0017`** (the first five minutes), deliberately:
PostgreSQL serves the operator who already has one, and `RFC 0017` serves the
person who has nothing. The owner has just said the second is the problem.

**P0 and P1 do not depend on this RFC being accepted.** If the backend is
never built, both are still improvements to what ships — which is the test
each prerequisite had to pass to be listed here.

## §7 — and one line in `README.md`

`README.md:77` tells a prospective adopter that *"swapping SQLite for Postgres
… is a localised change."* **It is localised in types and not in contracts**,
and the reader meets the README, not `ROADMAP.md:582`'s own correction of the
same claim. Already recorded for `RFC 0016`'s repositioning round
(`rfcs/handoffs/042-documentation-that-drifted/README.md`).

## §8 — decisions taken

1. **The reason is server stability and security first, extensibility
   second** (owner, 2026-10-11, superseding this RFC's own "operator
   preference" recommendation). **No requirement is amended** — none of the
   three is a scale claim. **SQLite stays the default and loses nothing**,
   a condition of this acceptance enforced at every phase gate.
   **§2 records which parts of the first reason PostgreSQL delivers, which
   are available sooner without it (P0b), and the one part that runs the
   other way** — on a single box SQLite's posture is the stronger, so the
   documentation must claim separation and recoverability, never "more
   secure".
2. **P0 lands at 0.50.0; P1 at 0.51.0** (architect, recorded here rather than
   returned to the owner, since the RFC left the choice open between them).
   **P0 is a correctness defect against a P0 requirement**, so it does not
   wait behind a documentation release; it is also small — a migration
   rewriting eleven triggers and the column defaults, plus the demonstration
   that must fail first. **P1 is a refactor with no live defect behind it**
   and is P2's own prerequisite, so it sits with `RFC 0017`'s round at 0.51.0
   and still lands a release ahead of the seam.

**One implementation hazard, named so it is measured and not discovered**:
P0 changes the stored format from `YYYY-MM-DD HH:MM:SS` to one carrying
milliseconds. **Check how the row decoding handles the fractional part before
writing the migration** — `sqlx`'s SQLite `DATETIME` decoding accepts several
shapes, and existing rows keep the old one until their next write, so both
must parse. A demonstration that passes on fresh rows and fails on
pre-migration rows is the failure mode to rule out.
