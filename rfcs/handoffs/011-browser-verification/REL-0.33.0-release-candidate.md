# REL-0.33.0 — release candidate

**Contents**: `BROWSER-001`, `BROWSER-002` (both halves), `LOCK-001`,
`LAYOUT-003`, `LAYOUT-004`, `LAYOUT-005`, `LAYOUT-006`, `LAYOUT-007`,
`A11Y-006` (superseded by the architect's direct verification, `NFR-A11Y-006`
Met). **Depends on**: `LAYOUT-007` reviewed and closed. **Do not start before
that review is in `.git-exclude/reviewed/`.**

**Do not tag. Do not publish.** Produce the candidate and stop.

## 1. Ordinary cut

A version-bump-and-changelog commit on `main`'s tip. `Cargo.toml`,
`Cargo.lock`, `CHANGELOG.md` only.

**No migration.** `0017` remains the most recent.

**Expected: 256.** Twelve scan modules, unchanged. The one new test is
`board_card_lock_value` (`LOCK-001`); everything else in this release added
none, and §2 says why that is the point rather than a gap.

## 2. The changelog, and the one thing it must not do

This is the release where the rendered layout is observed by a check for the
first time. The entry has to say exactly what that check is and, in the same
breath, exactly what it is not.

1. **Lead with the gate, sized honestly.** One assertion —
   `scrollWidth <= clientWidth` — on fourteen rendered pages at five widths,
   run as a CI job on the same footing as `fmt` and `clippy`, **not** a
   `cargo test`, and **not counted** in the 256. It is driven by a
   dependency-free Chrome DevTools Protocol harness; the only new artefact is
   `browser-checks/`. Say what the assertion is and that it is the only one:
   it does not observe overlap, target size, or anything visual. Say that it
   was watched go red before it was trusted — three planted defects and the
   navbar's — and that it reports every failing cell, not the first.

2. **Say what the gate could not see, because that is the release's actual
   lesson.** The gate was green on a tree with three overflow defects, because
   its fixture title had a space at every point. It was green on two more
   pages because they were not in its list. It was green on a third pair
   because its fixture *display name* had a space at every point. **What the
   gate sees is exactly its fixture and its page list**, and three times this
   release that was the gap. The fixture now carries a 64-character unbroken
   run in the issue title, project name, team name and display name; the
   page list is fourteen. State both so a reader can judge the coverage
   rather than infer it from the word "gate".

3. **Fixed, as a class, not a list of nine.** Unbreakable user text — a long
   run with no space — overflowed nine surfaces in **two shapes** whose
   remedies are not interchangeable: a flex or grid item whose content sets
   its own minimum width, and an ordinary block whose text overflows a
   correctly sized box. Applying the wrong remedy is silent in both
   directions, and one site in committed code already carried the wrong one.
   The rule is recorded once in the source. Name the surfaces briefly (board
   columns, issue detail, both delete interstitials, search results, teams
   list, team detail, project calendar, and the `/today` and `/settings`
   subtitles), and say plainly that **every one is conditional on content** —
   a title or name with a long unbroken run — not universal.

4. **Fixed, separately: every page overflowed a 320 px phone by 24 px for a
   user with a 21-character display name.** Not unbreakable text — a
   navigation row that could not give anything up. The name now truncates
   with an ellipsis only when the row cannot fit; at every wider width it is
   whole. **This is the defect the 320 px width exists to catch**, and the
   entry should say that 320 was held back one round until it could land
   green rather than quarantined.

5. **Definition of Done item 5 → Met**, the oldest condition in that table,
   open since 0.19.1 and now complete. `NFR-A11Y-006` was verified by driving
   each named flow to completion with emulated touch at three phone
   viewports, both with and without JavaScript for the status change.
   **Carry the three limits in the same sentence**: the flows verified are
   the ones the requirement names and no others; one browser (Chromium);
   emulated touch, not a device — no soft keyboard, no focus-driven viewport
   resize. And `NFR-A11Y-007` keeps its three call sites excluded by design,
   recorded — a met requirement with documented exceptions, not an unmet one.
   **Do not let "Met" stand alone.**

6. **Internal: `RFC 011` is complete.** Step 3 was withdrawn as mis-classified
   (`DEC-052`) — the JavaScript it would have covered was covering for a
   server-side guarantee, and that guarantee is now asserted directly: every
   board card renders a non-empty lock value. One test. The other three steps
   shipped across 0.29.0–0.33.0.

7. **What a reader should not conclude**, folded in, not appended:
   - **One property is gated, and only one.** Overlap (`§10.19`, open),
     target size as rendered, and everything visual remain unobserved by any
     check.
   - **The suite grew by one** while nine layout defects were fixed. That is
     correct: the layout is now observed by the gate, which is outside the
     suite by design, and a `cargo test` that reads markup cannot observe it.
   - **`§10.17` is open** — assertions that decay while staying green — and
     nothing here changes that.
   - The product still does not claim WCAG conformance.

**Run `find_violations`** over the finished section and report the character
count.

**The caution.** 0.31.0 risked reading as an achievement; 0.32.0 as an apology.
**This one risks reading as "layout is now covered."** It is not. One
assertion, fourteen pages, five widths, one fixture — and the release's own
history shows that the fixture and the list are the coverage. If the draft
lets a reader believe the browser now checks the product, it is wrong; if it
buries the gate under its own caveats, it is also wrong. The useful frame is
*what changed about what we can see*.

## 3. The tarball

`--prefix=peisear-0.33.0/`. **Package-relative checksum** — `sha256sum -c`
passes from inside the tarball's own directory. File list **prefix-stripped**
against `git ls-tree -r --name-only <commit>`, and **say that the strip
happened**.

**Three things in this archive matter specifically:**

- `browser-checks/` — `cdp.mjs`, `overflow-gate.mjs`, `README.md`. Confirm
  all three are present.
- `static/tailwind.css` **at the size `HEAD` carries** (`sha256sum` against
  the tree). A stale vendored stylesheet would ship an application whose
  navbar fix does not render — the classes it needs did not exist before
  `LAYOUT-006`.
- `crates/peisear-web/tests/board_card_lock_value.rs`.

**Run the gate from the extracted tree**, not only the tests: `cargo build`
inside it, then `node browser-checks/overflow-gate.mjs` — **70/70, exit 0,
0 non-local requests**. This release's headline is that archive carrying a
working gate; a tests-only sample would not show it.

Representative test sample inside the extracted tree: **`board_card_lock_value`**,
`touch_target`, `confirmation`.

## 4. Post-publication — state as pending

Bare tag `0.33.0`. Seven crates at `max_version` `0.33.0` (`DEC-047`).
**No `gh release create`.**

## 5. `DEC-028` — architect work, before the tag

Baseline and external design rebase to `0.33.0`: `§10.24`–`§10.26` closed,
`§10.25` closed once `LAYOUT-007` lands, external design `§17.8` moves from
*open, narrowing* to *one property gated*, and the Definition of Done row and
`NFR-A11Y-006` carry their 2026-09-10 wording. In progress; not the dev
team's.

## 6. Escalate rather than deciding

- **If the count is not 256.**
- **If the tarball lacks `browser-checks/`, or `static/tailwind.css` does not
  match `HEAD`'s.**
- **If the gate from the extracted tree is not 70/70.**
- **If `find_violations` flags the changelog.**

---

**Who holds what**: dev team — the candidate, after `LAYOUT-007`. **What's
blocked**: tag and publication, architect-side after the owner approves.
**What's next**: review request.
