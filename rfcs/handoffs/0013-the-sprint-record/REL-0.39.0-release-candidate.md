# REL-0.39.0 — release candidate

**Contents**: `RACE-001` (`2479327`, `05cd94c`, `4da1104`), `RACE-002`
(`376c21f`), `RACE-003` (`e7ab562`), `SPRINT-001` (`762a31e`, **half reverted**
by `fbfc1db`), `SPRINT-004` (`a9f9556`), `SPRINT-005` (`2c35237`), RFC 0013
accepted with `DEC-053` and `DEC-054`, and the requirement work at `7aa2192`,
`39d7a43`, `3e7d3bc`. All six reviewed and closed.
**Depends on**: nothing outstanding.

**Do not tag. Do not publish.** Produce the candidate and stop. **This is the
only work in flight** — nothing else opens until the release is out.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.

**Two migrations**: `0019_sprint_records.sql` and
`0020_sprint_record_contributor_basis.sql`. **Both backfill.** Say so — an
upgrader's existing data is rewritten by this release in a way §2.3 has to be
honest about.

**Expected: 346**, up from 0.38.0's 297. `DEC-007`'s block and `test.yml` both
gained `sprint_record`; `aggregate_privacy` and `race_guards` already existed.

**The overflow gate is 95 cells, not 90** — nineteen pages at five widths,
after `SPRINT-005` added a completed sprint's detail page. A page added, not
swapped. That figure appears in `browser-checks/README.md` and in the
specification, both already corrected.

## 2. The changelog

**This release changes a data model, narrows what one page shows, and retires a
decision it made four days ago.** Seven things.

1. **Lead with the migrations and what they rewrite**, as 0.38.0 did. An
   upgrader needs `0019` and `0020` and the backfill before anything else.

2. **A completed sprint's figures no longer change** — the release's subject.
   Its committed and completed totals, counts, carried-over and burndown are
   **captured when it completes** (`DEC-054`, RFC 0013) and read from the
   capture thereafter. Before this they were computed live from four inputs,
   so carrying an issue over and finishing it a month later moved a finished
   sprint's numbers.

3. **Say plainly that the backfill is wrong**, and do not soften it. Existing
   completed sprints are captured **from today's live computation, which
   already carries whatever drift has happened to them.** They become
   *consistently* wrong rather than continuing to move. The alternative was two
   classes of completed sprint with nothing to tell them apart. **This is a
   decision, not an accident**, and a reader deserves to know which.

4. **The velocity aggregate now shows less** (`NFR-PRIV-007`, `SPRINT-005`).
   A median across several sprints is shown only if **one of them** had two
   contributors on its own; previously the union of contributors decided it.
   A team whose sprints are each one person's work will lose a median line
   they had. **Give the reason in one sentence** — each sprint's number is that
   person's output, so the aggregate was reversible to an individual — because
   a feature quietly showing less invites exactly the wrong guess.

5. **Reopen** (`DEC-053`): an administrator can return a completed sprint to
   active, which discards its captured record; completing again recaptures.
   **What it is for**: correcting a sprint completed by mistake, which became
   impossible once the record was fixed. It is why capture is tolerable.

6. **Five concurrency defects**, in one paragraph and not five:
   check-then-write pairs that were not atomic. **The one worth naming
   individually is the last-admin guard** — two administrators pressing *Leave
   team* at once left a team with **no administrator**, which no remaining
   member could undo. Also the optimistic lock itself, which compared a value
   read earlier so two saves in one request window could both pass; it is
   atomic now. **`§10.31` carries the finding that reframes the rest**: the
   deferred transactions this replaced were failing unrelated concurrent writes
   outright, so what looks like a slowdown is a correctness fix.

7. **A retired decision, and this is the honest half of the release.**
   `SPRINT-001` shipped to `main` refusing to unassign an issue from a
   completed sprint, and **half of it was taken back out four days later**.
   `SPRINT-002` was written and withdrawn without being built. Both were aimed
   at freezing membership, which is **one of four inputs** to the figures they
   meant to protect — and they would have blocked carry-over, the product's
   designed path for unfinished work. **The dev team stopped the second before
   writing code, having measured the flow it would break.** RFC 0013 replaced
   the mechanism. `FR-SPR-004`'s rationale was right throughout; the sentence
   built on it was not.

**What a reader should not conclude**, folded in:
- **`§10.15` and `§10.17` remain open**; `§10.30` and `§10.31` closed this
  release.
- **The optimistic lock still does not refuse two saves inside one request
  window on a row written in that same second** — `updated_at` is one second
  and `NFR-CONC-001` states the limit. **Say "in one request window", not "at
  once".**
- Membership of a completed sprint is **deliberately not frozen**, so a
  completed sprint's issue list can change while its figures do not. The page
  labels which is which.
- No WCAG conformance claim.

**Run `find_violations`** over the finished section and report the character
count.

## 3. Verification before the candidate commit

- **`DEC-028` — both specifications amended to 0.39.0, and this is a gate, not
  a follow-up.** `docs/specification/README.md`: *"amended at each release, not
  afterwards, and the release candidate does not tag until they are."* **I
  amended the requirements through this release's work; what is outstanding is
  the header, the `Covers release` line and a `0.39.0` baseline row in both
  documents, plus `external-design.md`'s account of reopen and the two new
  headings.** I do that before you cut; **check it is done and say so in the
  review request.** At 0.38.0 I treated this as post-publication, tagged, and
  amended afterwards — one breach, and this line exists so it is not two.
- `DEC-007` **346**, three consecutive runs.
- **The overflow gate on the extracted tree, 95 cells.**
- `fmt`, `clippy --workspace --all-targets -D warnings`.
- **Both migrations on a database built by the released 0.38.0 binary**, on the
  final tree — `SPRINT-004` §3 and `SPRINT-005` §5 did this against
  intermediate trees; repeat it against the candidate, since `0019` and `0020`
  are what upgraders run in sequence.
- `cargo publish --workspace --dry-run` from a detached worktree of the
  candidate commit.

## 4. Escalate rather than deciding

- **If the count is not 346**, or the gate is not 95, or the block and
  `test.yml` disagree.
- **If `DEC-028` is not done** when you go to cut. Stop and say so.
- **If the two migrations behave differently in sequence** than each did alone.
- **If `find_violations` flags the changelog** — report the phrase, do not
  reword around the check.
- **If §2.4's velocity change is not true of the built artefact.** It is the
  one claim in the changelog that a user could contradict from their own
  screen.

## 5. Exit condition

A candidate commit on `main`, the changelog written, 346 green three times, the
gate at 95, both migrations exercised in sequence from a 0.38.0 database, the
dry run clean, and **both specifications covering 0.39.0**. Then stop and hand
it back.
