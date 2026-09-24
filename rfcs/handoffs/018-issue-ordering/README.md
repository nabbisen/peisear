# Handoffs — issue ordering

**RFC 0004, by retirement.** `ORD-001` closes substep D-5 by removing what it
was going to build on, rather than by building it. The reasoning is in
`.git-exclude/tasks/architect/021-d5-decision.md`; the owner approved the
amendment of `FR-DM-001` from five surfaces to four on 2026-09-24.

Kept separate from `017` (the sprint-plan backlog's priority sort) because the
two touch different queries and neither waits on the other — though they are
**the same defect twice**: SQL ordering a TEXT column as though its alphabetical
order carried meaning. `017` found it on `priority`; `ORD-001` found it on
`status` while enumerating.

| ID | Link | What | Release |
|---|---|---|---|
| ORD-001 | [ORD-001](./ORD-001-retire-d5-and-order-by-recency.md) | Retires D-5. `position` is removed from the schema, six SELECTs, two INSERTs, three structs and `peisear-core`'s public `Issue` — it was written on insert, never recomputed on a status change, and ordered two surfaces by something nobody chose. The two list queries become `created_at DESC`, which also removes `status ASC` — **alphabetical on TEXT, which put Done at the top of the default issue list.** No test referenced `position`, which is why both defects survived. | 0.38.0 |
| ORD-002 | [ORD-002](./ORD-002-timestamp-ties.md) | `CURRENT_TIMESTAMP` is one-second, and SQLite returns ties **oldest first**, so every `DESC` ordering reads backwards within a tie. Eleven sites; in **three** of them a tie picks a row rather than orders a list — *current capacity* resolves to the **superseded** value after two saves in one second, feeding workload, WIP and health. One rule: every timestamp ordering ends with a `rowid` tiebreak in the matching direction. | 0.38.0 |
| ORD-003 | [ORD-003](./ORD-003-sprint-status-order-and-test-home.md) | `issues_in_sprint` orders `i.status ASC` — TEXT, so alphabetical — and **a sprint's issues read Done first** on both sprint detail and the plan's sprint column. The **third** instance of the shape, after `ORD-001` and `PLAN-003`. Fixed by grouping in lifecycle order via `IssueStatus::lifecycle_rank`, mirroring `PLAN-003`'s `severity_rank` — *not* by removing the term as `ORD-001` did, because a sprint has no board beside it. Also moves the ordering tests into their own file. | 0.38.0 |
| REL-0.38.0 | [REL-0.38.0](./REL-0.38.0-release-candidate.md) | Release candidate — `ORD-001`, `PLAN-003`, `ORD-002`, `CAP-001`, `ORD-003`, plus `DEC-020` putting the specification in the repository. **A migration (`0018`), a breaking API change (`Issue.position`), and six visible ordering changes.** The suite found none of the defects. | 0.38.0 |

