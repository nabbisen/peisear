# Handoffs — the PostgreSQL schema

**0.55.0.** `RFC 018` **P3**, plus one round the RFC's phase table has
no home for (`PG-004`, §below). One subject: **a second backend that
can actually run** — schema, triggers and queries — with the
conformance suite still ahead at P4.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| PG-001 | [the connection](./PG-001-the-connection.md) | `connect`/`migrate` replacing `DIA-002`'s `unimplemented!()` stubs, pool options chosen **against a real server**, and a PostgreSQL service in CI — **nothing else in this series can be tested without it** | — | **Closed** — 3 rounds |
| PG-002 | [the schema](./PG-002-the-schema.md) | 22 migrations ported; the `migrate!` embed made dialect-aware; a drift guard between the two sets proposed. **`0021`'s millisecond token is the dangerous one** — `NFR-CONC-001`/`-002`'s subject, measured down from one second at 0.50.0 | PG-001 | **Next** |
| PG-003 | [the triggers](./PG-003-the-triggers.md) | 11 triggers, `RFC 018` §5's *"the real work"*. The five codes carried as `ERRCODE` rather than `DIA-001`'s bracket; a PostgreSQL `translate_trigger_error` sibling; **the five validations rejected identically on both backends, asserted on `MessageKey`** | PG-002 | After PG-002 |
| PG-004 | [the queries](./PG-004-the-queries.md) | **295 `?N` placeholders and the SQLite date functions.** Not in the RFC's phase table — see below | PG-003 | Last |
| REL-0.55.0 | the release candidate | The cut, `DEC-028`, `DEC-056`. Written when the four rounds close | all four | |

**Status lives in the table above, not in a message.**

---

## `PG-004` is a gap in the RFC's own plan, found at scoping

`RFC 018` §6 gives **P3** as *"migrations ported, 11 triggers
re-implemented, pool options"* and **P4** as *"the conformance suite
… against both backends."*

**Neither contains the queries.** Measured 2026-10-18: **295 `?N`
placeholder occurrences** across 20 files, plus 8 bare `?`.
PostgreSQL uses `$1`. So P3's own gate — *"the migration suite applies
cleanly on both"* — **is satisfiable with every query still broken**,
and P4's gate assumes they already work.

`DIA-002` named abstracting the placeholder style as its own stop
condition, for exactly this size. **Scoping it deliberately here is
cheaper than P4 discovering it**, which is the shape `0.53.0`'s leg 3
is the standing example of.

**The RFC is not amended.** It states phase order and gates;
`ROADMAP.md` states which release a phase lands in, and that is where
this round is recorded.

## What this series is not

**Not the conformance suite.** P4 runs the whole storage and lock
suite against both backends in CI and restates `NFR-CMP-003`. This
series proves five validations and the queries it touches — **a
narrower claim, deliberately.**

**Not a change to SQLite's path.** The default build, its tests, its
timings and its migration set are untouched. `0015`/`0016` are
byte-frozen and shipped. **A round that finds itself editing the
SQLite set has left its scope** — stop and report.

**Not open to a fifth round.** Anything else found goes to the
architect and waits, with the standing exception: a defect that
**blocks the release** is not a scope question.

---

## What the last two releases left for whoever reads this first

- **`cargo publish --workspace --dry-run` is a CI job** (`FIV-007`).
  A second migration directory changes what `cargo package` carries,
  which is precisely what that gate caught at `0.53.0`'s leg 3.
- **A release that adds a file holding a secret adds its `.gitignore`
  rule in the same change** (`§10.45`, 0.54.0).
- **Measure; do not carry.** `RFC 018` said three call sites where
  there were five, and 137 queries where there were 142. Every figure
  in these four handoffs was measured on 2026-10-18 and will drift.
