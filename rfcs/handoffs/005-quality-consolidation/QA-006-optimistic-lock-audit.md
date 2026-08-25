# QA-006 — Optimistic-lock audit

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P0 — 0.27.0
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §2
**Depends on**: `QA-005` should land first. They do not conflict — different
files — but `QA-005` puts the four structural guards into CI, and this
handoff's fixes are the first work that should ship behind them.

---

## 1. What this is

RFC 005 §2's table, filled in for every mutation, with the missing rows
added and the empty cells closed. It was reconciled against the code on
2026-08-25 and **the reconciliation found two defects**, both stated in §2
and §3 below. Start there; the rest of the table may well be clean.

**Read §2 of the RFC first.** It carries the reasoning and this handoff does
not repeat it.

## 2. Finding 1 — two of four destructive deletes take a lock

Verified by reading the handlers, not inferred:

| Handler | Lock | Form |
|---|---|---|
| `sprints::delete_sprint` | `check_optimistic_lock` at `sprints.rs:464` | `Form<LifecycleForm>` |
| `settings::delete_capacity` | at `settings.rs:288` | yes |
| `projects::delete` | **none** | **takes no form** |
| `issues::delete` | **none** | **takes no form** |

`render_delete_confirmation` already takes `hidden_fields: Vec<(String,
String)>`. The sprint interstitial passes `client_updated_at` through it; the
project and issue interstitials pass `Vec::new()`.

**Decide it once, and record the decision.** Both answers are defensible — the
row is gone either way, so a stale timestamp corrupts nothing — and this
handoff does not tell you which. What is not defensible is four routes
deciding it three different ways by accident.

- If **deletes should lock**: pass `client_updated_at` through the two
  interstitials, check it in both handlers, and add a `409` test per route.
- If **deletes should not lock**: remove it from the sprint and capacity
  paths, and put the reason in a comment where the next reader will hit it.

**Say which you chose and why, in prose, before showing the diff.** I would
rather review the reasoning than the patch.

## 3. Finding 2 — the issue confirmation does not name the cascade

`issues::delete_confirm` renders `ConfirmDeleteCannotBeUndoneNote` — *"This
cannot be undone."*

`projects::delete_confirm` renders *"All its issues will be deleted too. This
cannot be undone."*

`issues.parent_issue_id` is `REFERENCES issues(id) ON DELETE CASCADE`
(`0015_sub_issues.sql:65`), and `pool.rs:27` sets `foreign_keys(true)`.
**Deleting a parent issue deletes its sub-issues.** The screen whose purpose
is to name what will be deleted does not name them.

**Confirm the cascade actually fires before writing copy for it** — a test
that deletes a parent with two sub-issues and asserts both are gone. If it
does not fire, stop and report: that is a **worse** defect than this one
(orphaned rows), and it changes what needs writing.

Then: a new message key for an issue that has sub-issues, naming the
consequence. Leave `ConfirmDeleteCannotBeUndoneNote` for the childless case
rather than showing a cascade warning to a user whose issue has no children.
The count is available at `delete_confirm` — one query — and `FR-SUB-006`
already renders sub-issue counts elsewhere, so check how that copy is phrased
and match it rather than inventing a second phrasing for one fact.

New copy goes through `peisear-i18n` and §1.7 as usual.

## 4. The rest of the table

Fill in every cell of RFC 005 §2's table. For each mutation:

- Does it check the lock? (Read the handler. Do not infer from a sibling.)
- Is there a `409` test naming that **route**, not that requirement? Coverage
  is recorded per entry point — `NFR-CONC-005` read as covered once while the
  path the shipped client used was untested.
- Does the interface roll back, and is that rollback asserted or only written?

`plan/add` and `plan/remove` are recorded as intentionally lock-free
(join-table). Confirm that is still true rather than copying the row forward.

**Report rows that are already correct as well as rows that are not.** An
audit that lists only findings cannot be distinguished from an audit that
stopped early.

## 5. Escalate rather than deciding

- **If either §2 or §3 does not reproduce, stop and report before
  implementing.** A handoff describing a defect that is not there has happened
  in this project, and reporting it was the right move.
- If filling in the table turns up a mutation with **no** lock and **no**
  recorded reason, that is a third finding — report it, do not fix it inside
  this handoff.
- If the cascade test in §3 shows sub-issues surviving, stop.

## 6. Acceptance

1. §2 decided, with reasoning stated in prose, applied consistently to all
   four deletes.
2. §3's cascade confirmed by a test; copy added for the sub-issue case;
   childless case unchanged.
3. RFC 005 §2's table complete — every cell, including the ones that were
   already right.
4. A `409` test per locking route, each demonstrated failing with its lock
   check removed — **one at a time**.
5. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 7. Required review-request format

Workflow §9.2. §2's decision as prose before any diff. §3's cascade
transcript. The completed table.
