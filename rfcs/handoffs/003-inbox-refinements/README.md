# Handoffs — RFC 003, inbox refinements

Implementation companion for
[RFC 003](../../accepted/003-inbox-refinements.md), target **0.24.0**.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 003 itself, rewritten 2026-08-16.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| INBOX-001 | [INBOX-001](./INBOX-001-inbox-refinements.md) | All three items: silence-resume banner, inbox email opt-in prompt, sub-issue parent in search | — |
| REL-0.24.0 | [REL-0.24.0](./REL-0.24.0-release-candidate.md) | Release candidate for 0.24.0 | INBOX-001 |

## One handoff, not two

CAL-001 and CAL-002 were split because a migration cannot be corrected by
editing a file. RFC 003 has no migration and no irreversible step, and its three
items do not depend on each other. Splitting would add a review round and buy
nothing.

## This RFC was rewritten before it was dispatched

Accepted in May, returned to `proposed/` on 2026-08-16 after a reconciliation
against the shipped code, rewritten, and re-accepted the same day.

The finding that made the round trip worth it: the old text triggered the
silence banner on `global_acknowledged`, which records whether the **email
opt-in** was answered. An implementer following it would have used that
function, and a test written from the same RFC would have used it too — they
would have agreed with each other and both been wrong.

`INBOX-001`'s test 2 exists so that cannot recur.
