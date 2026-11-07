# FLOW-001 — five seconds to find the undo

**Series**: `059` (unit two, user workflows). **Blocked on**: nothing.

> **This round's direction was reversed on 2026-11-07, before it started,
> by the architect's own re-reading at the owner's request.** The first
> version said *delete the 5-second timer, it is less code than pausing
> it*. That was wrong, and §2 below is the measurement that says so. The
> round as it now stands is **smaller** than the one it replaces.

## 1. The defect

Four surfaces build an undo toast, and all four remove it on a fixed
timer:

```
crates/peisear-web/static/dm.js:209        }, 5000);
crates/peisear-web/static/board.js:187     }, 5000);
crates/peisear-web/static/calendar.js:244  }, 5000);
crates/peisear-web/static/plan.js:309      }, 5000);
```

`clearTimeout` appears once per file, inside `removeToast` — only when
the toast is already being taken away. **There is no pause on hover and
none on keyboard focus** (`grep` for `mouseenter|mouseleave|focusin|focusout`
across `static/`: four hits, all of them comments). A reader who has
Tabbed to Undo can have it removed from under their focus at 5 000 ms.

**WCAG 2.2.1 Timing Adjustable (Level A)**, and none of its exceptions
applies: 5 000 ms is not essential, nothing here is real-time, and it is
far under the twenty-hour exception. The owner's question of 2026-11-05
named **elder people**; five seconds to notice a message in the opposite
corner, read it, decide and reach it is an affordance announced to that
reader rather than offered.

## 2. Why deleting the timer is the wrong fix — measured

Each surface keys its toast on the **acted-on element**, not on the page:
`form._dmToast`, `card._boardToast`, `block._calToast`, `row._planToast`.
Each creates **its own container**, `class="toast toast-end toast-bottom z-50"`
(`dm.js:180` and its three siblings), and DaisyUI pins every such
container to the same place — `.toast{position:fixed}`,
`.toast:where(.toast-end){inset-inline-end:0}`,
`.toast:where(.toast-bottom){bottom:0}`. **Nothing removes another
element's toast when a new one appears**; there is no page-level sweep.

Two toasts built exactly as `dm.js` builds them, measured in the running
product at 1280 × 900:

```
first  { x: 955, y: 659, w: 310, h: 98 }
second { x: 1001, y: 659, w: 265, h: 98 }
overlap: true
elementFromPoint at the shared centre -> "Moved to Done."   (the second)
the first toast's own Undo button     -> covered: true, by the second's Undo
```

The two sentences print on top of each other and the two Undo buttons
land on each other; the earlier one is **unreachable by pointer**.

**So the 5-second timer is currently the only thing keeping simultaneous
toasts rare.** Deleting it does not remove a mechanism — it promotes an
occasional sub-five-second collision into the resting state after any two
actions, and would then require a single toast *region* (one container,
alerts appended, which is what DaisyUI's `flex-direction:column;gap:.5rem`
is for) plus dismissal and a cap. That is **more** mechanism than exists
today, and the first version of this round claimed the opposite without
measuring it.

## 3. The decision

**Pause the timer while the reader is engaging with the toast; change
nothing else.**

- `mouseenter` / `focusin` on the toast → `clearTimeout`.
- `mouseleave` / `focusout` → restart it.

That is the whole round. It satisfies **2.2.1**'s *pause* allowance, it
cannot increase the number of simultaneous toasts beyond what today
already produces (only a toast the reader is actively on outlives its
five seconds), and it pre-empts no design decision.

**What is deliberately not decided here**: *where* an outcome and its
undo should live. The outcome is currently announced **twice** — small
grey text in the `role="status"` region directly under the control, and
a cyan toast about 700 px away in the corner — and only the far copy
carries the Undo. The better answer may well be that the undo belongs
beside the thing it undoes and the toast should not exist at all; that
removes the overlap by construction and is the real *less is more*. It
is a **view** decision across four surfaces, it needs the unit-three
reading of those surfaces, and it is recorded as a candidate in this
series' README rather than smuggled into a timing fix.

## 4. Acceptance

- The four files each gain the two handler pairs and nothing else. No new
  control, no markup change, no string change — **this round adds no
  user-visible copy**.
- `undo_toast_attachment_scan`'s existing assertions stay green and
  unchanged; extend it to assert the pause handlers are attached in all
  four files, which is a source-level fact it is already in the business
  of checking.
- **A browser-level assertion is warranted here and is not gate-widening
  for its own sake.** `§10.32`'s closing note allows chasing an
  unmeasured branch *"when a defect appears behind it"*, and one has:
  `drag-outcome-gate.mjs` already reaches the toast and already asserts
  Tab lands on Undo. Add, on the two surfaces it already drives, that
  **the toast still exists after the timer would have fired while the
  pointer is over it**. `DEC-048` condition 2 forbids wall-clock
  dependence, so do **not** sleep 5 000 ms: make the duration a constant
  the gate can read and drive it short, or assert the handler's effect on
  the timer handle rather than the wall clock — and if neither is
  possible without a sleep, say so in the round report and assert the
  source-level fact only. **Do not add a `setTimeout` to the gate.**
- `DEC-007` from → to, measured with the runner, never a grep.

## 5. What this round must not do

- **Do not delete the timer**, and do not raise its 5 000 ms. Both are
  decisions that belong with §3's deferred question, not here.
- **Do not move the toast, re-home the Undo, or touch the
  `role="status"` live region**, its text or its visibility.
- Do not change what an undo *does*, on any surface.
