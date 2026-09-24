# Handoffs — the sprint record

**RFC [0013](../../accepted/013-the-sprint-record.md)**, accepted 2026-09-24,
`DEC-054`. Capturing what a sprint reported at completion, instead of freezing
what it contains.

| ID | Link | What | Release |
|---|---|---|---|
| SPRINT-004 | [SPRINT-004](./SPRINT-004-capture-the-record-and-discard-it-on-reopen.md) | `complete` captures the summary and burndown into two tables; `reopen` discards them; reads branch on status. **Reverts `SPRINT-001`'s unassign refusal** — it blocked carry-over and protected one of four inputs — and **folds in `SPRINT-003`**, because capture without discard is a trap. Migration `0019` backfills existing completed sprints, knowingly wrong by the drift already suffered. | 0.39.0 |
