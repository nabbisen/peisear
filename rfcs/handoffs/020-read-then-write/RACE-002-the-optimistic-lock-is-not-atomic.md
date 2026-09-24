# RACE-002 — the optimistic lock compares a value it read earlier

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.39.0
**Governing RFC**: none — defect fix, but it touches the product's central
concurrency mechanism, so read §3 before writing anything.
**Source**: flagged by the dev team in `CAP-001`'s review request §5 and marked
*inferred*; confirmed by reading in `.git-exclude/reviewed/CAP-001-review.md` §3.
**Depends on**: nothing. **Deliberately separate from `RACE-001`** — same shape,
different fix, and folding them together would hide that.

---

## 1. The defect

`check_optimistic_lock` (`crates/peisear-web/src/error.rs:272`) is a **pure
comparison function**. It receives `current_updated_at` — already read by the
caller — and compares it to the client's stamp. So every locking handler does:

1. read the entity, taking its `updated_at`,
2. compare it to `client_updated_at`,
3. write.

**Nothing is atomic across the three.** Two requests carrying the *same, valid*
stamp both read the same current value, both pass the comparison, and both
write. The second overwrites the first, and **the first writer is told their
save succeeded.**

That is the exact outcome the lock exists to prevent. The window is the few
milliseconds between a handler's read and its write, so this is rarer than
`CAP-001`'s race — but the feature exists for concurrent editing, its failure is
**silent data loss**, and it is on every locking surface: issue, project,
sprint, capacity, the board, the sprint plan and the calendar.

**This is not a reason to distrust the lock in general.** It catches the case it
was built for — a stale page, minutes or hours old — every time. What it does
not catch is two saves inside one request window.

## 2. The fix

**Fold the comparison into the write, and stop comparing a value read earlier.**

```sql
UPDATE issues SET … WHERE id = ?1 AND updated_at = ?2
```

with `?2` the client's stamp, and **zero affected rows meaning the conflict**.
One statement, atomic by definition, no transaction and no lock needed. This is
the canonical form of an optimistic lock; the current code is the
compare-then-write variant, which is the one with this hole.

**`0017`'s trigger is compatible and worth checking explicitly.** It fires
`AFTER UPDATE … WHEN OLD.updated_at = NEW.updated_at` and bumps the row. The
`WHERE` clause compares the *pre-update* value, so the two do not interfere —
but confirm it on a real update rather than reading the trigger, because the
whole fix rests on it.

**Zero rows must be distinguished from "row not found."** They are different
outcomes with different messages today (`OptimisticLockConflict` versus a 404),
and after this change both produce zero affected rows. Re-read the row on zero
to tell them apart — that read is now *after* the failed write, so it cannot
reintroduce the race.

**`check_optimistic_lock` does not disappear.** It still parses the client's
stamp and produces `LockValueUnreadable` for a missing, empty or malformed
value, which is a validation that belongs before any write. **Keep that half**,
and keep the entity-neutral wording `DEV-001-004` settled. What moves into SQL
is the comparison, not the parsing.

## 3. Scope, and why it is a whole handoff

Every locking write path changes shape. **Enumerate them before changing any**
— `check_optimistic_lock`'s callers, plus any storage write taking a
`client_updated_at` — and list them in the review request. The board, the sprint
plan and the calendar additionally consume the returned `updated_at` to keep
their client stamp fresh (`RFC 0004` D-1 step 2), so their responses must carry
the same value they do today.

**If one path cannot take the `WHERE` form**, say so and leave it; a partial
conversion that is honest about its boundary is better than a uniform one that
quietly does something else somewhere.

## 4. Verification

- **A concurrent test that fails before and passes after**, with the failure
  rate stated: two edits of one entity carrying the same valid stamp, released
  together → **one succeeds, one gets the conflict, and the loser's value is
  not in the database.** The before-run must fail reliably; if N=2 is flaky,
  raise N and say so.
- **Every existing lock test passes unmodified.** A stale stamp is still a
  conflict, an unreadable stamp is still `LockValueUnreadable`, a missing row is
  still a 404 — and that last one is the distinction §2 warns about.
- **The trigger interaction, measured**: a successful update still bumps
  `updated_at` exactly once.
- **The three JS surfaces still get a fresh stamp back** and a second
  in-place edit works without a reload — the `PLAN-002` round-1 defect was
  exactly this and a state check did not catch it, so **run the no-reload
  sequence**, not just a state assertion.
- `JS-003`'s outcome classification is unchanged: conflict stays conflict.
- `DEC-007`: report the new count. New test files mean the `CONTRIBUTING.md`
  block **and** the matching `test.yml` job — both authorised, and they travel
  together.
- Three consecutive workspace runs; `fmt`; `clippy`; **the overflow gate**, since
  this touches surfaces that render.

## 5. Escalate rather than deciding

- **If any path cannot take the `WHERE` form** (§3).
- **If zero-rows cannot be distinguished from not-found** on some path without
  a second query you think is too costly.
- **If the trigger does interfere.** That would change the fix, not the
  schedule, and it is the one assumption the whole design rests on.
- **If a user-visible string changes.** None should.

## 6. Exit condition

Two concurrent edits carrying the same valid stamp leave one winner and one
conflict, demonstrated by a test that failed reliably before; the comparison
happens in the write; parsing and its message stay where they are; every
existing lock behaviour is unchanged.
