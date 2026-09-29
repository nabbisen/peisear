# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.41.0] — 2026-09-29

### Highlights

- **A keyboard user changing a team member's role no longer demotes them on
  the way.** Moving Admin → Viewer used to commit the intermediate step
  (Admin → Member) on the first arrow key; the control now has a Save button
  — the first time this form has worked without JavaScript at all, because it
  never had a submit control before.
- **Undo is now three Tab presses away, not fifteen.** The five-second undo
  toast used to be appended to the end of the page; it now sits next to the
  control it belongs to, and focus never lands on `body` when it is
  dismissed or expires.
- **`/settings/notifications` no longer offers a preference for a
  notification that can never arrive.** Its presence was also holding back
  the *"everything is silenced"* banner until a user silenced something that
  did not exist.
- **The documentation is now a published site**, and **each release's notes
  now live on its own GitHub Release page** as well as in this file.
- No schema migration; no public item removed; no signature changed.

### Fixed

- **The team role `<select>` committed a change on `onchange`, which fires on
  every arrow key** (`components/teams.rs`). A keyboard user moving a member
  from Admin to Viewer passed through Member on the way, and the page
  submitted that intermediate demotion before the arrow keys reached Viewer.
  The control now shares its form with an explicit *Save* button and
  `onchange` is gone. **This was also the only way the role could be changed
  without JavaScript** — the form had no submit control at all until now, so
  this is a second, independent fix riding along with the first.
- **The undo toast on the board, the sprint plan and the calendar was not
  reachable from the keyboard in any realistic sense.** It was appended to
  the end of `<body>`, five Tab presses away on the sprint plan, thirteen on
  the board, and **fifteen on the issue list** — inside the toast's own
  five-second lifetime. It now sits in DOM order immediately after the
  control it belongs to (still drawn bottom-right; this is a DOM-order change
  only), and pressing Undo or letting it expire returns focus to that
  control rather than dropping it to `body`, but only when focus was inside
  the toast to begin with — a user who has moved on is never interrupted.
  - **A second, independent defect turned up in the same toast**: on the
    board, the calendar and the sprint plan, the element the toast now lives
    inside is `draggable="true"`. A real mouse press-and-drag on Undo — not a
    plain click — was being read by the browser as the start of that
    element's own drag: the click that activates Undo never fired, and on the
    board a stray drag could silently move the card to wherever the toast
    happened to be drawn. Fixed with a `mousedown` handler on the button
    itself; a plain click is unaffected, and each surface's own drag still
    works exactly as before, toast present or not.
- **A notification kind with no emitter was offered a preference anyway**
  (`project_trend_decline` — its detection was never built; `ROADMAP.md`
  still lists it as deferred). `/settings/notifications` rendered a row for
  it, and *"everything is silenced"* required a user to silence a
  notification that could never fire before the product would say so. The
  kind is out of the list that is offered and saved; its constant, label and
  strings stay in place for whoever builds the emitter.

### Changed

- **The documentation under `docs/` is now published** at
  `https://nabbisen.github.io/peisear/`, built from `main` by its own
  workflow and failing that build on a broken internal link (`DOCS-001`,
  `DEC-057`). No page moved and no inbound link elsewhere in the repository
  broke; sixteen links that pointed outside `docs/` (and four more into the
  excluded `docs/specification/history/`) now point at GitHub instead, pinned
  to `main`. Publishing `docs/static-js-verification.md` here is not a
  reversal of `STATIC-001` — that decision was about every self-hoster's own
  deployment serving the file, not about this project's own site carrying it.
- **A release now has four legs, not three**: tag, publish, then create the
  GitHub Release with the version's own changelog section as its body
  (`DEC-056`). The publish comes before the Release so the page can never
  announce a version whose crates never reached the registry.

### Internal

- **Two P1 requirements stopped being unfalsifiable.** `NFR-A11Y-002` and
  `NFR-A11Y-004` both read *"Partial."* and nothing else — no statement of
  what was partial, so nothing about them could be checked and nothing about
  them could ever be wrong. Auditing them (`A11Y-001`) is what found the two
  defects above. `NFR-A11Y-002` now governs changes made *within* a
  document and names where focus must land; what a full-page navigation owes
  instead is split out as `NFR-A11Y-009`, because arguing the two as one
  requirement had been hiding a real cost — eleven Tab presses and no skip
  link after an ordinary form submission, on every page that does one.
  `NFR-A11Y-004`'s *"label **and** icon"* is settled toward its own title:
  colour is never the only carrier, and a label alone satisfies it. Its
  remaining gap, unchanged by this release, is recorded rather than fixed:
  both charts separate their two series by lightness alone (1.77 : 1 on the
  burndown), with no patterning.
