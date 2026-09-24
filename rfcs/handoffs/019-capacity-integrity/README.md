# Handoffs — capacity integrity

**Not RFC-governed.** A concurrency defect in `user_capacities`, found while
measuring `ORD-002`'s premise.

| ID | Link | What | Release |
|---|---|---|---|
| CAP-001 | [CAP-001](./CAP-001-overlap-check-is-not-atomic.md) | `insert` and `update` check for an overlapping capacity period and then write, on two pooled connections with **no transaction between them**, so concurrent saves both pass the check — twelve at once stored six overlapping rows. The module comment says the window "is zero in practice" under WAL, which is **false**: WAL serializes write transactions, not a read-then-write across connections. Migration `0009` delegates the invariant to an application check that does not hold it. | 0.38.0 |
