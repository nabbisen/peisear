# RFC 0013: The sprint record — capturing what a sprint reported, instead of freezing what it contains

**Status**: **Accepted** (2026-09-24) — implementation may begin
**Target**: 0.39.0
**Related spec sections**: `SPEC §9.1`, `SPEC §17.3`, `GUI §5`
**Related requirements**: `FR-SPR-001`, `FR-SPR-003`, **`FR-SPR-004`** (whose
text this RFC changes), `FR-PLAN-001`
**Closes if adopted**: `FR-SPR-004` for real, rather than for one of its four
inputs
**Governing decisions**: `DEC-053` (reopen, accepted 2026-09-24); proposes
`DEC-054`
**Withdraws**: `SPRINT-002`; the first half of `SPRINT-001`
**Last updated**: 2026-09-24 — written after the dev team stopped
`SPRINT-002` before writing code, and after checking what the figures are
computed from

## Summary

`FR-SPR-004` says *"A completed sprint's issue membership MUST NOT be
editable"*, with the rationale *"editing a completed sprint rewrites history
and corrupts trend data."*

**The rationale is right and the requirement is the wrong mechanism.** A
completed sprint's figures are computed live from four inputs — its membership,
and each member issue's `status`, `effort` and `updated_at`. Freezing
membership addresses one of them, and it collides head-on with carry-over,
which is the product's designed way of handling unfinished work.

**This RFC captures the record at completion and leaves membership alone.**

## Background — how this was found, because it matters

`SPRINT-001` and `SPRINT-002` were written to enforce `FR-SPR-004` as worded.
`SPRINT-001` shipped to `main`; `SPRINT-002` did not, because the dev team
measured the flow it would break and stopped before writing code.

**What they found**: `backlog_for_team` excludes issues in an *active* sprint
only — deliberately, per its own doc comment — so an unfinished issue in a
completed sprint is a backlog candidate for the next sprint. Adding it there
goes through the upsert that moves the membership. **`SPRINT-002` would have
refused the primary flow of the page whose purpose is that flow**, for issues
the same page lists as candidates.

**What one further question found**: `summary` computes `completed_points` as
`SUM(CASE WHEN i.status = 'done' …)` over current members, and `burndown` reads
each member's `status`, `effort`, `assigned_at` and `updated_at`. Its own doc
says *"Compute the **live** summary numbers."*

**So a completed sprint's record drifts with no membership change at all.**
Carry an issue over and finish it a month later, and the *completed* sprint's
`completed_points` rises and its `carried_over` falls. Both handoffs were aimed
at one of four inputs and would have bought a guarantee neither could deliver.

There is a comment at `sprints.rs:620-626` that says the summary *"captures the
moment"*. **It has never been true.** This RFC makes it true.

## What is actually required

`FR-SPR-004`'s rationale wants one thing: **what a sprint reported when it
completed does not change afterwards.** Everything else is mechanism.

| input | drifts because | frozen by `SPRINT-001`/`-002`? |
|---|---|---|
| membership | carry-over moves the row | yes — and that is the collision |
| `issues.status` | the issue is finished later | no |
| `issues.effort` | anyone edits it | no |
| `issues.updated_at` | any edit at all | no |

## The options

**A — ship `SPRINT-002` as written.** Carry-over stops working from the
planning page. Rejected: it breaks the product's primary sprint workflow to fix
a quarter of the problem.

**B — refuse one route, allow the other.** Rejected on `SPRINT-002`'s own
argument: a protection you can walk around is worse than none, because the
message asserts a guarantee the product does not keep.

**C — capture the record at completion.** `complete` writes what the sprint
reported; reads of a completed sprint use the captured values; membership and
issues stay editable. **Recommended.**

**D — let an issue belong to several sprints.** The more truthful data model,
and it was taken seriously: a completed sprint keeps its row, so membership
never moves. **Rejected because it fixes the membership input and not the other
three** — a schema change that leaves the record drifting on issue status. If
the record is captured, D buys nothing it does not already have.

## Recommendation: C

**`complete` captures; `reopen` discards; nothing else changes.**

1. **A `sprint_records` table**, one row per completed sprint, holding the
   summary's four numbers. Scalar columns, following `metrics_snapshots` and
   `user_metrics_snapshots`, which are this project's existing shape for
   exactly this — not a JSON blob.
