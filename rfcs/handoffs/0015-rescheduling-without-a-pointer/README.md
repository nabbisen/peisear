# Handoffs — RFC 0015, rescheduling without a pointer

RFC: [0015](../../accepted/015-rescheduling-without-a-pointer.md) — accepted
2026-10-08, **`DEC-059`**.

**There is deliberately no handoff here yet, and this file exists so that is
a state rather than an oversight.**

| Part | Where it lives | Release |
|---|---|---|
| The clause | **landed** — `FR-DM-002`'s normative text, `DEC-059` | 0.44.0 |
| The control | **not yet written** — see the condition below | 0.45.0 |

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