- **`NFR-REL-007` was recorded `Implemented`, and neither half of it was
  true.** It required documentation to live under `docs/src` in an
  mdbook-compatible structure; this release's own documentation work builds
  the book from `docs/` itself, not `docs/src/`. The second false
  `Implemented` in three releases, and the first confirmed instance of the
  blind spot `REQ-001` named in its own words: that audit's sweep covered
  every requirement claiming *incompleteness*, and said plainly that it
  proved nothing about the requirements claiming to be *done*. This is one.
- **`REQ-002` pinned four requirements `REQ-001` had moved to Met on a probe
  rather than a test**: a personal project's issue excluded from a team's
  sprint backlog, *mark all read* offered only while something is unread, an
  unscheduled calendar day carrying no comment or fill, and the team privacy
  footnote actually on the team screen (its wording was already pinned
  elsewhere). Each is now a test, each seen to fail before it existed.
- **0.40.0's four malformed requirement entries are repaired.** A
  pattern-based amendment matched further than intended and two entries were
  truncated, two spliced, reading as prose that says what neither the old
  nor the new version said. Named rather than fixed quietly, because it was
  this project's own mistake and it shipped. Not in any published crate — the
  specification is at the repository root, outside what `cargo publish`
  packages — so 0.40.0's crates are unaffected and its tag is not re-cut.
- **Release tags now link to their release notes.** An annotated tag's
  message carries `Release notes: …/blob/<tag>/CHANGELOG.md`, pinned to the
  tag; `0.40.0` was re-tagged once, on the same commit, to add it.
- **`CHANGELOG.md` holds the current series only.** Versions 0.1 to 0.39
  moved, verbatim, into `changelog/`, one file per series of ten minor
  versions; a scan under `cargo test` checks the arrangement, including —
  from this release — that a dated section opens with `### Highlights`
  (`DEC-055`).
- **The suite grew from 352 to 369.** No test file is new.

**What a reader should not conclude.** `§10.15`, `§10.17` and `§10.32` remain
open; `§10.32` is open **by decision**. **Undo is still not on the keyboard
route on the board, the sprint plan or the calendar** — their keyboard path is
a native form POST that shows no toast at all, so there is nothing there for
this release's fix to reach. That is recorded against `FR-DM-005`, a feature
gap about consistency across surfaces; it is **not** an `FR-DM-002` violation
— an earlier draft of this section said it was, and that was wrong. The
charts' colour-only series separation is unchanged. The product still does not
claim WCAG conformance.

## [0.40.0] — 2026-09-25

**No schema migration** — `0020_sprint_record_contributor_basis.sql` remains the
most recent, and a database created by 0.39.0 starts on this release with nothing
applied. **Almost nothing a user can see changed, and the release is still
substantial**: it is the one where the requirements record was checked against the
code, and where one running-system behaviour changed, quietly, underneath.
No public item was removed and no signature changed.

### Fixed

- **Three surfaces stop overflowing on long names.** A name that is one
  unbroken run — a token, a URL, a checksum — took these off the edge of the
  screen: **the sprint plan's backlog filter**, whose project and assignee
  drop-downs were as wide as their longest entry (729 px and 626 px inside a
  320 px phone); and **the assignee's name** wherever it appears — the board card,
  the workload strip and issue detail. **The assignee name overflowed a 1280 px
  desktop as well as phones**, on a team project's board, so this was not only a
  phone problem. **No screen, route, state or copy changed.** A long name now
  wraps inside its badge, over as many lines as it needs, rather than running past
  the edge; the filter drop-downs fit their row and show the start of the
  selected name.

### Changed

