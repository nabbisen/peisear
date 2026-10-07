# A11Y-007 — two charts whose series are 1.77 : 1 apart

**Issued by**: Architect
**Date**: 2026-10-07
**Target release**: 0.42.0
**Related requirement**: **`NFR-A11Y-010` — Not met**, added 0.42.0.
**Depends on**: nothing. **After `A11Y-006`.**

---

## 1. What is measured

`A11Y-001`, converting the source `oklch` values itself and saying to treat
them as ±0.1:

| chart | series separation | worst against background |
|---|---|---|
| burndown (committed vs completed) | **1.77 : 1** | committed line vs white **2.65 : 1** |
| completed-work bars (completed vs carried) | **2.09 : 1** | carried bar vs white **1.86 : 1** |

`NFR-A11Y-010` requires **3 : 1**. Nothing here reaches it.

**This is not a colour-blindness problem and the handoff must not be written as
one.** Both charts are single-hue (240); hue carries nothing, so
`NFR-A11Y-004` is satisfied and was wrongly recorded otherwise until 0.42.0.
**The reader this fails is someone with low contrast sensitivity**, and the
failure is that two pale blues are the same pale blue to them.

## 2. What to do

**Raise the separations to 3 : 1 or better** — series against series, and each
series against its background.

- **Measure first, then choose.** Produce the candidate values and their ratios
  **before** changing anything, and report them. `A11Y-001`'s converter is in
  its evidence; **use a second method to cross-check** at least one value,
  because every number in this handoff rests on one conversion.
- **Patterning is allowed and is not required.** A dash or a marker on one
  series would separate them regardless of contrast, and the median line
  already uses `stroke-dasharray`. **If you propose patterning instead of
  contrast, say what it buys over a lightness step** — my reading is that
  contrast is the thing the requirement asks for and patterning is a second
  mechanism, but a measured argument beats my reading.
- **Stay single-hue.** Introducing a second hue would satisfy this requirement
  and break `NFR-A11Y-004`, which is currently Met. Lightness and chroma are
  the dials.
- **The vendored stylesheet is not to be edited** (`DEC-051`). These values are
  in the component; confirm that before you start.
- **`NFR-A11Y-003`'s tabular equivalents stay exactly as they are.** They
  answer a different requirement and `A11Y-001` was right that they do not
  close this one.

## 3. Verification

- **Every ratio, before and after**, for both charts: series-to-series and each
  series to its background. **All ≥ 3 : 1 after.**
- **Cross-checked by a second conversion method** on at least one value (§2).
- **The charts still render** at the gate's five widths, and the median line is
  still distinguishable from both series.
- **`contrast_scan` is unaffected** — it is `NFR-A11Y-005`'s text rule and this
  is not text. **Say that it passed rather than assuming it is unrelated.**
- `DEC-007` reported. A test pinning the ratios is worth one **if it can be
  written without re-implementing a colour-space conversion in the suite** —
  if it cannot, say so; an approximate test of an exact property is worse than
  none.
- `fmt`, `clippy`, three consecutive workspace runs, and **the overflow gate**
  if any markup changes.

## 4. Escalate rather than deciding

- **If 3 : 1 cannot be reached** within one hue without making a series look
  like the median line or like disabled text.
- **If the two conversion methods disagree** by more than the ±0.1 `A11Y-001`
  claimed.
- **If raising contrast makes the chart read as evaluative** — a dark bar
  against a pale one can look like a judgement, and `FR-SPR-003` and `§1.7`
  both guard against that. **This is the one place where the accessible answer
  and the product's tone could conflict**; if they do, bring it to me.

## 5. Exit condition

Both charts separate their series at 3 : 1 or better, and each series from its
background, within a single hue; every figure measured twice; the tabular
equivalents and the tone unchanged.
