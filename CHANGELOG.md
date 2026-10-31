# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

**`CFG-001` and `CFG-002`, the first two of three rounds applying
`RFC 022` §3's one rule**: *absence degrades; a value the operator
supplied and the product cannot use refuses startup, by name,
naming what would have been accepted.* `COOKIE_SECURE` and
`RUST_LOG` (`CFG-001`) already had a named error type and a
`Result` to refuse through; `SMTP_PORT` and `SMTP_TLS_MODE`
(`CFG-002`) did not — `peisear-notify` gained one. `CFG-003` (the
rule, enforced) is the remaining round.

### Changed

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

## [0.59.0] — 2026-10-31

**PostgreSQL is a configuration peisear says it supports, and now
means it; an operator who wants email can read about all seven
variables that configure it.** `RFC 021`, in four work rounds
(`SUP-001`–`SUP-004`). Stated at the RFC's own four conditions for
*supported*, because that is the rule this release was scoped
against: reachable from `install.md`, variables documented where
configuration is documented, CI exercising the same population it
exercises the default on, and a declared version range of one tested
version — `postgres:17`, said as one rather than implied as many.
`RFC 021` §5 — end the backend rather than finish it — was on the
table until `SUP-002` ran the suite; it is not the outcome.

**This release's second subject: both specifications' own record of
themselves had drifted**, found while cutting it. `requirements.md`
and `external-design.md` each carry a `Covers release` header and an
Appendix C baseline row, and across the two documents those gave
three different answers to "which release does the specification
cover" — one of them, `external-design.md`, had not been amended at
all since `0.57.0`. Both brought current; the three releases'
Appendix C rows that were never written are backfilled, each marked
as backfilled and dated, not written as though produced at their own
cuts. Recorded as `§10.36` instance 7 — a missing accounting row and
one that says nothing changed are different facts, and nothing
mechanical in this project could tell them apart.

### Added

- **`cargo install peisear --no-default-features --features
  postgres`** is the one documented way to reach PostgreSQL — not a
  fourth artefact shape; the archives and the container image stay
  SQLite only. `--no-default-features` is not optional: Cargo
  features are additive, so the feature name alone silently relinks
  the SQLite driver `§10.48`'s own fix removed.
  [Install](docs/getting-started/install.md)'s own "Running against
  PostgreSQL instead" has the exact command, why the flag matters,
  and that there is no first-run wizard on this path —
  `DATABASE_URL` and `JWT_SECRET` are both set explicitly before the
  first start.
- **All 33 `peisear-web` integration binaries now run against a real
  PostgreSQL server in CI, unfiltered** — 30 of them for the first
  time. Nine real defects found and fixed along the way: raw
  SQLite-only `?N` placeholders left unwrapped in test code; a
  `SELECT DISTINCT` combined with an `ORDER BY` column outside the
  select list; `SUM(...)` aggregates decoded as a non-nullable
  integer where SQL `NULL` is the correct answer over zero rows; a
  `SELECT 1` literal decoded at the wrong integer width; a SQLite-only
  `PRAGMA` with no PostgreSQL equivalent; raw string timestamp binds
  against a `timestamp with time zone` column; a same-second test
  fixture that held only by accident on SQLite's coarser clock; a
  named collation PostgreSQL has none of; and a test that opened
  `BEGIN IMMEDIATE` directly instead of using the real production
  lock seam. None of the 33 needed excluding as genuinely
  SQLite-specific.
- **All seven `SMTP_*` variables documented** in
  [Configuration](docs/getting-started/configuration.md)'s new
  "Email (SMTP)" section — four had been named in the startup
  warning, three (including `SMTP_TLS_MODE`) nowhere operator-facing.
- **`NFR-CMP-007 — Persistent state on the PostgreSQL backend`**, a
  new requirement. `NFR-CMP-003` reserved this decision for the owner
  on a trigger it set for itself — *"the day a PostgreSQL install path
  is documented"* — and this release fired it. Without the entry,
  0.59.0 would have shipped a supported configuration whose persistent
  state no requirement governed, with
  [Backup](docs/operations/backup.md) carrying a PostgreSQL section no
  requirement required. It requires what that page already said:
  peisear coordinates no backup for a server it did not start and does
  not own; `pg_dump`/`pg_restore`, WAL archiving or a provider's
  snapshot are the operator's own path; and the backup unit is the
  database alone, because no signing-key file is generated beside a
  PostgreSQL connection string — `JWT_SECRET` must be set explicitly
  or startup refuses, by name.

### Changed

- **The SMTP startup warning no longer names four variables as
  though they were the set.** It names none and points at the
  documentation, so it cannot drift from `peisear-notify`'s own
  `from_env` again the way the four-name list already had.
- **`docs/operations/backup.md` and `scaling.md`** no longer say
  PostgreSQL is unsupported — both corrected to the now-true status,
  with `scaling.md`'s own stale "37 of 335" accounting rewritten to
  what `SUP-002` actually proved.
- **`external-design.md §1.1`** no longer describes a single-backend
  product. `0.55.0`'s own Appendix C row made that description
  conditional on the feature staying off by default; this release's
  own install path removes that condition, so the prose, the
  boundary diagram and the external-entities table now name both
  backends.

### Fixed

