# RFC 0004d: Direct manipulation — the calendar (D-3)

**Status**: **Proposed**
**Target**: 0.36.0
**Umbrella**: [RFC 0004](./004-direct-manipulation.md) — substep D-3
**Governing decisions**: `DEC-021`, `DEC-013`, `DEC-049`
**Related requirements**: `FR-CAL-*`, `FR-DM-002/005`, `NFR-CONC-001`,
`NFR-LANG-001`, `NFR-A11Y-001/006/007`
**Last updated**: 2026-09-13

## Summary

Drag an issue block on the calendar to reschedule it, and drag its edge to
change how long it runs.

**This substep is the first since D-1 where the optimistic lock applies in
full.** An issue's `planned_start_at` and `planned_end_at` live on the
`issues` row, which `DEC-013`'s triggers maintain, so every umbrella
requirement about `client_updated_at`, the 409 path and the returned lock value
is in force — unlike D-4, where the join table has no lock to carry.

**The sketch has three actions and they do not have the same standing.** Two
are enhancements of a path that already ships. The third is a new affordance
with no no-JS equivalent, and umbrella requirement 0 forbids shipping it as
drafted.

| Sketch action | Requirement 0 |
|---|---|
| **Body drag → reschedule** (move both planned timestamps by the delta) | **Satisfied.** The issue edit form carries `datetime-local` inputs for `planned_start_at` and `planned_end_at`, and the handler parses both |
| **Edge drag → resize** (change `planned_end_at`) | **Satisfied.** Same form, same field |
| **Empty-cell drag → new-issue dialog with the date pre-filled** | **Not satisfied.** The new-issue form has no planned-date prefill from a query parameter. There is no plain-form path to "create an issue already scheduled on this day" |

**Recommendation: the third action leaves this substep.** Either it ships its
own no-JS path first — a query-parameter prefill on the new-issue form, which
is small and useful on its own — or it moves to its own substep. Folding it in
would make the enhancement the first implementation of the action, which is
exactly what requirement 0 exists to prevent.

## Background — reconciled against the code, 2026-09-13

