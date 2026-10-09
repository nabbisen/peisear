# Handoffs — RFC 0015, rescheduling without a pointer

RFC: [0015](../../accepted/015-rescheduling-without-a-pointer.md) — accepted
2026-10-08, **`DEC-059`**.

**The handoff is written** (`CAL-004`). This file kept the start condition
while it was deliberately unwritten; the condition — `DM-TEST-001` reviewed
and closed — was met at 0.44.0.

**One variation on `DEC-059` is flagged in `CAL-004` §0 and is the owner's to
confirm**: the RFC wrote Option A as a reschedule *page* reached from the
block; the handoff puts the control on the **issue detail page** instead,
because a second affordance in the block hits §2's geometry and retargeting
the block's link would take the pointer user's route to the issue away.

| Part | Where it lives | Release |
|---|---|---|
| The clause | **landed** — `FR-DM-002`'s normative text, `DEC-059` | 0.44.0 |
| The control | [`CAL-004`](./CAL-004-a-move-control-for-the-keyboard.md) + [`CAL-005`](./CAL-005-the-window-becomes-a-date.md) — **shipped**, `fa9d2ab` / `d9796b4`. **`FR-DM-002` Met.** | 0.45.0 |

## Why the control's handoff is not written yet

`DM-TEST-001` is with the dev team and **its item 4 pins the undo control's
DOM position on the calendar** — the surface this control changes. Writing the
implementation handoff now invites the two to run in parallel, which would
have the dev team asserting against markup that is about to move. `§7.1` of
the RFC is the full reasoning.

**The condition for writing it**: `DM-TEST-001` reviewed and closed. Nothing
else blocks it, and nothing else is waiting on it.

## What the handoff must carry when it is written

Not a design — `DEC-059` settled that. These are the four things `§8` left
open, which the handoff must **answer** rather than pass along:

1. **Target granularity.** *Same time, different day* is the drag's commonest
   gesture and the smallest honest first step. Whether time slots follow is a
   scope call to make explicitly, not by default.
2. **How many `<select>` options** before the control stops being usable. The
   month view's 35 cells is the upper bound to think about.
3. **Where the user lands after Save, and whether the calendar's scroll
   position survives it.** This is the backlog's *lost phone scroll position
   after a POST* arriving in scope, by `§7.1`'s decision that answering it
   here is cheaper than answering it twice. **It is not optional in this
   handoff.**
4. **`NFR-A11Y-007`** on the new control, and the **overflow gate at five
   widths** with it present. The gate is 120 cells; a new control on a
   calendar-adjacent page is exactly the shape that has moved that number
   four times.

And the acceptance: `FR-DM-002` reaches **Met** at 0.45.0 with a test, or the
handoff says why not. The clause it must satisfy is now in the requirement,
so the test is *does the equivalent sit on the element, and does it avoid
reconstruction* — both checkable, neither a judgement.
| The control, round 2 | [`CAL-005`](./CAL-005-the-window-becomes-a-date.md) | Round 1 accepted (`fa9d2ab`), 385 green. The dev team's escalation was right: in a **day-view** context the move control offered **exactly one option — the current day — so a Move was a no-op**, and the day view is the most natural path to a specific block. **The answer is to stop having a window, not to widen one**: a native `<input type="date">`, which the handler already accepts verbatim, so nothing server-side changes. It also dissolves RFC 0015 §8's *how many options* question and removes the overflow risk a date-labelled `<select>` carried. | 0.45.0 |

---
**Closed.** The RFC is in `done/` and its `§9` records what implementation
varied. Nothing here is outstanding.

**The one item this work named and did not carry**: the calendar's POST
lands the user at the top of the page, which is the ordinary
Post/Redirect/Get behaviour **every** mutation path in this product already
has — measured during `CAL-004`, not introduced by it. It returns to the
backlog as a measured, whole-path item rather than the vague *"lost phone
scroll position after a POST"* it was before, and it is the architect's.
