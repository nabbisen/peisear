# RFC 0018: The PostgreSQL backend — and the lock defect found while scoping it

**Status**: **Proposed** (2026-10-11) — the owner asked for the RFC and a
schedule; **one decision is theirs** (§2), and **§3 is a defect in the
shipped product that is not about PostgreSQL at all**
**Target**: prerequisites at **0.50.0/0.51.0**; the backend across
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

## §2 — the decision that is the owner's: *why* PostgreSQL

**This is the one thing I cannot settle, because the answer changes whether a
shipped requirement has to be amended.**

`ROADMAP.md:580` justifies the backend as *"multi-node, multi-user-at-scale
deployments."* But `NFR-PERF-001` states the target scale is **5–30 users on
a single node with SQLite**, and **"Horizontal scaling is out of scope"**;
`requirements.md §1.4` names **large-enterprise resource management** as a
use the product deliberately does not serve.

**So the roadmap's stated reason contradicts a shipped requirement.** Two
honest ways forward:

- **(a) Operator preference — recommended.** The reason is that an operator
  who already runs PostgreSQL wants one database to back up, monitor and
  replicate, rather than a second storage technology beside it. **This
  requires no requirement change**: it is a deployment choice at the same
  scale, and `NFR-CMP-002` (self-hostable, no specific provider) already
  points this way.
- **(b) Scale.** Then `NFR-PERF-001` is amended first, deliberately, with the
  new target stated — and `§1.4`'s non-intended users revisited, because
  "multi-user-at-scale" is adjacent to the use the product refuses.

**I recommend (a)**, and the whole schedule below assumes it. Under (a),
**SQLite stays the default and loses nothing** — which is also the only
version of this work that does not quietly change what peisear is.

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

## §8 — decisions required

1. **§2: the reason.** Operator preference (recommended, no requirement
   change) or scale (amend `NFR-PERF-001` first).
2. **Whether P0 and P1 land at 0.50.0 or 0.51.0** — 0.50.0 is already
   carrying `DOCS-006`, `RFC 0016`'s repositioning, the sprint-plan fixture,
   the `grow()` alignment defect and `§10.36`.
