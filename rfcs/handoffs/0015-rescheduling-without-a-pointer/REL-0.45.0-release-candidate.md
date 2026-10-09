# REL-0.45.0 — release candidate

**Contents**: `CAL-004` (`fa9d2ab`) and `CAL-005` (`d9796b4`) — RFC 0015's
control, `DEC-059`; **`FR-DM-002` reaching Met** and RFC 0015 closing
(`3b78b3a`); and the specification amendment with its sweep (`3dad873`).
**Every handoff reviewed and closed.**

**`DEC-028` is already done** — both specifications cover `0.45.0`
(`3dad873`), including §3's sweep. **Depends on**: nothing outstanding.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.

**No migration** — `0020` remains the most recent, the **sixth** release
running.

**Expected: 383.** The arithmetic is worth stating because it is not
monotonic: **376 → 385 → 383**. `CAL-004` added nine; `CAL-005` deleted three
and added one. **The three deletions are the point, not an accident** — they
asserted the day-view/week/month option counts of a `<select>` that no longer
exists, and an assertion kept alive past its subject is `§10.17`. Say *deleted
rather than adapted*, and name them if the sentence can carry it.

**This release does change production source**, unlike 0.44.0: two new
handlers, a new route, a component card, the calendar block's query threading
and five message-table strings. Nothing in the drag path and nothing in
`static/*.js`.

**The overflow gate is 120 cells** and this release put a new control on a
page the fixture sweeps. Run it from the extracted tree, and the trap script
after it.

## 2. The changelog

**This is the release where a P0 closes, and the closing is the story.** Not
the control — the fact that `FR-DM-002` could be *checked* at all. Five
things.

1. **Lead with what a user can now do**: reschedule a calendar block without
   a pointer, from the issue page, by picking a day — and **the length stays
   the same**, because the server applies one whole-day delta to both ends.
   Before this, the only keyboard route was the issue edit form's two
   absolute fields, which meant **computing the new end time by hand**: 77 Tab
   and Enter keystrokes across three pages against one drag.
2. **Then why that counted as a failure at all.** `DEC-059` gave
   `FR-DM-002` a clause one release ago — *offered by the element carrying the
   pointer affordance, and performing the same action rather than requiring
   the user to reconstruct its effect.* **Without that clause this would have
   read Met for five releases**, because the effect was always *reachable*.
   The clause is what made the gap falsifiable, and it is the transferable
   part: every future direct-manipulation surface is held to it.
3. **The stored row is identical to a drag's, by construction.** Both paths
   call the same function; the test moves one issue by form and drags another
   to the same day and compares. Say *by construction, not by agreement* —
   `DM-TEST-001` found the two schedule paths agreeing only because two
   authors made the same choice, and this is the difference.
4. **Two design choices went the other way from the RFC, and both are worth a
   reader's time** (RFC 0015 §9):
   - **Not a control on the block.** A calendar block's height *is* its
     duration, so a 44 × 44 px control cannot fit; and retargeting the
     block's existing link would have served the keyboard by **taking the
     pointer user's route to the issue away**.
   - **Not a dropdown of the days you were looking at.** In the day view that
     list held **one option — today — so a Move did nothing.** The window was
     an artefact of where the user clicked, not a choice they made. A date
     input removed it, **dissolving the question of how many options are too
     many** rather than answering it, and removing 100 net lines.
5. **The sweep's third run found four things and none was a stale status** —
   a test count citing 16 where 21 exist, a sentence counting *both* of
   *three*, a guard that reads `<a>`/`<button>`/`<summary>` and not the
   `<input>` this release added, and a requirement applied beyond its letter.
   **Three of four are counts or populations**, which is pass 4's job, and
   that is the first sign the *amend from the code* rule is holding.

**Do not claim the touch-target guard now covers the new input.** It does
not; the control passes on a browser measurement, and `NFR-A11Y-007` records
that distinction. Widening the guard is unscheduled and the notes should not
imply otherwise.

## 3. Verification before the candidate commit

- **`DEC-028`: already done** (`3dad873`). Confirm both specifications read
  `Covers release: 0.45.0` with a `0.45.0` baseline row each, and say so.
- **The sweep**: run by the architect; its four findings are in `3dad873`.
  **Confirm rather than re-run**, and report *swept, nothing further found*
  or what you found.
- `DEC-007` **383**, three consecutive runs, **from a block extracted out of
  the current `CONTRIBUTING.md`** rather than a saved copy.
- **Both browser scripts on the extracted tree**, in the job's own order: the
  overflow gate at **120 cells** with the move control present, then the trap
  script clean on three surfaces.
- **`mdbook build` from the extracted tarball** — fifth release with the site.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`, `rustdoc-links`
  with **seven** `Documenting` lines and the private-link count still
  **eight**.
- **No migration to exercise**, but start the candidate on a database built by
  the released **0.44.0** binary and confirm it comes up at 20 with nothing
  applied.
- `cargo publish --workspace --dry-run` from a **detached worktree of the
  candidate commit** — seven crates. **Read every `warning:` line rather than
  grepping for two words**; that is how 0.44.0's advisory-absence claim was
  established.
- **A behavioural check is asked for this time**, because production source
  changed and this is a new user-facing control. On the built artefact:
  create an issue with a planned start and end, open a calendar view, follow
  a block's link to the issue, use the **Move** control to shift it a day,
  and confirm **the duration is unchanged** and **you land back on the
  calendar view and date you came from**. Ten seconds, and it is the release's
  entire headline.

## 4. Escalate rather than deciding

- **If the count is not 383**, or the gate is not 120.
- **If the behavioural check lands you on a default calendar view** rather
  than the one you came from — that is `FR-NAV-005`'s application failing in
  the one place this release claims it.
- **If the duration changes** on a Move.
- **If `rustdoc-links`' private-link count is not eight** — `test.yml`'s
  comment and `NFR-REL-008` both cite that number.
- **If the sweep turns up a fifth finding.**

## 5. Exit condition

A candidate commit on `main`; a changelog whose lead is what a user can now
do and whose second beat is why it was a failure at all; 383 green three
times from a freshly extracted block; the gate at 120 and the trap script
clean; the book building from the tarball; the 0.44.0 database coming up
clean; a dry run of seven crates with every warning line read; and **the
Move demonstrated on the artefact, duration preserved, landing back where you
started.** Then stop.
