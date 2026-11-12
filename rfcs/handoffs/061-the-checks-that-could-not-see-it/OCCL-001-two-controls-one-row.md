# OCCL-001 — two controls, one row

**Series**: `061`, `0.63.0`'s authorized scope. **Register**: `§10.55`.
**Blocked on**: nothing.

## 1. What is wrong, measured

At **768 px and 800 px** the account menu's clickable box covers the
search field's right edge:

| viewport | 768 | 800 | 860 | 940 | 1024 | 1120 | 1280 | 1920 |
|---|---|---|---|---|---|---|---|---|
| input's right edge − menu's left edge | **+18 px** | **+13 px** | −8 | −8 | −8 | −8 | −8 | −8 |

A positive number is overlap. `document.elementFromPoint` 1 px and 4 px
inside the input's right edge returns the **account control**; at 20 px
inside it returns the input. So a click in that strip opens the menu
instead of focusing search.

**It is invisible for two reasons, and both matter.** The account
control is `btn btn-ghost`, so the overlapping region is transparent
padding and the header *looks* correct — a screenshot confirms it does.
And `overflow-gate` measures **overflow**: two controls can cover each
other without the page scrolling by a pixel, at **768, which is one of
its own five widths**, reporting a green cell.

## 2. What to build

**Re-scoped 2026-11-12: make the overlap impossible, then check for
it.** Controls in one flex row with a gap cannot cover each other; this
pair can only overlap because something refuses to shrink. **Find what
does** — the search field carries a fixed `w-48` — and let the row's own
layout do its job. A fix that adds a magic number to buy 18 px back is
not this round's answer, because the next width will need another one.

**No two interactive controls in the header share pixels, at any width
the gate measures.** The remedy is this round's to choose and justify,
under three constraints:

- only classes already in the shipped stylesheet (`css_scan`);
- every control still measures ≥ 44 × 44 (`NFR-A11Y-007`);
- the five navigation words, the brand and the account menu still sit in
  **one row** at 768 (`NAV-001`'s own threshold measurement).

Say which you chose, what the other options cost, and show the measured
row before and after.

## 3. The guard, which is the point of the round

**An occlusion check**: for each pair of visible interactive controls
whose boxes share a row, assert neither covers the other. Over the
gate's own five widths, on the pages it already visits.

This is the backstop, **not the mechanism** — the layout is what should
make overlap impossible. An overflow check and an occlusion check answer
different questions, and the project currently owns only the first,
which is why a defect sat at a measured width behind a green cell.

**Where it lives is yours to argue**: a fourth property inside
`overflow-gate.mjs`, or its own gate beside it. State the reason.

## 4. Acceptance

- The overlap is **0 or negative at every gate width**, measured, with
  the table above re-run and reproduced.
- `elementFromPoint` 1 px inside the search field's right edge returns
  the **search field**, at 768 and 800.
- `overflow-gate` still 120/120; `disclosure-marker-gate` unaffected.
- The occlusion check **planted** — reintroduce the overlap and show it
  failing by name, with the pair named in the message.
- `DEC-007` from → to if any Rust test count moves.

## 5. What this round must not do

- Do not change the navigation's words, order, or collapse threshold.
- Do not rebuild the stylesheet (`DEC-051`/`ASSET-001`: regeneration is
  manual, pinned and checksummed, and is not this round's business).
- Do not amend `requirements.md`; `§10.55` records this already.
