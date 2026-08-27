# Handoffs — RFC 012, touch target conformance

Handoffs implementing [RFC 012](../../accepted/012-touch-target-conformance.md).

| ID | Link | What | Release | Depends on |
|---|---|---|---|---|
| TT-001 | [TT-001](./TT-001-audit.md) | Step 2 — audit the 139 controls: mechanism per surface, and the adjacency map. **No code changes.** | 0.30.0 | step 1 (the amended requirement) |
| TT-002 | [TT-002](./TT-002-apply.md) | Step 3 — apply. 136 `Grow` + 3 checkbox `<label>` wraps; one home for the 44 px fact; the input/select `line-height` question named rather than assumed. | 0.31.0 | TT-001 |
| TT-003 | *(not yet written)* | Step 3 — `touch_target_scan` enforces the size clause across the tree. Cannot land before the tree can pass it. | 0.31.0 | TT-002 |

**Step 1 is the architect's own work and has no handoff** — `NFR-A11Y-007` and
external design `§5.7` carry the amended rule, and `DEC-049` records it.
