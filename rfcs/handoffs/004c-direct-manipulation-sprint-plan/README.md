# Handoffs — RFC 004c, the sprint plan (D-4)

Implementation companion for
[RFC 004c](../../accepted/004c-direct-manipulation-sprint-plan.md), target
**0.35.0**.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 004c, and RFC 004's cross-cutting contract above
  it — in particular requirement 10, added at 004c's own reconciliation: **a
  drag is not a touch path**, and no substep may claim phone parity from one.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| PLAN-002 | [PLAN-002](./PLAN-002-sprint-plan-drag.md) | The drag itself: the server-rendered attachment attribute, the copy island, `static/plan.js`, the one-at-a-time toast, and the filter-field defect the reconciliation turned up | — |

## What is different about this substep, in one place

**The no-JavaScript path already ships in full.** `PLAN-001` (0.22.0) built the
two columns and the button-driven moves through `POST /plan/add` and
`POST /plan/remove`. Umbrella requirement 0 — a working plain path first — is
satisfied before this handoff starts, which is the same position D-2 was in and
the opposite of D-1's.

**It is the first substep with no optimistic lock to carry.** `sprint_issues`
is a join table with no `updated_at`, so `DEC-013`'s triggers never touch it.
Umbrella requirements 5 and 6 — carry `client_updated_at`, return the new lock
value — **do not apply here at all**. That removes work rather than adding it,
and it is why `PLAN-002` §3.1 declines the JSON route D-1 needed.

**Keyboard parity already exists**, so the D-4 sketch's proposed binding was
withdrawn at acceptance. The move buttons are the keyboard path, as on the
board.