- **`requirements.md` §6.5 and `NFR-CMP-004`** had stopped at
  migration `0015`; there are 22 under SQLite and 23 under
  PostgreSQL today (`0023_rowid_surrogate_for_ordering`, the one the
  `rowid` ordering tiebreak needed once it became a real column
  rather than SQLite's implicit one). Both amended with the current
  counts and the asymmetry named, not only the numbers.

### What this release does not fix

- **`§10.51`** (opened by this release): `SMTP_PORT` and
  `SMTP_TLS_MODE` each discard a malformed value silently — where
  `PEISEAR_SETUP` refuses one by name on the very same page — and
  because the TLS mode derives from the port, a malformed port
  silently changes transport security too. Unscheduled; a design
  call with a security dimension, not a cut's to make.
- **`§10.50`**: no CI job compiles for any platform but Linux outside
  the release tag.
- **An attended start prompting is unproven on Windows** — five
  diagnosed attempts, the account in `release.yml`'s own comment.
- **PostgreSQL's declared range is one version**, `postgres:17` —
  narrower than the ecosystem, honestly.
- **The first-run wizard writes a SQLite `.env` only** — a PostgreSQL
  operator sets configuration by hand, a documented gap rather than a
  hidden one.
- **The container image's wizard is proven on a runner's pty and in
  CI, not inside the published image.**

## [0.58.1] — 2026-10-29

**The terminal nobody is reading.** `RFC 020` §7 (amended 2026-10-28):
`should_run_wizard`'s own predicate asked whether a terminal was
attached, not whether anyone could answer it — a background start
stops dead instead of serving, and the same cause cost `0.58.0` its
own Windows archive. Two series rounds against two scoped
(`TTY-001`, `TTY-002`), the second earning a round of its own after
its first review found the fix proven by hand but not yet guarded.

**`0.58.0`'s own claim was wrong in a third shape.** It said the
wizard "is skipped entirely where a prompt would be an outage,"
tested in two — no terminal, configuration already exists. A third
was not: a terminal attached that this process cannot read. There, a
prompt *was* the outage. The only check that ever put a terminal in
front of the binary was the Windows release walk, which runs on a
tag — it worked, and the earliest it could speak was while `0.58.0`
was already being produced.

### Added

- **`PEISEAR_SETUP`**, a launch-environment variable (`auto`, the
  default; `never`, to refuse all prompting) — the documented way to
  declare the answer on Windows, where no foreground-process-group
  concept exists for the kernel to answer the way it does on Unix.
- **`test.yml`'s `test-wizard-skip` gains case 7**: a real pty, the
  server backgrounded with `set -m` so it lands in a process group
  distinct from the terminal's own foreground one — this entry's own
  defining case, exercised by no check before this release. Seen red
  on a planted predicate (`[1]+  Stopped`, bash's own `wait: warning:
  job stopped`, in the log) and green at the commit that shipped.

### Fixed

- **`should_run_wizard`'s own predicate.** Replaced `stdin_is_terminal`
  with `can_prompt`: `PEISEAR_SETUP` first, then a terminal, then — on
  Unix — the kernel's own foreground-process-group test (`tcgetpgrp`
  compared against `getpgrp`, the same comparison it makes before
  raising `SIGTTIN`). A background job, and several init/supervisor
  shapes that detach from a controlling terminal while stdin still
  names a real one, now start on defaults rather than stopping, with a
  one-line notice on stderr naming that this process could not read
  the terminal.
- **The Windows release archive.** `Start-Process` gives a child a
  console nobody will ever type into; that console answered "a
  terminal is attached" with no way to answer the wizard's questions,
  so the walkthrough step timed out and the upload/attach steps never
  ran — `0.58.0`'s Release carried four of the five archives
  `install.md` offers, a platform `0.56.0` and `0.57.0` both had.
  Fixed by the predicate above, and the walkthrough divided into two
  cases named for the property each proves rather than the platform:
  an unattended start serves (both platforms; Windows declares it via
  `PEISEAR_SETUP=never`), an attended start prompts (Unix, a real
  pty).

### What this release does not prove

- **An attended start prompting is unproven on Windows.** Five
  measured attempts at injecting real key-event input into a separate
  console (`AttachConsole`, `WriteConsoleInput`, `CONIN$`), each
  fixing one concrete, diagnosed defect in the last, ended with every
  Win32 call reporting success and the wizard still not advancing past
  its first question. Not a product gap — Windows' own asymmetry means
  *attended* there is just *a terminal is attached*, which
  `Start-Process` already provides — and the property itself is
  proven on a real pty on Unix and by `test.yml`'s own case 3. The
  account is in `release.yml`'s own comment, at the point the step
  would go.
- **No CI job in this repository compiles for Windows at all outside
  the release tag.** `test.yml` is `ubuntu-latest` throughout, so
  platform-asymmetric `#[cfg]` code — three pieces, this series — is
  first compiled for Windows at the moment the archive is produced.
  Found twice in this series (this, and BSD `script` being a different
  program from GNU `script` on macOS); carried as a finding for the
  next proposal, not fixed here.

## [0.58.0] — 2026-10-28

**The first thing it says.** `RFC 020`: a start with no configuration
asks, writes the file it describes, and is skipped entirely where a
prompt would be an outage *(corrected in 0.58.1 — this held in the two
shapes tested, not in a third; see that section)*. Five rounds against
three scoped
(`WIZ-001`–`WIZ-005`) — two added under the standing exception for a
release-blocking defect, both found reviewing the shipped wizard
against the owner's own philosophy rather than against a diff.

**`RFC 020` §2's own test of itself, answered honestly rather than
assumed, because a project that answers a documentation defect by
building a feature has chosen the more expensive repair:**

> `RFC 020` asked whether a wizard would have **prevented** the three
> defects that motivated it or merely moved them. One of three.
> `§10.40` — a copied placeholder nobody was told to replace — could
> not have happened on a wizard path, because nothing on it copies
> `.env.example` at all; **that defect was already closed in 0.53.0**
> by the startup refusal and by commenting out every line of that
> file, so what this release adds is that the copy step is no longer
> needed, not a fix for something open. The other two it does not
> touch: documentation drifting from the binary's own output
> (`FIV-004` §1b, `FIV-001` §11b) is a prose/code gap, and `§10.41`
> was a packaging defect a wizard runs too late to see. This
> release's own housekeeping fixed the current instance of the first
> by reading the page against the real binary — the same means its
> original fix used, not anything the wizard does.

### Added

- **A first-run wizard.** With no `.env` and nothing set in the
  environment, and only on a real terminal, `peisear` now asks two
  questions with working defaults — where the database lives, what
  address to listen on — and makes three statements: the generated
  signing key and where it is, that cookies are not marked `Secure`
  and how to change that over TLS, and the `.env` it wrote and where.
  Holding enter for both keeps the defaults; either answer is
  validated against the exact rule startup itself would enforce and
  re-asked, naming the expected shape, on a bad one — a database
  answer that would create an unusable path or name a backend this
  build does not have, or an address answer that is not a numeric
  address and a port together (a hostname, a bare port, or the
  address alone are each refused and re-asked, not silently
  completed or resolved by DNS).
- **Skipped entirely, never partially**, wherever a prompt would be
  an outage: no terminal (a container, a systemd unit, CI), or
  configuration already exists — `.env` found by the same
  ancestor-searching resolution `Config::from_env` uses to load it,
  or any of the four variables already set in the environment.
  Existence, not completeness: a partial `.env` is already a working
  configuration and is never re-asked over.
- **`BIND_ADDR` is validated once, by name, in `Config::from_env`.**
  `Config::bind_addr` is now a `SocketAddr`, not a `String` — parsed
  and checked a single time, with `ConfigError::InvalidBindAddr`
  naming the variable, the value, and the expected shape on failure.
  This covers every source of a broken `BIND_ADDR`, not only the
  wizard's own prompt: a hand-edited `.env`, a copied file, or the
  environment variable itself. `main.rs` no longer re-parses the
  value downstream, so there is exactly one predicate left and the
  two can never disagree.
- **A CI job proving the skip condition by behaviour**, never by
  reading `.env` or `.env.example`: six cases, including a real pty
  for every terminal case, asserting the server's own bound address
  — not merely a line's absence — as the proof that a parent
  directory's `.env` or a corrected answer was actually found and
  obeyed, not just that nothing went wrong silently.

### Changed

- **`docs/getting-started/configuration.md`, `first-run.md`, and
  `README.md`'s Quick Start** re-pointed to a product that configures
  itself: the reference table stays intact for the operator writing a
  systemd unit, and the newcomer path no longer instructs `cp
  .env.example .env` before the first run. `.env.example` itself
  survives — measured at ten live references, including a security
  refusal, a scan rule, and a dedicated CI workflow — and removing it
  is a separate proposal, not a tidy-up this release performs.
- **`RFC 020` §5's three open questions resolved**, recorded in the
  RFC itself by appending rather than editing the question away: the
  wizard writes `.env` and says the path; the skip condition is
  existence, not completeness; `.env.example` survives.

### Fixed

- **The wizard announced success before failure.** An unvalidated
  address answer (`localhost:3000`, a bare `3000`, `0.0.0.0`, or
  `127.0.0.1` — not typos, but the four forms most likely typed
  against the shown default) wrote a `.env` that could not start,
  *after* printing that it had been written — and because that file
  now existed, the wizard was skipped on every later run, locking the
  operator out of ever correcting it through the one tool meant to
  help. Closed by validating the answer before it is written at all,
  and by giving `BIND_ADDR` the same named, single-parse validation
  startup itself relies on.
- **A `.env` in a parent directory was not recognised**, because
  detection checked the current directory only while loading searches
  every ancestor. The wizard printed *"No configuration found"* over
  a configuration that was in fact found and loaded, wrote a second
  `.env` into the child directory, and the instance came up on its
  own defaults rather than the operator's. Closed by resolving
  detection with the same ancestor-searching call loading already
  makes, rather than a second, narrower guess.

### What this release does not prove

- **Seven `SMTP_*` variables are documented nowhere operator-facing**
  — `SMTP_HOST`, `SMTP_PORT`, `SMTP_TLS_MODE`, `SMTP_USER`,
  `SMTP_PASSWORD`, `SMTP_FROM_ADDRESS`, `SMTP_FROM_NAME`, read by
  `peisear-notify` and documented only in that crate's own rustdoc.
  Measured, not fixed, in this release; recorded on `ROADMAP.md`.
- **The wizard is proven on a GitHub runner's own pty, not inside the
  published container image.** A difference there is the case that
  matters most, and a runner cannot exercise it.
- **Two questions cover two of the four variables `Config::from_env`
  reads.** `JWT_SECRET` is generated, never chosen; `COOKIE_SECURE` is
  a commented statement at its built-in default, not a question —
  both deliberate, neither the same as configuring them interactively.
- **`§10.36`'s instance 6** — a refactor that would have silently
  stopped `docs_scan` rule 4 from checking anything — was found and
  fixed inside this release, by the change that would have caused it,
  before it shipped. The register this project keeps for exactly this
  class paid for itself again.

## [0.57.0] — 2026-10-24

**`requirements.md` is this project's most-edited document, and until
this release nothing mechanical had ever read it.** `spec_scan` is a
new guard module, following `docs_scan`'s own shape, that does: two
citation rules, Appendix A checked both internally and against the
168 real requirement entries, a closed status vocabulary, and eight
test-count citations held to the test runner. **The first reading
found more wrong than anyone expected** — all four findings below were
measured, not assumed, and all four are fixed in this release.
**Nothing a user runs changed**: every file this release touches is
`crates/peisear-web/src/spec_scan*`, a handful of `tests.rs` files,
`.github/workflows/test.yml`, and the two specification documents
themselves.

### Added

- **`spec_scan`, a mechanical reader for `requirements.md` and
  `external-design.md`.** Two citation rules (every `§10.N` citation
  resolves to a real heading; no four-digit `RFC 0NNN` reference
  outside the frozen history directory); Appendix A's own arithmetic,
  checked row by row and at both grand totals; Appendix A checked
  against the 168 real requirement entries, area by area and column
  by column, with the rule asserting the population it examined (168
  entries, 25 area segments) so a narrower-matching pattern fails
  loudly rather than passing quiet; a closed status vocabulary mapping
  every entry's own governing status to one of Appendix A's five
  columns; and an extractor for this document's own
  `` `name` (N tests) `` acceptance citations.
- **Seven CI steps**, one per cited integration-test binary, checking
  each citation against `cargo test --test <name> -- --list` — never
  a grep over `#[test]` — in the same job that already builds that
  binary, so the marginal cost is one `--list` per target rather than
  a rebuild. Each asserts its own expected citation count before
  comparing, so a renamed extractor or a removed citation fails loudly
  rather than passing on an empty, vacuously-true comparison.

### Fixed

- **Thirteen `§10.N` citations in `requirements.md` and
  `external-design.md` pointed at four numbers with no heading.** All
  four had real entries in an older compact register — dated, naming
  their closing handoffs — just never promoted to the heading
  convention every neighbouring number eventually received. Promoted,
  content budgeted to the table row and its own citing sentences; rule
  1 is clean on the real tree.
- **Nine of Appendix A's twenty-five areas were stale** — some never
  re-derived since the document existed, two re-derived once and
  drifted since. Calendar's row read `Specified: 7` while all eight of
  its entries read `Met` or `Implemented`. Re-derived from the 168
  entries' own governing status; both grand totals recomputed from
  their member rows rather than adjusted by the deltas:
  `99 | 96 | 1 | 2 | 0 | 0` (functional), `69 | 60 | 3 | 0 | 2 | 4`
  (non-functional). Two entries' own stale prose, found while
  re-deriving, corrected first: `NFR-A11Y-009` read "Not implemented"
  when the document's own 0.43.0 note already called it the Deferred
  one; `FR-CAL-001` carried a correction note the running application
  itself contradicts (its route exists, its named test passes),
  withdrawn as superseded and left in place per this document's own
  convention — the original is never rewritten, the correction is
  recorded beneath it.
- **The status vocabulary had drifted**: `Partial` and `Partially
  implemented` named the same status; `Met` (thirty-one entries) had
  no column in Appendix A at all. The one spelling normalised; `Met`
  confirmed, not merely proposed, as a synonym for `Implemented` —
  the document's own 0.43.0 note already said so in as many words.
- **Three of eight test-count citations were wrong, live, on the
  tree**: `view_state` read 5, is 9; `aggregate_privacy` read 12, is
  13; `optimistic_lock` read 21, is 22 — each verified with the test
  runner and corrected directly.

### What this release does not prove

- **A requirement's status is not checked against the shipped code.**
  `§10.34`/`REQ-003` measured four candidate text rules against six
  historical instances of exactly this; the best scored 4 of 6 and was
  argued down by its own authors, because its zero false positives
  were on zero opportunities. Nothing here changes that finding —
  `spec_scan` reads the vocabulary a status uses, never whether that
  status still matches the running application.
- **Most prose counts stay unchecked, by design.** A dated measurement
  and a present-tense claim are indistinguishable in isolation; this
  project's own long-used convention of dating a measurement inline is
  what actually keeps the bulk of them honest, and only the one narrow,
  already-in-use `` `name` (N tests) `` shape is held to the runner.
- **Twenty-four `§10.N` numbers exist in both an older compact register
  and the full heading form**, and could drift against each other with
  nothing here to notice — the two forms use unrelated vocabularies, so
  there is no cheap reuse of the status-vocabulary check for them.
  Recorded as a known, unchecked population rather than built.
- **Thirty-seven `file:line` citations remain this project's most
  fragile citation form.** The class demonstrated itself twice during
  this very release — once inside a single round, caused by that
  round's own edits to the file it cited. Not fixed here; recorded for
  a later release to take on deliberately.
- **`optimistic_lock_atomicity`'s own test count sits outside the
  extractor's pattern**, by decision rather than oversight — it is
  cited in a list-continuation shape the extractor does not match, and
  the population this release holds to the runner is eight citations,
  not nine.

## [0.56.0] — 2026-10-22

**The claim that both backends work, made by something that would
notice if they did not — plus the operations story a second backend
implies.** `RFC 018` P4, the phase's own last one. Scoped as four
rounds; six closed, because two production defects the series' own
widening surfaced earned rounds of their own rather than being folded
into the ones that found them. **Nothing an existing SQLite operator
runs has moved** — the default build, its migrations, and every test
that already ran are untouched.

### Added

- **Every guarded write now executes on PostgreSQL.** Fourteen
  production call sites opened SQLite's own `BEGIN IMMEDIATE` — a
  syntax error there, so no guarded write ran on PostgreSQL at all
  before this release, though it compiled and every earlier gate
  passed. `dialect`'s own transaction seam (`begin_guarded`, plus a
  per-site mechanism — a row lock, a sibling-set lock, or an advisory
  lock, chosen by what each site actually guards) replaces all
  fourteen, and all four mechanisms are proven necessary by a test
  seen to fail without them, on a real server.
- **Every integer column in `migrations-postgres/` is `BIGINT`.**
  PostgreSQL's `INTEGER` is a fixed 4-byte type; this crate's own Rust
  layer binds every integer column as `i64`. A mechanical port of
  SQLite's own `INTEGER` (a type *affinity* of no fixed width) had
  carried the mismatch into eight shipped migration files, `issues.effort`
  among them — no issue could be read on PostgreSQL at all before this
  release. A no-allow-list conformance predicate over
  `information_schema.columns` now asserts it, seen red with every
  offender named before the fix and green after. Fixed the same
  round: `SUM(int4)` promotes to `int8` on PostgreSQL, but `SUM(int8)`
  promotes to `numeric` — widening `effort` had silently broken four
  existing `SUM(effort)` queries, caught by the real-server suite, not
  a local gate.
- **The storage suite and the 37 lock tests run against both
  backends, in CI, on every push.** `peisear_storage::pool::TestDatabase`
  gives PostgreSQL the isolation a fresh SQLite temp file already gave
  for free — a database created and migrated for one test alone, not a
  shared schema or a shared, truncated database whose correctness
  would depend on a test-runner flag. Replaces the one place this
  project's own `--test-threads=1` reset-by-hand shape had survived
  since the suite first ran under the `postgres` feature
  (`optimistic_lock_guards`). Twelve test helpers that read a row
  straight back to assert on it, writing raw SQLite-style `?`
  placeholders never exercised against a real server before this
  release made that possible, are fixed the same way every production
  query already is (`dialect::sql`, now `pub` for exactly this
  cross-crate reason).
- **`peisear`/`peisear-web` can select the `postgres` feature.**
  `peisear-storage`'s own feature existed since `0.54.0` with no way
  to reach it from the published binary; a CI boot smoke now starts
  `peisear` against a real PostgreSQL server for the first time.
  **This is not an install path** — see "What this release does not
  prove," below.
- **`docs/operations/backup.md` has a PostgreSQL section.** The
  backup unit is the operator's own instance, backed up by the
  operator's own means — peisear coordinates nothing for a server it
  did not start. Every command in it (`pg_dump`/`psql`/`pg_restore`)
  was run against a real server before being written down, including
  a full round trip, and the same commands are now a permanent CI
  step rather than a one-time proof. `docs/operations/scaling.md`
  restates the feature's status as a property rather than a count that
  would be wrong by the next release, and names the `JWT_SECRET`
  fact below next to the connection string. `NFR-CMP-003` (the
  single-file-database requirement) is retitled and scoped to say *the
  SQLite backend* explicitly — nothing it required is loosened.

### Fixed

- **`JWT_SECRET` is mandatory on PostgreSQL, and peisear now says so
  before a reader meets it as a startup refusal.** PostgreSQL's own
  connection string has no filesystem path to generate a signing-key
  file beside, unlike SQLite's. The refusal itself was already
  correct; nothing documented it until this release.
- **A `SELECT`-list alias used inside `HAVING`**, which PostgreSQL
  does not resolve the way it resolves one in `ORDER BY`/`GROUP BY`,
  broke `project_health`'s own WIP-violator count on PostgreSQL —
  found only because this release is the first to render any
  `peisear-web` page against a real server at all. Fixed, and covered
  by `project_health`'s first unit test.
- **`0.55.0`'s own crate-root doc overstated a measurement.** It said
  the remaining PostgreSQL suite failures were *"not because a
  production query is wrong"* — true narrowly (a transaction-control
  statement is not a query) but read, and accepted in review, as
  covering all of them. The real second cause was the fourteen
  `BEGIN IMMEDIATE` call sites above. `[0.55.0]`'s own section here is
  unchanged — shipped history is not rewritten — and `lib.rs`'s own
  doc comment already carries the correction.

