# FLOW-001 — five seconds to find the undo

**Series**: `059` (unit two, user workflows). **Blocked on**: nothing.

## 1. What was measured

Four surfaces build an undo toast, and all four remove it on a fixed
timer:

```
crates/peisear-web/static/dm.js:209        }, 5000);
crates/peisear-web/static/board.js:187     }, 5000);
crates/peisear-web/static/calendar.js:244  }, 5000);
crates/peisear-web/static/plan.js:309      }, 5000);
```

`clearTimeout` appears in each file exactly once, inside `removeToast`
— i.e. only when the toast is already being taken away. **There is no
pause on hover and none on keyboard focus.** A reader who has Tabbed to
the Undo button can have it removed from under their focus at 5 000 ms;
`removeToast` then returns focus to the control, which is careful and
correct handling of a situation that should not arise.

Observed in the running product (issue detail, status changed from Open
to In Progress, 1280 × 900): the outcome appears **twice** — once as
small grey text directly under the status buttons, which is the
`role="status"` live region, and once as a cyan toast in the **bottom
right corner**, about 700 px diagonally from the control that was just
pressed. **The Undo button exists only in the toast.**

`A11Y-004` already placed the toast in DOM order immediately after the
acted-on control so that Undo is the next Tab stop, and
`drag-outcome-gate.mjs` asserts that it really is. That work is good and
this round must not undo it: the problem is not where the toast sits in
the Tab order, it is that **both** the sighted path (find it in the far
corner) and the keyboard path (press Tab once) are on a 5-second clock.

## 2. The reader this is for

The owner's question on 2026-11-05 named **elder people** specifically.
Five seconds to notice a message in the opposite corner of the screen,
read it, decide, and reach it, is not an affordance offered to that
reader; it is an affordance announced to them. **WCAG 2.2.1 Timing
Adjustable (Level A)** is the criterion, and none of its exceptions
applies — the 5 000 ms is not essential, nothing is real-time, and it is
far under the twenty-hour exception.

## 3. The decision

**Delete the timer.** The toast lives until one of: Undo is pressed; the
toast is dismissed; a new toast replaces it on the same surface (the
code already keeps one per surface, via `form._dmToast` and its three
siblings); or the reader leaves the page. No pause logic, no resume
logic, no extend control, and **less code than today**, which is the
test this project applies to a simplification.

**The alternative, recorded because a reviewer would otherwise raise
it**: keep a timer, pause it on `mouseenter` and on `focusin`, resume
on `mouseleave`/`focusout`, and raise the floor. That conforms too. It
is rejected here because it adds four event handlers and two state
transitions per surface — on four surfaces — to preserve a behaviour
nobody has asked for, and because "it stays until you deal with it" is a
sentence a reader can predict, while "it stays five seconds unless your
pointer is over it" is not.

## 4. What to build

- Remove the `setTimeout`/`timer` pair from all four files. `removeToast`
  keeps its `clearTimeout` guard only if a timer can still exist; if none
  can, delete that too rather than leave a dead branch.
- Keep: the DOM-order placement (`A11Y-004`), the focus restoration on
  removal, the one-toast-per-surface replacement, and every string.
- **The toast needs a dismiss affordance now that it does not expire.**
  A close button inside the toast, labelled with a word through the
  message table like everything else, meeting `touch_target_scan`'s own
  44 px floor. It is the only new control in this round.

## 5. Acceptance

- The three browser gates stay green, and `drag-outcome-gate.mjs`'s own
  *"Tab from the acted-on block lands on the undo button"* assertion is
  **unchanged and still passing** — it is the proof that `A11Y-004`
  survived this round.
- A new browser-gate assertion is **not** what this round adds. The
  honest check is in `undo_toast_attachment_scan`'s own territory: assert
  from source that no `setTimeout` remains in the four files' toast
  paths, and that each toast carries a dismiss control. Say plainly in
  the round report that *"the toast is still present after N seconds"* is
  **not** asserted by anything, and why: `DEC-048` condition 2 forbids a
  wall-clock-dependent check, and a gate that sleeps to prove a timer is
  gone is exactly that.
- `peisear-i18n`'s own suite for the new label, including the fixture
  locale.
- `DEC-007` from → to, measured with the runner.

## 6. What this round must not do

- **Do not move the toast.** Where the outcome is drawn relative to the
  control that caused it is a real question — the live region and the
  toast say the same sentence in two places — and it is a **view**
  question, for unit three, not a workflow one. Changing both at once
  would make neither measurable.
- **Do not touch the `role="status"` live region**, its text, or its
  visibility.
- Do not change what an undo *does*, on any surface.
