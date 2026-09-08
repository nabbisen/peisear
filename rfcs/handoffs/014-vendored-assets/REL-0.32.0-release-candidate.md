# REL-0.32.0 — release candidate

**Contents**: `LAYOUT-001`, `LAYOUT-002`, `TT-004` (three rounds), `ASSET-001`.
All reviewed and closed. **Depends on**: nothing outstanding.

**Do not tag. Do not publish.** Produce the candidate and stop.

## 1. Ordinary cut

A version-bump-and-changelog commit on `main`'s tip — no branch, no boundary,
none of 0.30.0's difficulty. `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.

**No migration.** `0017` remains the most recent.

**Expected: 255.** Twelve scan modules, unchanged.

## 2. The changelog, and the one thing it must not do

This release has an unusual shape and the entry has to carry it honestly.

1. **A self-hosted instance was rendering unstyled without internet access, and
   now is not.** Lead with it. `.btn` at 17 px instead of 44, Times New Roman,
   the account menu unable to collapse — measured, with both CDNs blocked.
   **Say plainly that nothing was broken**: every link worked, every form
   submitted, the no-JavaScript path held. It was a degradation, not a failure,
   and overstating it would be as wrong as having missed it. `NFR-CMP-002` said
   *self-hostable, Implemented*; the claim had never been checked.
   451 KB of runtime JIT compiler → **14 KB of static CSS**; DaisyUI vendored as
   the prebuilt stylesheet it already was. **State the byte figures** — raw and
   gzipped, both files — rather than implying an improvement nobody measured.

2. **Two layout defects fixed.** Every authenticated page scrolled sideways when
   the signed-in email was long — a flex item's content-based minimum width, not
   the dropdown's position, and **conditional on the account**, not universal.
   And the project toolbar could not wrap below 390 px. **Neither was caused by
   the touch-target work**; that was suspected and measured false.

3. **The touch-target rule now covers every interactive element**, not the
   class-carrying subset. `DEC-050` replaced a named limit — measured and found
   to be an account menu on every page, `<summary>` toggles at 16 px, and the
   product's own indicator basis links — with **one rule and one declared
   exception**. Three call sites are excluded by design and the entry should name
   why for at least the calendar: its event blocks are sized in proportion to an
   appointment's duration, so a 44 px floor would make a fifteen-minute meeting
   look like a two-hour one.

4. **Definition of Done item 5 moved** — the oldest condition in that table,
   unmoved since 0.19.1 — to *"Met, with mobile completion outstanding"*.
   **`NFR-A11Y-006` is still open**, and the entry says so in the same sentence.

5. **What a reader should not conclude**, folded in, not appended:
   - **The suite did not grow.** 254 → 255, and three of the four handoffs added
     **no tests at all**, because nothing here observes rendered layout
     (`§10.15`'s counterpart, now `§17.8`). Every defect above was found by a
     person looking at the product with a browser, once. **That is not a gate
     and this release does not add one.**
   - **`§10.21` is open** — a 314 px overlap on issue rows, visible only when a
     project has content.
   - The product still does not claim WCAG conformance.

**Run `find_violations`** over the finished section and report the character
count.

**The caution, and it is the opposite of 0.31.0's.** That release risked reading
as an achievement announcement. **This one risks reading as an apology.** Four
defects shipped for many releases and were found in three days — the useful frame
is *what changed about how we look*, not *what we got wrong*. If the draft reads
as either a triumph or a confession, it is wrong.

## 3. The tarball

`--prefix=peisear-0.32.0/`, the standard since 0.30.0. **Package-relative
checksum** — `sha256sum -c` passes from inside the tarball's own directory.
File list **prefix-stripped** against `git ls-tree -r --name-only <commit>`, and
**say that the strip happened**.

**Two new files are in this archive and both matter**: `static/tailwind.css` and
`static/daisyui.min.css`. Confirm both are present and that the extracted tree's
`layout.rs` references neither CDN.

Representative sample inside the extracted tree: **`touch_target`**,
`confirmation`, `board_keyboard`.

## 4. Post-publication — state as pending

Bare tag `0.32.0`. Seven crates at `max_version` `0.32.0` (`DEC-047`).
**No `gh release create`.**

## 5. `DEC-028` already satisfied

Baseline and external design are amended and rebased to `0.32.0` — the requirement
status, Definition of Done item 5, `§10.18`–`§10.23`, `NFR-CMP-002`, and external
design `§5.7`, `§17.7` and the new `§17.8`. Architect work, already done, second
release running.

## 6. Escalate rather than deciding

- **If the count is not 255.**
- **If the tarball is missing either vendored CSS file** — the release would
  ship an application that renders unstyled, which is the defect this release
  exists to fix.
- **If `find_violations` flags the changelog.**

---

**Who holds what**: dev team — the candidate. **What's blocked**: tag and
publication, architect-side after the owner approves. **What's next**: review
request.
