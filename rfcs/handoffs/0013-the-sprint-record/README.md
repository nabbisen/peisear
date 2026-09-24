# Handoffs — the sprint record

**RFC [0013](../../accepted/013-the-sprint-record.md)**, accepted 2026-09-24,
`DEC-054`. Capturing what a sprint reported at completion, instead of freezing
what it contains.

| ID | Link | What | Release |
|---|---|---|---|
| SPRINT-004 | [SPRINT-004](./SPRINT-004-capture-the-record-and-discard-it-on-reopen.md) | `complete` captures the summary and burndown into two tables; `reopen` discards them; reads branch on status. **Reverts `SPRINT-001`'s unassign refusal** — it blocked carry-over and protected one of four inputs — and **folds in `SPRINT-003`**, because capture without discard is a trap. Migration `0019` backfills existing completed sprints, knowingly wrong by the drift already suffered. | 0.39.0 |
| SPRINT-005 | [SPRINT-005](./SPRINT-005-the-privacy-gate-reads-live-data-over-a-captured-record.md) | **A privacy defect `SPRINT-004` exposed.** `distinct_contributors` decides whether a completed sprint's *captured* trajectory is shown, from **live** membership and status — and it can **open**: an issue left in a completed sprint and finished later raises the count, so a one-contributor sprint can cross the `NFR-PRIV-007` floor and start disclosing a per-person trajectory nobody decided to disclose. Captures the basis, not the verdict. Also the reopen flash and the gate fixture's completed sprint. | 0.39.0 |

