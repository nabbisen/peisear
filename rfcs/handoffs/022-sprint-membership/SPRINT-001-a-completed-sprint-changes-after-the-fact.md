# SPRINT-001 — a completed sprint's record changes after the fact

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.39.0
**Governing RFC**: none — defect fix.
**Source**: both found by the dev team in `RACE-003`'s review request §5;
confirmed and ruled in `.git-exclude/reviewed/RACE-003-review.md` §5.
**Depends on**: `RACE-003`, shipped — §1 reuses `add_issue_if_status`'s shape.

**Two defects with one consequence and two different causes.** §2 is a race;
**§1 is not, and §1 is the worse one.** Grouped by what they do rather than by
how, because that is what a reader needs.

---

## 1. Unassigning rewrites a completed sprint — no concurrency required

`handlers/sprints.rs:607`, `assign_issue`'s unassign branch:

```rust
if sprint_id_trimmed.is_empty() {
    // Unassign.
    sprints::remove_issue(&state.db, &issue_id).await?;
}
```

**No status check.** `remove_issue` takes only the issue id, so it removes the
membership wherever it is — including a **completed** sprint.

**Twelve lines below, the same function's assign branch says:**

> *Refuse to assign to a completed sprint — historical sprint summaries should
> remain stable.*

**The product states the principle for one direction and ignores it for the
other**, and it applies harder to removal. `sprints::burndown` reads
`FROM sprint_issues si JOIN issues i`, so deleting a row **retroactively
changes a completed sprint's burndown and its committed figure.** A sprint that
recorded fifteen points committed and four completed can be made to say
something else, months later, by one person changing one issue's sprint field.

**One click, no race.** Every survey this month has been hunting concurrency;
this needed none, which is why it survived them.

**The fix**: the unassign branch refuses when the issue's current sprint is
`Completed`, with the reason the assign branch already gives.

- **A new message key is needed** — `CannotAssignToCompletedSprintMessage` says
  *assign*, and a user unassigning would be told the wrong thing. Add one,
  worded for removal, through the message table like any other
  (`NFR-LANG-001`); **do not reuse the assign key and do not compose a string.**
- `remove_issue` should not silently do nothing: put the check where the write
  is, in a storage function that re-reads the status, the shape
  `add_issue_if_status` established. That also closes §2.
- **The early check in the handler stays**, for the ordering reason `RACE-002`
  §3 settled.

**A question to answer rather than assume**: `plan_remove` on a *planned*
sprint and unassign on an *active* one must both still work. Only `Completed`
is refused. If you find a fourth caller of `remove_issue`, report it.

## 2. `plan_remove` races `start` — `plan_add`'s mirror

`plan_remove` reads the sprint, requires `Planned`, then calls
`sprints::remove_issue` on the pool. A concurrent `start` lets it remove an
issue from a sprint that is no longer plannable.

Same file, same shape and same fix as `RACE-003` §1: the status re-read and the
delete under one `BEGIN IMMEDIATE`, in a storage function, with the route's own
refusal message passed in — `remove_issue_if_status`, mirroring
`add_issue_if_status`. `SprintPlanNotEditableMessage` is unchanged.

**Lower severity than §1**: it needs a race, and the sprint it damages is one
being started rather than one long finished.

## 3. Verification

- **§1 fails before and passes after, sequentially** — no barrier, no forced
  interleaving. Assign an issue to a sprint, complete the sprint, record its
  `summary` and `burndown`, unassign, and assert **both are unchanged** and the
  request is refused with the new message. **The assertion that matters is the
  figures, not the status code** — the defect is that history moved.
- **§2 needs `RACE-003`'s forced interleaving**, not a barrier, for the reason
  §1 of that review gives: both orders leave the same state. Hold the lock,
  let the requests block at their write, `start` inside the held transaction,
  commit. Report the before failure rate and the after.
- **Everything that must still work**: unassigning from a planned sprint and
  from an active one; `plan_remove` on a planned sprint; assigning across
  sprints. Name the tests.
- **The new message** appears in the message table and passes the i18n guards
  and the vocabulary check.
- `DEC-007`: report the new count, last recorded **326**. `race_guards.rs`
  exists, so no block edit is expected; if you add a file, the block **and**
  `test.yml` travel together, authorised.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched unless the new message renders somewhere that wraps — if it does,
  run it.

## 4. Escalate rather than deciding

- **If `summary` or `burndown` turn out to be stable across a removal** after
  all — that would mean I have misread which figures they derive, and it
  changes §1's severity, not its fix.
- **If refusing the unassign breaks a legitimate flow** — an issue moved
  between sprints, a sprint completed by mistake. That is a product question
  and it is the owner's, not yours: report it and stop.
- **If a seventh instance of the read-then-write shape appears.** Four surveys
  have found it ten times. Report; do not fix.
- **If any existing user-visible string changes.** The new one is an addition.

## 5. Exit condition

A completed sprint's `summary` and `burndown` are the same after an attempted
unassign as before it, and the attempt is refused with a message written for
removal; `plan_remove` cannot remove from a sprint started under it; planned
and active sprints are unaffected in both directions.
