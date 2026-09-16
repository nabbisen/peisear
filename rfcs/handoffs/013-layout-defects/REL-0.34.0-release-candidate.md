# REL-0.34.0 — release candidate

**Contents**: `LAYOUT-009` and its round 2 (`b458943`, `fbe5a6b`), `TT-005`
(a measurement, no source change), `TT-006` (`05030d1`, a doc comment).
All reviewed and closed. **Depends on**: nothing outstanding.

**Do not tag. Do not publish.** Produce the candidate and stop.

## 1. Ordinary cut

A version-bump-and-changelog commit on `main`'s tip. `Cargo.toml`,
`Cargo.lock`, `CHANGELOG.md` only.

**No migration.** `0017` remains the most recent.

**Expected: 256, unchanged.** Nothing here adds a test, and `TT-005`
deliberately added none — it was a measurement, and the reason it produced no
guard is §2's fourth point.

## 2. The changelog

**This release is smaller than the last four and it is a different shape from
all of them.** One user-visible fix, and two corrections to the project's own
published record. Five things, in this order.

1. **Lead with the headers, because that is what a user gets.** Page headers
   that pair a title with actions now wrap instead of squeezing the title.
   Give the numbers for the worst case: a sprint detail page at 320 px used to
   render its title in a **54 px column, eleven lines deep**, with an ordinary
   sprint name; it now takes the full width in two lines, with the actions
   below and every control still 44 px. Issue detail and team detail are the
   same fix. **Say that nothing overflowed before and nothing overflows now** —
   this was never a horizontal-scroll defect, which is exactly why the gate
   that ships in 0.33.0 was green on it the whole time, and that sentence is
   worth writing because the previous release's headline was that gate.

2. **The first correction: two defects this project recorded did not exist.**
   `§10.19` recorded three overlapping control pairs found by a browser
   inspection on 2026-09-06. One is real. **The two involving a disclosure
   toggle could not be reproduced — not on the current tree, and not on the
   2026-09-06 tree they were measured from**, rebuilt and served with the
   stylesheets it shipped with, by two independent probes each validated by
   planting an overlap of known size and finding it. They are withdrawn.
   **State that what produced the original numbers is still not identified**,
   because it is not, and a withdrawal that invents an explanation is worth
   less than one that does not.

3. **The second correction, and it is the one that was published.** The 0.31.0
   entry told readers that the touch-target rule's adjacency half — *that two
   touch targets must not overlap* — was **"structurally guaranteed for every
   control now in the tree"**. That was too broad when it was written and has
   never been corrected here. **There is one overlap in the product**:
   segmented buttons in a `join` group share a 1 px column, because the
   grouping is made by collapsing their adjacent borders. It is deliberate, it
   is the same on every tree tried across seven releases, and a tap on the seam
   reaches one of the two — an adjacent member of the group the user was aiming
   at. The requirement now says so rather than claiming otherwise. **Correct
   the claim plainly and do not inflate it**: a 1 px seam between visually
   contiguous segments is not a defect a user can encounter, and the thing that
   was wrong was the word "every".

4. **Why no new guard came out of any of this** — one sentence, because it is
   the release's most useful line. Overlap is not becoming a gate assertion:
   the measurement found one overlap class and it is deliberate, so an
   overlap gate would ship with an exception on its first day. The register
   has the fuller reason; the entry needs the conclusion.

5. **Internal, brief.** A structural guard's own doc comment claimed the guard
   had no exception list while it had carried one for two releases. The list is
   three call sites, each decided and each documented beside the code; the
   defect was the denial. Corrected, doc only, no behaviour change.

**What a reader should not conclude**, folded in rather than appended:

- **The suite did not grow**, and this time not one line of product behaviour
  was added either. That is what the release is.
- **`§10.15` and `§10.17` are the two remaining open register entries**, and
  both are open by decision rather than by schedule — the shipped JavaScript
  is executed by no test and cannot usefully be, and assertions that decay
  while staying green have no gate that would catch them.
- The product still does not claim WCAG conformance.

**Run `find_violations`** over the finished section and report the character
count.

**The caution.** 0.31.0 risked reading as an achievement, 0.32.0 as an apology,
0.33.0 as *"layout is now covered"*. **This one risks reading as a release
about paperwork** — or, in the other direction, as a confession that the
accessibility work was unsound. It is neither. A real mobile defect is fixed on
three pages, and two claims this project made about itself are corrected
because checking found them wrong. **The useful frame is that the record gets
the same treatment as the code.** If the draft buries the header fix under the
corrections, it is wrong; if it mentions the corrections in passing, it is also
wrong.

## 3. The tarball

`--prefix=peisear-0.34.0/`. **Package-relative checksum** — `sha256sum -c`
passes from inside the tarball's own directory. File list **prefix-stripped**
against `git ls-tree -r --name-only <commit>`, and **say that the strip
happened**.

`static/tailwind.css` must match `HEAD`'s byte for byte — no class changed this
release, so a difference would mean a stray regeneration rather than a missing
one, and it is worth one `sha256sum` either way.

**Run the gate from the extracted tree**: `cargo build -p peisear` inside it,
then `node browser-checks/overflow-gate.mjs` — **90/90, exit 0, 0 non-local
requests**. Build with `CARGO_TARGET_DIR` pointed somewhere on disk rather than
under `/tmp`; a full target tree is ~20 GB and the scratch filesystem is a
30 GB tmpfs, which I filled doing exactly this for 0.33.0 and spent a while
reading a link failure as a tree defect.

Representative tests from inside the extracted tree: **`touch_target`**,
`sprint_plan`, `confirmation`.

## 4. Post-publication — state as pending

Bare tag `0.34.0`. Seven crates at `max_version` `0.34.0` (`DEC-047`).
**No `gh release create`.**

## 5. `DEC-028` — architect work, in parallel

The baseline and external design rebase to `0.34.0`: `§10.19` and `§10.28`
closed, `§10.27` closed in tree, `NFR-A11Y-007`'s amended adjacency clause, and
external design `§17.8`'s overlap decision. Mine, in flight; it does not block
the candidate and it does block the tag.

## 6. Escalate rather than deciding

- **If the count is not 256.**
- **If `static/tailwind.css` differs from `HEAD`'s.**
- **If the gate from the extracted tree is not 90/90.**
- **If `find_violations` flags the changelog.**
- **If the 0.31.0 correction in §2.3 reads as either bigger or smaller than it
  is.** That paragraph is the one I would most like a second opinion on, and
  disagreeing with my framing is a useful review result rather than an
  obstruction.

---

**Who holds what**: dev team — the candidate. Architect — the baseline rebase,
then tag and publication after the owner approves. **What's next**: review
request.
