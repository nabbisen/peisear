# Handoffs — the fixture's unrendered branches

**Not RFC-governed.** `§10.32`: a page can only overflow on content it renders,
and the gate's fixture has three times been found not to render the branch a
defect was behind. This closes the branches now known — **not all of them**.

| ID | Link | What | Release |
|---|---|---|---|
| GATE-002 | [GATE-002](./GATE-002-make-the-fixture-render-what-the-pages-can-render.md) | Adds an issue-detail page with an assignee badge (**95 → 100**, a page added rather than a baseline moved), calendar blocks on both axes, and a second assigned card. Each **asserted present by the fixture**, since a green cell on a page that rendered nothing is the whole subject of `§10.32`. Expect red. | 0.40.0 |
| GATE-003 | [GATE-003](./GATE-003-two-calendar-layouts-the-gate-has-never-visited.md) | The gate sweeps the calendar's **week view only** — `?view=day` and `?view=month` are distinct layouts and neither has ever been visited. **The day view is where `CAL-003`'s `h-full` collapsed a block to 20 px**, found by a rendering check because the gate could not see it; it still cannot. Four URLs, 100 → 120 cells. | 0.40.0 |
| GATE-004 | [GATE-004](./GATE-004-the-inbox-sweeps-its-empty-state.md) | `/inbox` is swept with **no notifications**, so it renders its empty state at all five widths while notification rows carry issue and project text. **The last branch added reactively** — after this, `§10.32` says an unrendered branch is recorded and not chased unless a defect appears behind it. | 0.40.0 |

