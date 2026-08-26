# Handoffs — RFC 005, quality consolidation

Implementation companions for
[RFC 005](../../proposed/005-quality-consolidation.md), target **0.27.0**
except where a item is pulled forward.

**This file is an index, not a status board.** It lists what each handoff
covers and what it depends on. It changes when a handoff is added — not when
one is reviewed.

- **Current status of any handoff**: `.git-exclude/reviewed/` — one verdict
  file per unit, dated, with the outcome and any correction.
- **What to work on this cycle**: the dispatch note for the release.
- **Design decisions**: RFC 005 itself.

That separation follows `000-rfc-lifecycle-policy.md` §"Turning handoffs into a
second RFC lifecycle".

## Handoffs

| # | Handoff | Covers | Target | Depends on |
|---|---|---|---|---|
| QA-001 | [QA-001](./QA-001-test-harness-collision.md) | `TestApp::spawn` name collisions; a repeated full-workspace run in the gate set (RFC 005 §9, baseline `§10.13`) | **0.22.0** — pulled forward | — |
| QA-002 | [QA-002](./QA-002-three-defects.md) | An active sprint may not be deleted; project delete must not report a false success; `prose_scan` stops reading comments (RFC 005 §10) | **0.25.0** — pulled forward | CONF-001 |
| QA-003 | [QA-003](./QA-003-untested-script-tags.md) | `board.js` and `search.js` are referenced by no test — the board's tag can be deleted with the suite green; a comment claims otherwise (RFC 005 §12) | 0.27.0 | — |
| QA-004 | [QA-004](./QA-004-dec-007-block-omits-a-crate.md) | `DEC-007`'s command block omits the `peisear` facade; a guard so the list cannot drift from the workspace again (RFC 005 §13) | 0.27.0 | — |
| QA-005 | [QA-005](./QA-005-guards-have-no-ci-job.md) | `prose_scan`, `static_js_scan`, `test_harness_scan` and `dec_007_scan` all live in `peisear-web --lib`, which CI never runs and `DEC-007`'s block omits (RFC 005 §14) | 0.27.0 | — |
| QA-006 | [QA-006](./QA-006-optimistic-lock-audit.md) | The optimistic-lock audit; two of four destructive deletes lock and two do not, and the issue confirmation does not name its cascade (RFC 005 §2) | 0.27.0 | QA-005 first |
| QA-007 | [QA-007](./QA-007-authorization-audit.md) | The authorization audit; seven unaudited rows, three of them `GET` halves that authorise a mutation (RFC 005 §1) | 0.27.0 | QA-005 first |
| QA-008 | [QA-008](./QA-008-language-guard-and-two-closures.md) | The English renderer accepts non-English copy; `mark-all-read` can be made inert; the CI job can be deleted — three things that read as covered (RFC 005 §3, §1) | 0.27.0 | — |
| QA-009 | [QA-009](./QA-009-enumeration-and-ci-target-parity.md) | **`MessageKey::all()` is missing five live variants today** — the P0 vocabulary guard has never seen them; plus the twenty CI targets nothing pins (RFC 005 §3, §14) | 0.27.0 | — |
| QA-010 | [QA-010](./QA-010-the-sets-the-guards-walk.md) | The fourteen label enums, `peisear-core`'s kind/channel lists, and `prose_scan`'s two directories — three enumerations nothing checks. **Nothing broken today**; tripwires, not repairs (RFC 005 §3, §14) | 0.27.0 | — |
| REL-0.27.0 | [REL-0.27.0](./REL-0.27.0-release-candidate.md) | The release candidate — two user-visible changes from `QA-006`, and the guard work told truthfully | 0.27.0 | QA-003..010 |
| QA-011 | [QA-011](./QA-011-keyboard-completeness-and-live-regions.md) | `NFR-A11Y-001`'s **P0** keyboard audit — §5 had specified the P3 shortcuts instead — and conflicts announced through a polite live region (RFC 005 §5) | 0.28.0 | — |
| QA-012 | [QA-012](./QA-012-contrast-audit.md) | `NFR-A11Y-005`'s contrast audit, aimed at the 130 opacity modifiers rather than the theme's own tokens (RFC 005 §4) | 0.28.0 | — |
| QA-013 | [QA-013](./QA-013-the-seventy-floor.md) | Raise the muted tier to a `/70` floor — 111 sites — and guard the banned range; owner-approved from `QA-012`'s table (RFC 005 §4) | 0.28.0 | QA-012 |
| QA-014 | [QA-014](./QA-014-touch-targets.md) | `NFR-A11Y-007` measured: ~149 controls below 44 px, exactly one compliant and nothing asserting it. **Measurement only** — the 44 px requirement may itself be what changes (RFC 005 §6) | 0.28.0 | — |
| QA-015 | [QA-015](./QA-015-mistap-safety.md) | Three checkboxes below the **AA** floor, and the confirmation screen's irreversible Delete within a mis-tap of its own Cancel — pulled ahead of 0.30.0's touch-target pass (RFC 005 §6) | 0.28.0 | QA-014 |
| QA-016 | [QA-016](./QA-016-aggregate-inferability.md) | The burndown and velocity charts, not the workload chip — §7's three named surfaces are all settled, one by a revert of what §7 proposes. **Audit only** (RFC 005 §7) | 0.28.0 | — |
| QA-017 | [QA-017](./QA-017-drop-the-trajectory.md) | Keep the aggregate, drop the trajectory — burndown and median line suppressed below two distinct contributors, **with no copy explaining why** (RFC 005 §7) | 0.28.0 | QA-016 |
| QA-018 | [QA-018](./QA-018-verify-the-claims.md) | §8's three greps are clean; the sweep it was a proxy for is not — 40 acceptance citations unverified, 84 `Implemented` requirements citing nothing, and §9.2 claiming four. **Audit only** (RFC 005 §8) | 0.28.0 | — |

## Why two items are out of phase

RFC 005 is Phase E's QA pass and targets **0.27.0**. §9 is pulled forward because
every release between here and there runs the gates it corrects, and a suite
that fails half the time on `cargo test --workspace` trains people to re-run
rather than read.

§10 is pulled forward for the same reason: two of its three defects are
reachable today, and one of them lets a team's running sprint be deleted.

The rest of RFC 005 stays at **0.27.0** — its remaining scope audits Phase D,
which RFC 004's substeps create, so it cannot precede them. D-1 and D-2 shipped
in 0.26.0, so that audit is now live rather than pending.

§12, §13 and §14 were added on 2026-08-25 from `REL-0.26.0`'s review and the
0.26.0 baseline update and are in
phase: both are Phase E test debt, and §8's follow-up sweep is exactly where a
dependency nothing checks belongs.
