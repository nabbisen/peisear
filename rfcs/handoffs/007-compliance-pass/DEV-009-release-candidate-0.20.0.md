# DEV-009 — Prepare the 0.20.0 release candidate

**Issued by**: Architect
**Date**: 2026-08-03
**Priority**: release-blocking
**Governing RFC**: 007
**Depends on**: **DEV-005 item A must land first.** It is the last outstanding
work in the release; the candidate cannot be cut around it.

---

## 1. Purpose

Turn the completed compliance pass into a release candidate the architect can
evaluate and recommend. This is workflow Phase 7 — you produce the candidate,
I evaluate it, the owner approves or declines.

**Do not tag. Do not publish.** The owner approves the release first
(workflow Phase 8). Producing an artefact is not releasing it.

## 2. Why this is a separate handoff

DEV-005 is partially approved already. Folding release preparation into it
would reopen a reviewed unit to add unrelated work — the same objection that
rejected option 1 in the ISSUE-001 ruling. Release preparation is its own
concern with its own evidence.

## 3. Change scope

- `Cargo.toml` — the workspace version, one line
- `CHANGELOG.md` — close out the `[Unreleased]` section
- A release tarball, produced but not published

Nothing under `crates/*/src`. No code. If you find yourself editing code,
something is wrong — stop and report.

## 4. Item 1 — version bump: exactly one line

`Cargo.toml:23` — `version = "0.19.1"` → `"0.20.0"`.

**That is the whole change.** Do not go looking for more:

- All six member crates use `version.workspace = true` and inherit it.
- The five inter-crate declarations in `[workspace.dependencies]` are
  `version = "0"` — a `^0` requirement satisfied by any `0.x`. They do **not**
  need updating.

The 0.19.1 handoff bundle's release procedure says to bump "the workspace + 5
inter-crate deps". **That instruction is stale** and describes a manifest shape
this repository no longer has. I repeated it once myself before checking. Verify
against the file, not against the procedure.

0.20.0 is a minor bump, not a patch: the release changes user-visible behaviour
— the health score badge is gone, workload chips no longer show capacity, and
the board gains a status control.

## 5. Item 2 — close out the changelog

Rename `## [Unreleased]` to `## [0.20.0] - 2026-08-03` and open a fresh empty
`[Unreleased]` above it.

Then **read what is there before accepting it.** The entries were written per
handoff, at different times, by someone working inside each task. Check that,
read end to end as one release:

- Every one of the seven corrections appears — DEV-001 through DEV-008 plus the
  three one-line corrections.
- Each records **why**, not only what (`NFR-MNT-009`).
- Nothing claims something the review did not accept. In particular the single-
  member suppression was added and then withdrawn; the changelog should reflect
  where that landed, not the intermediate state.
- No prohibited vocabulary (§1.7). The changelog is user-visible.

Report anything you had to change, and why.

## 6. Item 3 — final gate run, cold cache

`cargo clean` first, then the full set:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
# per-crate; the combined --workspace test link OOMs (DEC-007)
cargo test -p peisear-core    --lib
cargo test -p peisear-auth    --lib
cargo test -p peisear-storage --lib
cargo test -p peisear-notify  -- --test-threads=1
for t in auth_boundary board_keyboard breadcrumb health_explainability \
         issue_edit_url optimistic_lock search smoke status_segment \
         sub_issues today_panel view_state workload_privacy; do
  cargo test -p peisear-web --test "$t" -- --test-threads=1
done
```

Note the test list has grown: `board_keyboard` (DEV-002) and
`workload_privacy` (DEV-003) are new. Confirm both have CI jobs — a test crate
without one silently never runs (`DEC-007`).

This run is the release's gate evidence. Capture all of it.

## 7. Item 4 — the tarball

Per `NFR-REL-002`:

- Named `peisear-0.20.0.tar.gz`.
- **Files at the archive root — no intermediate parent directory.** Extracting
  must yield `./Cargo.toml`, `./crates/`, not `./peisear-0.20.0/Cargo.toml`.
- Excludes `target/`, `data/`, `.git/`, `.git-exclude/`.

**Verify by extracting it into an empty directory and listing the top level.**
Do not assert the layout from the command you used to build it — that is the
one property most easily got wrong and most easily checked.

## 8. Item 5 — release-candidate information

Assemble, per workflow Phase 7:

| Field | Notes |
|---|---|
| Version | 0.20.0 |
| Source commit | the exact SHA the tarball was built from |
| Included changes | the seven corrections, by handoff |
| Excluded changes | what was found but deliberately not fixed — see below |
| Executed tests / results | §6 |
| Build results | §6 |
| Supported environments | note the MSRV change: 1.85 → 1.88.0 |
| Known issues | §9 below |
| Migration considerations | none expected — no migration was added; **confirm** `crates/peisear-storage/migrations/` still ends at `0015` |
| Rollback / recovery | forward-fix only; no schema change to reverse |

**Excluded changes worth naming explicitly**, because they were found during
this release and left open on purpose:

- The `project_workload` / `list_assignee_candidates` owner-only defect —
  issues cannot be assigned to team members (ISSUE-003; awaiting its RFC).
- Eight pre-existing `#[allow(clippy::too_many_arguments)]` suppressions in
  `peisear-web`.
- `workspace-layout.md`'s stale file tree.

## 9. Known limitations to record

- Drag-and-drop rollback (`FR-DM-004`) is verified by HTTP contract and code
  review, not by driving browser drag events.
- Keyboard operability is verified by the HTTP contract a form submission
  produces, not by driving real Tab/Enter input.

State both plainly. They are honest limits of the environment, not defects, and
the release recommendation will carry them as residual risk.

## 10. Acceptance criteria

1. `Cargo.toml` declares `0.20.0`; `cargo build --workspace` succeeds.
2. `CHANGELOG.md` has a dated `[0.20.0]` section and a fresh empty
   `[Unreleased]`.
3. All gates green from a cold cache, with logs.
4. Tarball exists, is correctly named, and **verified by extraction** to have no
   intermediate directory.
5. The §8 information packet is complete.
6. No code changed.

## 11. Prohibited

- **Do not tag.** **Do not publish.** Not to crates.io, not to GitHub releases.
- Do not fix anything you notice along the way. Report it. A release candidate
  is a snapshot of reviewed work; a late unreviewed fix invalidates the review.
- Do not edit the changelog to make the release sound tidier than it was. The
  suppression walk-back happened and should read as it happened.

## 12. Required review-request format

Per workflow §9.2, into `.git-exclude/review-request/`. The §8 packet can be the
body of it.

**Escalate rather than deciding** if any gate fails, if the extraction check
shows an unexpected layout, or if closing the changelog turns up an entry that
does not match what was reviewed.
