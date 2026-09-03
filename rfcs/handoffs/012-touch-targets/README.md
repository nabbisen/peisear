# Handoffs — RFC 012, touch target conformance

Handoffs implementing [RFC 012](../../accepted/012-touch-target-conformance.md).

| ID | Link | What | Release | Depends on |
|---|---|---|---|---|
| TT-001 | [TT-001](./TT-001-audit.md) | Step 2 — audit the 139 controls: mechanism per surface, and the adjacency map. **No code changes.** | 0.30.0 | step 1 (the amended requirement) |
| TT-002 | [TT-002](./TT-002-apply.md) | Step 3 — apply. 136 `Grow` + 3 checkbox `<label>` wraps; one home for the 44 px fact; the input/select `line-height` question named rather than assumed. | 0.31.0 | TT-001 |
| TT-003 | [TT-003](./TT-003-guard.md) | Step 3 — the size guard with no exception list, `§10.16`'s filesystem→block reopening, the test-side literals, and the unscoped-assertion sweep. **Closes RFC 012.** | 0.31.0 | TT-002 |
| REL-0.31.0 | [REL-0.31.0](./REL-0.31.0-release-candidate.md) | Release candidate — RFC 012 in full. Ordinary cut from `main`'s tip; the milestone is real and the entry must not spend the word "met". | 0.31.0 | TT-003 |

**Step 1 is the architect's own work and has no handoff** — `NFR-A11Y-007` and
external design `§5.7` carry the amended rule, and `DEC-049` records it.
