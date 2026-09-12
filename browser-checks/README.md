# Browser checks — a gate, not a test

`BROWSER-001` (RFC 011 step 4, `DEC-048`). This directory holds the
horizontal-overflow gate: one assertion, `scrollWidth <= clientWidth`, on
eighteen rendered pages at five widths, driven by a minimal Chrome DevTools
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
  creates fixtures through the real forms, sweeps eighteen pages × five
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

Eighteen pages: `/today`, `/inbox`, `/today/calendar`, `/projects`, a
project detail page (the board — it is the default view), the same
project's list view, a project calendar, a team detail page, a sprints
list, a sprint detail page, a sprint plan page, an issue detail page,
the new-issue form, `/settings`, `/settings/notifications`, `/teams`,
`/search`, and a delete confirmation interstitial.

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