### What this release does not prove

- **298 of 335 `peisear-web` integration tests have never run against
  PostgreSQL**, among them the only tests proving `NFR-PRIV-007`'s
  contributor floor and `NFR-PRIV-001`'s capacity visibility — both
  **P0** privacy controls, enforced by SQL this release rewrote per
  backend.
- **The alias-in-`HAVING` class above is closed at one occurrence, not
  as a class.** The same mistake in a `WHERE` clause cannot be found
  by grepping a keyword, and thirty of the thirty-three integration
  files above have never rendered against PostgreSQL at all.
- **A `--features postgres` build still links SQLite's own bundled C
  driver** (387 `sqlite3_*` symbols) — `peisear-storage`'s `sqlite = []`
  feature gates the `cfg`, not `sqlx`'s own dependency, which is
  unconditional at the workspace level. Harmless while PostgreSQL is
  unsupported; becomes a real defect the day an install path is
  documented.
- **No supported PostgreSQL version range is declared.** This release
  runs against `postgres:17` only; whether its queries hold on an
  earlier major is unmeasured.
- **Running peisear on PostgreSQL is still not a supported
  configuration.** Same five archives, same single container tag,
  `cargo install peisear` still builds SQLite only, and no document
  anywhere gives an install command for the other backend.

## [0.55.0] — 2026-10-20

