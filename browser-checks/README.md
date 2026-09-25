# Browser checks — a gate, not a test

`BROWSER-001` (RFC 011 step 4, `DEC-048`). This directory holds the
horizontal-overflow gate: one assertion, `scrollWidth <= clientWidth`, on
twenty-four rendered pages at five widths, driven by a minimal Chrome DevTools
Protocol harness with no npm dependency.

**This does not run inside `cargo test --workspace` and is not counted in
`DEC-007`'s own inventory.** It runs as its own CI job, on the same footing
`fmt` and `clippy` already have — a check that gates a merge without being
a Rust test. See `rfcs/handoffs/011-browser-verification/BROWSER-001-overflow-gate.md`
for why that separation is load-bearing (§2): a Rust integration test here
would put Chromium on every contributor's `cargo test --workspace`,
including `DEC-007`'s own three-consecutive-run gate before every release.

## What it checks, and deliberately does not

**One assertion only**: `scrollWidth <= clientWidth`, the check that found
`LAYOUT-001` and `LAYOUT-002` — two defects that had shipped for many
releases and that nothing else this project owns could observe.

**Not target size** (`TT-004`'s own guard covers the class-carrying
subset from source; a rendered-size gate is step 4's second-ranked
candidate, not this one). **Not overlap** (`§10.19`/`§10.21` are real, but
their measurement artefacts — a closed `<details>`'s stale
`getBoundingClientRect`, a dropdown's close-animation `scale()` — aren't
understood well enough yet to gate on). **Not JavaScript** (`DEC-048`
condition 2 forbids wall-clock-dependent checks, and `JS-001` measured the
untested surface as thin anyway).

**Do not widen this gate.** Adding a second assertion before the first has
run stable in CI for a while is how `DEC-048` condition 3 starts costing
attention.

## Files

- `cdp.mjs` — the harness. Drives one headless browser tab over CDP using
  Node's built-in `WebSocket` and `fetch`; no bundled browser, no npm
  dependency. See its own doc comment for the browser-binary resolution
  order and its three known limits (one page at a time, waits on
  `document.readyState` only, no notion of animation completion).
- `overflow-gate.mjs` — the gate itself: starts a scratch instance,
  creates fixtures through the real forms, sweeps twenty-four pages × five
  widths, and exits non-zero if any cell overflows or if any request
  reached a host other than `127.0.0.1`.

## Running it locally

```bash
cargo build -p peisear
node browser-checks/overflow-gate.mjs
```

Needs a Chrome-family browser on `$PATH` (`google-chrome-stable`,
`google-chrome`, `chromium`, or `chromium-browser`) or `CDP_BROWSER_BIN`
pointing at one. `PEISEAR_BIN` overrides the binary path (default
`target/debug/peisear`); `PEISEAR_PORT` overrides the scratch port
(default `4173`).

## `DEC-048`, condition by condition

1. **No external network.** `overflow-gate.mjs` calls
   `blockNonLocalRequests()` before the sweep starts and asserts the
   blocked list is empty at the end — not just "nothing broke," but "here
   is the request list, and it's empty." `ASSET-001` (0.32.0) is what made
   this checkable at all: until then every page fetched CSS from two CDNs,
   and a gate that can fail because a CDN is slow is exactly what this
   condition forbids.
2. **No wall-clock dependence.** `goto()` waits on `document.readyState`;
   `waitForSelector()` polls for a specific element rather than sleeping a
   fixed duration. If a page ever needs an actual settle delay to measure
   stably, that is a finding about the page, to be named and fixed, not a
   licence to add a `setTimeout`.
3. **A flake is a defect with an owner — decided now, not after.** If this
   gate produces an unstable result (fails intermittently on an unchanged
   tree), the response is: open an issue naming the specific page/width
   cell and the failure mode, mark the check `continue-on-error: true` in
   the workflow with a comment linking that issue, and the issue's owner
   is whoever is working `RFC 011` step 4 at the time — currently the
   architect. **No silent retry**: no re-run step, no matrix retry, no
   `continue-on-error` added without an issue link beside it. `§10.13`'s
   own defect survived four releases because re-running was the response
   each time.
4. **A quarantined check is not coverage.** If this gate is ever
   quarantined per condition 3, the next release candidate names it as
   quarantined rather than reporting a clean run.

