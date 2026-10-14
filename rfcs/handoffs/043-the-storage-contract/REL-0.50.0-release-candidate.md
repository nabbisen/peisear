# REL-0.50.0 — release candidate

**Contents**: `STORE-001` (`f80762a`), `STORE-002` (`63ea707`), `SEC-001`
(`8aec73d`), `REG-001` (`9e5cd74`), and `RFC 018` accepted (`2b91925`,
`a84a47e`, `b6c9b49`, `47ea5da`). **Every handoff reviewed and closed.**

**`DEC-028` is this round's work** — neither specification covers `0.50.0`
yet — **and it carries two amendments beyond the baseline rows**, §2.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## §1 — the ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only. **`0.49.0` → `0.50.0`.**

**One migration, `0021`** — the eleventh, and **the first since `0020`
in ten releases**. The database check below changes shape because of it.

**Expected: 431**, from 421. Ten tests: five guard tests (`STORE-002`),
three in `config.rs` (`SEC-001`), and two from `STORE-001`.

**Production source changed in six files** (`git diff --name-only
0.49.0..HEAD`, filtered):

| File | What | Does a user feel it? |
|---|---|---|
| `migrations/0021_…sql` | seven triggers, millisecond precision | **Yes — the lock works in a window where it did not** |
| `peisear-storage/src/pool.rs` | `synchronous = FULL`, plus tests | **Yes — durability on power loss** |
| `peisear-web/src/config.rs` | the DSN redactor, hand-written `Debug` | **Yes — the startup line changed** |
| `peisear/src/main.rs` | the redacted call site | with the above |
| `peisear-storage/src/optimistic_lock_guards.rs` | new, test-only | No |
| `peisear-storage/src/lib.rs` | `mod` lines | No |

## §2 — `DEC-028`: the baseline rows, and two amendments

**The rows and the five-pass sweep as usual.** Then two corrections that are
this release's own, both from `REG-001`'s review:

1. **`NFR-MNT-005`'s status becomes `Divergent`.** Its text says *"In-file
   `#[test]` modules inside implementation source MUST NOT be used"* and its
   status reads `Implemented`. **Measured: 25 files under `crates/*/src/`
   carry in-file test attributes, holding 79 `#[test]` / `#[tokio::test]`
   functions** — including every scan this project relies on. **Change the
   status and add a one-line `§10` pointer. Do not amend the text** — which
   rule is right is 0.51.0's reconciliation, and a status is not a design
   decision.
2. **`NFR-CONC-002` gains a precision paragraph.** The requirement reuses
   `updated_at` as the lock token; after `0021` **that token advances at
   millisecond resolution, so two writes to one row inside the same
   millisecond still share it.** Say the window plainly, and that it is
   accepted rather than closed. A reader of that requirement currently
   cannot tell what the contract depends on.

## §3 — the changelog

**This release's subject is the storage contract: what a committed write
means, and what leaves the process.** Five beats.

1. **Lead with the lost update, because it is a P0 requirement that had a
   window it could not see.** `NFR-CONC-001` exists to stop one person's
   edit silently overwriting another's. SQLite's `CURRENT_TIMESTAMP` is
   whole-second and `NFR-CONC-002` reuses `updated_at` as the lock token, so
   **two writes to the same row inside one second shared a token** and the
   second was admitted. **Demonstrated through the real handlers before it
   was fixed** — the test failed on the unfixed tree and passes now.
   **Say the residual**: the window is now one millisecond, not zero. A
   release that fixes a concurrency defect and implies it closed the window
   entirely would be the wrong claim.
2. **A committed write now survives a power cut.** `synchronous = FULL`:
   the write-ahead log is fsynced **on every commit**, where before it was
   synced at checkpoints and recently committed transactions could be lost
   on a power cut or OS crash. **Measured, not assumed** — about 2 ms on a
   ~440 ms write path on NVMe, order randomised across ten rounds. **Say it
   is deliberately not configurable**, and why.
