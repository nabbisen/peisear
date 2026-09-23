# REL-0.36.0 — release candidate

**Contents**: `CAL-003` (`0f31c08`) — the day-view reschedule. Reviewed and
closed. **Depends on**: nothing outstanding.

**Do not tag. Do not publish.** Produce the candidate and stop.

## 1. Ordinary cut

A version-bump-and-changelog commit on `main`'s tip. `Cargo.toml`,
`Cargo.lock`, `CHANGELOG.md` only.

**No migration.** `0017` remains the most recent. The reschedule writes columns
that migration `0016` added at 0.23.0; nothing new was needed, which is what
RFC 0004's substep contract predicted when it said only D-5 needs schema.

**Expected: 268**, up from 0.35.0's 262 — eleven new tests across two existing
files, so `DEC-007`'s command block is unchanged. If the block needs an edit,
that is an escalation.

## 2. The changelog

Six things, in this order.

1. **Lead with what the user can now do.** On the calendar's **day view**, an
   issue block can be dragged up or down to move the appointment to a new time.
   **Both planned timestamps move together**, so the appointment keeps its
   length and the block keeps its height — say that, because it is the
   behaviour a reader would otherwise have to test to learn. Movement snaps to
   fifteen minutes. A five-second Undo follows each move.

2. **Name the boundary in the same breath, briefly.** It is **the day view
   only** — the one view whose vertical axis is time, so a drag distance means
   a duration; in the week and month views it would mean nothing. And it
   **moves an appointment, it does not resize one**: a resize needs a grab
   handle at the block's edge, and a handle large enough to meet the product's
   own 44 px touch-target floor would not fit inside a fifteen-minute block
   without destroying the proportionality that makes the day view readable.
   **One clause for the reason, not a paragraph** — the point is that the limit
   is principled, not that the reasoning is interesting.

3. **The plain path, and it is different from last release's.** 0.35.0 could
   say "the buttons are still there beside the drag". **Here there is no
   control on the page at all** — the calendar shows links. Rescheduling
   without a pointer means opening the issue and editing its planned dates,
   which is exactly what it was before this release. **So say what a phone user
   does**, rather than only that the drag is unavailable to them: drag-and-drop
   does not fire for touch, and the issue's own edit form is the path, as it
   always was.

4. **A conflict is now possible, and 0.35.0's entry said the opposite about
   its own feature.** If someone else changed the issue while the calendar was
   open, the move is refused and the page reloads showing the current state,
   rather than overwriting their change. **This is not a new hazard** — it is
   the same optimistic-lock behaviour the issue edit form and the board have
   had for releases, now reaching a third surface. Say it plainly and do not
   dress it as a feature.

5. **Internal, one line.** The move goes through a narrow endpoint that carries
   only the two timestamps and the version stamp, beside the existing form
   path rather than replacing it.

6. **What a reader should not conclude**, folded in:
   - **The drag itself is executed by no test.** Eleven new tests assert the
     endpoint, the markup and the copy; none performs a drag. The same standing
     limit as the project's other scripts.
   - **No schema changed**, and no data moved.
   - `§10.15` and `§10.17` remain the two open register entries, both open by
     decision.
   - The product still does not claim WCAG conformance.

**Run `find_violations`** over the finished section and report the character
count.

**The caution.** Each recent entry has had its own trap: 0.33.0's was *"layout
is now covered"*, 0.34.0's was reading as paperwork, 0.35.0's was claiming
reach. **This one is different and comes from the sequence rather than the
text: it is the second drag feature in two releases.** An entry that reads as
*"direct manipulation is arriving everywhere"* would be announcing a programme
this project has not committed to — the remaining substep is unscheduled and
needs schema, and nothing has been decided about it. The other half of the same
trap is implying the calendar has become a scheduling application: it does
**one** thing more than it did last week. **The useful frame is a specific,
bounded addition to one view** — not a direction of travel.

## 3. The tarball

`--prefix=peisear-0.36.0/`. **Package-relative checksum** — `sha256sum -c`
passes from inside the tarball's own directory. File list **prefix-stripped**
against `git ls-tree -r --name-only <commit>`, and **say that the strip
happened**.

**`static/calendar.js`** is the second new file under `static/` in two
releases. Confirm it is in the archive, that its `sha256` matches `HEAD`'s, and
that the extracted tree's calendar page references it with `defer`.

**`static/tailwind.css` changed this release, and that is expected — the
opposite of the last two.** For 0.34.0 and 0.35.0 a difference would have meant
a stray regeneration; here a *lack* of difference would mean a missing one.
Confirm all three of these and report them:

- the file regenerates **byte-identical** from the committed source with the
  pinned standalone binary, whose checksum matches the committed table;
- the class-level diff against 0.35.0's cut is **exactly one added rule**,
  `.h-full`, and nothing removed;
- the file in the tarball matches `HEAD`'s.

**Run the gate from the extracted tree**: `cargo build -p peisear` inside it,
then `node browser-checks/overflow-gate.mjs` — **90/90, exit 0, 0 non-local
requests**. Build with `CARGO_TARGET_DIR` on disk rather than under `/tmp`.

Representative tests from inside the extracted tree: **`calendar_surfaces`**,
`response_outcomes`, `optimistic_lock`.

## 4. Post-publication — state as pending

Bare tag `0.36.0`. Seven crates at `max_version` `0.36.0` (`DEC-047`).
**No `gh release create`.**

## 5. `DEC-028` — architect work, in parallel

The baselines rebase to `0.36.0`: `FR-CAL-*` and the day view's externally
observable behaviour, external design `SCR-27/28`, the test inventory, and the
history rows. Mine, and it blocks the tag rather than this candidate.

## 6. Escalate rather than deciding

- **If the count is not 268**, or `DEC-007`'s block needs an edit.
- **If `static/tailwind.css` does not regenerate byte-identical**, or its
  class-level diff is anything other than one added `.h-full`.
- **If `static/calendar.js` does not match `HEAD`'s.**
- **If the gate from the extracted tree is not 90/90.**
- **If `find_violations` flags the changelog.**
- **If §2.4's conflict sentence reads as a warning rather than a fact.** It is
  the one most likely to come out sounding like a caveat about the feature when
  it is a description of behaviour the product has had for releases.

---

**Who holds what**: dev team — the candidate. Architect — the baseline rebase,
then tag and publication after the owner approves. **What's next**: review
request.