**A second backend that runs.** `RFC 018` P3, plus the queries, a gap
its own phase table had no home for — four rounds (`PG-001`–`PG-004`).
The `postgres` feature `0.54.0` published with `unimplemented!()`
behind it now has real code: a connection, a ported schema, eleven
triggers, and every production query. **Nothing an existing SQLite
operator runs has moved** — the default build, its migrations and its
tests are untouched, and the feature stays off unless chosen.

### Added

- **`peisear-storage`'s `postgres` feature now has real code behind
  every part of it.** `pool::connect`/`migrate` open a real connection
  and apply a real, ported migration set (`migrations-postgres/`, 23
  files — one of them, `0023`, adds a surrogate `rowid` column
  PostgreSQL needs and SQLite already has implicitly, so nine
  production queries' own `ORDER BY ..., rowid` tiebreak stays
  byte-identical on both backends). The schema's eleven triggers exist
  on PostgreSQL too, raising the same five validations via `ERRCODE`
  rather than SQLite's bracketed message prefix, both translated to
  the same `MessageKey`. Every production query runs: a parser
  (`dialect::sql`) translates SQLite's `?N`/bare-`?` placeholders to
  PostgreSQL's `$N` mechanically, and a handful of genuinely
  backend-specific constructs — SQLite's date functions, the
  optimistic-lock guard's text/numeric timestamp normalisation,
  `sprint_records`'s upsert — are rewritten per backend and justified
  at each call site.
