# Handoffs — RFC 002, calendar surfaces

Implementation companions for
[RFC 002](../../accepted/002-calendar-surfaces.md), target **0.23.0**.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/`.
- **Design decisions**: RFC 002 itself, as amended 2026-08-13.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| CAL-001 | [CAL-001](./CAL-001-planned-dates-schema.md) | Migration `0016`, the two `Issue` fields, the storage queries, and the issue form's date inputs | — |
| CAL-002 | [CAL-002](./CAL-002-calendar-surfaces.md) | The two calendar surfaces, view modes, period navigation, sprint band, crowding chip | CAL-001 |
| REL-0.23.0 | [REL-0.23.0](./REL-0.23.0-release-candidate.md) | Release candidate for 0.23.0 — and the first with a schema migration to account for | CAL-002 |

## Why two handoffs

RFC 002 is the largest single RFC this project has implemented: a schema
migration, two `peisear-core` fields, two storage queries, a form change, two
new routes, three view modes, period navigation, a sprint overlay and a
crowding chip.

RFC 001 was smaller than that and still needed two review rounds. Splitting at
the data boundary gives a review point where the schema and the queries can be
wrong on their own, before any of it is load-bearing for a page — and a
migration is the one thing in this project that cannot be corrected by editing
a file.

CAL-002 was written after CAL-001 was reviewed, not before — and what CAL-001
found did shape it. Three of CAL-002's five reconciliations exist because of
CAL-001: the test target it created, the trigger-message register question it
settled, and the UTC limitation it documented, which CAL-002 is the first
surface to make visible to a user.

## RFC 002 was amended before dispatch

It was written 2026-05-04 and targeted 0.21.0. Five things in it are now wrong
or under-specified; all five are corrected in the RFC and restated in CAL-001
§2. The pattern is the same one RFC 001 hit — an accepted RFC is a design
decision with a shelf life — and RFC 003 (0.24.0) is the same vintage again.
