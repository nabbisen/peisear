# SPRINT-004 — capture the record at completion, discard it on reopen

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.39.0
**Governing RFC**: [0013](../../accepted/013-the-sprint-record.md), accepted
2026-09-24, `DEC-054`. **Read it first** — the reasoning for capturing rather
than freezing is there and is not repeated here.
**Supersedes**: `SPRINT-003` (reopen, `DEC-053`), folded in below.
**Withdraws**: `SPRINT-002`, never implemented.
**Depends on**: nothing outstanding.

**One handoff, not two, because capture without discard is a trap**: captured
records and no way to correct a sprint completed by mistake is a worse state
than today. They land together or not at all.

---

## 1. First, take two things back out

**1.1 — Revert `SPRINT-001`'s first half.** The unassign refusal
(`assign_issue`'s branch, the early check, `remove_issue_if_status`'s use on
that route) and **its message key
`CannotUnassignFromCompletedSprintMessage`** — enum variant, enumeration entry,
`en.rs`, fixture. It blocked carry-over from the issue form and buys nothing
once the record is captured.

Its tests go with it: `unassigning_from_a_completed_sprint_is_refused_and
_history_does_not_move` is asserting the opposite of what will now be true.

**1.2 — Keep `SPRINT-001`'s second half.** `plan_remove` racing `start`, and
`remove_issue_if_status` itself, stay: that is a race about whether a sprint is
**plannable**, which is independent of history and still real. Its test stays.

**Do this as its own commit**, so the revert is reviewable as a revert and the
capture is not tangled with it.

## 2. The capture

**2.1 — Two tables, migration `0019`.** Scalar columns, following
`metrics_snapshots` / `user_metrics_snapshots` — **not JSON**.

- `sprint_records`: one row per completed sprint — `sprint_id` (primary key,
  `REFERENCES sprints(id) ON DELETE CASCADE`), `committed_points`,
  `completed_points`, `committed_count`, `completed_count`, `captured_at`.
  **Carried-over is derived, not stored** — it is `committed − completed`
  today and storing it invites the two to disagree.
- `sprint_burndown_points`: one row per sprint per day — `sprint_id`, `day`,
  `cumulative_committed`, `cumulative_completed`, with
  `PRIMARY KEY (sprint_id, day)` and the same cascade. That is `BurndownPoint`
  as it already exists in `peisear-core`.

**2.2 — `complete` writes both, inside its existing `BEGIN IMMEDIATE`.**
Computing the figures and writing them must be in the same transaction that
sets the status, or this reintroduces exactly the class `RACE-001` and
`RACE-002` closed. **Say in the review request which statements are inside it.**

**2.3 — `reopen` deletes both, inside its transaction.** See §4.

**2.4 — `summary` and `burndown` branch on status**: a `completed` sprint reads
its captured row; anything else computes as today. One branch each, at the top,
with a comment pointing at `DEC-054` — and **delete the
`sprints.rs:620-626` comment claiming the summary already "captures the
moment"**, which has never been true and is what made this defect invisible.

**2.5 — A completed sprint with no record** (possible only if §3's backfill
missed it, or a future bug) **must not silently compute live** — that is the
defect returning quietly. Decide between an error and a visibly-degraded read,
**and say which and why**. My preference is the error: a missing record is a
fault, and a figure that looks right is worse than one that stops.

## 3. The backfill — approved, and it is wrong on purpose

**Migration `0019` backfills every existing completed sprint** from the live
computation at migration time.

**It is wrong by exactly the drift that has already happened**, and that is
accepted: a sprint reporting a drifted figure *consistently forever* beats one
that keeps drifting, and the alternative leaves two classes of completed sprint
with nothing to tell them apart. **The changelog says this plainly** — I write
that; do not soften it in a comment.

- The burndown series must be backfilled too, per sprint per day.
- **If a sprint's live computation cannot be expressed in SQL** inside a
  migration, say so rather than approximating: a backfill that quietly differs
  from what the page showed yesterday is the one outcome worse than the drift.

## 4. Reopen (was `SPRINT-003`, `DEC-053`)

An administrator returns a completed sprint to `active`. **`SPRINT-003`'s
design stands unchanged** — read it for the eight decisions and their sources —
with these amendments:

- **It deletes the sprint's `sprint_records` and `sprint_burndown_points` rows
  inside its transaction.** This is what makes reopen *mean* something rather
  than being a status flip, and it is why the two handoffs merged.
- Its justification changes and its behaviour does not: reopen is **un-capture
  and resume**, no longer *the only way to correct a frozen sprint*.
- Its round-trip test changes shape — see §5.

## 5. Verification

- **The drift that started this, fixed**: complete a sprint with one done and
  one unfinished issue; record its figures; **carry the unfinished issue to the
  next sprint via the planning page** — which must **succeed**; then **finish
  it**. The completed sprint's `summary` and `burndown` are **identical** at
  all three points. This is the test the whole RFC exists for; write it first.
- **Carry-over works from both routes** — the planning page and the issue
  form's sprint field. §1.1's revert is what makes the second one true.
- **All four inputs**: after completion, changing a member issue's `status`,
  its `effort`, and its `updated_at` (any edit) each leave the record
  unchanged. Four assertions, because four inputs is the finding.
- **Reopen**: figures and burndown are live again, membership edits affect
  them, and completing again captures afresh — **assert the second capture
  differs from the first** where the work changed, or the discard is not
  proven.
- **The round trip**: complete → reopen → correct → complete, with the figures
  reflecting the correction.
- **Backfill**: a database built by the **released 0.38.0 binary** with at
  least one completed sprint, migrated; every completed sprint has a record and
  a burndown series; **the rendered figures match what the 0.38.0 binary
  showed for the same database** — that is the check that the backfill is
  faithful rather than merely present. Empty database too.
- `SPRINT-001` §2's `plan_remove` test passes unmodified.
- **The i18n guards** after removing a message key.
- `DEC-007`: report the new count, last recorded **329**, and note the tests
  removed by §1.1 separately from those added. New test file → the
  `CONTRIBUTING.md` block **and** the `test.yml` job, both authorised.
- Three consecutive workspace runs; `fmt`; `clippy`; **the overflow gate** — a
  new control on the sprint page.

## 6. Escalate rather than deciding

- **If capturing inside `complete`'s transaction is not possible** as written.
- **If the backfill cannot be faithful** (§3).
- **If a fifth input to the figures exists** that RFC 0013's table misses.
  Four was the count from reading `summary` and `burndown`; it is not a proof.
- **If reverting §1.1 turns out to break something** other than its own tests.
- **If any user-visible string other than §1.1's removal and reopen's two
  additions changes.**

## 7. Exit condition

A completed sprint's figures and burndown do not move when its issues are
carried over, finished, edited or re-estimated; carry-over works from both
routes; reopen discards the capture and completing recaptures; every
pre-existing completed sprint has a record matching what 0.38.0 displayed.
