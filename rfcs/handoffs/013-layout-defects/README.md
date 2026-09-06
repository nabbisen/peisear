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
| LAYOUT-001 | [LAYOUT-001](./LAYOUT-001-navbar-overflow.md) | Every authenticated page scrolls horizontally by ~17 px, at every viewport width, because the closed account dropdown still occupies layout. | 0.32.0 |
