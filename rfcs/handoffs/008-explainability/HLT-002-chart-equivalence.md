# HLT-002 — Chart equivalence, and three ways to leak what 0.28.0 suppressed

**Issued by**: Architect
**Date**: 2026-08-27
**Priority**: P1 — `NFR-A11Y-003`; with `HLT-001` it closes `§10.4` and
**Definition of Done item 3**
**Governing RFC**: [008](../../done/008-explainability.md) §5
**Depends on**: `HLT-001`, closed. Independent of it in code.

---

## 1. What `NFR-A11Y-003` asks, and what already exists

Three things per chart. **Reconciled 2026-08-27 — the first exists for both,
and the status says "Not implemented":**

| | Burndown | Completed-work bars |
|---|---|---|
| One-sentence summary label | **yes** — dynamic, range and max | **weak** — *"Bar chart of recent sprint outcomes"* |
| Two-to-three sentence textual summary | no | no |
| Tabular equivalent | no | no |

**The bar chart's label names what kind of chart it is, not what it shows.**
That is an accessible *name*; the requirement asks for a *summary*. Rewrite it
to describe the data.

**A caption is not a summary.** The completed-work chart already carries
*"Each pair of bars: completed (filled) and carried over (light)"* — that
explains the **encoding**. `NFR-A11Y-003` wants the **finding**. The caption
stays; it does a different job.

## 2. The table is a second rendering, not a reconstruction

Both charts have their data in hand at render time — `Vec<BurndownPoint>` and
`Vec<(Sprint, SprintSummary)>`. **The table renders those values. It does not
recompute them, re-query them, or re-derive them.**

If the table and the chart can ever disagree, the implementation is wrong.
This is `HLT-001`'s one-authority rule in a smaller place, and `QA-019`'s
before that.

The bars plot `completed_points` and `carried_over_points`; the burndown plots
`cumulative_committed` and `cumulative_completed` per day. **Those are the
columns.**

## 3. Three ways to undo 0.28.0, in the order you will meet them

`QA-017` suppressed the burndown and the median line below two distinct
contributors, because an aggregate that resolves to one person is that person's
data. **A tabular equivalent is the easiest possible way to put it back.**

**3.1 — The burndown's table goes with its card.** The card is already gated:

```rust
let burndown_card = (!matches!(sprint_status, SprintStatus::Planned)
    && !burndown.is_empty()
    && show_trajectory)
    .then(|| render_burndown(burndown));
```

Put the table **inside** `render_burndown`. Then it cannot outlive the chart,
and no future edit can separate them. A table built beside the card, gated by a
second copy of the same condition, is two homes for one predicate.

**3.2 — The velocity table keeps its bars and must lose its median row.** Below
two contributors the bars still render and only the median line is suppressed.
**A table row labelled "median" discloses exactly the computed statistic the
line was suppressed to withhold** — and it is the natural thing to include,
because the median is "part of the chart's data".

Gate that row on `show_median`, the same flag the line uses. Not a second
predicate; the same one.

**3.3 — The summary is prose and can say what the table does not show.** A
burndown summary describes a trajectory — which is the suppressed thing, so it
goes with the card. A velocity summary must not state the median when the
median is suppressed.

*"Completed points across the last five sprints: 8, 13, 5, 11, 9."* — fine.
*"The median is 9."* — not, when suppressed.

**Of the three, 3.2 is the one I expect to be got wrong.** It is the only one
where the chart stays and something inside it must not.

## 4. What the summary should say

Two to three sentences stating **what the data shows**, from the values already
in hand. Not the encoding, not an interpretation, and not a judgement —
`NFR-LANG-002`'s Watch ceiling and §1.7 both apply, and `en.rs` now carries a
note about the two rules and where each is enforced. **Read it before drafting;
`HLT-001` round 2 lost a draft to exactly this.**

No trend language, no "improving", no "declining". State the numbers and their
shape.

## 5. Not in scope

- **No new chart, no change to either chart's rendering** beyond the bar
  chart's accessible name.
- **No change to `QA-017`'s predicates.** Reuse `show_trajectory` and
  `show_median`; do not recompute contributor counts.
- **No history.** `FR-HLT-007`'s third limb stays deferred.

## 6. Escalate rather than deciding

- **If the table cannot be rendered from the same values the chart plots**
  without a second query or a recomputation, stop and report the shape.
- **If putting the burndown's table inside `render_burndown` is awkward** —
  layout, borrow, whatever — stop rather than moving it outside the gate. The
  gate is the point.
- If the bar chart's accessible name cannot describe its data without naming
  the median, say so; that would make the name itself a §3.2 case.

## 7. Tests

| # | Check |
|---|---|
| 1 | Each chart renders a `<table>` whose cells equal the plotted values |
| 2 | Two contributors: the burndown's table renders |
| 3 | **One contributor: the burndown's table is absent along with the chart** |
| 4 | **One contributor: the velocity table renders, and has no median row** |
| 5 | One contributor: no summary sentence states a median |
| 6 | The bar chart's accessible name describes data, not chart type |

**3, 4 and 5 are the privacy tests** and should be written before the feature,
as `HLT-001`'s was. Plant each against its predicate's removal.

## 8. Acceptance

1. Tables for both charts, rendered from the plotted values.
2. Summaries stating the finding, within §1.7 and the Watch ceiling.
3. The bar chart's name rewritten.
4. §3's three cases each covered by a test written first and planted.
5. No second predicate anywhere — `show_trajectory` and `show_median` reused.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 9. Required review-request format

Workflow §9.2. State plainly where each table's gate lives and why. Each plant
transcript separately.