- **A migration-drift guard between `migrations/` and
  `migrations-postgres/`** (filename-level: the two sets must name the
  same migrations, with one named, tested exception for `0023`, which
  ports nothing — SQLite has no equivalent to port) runs under every
  feature, not only the one currently selected.
- **A PostgreSQL service container joins CI**, running a targeted set
  of real-server tests — the connection, the migration set reaching
  its own highest version, timestamp precision, the trigger-and-query
  guards above. **What this release does not prove**: this crate's
  own test *suite* still fails under the `postgres` feature outside
  that targeted set, because several test *helpers* hardcode a
  `sqlite://` pool, not because a production query is wrong; there is
  no conformance suite yet proving this crate's full storage and lock
  tests pass against both backends (`RFC 018` P4); and `NFR-CMP-003`
  (single-file database) is deliberately left describing SQLite alone
  — P4's own requirement to amend, not this release's.

## [0.54.0] — 2026-10-18

**The dialect seam.** `RFC 018` P1 + P2: trigger validation errors stop
depending on the exact wording of a message, and the storage layer
gains a place a second backend would attach — with no second backend
written yet.

### Changed

- **Trigger validation errors are matched on a code, never on
  prose.** `translate_trigger_error` used to match `sqlx::Error`'s
  rendered string against five English sentences, two of them
  prefixes of the real message rather than the whole of it — a
  reworded trigger message could have kept passing a stale match
  silently. The five triggers now carry a bracketed `[E_CODE]` prefix
  ahead of the unchanged human sentence; a new migration,
  `0022_trigger_error_codes.sql`, adds it without touching the two
  already-shipped migrations that first created these triggers —
  `sqlx` refuses to re-run a modified migration against a database
  that already applied it, so `0015`/`0016` stay byte-identical to
  what `0.53.0` shipped, forever.
