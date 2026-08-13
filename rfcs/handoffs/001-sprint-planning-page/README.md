# Handoffs — RFC 001, sprint planning page

Implementation companion for
[RFC 001](../../accepted/001-sprint-planning-page.md), target **0.22.0**.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 001 itself, as amended 2026-08-13.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| PLAN-001 | [PLAN-001](./PLAN-001-sprint-planning-page.md) | The page, both move routes, filters, read-only modes — RFC 001 minus the capacity hint | TEAM-001 |

## The capacity hint is not here

RFC 001's capacity hint sums each participating member's capacity. With one
participating member the sum **is** that person's capacity; with two, a member
subtracts their own and has the other's. `NFR-PRIV-001` makes capacity
self-only and is P0.

That makes it the first genuine `NFR-PRIV-007` case in the product — an
aggregate reversible to an individual — in a product built for teams of about
five. It needs an owner decision and a design that survives a two-person team,
and neither belongs inside a feature handoff. RFC 001 §Security has been
corrected; the claim that a member cannot infer another's capacity from the
hint was false.

Everything else in RFC 001 is unblocked, so the page ships without it. The
committed total stays — that is a sum of effort on issues already visible on
the same page.
