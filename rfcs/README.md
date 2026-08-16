# peisear RFCs

Design records for this project. Governed by
[RFC 000 — RFC lifecycle policy](./done/000-rfc-lifecycle-policy.md),
which lives in `done/` per its own self-application clause.

**The folder is the source of truth for an RFC's state.** The `Status`
field inside each file mirrors it; if the two disagree, the folder wins.

## Layout

```
rfcs/
  README.md      ← this index
  proposed/      ← open for review; implementer should not start
  accepted/      ← review complete; implementer may start
  done/          ← shipped
  archive/       ← withdrawn or superseded
  handoffs/      ← implementation companions, keyed by RFC number
```

This project uses the policy's **5-folder variant**. The policy
recommends the 4-folder shape by default and reserves the fifth for
projects where "the maintainer signed off" is a distinct event from
"the implementer finished" — which holds here: the architect designs,
the owner approves, the dev team implements.

## Accepted

Design settled. Implementation may begin.

| ID | Title | Target |
|----|-------|--------|
| 003 | [Inbox refinements](./accepted/003-inbox-refinements.md) — rewritten against the shipped code, then accepted; *has handoffs* | 0.24.0 |

## Proposed

Open for review. Design may still change.

| ID | Title | Target |
|----|-------|--------|
| 004 | [Direct manipulation](./proposed/004-direct-manipulation.md) | 0.25.0 |
| 005 | [Quality consolidation](./proposed/005-quality-consolidation.md) — §9 pulled forward, *has handoffs* | 0.24.0 |

## Implemented

| ID | Title | Shipped in |
|----|-------|------------|
| 002 | [Calendar surfaces](./done/002-calendar-surfaces.md) — *has handoffs* | 0.23.0 |
| 001 | [Sprint planning page](./done/001-sprint-planning-page.md) — *has handoffs* | 0.22.0 — minus the capacity hint |
| 009 | [Team assignment and workload scope](./done/009-team-assignment-and-workload.md) — *has handoffs* | 0.22.0 |
| 006 | [i18n architecture and vocabulary guard](./done/006-i18n-architecture.md) — *has handoffs* | 0.21.0 |
| 007 | [0.20.0 compliance pass](./done/007-compliance-pass.md) | 0.20.0 |
| 000 | [RFC lifecycle policy](./done/000-rfc-lifecycle-policy.md) | policy in effect |

## Archive

None.

## Handoffs

A handoff is an optional implementation companion. It records **how to
implement and verify** a decision; the RFC records **what was decided
and why**. A handoff never overrides its RFC — if implementation
uncovers a design conflict, the RFC is amended first.

Handoffs have no lifecycle of their own. Their state is inherited from
the RFC number they are keyed to.

| Directory | Governing RFC |
|---|---|
| [`handoffs/003-inbox-refinements/`](./handoffs/003-inbox-refinements/README.md) | 003 — active |
| [`handoffs/002-calendar-surfaces/`](./handoffs/002-calendar-surfaces/README.md) | 002 — historical, RFC implemented |
| [`handoffs/001-sprint-planning-page/`](./handoffs/001-sprint-planning-page/README.md) | 001 — historical, RFC implemented |
| [`handoffs/005-quality-consolidation/`](./handoffs/005-quality-consolidation/README.md) | 005 — §9 delivered at 0.22.0; the rest at 0.24.0 |
| [`handoffs/009-team-assignment-and-workload/`](./handoffs/009-team-assignment-and-workload/README.md) | 009 — historical, RFC implemented |
| [`handoffs/006-i18n-architecture/`](./handoffs/006-i18n-architecture/README.md) | 006 — historical, RFC implemented; `COPY-001` outstanding |
| [`handoffs/007-compliance-pass/`](./handoffs/007-compliance-pass/README.md) | 007 — historical, RFC implemented |

## Conventions

- Filenames are `NNN-slug.md`, zero-padded to **three digits**.
- Numbers are assigned at creation, are stable forever, and are never
  reused — a withdrawn number stays withdrawn.
- Moving a file between folders is what changes its state. Update the
  `Status` field and this index in the same change, and sweep inbound
  links (`grep -rn 'NNN-slug' rfcs ROADMAP.md docs`).
- Take the next free number when opening an RFC: **008**.

## What an RFC is for

- **Capture decisions before they're forgotten.** A discussion in chat
  ends; an RFC stays.
- **Hand work off cleanly.** The implementer reads it and can start
  without re-deriving the rationale.
- **Surface unknowns.** "Open questions" is a real section. If the
  implementer hits one, they escalate rather than deciding.

## Template

Two shapes — pick by scale, not formality.

### Lightweight (the default)

```markdown
# RFC NNN: Title

**Status**: Proposed
**Target**: <version>
**Related spec sections**: §X.Y
**Last updated**: YYYY-MM-DD

## Summary
One paragraph. What changes and why now.

## Design
Routes, schema, components, data flow. Concrete enough to begin.

## Out of scope
What is deliberately excluded, and where it lives instead.

## Open questions
Numbered. Each names the options and a default-if-no-decision.

## References
```

### Detailed

Add **Background**, **Requirements**, **Test plan**, and **Security &
privacy considerations**. Trigger the detailed shape when the change
crosses a crate boundary, involves a migration, touches the §11.5
privacy boundary or the §21.4 optimistic lock, or adds a public
surface (URL, endpoint, exported helper).

## Language

English, matching the rest of the repository. Code, SQL, and schema
fragments use the language of the artefact.
