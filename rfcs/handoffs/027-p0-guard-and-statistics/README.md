# Handoffs — a P0 guard, and letting the planner see the data

**Not RFC-governed.** Two unrelated items, both from `REQ-001` and `PERF-001`'s
reports. Either order; **after `LAYOUT-010`**, which is holding a red gate.

| ID | Link | What | Release |
|---|---|---|---|
| CAL-004 | [CAL-004](./CAL-004-the-p0-guard-checks-a-different-property.md) | `FR-CAL-007` is **P0** and its status says a guard asserts *occupancy rate, week-over-week, free-hours totals* are absent. The shipped guard asserts no `%`, no `" of "`, no ratio — a real property and a **different** one, so those concepts could appear in words carrying no quantity and the guard would stay green. The behaviour holds; the check does not cover what it claims. Third instance of `§10.28`'s family, first on a P0. | 0.40.0 |
| PERF-002 | [PERF-002](./PERF-002-let-each-install-decide-its-plans.md) | `PRAGMA optimize` on connection close — **not for the 7×**, but because we cannot predict install shapes and it is the only option that lets each install's own data choose its plans. Settles the three things `PERF-001` did not measure: on-close versus periodic, cost on a warm database, and eight connections rather than one. | 0.40.0 |
