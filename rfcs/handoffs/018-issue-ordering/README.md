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
