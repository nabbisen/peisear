# Handoffs — RFC 005, quality consolidation

Implementation companions for
[RFC 005](../../proposed/005-quality-consolidation.md), target **0.24.0**
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

## Why one item is out of phase

RFC 005 is Phase E's QA pass and targets 0.24.0. §9 is pulled forward because
every release between here and there runs the gates it corrects, and a suite
that fails half the time on `cargo test --workspace` trains people to re-run
rather than read.

§10 is pulled forward for the same reason: two of its three defects are
reachable today, and one of them lets a team's running sprint be deleted.

The rest of RFC 005 stays at 0.26.0 — its remaining scope audits Phase D, which
RFC 004's substeps create, so it cannot precede them.
