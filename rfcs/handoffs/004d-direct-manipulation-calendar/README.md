# Handoffs — RFC 004d, the calendar (D-3)

Implementation companion for
[RFC 004d](../../accepted/004d-direct-manipulation-calendar.md), target
**0.36.0**.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 004d, and RFC 004's cross-cutting contract above
  it — including requirement 10, **a drag is not a touch path**, which this
  substep inherits as much as D-4 did.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| CAL-003 | [CAL-003](./CAL-003-day-view-reschedule.md) | The day-view reschedule: a new narrow JSON endpoint returning the lock value, the block's wrapper and identity attributes, `static/calendar.js`, and the state sync `PLAN-002` earned the hard way | — |

## What is different about this substep

**It is the first since D-1 where the optimistic lock applies in full.** An
issue's planned timestamps live on the `issues` row, which `DEC-013`'s triggers
maintain, so `client_updated_at`, the `409` path and the returned lock value
are all in force. That is the opposite of D-4, whose join table had no version
to compare — and it is why **this island carries an `outcomes` block where
`PLAN-002`'s deliberately did not.**

**The page has nothing to fall back to.** Every other enhanced surface has a
plain control beside the enhanced one; the calendar has only links, so a
failure here announces and reloads rather than resubmitting — `dm.js`'s undo
rule rather than its main-path rule.

**Two of the D-3 sketch's three actions were cut at acceptance**, for reasons
the project's own rules gave: the empty-cell action has no plain-form path, and
a resize handle cannot carry the 44 px floor inside a fifteen-minute block
without breaking the proportionality `DEC-050` protects.
