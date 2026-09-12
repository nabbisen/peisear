# Handoffs — layout defects found by inspection

**Not RFC-governed.** These are plain defect fixes for things found by looking at
the rendered product with a browser, recorded in
`.git-exclude/tasks/architect/013-browser-inspection-findings.md` (2026-09-06).

They are separated from RFC 011 deliberately. **RFC 011 asks whether browser
verification belongs in CI. These are defects a browser found once.** The second
does not depend on the first, and folding them together would make a defect fix
wait on an open decision.

| ID | Link | What | Release |
|---|---|---|---|
| LAYOUT-001 | [LAYOUT-001](./LAYOUT-001-navbar-overflow.md) | ✅ Done. Authenticated pages scrolled horizontally when the signed-in email was long and unbreakable — a flex item's content-based minimum width, not the dropdown's position. | 0.32.0 |
| LAYOUT-002 | [LAYOUT-002](./LAYOUT-002-project-toolbar.md) | Project detail overflows 6 px at 390 px: the view/action toolbar cannot wrap. **Not caused by RFC 012** — measured. | 0.32.0 |
| LAYOUT-003 | [LAYOUT-003](./LAYOUT-003-board-column-overflow.md) | The board's columns overflow 33 px at 320 px on an unbreakable issue title — `LAYOUT-001`'s mechanism again, below the gate's narrowest width. | 0.33.0 |
| LAYOUT-004 | [LAYOUT-004](./LAYOUT-004-unbreakable-text-two-shapes.md) | Three more surfaces overflow on unbreakable text — in **two shapes** with non-interchangeable remedies. Issue detail overflows at widths the gate already sweeps. | 0.33.0 |
| LAYOUT-005 | [LAYOUT-005](./LAYOUT-005-calendar-and-team-detail.md) | The two sites `LAYOUT-004` reported and did not fix — project calendar (shape B) and team detail (shape A) — and the two gate pages that would have seen them. | 0.33.0 |
| LAYOUT-006 | [LAYOUT-006](./LAYOUT-006-navbar-at-320.md) | Every page overflows a 320 px phone when the display name is 21 characters — a third mechanism, a flex row's content minimum. Sequenced before `BROWSER-002`'s 320 px. | 0.33.0 |
| LAYOUT-007 | [LAYOUT-007](./LAYOUT-007-display-name-is-user-text.md) | The display name is user text too: `/today` and `/settings` quote it in a subtitle that cannot break, and the gate's fixture name has a space at every point. Two shape-B fixes and a fourth fixture property. | 0.33.0 |
| LAYOUT-008 | [LAYOUT-008](./LAYOUT-008-sprints-and-a-third-shape.md) | The sprint pages (one overflowing on a desktop), the issue form's workload chips, and a **third shape** — a container that sizes itself to its text, where neither known remedy works and `overflow-wrap: anywhere` does. Four more gate pages and a sprint in the fixture. **`REL-0.33.0` waits on it.** | 0.33.0 |
| LAYOUT-009 | [LAYOUT-009](./LAYOUT-009-header-rows-do-not-wrap.md) | Sprint detail's title reads **eight lines deep at 320 px** with an ordinary name, and issue detail's four: header rows that cannot wrap squeeze the title instead. `LAYOUT-002`'s mechanism on a header. Passes the overflow gate; not `§10.25`'s class. | 0.34.0 |
