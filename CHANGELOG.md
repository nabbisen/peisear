# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

**The guard `0.60.0` shipped without, and the page `install.md` said
would clone.** Two rounds of `054` landed after that cut rather than
before it, so they wait here for whichever release is cut next. Neither
changes behaviour; both are about a reader following written
instructions and reaching a working instance.

### Added

- **A check now enforces what the README's Quick Start may contain**,
  where `0.60.0` left it to a reader. `docs_scan` rule 9: every command
  in the Quick Start must appear verbatim inside a ` ```bash ` fence in
  [Install](docs/getting-started/install.md) — derived from that page
  rather than from a hardcoded list of paths, so adding an install path
  cannot drift the two documents apart a fourth time. It carries an
  asserted population count, and its anchor reports **its own lookup
  failing by name** rather than examining nothing and passing.

- **Two platforms are now compiled before a tag, where one was.** A
  `windows-latest` job runs `cargo clippy -p peisear -- -D warnings`, so
  the kind of defect that cost `0.58.0` a published Windows archive — a
  helper live on Unix and dead on Windows — fails on a push instead of at
  the moment the archive is built. Proven by planting that exact defect:
  of the forty-eight jobs in the run, **the new one was the only one that
  failed**. Three of the five published platforms are still first
  compiled at the tag.
- **The walk that checks the documented copy-and-run path now runs by
  itself.** It had asserted correctly since `0.53.0` and fired three
  times in its life, the last eighteen days and four releases before
  this, because it was manual-only. It now runs on pushes to `main` and
  on pull requests, scoped to the six paths that can break it — which
  includes pull requests from forks, the flow `CONTRIBUTING.md` actually
  instructs.
- **A class a component writes now has to have a rule the product
  serves, or name itself as a JavaScript hook.** `CSS-001` fixed the
  stylesheet by hand; nothing checked the relationship afterward,
  because `cargo build` runs nothing in `style/tailwindcss/` by design
  (`DEC-051` — a self-hoster needs no Node), so the one thing between a
  missing rule and silence was a README sentence that was true, correct
  and useless for three weeks. `css_scan` now reads every `class="…"`,
  `grow("…")` and `grow_text("…")` literal in
  `crates/peisear-web/src/components/**/*.rs` and checks each token
  against the three stylesheets the binary actually serves: 291 class
  tokens examined, 2 recognised as `static/*.js` selector hooks rather
  than style, 0 unexplained.

### Fixed

- **The navigation bar actually draws.** The three entries below changed
  the markup correctly and changed nothing a reader could see: the
  stylesheet the application serves is built ahead of time and committed,
  it had not been rebuilt since 7 October, and the three classes the new
  navigation needs had no rule in it. The primary row was therefore
  hidden at *every* window width — a 1920 px desktop showed one button
  marked **Menu** and nothing else, with no search box and no unread
  badge anywhere. The stylesheet is rebuilt, and the two places that
  document how to rebuild it now name the file the application really
  serves; they had pointed at a path that stopped existing in `0.53.0`,
  so following them exactly changed nothing. **Never released** — this
  was only ever true of unreleased work.

- **The five primary destinations are visible and named.** `Today`,
  `Inbox`, `Projects`, `Teams` and `Search` were specified as the
  navigation's five entry points, and four of them sat behind a menu
  labelled with the reader's own username while `Projects` was reachable
  only by clicking the logo. They are now a labelled row, collapsing on
  narrow screens behind a control labelled **Menu** rather than a name.
  `Settings`, which was never one of the five, moved to the account menu.
- **The notification control says what it is.** It was an icon with no
  word; it now carries the word *Inbox*, and its accessible name matches
  what is on screen — it had still said *"Notifications"*, the name the
  page carried before it was renamed, so anyone using voice control asked
  for one thing and the control answered to another.
- **The unread badge is correct on every page.** It was computed on
  fourteen of twenty-six authenticated page renders and silently assumed
  to be zero on the other twelve — including `/today`, the first screen
  after signing in, and the projects, issue, search and settings pages. A
  reader with unread notifications saw nothing there. The count is now
  required rather than defaulted, so a page cannot render without it.
- **An undo can no longer disappear while a reader is still on it.**
  Every undo toast (moving an issue on the board, the sprint plan, or a
  calendar; changing an issue's status) removed itself five seconds
  after it appeared, with no pause for a pointer resting on it or
  keyboard focus inside it — `WCAG 2.2.1 Timing Adjustable`. The timer
  now pauses while a reader is hovering or focused on the toast and
  resumes when they leave; nothing else about the toast changes.
- **The Inbox answers to its own name, and every page title says what
  application it belongs to.** Clicking *Inbox* in the navigation
  landed on a page titled and headed *Notifications* — the name the
  destination carried before `NAV-001` renamed the control itself; the
  items in the inbox are still called notifications, only the
  destination's own title and heading changed. Every page title now
  also ends with *"— peisear"*, so a tab, a bookmark, or a history
  entry for *Calendar* or *My dashboard* says which product it's from;
  five titles that already carried the suffix by hand are unchanged in
  what a reader sees. `Today` → *My dashboard* is not renamed in this
  round; it waits on a content decision for `/today` itself.

- **[Build from source](docs/getting-started/build-from-source.md) now
  teaches the step its title promises.** `install.md` told the reader
  that page *"clones the repository and builds from a checkout"* — and
  it never cloned anything: it installed `rustc` two ways, verified the
  version, and its `## Next` jumped to Configuration, skipping the step
  in between. One published document promising what another did not
  contain.
- **`.github/CONTRIBUTING.md`'s development setup no longer promises a
  clone it does not show.** Step 2 read *"Clone the workspace and run
  the test suite"* over commands that began at `cargo check`. Step 1
  already links the page that now teaches the checkout, so the claim is
  removed rather than duplicated.

- **Two guards that cannot pass by examining nothing.** A clippy
  invocation now runs under `--no-default-features --features postgres`,
  where the only previous invocation ran default features and therefore
  never compiled the PostgreSQL-only query bodies at all — proven by
  planting a lint inside one and watching the default run stay green. And
  `spec_scan` rule 3 pins which requirement statuses claim an absence of
  test coverage, reporting any *change* to that set rather than judging
  whether a claim is true, so a new one cannot be written unnoticed.

### Fixed

- **`peisear-storage`'s dialect module no longer tells a reader that
  conditional compilation lives in two files.** It lives in eleven, at 22
  sites, fourteen of them production query pairs — a second,
  PostgreSQL-only SQL body beside the SQLite one — and the comment said
  there were none. It now says where they are and, more usefully, **why**:
  three distinct reasons (date and time functions, a row lock where SQLite
  leans on a whole-database one, and an upsert with no SQLite equivalent),
  only the first of which is mechanical. A contributor who trusted the old
  sentence would have edited one body and shipped the other unchanged.
- **Twelve requirement statuses that contradicted the shipped code**, four
  of them from a single commit that added four tests and updated none of
  the four statuses they existed to satisfy. Two are **P1** and one a
  **P0**. No behaviour changes; what changes is that the specification now
  describes what the suite actually covers.

### Changed

- **`install.md`'s prebuilt-binary command uses the same glob the README
  does** — `peisear-*-x86_64-…` rather than `peisear-<version>-x86_64-…`.
  A `<version>` placeholder pasted literally fails; the glob pastes and
  runs, and the two documents now agree byte for byte.
- **The README's source-checkout block has its own
  `## Build from source` heading** instead of sitting unheaded at the end
  of the Quick Start. The README's own closing sentence already said
  Install *"covers all three paths in full"* while that section held
  four, so the heading was what disagreed with the document — the
  block's position, order and commands are unchanged.

## [0.60.0] — 2026-11-01

**A value the operator supplied and peisear cannot use stops the
server, by name, naming what would have been accepted — and absence
still degrades exactly as before.** `RFC 022`, three work rounds
(`CFG-001`–`CFG-003`, two of them sent back a round), generalising
the shape three variables already had (`PEISEAR_SETUP`, `JWT_SECRET`,
`BIND_ADDR`) to the four that had not: `COOKIE_SECURE`, `RUST_LOG`,
`SMTP_PORT`, `SMTP_TLS_MODE`. The product now has **one** rule for
this input class where it had two. `§10.51` — opened at `0.59.0`,
found by the architect's own review — closes with it, on a plant
rather than a feeling: a reintroduction of its own named defects, in
the layout they actually had, now fails the suite by name rather than
passing it silently.

**`README.md`'s own front door changed too, landed under
`[Unreleased]` the day before this cut and folded in here.** The
Quick Start had offered only the two paths that need Rust, one of
them also `git`, for seven releases, while
[Install](docs/getting-started/install.md) had carried a prebuilt
binary and a container image since `0.53.0` — the front door
contradicted the install page, and `NFR-REL-007` is why nothing
caught it: it required the README's *ordering* and said nothing
about what the quick start must contain. That entry now carries the
missing clause; **what enforces it is still a reader, not a scan** —
the guard for it (`QS-002`) is open and this release does not wait
for it (`§7`).

### Changed

- **`README.md`'s Quick Start now leads with a path that needs
  nothing installed** — download the binary for your platform and run
  it — followed by `docker run` as a one-command alternative, then
  `cargo install` marked *if you already have Rust*, then the clone,
  which is for reading or changing the source and says so.
- **`COOKIE_SECURE` no longer treats every value other than `1`/
  `true` as `false`.** It still does, for exactly `1`, `0`, `true`,
  or `false` (case-insensitive for the words) — unset still degrades
  to `false`, unchanged. Anything else now refuses startup, naming
  the value given and the accepted set, instead of silently running
  with cookie security the operator did not actually ask for.
  `yes`/`no`/`on`/`off` are deliberately not accepted alongside
  them: a set with eight spellings is a set nobody remembers, and
  `1`/`true` must keep working exactly as before regardless (`RFC
  022` §5) — narrowing the accepting direction would be a worse
  regression than the one this round fixes.
- **`RUST_LOG` no longer silently falls back to the built-in filter
  on a value it cannot parse.** Unset still takes the built-in
  filter, unchanged. A value `tracing_subscriber::EnvFilter` rejects
  now refuses startup, naming `RUST_LOG` and carrying `EnvFilter`'s
  own parse error rather than a message this round would have had to
  invent.
- **`docs/getting-started/configuration.md`**'s reference table
  states `COOKIE_SECURE`'s accepted set and notes that both
  variables now refuse rather than silently default on a bad value.
- **`SmtpConfig::from_env` is now fallible** (`Result<Option<Self>,
  SmtpConfigError>`, was `Option<Self>`) — a signature change and one
  `?` at its single call site, not a refactor. `Ok(None)` still means
  "email channel not configured" and still never blocks a boot;
  `Err(_)` is the new fact, "a value was given and it is wrong."
- **`SMTP_PORT` no longer silently falls back to `465` on a value
  that does not parse as a port number.** Unset still takes `465`,
  unchanged. A present, unparseable value (including an explicitly
  empty one) now refuses startup, naming `SMTP_PORT` and the value
  given.
- **`SMTP_TLS_MODE` no longer silently derives a mode from the port
  on a value `from_str_loose` does not recognise.** Unset still
  derives from `SMTP_PORT`, unchanged. A present, unrecognised value
  now refuses startup, naming `SMTP_TLS_MODE` and listing every
  spelling the parser actually accepts.
- **The cascade this closes**: a malformed `SMTP_PORT` used to
  silently fall back to `465`, and that defaulted port then silently
  decided `SMTP_TLS_MODE` too — changing transport security with no
  sign anything was wrong. After this round a malformed `SMTP_PORT`
  refuses before any `SMTP_TLS_MODE` decision is ever made.
- **`docs_scan` rule 8** (`CFG-003`) enforces the rule above itself —
  a plantable predicate over `env::var` call sites plus an asserted
  census, not a list of the thirteen variable names. Internal; no
  behaviour of its own.
- **`docs/specification/external-design.md` §11.2 and its own SMTP
  entity row** now name the third, refusing case alongside the two
  degrading ones.

### Fixed

- **`CHANGELOG.md`'s own archive**, `DEC-055`'s first move since
  `0.50.0`: ten versions (`0.50.0`–`0.59.0`, eleven section headings
  since `0.58.1` is a patch) moved verbatim to
  [`changelog/0.50-0.59.md`](changelog/0.50-0.59.md), in their own
  commit.

### What this release does not fix

- **`NFR-REL-007`'s new clause ships enforced by a reader, not a
  check.** `QS-002`, the `docs_scan` rule that would enforce it, is
  written and open; this release does not wait for it — `§10.36`'s
  class, accepted for one release and named rather than left
  implicit.
- **`RUST_LOG` is fixed but not guarded.** No literal
  `env::var("RUST_LOG")` exists anywhere in this codebase for
  `docs_scan` rule 8 to anchor on, and its reach is narrow even
  fixed: `RUST_LOG=dbug` is accepted as a target name with an
  implicit `TRACE` level. Both bounds are in `§10.51`'s own closing
  note and `RFC 022` §4.
- **`§10.50`** — no CI job compiles for any platform but Linux
  outside the release tag.
- **`§10.32`** — `BROWSER-001`'s fixture cannot render four branches
  its pages contain.
- **An attended start prompting is unproven on Windows.**
- **`SMTP_TLS_MODE` accepts seven spellings for two modes**, five of
  them undocumented; deliberately not narrowed, on `RFC 022` §5's own
  constraint.

**No migration.**

## Earlier series

Older release notes are archived, one file per series of ten minor
versions:

- [0.50 to 0.59](changelog/0.50-0.59.md)
- [0.40 to 0.49](changelog/0.40-0.49.md)
- [0.30 to 0.39](changelog/0.30-0.39.md)
- [0.20 to 0.29](changelog/0.20-0.29.md)
- [0.10 to 0.19](changelog/0.10-0.19.md)
- [0.1 to 0.9](changelog/0.1-0.9.md)
