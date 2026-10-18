# PG-003 — the triggers

**Target**: 0.55.0, after `PG-002`. `RFC 018` §5 — *"the triggers,
which are the real work."*

**`FR-SUB-001` is explicit that application-level enforcement is not
sufficient**, so the Rust-side alternative does not exist: **the
triggers must be ported, not replaced.** Two implementations of one
invariant, on two engines.

---

## §1 — eleven triggers, two requirement families

| source | triggers | carries |
|---|---|---|
| `0014`, `0017`, `0021` | 7 | `NFR-CONC-003` — `updated_at` has exactly one authority |
| `0015` → `0022` | 2 | `FR-SUB-001..003` — one level, same project, no self-parent |
| `0016` → `0022` | 2 | planned-date validation |

**Measured 2026-10-18 against a live migrated database**: 11 triggers,
8 `RAISE` statements. Re-measure; `0022` re-created four of them and a
future migration may again.

## §2 — the error codes, which already exist

`DIA-001` put a bracketed `[E_CODE]` inside SQLite's `RAISE(ABORT,
'…')` **because SQLite has no out-of-band code**. `dialect.rs`'s own
doc comment says what PostgreSQL needs instead:

> `RAISE EXCEPTION … USING ERRCODE = '…'` carries a real out-of-band
> code, so `P3` needs these five code *strings* moved to `ERRCODE`
> values, not the bracket convention itself.

So: **the same five codes, carried properly.** `translate_trigger_error`
is SQLite's — PostgreSQL needs **its own function**, reading
`sqlx::Error`'s database error code rather than parsing a message.
`DIA-002` already moved the SQLite one into `dialect`; add the sibling
beside it.

**The guard `DIA-001` round 3 built reads live triggers from
`sqlite_master`.** PostgreSQL's catalogue is `pg_trigger`/`pg_proc`.
**Both backends need the both-directions property**, and the guard
that asserts it has to work on each. Say whether that is one function
over an abstraction or two functions with one test each — **and
prefer two honest functions to one leaky abstraction**, which is the
lesson `DIA-002`'s unabstracted list already recorded.

## §3 — the only way to know they agree

`RFC 018` §5: *"The only way to know they agree is to run the same
tests against both."*

**This round proves agreement on the five validations**, not on the
whole suite — that is `P4`. For each of the five: the same operation,
rejected on both backends, producing the same `MessageKey`. **Same
assertion, two pools.**

**A rejection for a different reason is not agreement.** Assert the
`MessageKey`, not merely that an error occurred.

## §4 — specification work (`DEC-028`)

- **`FR-SUB-001..003`** say enforcement is at the database layer.
  After this round there are two databases. **Check whether the text
  needs to say so**, and report rather than rewrite if it reads
  correctly already.
- **`NFR-CONC-003`** — one authority for `updated_at`, now on two
  engines. Same question.

## §5 — gates

- `DEC-007`, feature absent — unchanged.
- **The five validations, both backends, same `MessageKey`.** This is
  the round.
- The both-directions code guard, **running on both catalogues**, with
  a planted shrunk and a planted widened registry each shown to fail —
  `DIA-001` round 1's plantable-parameter shape still applies.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`, `publish-dry-run`.

## §6 — escalate rather than deciding

- **If a trigger cannot be ported faithfully** — if PostgreSQL's
  semantics differ in a way that changes when it fires. That is a
  requirement question.
- **If `ERRCODE` cannot carry the five codes** without inventing a
  class. Bring what you found.

## §7 — exit condition

Eleven triggers on PostgreSQL; the five codes carried as `ERRCODE`
rather than a bracket; a PostgreSQL `translate_trigger_error` sibling
reading the code out of band; the both-directions guard working on
both catalogues and seen to fail both ways on each; **the five
validations rejected identically on both backends, asserted on
`MessageKey`**; `FR-SUB-001..003` and `NFR-CONC-003` checked and
reported; all gates re-run.
