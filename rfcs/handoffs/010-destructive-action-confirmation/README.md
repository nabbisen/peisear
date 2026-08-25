# Handoffs — RFC 010, destructive-action confirmation

Implementation companion for
[RFC 010](../../done/010-destructive-action-confirmation.md), target
**0.25.0**, ahead of any RFC 004 substep.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 010 itself; all three of its open questions were
  settled at acceptance.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| CONF-001 | [CONF-001](./CONF-001-confirmation-interstitial.md) | The interstitial, four `GET` routes, four originating controls | — |

## Four of nine, not nine

Five of the nine `confirm()` dialogs guard actions that are reversible through
the interface — leave team, remove member, detach project, remove capacity row,
silence all. They are untouched, and their `prose_scan` allowlist entries stay.

Silence-all is the one that changed underneath the gap: external design §17.4
was recorded at 0.21.0, when silencing everything was a one-way trip. RFC 003's
resume banner shipped at 0.24.0, so it is now undoable in one click — and a
dialog guarding a one-click undo is friction rather than protection.

## Why before RFC 004

RFC 004 is a phase of JavaScript enhancement. This is the existing
JavaScript-enhancement defect, under the same rule (`DEC-021`). Fixing it first
means the phase starts from a codebase where the un-enhanced path is not the
dangerous one.