- **A `postgres` feature exists on `peisear-storage`, off by
  default, with no PostgreSQL code behind it.** Not a query, not a
  migration, not a connection string — a `dialect` module owns the
  `Pool` type and the trigger error-code translation, both of which
  genuinely differ by backend; everything else a second backend would
  need (placeholder style, connection tuning, migration syntax) is
  named and deliberately left for the round that has a real server to
  design it against. Selecting the feature compiles and links,
  proving the type seam holds, and panics the moment anything opens a
  connection — stated plainly in the crate's own front page, not only
  in the module that panics.

### Added

- **`docs_scan` rule 7**: every uncommented `KEY=value` line in
  `.env.example` must match its own source's default — `config.rs`
  for most variables, `main.rs`'s `RUST_LOG` fallback,
  `peisear-notify`'s `SMTP_PORT` default. `JWT_SECRET` has no default
  to match (unset generates one); an uncommented value is itself the
  violation, which is exactly the shape of `0.53.0`'s own `§10.40`
  defect, now checkable mechanically rather than fixed once by hand.

## [0.53.0] — 2026-10-17

**The first five minutes of using peisear, end to end.** `RFC 017`:
generate a signing key instead of shipping one in every copy of the
source, give a newcomer three real ways to install it, and make the
one documented path that was publishing an unstyled application
publish the real thing.

### Security

- **The published `JWT_SECRET` fallback is gone.** An instance started
  without `JWT_SECRET` set used to sign every session cookie with the
  literal `config.rs` carried in the public repository. It now
  generates 32 random bytes, persists them beside the resolved
  database path, and fails loudly — rather than silently falling back
  on a missing-and-unwritable or corrupted key file — if that cannot
  be done (`FIV-001`). `BIND_ADDR` now defaults to `127.0.0.1:3000`
  rather than every interface, so a first run is reachable from this
  machine only until someone chooses otherwise. **If you relied on the
  previous `0.0.0.0:3000` default to reach an instance from another
  machine**, set `BIND_ADDR=0.0.0.0:3000` explicitly — the container
  image already does this for you, so this only affects a
  build-from-source or `cargo install` deployment that had no `.env`
  entry for it.
- **An instance that starts with a published placeholder secret now
  refuses to start**, naming the remedy, rather than running signed by
  a value anyone reading the source already knows. `.env.example`
  itself now ships every variable commented out, each shown at its own
  built-in default — a copied `.env` changes nothing and can never
  hand anyone a setting they did not choose, so the documented
  `cp .env.example .env` path reaches a working instance, not the
  refusal. The refusal stays: it is what makes that file's state
  checkable rather than assumed, and it still fires for anyone who
  uncomments the line and types the placeholder back in by hand.

### Added

- **Three real ways to install peisear**, replacing a page that only
  ever described building from source:
  `docs/getting-started/install.md` now offers a downloaded archive,
  `docker run`, or `cargo install`, each ending at
  `http://localhost:3000`; the original build-from-source content
  moved intact to `docs/getting-started/build-from-source.md`
  (`FIV-004`).
- **A per-platform archive on every GitHub Release** — five targets
  (`x86_64`/`aarch64` Linux, `aarch64`/`x86_64` macOS,
  `x86_64` Windows), built natively with no cross-compilation and no
  third-party GitHub Action, attached after the Release already
  exists and never gating it (`FIV-002`).
- **A container image, `ghcr.io/nabbisen/peisear`**, built from a
  `Dockerfile` in the repository (`gcr.io/distroless/cc-debian12`,
  no shell, no package manager) and published alongside the Release
  the same way, with `BIND_ADDR=0.0.0.0:3000` set by the image itself
  (`FIV-003`).
- **One line of plain-English output after a successful bind**,
  naming the URL to open on loopback, or warning that the instance is
  reachable from the network otherwise — alongside the existing
  structured log line, not instead of it (`FIV-004`).
- **Both new publishing steps are exercised by `workflow_dispatch`
  against a real ref and have never yet run against a real Release.**
  `NFR-REL-009` (five archives attached) and `NFR-REL-010` (an image
  pushed to `ghcr.io`) stay **Divergent** — shipped, dispatch-tested,
  but their own `release: published` guard has not fired for real
  yet. They close the first time an actual Release does.

### Fixed

- **`cargo install peisear` installed an application with no
  stylesheets and no scripts.** `static/` sat outside every published
  crate; `0.52.0`'s own `README.md` led a newcomer straight to it.
  `static/` is now embedded into the binary at compile time
  (`include_dir`), served with a computed `ETag` and
  `Cache-Control: no-cache` rather than no caching header at all —
  closing `requirements.md §10.41` (`FIV-005`).
- **`install.md`'s caption named an empty project list; none of its
  five screenshots showed one, and four duplicated the very next
  page.** Replaced with a single capture of the real empty state
  (`FIV-004`).
- **`/today`, the screen `README.md` sells hardest, had never been
  captured.** One new capture, one reused image, both in `README.md`
  (`FIV-006`).

### Internal

- **`NFR-REL-011` moves from Partial to Implemented**: the documented
  install paths are now each exercised end to end by CI, on the
  artefact the path actually delivers, not by a human walk before a
  tag. `image.yml` already walked the container path; this release
  adds the other two — `release.yml` now extracts the archive it just
  built and runs it to an authenticated session on every one of the
  five targets, and a new `test-install-walk` job in `test.yml` does
  the same for `cargo install`, run from outside the checkout, on
  every push and pull request.
