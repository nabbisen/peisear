# Handoffs — RFC 004a, status change (D-1)

Implementation companions for
[RFC 004a](../../accepted/004a-direct-manipulation-status.md), target
**0.25.0**, after `CONF-001`.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 004a, and RFC 004's cross-cutting contract above it.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| STATUS-001 | [STATUS-001](./STATUS-001-no-js-status-control.md) | Step 1 — a status control on issue detail and issue list that works without JavaScript | CONF-001 |
| REL-0.25.0 | [REL-0.25.0](./REL-0.25.0-release-candidate.md) | Release candidate for **all** of 0.25.0 — CONF-001, QA-002, STATUS-001 | STATUS-001 |
| STATUS-002 | *not yet written* | Step 2 — the click affordance, the in-place update, the undo toast, and `change_status` returning the new lock value | STATUS-001; **not in 0.25.0** |

## Two handoffs because it is two steps

Step 1 is shippable alone: two surfaces gain a status control they have never
had. Step 2 is the enhancement on top.

The order is not a preference. RFC 004's cross-cutting requirement 0 says the
no-JS path ships first and that the enhancement may not be the first
implementation of an action. What the reconciliation found is that here, step 1
is the larger half — the issue list renders status as text and the issue detail
page renders three inert buttons, so neither surface has ever had a working
control.

**And the shape being avoided is the one this release is removing elsewhere.**
Attaching a click handler to three buttons that currently do nothing would give
some users a working control and others a silent no-op — external design §17.4,
which `CONF-001` is fixing for nine controls in this same release.

STATUS-002 is written after STATUS-001 is reviewed.
