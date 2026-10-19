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

## §6b — the two placeholder migrations must not ship

*Added at `PG-002`'s review.* `migrations-postgres/0021_…` and
`0022_…` exist for version-number parity and are **comment-only —
zero executable statements**, measured. `PG-002` says so plainly,
which is why this is a note and not a finding against it.

**They are a checksum trap the moment anyone runs them.** `sqlx`
stores each applied migration's checksum and refuses to run when one
changes (`MigrateError::VersionMismatch`) — the defect `DIA-001` round
2 was reversed for, demonstrated then against a real 0.53.0 database.
**Filling these two files in this round changes their checksums.**

**Safe today, and only today**: no PostgreSQL database exists outside
CI's own ephemeral service, so there is nothing to break. **That stops
being true the moment 0.55.0 ships.**

So: **this round fills them, and the release must not cut while either
is still a placeholder.** Stated here as a gate rather than left to
the ordering holding — if `PG-003` were to slip past the cut, 0.55.0
would publish two migrations that a later release then modifies under
anyone who ran them.

**If you fill them and the version numbers no longer make sense** —
if PostgreSQL's trigger content does not divide the way SQLite's did
between `0021` and `0022` — **say so rather than forcing content into
a file to keep a number.** The drift guard compares filenames, so the
numbers must stay; what goes in them is yours.

## §7 — exit condition

**Neither `migrations-postgres/0021_…` nor `0022_…` left as a
comment-only placeholder** (`§6b`); eleven triggers on PostgreSQL; the
five codes carried as `ERRCODE`
rather than a bracket; a PostgreSQL `translate_trigger_error` sibling
reading the code out of band; the both-directions guard working on
both catalogues and seen to fail both ways on each; **the five
validations rejected identically on both backends, asserted on
`MessageKey`**; `FR-SUB-001..003` and `NFR-CONC-003` checked and
reported; all gates re-run.

---

# PG-003 round 2 — the features are exclusive; the facts are not

Round 1 is accepted (see the review). The eleven triggers are real and
measured, `ERRCODE` carries the five codes properly, both plants fire
on both catalogues, and the placeholders are gone. **One constraint
was accepted that did not have to be.**

## §8 — "no test can reference both `const`s" is true, and the wrong conclusion

`dialect.rs:145` says it plainly, which is why this is a finding
against the design and not against the honesty:

> *"so no test can reference both `const`s at once to compare them.
> Kept in sync by hand instead — both lists are five entries long,
> both ordered … so a reviewer can diff them by eye at either file.
> If a sixth validation ever needs a code, add it to both in the same
> change."*

**The premise is correct**: `TRIGGER_ERROR_CODES` is
`#[cfg(feature = "sqlite")]`, `TRIGGER_ERROR_CODES_POSTGRES` is
`#[cfg(all(feature = "postgres", not(feature = "sqlite")))]`, and the
`compile_error!` guards make the two features mutually exclusive, so
no compilation sees both.

**The conclusion does not follow.** The *features* are exclusive; the
*fact they both encode* is not. **"These five validations exist, in
this order, mapping to these five `MessageKey`s" is one fact, and it
is currently written twice.**

**What that costs.** Add a sixth validation to SQLite only, and:

- SQLite's guard passes — its registry and `sqlite_master` agree;
- PostgreSQL's guard passes — its registry and `pg_trigger` agree;
- the five-validations test covers the five that exist on both;
- **nothing fails**, and the two backends silently enforce different
  rules.

`RFC 018` §5's whole premise is *"the only way to know they agree is
to run the same tests against both."* Each half is guarded against
its own engine. **The halves are not guarded against each other.**

## §9 — one list, two vocabularies

**Hoist the shared fact into an unconditional `const`** — the ordered
list of the five `MessageKey`s, with no `cfg` on it at all, since a
`MessageKey` is backend-agnostic and `peisear-i18n` is a dependency
either way.

Then each backend's table is checked against **that one list**, under
its own feature:

- the SQLite build asserts `TRIGGER_ERROR_CODES`'s keys equal it, in
  order;
- the PostgreSQL build asserts the same of `TRIGGER_ERROR_CODES_POSTGRES`.

**Neither test needs to see the other's `const`**, which is what made
this look impossible — and a sixth validation added to one side now
fails the other side's build the next time it runs.

**Keep both code vocabularies exactly as they are.** `E_SUB_ISSUE_DEPTH`
and `PE001` are genuinely different things — a bracket in a message
and an out-of-band `SQLSTATE` — and `DIA-002`'s own lesson is not to
abstract a difference that is real. **The codes stay two; the list of
validations becomes one.**

**Delete the "diff them by eye" paragraph** when the guard lands; a
comment telling a reader to do by hand what a test now does is worse
than no comment.

## §10 — gates

- `DEC-007`, feature absent — **moves** by the SQLite-side assertion.
  Account for it.
- **The new assertion seen to fail**, on at least one side: plant a
  sixth entry in one table without touching the shared list, watch it
  fail, restore. A guard never seen to fail is `§10.36`.
- `fmt`, `clippy` both ways, `rustdoc` **under both features** (this
  round already adopted that; keep it), `mdbook`, `publish-dry-run`,
  the CI PostgreSQL job.

## §11 — exit condition

One unconditional list of the five validations' `MessageKey`s; each
backend's table asserted against it under its own feature, with
neither test referencing the other's `const`; both code vocabularies
unchanged; the "diff by eye" paragraph removed; the assertion shown to
fail against a planted sixth entry; `DEC-007`'s delta accounted for;
all gates re-run.
