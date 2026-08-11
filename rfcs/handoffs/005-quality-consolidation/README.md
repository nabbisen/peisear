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

## Why one item is out of phase

RFC 005 is Phase E's QA pass and targets 0.24.0. §9 is pulled forward because
every release between here and there runs the gates it corrects, and a suite
that fails half the time on `cargo test --workspace` trains people to re-run
rather than read.

The rest of RFC 005 stays at 0.24.0 and is not yet dispatched.
