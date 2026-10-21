# CON-006 — the integer that is four bytes

**0.56.0, `RFC 018` P4, inserted after `CON-005` and before
`CON-003`.** The second defect found by the same mechanism as the
first: a layer that could not be reached until the layer in front of
it was fixed. `CON-005` round 2 found it, escalated it, and was right
to — it is the schema, not the transaction seam.

**`CON-003` is blocked on this**, for a reason sharper than the one
`CON-005` blocked it with: **no issue can be read on PostgreSQL.**

---

## §1 — the measured failure, and what it actually covers

`CON-005` round 2, the first test ever to round-trip
`user_capacities.points` against a real PostgreSQL server:

```
ColumnDecode { index: "points", source: "mismatched types; Rust type
i64 (as SQL type INT8) is not compatible with SQL type INT4" }
```

`migrations-postgres/0009_user_capacities.sql:27` declares `points
INTEGER`. In SQLite `INTEGER` is a type *affinity* of no fixed width
and stores any integer; in PostgreSQL it is a fixed 4-byte `int4`.
The port was line-for-line, so the word carried over and the meaning
did not.

**The round named 29 other declarations as a worklist and said
plainly it had not checked them. Checked now:**

| Column(s) | Declared | Rust decodes as | Status |
|---|---|---|---|
| `user_capacities.points` | `INTEGER` (`0009:27`) | `i64` | **measured failure** |
| `issues.effort` | `INTEGER` (`0002:6`) | `IssueRow.effort: Option<i64>` (`issues.rs:16`) | **confirmed by reading** |
| `metrics_snapshots` — all 10 | `INTEGER` (`0007:9–19`) | `i64` ×10 (`metrics_snapshots.rs:37–46`) | **confirmed by reading** |
| `user_metrics_snapshots` — 4 of 7 | `INTEGER` (`0008:13–18`) | `i64`/`Option<i64>` (`user_metrics_snapshots.rs:27–31`, `:47–51`) | **confirmed by reading** |
| `sprint_records.committed_points` | `INTEGER` (`0019:34`) | `i64` (`peisear-core/src/lib.rs:2432`) | **confirmed by reading** |
| `sprint_records`' other 3, the burndown series' 2, `0020`'s 2, `personal_limits`' 2, `0001.position`, the two 0/1 flags | `INTEGER` | **not checked by me** | worklist |
| `users.capacity_points` (`0004:8`) | `INTEGER` | — | **dropped** by `0009:50`; not a live column |

**Census, counted twice**: 31 lines in `migrations-postgres/*.sql`
mention `INTEGER`; 1 is a comment; **30 are column declarations**; 29
are other than `points`; **28 of those are live.** The round's own
"29 other declarations across 8 migration files" is right. The "27
further candidates" in its closing recommendation is a slip — the
same number stated twice, differently, which is why this handoff
recounts rather than quotes.

**`issues.effort` is the one that matters most.** It is selected by
`find_by_id` and every issue list, so **every issue read on
PostgreSQL fails to decode.** Not a metric, not a report — the
product's most-read table. This is also why no amount of work on
`CON-003` could have succeeded.

**The pattern is uniform, not scattered**: the Rust side uses `i64`
everywhere, because that is SQLite's natural integer width; the port
wrote `INTEGER` everywhere. So the expected number of exceptions is
small, and a column-by-column worklist is the wrong shape of fix.

## §2 — fix the invariant, not the list

**Every integer column in the PostgreSQL schema becomes `BIGINT`.**
Not "every column the audit found" — every one. The Rust side is
uniformly `i64`; a schema that is uniformly 8-byte integers cannot
disagree with it, and there is no list to keep correct.

**Then assert it mechanically**, against the live schema in the
conformance suite:

> no column in the PostgreSQL schema has data type `integer`

One predicate over `information_schema.columns`, no per-column
allow-list — `§10.39`'s lesson, and the same shape as `PG-003`'s
live-trigger check, which reads the server rather than the migration
files. **The next mechanical port cannot reintroduce this**, which
is the whole point; fixing 28 columns by hand and moving on would
leave the mechanism that produced them intact.