- Both new CI steps were proven by `workflow_dispatch` against a real
  ref before this release, the same method `FIV-002` and `FIV-003`
  used for their own workflow changes — never taken on faith at the
  tag.

## [0.52.0] — 2026-10-14

**Nothing you can see changed, and nothing you run differently.** No
screen, no route, no copy, no migration — the third release running
with none. Every line that ships to a running instance is what
`0.51.0` shipped; the only line gated behind `#[cfg(test)]` is a new
module declaration.

**What changed is the checking.** This project decides what is true
about itself by running guards, and this release is about two of them
that were not answering the question they were named for — both found
by looking at the guards rather than at the product.

### Fixed

- **A guard reporting clean on files it could not read.**
  `strip_line_comments` existed in six copies; five truncated any line
  holding `xmlns="http://…"`, which corrupted quote-pairing for the
  rest of five component files. `touch_target_scan` is one of those
  five, and `NFR-A11Y-007` rests on it. One copy now —
  `scan_support::strip_line_comments`, the one that was already
  correct — which reverses `QA-002`'s duplication call for this helper
  and nothing else. Demonstrated by plant: a violation planted in a
  corrupted region passed at `0.51.0` and fails now.
- **A rule that enforced the spelling of what it banned.** `docs_scan`
  rule 5 forbids citing the project's private, gitignored area, and
  matched only the literal path; this project's own house style for
  such a citation is the bare file name. 64 such citations sat in
  published doc comments and in the specification itself while the
  rule reported clean. Cleared by stating the substance — never by
  publishing the private file — and the needle widened to a shape that
  reads nothing under the gitignored path, so it works the same way in
  a published crate or an extracted tree as it does here.

### Internal

- Fifteen files that were wholly tests now use the shape
  `project-instructions-rust.md` gives (`<name>.rs` + `<name>/
  tests.rs`). `NFR-MNT-005` is `Implemented`, with a measured live
  count of zero; `§10.37` closes.
- The `peisear-i18n`/`peisear-core` guard-location inconsistency,
  raised against this release, is declined: the hazard it rested on
  (a `tests/` file not running in CI) is already closed mechanically
  by `dec_007_fs_scan`/`dec_007_ci_scan`, demonstrated by a planted
  test file that the scan catches by name. Either location is correct.
- `components.rs`'s own doc comment named the touch-target guard's
  call-site count as 136; it is 212 (171 `grow()` and 41
  `grow_text()`). Corrected with both figures stated, so the next
  reader sees why the guard keys on the symbol rather than a count.

**What this release does not fix.** The widened rule-5 needle has a
residual of nine privately-named files it cannot match by shape,
stated with the number in `§10.39`; 45 occurrences in `//`
implementation comments and `crates/*/tests/` are out of scope by
design and left. The figures in this release's own specification
register were corrected three times before they were right — recorded
there, because measurement discipline is what this release is about.

**No migration, the third release running — `0021` remains the most
recent.** Test inventory **442 → 451**: `scan_support`'s four new
tests and the widened rule-5 needle's five; moving a test between
files, as the fifteen-file split did, neither creates nor destroys
one.

## [0.51.0] — 2026-10-13

**This release's subject is documentation accuracy — and it contains two
things that are not documentation, because the scope was widened to include
them.** A label that sat above its neighbour now sits beside it, and the
sprint plan no longer overflows on a phone. Nine decisions a requirement
cites were stated nowhere at all; now each is, once, where it governs. This
is not a documentation-only release.

### Fixed

- **A 44px touch target was being applied without centring its content, at
  41 sites.** `components::grow_text()` composes the touch target plus
  `inline-flex items-center`, for a plain-text label such as a breadcrumb or
  a nav link — distinct from `components::grow()`, which keeps its original
  meaning at 171 sites that were already flex containers or form controls.
  A reader notices this on any project page (the capacity hint now sits
  level with `WORKLOAD`, not above it) and on an issue's sub-issue list (a
  status badge now sits beside its title, not below).
- **The sprint plan's own Backlog row no longer overflows a 320px phone.**
  A row's source-project name had no truncation — latent for as long as the
  plan page's Backlog and Sprint Items panels were always swept empty by
  the fixture that exercises them.

### Changed

- **`README.md`'s front door now describes the product.** It read *"issue
  tracking"*; the requirements say the aim is keeping a project — and the
  people in it — sustainable. The hero, overview and features now say
  that, and the backup bullet no longer teaches the technique
  `docs/operations/backup.md`'s own "The wrong way" section forbids.
- **The documents that describe the code now agree with it.** Three
  documents named six crates where there are seven. A deployment manifest
  listed two of eight shipped files. Nineteen citations pointed into the
  project's own gitignored private working area — closed per the ruling
  that such a link is either missing documentation or a useless one.

### Added

- **Nine decisions a requirement cites, stated once, where they govern** —
  previously named only as a *Source* citation or scattered across several
  mentions with no single statement anyone could point to. `DEC-013`
  (`NFR-CONC-003`, why the database alone maintains `updated_at`) and
  `DEC-049` (the touch-target presumption, amended — see Fixed, above —
  with its superseded clause marked withdrawn rather than rewritten) are
  named specifically; seven more follow the same rule.
- **`alignment_scan`**, a new guard: every `grow()` call site must already
  be a form control or a flex container, and no `grow_text()` site may be
  a form control. It caught six sites its own first pass misclassified on
  the day it was written — daisyUI card links and explicit `block` links
  wrapping rich content, not the plain-label shape the fix exists for.

### Internal

- Rule 1b now checks the crate *count*, not only the names. Rule 2 reads
  `.github/CONTRIBUTING.md` too, not only `crates/*/tests/`. Rule 5 is
  narrowed to documents (a decision citing `.git-exclude/` for its
  reasoning is now a violation) and switched on.
