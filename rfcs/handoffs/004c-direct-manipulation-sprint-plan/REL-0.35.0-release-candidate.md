# REL-0.35.0 — release candidate

**Contents**: `PLAN-002` — rounds 1 and 2, and the test pinning
(`56f1986`, `4d1b1f9`, `637f5a2`). Reviewed and closed. **Depends on**:
nothing outstanding.

**Do not tag. Do not publish.** Produce the candidate and stop.

## 1. Ordinary cut

A version-bump-and-changelog commit on `main`'s tip. `Cargo.toml`,
`Cargo.lock`, `CHANGELOG.md` only.

**No migration.** `0017` remains the most recent.

**Expected: 262**, up from 0.34.0's 256 — six new tests in an existing file, so
`DEC-007`'s command block is unchanged. If the block needs an edit, that is an
escalation.

## 2. The changelog

**This is the first release in six with something a user can see.** 0.29.0 was
the last one with an `### Added` section; 0.30.0 through 0.34.0 were defect and
quality work, deliberately and for good reasons that those entries give. The
0.34.0 baseline named the drought as a thing to decide rather than drift into,
and this is the decision taking effect. **Say that plainly, once, without
apologising for the five releases** — they found real defects and the entry
should not relitigate them.

Five things, in this order.

1. **Lead with the drag.** On the sprint planning screen, an issue can be
   dragged from the team backlog into the sprint and back out. The move shows
   immediately and the request goes out behind it, so a planner working through
   a backlog is not waiting for a page load per decision. A five-second Undo
   follows each move.

2. **Say what did not change, in the same breath, because it is the point.**
   **The move buttons stay.** They are the path without JavaScript, the path
   from the keyboard, and — this one matters and is easy to get wrong — **the
   path on a phone**. HTML drag-and-drop does not fire for touch input, so the
   drag is a pointer affordance and nothing else. **The entry must not say or
   suggest that anything can now be dragged on a phone.** This is the
   cross-cutting rule added to the direct-manipulation RFC while this substep
   was being written, and it exists because it would otherwise be claimed by
   accident.

3. **A real bug fixed alongside it.** Moving an issue out of a sprint with the
   button silently discarded the backlog filter, while moving one *in* kept it
   — the remove form omitted three fields its own handler already accepted. A
   planner who had filtered the backlog to one project lost that filter every
   time they took something out. Fixed, and it is a fix a user feels
   independently of the drag.

4. **One toast at a time, and what that costs.** A planner may make several
   moves in a few seconds, so each move replaces the previous toast rather than
   stacking. **The replaced move is no longer undoable through the toast** —
   the buttons still reverse it. Say the cost; it is the same bargain the
   five-second timeout already makes, and a reader should not discover it.

5. **What a reader should not conclude**, folded in rather than appended:
   - **The drag itself is executed by no test.** The suite grew by six and not
     one of them performs a drag: they assert the markup, the copy island and
     the server. This is the same standing limit the project has recorded for
     its other scripts, and it is stated rather than implied.
   - **Nothing about concurrency changed.** Two planners editing one sprint
     converge rather than conflict, because the underlying row carries no
     version to compare; there is no new conflict path and none was added.
   - `§10.15` and `§10.17` remain the two open register entries, both open by
     decision.
   - The product still does not claim WCAG conformance.

**Run `find_violations`** over the finished section and report the character
count.

**The caution.** The last four entries each had their own trap; this one's is
**claiming reach the feature does not have**. A drag is the kind of thing that
sounds like it works everywhere, and here it works with a pointer, on one
screen, for members and admins, on a sprint still being planned. An entry that
leaves a reader thinking they can reorganise a sprint on their phone is wrong
even if every sentence in it is individually true. The useful frame is *a
faster path for the people already doing this work, beside the one that was
always there.*

## 3. The tarball

`--prefix=peisear-0.35.0/`. **Package-relative checksum** — `sha256sum -c`
passes from inside the tarball's own directory. File list **prefix-stripped**
against `git ls-tree -r --name-only <commit>`, and **say that the strip
happened**.

**`static/plan.js` is a new shipped artefact** and the first new file in
`static/` since the CSS was vendored. Confirm it is in the archive, that its
`sha256` matches `HEAD`'s, and that the extracted tree's sprint plan page
references it with `defer`.

`static/tailwind.css` must still match `HEAD` byte for byte. No class changed
this release, so a difference means a stray regeneration rather than a missing
one.

**Run the gate from the extracted tree**: `cargo build -p peisear` inside it,
then `node browser-checks/overflow-gate.mjs` — **90/90, exit 0, 0 non-local
requests**. Build with `CARGO_TARGET_DIR` on disk rather than under `/tmp`.

Representative tests from inside the extracted tree: **`sprint_plan`**,
`touch_target`, `confirmation`.

## 4. Post-publication — state as pending

Bare tag `0.35.0`. Seven crates at `max_version` `0.35.0` (`DEC-047`).
**No `gh release create`.**

## 5. `DEC-028` — architect work, in parallel

The baseline and external design rebase to `0.35.0`: the test inventory, the
sprint plan's externally observable behaviour in external design `§3`, and the
history rows. Mine, and it blocks the tag rather than this candidate.

## 6. Escalate rather than deciding

- **If the count is not 262**, or `DEC-007`'s block needs an edit.
- **If `static/plan.js` or `static/tailwind.css` does not match `HEAD`'s.**
- **If the gate from the extracted tree is not 90/90.**
- **If `find_violations` flags the changelog.**
- **If §2.2's reach caveat reads as a disclaimer rather than a fact.** It is
  the sentence I most expect to come out wrong, and telling me it reads badly
  is a useful review result.

---

**Who holds what**: dev team — the candidate. Architect — the baseline rebase,
then tag and publication after the owner approves. **What's next**: review
request.
