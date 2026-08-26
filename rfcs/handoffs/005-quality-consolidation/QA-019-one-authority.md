# QA-019 — `updated_at` has two authorities; the requirement says one

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: **P0** — the owner's words: *"Should be fixed. Security is
strongly prioritized."*
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §8
**Depends on**: `QA-018`, closed. Its finding is the input.

---

## 1. The finding, verified twice

`NFR-CONC-003` (P1): *"The application MUST NOT write `updated_at`; database
triggers MUST maintain it, so the lock value has exactly one authority."*

**Four application sites write it, across three tables:**

```
crates/peisear-storage/src/view_states.rs:71
crates/peisear-storage/src/projects.rs:138
crates/peisear-storage/src/issues.rs:467
crates/peisear-storage/src/issues.rs:572
```

**Triggers exist for `sprints`, `teams`, `team_memberships`,
`user_capacities` — and for none of `issues`, `projects`,
`user_view_states`.**

`0014_updated_at_columns.sql`'s own header explains how: it added the column
*and* the trigger to the four entities that lacked the column, and `projects`
and `issues` already had it from `0001`. The trigger convention arrived after
those two tables did, and never went back for them.

**Reproduce both halves before changing anything.** The dev team found two
sites; there are four. If you find a fifth, stop and report.

## 2. Why this is P0 when the lock currently works

All 16 `optimistic_lock` tests pass. Nothing is broken today.

**The risk is the shape of `§10.6`.** `issues.rs` already has two `SET` sites.
A third mutation path added without ` , updated_at = CURRENT_TIMESTAMP` leaves
the column stale — and a subsequent stale-lock check then **passes when it
should reject**, because the stored value still matches what the client last
saw. That is a silent optimistic-lock bypass: no error, no log line, one
member's edit overwriting another's.

`§10.6` was exactly that — the kanban endpoint bypassing the lock — and it
went four releases unnoticed. The requirement's phrase *"exactly one
authority"* is not stylistic; it is the property that makes the failure
unconstructible rather than merely absent.

## 3. The fix, and the order matters less than you would think

**Migration `0017`.** That number is free: it was reserved for a deferred email
opt-in state and **unreserved at 0.24.0** when RFC 003's rewrite found the
migration unnecessary. Confirm it is still free before using it.

**Copy the existing trigger shape verbatim** from `0014`, including its `WHEN`
clause:

```sql
CREATE TRIGGER sprints_updated_at
    AFTER UPDATE ON sprints
    WHEN OLD.updated_at = NEW.updated_at
BEGIN
    UPDATE sprints SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
```

**That `WHEN` clause is why the ordering is safe in either direction.** The
trigger fires only when the `UPDATE` did *not* itself change `updated_at`, so
adding the triggers while the application still sets the column explicitly is a
no-op — no double-bump, no behaviour change. Then removing the four clauses
hands the job to the trigger with no window where nothing maintains it.

Do it in that order anyway — triggers first, clauses second, as two commits —
so a bisect lands on a working tree either way.

## 4. One question I have not settled: `user_view_states`

`NFR-CONC-003` says the application must not write `updated_at`, full stop.
But `user_view_states` holds per-user view preferences — last board/list
choice, filters. It is not obvious that it participates in the optimistic-lock
contract at all, and a stale value there costs nothing.

**Determine whether it does**, and say so:

- If it is lock-participating, it gets a trigger like the others.
- If it is not, it still gets one — the requirement has no carve-out — but say
  plainly that this table is included for uniformity rather than for the
  safety property, so the next reader knows which.

**Do not drop it silently either way.**

## 5. The guard — this is the part that makes it stay fixed

A migration fixes today. A scan fixes the class.

Assert that **`updated_at = CURRENT_TIMESTAMP` does not appear in
`crates/peisear-storage/src/`**. After §3 there should be zero occurrences, and
any future one is the defect this handoff exists to remove.

Same family as the five existing guards. Put it where a failure names the
requirement — `NFR-CONC-003` — and the reason, the way `touch_target_scan`'s
message does. **Pin nothing about DaisyUI here**; this one rests on a schema
fact, so cite the migration instead.

**If the scan cannot be written without excluding a legitimate site**, stop and
report — an exclusion would mean §3 is incomplete.

## 6. Tests

| # | Check |
|---|---|
| 1 | `issues`: an `UPDATE` that does not touch `updated_at` still advances it |
| 2 | `projects`: same |
| 3 | `user_view_states`: same |
| 4 | The existing four trigger-backed tables are unaffected |
| 5 | A stale `client_updated_at` still yields `409` on issue and project update — the lock's behaviour is unchanged by the change of authority |

**Test 5 is the one that matters.** The point of this handoff is that the lock
keeps working while its authority moves; a test proving the mechanism survived
the migration is worth more than the three that prove the triggers fire.

Plant each: drop one trigger at a time and watch its test fail.

## 7. Escalate rather than deciding

- **If a fifth application write site exists**, stop and report.
- If `0017` is not free, stop — the numbering is architect's.
- **If adding a trigger to `issues` changes any existing test's behaviour**,
  stop. It should not: `0014`'s `WHEN` clause makes the trigger inert while the
  application still writes the column, and after §3 the application no longer
  does. If something moves anyway, my model of the `WHEN` clause is wrong and I
  want to know before it ships.
- If `user_view_states` turns out to be lock-participating after all, say so
  prominently — that would make this a live-risk table rather than a uniformity
  one.

## 8. Acceptance

1. §1's four sites and three tables reproduced; a fifth reported or ruled out.
2. Migration `0017`: triggers for `issues`, `projects`, `user_view_states`,
   copying `0014`'s shape including `WHEN`.
3. The four application clauses removed, as a second commit.
4. §4 answered explicitly.
5. The §5 scan present, running in CI, planted.
6. Five tests, each planted by dropping the relevant trigger.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 9. Required review-request format

Workflow §9.2. §4's answer as prose. Say plainly whether test 5 needed any
change — if the lock's own tests had to be adjusted, that is a finding.
