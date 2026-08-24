# Handoffs — RFC 004b, the board (D-2)

Implementation companion for
[RFC 004b](../../accepted/004b-direct-manipulation-board.md), target **0.26.0**.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 004b, and RFC 004's cross-cutting contract above it.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| BOARD-001 | [BOARD-001](./BOARD-001-board-copy-and-parity.md) | The three strings in `board.js`; a guard over `static/*.js`; the board's undo | STATUS-002 |
| REL-0.26.0 | [REL-0.26.0](./REL-0.26.0-release-candidate.md) | Release candidate for 0.26.0 — STATUS-002 and BOARD-001 | BOARD-001 |

## The drag is not in scope, because it already ships

`static/board.js` has done column-to-column drag since before RFC 004 was
written. D-2's sketch describes adding it; a reconciliation before this handoff
found it already there
(`.git-exclude/tasks/architect/010-d2-reconciliation.md`).

What D-2 actually is: the cross-cutting contract the board predates. The item
that matters is that **three user-visible English sentences live inside
`board.js`** — outside the message table and outside every guard this project
has built. `prose_scan` covers Rust; `static/*.js` is out of scope by
construction, and `search.js` is RFC 006's one *named* exclusion. `board.js` was
never named. It was not excluded, it was unexamined.

The guard in §3 is the durable half. The strings are what made it necessary.
