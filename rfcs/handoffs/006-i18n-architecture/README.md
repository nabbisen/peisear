# Handoffs — RFC 006, i18n architecture and vocabulary guard

Implementation companions for
[RFC 006](../../accepted/006-i18n-architecture.md), target **0.21.0**.
These record **how to implement and verify**; the RFC records **what was
decided and why**. A handoff never overrides its RFC — if implementation
uncovers a design conflict, stop and escalate.

RFC 006 was accepted on 2026-08-03 with all five of its open-question
defaults standing.

## What this release is actually for

Not languages. `NFR-LANG-005` keeps additional locales deferred; **one locale
ships**.

It is about creating one place where all user-visible copy lives, so §1.7 can
be *checked* instead of trusted. `FR-HLT-006` and `NFR-LANG-001` are both P0
and were, until now, recorded as "Implemented by convention; no automated guard
exists".

Release 0.20.0 is the argument for this one. It corrected a `Score N / 100`
badge, a `Concern` severity mapped to danger colouring, and the literal string
`"Failed to update status"` — a phrase §1.7 names explicitly. Each was found by
a person reading code. None would have survived a lint over a string table.

## Sequence

| # | Handoff | Scope | State |
|---|---|---|---|
| 1 | [I18N-001](./I18N-001-crate-and-guard.md) | `peisear-i18n` crate, `Locale`, key type, English table, **the guard**, CI wiring | **Approved** |
| 2 | [I18N-002](./I18N-002-core-message-descriptors.md) | `peisear-core` emits descriptors — gap §10.8 | **Approved** — correction landed in `f3b261b`, verified |
| 3 | [I18N-003](./I18N-003-notify-copy.md) | `peisear-notify` titles, bodies, email copy | **Approved** |
| 4 | [I18N-004](./I18N-004-health-explanation-defects.md) | **ISSUE-006 fix**: clamp `summarize` to `DisplayHealthState`; two new keys; fix the ceiling test; absorb `IndicatorKind::label()` | **Ready** |
| 4b | [REL-0.20.1](./REL-0.20.1-release-candidate.md) | Release candidate for the fix: version, changelog, gates, tarball | Ready when 4 is accepted |
| 5 | I18N-005a–e | `peisear-web` by surface group: shell/nav · project/issue · team/sprint · today/inbox/settings · errors/validation | Blocked on 4 |

**No corrections outstanding.** I18N-001, 002 and 003 are all approved and
closed.

### Two releases in one queue

I18N-004 and REL-0.20.1 ship as **0.20.1**, a patch correcting work RFC 007
delivered. Everything else here is **0.21.0** under RFC 006.

Both sit in this directory because ISSUE-006 was found during RFC 006's
execution. RFC 007 stays in `done/` — its work shipped — carrying a status note
that the ceiling was not fully closed at 0.20.0. Reopening a completed RFC, or
minting a new one for a two-defect fix, would represent that history less
honestly than this does.

### Sequencing note

**I18N-004 comes before the web conversions**, and they are renumbered
accordingly. `ISSUE-006` found a **P0 `NFR-LANG-002` violation still shipping**
— `summarize()` names the unclamped `Concern` state in prose, in the paragraph
directly beneath the health heading. A P0 does not queue behind mechanism work.

Ruling: [`ISSUE-006-decision.md`](../../../.git-exclude/reviewed/ISSUE-006-decision.md).
It ships as **0.20.1**, approved by the owner — the 0.20.0 changelog claims the
ceiling is enforced, and that claim is public and incomplete.

**0.21.0 is not complete until every shipped user-visible string is
converted.** Partial migration is the failure mode to avoid: a guard covering
half the copy invites the belief that the copy is covered.

## What the guard can and cannot do

Stated here so it is not over-trusted later:

- It covers **copy**, not interpolated data. An issue titled "velocity spike"
  is user data, not a violation.
- It catches vocabulary, not tone. `FR-HLT-006` still needs human review.
- It cannot catch a prohibited word assembled at runtime from fragments —
  which is why composing user-visible sentences by concatenation is prohibited.

## Carried over from 0.20.0

Two conventions earned the hard way, and they apply here:

1. **Coverage is per entry point, not per requirement.** `NFR-CONC-005` read
   as covered while the path the shipped client used was untested.
2. **A test crate without a CI job does not exist.** Add the job in the same
   change as the test.

## Escalation

Each handoff names its own triggers. In general, escalate rather than deciding
if a string's correct rendering is ambiguous, if the domain boundary in RFC 006
§D3 turns out to need a different shape than the RFC assumes, or if the guard
would need suppressing anywhere.

Review requests go to `.git-exclude/review-request/`, one per handoff, in the
format each specifies (workflow §9.2).
