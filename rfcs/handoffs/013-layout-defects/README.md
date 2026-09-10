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