2. **A `sprint_burndown_points` table**, one row per sprint per day
   (`day`, `cumulative_committed`, `cumulative_completed` — `BurndownPoint` as
   it already exists). The burndown of a completed sprint is a series and a
   series is rows.
3. **`summary` and `burndown` read the captured record when the sprint is
   `completed`, and compute live otherwise.** One branch each, at the top.
4. **`complete` writes both, inside its existing `BEGIN IMMEDIATE`**
   (`RACE-001`). Capturing outside the transaction that sets the status would
   reintroduce exactly the class those handoffs closed.
5. **`reopen` (`DEC-053`, `SPRINT-003`) deletes both**, inside its transaction.
   A reopened sprint is live again, and completing it captures afresh. This is
   what makes reopen *mean* something rather than being a status flip.
6. **`FR-SPR-004` is rewritten** from membership to the record. Its rationale
   is unchanged; its mechanism was the defect.

**What this buys that freezing does not**: carry-over works as designed;
`backlog_for_team` needs no change; `SPRINT-002` is unnecessary; the
walk-around problem disappears because there is nothing left to walk around;
and the record is stable against all four inputs rather than one.

**What it costs**: a migration, two tables, a branch in two read paths, writes
in two lifecycle transitions, and a backfill decision (below).

## Decisions taken 2026-09-24

**Accepted with both recommendations.** `DEC-054` is adopted: *the record of a
completed sprint is captured at completion, not computed.*

1. ~~**Backfill or not?**~~ — **Backfill**, at migration time, from the live
   computation. It is the best available answer and it is **wrong by exactly
   the drift that has already happened**; that is stated here, and belongs in
   the changelog rather than being discovered later. The alternative would
   leave two classes of completed sprint with nothing distinguishing them to a
   reader.
2. ~~**Does the issue list stay live?**~~ — **Yes, and it is labelled.** The
   list answers *what is in this sprint now*; the figures answer *what it
   reported*. A carried-over issue therefore leaves the list while the captured
   count still counts it, and the page says which is which. Capturing the
   membership list as a third table was the alternative and is not taken.

## Open questions — as put, now settled above

1. **Existing completed sprints have no record.** Backfill them at migration
   time from today's live computation — which is the best available answer and
   is *wrong by exactly the drift that has already happened* — or leave them
   computing live, with the capture applying only to sprints completed from
   0.39.0. **My recommendation: backfill, and say so in the changelog.** A
   sprint that reports a drifted figure consistently forever is better than one
   that keeps drifting, and the alternative is two classes of completed sprint
   with no way for a reader to tell them apart.
2. **Does a completed sprint's issue *list* stay live?** Its figures will be
   captured; `issues_in_sprint` would still show current membership, so a
   carried-over issue leaves the list while the captured count still counts it.
   **My recommendation: yes, live, and label it** — the list is "what is in
   this sprint now", the figures are "what it reported". If that reads as a
   contradiction to the owner, capturing the membership list too is the
   alternative and it is a third table.
3. **`DEC-054`** — this RFC's proposed decision: *the record of a completed
   sprint is captured at completion, not computed.*

## Schedule

One substep, 0.39.0, after the `SPRINT-001` revert below. `SPRINT-003`
(reopen) moves to depend on this RFC rather than on `SPRINT-002`, and should
ship with it — capture without discard is a trap.

## Out of scope

Velocity's own definition. Anything about `issue_events`. Sprint templates,
recurring sprints, multi-team sprints. The one-active-sprint rule, which
`RACE-001` settled.

## What comes out of `main` if this is accepted

- **`SPRINT-002`: withdrawn**, never implemented.
- **`SPRINT-001` §1** — the unassign refusal and its message key — **reverted.**
  It blocks carry-over from the issue form and buys nothing once the record is
  captured.
- **`SPRINT-001` §2** — `plan_remove` racing `start` — **stays.** That is a
  race about whether a sprint is *plannable*, independent of history.
- Nothing is released: all of it postdates the 0.38.0 tag.

## References

- `.git-exclude/reviewed/SPRINT-002-003-escalation-review.md` — the ruling and
  the four-input finding
- `.git-exclude/review-request/SPRINT-002-003-escalation/` — the measurement
  that stopped the work
- `FR-SPR-004`, and its `0.39.0` correction recording that its old status
  blamed an unbuilt screen for three shipped routes
