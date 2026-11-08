# FLOW-001 round 2 — two hands on one latch

**Series**: `059` (unit two, user workflows), `0.61.0`'s authorized
scope. **Follows**: `FLOW-001` (`223a740`), closed with one finding.

## 1. The finding

Round 1 gave the toast two ways to pause its five-second timer — the
reader's pointer and the reader's keyboard focus — and **one piece of
state to record it in**: `current.timer`, set to `null` while paused.
Nothing records *which* of the two is holding it, so **releasing either
one resumes the countdown, even while the other still holds**.

Measured in the real browser, on the `plan` surface, with
`drag-outcome-gate.mjs`'s own scaffolding and real
`Input.dispatchMouseEvent` moves — the sequence is: Tab to Undo, then
move the pointer across the toast and away.

```
focusInside=true  pausedByFocus=null  afterEnter=null  afterLeave=2  stillFocused=true
```

Read left to right: keyboard focus is **inside the toast**, on the Undo
button; the timer is correctly paused by that focus; the pointer
entering changes nothing (still paused); **the pointer leaving starts a
new timer — handle `2` — while focus is still inside the toast.**

So a reader who has Tabbed to Undo and then merely moves the mouse
across the toast and off it has the five seconds restarted underneath
their own keyboard focus. **That is the defect `FLOW-001` exists to
fix, re-entering through a narrower door.** It is not a regression
introduced by round 1 — before round 1 the timer never paused at all —
but the property the round claims is *"the five seconds stops counting
while the reader is on it"*, and in this sequence it does not.

**A second consequence of the same missing state, from the code rather
than from a measurement**: `resumeAutoRemove` starts a timer without
clearing one that may already be running, so two resumes in a row
(`focusout` then `mouseleave`) leave **two pending removals**, and the
earlier one wins — cutting the time short rather than extending it. I
have not measured this one; it needs a wall-clock wait, which the gate
may not have. The same fix closes it.

## 2. Where this was visible and was stepped around

`assertAutoRemovePausesOnHover` opens with
`document.activeElement && document.activeElement.blur()`, and its own
comment explains why: the Tab-order check that runs before it leaves
real focus on the Undo button, *"so the same pause/resume pair this
function is about to test for hover had already fired once for focus
(`focusin`), and the timer is still paused from it."*

That is the entanglement itself, diagnosed correctly and then **removed
from the test's path instead of being asked about**. The first run
failing with *"expected a running auto-remove timer before hovering,
found none"* was the two sources sharing one latch, reported by the
gate, one step before the question *what happens when one of them lets
go first?* The round's report is candid that this was a finding about
its own test design; the finding was one level deeper.

**Nothing here says the `blur()` is wrong** — isolating one signal from
another is the right instinct for that assertion. What is missing is the
assertion for the case the `blur()` steps over, and §4 asks for it.

## 3. The fix

Each handler checks **the other** source before resuming, at a moment
when that other source's state is reliable, and clears any timer still
running before starting a new one:

- on `mouseleave`: resume only if focus is not inside the toast —
  `!toast.contains(document.activeElement)`. No focus change is in
  flight during a pointer event, so `activeElement` is trustworthy here.
- on `focusout`: resume only if the pointer is not over the toast —
  `!toast.matches(':hover')`. The pointer's position is trustworthy
  during a focus event.
- in `resumeAutoRemove`: `clearTimeout(current.timer)` before assigning
  a new handle.

**Do not reach for `relatedTarget`, a hover flag, a counter or a state
machine.** Each handler asking the DOM for the other source's live
truth is two conditions and no new state, and the DOM is already
keeping that state correctly. Round 1's own comment says `focusin`/
`focusout` need no `relatedTarget` check for the single-focusable-child
case, and that stays true.

**Why not resume on a timer of its own, or simply never resume**: both
were considered and rejected in round 1's own §3 and this round does
not reopen them. The toast still goes away five seconds after the
reader stops engaging with it; this round only makes *stops engaging*
mean both hands, not either.

## 4. Acceptance

- **The browser-level assertion for the case the `blur()` steps over**,
  on the same two surfaces `drag-outcome-gate.mjs` already drives:
  with focus on the Undo button, a real pointer enter **and leave**
  must leave the timer `null`, and `document.activeElement` must still
  be inside the toast when that is read — assert both, so the check
  cannot pass because focus quietly moved. Then blur and assert the
  timer resumes, which is the existing behaviour and must not change.
  No sleep, no wall clock (`DEC-048` condition 2).
- The mirrored case asserted too, in whichever direction is cheap on
  these surfaces: pointer over the toast, focus enters and leaves,
  timer stays `null`.
- `undo_toast_attachment_scan`'s four new tests stay green and keep
  asserting attachment; **extend them to pin the two guards**, so a
  future edit cannot drop a condition and leave the listeners in place.
  A source scan cannot see behaviour, which is exactly why the browser
  assertion above is the primary evidence and this is the backstop.
- **Run the citation check before pushing this time**: adding tests to
  `undo_toast_attachment_scan` moves a count `requirements.md` cites in
  three places and `spec_scan` pins. Round 1 found that from CI; the
  mechanism was already in `.github/CONTRIBUTING.md`.
- `DEC-007` from → to, measured with the runner.

## 5. What this round must not do

- **Do not delete, extend or tune the 5 000 ms**, do not move the
  toast, do not add a dismiss control, do not touch the
  `role="status"` region, and do not change what Undo does. Round 1's
  own §5 holds unchanged.
- **Do not build a toast region or a cap.** Where an outcome and its
  undo belong is open question 2 in this series' README, for unit
  three.
