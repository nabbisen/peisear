# FOCUS-001 — where Undo leaves you

**Series**: `060`, inside `0.62.0`'s authorized scope as a **repair of
this series' own work**, not new scope. **Blocking**: the `0.62.0` cut.
**Cause**: an architect's instruction, named below.

## 1. What is wrong, measured

`VIEW-003` replaced the corner toast with a relocatable live region, and
the offer now ends with `region.textContent = ""` — which removes the
**focused** Undo button from the document.

Measured 2026-11-12 on the issue-detail status surface (`dm.js`), with
real CDP key events rather than `element.click()`:

```
focus reached the Undo button: after 3 Tab press(es)  -> BUTTON.btn.btn-xs [Undo]
immediately after activating Undo                     -> BODY
1.5 s later, after the request settled                -> BODY
```

`NFR-A11Y-002` (**P1**) states *"Focus does not land on `body` on any
measured path"* and counts the undo's own endings among the eight sites
it measured. A keyboard reader who uses Undo is returned to the top of
the document and must traverse the page again — which is the exact defect
`A11Y-004`/`A11Y-005` closed in 0.41.0.

**This is not released.** It is on `main` only, which is why this round
exists before the cut rather than after it.

**Whose it is**: the architect's. `VIEW-003`'s handoff said *"`removeToast`'s
focus restoration has no counterpart once the control never moves focus
in the first place. Delete it; do not port it."* That is true of the
toast *taking* focus and false of the Undo control *receiving* it — which
is the same round's own Tab-order property. The round did exactly what it
was told and said so in its report.

## 2. What to build

**Restore the ladder `NFR-A11Y-002` already names**, in all four scripts
(`dm.js`, `board.js`, `calendar.js`, `plan.js`), when the offer ends
**because the reader acted on it**:

1. the origin — the control that caused the outcome, if it is still in
   the document and focusable;
2. otherwise the first focusable control still inside the same holder;
3. otherwise `main` with `tabindex="-1"`.

**Only when focus was inside the region.** An unconditional restore takes
focus from a reader who had already moved on — that is the same reasoning
`A11Y-004` recorded, and it still holds.

**The other two endings need a decision, not a guess.** The offer also
ends when the next outcome replaces it, and when the reader leaves the
page. Replacement happens while the reader is acting elsewhere, so moving
their focus would be the defect, not the fix. **Do not restore focus on
replacement** — and if the measurement says otherwise on some surface,
stop and report rather than deciding it inside this round.

## 3. What to measure

**All four surfaces**, not one. The architect measured `dm.js` only; the
other three carry the identical two lines, **read and not measured**, and
closing that bound is part of this round.

For each surface: Tab to Undo, activate it with a **real key event**, and
record `document.activeElement` immediately and after the request
settles. A `element.click()` does not prove the keyboard path.

## 4. Acceptance

- `drag-outcome-gate.mjs` gains a **fourth property**: after activating
  Undo, focus is **not** `body` and is where the ladder says. It already
  asserts that Tab *reaches* Undo; this is the half that was missing, and
  it is the point of `§10.57`. The new assertion must fail by name when
  the restore is removed — **plant it and show the failure text**.
- The three existing `drag-outcome-gate` properties keep their names and
  stay true.
- `undo-mousedown-trap.mjs` unaffected; `A11Y-005`'s `mousedown`
  `preventDefault` + explicit `focus()` pair is **not** touched.
- `undo_toast_attachment_scan` extended or left alone, whichever is
  honest — a source scan can see that a restore call exists; it cannot
  see where focus lands. Say which you did and why.
- `DEC-007` from → to, re-derived with the runner.
- The four surfaces' measurements in the report, as a table.

## 5. What this round must not do

- Do not reintroduce the timer, the toast, or a corner container.
- Do not change where the region attaches, what it says, or what Undo
  does.
- Do not amend `requirements.md` — `NFR-A11Y-002` already records this
  divergence and `§10.57` the class, both written by the architect
  2026-11-12.
- Do not touch `/today`, the health block or the workload strip.
