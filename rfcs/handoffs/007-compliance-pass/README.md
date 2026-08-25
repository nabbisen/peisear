# Handoffs — RFC 007, 0.20.0 compliance pass

Implementation companions for
[RFC 007](../../done/007-compliance-pass.md). These record **how to
implement and verify**; the RFC records **what was decided and why**.

A handoff does not override its RFC. If implementation uncovers a design
conflict, stop and escalate — the RFC is amended first, then the handoff.
These inherit RFC 007's state; they have no lifecycle of their own.

## Order

| Handoff | When | Why |
|---|---|---|
| [DEV-006](./DEV-006-workspace-formatting.md) — workspace formatting | **First, alone** | Blocks everything; a 44-file reformat mixed into any other diff makes that diff unreviewable |
| [DEV-007](./DEV-007-storage-clippy-conformance.md) — storage clippy debt | After DEV-006; parallel with the rest | Confined to `peisear-storage`, which nothing else here touches |
| [DEV-001](./DEV-001-kanban-optimistic-lock-contract-repair.md) — optimistic-lock repair | After DEV-006 | P0 data integrity. **Complete — rebase onto DEV-006 and resubmit** |
| [DEV-003](./DEV-003-capacity-privacy-workload-chips.md) — capacity privacy | After DEV-006, parallel | P0 privacy — live disclosure |
| [DEV-004](./DEV-004-health-presentation-watch-ceiling.md) — health Watch ceiling | After DEV-006, parallel | P0 + P1 |
| [DEV-005](./DEV-005-small-debt-bundle.md) — small-debt bundle | After DEV-006, parallel | P2 — config, one test, docs only |
| [DEV-002](./DEV-002-kanban-keyboard-status-control.md) — board keyboard control | **After DEV-001 merges** | Must reuse the lock check DEV-001 establishes |
| [DEV-008](./DEV-008-web-clippy-conformance.md) — web clippy debt | **Last** — after DEV-001..004 land | Its findings sit in files those four all edit; and it closes the workspace gate |

### Outstanding corrections from review

Four one-line changes, no re-implementation. See `.git-exclude/reviewed/`.

| Correction | Handoff | Source |
|---|---|---|
| Entity-neutral wording in `check_optimistic_lock` — the shared message must not say "board" | DEV-001 | `DEV-001-004-review.md` §1.4 |
| `draggable="false"` on the inner card `<a>` | DEV-002 | `DEV-002-005-review.md` §1.3 |
| "four sibling crates" in `crates/peisear/src/lib.rs:4` — item C scope now permits doc comments under `crates/*/src` | DEV-005 | `DEV-002-005-review.md` §2.3 |
| One commit per handoff, dependency order, DEV-006 first | all | `DEV-006-007-review.md` §3 |

### Remaining work — two items, in order

1. **DEV-005 item A** — see below. The last outstanding *fix*.
2. **[DEV-009](./DEV-009-release-candidate-0.20.0.md)** — release-candidate
   preparation. Version bump, changelog close-out, cold-cache gate run,
   tarball. **Depends on item A.** Do not tag or publish: the owner approves
   the release first.

**Everything else in RFC 007 is complete and reviewed.**

Item A is **unblocked**: the owner ratified `rust-version` → `1.88.0`
(`DEC-044`) on 2026-08-03, and the handoff's §4 has been reissued with the
corrected design — `rust-toolchain.toml` pins **1.97.1** for determinism,
`rust-version` declares **1.88.0** as the build floor, and a separate build-only
CI job verifies the floor.

It is small, and it matters more than its size suggests: without the pin, the
`fmt` and `clippy` results this release just achieved are hostage to the next
`@stable` bump — which is exactly the drift that produced ISSUE-001.

Read the reissued §4 in the handoff, not the original text.

DEV-006 lands first and alone. Everything else then runs in parallel on
disjoint files; DEV-005 touches no source under `crates/*/src` at all,
and DEV-007 touches only `peisear-storage`.

**DEV-006 and DEV-007 were added 2026-08-01** after ISSUE-001 established
that `cargo fmt` and `cargo clippy` have never passed — CI has failed on
every push since it was introduced, while all 14 test and build jobs
passed throughout. Ruling and reasoning:
`.git-exclude/reviewed/ISSUE-001-decision.md`.

Review requests go to `.git-exclude/review-request/` in the format each
handoff specifies (workflow §9.2).

## Before you start: the 0.19.1 baseline has known-incorrect statuses

Onboarding sends you to
`.git-exclude/specs/peisear-0.19.1-requirements-en.md`. Read it — it is the
right document — but four recorded statuses are wrong, and three bear
directly on this queue. Read
`.git-exclude/tasks/architect/004-requirements-baseline-amendments.md` **§B**
alongside it.

| Requirement | Baseline says | Reality |
|---|---|---|
| `NFR-CONC-001`, `NFR-CONC-005` | Implemented (P0) | **Not implemented** on the kanban status path — this is DEV-001 |
| `FR-DM-001` | Deferred to Phase D | Kanban drag has been shipping since ~0.6.0 |
| `FR-DM-002` | Deferred (P0) | In force and **violated** — this is DEV-002 |
| `FR-HLT-009` | Partial, for the stated reason | Partial for an additional reason: the domain crate generates user-visible prose |

The trap worth naming: if you onboard believing the optimistic-lock contract
is complete, DEV-001 reads as a strange edge case rather than what it is — a
P0 contract that went unenforced across four releases.

Everything else in the baseline is sound, including all of §5.1 (privacy),
§5.2 apart from the two rows above, and §1.7 (prohibited vocabulary), which is
normative and load-bearing for DEV-004.

## Escalation

Each handoff names what to escalate rather than decide. Per the workflow, an
implementer must not independently resolve requirements, architecture, public
API, persistence-format, or security-boundary questions. State the issue in a
review request; do not proceed on an assumption.
