# Handoffs — the assignee badge

**Not RFC-governed.** A defect found by measuring twelve pages the overflow
gate does not visit, and a coverage gap one layer below the one `GATE-001`
closed.

| ID | Link | What | Release |
|---|---|---|---|
| LAYOUT-011 | [LAYOUT-011](./LAYOUT-011-the-assignee-badge-the-gate-has-never-seen.md) | A display name in a bare `badge badge-sm badge-ghost` at three sites — 441 px, overflowing the team board at 320, 390, **768 and 1280**, and issue detail at 320 and 390. **A desktop overflow**, the second after `LAYOUT-008`. **No assignee badge has ever rendered under the gate**: its issues are unassigned, so the pages it already sweeps cannot show one. `GATE-001` closed an empty fixture; this is a populated one that never exercises a branch. | 0.40.0 |