If some column genuinely must be 4-byte, the predicate gets a named
exception in the same style `migration_drift` already uses for
`0023_rowid_surrogate_for_ordering.sql` — named, with its reason, in
the test. **Do not start from an exception list.**

## §3 — edit the migrations in place, and why that is allowed here

This is the architect's decision, stated so it can be overruled
rather than discovered.

The append-only rule exists for one measured reason: `sqlx` records
each applied migration's checksum and refuses to start when one
changes, so amending a shipped migration breaks every existing
install (demonstrated 2026-10-17 on `0015`/`0016`).

**That reason cannot apply to `migrations-postgres/`.** The only way
to obtain a PostgreSQL database from this project is to build with
`--features postgres`, and until `CON-001` landed yesterday no such
build existed; no release documents an install path, no archive and
no container image carries the feature, and `0.56.0` deliberately
ships none. The only PostgreSQL databases in existence are CI's
service containers, recreated per run. **There is no database whose
checksum could break.**

So: **amend `0002`, `0007`, `0008`, `0009`, `0019`, `0020` and
`0001` in place**, so each file is correct as written. The
alternative — a postgres-only `0024` of `ALTER COLUMN … TYPE BIGINT`
— would need a second named exception in `migration_drift`, and
would leave every reader of `0007` seeing `INTEGER` and needing to
know about a later file. **Applying a rule where its reason holds,
and not as a ritual, is `§10.43`'s own lesson.**

**Write the expiry into the amendment**: the moment a PostgreSQL
install path is documented — `ROADMAP.md`'s P5 — `migrations-postgres/`
becomes append-only on the same terms as `migrations/`. Say it in the
round's own commit and in `docs/development/`'s migration guidance,
so the next person does not read this round as a precedent for
`migrations/`.

**`migrations/` (SQLite) is not touched.** `INTEGER` is correct
there, the files have shipped to real installs, and a round that
finds itself editing them has left its scope.

## §4 — gates

- **The predicate in §2, green against a live server**, and **seen
  to fail**: run it once against the schema before the amendment and
  show it red. A check never seen to fail is not evidence, and this
  one is the round's whole deliverable.
- `migration_drift` green **with no new exception** — if the fix
  needs one, that is a finding, not a detail.
- The unfiltered `postgres` `--lib` suite: **50 compile, 50 pass, 0
  ignored.** `CON-005`'s remaining `#[ignore]` is this round's to
  delete, and the five `optimistic_lock_guards` tests must be green
  on both backends.
- **A read, not just a decode**: `issues::find_by_id` exercised
  against PostgreSQL and seen to return an issue. That is the
  failure this round exists to close and nothing currently tests it.
- `DEC-007` — the SQLite side must not move. 515.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`,
  `publish-dry-run`; `test.yml`'s own gate if touched.

## §5 — escalate rather than deciding

- **If a column cannot be `BIGINT`** — a `CHECK`, an index, a
  foreign key that requires the narrower type. Bring it with the
  reason; that is the named exception §2 allows, and it must be
  argued, not assumed.
- **If the 0/1 flags should become `BOOLEAN` on PostgreSQL.** They
  are `INTEGER CHECK (x IN (0,1))` by the same line-for-line port.
  `BIGINT` makes them work; `BOOLEAN` would make them *right*, and
  would change what Rust decodes. **Raise it, do not take it** —
  it is a different subject from this round's.
- **If anything about `migrations/` needs to change.** Stop.
- **If the predicate finds integer columns this handoff did not
  name.** Report the full list; it means the census above was also
  bounded, and the predicate is doing its job.

## §6 — exit condition

Every integer column in `migrations-postgres/` `BIGINT`, by in-place
amendment of the files named in §3 with the expiry recorded; the
no-`integer`-columns predicate in the conformance suite, **seen red
before the fix and green after**; `migration_drift` green with no new
exception; `issues::find_by_id` seen to return an issue against
PostgreSQL; the unfiltered `postgres` suite at 50 passed, 0 ignored,
with `CON-005`'s `#[ignore]` deleted; the 0/1-flag question raised
and not taken; `migrations/` untouched; `DEC-007` at 515; all gates
re-run after the last edit.
