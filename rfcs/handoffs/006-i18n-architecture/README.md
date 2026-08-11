# Handoffs — RFC 006, i18n architecture and vocabulary guard

Implementation companions for
[RFC 006](../../accepted/006-i18n-architecture.md), target **0.21.0**.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/` — one verdict
  file per unit, dated, with the outcome and any correction.
- **What to work on this cycle**: the dispatch note for the release.
- **Design decisions**: RFC 006 itself, including §D6's conversion conventions.

That separation is deliberate. An earlier version of this file carried a State
column, which is a separate handoff lifecycle — the anti-pattern
`000-rfc-lifecycle-policy.md` §"Turning handoffs into a second RFC lifecycle"
names. It also went stale under readers twice, costing the dev team a
reconciliation against the code each time.

## What this release is for

Not languages. `NFR-LANG-005` keeps additional locales deferred; **one locale
ships**.

It creates one place where all user-visible copy lives, so §1.7 can be
*checked* instead of trusted. `FR-HLT-006` and `NFR-LANG-001` are both P0 and
were recorded for two releases as *"Implemented by convention; no automated
guard exists"*.

0.20.0 is the argument. It corrected a `Score N / 100` badge, a `Concern`
severity mapped to danger colouring, and the literal string
`"Failed to update status"` — a phrase §1.7 names explicitly. Each was found by
a person reading code. None would have survived a lint over a string table.

## Handoffs

| # | Handoff | Covers | Depends on |
|---|---|---|---|
| 1 | [I18N-001](./I18N-001-crate-and-guard.md) | `peisear-i18n` crate, `Locale`, key type, English table, the guard, CI wiring | — |
| 2 | [I18N-002](./I18N-002-core-message-descriptors.md) | `peisear-core` emits descriptors; gap §10.8 | 1 |
| 3 | [I18N-003](./I18N-003-notify-copy.md) | `peisear-notify` titles, bodies, email copy | 1 |
| 4 | [I18N-004](./I18N-004-health-explanation-defects.md) | ISSUE-006 fix: severity clamp, two corrected sentences, ceiling test, `IndicatorKind::label()` | 2 |
| 4b | [REL-0.20.1](./REL-0.20.1-release-candidate.md) | Release candidate for the above | 4 |
| 5a | [I18N-005a](./I18N-005a-shell-and-navigation.md) | Shell and navigation; **established RFC 006 §D6** | 2 |
| 5b | [I18N-005b](./I18N-005b-project-and-issue.md) | Project and issue; absorbs `IssueStatus::label()`, `Priority::label()` | 5a |
| 5c | [I18N-005c](./I18N-005c-team-and-sprint.md) | Team and sprint; carries the normative `FR-TEAM-005` footnote | 5a |
| 5d | [I18N-005d](./I18N-005d-today-inbox-settings.md) | Today, inbox, settings, search | 5a |
| 5e | [I18N-005e](./I18N-005e-errors-and-validation.md) | Errors, validation, auth | 5a |
| 6 | [I18N-006](./I18N-006-remaining-prose-surface.md) | `peisear-core` and `peisear-storage` prose, `BurnoutSignal.label`, validator-literal scan test; **completes 0.21.0** | 5b–e |

5b–e are parallel with each other. They all add variants to the same
`MessageKey` enum, so expect merge contention there and sequence commits
accordingly.

**Two releases in one directory.** I18N-004 and REL-0.20.1 shipped as **0.20.1**,
a patch correcting work RFC 007 delivered; everything else is **0.21.0**. Both
sit here because ISSUE-006 was found during RFC 006's execution.

**0.21.0 is not complete until every shipped user-visible string is converted.**
A guard covering half the copy invites the belief that the copy is covered.

## What the guard can and cannot do

Stated so it is not over-trusted:

- It covers **copy**, not interpolated data. An issue titled "velocity spike"
  is user data, not a violation.
- It catches vocabulary, not tone. `FR-HLT-006` still needs human review.
- It cannot see through runtime concatenation — which is why composing
  user-visible sentences from fragments is prohibited.

## Escalation

Each handoff names its own triggers. In general, escalate rather than deciding
if a string's correct rendering is ambiguous, if the guard rejects existing
copy, or if the guard would need suppressing anywhere.

Review requests go to `.git-exclude/review-request/`, one per handoff, in the
format each specifies (workflow §9.2).
