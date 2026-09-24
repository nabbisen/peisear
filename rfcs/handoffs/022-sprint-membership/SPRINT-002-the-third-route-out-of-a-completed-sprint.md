# SPRINT-002 — the third route out of a completed sprint

> **WITHDRAWN 2026-09-24, never implemented** (RFC 0013). The dev team stopped
> before writing code: this would have refused the planning page's primary
> carry-over flow for issues the same page lists as candidates. Checking further
> showed why — membership is **one of four inputs** to a completed sprint's
> figures, so freezing it could not deliver the guarantee this handoff claimed.
> `SPRINT-004` captures the record instead. **Kept for the reasoning, not as
> work**; §1's *a protection you can walk around is worse than none* still holds
> and is why partial enforcement was rejected.

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.39.0
**Governing RFC**: none — defect fix.
**Source**: found and measured by the dev team while implementing `SPRINT-001`
(§3 of its review request); confirmed in
`.git-exclude/reviewed/SPRINT-001-review.md` §1.
**Depends on**: `SPRINT-001`, shipped.

---

## 1. The defect

`sprints::add_issue`'s upsert:

```sql
ON CONFLICT(issue_id) DO UPDATE SET
    sprint_id = excluded.sprint_id,
    assigned_at = CURRENT_TIMESTAMP
```

One row per issue, so assigning an issue to a sprint **moves** it — and takes
it out of whatever sprint it was in. **Nothing checks that sprint.** Measured:
a completed sprint recording `committed 15 / 3 issues` read
`committed 12 / 2 issues` after one of its issues was assigned elsewhere
through the ordinary route, answered `303`.

**Same consequence as `SPRINT-001` §1, by a route that handoff did not close.**
`SPRINT-001` refuses removing an issue *from* a completed sprint; this reaches
the identical outcome by adding it *to* another one.

**That is why this is urgent rather than tidy.** After `SPRINT-001` a user who
tries the direct route reads *"Cannot remove issues from a completed sprint"*
and can then do exactly that with the next control along. **A protection you
can walk around is worse than no protection**, because the message asserts a
guarantee the product does not keep.

## 2. The fix

**In `add_issue_if_status`, refuse when the issue's *current* sprint is
`Completed` and differs from the target.** The target-side check stays as it
is; this adds the source side.

- **The refusal is `SPRINT-001`'s new message**,
  `CannotUnassignFromCompletedSprintMessage` — the user is being stopped from
  taking an issue *out of* a completed sprint, which is what that message says.
  **Do not add a second key** unless the wording genuinely does not fit from
  the assign page, in which case report it rather than composing one.
- **Same sprint is not a move**: re-assigning an issue to the sprint it is
  already in must stay a no-op, not a refusal.
- **The early check** goes in the handler beside the target-status one, for the
  ordering reason `RACE-002` §3 settled.
- One `BEGIN IMMEDIATE`, as `SPRINT-001` established. The source read and the
  upsert are one operation.

## 3. What this deliberately does not do

**It does not give anyone a way to correct a sprint completed by mistake.**
After this, a completed sprint's membership is closed in all three directions.

That gap is real and it is **the owner's to close**, with an explicit *reopen*
action if they want one — a visible state change rather than a silent edit to a
finished record. **Do not build one here, and do not soften this fix to leave
room for it.** If the owner asks for reopen, it arrives as its own handoff and
this fix is what makes it meaningful.

## 4. Verification

- **Fails before, passes after, sequentially** — no race needed. A completed
  sprint with recorded `summary` and `burndown`; assign one of its issues to
  another sprint; **assert both figures are unchanged**, the membership is
  still in the completed sprint, and the request is refused. **Figures first**,
  as `SPRINT-001` did.
- **Everything that must still work, named and tested**: moving an issue
  between two *planned* sprints; between *active* ones; from planned to active;
  assigning an issue that is in **no** sprint; re-assigning an issue to the
  sprint it is already in (a no-op, not a refusal).
- **`SPRINT-001`'s three tests pass unmodified.**
- `DEC-007`: report the new count, last recorded **329**. `race_guards.rs`
  exists; if you add a file, the block **and** `test.yml` travel together,
  authorised.
- Three consecutive workspace runs; `fmt`; `clippy`. **Run the overflow gate**
  if the refusal renders anywhere new — you judged that correctly last time.

## 5. Escalate rather than deciding

- **If `CannotUnassignFromCompletedSprintMessage` reads wrongly** from the
  assign page. Report the sentence you would want; do not write it.
- **If refusing the same-sprint case turns out to be needed** for some flow —
  that would mean I have misread the upsert, and it changes the fix.
- **If a fourth route out of a completed sprint exists.** Cascade deletes are
  known and excluded (the issue no longer exists). Anything else is new.
- **If any existing user-visible string changes.** None should; no new one is
  expected either.

## 6. Exit condition

A completed sprint's `summary` and `burndown` are unchanged by every route that
can alter `sprint_issues` short of deleting the issue itself; moves between
planned and active sprints are unaffected; the refusal reuses `SPRINT-001`'s
message.