- Nine files whose tests sat inside their own implementation source moved
  to `src/<mod>/tests.rs`. `NFR-MNT-005`'s status stays `Divergent`:
  fifteen files remain — fourteen test-only modules with no
  implementation to separate tests *from*, plus the `alignment_scan`
  this release wrote — which is an open question for the owner's own
  `project-instructions-rust.md`, not this release's to settle alone.
- `NFR-CONC-003`'s own `updated_at_authority_scan` citation now states the
  guard's coverage limit: the needle catches only a `SET updated_at =`
  where `updated_at` is the first column named. No live violation found;
  widening it is not done here.

**What this release does not fix.** `strip_line_comments` is copied into
**six** scans, and **five** carry a naive `split("//")` that corrupts
quote-pairing on five component files holding a URL string literal
(`xmlns="http://..."`), found measuring this release's own call-site count.
The sixth copy — `alignment_scan`'s, written here — is already
string-aware, so the fix is five copies adopting one that already works.
Guard infrastructure, not documentation; its own round next.
**And rule 5 cannot see the commonest form of what it bans.** It scans
documents, RFCs *and* published doc comments, but matches on the
gitignored path itself — and a citation written as a bare review-file
name never spells it. 31 such files exist only in the project's private
working area. `crates/` carries **60** of those citations across 25
files, **27** of them doc comments that render on docs.rs. The rule
reports clean, which is `§10.36` in the guard `RFC 019` asked for.
`updated_at_authority_scan`'s first-column limit, above.
`NFR-MNT-005`'s kind-2 question, above.

**No migration, the second release running — `0021` remains the most
recent.** Test inventory **431 → 442**, measured directly, three
consecutive clean runs.

## [0.50.0] — 2026-10-12

**This release's subject is the storage contract: what a committed write
means, and what leaves the process.** A P0 concurrency requirement had a
window it could not see; a committed write now survives a power cut; two
secrets stopped being one careless line away from a log file; data at rest
joins the threat model for the first time; and a new register entry records
a class of check that found itself, once, in the release that named it.

### Fixed

- **A one-second window in which the optimistic lock could be spent twice.**
  `NFR-CONC-001` exists so one person's edit cannot silently overwrite
  another's, but SQLite's `CURRENT_TIMESTAMP` is whole-second and the lock
  token reuses it (`NFR-CONC-002`), so two edits to the same row inside one
  second shared a token and the second was admitted. **Demonstrated through
  the real handlers before it was fixed** — the test failed on the unfixed
  tree and passes now. Migration `0021` moves the token to millisecond
  resolution. **Say the residual plainly: the window is now one
  millisecond, not zero.** This does not close `NFR-CONC-001` entirely —
  `NFR-CONC-002`'s own entry now says so.

### Changed

- **A committed write now survives a power cut or an OS crash immediately
  after, rather than at the next checkpoint.** `synchronous = FULL`: the
  write-ahead log is fsynced on every commit. **Measured, not assumed** —
  about 2 ms on a ~440 ms real write path on NVMe, order randomised across
  ten rounds to rule out a measurement-order artefact that misled the first
  pass. **Deliberately not configurable** — documented in
  `docs/operations/backup.md`, with why: a lever that can silently trade
  away a durability guarantee is worse than no lever.

### Security

- **Two secrets stopped being one careless line away from a log file.**
  Neither was leaking today; closing a hazard while it is still cheap is
  the point. The database URL is now redacted at the source before the
  startup line logs it — scheme, host and path kept, userinfo dropped
  entirely — so the password in a future PostgreSQL DSN cannot reach a log
  file by a future line forgetting to redact it. And `Config` no longer
  derives `Debug` over the struct holding the session-signing key; the
  hand-written replacement redacts it to a fixed placeholder.

### Added

- **Data at rest is in the threat model for the first time.** 194 lines of
  security documentation had never used the words *encrypt*, *at rest* or
  *disk*, for a product whose defining constraint is that personal
  sustainability data is visible only to its subject. `docs/security/hardening.md`
  now names the asset, names volume encryption as the control, and records
  the owner's declined alternatives — SQLCipher, column-level encryption —
  on the record, without claiming more than that.

### Internal

- **A new register entry, `§10.36`: a check that cannot tell *nothing
  wrong* from *nothing examined*.** Five instances across the last two
  releases, and the honest part is told plainly: the class was named,
  written down, and **reintroduced in this same release** while a flake
  was being fixed, caught in review before it shipped. `pool.rs`'s
  round-trip test now plants a known fraction rather than relying on the
  clock, so neither half of it can coincide its way to green.
- `optimistic_lock_guards.rs` (new, renamed from `store002_guards.rs`,
  which was the workspace's only ID-named test module) and
  `optimistic_lock.rs`'s own `STORE-001` test both retry their legitimate
  edit, bounded, until the token moves — the same `.000` coincidence `§10.36`
  names, closed by construction rather than by luck.
- `NFR-MNT-005`'s status is corrected from `Implemented` to `Divergent`:
  25 files under `crates/*/src/` carry 79 in-file tests, which its own text
  prohibits. The practice is almost certainly right and the requirement
  almost certainly wrong — reconciling which is `0.51.0`'s decision; this
  release only stops the document disagreeing with the code.

**What this release does not fix.** The lost-update window above is
narrowed, not closed. `updated_at_authority_scan`'s single spelling
(`updated_at = CURRENT_TIMESTAMP`) does not catch every way the application
could write the column itself, and `0021` made the likeliest future mistake
harder to see, not easier — widening it is `0.51.0`'s. `NFR-MNT-005`'s rule
is corrected in status only, not reconciled.

**No migration since `0020` in ten releases — `0021` is the eleventh.**
Test inventory **421 → 431**: five guard tests (`STORE-002`), three in
`config.rs` (`SEC-001`), two from `STORE-001`.

## Earlier series

Older release notes are archived, one file per series of ten minor versions:

- [0.40 to 0.49](changelog/0.40-0.49.md)
- [0.30 to 0.39](changelog/0.30-0.39.md)
- [0.20 to 0.29](changelog/0.20-0.29.md)
- [0.10 to 0.19](changelog/0.10-0.19.md)
- [0.1 to 0.9](changelog/0.1-0.9.md)
