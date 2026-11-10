# VIEW-003 — one outcome, one place

**Series**: `060` (unit three), `0.62.0`'s scope as re-authorized
2026-11-11. **Blocked on**: nothing.

## 1. What is wrong, already measured

A completed direct manipulation announces itself **twice**:

- in the `role="status"` region **directly under the control**, as small
  grey text; and
- in a toast in the **bottom-right corner of the viewport**, about
  700 px away on a 1280 px screen — which is **the only copy carrying
  Undo**.

And because each toast is built per acted-on element
(`form._dmToast`, `card._boardToast`, `block._calToast`,
`row._planToast`), every one of them a container pinned to the same
corner with nothing clearing another's, **two of them overlap**:
measured at `FLOW-001`, both sentences print over each other and the
earlier **Undo** is covered and unreachable by pointer. The five-second
timer — which `FLOW-001 r2` then taught to pause — is what keeps that
rare.

## 2. The decision

**One outcome, one place: the undo belongs beside the thing it undid,
and the corner toast stops existing.**

This removes all three problems **by construction** rather than managing
them: there is no second copy of the sentence, no distance to cross, and
no shared corner for two toasts to collide in. `A11Y-004`'s own property
— *Undo is the next Tab stop after the acted-on control* — becomes
trivially true instead of carefully maintained in DOM order.

**All four surfaces, in one round.** `dm.js`, `board.js`, `calendar.js`,
`plan.js` are one shape in four files; two of them are on this row's own
screens and two are not, and the row was re-authorized on 2026-11-11
precisely so this is not split. **A mechanism half-moved is the defect
it is fixing.**

## 3. What to build

- The outcome sentence and its **Undo** render together, in the
  `role="status"` region that already exists beside each surface's
  acted-on element. One sentence, one region, one control.
- **Delete the toast**: the container, its `toast-*` classes, its timer,
  and `FLOW-001 r2`'s pause/resume pair, which exists only because the
  thing it guards is about to stop existing. Fewer moving parts than
  before this series started, which is the test for this kind of change.
- **Keep the timer's *purpose*, not its mechanism.** The undo offer
  should not sit on screen for ever: it ends when the reader acts, when
  the next outcome replaces it in the same region, or when they leave
  the page. **No wall clock.** If that turns out to need a duration
  after all, stop and say so in the report rather than reintroducing one
  quietly — `FLOW-001`'s own reversal is what that costs.
- `removeToast`'s focus restoration has no counterpart once the control
  never moves focus in the first place. Delete it; do not port it.

## 4. Acceptance

- **`drag-outcome-gate.mjs`'s three existing properties stay true and
  keep their names**: the live region receives the outcome text, the
  stored effect is confirmed over HTTP, and **Tab from the acted-on
  element reaches Undo**. The third is the one this round makes easy;
  it must still be asserted, because *easy* and *true* are different.
- `FLOW-001 r2`'s two pause assertions are **deleted with the timer they
  describe**, not left asserting a thing that no longer exists. Say so
  in the report by name.
- `undo_toast_attachment_scan` is rewritten or retired, whichever is
  honest: it pins a DOM chain that is going away. **Do not leave a
  source scan asserting a shape nobody renders.** Its `requirements.md`
  citations move with it — run the citation check **before** pushing.
- A plant per surface: remove the Undo control from the region and show
  the gate failing **by name**, on the two surfaces the gate drives.
- `browser-checks` 120 cells; `touch_target_scan` on the relocated
  control.
- `DEC-007` from → to, measured with the runner.

## 5. What this round must not do

- Do not change what an undo *does*, on any surface.
- Do not change any outcome sentence's wording.
- Do not touch the health or workload disclosures this series shipped.
- Do not reintroduce a timer in any form without stopping first (§3).