- **`PRAGMA optimize` runs when a database connection is closed.** This is the
  one change to a running system. SQLite plans every query from statistics about
  the data, and nothing here ever gathered any, so it planned from built-in guesses
  — and *which* guesses depends on the SQLite version linked in, so a dependency
  bump could change plans with no change to this repository. Now each install's
  own data decides. It runs when the pool closes a connection it has finished
  with — one idle for ten minutes, or older than thirty, found by a check that
  runs every ten, so on a quiet install the statistics arrive within about twenty
  minutes of the last use (measured on the built binary). **It never runs at
  startup and never inside a request**, and a process that is stopped does not
  run it.
  - **The reason is not the 7×.** On a 60,000-issue database it makes one health
    query about seven times faster and search 1.3 to 1.6 times as fast, and moves
    nothing else — not enough to justify the change. The reason is that install
    shapes cannot be predicted, and both alternatives (running `ANALYZE` in a
    migration, or leaving it alone) require predicting them: a migration takes its
    one snapshot when the database is nearly empty, the worst moment to sample.
  - **The near-miss.** The SQLite manual's own recommended sampling limit
    (`analysis_limit` 400) was measured on that database and produced statistics
    too coarse to change the plan it was meant to change; that health query was
    slower than with no statistics at all (16.4 ms against 8.2). Following the
    documentation would have shipped a regression. The limit is left to SQLite.
  - **What it costs.** A close on a database that already has statistics takes
    about 0.2 ms. The first close takes 3 to 4 ms; eight connections closing at
    the same instant, once, cost a concurrent write about 9 ms.
  - **One plan changes that you might notice in a query log**: on a used install
    the issue list reads through the whole-project index rather than the
    partial one. It is not slower, and it is recorded against `NFR-PERF-003`.

### Internal

- **The requirements record was checked against the code** (`REQ-001`), and this is
  the release's subject. Every requirement whose status is a claim was audited.
  **Sixteen were stale, all understating what ships** — three `Specified` entries
  picked at random before the audit was commissioned were all in the product.
  **One would have caused work**: `FR-HLT-006` recorded that no automated
  vocabulary guard exists, and one does, which invites someone to build it twice.
  **Two entries contradicted each other** about whether the health score badge
  still exists (it was retired at 0.20.0). And **six requirements read two ways**,
  where the status had silently taken one.
  - **Why that was worth a release slot:** a requirement nobody believes is live is
    a requirement nobody checks, and the previous release lost two handoffs to a
    requirement sentence that had not been checked against how the product
    behaves — one reached `main` and half of it was reverted, one was written and
    withdrawn unbuilt.
- **`FR-CAL-007`, a P0, reaches Met.** The calendar must not show occupancy, a
  week-over-week comparison or free hours. Its behaviour always held; the guard
  its status said existed checked a different property (no percentage, no ratio),
  so those concepts could have appeared in words carrying no quantity and the guard
  would have stayed green. Three tests now assert the named concepts on the served
  pages of both calendars and on the message table, each term seen to fail when
  planted. They cover the named terms and their spellings, not synonyms.
- **The overflow gate grew from 95 to 120 cells**, from nineteen pages to
  twenty-four. Its fixture had three
  times not rendered the branch a defect was sitting behind: a completed sprint
  with no issues, a team with no projects, issues with no assignee. It now holds a
  completed sprint with issues and two contributors, an assigned issue planned for
  today, and the day and month calendar layouts on both axes, and **the fixture
  fails the run if a branch it exists to render is not on the page.** `§10.32`
  records this, with a stopping rule that was tested at once: `GATE-004`, which
  would have swept the inbox with notifications in it, was **withdrawn under
  it** — no route can produce one, and the two kinds that exist carry fixed
  text.
- **The suite grew from 346 to 352.** No test file is new, so `DEC-007`'s command
  block and `.github/workflows/test.yml` are unchanged.
- **Both specifications are amended to 0.40.0.**

**What a reader should not conclude.** `§10.15` and `§10.17` remain open, and
**`§10.32` is open by decision**: its stopping rule is deliberate, not a gap
nobody reached. **A green gate cell still means one thing — no horizontal
overflow.** The calendar's day view is now *visited* and is not *checked* for the
vertical collapse `CAL-003` once found; visiting is not checking. **Four
requirements now recorded Met have no test pinning the specific limb they
name** — the exclusion of personal projects from the plan backlog, the hidden
*mark all read* control, an unscheduled day carrying no comment, and the team
page's footnote on the screen — and the record says so. The product still does not
claim WCAG conformance.

## Earlier series

Older release notes are archived, one file per series of ten minor versions:

- [0.30 to 0.39](changelog/0.30-0.39.md)
- [0.20 to 0.29](changelog/0.20-0.29.md)
- [0.10 to 0.19](changelog/0.10-0.19.md)
- [0.1 to 0.9](changelog/0.1-0.9.md)