**The calendar renders blocks as links.** `calendar.rs::render_block` emits
`<a href=… class="block text-xs px-1.5 py-0.5 rounded bg-primary/10 … truncate">`,
positioned by percentage in the day view (RFC 002's design). The month and week
views pack several chips into one cell.

**Two consequences the sketch does not anticipate:**

1. **A draggable link drags the link.** An `<a href>` is draggable by browser
   default, and D-2 hit this exactly: without `draggable="false"` the drag
   became a link drag, carrying the href in `dataTransfer` and showing the
   link ghost, because two nested drag sources existed
   (`DEV-002-005-review.md` §1.3). Whatever this substep attaches, that trap is
   already documented and must not be re-paid for.
2. **These blocks are two of the three call sites `DEC-050` excluded from the
   44 px floor**, keyed in `touch_target_scan.rs` on `bg-primary/1` — because a
   block's height is proportional to an appointment's duration and a 44 px
   floor would make a fifteen-minute meeting look like a two-hour one. **A drag
   handle added to a block is a new interactive element and is not covered by
   that exclusion.** It takes `grow()`, or it is a finding. If a 44 px handle
   cannot fit inside a fifteen-minute block, that is a design question to
   escalate before building, not to solve with a fourth exclusion.

**And the fact D-4's RFC records, which this substep inherits**: `board.js`
has no touch or pointer handling at all, so the product's only drag does not
exist on a phone. A calendar drag will be mouse-only for the same reason. The
calendar is one of `NFR-A11Y-006`'s four verified mobile flows, and it was
verified through navigation and links, not drag. **No part of this substep may
be described as how a phone user reschedules an issue.**

## Requirements

1. **Reschedule and resize only.** The empty-cell action is out, per §Summary.
2. **No user-visible string is authored inside the new script** — the island
   pattern, as in D-2 and D-4.
3. **The optimistic lock applies in full.** The mutation carries
   `client_updated_at`; on 409 the UI reverts and says the current state is now
   shown, with no silent retry (requirement 5). The endpoint returns the new
   `updated_at` and the client updates the block's `data-updated-at` from it
   (requirement 6). **Use the one shared lock function with two entry points
   that `DEV-001` established; introduce no second lock check.**
4. **`LOCK-001`'s guarantee extends to the calendar block.** Every board card
   renders a non-empty `data-updated-at` and one test asserts it, per card. A
   calendar block that a drag mutates needs the same value and the same
   assertion — the guarantee is what makes an optimistic update possible, and
   it should be asserted where it is relied on.
5. **Undo restores the previous start and end**, both of them, as one inverse
   mutation (requirement 4, and the umbrella's substep contract names this
   direction explicitly).
6. **Rollback on failure before the mutation lands**, falling back to the plain
   form; the fallback catch ends where the mutation is confirmed
   (requirement 2a).
7. **Any new interactive element takes `grow()`** (`NFR-A11Y-007`), and the
   existing block exclusion is not extended.
8. **The day view's proportional geometry is preserved.** A resize changes
   `planned_end_at`, and the block's height must follow from the new duration
   rather than from where the pointer stopped. The two must not be allowed to
   disagree.
9. **`calendar_surfaces`'s ten tests pass unchanged**, and the no-JS path — the
   edit form — is untouched.

## Design

### D1 — Which view, and the case for one

Three views exist: day (percentage-positioned, an hour ruler), week and month
(chips packed into a cell). **Reschedule and resize mean different things in
each**, and a resize in a month cell has no visual handle to grab.

**Recommendation: the day view only, in this substep.** It is the view whose
geometry is time, so a drag distance maps to a duration; in month and week a
drag maps to a date at best, and resize does not map at all. Week and month
can follow once the day view's mechanics are proved, or not at all if the
value is not there.

### D2 — The delta, computed where it can be checked

A body drag moves both timestamps by one delta. **The delta is computed from
the pointer's pixel offset against the ruler's scale**, which is mechanics and
stays in the script. **What the new values are is policy**: the server already
parses `datetime-local` and already validates the pair. The client sends
candidate values; the server decides. `JS-001`'s distinction, applied in
advance.

Rounding is a decision, not an implementation detail: a fifteen-minute snap is
what a calendar usually does, and it must come from the island rather than a
literal in the script.

### D3 — What must not change

- `render_block`'s link and its `href` — the block is still a link to the
  issue, and a click still navigates.
- The `bg-primary/1` exclusion in `touch_target_scan.rs`, and the reason
  recorded beside it.
- `FR-CAL-003`'s planned-date semantics and migration `0016`. **No new
  schema** — the umbrella's substep contract says only D-5 needs one, and this
  substep confirms it.
- The `break-words` and `min-w-0` classes `§10.25` put on the calendar's
  heading.

## Test plan

- **Rust**: `calendar_surfaces`'s ten tests unchanged. New assertions for the
  block's `data-updated-at` (requirement 4), in the shape `LOCK-001` used —
  per block, and non-empty, which is the property that fails when the guarantee
  is lost.
- **The lock path is testable without a browser** and should be: a reschedule
  with a stale `client_updated_at` returns 409, through the same shared
  function the other surfaces use. That is a Rust test, not a JavaScript one.
- **The drag is executed by no test** (`§10.15`, open permanently by decision).
  Stated, not implied.
- **`BROWSER-001`** sweeps the project calendar as of `LAYOUT-005`. A handle
  that widens a block turns a cell red.

## Security and privacy considerations

Rescheduling is an ordinary issue mutation and carries the same authorisation
as the edit form. `NFR-PRIV-*` is unaffected: a calendar block shows what the
viewer can already see.

## Out of scope

- **The empty-cell → new-issue action** (§Summary). Its no-JS prefill path
  ships first, separately, or the action does not ship.
- Week and month view drag (§D1).
- Reordering, which is D-5.
- Drag on touch, which does not work (§Background).

## Open questions

1. **Is the day view enough?** §D1 recommends it. If the value is mostly in
   the month view — where a user plans a week at a glance — then this substep
   is the wrong shape and should be rethought before it is accepted.
2. **Does a 44 px drag handle fit a fifteen-minute block?** If not, the choice
   is between a handle that violates `NFR-A11Y-007`, a block minimum that
   breaks the proportionality `DEC-050` protected, and dragging the block body
   with no handle at all. **Recommendation: the body, no handle** — it needs no
   new interactive element, so the question does not arise. Resize then needs
   an edge affordance, which is the same question again and is why resize may
   belong in its own substep.
3. **Fifteen-minute snap, or free?** §D2 recommends snapping and says the
   value belongs in the island.

## References

- [RFC 0004](./004-direct-manipulation.md) — umbrella
- [RFC 0004c](./004c-direct-manipulation-sprint-plan.md) — D-4, and the touch
  finding this substep inherits
- [RFC 0002](../done/002-calendar-surfaces.md) — the views and their geometry
- [RFC 0004b](../done/004b-direct-manipulation-board.md) — the link-drag trap
- `.git-exclude/tasks/architect/018-0.34.0-scope-proposal.md` §3
