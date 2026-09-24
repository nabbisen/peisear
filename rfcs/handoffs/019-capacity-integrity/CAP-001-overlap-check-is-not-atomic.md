# CAP-001 — the capacity overlap check is not atomic, and a comment says it is

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.38.0
**Governing RFC**: none — defect fix.
**Source**: found by the dev team while measuring `ORD-002`'s premise, and
confirmed from the code in `.git-exclude/reviewed/ORD-002-review.md` §3.
**Depends on**: nothing. `ORD-002` has shipped and its `rowid` tiebreak is not
a partial fix for this — it makes the *read* deterministic and is worth having
either way.

---

## 1. The defect

`user_capacities::insert` checks, then writes:

```rust
if let Some(conflict) = overlaps_existing(pool, user_id, …).await? { return Err(Conflict) }
…
sqlx::query("INSERT INTO user_capacities …")
```

**`overlaps_existing` takes `pool`, and so does the INSERT.** They are two
statements on two pooled connections with no transaction spanning them, so two
concurrent requests can both pass the check before either writes. Measured by
the dev team: twelve simultaneous saves stored **six** overlapping open-ended
rows. `update` (`:360`) has the same shape.

**Sequentially the guard works** — the second save is refused and the handler
renders a visible conflict message. This is reachable only by concurrency: a
double-submit where both requests are in flight together.

## 2. The part that makes it worth doing now

`user_capacities.rs:35-40` says:

> There is a small window between `overlaps_existing` and the INSERT where a
> concurrent writer could land a conflicting row. **For peisear's
> single-process / WAL-serialized-write model this window is zero in practice**,
> but a future PostgreSQL backend would want a transaction-level lock or an
> exclusion constraint. Documented; not addressed today.

**The bolded claim is false.** WAL serializes write *transactions*; it does
nothing about a read on one connection followed by a write on another with no
transaction between them. The measurement falsifies it directly.

A documented safety assumption that does not hold is worse than an
undocumented gap, because it is the reason nobody looks again — and this
comment has been the reason since `0009`. Migration `0009` makes the same
division of labour explicit ("the application's overlap check ensures only one
such row"), so the schema is relying on a guarantee the application does not
provide.

**What goes wrong when it fires**: several overlapping rows exist where the
code's stated invariant is at most one. `/settings` lists them all, and the
"current capacity" pick returns one of them — deterministically since
`ORD-002`, but the number is still the resolution of a state that should not
exist.

## 3. The design decision

**Make the check and the write one atomic operation, in a transaction, with the
check re-run inside it.**

- `insert` and `update` each open a transaction, run `overlaps_existing`
  **through that transaction** rather than the pool, and write inside it.
  `overlaps_existing` therefore needs to take an executor rather than `&Pool`
  — the smallest change that makes the guarantee real, and it keeps the useful
  error (the conflicting row's id and dates) that `0009` chose this design for.
- **Keep the application-level check.** `0009` weighed a trigger against it and
  chose the application for a better message; that reasoning still holds. This
  handoff makes the existing choice correct rather than replacing it.
- **Not a `UNIQUE` index or a trigger.** Overlap is a range predicate, not an
  equality, so no index expresses it, and `0009` already records why a trigger
  was declined. If the transaction turns out not to be enough, that is an
  escalation, not a fallback to improvise.
- **SQLite needs the write lock taken at the start**, or two transactions can
  still both read before either writes and one fails at commit. Use a
  begin-immediate transaction (or whatever the sqlx version exposes for it) so
  the lock is held across the check. **Say which mechanism you used**; this is
  the whole substance of the fix and "wrapped it in a transaction" is not
  specific enough to review.

**The module comment is rewritten** to say what is true: the window existed,
it was not zero, it was measured, and it is now closed by the transaction.
Leave the old claim visible as a correction — it is the more useful half.

## 4. Verification

- **A concurrent test that fails before and passes after**, and **report both
  runs**. N simultaneous saves of overlapping open-ended periods for one user
  end with **exactly one** row stored and the rest refused. The dev team's
  twelve-at-once shape reproduces it; pick N so the before-run fails reliably
  rather than occasionally, and say what N is and how often it failed.
- The same for `update`.
- **Sequential behaviour is unchanged**: a second overlapping save is refused
  with the same visible message naming the conflicting row, its dates and its
  points. Existing tests should cover this; say which.
- Non-overlapping periods still insert — the guard must not become a lock on
  the whole table for unrelated rows.
- `ORD-002`'s three capacity tie tests still pass, unmodified.
- `DEC-007`: report the new count, last recorded **289**. A new test file means
  editing `.github/CONTRIBUTING.md`'s block in the same change — **authorised**
  for this handoff.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched.

## 5. Escalate rather than deciding

- **If a transaction is not sufficient** under the sqlx/SQLite combination in
  use — measure and report rather than adding a retry loop.
- **If the write lock measurably delays unrelated capacity writes.**
- **If the same check-then-write shape exists elsewhere.** Worth a look while
  you are here: anything that validates with a `SELECT` and then writes without
  a transaction has this defect, and this one was found by accident. Report
  what you find; do not fix it in this handoff.
- **If a user-visible string changes.** The conflict message should not.

## 6. Exit condition

Concurrent overlapping saves leave exactly one row; a test demonstrates it and
failed before the change; the module comment states what is true; sequential
behaviour and the conflict message are unchanged.
