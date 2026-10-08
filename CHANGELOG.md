# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.44.0] — 2026-10-08

**This release ships no behaviour, and that is the honest summary rather
than one dressed up.** It is a rule, a lockfile bump, nine tests and a
correction to the project's own record. No file under `crates/*/src/` or
`static/` differs from 0.43.0.

### Changed

- **`FR-DM-002` (a `P0`) gains a clause, so the calendar fails a written
  test instead of an opinion.** The requirement now reads: a keyboard
  equivalent MUST be offered **by the element that carries the pointer
  affordance** — on it, or one activation away — and MUST **perform the
  same action, not require the user to reconstruct its effect**. Measured
  against it: the board, the issue list/detail, and the sprint plan pass.
  The calendar fails both halves — reaching a reschedule costs **77 Tab
  and Enter keystrokes across three pages**, not one activation on the day
  view itself, and the issue edit form's two independent `datetime-local`
  fields make a keyboard user compute the new end time by hand, where a
  drag shifts both ends by the same delta. **The surfaces are not
  inaccessible** — the effect is reachable on every one of them, and both
  write paths (the form, the drag's own JSON endpoint) carry the identical
  optimistic lock. What is wrong is the cost and the shape, not the
  presence, of the calendar's keyboard path. **The remedy is scheduled,
  not pending**: RFC 0015 Option A — a purpose-built move control one
  activation from the block, the server computing both timestamps so
  duration survives a keyboard move — at **0.45.0**.

### Fixed

- **`FR-DM-006` (the undo window) reaches Met on all four
  direct-manipulation surfaces — reversing a correction this document made
  one release earlier.** 0.43.0 corrected it to `Partial — three of four`,
  because the status's own words named three places, a fourth surface was
  known to ship, and nobody opened `calendar.js` to check the fourth before
  writing the correction. It does have the identical real button, the
  identical 5-second window, and the identical keyboard reachability the
  other three have. Recorded plainly, including why it happened: **the
  procedure now reads amend from the code, never from the entry's own
  words** — this is the sweep's own first finding, corrected by its second
  pass.
- **Two yanked dependencies, moved past.** `spin 0.9.8` (shipped, pulled in
  via `sqlx-sqlite`) and `chacha20 0.10.0` (test-only, via `axum-test`)
  were both withdrawn upstream. Both moved to the next non-yanked version
  within the same minor (`0.9.9`, `0.10.2`) — a lockfile-only change, four
  lines. **Neither is known to be vulnerable**: *yanked* means the
  publisher withdrew the version, which can be a security withdrawal or a
  mistaken publish, and which one it was is not established here. Found by
  the release procedure's own `cargo publish --dry-run` — the system
  working as intended, not a defect that shipped.

### Internal

- **Nine new tests, pinning the measurement that found the above.** The
  one worth a sentence on its own:
  `plan_add_then_remove_leave_issues_updated_at_untouched` — a plan move
  that began writing `issues.updated_at` would silently invalidate every
  open browser tab's optimistic-lock stamp for that issue, on every drag
  or button move, with no error anywhere to notice it by. The other eight
  pin sprint-plan field parity, calendar lock parity between the issue
  edit form and the drag's JSON endpoint, and the undo toast's attachment
  to its acted-on element across all four scripts.
- **One of the nine is a source scan, not a test of running code**
  (`undo_toast_attachment_scan`, four of the nine): it pins what
  `static/*.js` is *written* to do, not what a running page does. Renamed
  from its first name (`undo_dom_order`) before this document cited it,
  because the original name claimed a runtime fact the mechanism — a
  substring check over source text — cannot reach.
- **One check added is not a Rust test at all.** `undo-mousedown-trap.mjs`
  is a browser gate, wired into the existing `browser-overflow-gate` CI
  job as a second step rather than a new job (no second build, no second
  Chromium install), and it is outside the 376 figure below. It executes
  `board.js`, `plan.js` and `calendar.js` in a real headless browser and
  asserts a real mousedown-plus-movement on Undo still fires a click
  rather than starting a drag — the exact regression `A11Y-005` found and
  fixed, now held in place by something that runs instead of only being
  read.
- **`§10.15`'s title changes after eighteen releases** — from *no test*
  to *almost no test*: one gate now executes the shipped scripts for one
  property, on three of the five files and one property out of many.
  **`RFC 011`'s refusal to buy a JavaScript test harness is unchanged** —
  this is one gate on one real regression class, not a reversal of that
  decision, and a reader should not conclude otherwise from the residue
  count (1,909 lines in five files) shrinking in significance.
- **Test inventory: 376, up from 367.** Three consecutive runs, extracted
  fresh from `.github/CONTRIBUTING.md`'s own command block each time —
  the block changed twice this release (`DM-TEST-001`, `DM-TEST-002`), and
  a stale saved copy undercounts by exactly the tests the most recent
  change added, which is how this release's own review first measured 372
  before catching it.
- **No schema migration** — `0020` remains the most recent, the fifth
  release running. The overflow gate stayed at 120 cells, unexercised:
  nothing here renders a page or changes a component's markup.

**What a reader should not conclude.** `FR-DM-002` reading `Partial` with
a named clause is not a newer or worse defect than last release's
unqualified `Partial` — it is the same gap, now checkable by inspection
instead of resting on judgement, with its remedy dated. The 376 figure
mixes a source scan (four tests) with assertions against running code
(five tests); neither the published count nor `undo_toast_attachment_scan`'s
own green result should be read as end-to-end runtime coverage of the
undo gesture — only the browser gate, for the one property it checks, is
that.

## [0.43.0] — 2026-10-08

This release has two halves, and they pull in opposite directions. One closes
a long privacy thread: identity moving from a loose convention to something
the compiler now enforces. The other reports that the project's own record of
itself was wrong about a `P0`.

### Changed

- **Personal-data storage now refuses anything but a cryptographically-verified
  identity, at compile time.** `NFR-PRIV-005` reaches **Met**: across all 36
  storage functions that can take an identity, `user_id: &str` now returns
  **zero results** in all six personal-data modules — it was 11 of 36 still
  on `&str` after the first pass (`PRIV-002`), and a second sealed type
  (`SubjectId`, `PRIV-003`) closed the gap for the one caller with no
  requester: a background job that iterates users on a schedule rather than
  answering a request. **The boundary this protects already held** — two
  independent measurements, a release apart, found the same 36 functions
  already scoping correctly on identity. What changes is that the boundary
  can no longer be broken by inattention, because the parameter type itself
  refuses anything that was not minted from a verified JWT or derived from
  one — not that a hole closed. The four functions that remain on `&str`
  cannot take a sealed identity and are not a shortfall: three are the
  authentication path, where no caller identity exists yet because
  establishing one is what the call does, and one is the same background
  job's own user enumeration.
- **A new requirement, `NFR-REL-008` (published API documentation)**: the
  seven published crates' API documentation MUST build with no unresolved
  intra-doc links, enforced by an automated gate. It exists because this
  project publishes one artefact it does not author — `docs.rs` builds these
  crates on every release — and nothing was checking it (see Fixed, below).

### Fixed

- **Ten broken links in the published API documentation** (`DOCS-002`).
  `docs.rs` builds these seven crates, so each unresolved link was live in
  the API documentation of every released version since it was introduced.
  The honest framing is that **nothing had ever looked**: `clippy
  --all-targets` does not check doc links and `cargo doc` was in no gate.
  Seven were path corrections to sentences already true about the code —
  two functions renamed without the module documentation describing them,
  a type used fully-qualified in code and linked bare, a function described
  from a module it does not live in. Three named a shape the code no longer
  has — a constant since split into two, a method an enum never carried,
  and a function whose work had been folded into a sibling's upsert — and
  are corrected to say what the code does rather than retargeted to clear
  the warning. A new CI job, `rustdoc-links`, denies unresolved links on
  every push; its local command is documented in `.github/CONTRIBUTING.md`
  beside Formatting and Linting, so the checklist and CI cannot drift apart.
  (The initial count of nine, and a separate count of 23 "issues," were both
  short — the counting run aborted before reaching `peisear-web`, and the
  23 included three summary lines rather than individual issues. The real
  count was ten links and eight further warnings, left as deliberate
  cross-references to implementation detail.)

### Internal

- **A `P0` requirement, `FR-DM-002` (keyboard parity), was recorded complete
  over half of what it governs.** Its status read "Implemented for both
  shipped surfaces" while **four** direct-manipulation surfaces ship: the
  sprint-planning drag (0.35.0) and the calendar block drag (0.36.0) have no
  keyboard test, and `calendar.rs` has no form, button or select at all.
  `FR-DM-006` (the undo window) was in the same position. **Both are now
  recorded as `Partial`.** This is not a newly-discovered accessibility gap
  — the surfaces may well be fine. What is established is that nobody
  checked, and the status said otherwise for five releases.
- **The sweep that found it, and `REQ-003`'s answer to the question behind
  it.** Four candidate text rules for catching a stale requirement status
  were tested against six known historical instances, measuring false
  positives over all 162 register entries; none reached the bar. The best
  scored 4 of 6 and was argued down by the dev team that built it, because
  its zero false positives were measured on zero opportunities — no live
  entry had both a stale-shaped status and the sibling citation the rule
  looked for, so it had never had the chance to be wrong. The remedy is
  procedural: a four-pass review, by hand, of both specifications at every
  release candidate, now recorded in
  `docs/development/changelog-and-releases.md`. Its first run, this release,
  found six things — three stale entries (`FR-DM-002`, `FR-DM-006`,
  `NFR-PRIV-005`) and three stale summary rows in the requirements
  appendix.
- **Test inventory: 367, two fewer than 0.42.0's 369, and that drop is
  deliberate.** `PRIV-002` deleted `untrusted_id_scan.rs` and its two
  assertions (`path_extracted_user_id_only_reaches_require_self`,
  `every_path_extraction_binds_the_name_user_id`) — a 212-line text-pattern
  guard approximating a property the type system now holds everywhere,
  rather than only where the scan happened to look.
- **A requirement identifier was used twice.** `NFR-A11Y-009` was added at
  0.41.0 for *Where a navigation leaves the reader* while already belonging
  to *Keyboard shortcuts*, a P3 item dating to the 0.19.1 baseline. The newer
  entry is renumbered **`NFR-A11Y-011`**; the older keeps the number, because
  it is cited in two superseded baselines that are retained unedited as the
  record of their own releases. **The 0.41.0 and 0.42.0 sections below still
  say `NFR-A11Y-009`** and are left as written: a release section records what
  was said at the time, and the published release notes carry the same text.
  Found by `REQ-003` while parsing the document for something else.
- **No schema migration** — `0020` remains the most recent, the fourth
  release running. The overflow gate stayed at 120 cells, untouched by every
  change in this release and not exercised by any of them.

**What a reader should not conclude.** `FR-DM-002` and `FR-DM-006` reading
`Partial` is not a report that the sprint-planning or calendar drag is
inaccessible — it is a report that nobody has checked, which the architect's
own recommendation is to ship rather than hold the release for, since the
condition has held unnoticed since 0.36.0 and a release correcting the record
is the right place for the correction to appear. `§10.17`, `§10.30` and
`§10.34` remain open; `§10.34` is open **by decision** — it closes after two
consecutive releases sweep clean, not on this one's result. The eight
remaining public-doc-comment references to private implementation detail
(`DOCS-002`, §C) are left as-is on purpose, not as a residual defect.

## [0.42.0] — 2026-10-07

### Highlights

- **Eleven Tab presses to reach the content after any ordinary form
  submission are now one press and an activation.** A skip link, first in
  the page, hidden until focused — chosen over the requirement's other two
  remedies because it is the only one that works with scripting off.
- **Two charts were only readable to someone with full contrast
  sensitivity.** Not a colour-blindness gap — both charts are single-hue and
  always were — but two pale blues that were, to a low-contrast-sensitivity
  reader, one pale blue. Both now separate at 3 : 1 or better.
- **A focus ring in the account menu was drawn in transparent.** Four
  links — Today, Teams, Inbox, Settings — now draw the same ring every
  other link on the page already does.
- No schema migration; no public item removed; no signature changed.

### Fixed

- **A skip link closes the reach gap `NFR-A11Y-009` named**
  (`components/layout.rs`). `href="#main"` lands on a new
  `id="main" tabindex="-1"` on each page shell's own `<main>` — the
  `tabindex` is load-bearing, since a bare fragment jump moves scroll but
  not focus. Visually hidden until focused (`sr-only`/`focus:not-sr-only`,
  never `display:none`, so it stays reachable with scripting off), and
  measured rather than assumed at every step: Tab presses from a fresh
  load, eleven (ten on a phone board) before, one press and an activation
  after, on every page; `document.activeElement` lands on `main` itself,
  with scripting on or off; the five pages with an `autofocus` form input
  are unaffected. The hidden element's own box turned out to be 44×44, not
  1×1 — `min-h-11`/`min-w-11` beat `sr-only`'s own `1px`/`1px` — and a
  second measurement found it contributes nothing to any page's scroll
  width and is not clickable: `clip: rect(0,0,0,0)` removes it from
  hit-testing as well as from painting, checked with a real click at six
  points inside the box, on two separate builds.
- **The burndown's two lines and the completed-work bars were 1.77 : 1 and
  2.09 : 1 apart**, with one bar at 1.86 : 1 against its own background —
  all short of the 3 : 1 a graphical object needs. Both charts now share
  one pair of values, still single-hue, reaching 3.40 : 1 between series
  and 4.22 : 1 / 14.36 : 1 against the page. Every figure measured by two
  independent colour-space conversions, agreeing to the rendered pixel
  value.
- **The account menu's four links drew no visible focus ring** — a
  vendored selector set it to transparent, overriding the ring every other
  link on the page draws. Measured first, since the grey row left behind
  might already have been enough: it reached 1.22 : 1 against its
  neighbours, well short of 3 : 1, so the four links now reuse the exact
  ring appearance the other ten stops already draw.
- **The sprint plan page had two `<main>` landmarks**, one of them a plain
  layout grid with no landmark intent of its own — invalid regardless, and
  made load-bearing by the skip link now landing on the first one it
  finds. Changed to a `<div>`; confirmed the only other site in the tree
  using the tag this way.

### Changed

- **A new requirement, `NFR-A11Y-010` (non-text contrast)**: WCAG 1.4.11's
  3 : 1 for graphical objects and interactive-control boundaries, which
  this project had only for text until now. It exists because last
  release's rewording of `NFR-A11Y-004` turned out not to cover the finding
  it had been carrying as a status — a different reader, a different
  failure, needed its own requirement rather than a borrowed one.

### Internal

- **Two corrections to this document's own editing, recorded rather than
  quietly applied.** `NFR-A11Y-002`'s status contradicted its own body.
  `NFR-A11Y-004`'s was written against a finding rather than against the
  sentence it sat under. The third and fourth instances of one habit —
  amending an entry by adding a correction and leaving the `*Status*` field
  as it was.
- **Test inventory unchanged at 369.** Four handoffs, and this release's
  own work was measurement, not assertion: six hit-test points, two
  independent colour conversions, fourteen computed focus styles. One test
  was rescoped rather than added — a keyboard-reachability assertion that
  had gone stale the moment the skip link gave the page a second,
  legitimate reason to carry `tabindex="-1"`. A test pinning the charts'
  exact contrast ratios was considered and not written, since proving it
  would mean a third implementation of the same colour-space conversion in
  the suite.
- **`static/tailwind.css` was regenerated twice**, additive both times —
  zero rules removed, same Tailwind version. `style/tailwindcss/README.md`
  now records why a handful of the added rules correspond to no class
  anyone wrote: Tailwind extracts candidates from any word in scanned
  content, including prose in code comments.
- **The overflow gate stayed at 120 cells**, run for three of the four
  handoffs; the fourth changed only colour literals, nothing the gate
  measures, and said so rather than skipping silently.

**What a reader should not conclude.** `§10.15`, `§10.17` and `§10.32`
remain open; `§10.32` is open **by decision**. Nothing graphical other than
the two charts has been audited for contrast — `NFR-A11Y-010` says so in
its own status, and this is not a sweep. Undo is still not on the keyboard
route on the board, the sprint plan or the calendar (`FR-DM-005`), and the
scroll position a phone loses after a form submission is still lost — this
release's skip link reaches the content; it does not restore where the
reader was. Chrome only; no screen reader has been used. The product still
does not claim WCAG conformance, and this release is the one where that
sentence needs the most care, since two of its criteria were just cited by
number: citing a criterion is not conforming to it.

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
