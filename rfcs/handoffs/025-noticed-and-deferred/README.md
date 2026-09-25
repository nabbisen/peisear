# Handoffs — noticed and deferred

**Not RFC-governed.** Two things found during other work and deliberately not
folded into it. Grouped because that is what they have in common; they are
otherwise unrelated and can be taken in either order, **after `REQ-001`**.

| ID | Link | What | Release |
|---|---|---|---|
| PERF-001 | [PERF-001](./PERF-001-the-query-planner-never-has-statistics.md) | Nothing in this product runs `ANALYZE` or `PRAGMA optimize`, so SQLite plans every query without statistics — a property nobody chose, which has already shaped two measurements. **Measure first; a recorded "no change wanted" is a fine outcome.** | 0.40.0 |
| GATE-001 | [GATE-001](./GATE-001-the-fixture-completed-sprint-has-no-issues.md) | The completed sprint `SPRINT-005` added to the overflow gate's fixture **has no issues in it** — the fixture's project is personal and only a team project's issues can join a sprint. The gate sweeps that page with an empty list, and a completed sprint's issue list is exactly where a long title would overflow. | 0.40.0 |
