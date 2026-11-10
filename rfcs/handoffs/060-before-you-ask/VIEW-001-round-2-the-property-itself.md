# VIEW-001 round 2 — the property itself

**Series**: `060` (unit three), `0.62.0`'s authorized scope.
**Follows**: `VIEW-001` (`87e0fb0`), closed with one finding.

## 1. The finding

Round 1 built the collapse and proved what the collapsed line *says*
and which indicator it chooses. **It did not pin that the block is
collapsed at all.** Planted on the merged tree — `open=true` on the new
outer `<details>`, which is precisely the defect the round removes:

```
cargo test -p peisear-web --lib                        246 passed, 0 failed
cargo test -p peisear-web --test health_explainability  11 passed, 0 failed
cargo test -p peisear-web --test touch_target            6 passed, 0 failed
```

Green, with the health block expanded by default. The two new
integration tests assert the headline's content and its order relative
to the nested heading; both hold open or closed.

**The assertion already exists for the sibling screen.**
`today_panel.rs`'s `today_folds_rhythm_panel_by_default` pins exactly
this property for `/today`, and `FR-PER-007`'s acceptance cites it by
name. Round 1 carried the discipline across and left the guard behind.

## 2. The second half, measured by round 1 itself

Round 1 was told to measure `WorkloadStrip`'s place rather than prefer
one, and it did: hiding it as well moves the first work a further **90 px**
at 1280 and **124 px at 390**. It then correctly declined to build that,
because round 1's own §5 forbade collapsing anything else.

**It is in scope now.** The rule this series implements is *analysis
about the work is one line until the reader asks for it*; the workload
strip is analysis about the work, on the same screen, occupying the same
space. Excluding it from round 1 was a bound on that round — one change,
one measurement — not a boundary of the rule.

## 3. What to build

1. **The guard.** A test that the project screen's health block is
   collapsed by default, in the shape `today_folds_rhythm_panel_by_default`
   already uses: the labels are present in the HTML so a screen reader
   can find them and a reader can open them, and the `<details>` carries
   no `open`. Put it where the sibling lives — a project-screen test
   binary — not inside the unit tests, because the property is about the
   rendered page.
2. **The same guard for `WorkloadStrip`**, written at the same time as
   the strip's own collapse, never after.
3. **`WorkloadStrip` collapses by the same rule**: one line until asked
   for. Its line is **not** a per-person list — that is the expanded
   content. What the line says is this round's own proposal, in its
   report, with the reason; the architect accepts or changes it, as with
   round 1's precedence chain. **Do not invent a number for it**: a
   count of people over capacity is a score by another name, and the
   screen already refuses one (`FR-HLT-008`'s reasoning, applied by
   analogy rather than by citation — it governs the health composite,
   not this).

## 4. Acceptance

- **Both plants, shown red then green, and they are the round's own
  subject**: `open=true` on each of the two blocks must fail the new
  guards **by name**.
- The before/after measurement re-run and reported at both widths for
  both views, as round 1 did — and reported as **the collapsed height,
  constant**, alongside the delta. Round 1's `−48 px` was true and
  understated what changed: the default was 92 px on that fixture and is
  44 px now, while the same block fully open is **560 px at 1280 and
  865 px at 390**. A delta measured on a near-healthy fixture is not the
  property; the constant is.
- Every existing health and workload test green and unchanged, including
  `basis_route`'s five.
- `touch_target_scan` on the second disclosure control.
- `DEC-007` from → to, measured with the runner.

## 5. What this round must not do

- **Do not touch `/today`**, and do not collapse any of the other
  eleven ungoverned `<details>` — the census stays closed.
- **Do not write `external-design §5.10`** or amend a requirement; the
  architect states the rule once, when this series closes.
- Do not change any indicator's or workload figure's computation,
  wording, or basis.
