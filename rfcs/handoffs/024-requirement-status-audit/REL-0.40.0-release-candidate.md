# REL-0.40.0 — release candidate

**Contents**: `LAYOUT-010` (`14d1b1e`), `GATE-001` (`7fd8141`), `CAL-004`
(`0ab5600`), `PERF-002` (`3a1326a`), `LAYOUT-011` (`328bcc8`), `GATE-002`
(`5477a3e`), `GATE-003` (`f47c750`), `GATE-004` **withdrawn** (`b66e6ce`,
`35eddae`), and `REQ-001`'s report worked into the record (`7b51a3e`).
**`DEC-028` is already done** — both specifications cover 0.40.0 (`9d5c4ee`).
**Depends on**: nothing outstanding.

**Do not tag. Do not publish.** Produce the candidate and stop. **This is the
only work in flight.**

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.

**No migration** — `0020` remains the most recent. Say so; the last two
releases each carried one and a reader will be checking.

**Expected: 352**, up from 0.39.0's 346. **No new test file**, so
`DEC-007`'s block and `test.yml` are unchanged — the first release in three
where that is true.

**The overflow gate is 120 cells, not 95** — 24 pages at five widths, after
four fixture changes. `browser-checks/README.md` and the specification both
carry it.

## 2. The changelog

**Almost nothing a user can see changed, and the release is still substantial.**
That is the opposite of 0.39.0's problem and it needs the opposite care: do not
pad it into sounding like a feature release, and do not let "no behaviour
changed" make it sound like nothing happened. Six things.

1. **Lead with what a user *can* see**, because it is short and real: **three
   surfaces stop overflowing on long names** — the sprint plan's backlog
   filter, and the assignee name on the board, the workload strip and issue
   detail. **One of them overflowed a 1280 px desktop**, not only phones. No
   screen, route, state or copy changed.

2. **The release's subject is the record being checked against the code.**
   `REQ-001` audited every requirement whose status is a claim. **Sixteen were
   stale**, all understating what ships. **One was in the other direction** —
   `FR-HLT-006` recorded that no automated vocabulary guard exists, and one
   does, which is the kind that causes work rather than merely failing to
   prevent it. **Two entries contradicted each other** about whether the health
   score badge exists.

3. **Say why that was worth a release slot**, in one sentence: a requirement
   nobody believes is live is a requirement nobody checks — and **the previous
   release lost two handoffs to exactly that**, one shipped to `main` and half
   reverted, one written and withdrawn unbuilt.

4. **`PRAGMA optimize` on connection close** — a running-system behaviour change
   and the only one here. **Give the real reason**: not the 7× it buys on one
   health query, but that install shapes cannot be predicted, and the
   alternatives both require predicting them. **Mention the near-miss**: the
   SQLite manual's own recommended `analysis_limit` of 400 was measured
   **worse than no statistics at all**, and shipping it would have been a
   regression.

5. **The gate grew 95 → 120 cells** because its fixture three times did not
   render the branch a defect was sitting behind — an empty sprint, a team with
   no projects, unassigned issues. **`§10.32`** records it, with a stopping
   rule that was tested immediately: `GATE-004` was **withdrawn under it**.

6. **`FR-CAL-007`, a P0, reaches Met.** Its behaviour always held; its mandated
   guard checked a different property, so the concepts it names could have
   appeared in words carrying no quantity and the guard would have stayed
   green.

**What a reader should not conclude**, folded in:
- **`§10.15`, `§10.17` and now `§10.32` are open.** `§10.32` is open **by
  decision** — the stopping rule is deliberate, not a gap nobody got to.
- **A green gate cell still means one thing**: no horizontal overflow. The
  calendar's day view is now *visited* and still not *checked* for the vertical
  collapse `CAL-003` found — **visiting is not checking**, and the README says
  so.
- Four requirements now recorded Met ship with **no test pinning the specific
  limb they name** — named in the record, not hidden.
- No WCAG conformance claim.

**Run `find_violations`** over the finished section and report the character
count. **"velocity" is still prohibited**; 0.39.0's escalation is the
precedent.

## 3. Verification before the candidate commit

- **`DEC-028`: already done** (`9d5c4ee`). **Confirm it and say so** — do not
  skip the check because I claim it.
- `DEC-007` **352**, three consecutive runs; block and `test.yml` unchanged.
- **The overflow gate on the extracted tree: 120 cells.** Report the **run
  time** — `§10.32` has two numbers and this is the third, on the artefact
  rather than a working tree.
- `fmt`, `clippy --workspace --all-targets -D warnings`.
- **No migration to exercise**, but **start the candidate binary on a database
  built by the released 0.39.0 binary** and confirm it comes up at migration 20
  with nothing applied — a release that adds none should be shown to add none.
- **`PRAGMA optimize` on the artefact**: a used install writes `sqlite_stat1`
  on close, and a fresh one does not at startup. One check; it is the only
  runtime behaviour change.
- `cargo publish --workspace --dry-run` from a detached worktree.

## 4. Escalate rather than deciding

- **If the count is not 352**, or the gate is not 120.
- **If `find_violations` flags anything** — report the phrase, do not reword
  around it.
- **If the gate's run time has grown materially** on the extracted tree.
- **If §2.1's three surfaces are not actually fixed** in the built artefact.
  They are the only user-visible claims in the section.

## 5. Exit condition

A candidate commit on `main`, the changelog written, 352 green three times, the
gate at 120 with its run time, the 0.39.0 database coming up clean with no
migration applied, `optimize` shown working on the artefact, and the dry run
clean. Then stop and hand it back.
