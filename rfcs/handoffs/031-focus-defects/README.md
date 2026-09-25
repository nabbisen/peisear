# Handoffs — focus defects

**Not RFC-governed.** Two defects `A11Y-001` measured, in order of how directly
they harm someone.

| ID | Link | What | Release |
|---|---|---|---|
| A11Y-002 | [A11Y-002](./A11Y-002-a-navigation-key-commits-a-demotion.md) | `<select name="role" onchange="this.form.submit()">` — **an arrow key commits a role change.** A keyboard user moving Admin → Viewer demotes to Member on the way, and lands on `body`. The only control in the product where a navigation key writes to the database. | 0.41.0 |
| A11Y-003 | [A11Y-003](./A11Y-003-undo-is-not-on-the-keyboard-path.md) | The undo toast is appended to the end of `<body>` and lives five seconds: **13 Tab presses to reach it on the issue list, 33 on the board.** `FR-DM-002` requires every direct-manipulation action to have a keyboard equivalent; undo was built on the pointer path only and shipped that way for five releases. **Expects a round trip** — options and costs before any fix. | 0.41.0 |
