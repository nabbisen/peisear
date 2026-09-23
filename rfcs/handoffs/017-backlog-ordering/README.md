# Handoffs — backlog ordering

**Not RFC-governed.** Defect fixes in how backlogs are ordered, found while
reviewing D-5's design against the code.

Kept separate from `004c` (the sprint-plan drag) and from any future `0004e`
(the issue-list reorder, awaiting an owner decision) on the same principle
`013` was separated from RFC 011: **a defect in shipped behaviour should not
wait on an open question about a feature.** D-5 asks whether a manual backlog
order is wanted; `PLAN-003` fixes an ordering that is already shipped and
already wrong.

| ID | Link | What | Release |
|---|---|---|---|
| PLAN-003 | [PLAN-003](./PLAN-003-backlog-priority-sorts-alphabetically.md) | The sprint-plan backlog orders `priority DESC` on a TEXT column, so SQLite sorts it alphabetically — `urgent medium low high`, with **`high` last, below `low`**. Since `PLAN-001`; no test pins the order. The one SQL priority ordering in the workspace; every other is Rust with an explicit rank, which this change makes the single one. | 0.38.0 |