3. **Two secrets stopped being one careless line away from a log file.**
   The database URL is redacted at the source, so the password in a future
   PostgreSQL DSN cannot reach journald; and `Config` no longer derives
   `Debug` over the session signing key. **Say plainly that neither was
   leaking** — both were latent, and closing a hazard while it is still
   cheap is the point.
4. **Data at rest is in the threat model for the first time.** 194 lines of
   security documentation had never used the words *encrypt*, *at rest* or
   *disk* — for a product whose defining constraint is that personal
   sustainability data is visible only to its subject. The asset is named,
   **volume encryption** is named as the control, and the plain statement
   that **peisear does not encrypt its own storage** is now where an
   operator reads.
5. **`§10.36`**, a new register entry: *a check that cannot tell nothing
   wrong from nothing examined.* **And tell the honest part** — the class
   was named, written down, and then **reintroduced in this same release**
   while a flake was being fixed, and caught in review. A register entry
   whose first week produced a sixth instance says more about why it exists
   than any definition.

**What this release does not fix**, named as usual: `NFR-MNT-005`'s
reconciliation (status corrected, rule undecided);
`updated_at_authority_scan`'s single spelling, which bans
`updated_at = CURRENT_TIMESTAMP` and not the write generally; and 0.51.0's
documentation list — the two architecture documents, the deployment
manifest, the citation count, and `README.md`'s own repositioning.

**One thing the notes must not claim**: that the lost-update window is
closed. See beat 1.

## §4 — verification before the candidate commit

- **`DEC-028`**: both specifications to `0.50.0`, a baseline row each, the
  **five-pass sweep reported pass by pass including "nothing"**, and §2's
  two amendments.
- `DEC-007` **431**, three consecutive runs, **from a block extracted fresh
  out of the extracted tree's own `.github/CONTRIBUTING.md`**. **Report
  whether any run flaked** — this release fixed two flakes and named the
  class; silence is not the same as none.
- **All three browser scripts on the extracted tree**, in the job's order —
  overflow gate **120 cells**. `synchronous = FULL` changes write timing and
  two of the three measure behaviour after a write.
- **`mdbook build` from the extracted tarball**, with the three screenshots
  confirmed present in the output.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, **eight**
  private-link warnings, zero broken.
- **The database check, which changes this release.** Build the `0.49.0`
  binary **from its tag's own commit, `a58ffd1`** (`git rev-parse 0.49.0`
  returns the annotated tag object `bdfca56`; use `0.49.0^{commit}`).
  Create a database through the real forms, confirm **20**. Boot the
  candidate against that same file: **21, with one applied**. Then
  **exercise it**, do not stop at the count: update one of the old
  whole-second rows through the candidate, confirm the stored value gains a
  fraction, and confirm a resubmitted stale token is **refused**.
- `cargo publish --workspace --dry-run` from a **detached worktree of the
  candidate commit** — seven crates, every `warning:` line read.
- **Two behavioural checks**, because two user-facing behaviours changed:
  start the built artefact with a **password-bearing `DATABASE_URL`** and
  confirm the startup line shows scheme, host and path and **no
  credentials**; and confirm the lock demonstration's own scenario on the
  artefact rather than only in the test suite.

## §5 — escalate rather than deciding

- **If the count is not 431**, or the overflow gate is not 120, or the
  database does not come up at **21 with one applied**.
- **If any `DEC-007` run flakes**, even once, even if a re-run is green.
  **Especially then.**
- **If the startup line shows any part of a credential.**
- **If the five-pass sweep turns up anything** — this release rewrote two
  operator-facing documents and a requirement's status.

## §6 — exit condition

A candidate commit on `main` at **0.50.0**; both specifications amended with
the baseline rows and §2's two corrections, and the sweep reported pass by
pass; a changelog that leads with the lost update and **does not claim the
window is closed**; **431** green three times with flakes named or their
absence stated; three browser scripts clean; the book building from the
tarball with its images; a `0.49.0` database coming up at **21** and
**demonstrably protected afterwards**; a dry run of seven crates with every
warning read; and a startup line with no credential in it. Then stop.
