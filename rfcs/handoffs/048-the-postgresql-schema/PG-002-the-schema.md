# PG-002 — the schema

**Target**: 0.55.0, after `PG-001`. `RFC 018` P3's *"migrations
ported"*. Gate, from the RFC: **the migration suite applies cleanly
on both.**

---

## §1 — what has to be ported, measured 2026-10-18

**22 migration files.** Re-measure; the count moves.

SQLite-specific constructs, counted by the files that contain them:

| construct | files |
|---|---|
| `datetime(` | 5 |
| `RAISE(ABORT` | 5 — **`PG-003`'s**, not this round's |
| `ON CONFLICT` | 3 |
| `strftime` | 1 |
| `INSERT OR REPLACE` | 1 |
| `INTEGER PRIMARY KEY` | 1 |

**`strftime` and `datetime(` are not cosmetic.** They are in
`0021_updated_at_millisecond_precision.sql`, which is the
optimistic-lock token's own precision — `NFR-CONC-001`/`-002`'s
subject, and the thing `0.50.0` measured a one-second window down to
one millisecond. **A PostgreSQL port that gets the token's resolution
wrong reintroduces a defect this project has already paid for once.**
Read `§10.x`'s `0.50.0` row before writing that migration.

## §2 — two sets, and how the binary picks one

`pool.rs` embeds `sqlx::migrate!("./migrations")` — **one directory,
at compile time**. PostgreSQL needs its own set, and the embed has to
become dialect-aware.

**That is this round's first design decision.** Two directories and
two `migrate!` calls behind the existing `cfg` is the obvious shape;
**say why you chose what you chose**, and in particular say what
stops the two sets drifting — twenty-two files duplicated with no
guard between them is `§10.39`'s class waiting to happen.

**A guard is wanted and its shape is yours to propose**: the same
version numbers in both sets, the same descriptions, nothing in one
that is missing from the other. **Propose it; do not build a
comparison that reads SQL semantics** — that is a different project.

## §3 — what this round is not

- **Not the triggers.** `PG-003`. Port the schema around them and say
  which migrations are left incomplete pending that round.
- **Not the queries.** `PG-004`, 295 placeholders.
- **Not a rewrite of the SQLite set.** `0015`/`0016` are byte-frozen
  (`DIA-001` round 2, and `0.53.0` shipped them). **The SQLite
  directory does not change in this round at all** — if you find
  yourself editing it, stop and report why.

## §4 — specification work (`DEC-028`)

- **`NFR-CONC-002`** carries a precision paragraph from 0.50.0. If
  PostgreSQL's token resolution differs from SQLite's, **that entry
  gains a second sentence, not a correction.** Two backends, two
  measured resolutions, both stated.
- **`NFR-CMP-003`** (single-file database) is `P4`'s to restate, per
  the RFC. Not here. Say you left it.

## §5 — gates

- `DEC-007`, feature absent — unchanged.
- **The migration suite applied cleanly on both**, which is the RFC's
  own P3 gate: a fresh SQLite database and a fresh PostgreSQL
  database, both migrating from empty, both reaching the same version
  number. **Report both.**
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`,
  **`publish-dry-run`** — a second migration directory changes what
  `cargo package` carries, which is exactly what that gate caught at
  `0.53.0`'s leg 3.
- **`cargo package -p peisear-storage --list` showing both migration
  directories**, because an embedded directory outside the package is
  the defect `§10.44` records.

## §6 — escalate rather than deciding

- **If a construct has no faithful PostgreSQL equivalent.** Bring the
  construct and the candidates; do not pick the nearest one quietly.
- **If the two sets cannot be guarded without reading SQL meaning.**
  Report the shape you reached.
- **If porting `0021` changes the lock token's resolution.** Stop.
  That is a requirement question, not a migration question.

## §7 — exit condition

A PostgreSQL migration set; the embed dialect-aware with the choice
reasoned; a drift guard between the two sets proposed, with its
bounds; **both suites applying cleanly from empty and reaching the
same version**, both reported; `0021`'s token resolution measured on
PostgreSQL and compared to SQLite's, with `NFR-CONC-002` amended if
they differ; the SQLite set untouched; `cargo package --list` showing
both directories; all gates re-run.
