# Handoffs — requirement status audit

**Not RFC-governed.** A record defect, and the root cause of this session's two
most expensive mistakes.

| ID | Link | What | Release |
|---|---|---|---|
| REQ-001 | [REQ-001](./REQ-001-what-the-record-claims-against-what-the-code-does.md) | **Four times in three days a requirement's recorded status was wrong**, twice at the cost of real work — `FR-DM-001` (twice), `FR-SPR-002`, `FR-SPR-004`. Three `Specified` requirements checked at random before writing the handoff are all shipped. Checks the 15 entries whose status is a claim, sweeps all 162 for acceptance citations naming tests that no longer exist, and **reports rather than amends**. | 0.40.0 |
| REL-0.40.0 | [REL-0.40.0](./REL-0.40.0-release-candidate.md) | Release candidate — three layout fixes, `PRAGMA optimize`, the gate at 120 cells, and the record audited against the code. **No migration, no new test file, and almost nothing a user can see** — which needs the opposite care from 0.39.0's changelog. | 0.40.0 |

