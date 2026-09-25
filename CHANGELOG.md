# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Internal

- **Release tags now link to their release notes.** An annotated tag's message
  carries `Release notes: …/blob/<tag>/CHANGELOG.md`, pinned to the tag; `0.40.0`
  was re-tagged once, on the same commit, to add it. The workflow is written in
  `docs/development/changelog-and-releases.md`.
- **`CHANGELOG.md` holds the current series only.** Versions 0.1 to 0.39 moved,
  verbatim, into `changelog/`, one file per series of ten minor versions. Their
  stale reference-style link footers, which pointed at GitHub Release pages that
  never existed, were dropped. A scan under `cargo test` checks the arrangement.
  From 0.41.0 a release section opens with `### Highlights`.

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
