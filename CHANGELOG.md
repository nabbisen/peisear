# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
