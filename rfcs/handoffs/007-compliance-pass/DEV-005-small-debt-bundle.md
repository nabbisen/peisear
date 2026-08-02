# DEV-005 — 0.20.0 small-debt bundle

**Issued by**: Architect
**Date**: 2026-08-01
**Priority**: mixed — see per-item
**Governing decision**: `DEC-032`
**Depends on**: nothing. Parallel with DEV-001/003/004.

---

## 1. Purpose

Four small items that are individually too small to earn a release slot and
have therefore never been scheduled. `DEC-032` bundles them into 0.20.0.

Two of them are documentation that would otherwise contradict the release the
moment it ships.

| Item | Source |
|---|---|
| A — `rust-toolchain.toml` absent; CI never exercises MSRV | TRK-022, `RSK-002`, `NFR-CMP-001` |
| B — one `#[ignore]`d authorisation test | TRK-023, `RSK-003` |
| C — "four crates" in README and three architecture docs | TRK-024 |
| D — `ROADMAP.md` stale | TRK-028 — **already done, do not repeat** |

Item D was completed by the architect on 2026-08-01. It is listed only so the
bundle's scope is unambiguous.

## 2. Change scope

- `rust-toolchain.toml` (new, repository root)
- `.github/workflows/test.yml`
- `crates/peisear-web/tests/auth_boundary.rs`
- `README.md`
- `docs/architecture/README.md`, `crate-boundaries.md`, `workspace-layout.md`
- `CHANGELOG.md`

## 3. Non-change scope

- No source code in `crates/*/src`. This bundle touches configuration, tests,
  and documentation only.
- Do not touch anything DEV-001/003/004 own — see their handoffs.
- Do not restructure `docs/` into mdbook layout. That is TRK-025, scheduled
  for 0.21.0.
- Do not wire `cargo audit` / `cargo deny`. That is TRK-026, also 0.21.0.

---

## 4. Item A — pin the toolchain, and actually exercise the MSRV

**Priority: P2.** **Reissued 2026-08-01** after ISSUE-004. The original text
conflated two different artefacts and is superseded by what follows.

You established that 1.85 does not build and 1.88.0 does. The owner has
ratified `1.88.0` (`DEC-044`). Proceed.

**The pinned toolchain and the MSRV are two different things**, and the original
instruction to pin `rust-toolchain.toml` "to the declared MSRV" was wrong:

| Artefact | Purpose | Value |
|---|---|---|
| `rust-toolchain.toml` | Determinism of `fmt`/`clippy` across contributors and CI | **1.97.1** |
| `rust-version` in `Cargo.toml` | The floor for building from source | **1.88.0** |

Pinning development to the MSRV would put everyone on the oldest supported
compiler, and would endanger this release directly: DEV-006 and DEV-007
produced their results at 1.97.1, and `rustfmt` output is not guaranteed
identical at 1.88. That could turn the gate red again and undo what this
release just fixed.

1. Add `rust-toolchain.toml` at the repository root pinning **1.97.1**.
2. Set `rust-version = "1.88.0"` in the workspace `Cargo.toml`.
3. Add **one** CI job running `cargo build --workspace` on **1.88.0**, so the
   MSRV claim is tested rather than asserted. Build only — no fmt, no clippy;
   those run on the pinned toolchain.
4. Leave the existing jobs on `@stable`, so upstream breakage still surfaces
   early.

**Why this job matters beyond this release.** The MSRV is set by the dependency
tree, not chosen — `leptos` 0.8 is what puts the floor at 1.88. It will drift
upward on future `cargo update`s. This job is what makes that drift visible the
day it happens, instead of three months later, which is exactly how ISSUE-004
came about. If it goes red, that is the signal to raise the declared value
deliberately — still not something to change silently.

## 5. Item B — resolve the ignored authorisation test

**Priority: P2.** `cross_user_settings_post_returns_403` in
`crates/peisear-web/tests/auth_boundary.rs` is `#[ignore]`d because no
user-scoped POST endpoint exists — settings mutations are self-scoped by
session rather than addressed by `user_id` in the path.

The baseline (§9.3) offers two dispositions: activate it against a new
endpoint, or withdraw it with recorded cause. `FR-API-006` — which would
provide such an endpoint — is unscheduled.

**Withdraw it, with cause.** Remove the test and record in `CHANGELOG.md` that
it asserted a boundary that cannot exist while settings are session-scoped, and
that it should be reinstated if `FR-API-006` ever lands.

Do **not** simply delete it silently, and do **not** leave it `#[ignore]`d — an
ignored test on a privacy boundary reads like coverage that does not exist.

Confirm before removing that no *other* test covers cross-user settings access.
If one does, say so in the review request; that changes the disposition.

## 6. Item C — correct the crate count

**Priority: P2.** The workspace has **six** members: `peisear`,
`peisear-core`, `peisear-auth`, `peisear-storage`, `peisear-notify`,
`peisear-web`. Verify against `Cargo.toml` rather than trusting this list.

"Four crates" appears in `README.md` and in three files under
`docs/architecture/`. Correct every occurrence, including prose that depends on
the number ("the four implementation crates", "alongside the existing four",
"in four documents" where it refers to crates rather than documents — check
each, they are not all the same claim).

While in `README.md`, **leave the kanban drag-and-drop description alone.** It
is accurate: the board does implement drag. What is wrong is the requirements
baseline, which records `FR-DM-001` as deferred while that surface has been
shipping. That correction is the architect's, in the baseline amendments.

## 7. Required tests

No new tests, except that item A adds a CI job. Item B removes one.

The full suite must still pass after item B's removal.

## 8. Acceptance criteria

1. `rust-toolchain.toml` exists; a CI job builds the workspace on the declared
   MSRV; that job passes.
2. No `#[ignore]`d test remains in `auth_boundary.rs`.
3. No occurrence of a four-crate claim remains in `README.md` or `docs/`.
4. `CHANGELOG.md` records item B's withdrawal *with its cause*, per
   `NFR-MNT-009`.
5. fmt, clippy `-D warnings`, and the full test suite are clean.

## 9. Prohibited shortcuts

- Do not raise the declared MSRV to make a build pass (item A step 3).
- Do not delete the ignored test without recording the cause.
- Do not "fix" the crate count by removing the sentence that mentions it —
  the architecture docs exist to explain the boundaries; state the right
  number.

## 10. Required evidence

- Changed-file list.
- Output of the new MSRV CI job, or the local `cargo +1.85 build --workspace`
  equivalent.
- Full test-suite output after item B's removal.
- `grep -rn "four crates\|four implementation" README.md docs/` returning
  nothing.

## 11. Required review-request format

Per workflow §9.2, into `.git-exclude/review-request/`.

**Escalate rather than deciding** if the MSRV build fails, or if another test
already covers cross-user settings access.
