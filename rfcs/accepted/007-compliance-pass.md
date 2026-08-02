# RFC 007: 0.20.0 compliance pass

**Status**: Accepted
**Target**: 0.20.0
**Related spec sections**: §3 (vocabulary), §11.5 (privacy boundary), §21.4 (optimistic lock), §28.2/§28.4/§28.6 (health presentation), §32 (keyboard alternatives)
**Related requirements**: `NFR-CONC-001`, `NFR-CONC-005`, `NFR-PRIV-001`, `NFR-PRIV-002`, `NFR-PRIV-007`, `FR-DM-001`, `FR-DM-002`, `FR-HLT-008`, `FR-HLT-009`, `NFR-LANG-001`, `NFR-LANG-002`
**Governing decisions**: `DEC-018`, `DEC-019`, `DEC-023`
**Handoffs**: [`rfcs/handoffs/007-compliance-pass/`](../handoffs/007-compliance-pass/README.md)
**Last updated**: 2026-08-01

## Summary

Correct four standing violations of product-defining requirements, found by
direct comparison of the 0.19.1 baseline against the code. No new features.
Release 0.20.0 contains this and nothing else.

| Handoff | Corrects |
|---|---|
| DEV-001 | Kanban status endpoint accepted mutations with no optimistic-lock value |
| DEV-002 | Board status change had no keyboard path |
| DEV-003 | Members' capacity disclosed to other members |
| DEV-004 | Health presentation exceeded the `Watch` ceiling and rendered a 0–100 score |
| DEV-006 | `cargo fmt` has never passed — 44 files were never formatted |
| DEV-007 | `cargo clippy -D warnings` has never passed — 21 errors in `peisear-storage` |

**DEV-006 and DEV-007 added 2026-08-01**, after the dev team escalated
pre-existing gate failures (ISSUE-001). Verification showed CI has failed on
**every push since it was introduced** — five runs, zero successes — with
`cargo fmt` and `cargo clippy` red throughout while all 14 test and build jobs
passed. `NFR-MNT-007` is recorded as *"Implemented in CI"* and never has been.
That is the same class of defect as the other four, so it belongs here rather
than in a separate release. **Scope change ratified by the owner 2026-08-01
(`DEC-043`).**

DEV-007 carries an explicit ambition limit: clear the lints conservatively, do
not redesign the storage crate inside a correction release. If the fourteen
`type_complexity` findings indicate a real problem with the crate's shape —
they probably do — that is reported and becomes its own RFC, not work absorbed
here.

## Background

Three of these are recorded in the 0.19.1 baseline as compliance gaps (§10.2)
or as requirements whose status was **misreported as satisfied**. The fourth —
the optimistic-lock bypass — was not recorded at all: `NFR-CONC-001` and
`NFR-CONC-005` are both P0 and were both marked `Implemented`, while
`POST /projects/{id}/issues/{issue_id}/status` accepted and applied lock-free
mutations, and the shipped kanban client never sent a lock value. The contract
went unenforced on that path across four releases.

Each violation sits on a commitment the product describes as defining rather
than incidental: data integrity, keyboard reach, the personal-data boundary,
and the refusal to grade. Those are not deferrable to a later quality phase,
which is why they precede RFC 001 rather than folding into RFC 005.

Full analysis and options: `002-decision-request-scope-and-posture.md`.
Approved decisions and rationale: `003-approved-decisions-2026-07-31.md`.
Baseline corrections this produces: `004-requirements-baseline-amendments.md`.

## Requirements

### Must

1. No request path mutates issue status without a validated lock value.
2. Every status change reachable by pointer is reachable by keyboard, with
   scripting disabled.
3. No surface discloses another user's capacity value, WIP limit, or any state
   derived from either.
4. No user-visible surface or API response exposes a severity above `Watch`,
   danger colouring for health state, or a 0–100 health figure.
5. No user-visible string introduced or touched violates §1.7.
6. Internal computation is unchanged. Every correction is at the presentation
   or contract boundary, per `FR-HLT-009`.

### Non-goals

