# Handoffs — reopening a sprint

**`DEC-053`, owner-accepted 2026-09-24.** A feature, and the answer to the gap
`SPRINT-001` and `SPRINT-002` open by closing `FR-SPR-004` properly: with all
three routes shut, a sprint completed by mistake can never be corrected.

Reopen makes the correction **a deliberate, visible state change** rather than
a silent edit to a finished record — which is the whole reason it is preferred
over leaving history editable.

| ID | Link | What | Release |
|---|---|---|---|
| SPRINT-003 | [SPRINT-003](./SPRINT-003-reopen-a-completed-sprint.md) | An administrator can return a completed sprint to `active`, clearing `completed_at`, refused while the team has another active sprint (`FR-SPR-002`, same rule and same atomicity as `start`). A handoff rather than an RFC because all eight design decisions follow from existing patterns — each is named with its source. **Ships with or after `SPRINT-002`.** | 0.39.0 |
