# Handoffs — sprint membership

**Not RFC-governed.** Changes to `sprint_issues` that alter a sprint's record
after it has completed. Grouped by consequence rather than by cause: one is a
race, one needs no concurrency at all.

| ID | Link | What | Release |
|---|---|---|---|
| SPRINT-001 | [SPRINT-001](./SPRINT-001-a-completed-sprint-changes-after-the-fact.md) | `assign_issue`'s **unassign** branch has no status check, so an issue can be removed from a **completed** sprint — and `burndown` reads `sprint_issues`, so that **retroactively changes what the sprint recorded**. Twelve lines below it, the assign branch refuses the same thing with the comment *"historical sprint summaries should remain stable"*. **One click, no race**, which is why four concurrency surveys missed it. Also `plan_remove`, which is `plan_add`'s mirror and does need a race. | 0.39.0 |