## Fixtures

`overflow-gate.mjs` registers a user, creates a team, a sprint, a project
and two issues — all through the real HTTP forms the application exposes, the
same way the investigations that found `LAYOUT-001` and `§10.21` did.

Five choices are deliberate, not incidental:

- **The signed-in email is long and hyphen-free**
  (`browseroverflowgatefixtureaccount@example.org`). `LAYOUT-001` was
  conditional on exactly that shape — a short-email fixture would not
  have caught it, and neither would one with a hyphen the browser could
  wrap at.
- **One issue's title is long.** `§10.21` (a 314px anchor/row overlap) was
  invisible on the architect's own ten-page sweep because the fixtures
  were empty — a layout gate with empty fixtures measures a layout nobody
  sees.
- **The issue title, the project name and the team name each carry a
  64-character unbroken run** (`BROWSER-002`). Long is not the same as
  unbreakable: for a full release this gate swept issue detail at 390 and
  414 while that page overflowed by hundreds of pixels, and reported
  48/48 clean — because every fixture string had a space at every point,
  so no container ever received text it could not break. Four of the five
  layout defects this project has found were conditional on unbreakable
  text. The run is a SHA-256 (the empty string's), which is both
  guaranteed break-free and the kind of thing that really does end up in
  an issue title.
- **The signed-in user's display name carries one too** (`LAYOUT-007`),
  77 characters against the register form's own 80-char ceiling. This
  was the last fixture field whose value had a space at every point, and
  it hid three more overflows on pages this gate was already sweeping
  green: `/today`'s subtitle, `/settings`' subtitle, and the list view's
  assignee filter, whose `<select>` takes its width from its widest
  `<option>` — display names. The spaced words lead so the navbar still
  shows something recognisable once `LAYOUT-006`'s truncation takes the
  rest.
- **A sprint whose name and goal both carry the run** (`LAYOUT-008`).
  Three pages render one or both — the sprints list, sprint detail and
  the plan page — and none of the three was in the page list until that
  handoff added them. Sprint detail is also the only site found so far
  that overflows a **1280 px desktop**, so a phone-only fixture would
  not have been enough either.

**The pattern across `BROWSER-002`, `LAYOUT-007` and `LAYOUT-008` is the
thing to remember**: every time a fixture field gained an unbroken run, this gate
found defects on pages it was already visiting. What the gate sees is
exactly its fixture and its page list.

## Coverage

Twenty-four pages: `/today`, `/inbox`, `/today/calendar`, `/projects`, a
project detail page (the board — it is the default view), the same
project's list view, a project calendar, **the day and month calendar layouts on both axes**, a team detail page, a sprints
list, a sprint detail page, **the same page for a completed sprint**, a
sprint plan page, an issue detail page, the new-issue form, `/settings`,
`/settings/notifications`, `/teams`, `/search`, and a delete confirmation
interstitial. (24 × 5 widths = 120 cells; it was 20 × 5 = 100 until `GATE-003`, 19 × 5 = 95 until `GATE-002`,
and 18 × 5 = 90 until `SPRINT-005`.)

`SPRINT-005` added the completed-sprint form of the sprint detail page
(`§10.27`): a sprint started and completed through the real routes, so it
carries a captured record. The page carries the **Reopen sprint** control and
`SPRINT-004`'s two headings, which no page in the list reached while the
fixture's only sprint was planned — the gate reported 90/90 about a
different page.

`GATE-001` gave that sprint members. The completed sprint's issue list is
user text in a list — where `§10.25`'s three shapes were all found — and the
sprint had none, because the fixture's project is personal and only a team
project's issues can join a sprint. So the fixture now also holds **a team
project** (its name carries the unbroken run) and **a second account**, added
to the team through the members route. The personal project stays as it was.
Three issues are created in the team project through the form, added to the
sprint through the plan route while it is planned, and marked done through the
status route before the sprint is completed: one whose title carries the
unbreakable run (`LONG_ISSUE_TITLE`), one long title made only of ordinary
words (which must *wrap*), and an ordinary one. The two assignees are the
**two contributors** `NFR-PRIV-007` requires before the burndown renders — a
sprint with one would leave the chart out of the sweep. `createFixtures`
fetches the completed sprint's page and **fails the run if the three titles,
the two headings or the burndown are absent**, so the fixture cannot decay
into sweeping an empty list again. No page was added: the same 19 × 5 = 95
cells, one of them now carrying a populated list.

A team project is visible on more than one page. `team_detail` and
`projects` list it, `search` (`q=fixture`) now matches the long issue title
in it as well, and the sprint plan page's filter bar lists it and the second
account — the filter's `<select>`s size to their longest option, so a project
name or display name with an unbroken run widened the page until `LAYOUT-010`
fixed it — the fixture's team had no project before, so both lists were empty
and the gate could not have seen it.

`LAYOUT-011` added an **assigned** issue to the personal project — a new one, in
progress, assigned to the project's owner (the account whose display name carries
the unbroken run); the two issues that were there are unchanged. Every fixture
issue was unassigned until then, so no assignee badge and no workload chip had
ever rendered under the gate, and both overflowed a 320 px phone while it
reported green (the team project's board also overflowed 768 and 1280). The
board and list now render a card/row, a badge and the workload strip with the
long name; the cell count is unchanged (95). **Not covered by that issue:**
issue detail — the gate's issue-detail page is the older, unassigned issue, so
its assignee badge is exercised only by the unit test in
`tests/assignee_candidates.rs` and by hand, not by this gate.

`GATE-002` (`§10.32`) made the fixture **exercise branches its pages have**, and
assert that it did. The page list gained **`issue_detail_assigned`** (the assigned
issue's detail page, so its assignee badge is held by the gate; the older issue's
page is unchanged) — **95 → 100 cells**. The personal project gained one more
issue, **assigned, in progress and planned for today**, whose title carries the
run: it is the board's second assigned card, and it puts a block on **both**
calendars (the personal one lists assigned issues only). It is planned through
the *edit* route — the create form does not take a planned window — with the
lock value the edit page renders. `createFixtures` then **fails the run** unless
it finds: an assignee `badge` holding the display name on issue detail; two such
badges and a workload-strip chip on the board; and the planned issue's block on
`/today/calendar` and on the project calendar. **A green cell on a page that
rendered nothing is the failure this is for**; the assertions are what tell the
two apart.

`GATE-003` added the **day and month** calendar layouts on both axes — four
more URLs, **100 → 120 cells** — each anchored explicitly on the day the fixture's
planned issue is planned for (`?date=`, the same UTC date, so it is on the page in
any month) and each **asserted to contain that issue's block**. The day view is
where `CAL-003`'s `h-full` once collapsed a block; note that this gate still
measures **horizontal overflow only**, so it would not see that collapse — the
layout is now *visited*, not fully *checked*.

**This is not exhaustive, and is not meant to be** (`§10.32`). It closes the
branches that were known, not the ones that are possible. Known and *not*
covered: a **second assigned account** cannot appear on the personal board (a
personal project's only assignee candidate is its owner), so the board shows one
name twice, never two; the team project's board, list, calendar and issue detail
are not gate pages; the **inbox renders its empty state** (the fixture produces no
notifications), so notification rows — which carry issue and project text — are
never swept.

`LAYOUT-008` added the three sprint pages and the new-issue form. All
four were red on the run that added them.

`LAYOUT-005` added the project calendar and team detail. Both carried an
overflow this gate could not see, for the same reason `BROWSER-002`'s
fixture could not see the other five: **a page absent from this list is
not covered by the fixture, however good the fixture is.** It also
renamed the `board` key, which pointed at `?view=list` — the board is
`project_detail`.

Five widths: 320, 390, 768, 1280, 1920.

`BROWSER-002` added 320. It was held back one round because it turned
every page red by 24px — `LAYOUT-006`'s navbar defect, not a per-page
one, and `DEC-048` condition 3 forbids landing a red gate without an
issue link beside it. What the width buys is narrower than its original
case: with an unbreakable run in the fixture, the board defect it was
meant to catch (`LAYOUT-003`) already fails at 390. Its value is the
defect that manifests **only** at the narrowest phone, which is what it
caught on its first run.

**Every failing cell is reported, not just the first** — a gate that stops
at the first failure hides the shape of the problem.