7. No new feature, screen, or entity.
8. No schema change. 0.20.0 adds no migration.
9. The remaining §10 gaps — indicator-set divergence (§10.1), storage-layer
   defence in depth (§10.3), explainability affordances (§10.4) — stay open.

## Design

The design decisions are recorded as `DEC-018`, `DEC-019` and `DEC-023`; this
RFC does not restate them. The shape of each correction:

- **Lock contract** — remove the bypass; reject an absent lock value as 400;
  render the lock value on board cards; make the client send it and handle the
  409 that becomes reachable as a result. One shared lock check, two entry
  points.
- **Keyboard reach** — a per-card form POST control, no JavaScript, sharing
  that same lock check. `DEC-018` chose this over removing drag or pulling RFC
  004 forward: a form POST satisfies `FR-DM-002` literally without touching the
  drag contract RFC 004 owns.
- **Capacity** — remove the denominator, the over-capacity annotations, and the
  capacity-derived badge colour from shared surfaces. Retain in-flight load,
  which `NFR-PRIV-002` as amended permits. Suppress single-member aggregates.
- **Health presentation** — clamp the four-state internal model to three at the
  render boundary, structurally rather than by remembered call; retire the
  score badge; render the composite at equal weight beside the indicators.

## Test plan

Each handoff carries its own. Cross-cutting expectations:

1. Every correction ships with a test that **fails against unmodified `main`**.
   DEV-001 and DEV-004 require the failing run to be pasted as evidence — a
   regression test that passes before the fix is not testing the defect.
2. Disclosure and vocabulary tests assert on rendered HTML, not internal state.
3. DEV-004's ceiling test requires fixture data that actually reaches `Concern`.
   A ceiling test over data that never approaches the ceiling proves nothing.
4. `RSK-001` closes here: the cold-cache CI run, outstanding since 0.19.1 and
   dated "before 0.20.0", is captured as part of this release's evidence.

## Security and privacy considerations

Required — this RFC touches both §11.5 and §21.4.

- **§21.4 optimistic lock.** DEV-001 restores the contract on the only path
  that bypassed it. No force-overwrite path is introduced (`NFR-CONC-004`); no
  automatic retry on 409; the client may not fabricate a lock value.
- **§11.5 privacy boundary.** DEV-003 removes a live disclosure. It must not be
  solved by hiding elements in the UI or gating on role — `NFR-PRIV-004` makes
  the API the boundary, and `NFR-PRIV-003` means no role may see it.
- **No authorisation change.** No extractor, session, or access check is
  modified. DEV-001 keeps the project-access check ahead of the lock check:
  authorisation precedes concurrency.
- **API surface.** DEV-004 clamps `/api/users/{id}/burnout`'s `indicator` field,
  which external design §8.3 already required to observe the ceiling.

## Out of scope

- The v3 expansion entities (`DEC-023` defers them to roadmap replanning).
- i18n and the vocabulary guard — RFC 006, 0.21.0.
- Phase D direct manipulation — RFC 004, 0.25.0.
- Reproducible-release definition (`NFR-REL-005`) — **unscheduled pending owner
  decision**; see open question 2.

## Open questions

1. **Does the compliance pass need its own release notes framing?** DEV-004
   removes a visible score badge and DEV-003 removes visible information.
   Both are corrections, but a user sees removals. *Default: `CHANGELOG.md`
   records each with its rationale per `NFR-MNT-009`; no separate notice.*
2. **Is `NFR-REL-005` (reproducible release) in this release?** Amendment A4
   recommended 0.20.0 because `DEC-025` removed its "before 1.0" justification.
   Adding it expands an agreed four-task scope. *Default: not in 0.20.0; record
   `NFR-REL-005` as undated rather than let A4 imply a schedule that was never
   approved.*

## References

- [RFC 004 — direct manipulation](./004-direct-manipulation.md) (DEV-002 is its
  no-JS baseline for D-2)
- [RFC 006 — i18n architecture](../proposed/006-i18n-architecture.md) (the
  vocabulary guard that would have caught DEV-004's defects mechanically)
- [RFC 000 — RFC lifecycle policy](../done/000-rfc-lifecycle-policy.md)
- Requirements baseline §1.7, §5.1, §5.2, §10.2; external design §7.3, §15,
  §17.1
