# GRAM-002 — every counted sentence, not the five

**Series**: `061`, `0.63.0`'s authorized scope. **Register**: `§10.59`.
**Blocked on**: nothing.

## 1. What is wrong

Five English strings still disagree with themselves at one:

| key | at `count == 1` |
|---|---|
| `IssuesCountText` | *"1 issues"* |
| the sprint committed summary | *"… committed across 1 issues"* |
| the velocity chart's summary | *"Completed points across the last 1 sprints"* |
| `TrendDirectionLabel`'s announced form | *"trend: up by 1 points"* |
| the capacity announcement | *"Capacity 1 points, period …"* |

`GRAM-001` fixed the sixth — the one `VIEW-001` had promoted to a
project screen's headline — and left these by instruction.

**Fixing five strings is the smaller half of this round.**

## 2. The decision, which is the architect's and is made here

**`MessageKey::all()` gains a singular sample for every variant that
carries a count.** Today it samples each variant once — e.g.
`IndicatorExplanationWipCompliance { count: 2 }` — so no test can reach
the form that disagrees without constructing variants by hand, and a
hand-maintained list would not notice a new count-bearing variant
arriving later.

With a singular sample in `all()`, **the population pin does the work**:
adding a count-bearing variant changes `MessageKey::all()`'s own
asserted population, and whoever adds it must add its singular sample to
make the pin true again. That is the same mechanism `page_title_scan`
already relies on, pointed at a different property.

`page_title_scan`'s population assertion moves from **715** to whatever
the new total is — **re-measured with the runner, in the same commit**,
never adjusted to make a test pass.

## 3. What to build

- The five corrections. Numerals stay; nouns and verbs agree.
- The singular samples in `all()`.
- **The guard**: render every entry of `MessageKey::all()` and assert
  none reads as a numeral `1` followed by a plural noun, with a **named
  allowlist** for the legitimate cases — `"1 pt"` is a unit, not a
  plural, and there may be others. State the allowlist's size and each
  entry's reason, the way `css_scan` states its two.

## 4. What to measure, and report as a table

The **real population of count-bearing variants**, read from the enum
definition rather than from this handoff's list of five. If it is larger
than six, say so and fix what you find — this handoff's table is what
the architect measured on 2026-11-12, not a guarantee.

Report: variants carrying a count, how many `all()` now samples at 1,
allowlist size, and `DEC-007` from → to.

## 5. Acceptance

- Each of the five asserted **byte-exact at 1 and at 2**, not by a
  substring on the shared tail — that is why three existing tests could
  quote one of these sentences for releases without seeing it.
- `count == 0` checked and reasoned about per string, not assumed:
  *"0 issues"* is correct English, *"0 issue"* is not.
- The guard **planted**: revert one correction and show the guard
  failing by name.
- `peisear-i18n`'s own suite including the fixture locale; any existing
  test that quotes one of these five updated with a stated reason after
  checking which count its fixture drives.
- `DEC-007` from → to, and `MessageKey::all()`'s pin re-measured.

## 6. What this round must not do

- Do not restructure the message table, add a pluralisation library, or
  introduce a general plural-rules mechanism. **English agreement in
  this locale's own text is the whole scope**; a locale that needs more
  than two forms is a different decision, and `FR-I18N` is not amended
  here.
- Do not amend `requirements.md`; `§10.59` records this already.
