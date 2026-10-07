# A11Y-009 — an invisible 44×44 target, and a second `<main>`

**Issued by**: Architect
**Date**: 2026-10-07
**Target release**: 0.42.0
**Source**: both raised by the dev team in `A11Y-006` — §"Where it could have
gone wrong" and §"Left for you". Ruled in
`.git-exclude/reviewed/A11Y-006-review.md` §3 and §4.
**Depends on**: `A11Y-006`, landed. **Two unrelated items; §1 needs a
measurement before anything changes.**

---

## 1. Is the hidden skip link hit-testable?

`grow()` appends `min-h-11 min-w-11` unconditionally, and an explicit
`min-width`/`min-height` beats `sr-only`'s `width:1px;height:1px`. **So the
hidden skip link is a 44×44 box at `(-1,-1)`–`(43,43)` on every page**, clipped
out of painting by `clip: rect(0,0,0,0)`.

`A11Y-006` measured a click *outside* it. **Measure one inside it**: a real
`Input.dispatchMouseEvent` at roughly `(20,20)` on a page whose top-left corner
holds nothing else.

- **If nothing happens** — `clip` removes it from hit-testing — then the
  arrangement is correct as it stands. **Report that and change nothing**; it
  becomes a recorded fact rather than a worry, and say so in `layout.rs` beside
  the component so the next reader does not re-ask.
- **If the skip link activates**, every page has an invisible target in its
  corner, and it needs fixing.

**If it needs fixing, the tension is mine and here is the ruling.**
`DEC-050`'s floor exists so a *usable* control is big enough to hit. **A
visually hidden control is not a control yet** — it becomes one when focus
reveals it. So the floor applies **in the focused state only**.

- Prefer **`focus:min-h-11 focus:min-w-11`** over `grow()`'s unconditional
  pair, so the hidden box collapses to `sr-only`'s 1×1 and the visible one
  still clears 44.
- **`touch_target_scan` will object**, because the declaration it looks for
  will no longer be unconditional. **Do not weaken the scan.** Use its
  existing named-exclusion mechanism — the shape `TT-004` established — with
  the reason being this ruling, and **heed `§10.28`**: the scan's doc must not
  claim a coverage it no longer has.
- **Re-measure the focused box at 320 px** afterwards: it was 188.9×44 and must
  still clear 44.

## 2. Two `<main>` elements on the sprint plan

`components/sprint_plan.rs:157` wraps the backlog/sprint grid in a second
`<main class="grid …">`, purely as a layout container. `AppShell` already
renders the real landmark, so the page has **two `main` landmarks**, which is
invalid ARIA.

**Pre-existing, and it is not `A11Y-006`'s fault** — but the skip link now
lands on the first one, which makes "there is exactly one" something the
product depends on rather than merely ought to have.

- **Change the inner one to a `<div>`.** It carries layout classes and no
  landmark intent.
- **Check for others**: any element in `components/` using `main` as a layout
  wrapper. **Report the count** rather than fixing only this one.
- The skip link must still land on `AppShell`'s `<main>` on that page —
  measure it, since this is the page where it was ambiguous.

## 3. Verification

- **§1's measurement, reported either way**, and the resulting state of
  `layout.rs`'s comment.
- **If §1 changes anything**: the hidden box's rect, the focused box's rect at
  320 px, `scrollWidth` contribution still 0 at all five gate widths, and the
  scan's exclusion with its reason.
- **§2**: `document.querySelectorAll('main').length === 1` on the sprint plan
  and on the pages `A11Y-006` measured; the skip link still reaching `main`
  there.
- **The overflow gate** — §1 touches a box on every page and §2 changes an
  element on one. **Run it and report the count.**
- `DEC-007` reported, last recorded **369**. `fmt`, `clippy`, three consecutive
  workspace runs.

## 4. Escalate rather than deciding

- **If §1's measurement is ambiguous** — different answers in different
  conditions. Report what you saw; do not pick the convenient one.
- **If `touch_target_scan` cannot take a named exclusion** for this without
  weakening what it covers.
- **If the inner `<main>` turns out to carry landmark intent** after all.
- **If more than one other component uses `main` as a wrapper** (§2).

## 5. Exit condition

Either a recorded measurement showing the hidden skip link is not clickable,
or a focused-only touch-target floor with the scan's exclusion reasoned; and
exactly one `main` landmark per page, with the skip link measured reaching it
on the sprint plan.
