# PG-004 — the queries

**Target**: 0.55.0, last. **Not in `RFC 018`'s phase table** — see
`§1`. This is the round that makes the backend able to execute
anything.

---

## §1 — the gap in the RFC's own plan, named at scoping

`RFC 018` §6 gives P3 as *"migrations ported, 11 triggers
re-implemented, pool options"* and P4 as *"the conformance suite …
against both backends."*

**Neither contains the queries**, and the queries do not work:

| fact | measured 2026-10-18 |
|---|---|
| `?N` placeholder occurrences in `peisear-storage/src/` | **295** |
| bare `?` placeholders | 8 |
| files carrying them | 20 |

PostgreSQL uses `$1`, not `?1`. **P3's gate (*"the migration suite
applies cleanly on both"*) is satisfiable with every query still
broken**, and P4's gate assumes they work. The 295 fall between the
two phases, and `DIA-002` named abstracting them as its own stop
condition precisely because of the size.

**This round is that gap, scoped deliberately rather than discovered
by P4.** The RFC is not amended — `ROADMAP.md` carries which release
a phase lands in, and this round is recorded there.

## §2 — the shape, and the one decision that matters

**Do not hand-edit 295 placeholders.** A mechanical, testable
translation at the dialect boundary is the obvious candidate: the
queries keep `?N` as written, and the PostgreSQL path rewrites `?N` →
`$N` on the way to `sqlx::query`.

- That makes the call-site change **one token per query**
  (`sqlx::query(` → `dialect::query(`), which is mechanical and
  greppable, not 295 judgement calls.
- **The rewrite is a parser, and a parser has edges**: a `?` inside a
  string literal, inside a comment, a `??`, a cast like `::jsonb`.
  **Enumerate the edges, test each, and say which you decided are not
  reachable in this codebase** — with the grep that shows it.
- **Prefer rejecting what you cannot parse** over passing it through.
  A query that silently keeps a `?` on PostgreSQL fails at runtime
  with a parse error pointing at SQL nobody wrote.

**If you find a better shape than a rewrite, bring it.** This is a
recommendation with a reason, not a design handed down — and
`DIA-002`'s unabstracted list exists precisely so P3's rounds could
choose differently once they had a second implementation in front of
them.

## §3 — what else differs, beyond placeholders

`PG-002` ports the migrations' SQLite constructs. **The same
constructs appear in query strings**, and those are this round's:

| construct | files under `src/` |
|---|---|
| `datetime(` / `strftime` | `metrics_snapshots.rs`, `user_metrics_snapshots.rs`, `user_burnout.rs`, `project_health.rs`, `personal_metrics.rs` |
| `ON CONFLICT` | **5 occurrences, 3 files** — `notifications.rs`, `sprints.rs`, `view_states.rs` |
| `INSERT OR REPLACE` | **1**, `sprints.rs:406` |

*Re-measured at `PG-002`'s review, because both this table and
`PG-002`'s were built from one command that searched migrations and
`src/` together.* **`PG-002`'s report concluded these constructs were
a false positive from prose in `0009`. They are not** — the prose
match is real but separate, and the SQL below is real too:

- **`ON CONFLICT … DO UPDATE` is valid in both engines** (SQLite since
  3.24), so those five are likely to port unchanged. **Check, do not
  assume** — the column-list spelling differs between the two call
  styles already in use here (`ON CONFLICT(col)` and
  `ON CONFLICT (col)`).
- **`INSERT OR REPLACE` is SQLite-only and has no PostgreSQL
  equivalent.** `sprints.rs:406`'s `INSERT OR REPLACE INTO
  sprint_records` must become an explicit `INSERT … ON CONFLICT …
  DO UPDATE`, and **the two are not equivalent**: `INSERT OR REPLACE`
  deletes the conflicting row and inserts a new one, firing delete
  triggers and resetting unlisted columns to their defaults;
  `ON CONFLICT DO UPDATE` updates in place. **Say which semantics the
  record actually needs** before choosing the rewrite.

**The date functions are the hard half, not the placeholders.**
`strftime`/`datetime()` have no identical PostgreSQL spelling, and
five of the files carrying them compute **personal sustainability
metrics** — `NFR-PRIV-*`'s own subject, where a wrong window is a
wrong statement about a person, not a wrong number. **Each one gets
its equivalent justified and its output compared on both backends.**

## §4 — gates

- `DEC-007`, feature absent — unchanged. **The SQLite path must not
  move**, and 295 edited call sites is exactly how it would.
- **The same query, both backends, same result** — for every function
  touched, not a sample. If that is too many to assert individually,
  say so and propose the population you did assert, with its bound.
- The five date-function files' outputs compared on both backends.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`, `publish-dry-run`.

## §5 — escalate rather than deciding

- **If the rewrite cannot be made total** — if some query shape
  cannot be translated safely. Bring it; a known-unsupported shape
  that fails loudly is acceptable, one that passes silently is not.
- **If a date function's PostgreSQL equivalent changes a metric's
  value.** Stop. That is `NFR-PRIV-*` and a requirement question.
- **If the SQLite suite moves at all.** Stop.

## §6 — exit condition

Every production query executable on both backends; the translation
mechanical with its edge cases enumerated, tested, and the
unreachable ones shown unreachable by grep; the five date-function
files' outputs compared on both backends and any difference
escalated rather than absorbed; `DEC-007` unchanged with the feature
absent; all gates re-run.
