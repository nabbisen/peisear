# A11Y-004 — the toast beside the control, and a restore when it ends

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.41.0
**Related requirements**: `NFR-A11Y-002` (as rewritten at 0.41.0).
**Source**: `A11Y-003`'s measured shapes; ruled in
`.git-exclude/reviewed/A11Y-003-shapes-review.md`. **Depends on**: nothing.

**Shape 2 plus shape 1's second half. Shape 1 is rejected** — a second Enter on
a status button would have pressed Undo and silently reversed the change, which
is a worse defect than the reach problem and would have shipped looking like a
clean win.

---

## 1. The change

**1.1 — The toast goes beside the control that was acted on**, in DOM order
immediately after it, so it is the next Tab stop. It stays drawn where it is
(`position: fixed`, bottom-right) — **this is a DOM-order change, not a visual
one**.

**1.2 — When the toast is removed, pressed or expired: if focus is inside it,
return focus to the acted-on control.** If focus is elsewhere, do nothing —
that is the whole reason shape 1 was rejected, and the condition is what keeps
this from taking focus from anyone.

**Both, in all four `showUndoToast` functions** — `dm.js`, `board.js`,
`plan.js`, `calendar.js`. `A11Y-003` measured two of the four and **read** the
plan and calendar ones as near-duplicates; **confirm that before you change
them**, and say if they differ.

## 2. What must hold

- **Reach, measured**: issue detail and list to **3** presses or fewer. The
  board's pointer-then-keyboard count stays ~13 and **that is accepted** — the
  board's keyboard route has no toast at all (`A11Y-003` §0.1), which is a
  separate gap recorded against `FR-DM-005` and not this handoff's.
- **Both endings**: pressed, and expired while focused. **Focus must not be
  `body`** — `NFR-A11Y-002` forbids it in terms.
- **A second Enter must not undo.** Assert it: press the status button twice
  and confirm the change stands. That is the rejected shape's failure mode and
  the reason this one exists.
- **The origin may have moved.** On the board the card is relocated by the
  drop; `A11Y-003` measured `focus()` still working. **If the origin is gone
  entirely, focus must land somewhere named** — say where and why.
- **Placement**: the toast must not be clipped, and must not become a child of
  a form it should not submit. `A11Y-003` put a prototype inside a `<td>` and a
  card. **Check 320 px**, which it did not.

## 3. Verification

- **`§10.15`'s two required evidence runs.** This is `static/*.js`, executed by
  no test, and this change touches the DOM position of an element. **The
  before-and-after rendering check is not optional here** — `CAL-003`'s
  `h-full` is what happens when markup moves and only the overflow number is
  watched. The no-reload sequence run as well.
- **Tab counts before and after**, all four surfaces, in one run — the
  measurement that found this closes it.
- **The pointer path unchanged**: no ring appears where none did, the toast is
  drawn in the same place, `elementFromPoint` still finds Undo.
- `JS-003`'s outcome classification and the announcements untouched.
- **The overflow gate** — an element changes parent on pages it sweeps.
- `DEC-007` reported, last recorded **369**; `fmt`, `clippy`, three consecutive
  workspace runs.

## 4. Escalate rather than deciding

- **If the four `showUndoToast` functions are not near-duplicates** (§1).
- **If beside-the-control cannot be done on one of the four** without the toast
  being clipped or landing inside a form. Report the surface; do not special-case
  it silently.
- **If the restore cannot be made conditional on focus being inside the toast**
  — an unconditional restore is shape 1's theft in a different costume.
- **If 320 px placement forces a visual change.** That is a design decision and
  it is mine.

## 5. Exit condition

Undo is three Tab presses away on the issue list and issue detail; focus never
lands on `body` when a toast ends; pressing Enter twice on a status button does
not undo; the pointer path is unchanged; both `§10.15` evidence runs recorded.
