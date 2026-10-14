# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
