# Handoffs — the plan page's filter bar

**Not RFC-governed.** A layout defect the gate could never have found, because
the surface was swept with empty option lists.

| ID | Link | What | Release |
|---|---|---|---|
| LAYOUT-010 | [LAYOUT-010](./LAYOUT-010-the-plan-page-filter-selects.md) | The sprint plan's project and assignee `<select>`s size to their longest option — 729 px and 626 px inside a 320 px viewport, 425 px of overflow. `§10.25`'s shape A on an input element. **The gate has swept that page since `BROWSER-001` and could not have failed on it**: the fixture's team had no projects, so both lists held only *All projects* and *Anyone*. Lands with `GATE-001`'s fixture, one round. | 0.40.0 |
