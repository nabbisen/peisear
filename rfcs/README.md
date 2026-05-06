# peisear RFCs

This folder collects design documents for upcoming work — one
file per Roadmap theme. The format is small on purpose: most
RFCs are not full specifications but contracts between whoever
authored the decision and whoever picks up the implementation.

## Reading order

The number prefix is *file ordering*, not strict precedence —
themes can ship in parallel where dependencies allow. The
typical sequence today:

| RFC | Theme | Phase | Target version |
|---|---|---|---|
| [0001](./0001-sprint-planning-page.md) | Sprint planning page | C-PR2 | 0.20.0 |
| [0002](./0002-calendar-surfaces.md) | Calendar surfaces | C-PR3 | 0.21.0 |
| [0003](./0003-inbox-refinements.md) | Inbox refinements | C-PR4 | 0.22.0 |
| [0004](./0004-direct-manipulation.md) | Direct manipulation | D | 0.23.0 |
| [0005](./0005-quality-consolidation.md) | Quality consolidation | E | 0.24.0 |

When opening a new RFC, take the next free number (0006, 0007,
…) and add an entry to the table above.

## What an RFC is for

- **Capture decisions before they're forgotten.** A discussion
  in a chat ends; an RFC stays.
- **Hand work off cleanly.** The implementer reads the RFC and
  has enough to start without re-deriving the rationale.
- **Surface unknowns.** "Open questions" near the bottom of
  every RFC is a real section, not boilerplate. If the
  implementer hits one of them, they decide and update the
  RFC.

What an RFC is **not** for: replacing
`docs/spec/peisear-feature-spec-v2.1.md`. The spec is the
canonical product description. RFCs are about the *next slice*
of implementation work, with enough specificity that a person
(or LLM) coming in cold can pick it up.

## Template

There are two shapes — pick by scale, not by formality:

### Lightweight (the default)

```markdown
# RFC NNNN: Title

**Status**: Draft | Accepted | Implemented | Superseded
**Target**: <version, phase>
**Related spec sections**: §X.Y, §A.B
**Last updated**: YYYY-MM-DD

## Summary

One paragraph. What this changes and why now.

## Design

The shape of the change: routes, schema, components, data
flow. Concrete enough that the implementer can begin without
asking. Include code/SQL fragments where they sharpen the
description.

## Out of scope

What's deliberately not in this RFC, with a pointer to where
it lives instead. Common entries: future PRs in the same
phase, deferred Phase D work, observability.

## Open questions

Numbered list of unresolved decisions. Each entry names the
options and a default-if-no-decision. The implementer is
allowed to resolve these and update the RFC.

## References

Spec sections, prior CHANGELOG entries, related RFCs.
```

### Detailed (medium-or-larger scope)

Add these four sections to the lightweight shape:

- **Background** — why this change at all. Optional if obvious
  from context.
- **Requirements** — what must be true after the change ships.
  Distinguish must-haves from nice-to-haves.
- **Design** — replaces the lightweight "Design" section.
  Architecture, data model changes, route table, UI sketches,
  error paths.
- **Test plan** — what proves the requirements are met. Mention
  test crate names where relevant.
- **Security & privacy considerations** — auth boundaries
  affected, data exposure changes, audit-log implications.
  Required when `§11.5` (privacy boundary) or `§21.4`
  (optimistic lock) is in scope.

Trigger the detailed shape when:

- The change touches more than one crate boundary.
- Schema migrations are involved.
- The change affects `§11.5` privacy boundary or `§21.4`
  optimistic lock.
- The change introduces a new public-ish surface (URL, API
  endpoint, exported helper).

## Lifecycle

- **Draft** — written, not yet accepted. Comments and revisions
  expected.
- **Accepted** — agreed by the author and primary reviewer.
  Implementation can start.
- **Implemented** — code merged. The RFC stays in the folder
  as historical record.
- **Superseded** — replaced by a newer RFC; link forward.

When an RFC ships, update its status header but **do not
delete the file**. Future readers benefit from the rationale
trail more than from a tidy folder.

## Language

English, matching the rest of `docs/`. Code, SQL, and
schema/migration fragments use the language of the artefact
itself (Rust, SQL).
