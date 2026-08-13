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

## The half that is not here, and why

RFC 009 splits deliberately.

**Requirements 1–4** — assignment — are blocking and privacy-inert. They
disclose display names, which team membership already discloses. That is
TEAM-001.

**Requirement 5** — who sees per-user workload rows — is a new disclosure of
capacity and in-flight points under `NFR-PRIV-007` and external design §11.5.
It is vacuous today only because the set has one row: the owner, to themselves.
RFC 009 open question 2 is unanswered and is the owner's.

Until it is answered, `project_workload` gains no new consumers. TEAM-001
corrects the query so it cannot disagree with the candidate set, and stops
there.

**Accepting RFC 009 did not answer open question 2.** A second handoff follows
once it is.
