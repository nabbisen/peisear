# Handoffs — RFC 009, team assignment and workload scope

Implementation companions for
[RFC 009](../../accepted/009-team-assignment-and-workload.md), target
**0.22.0**, ahead of RFC 001.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 009 itself.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| TEAM-001 | [TEAM-001](./TEAM-001-assignee-candidates.md) | RFC 009 requirements 1–4: the candidate set becomes team membership plus owner; one shared definition for both queries | — |

## The second half — withdrawn as a privacy question

*Updated 2026-08-13, after TEAM-001.*

RFC 009 originally split assignment from "who sees per-user workload rows",
holding the second on an owner decision. **That split was wrong.**
`NFR-PRIV-002` explicitly permits sharing workload distribution — each member's
volume of in-flight work — within a project or team, and `ISSUE-003` had
already ruled it holds regardless of how many members a strip lists. Capacity
and everything derived from it was stripped by `DEV-003` at 0.20.0 and is not
involved.

The instruction was also incoherent: `WorkloadStrip` and `WorkloadHint`
iterate whatever the query returns, so "the two queries must agree" and "do not
widen the consumers" cannot both hold. TEAM-001 escalated that rather than
inventing a workaround, which was correct.

TEAM-001 therefore covers RFC 009 in full. No second handoff is pending, and
nothing here is waiting on the owner.
