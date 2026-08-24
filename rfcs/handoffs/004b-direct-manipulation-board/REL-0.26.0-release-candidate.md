# REL-0.26.0 — Prepare the 0.26.0 release candidate

**Issued by**: Architect
**Date**: 2026-08-25
**Covers**: `STATUS-002` (RFC 004a step 2), `BOARD-001` (RFC 004b / D-2) — both
reviewed and approved, both rounds
**Priority**: release-blocking
**Depends on**: nothing outstanding

---

## 1. Purpose

Cut the minor release carrying 0.26.0's two handoffs.

**Do not tag. Do not publish.** The owner approves first. A green review is an
input to that decision, not a substitute for it.

## 2. Change scope

- `Cargo.toml` — the workspace version, one line
- `CHANGELOG.md` — a new `[0.26.0]` section
- A release tarball, produced but not published, **placed in `evidence/`** with
  a package-relative checksum

Nothing under `crates/` or `static/`.

## 3. Item 1 — version bump

`Cargo.toml:24` — `"0.25.0"` → `"0.26.0"`.

**Minor.** One response-shape change (`change_status` returns `200` with a body
where it returned `204`), one new client-side behaviour on three surfaces, no
migration, no route added or removed.

## 4. Item 2 — the changelog

Add a dated `[0.26.0]` section and open a fresh `[Unreleased]` above it.

**The honest framing of this release is that a guard existed for Rust and never
for JavaScript.** Both handoffs are about the same weakness in different places,
and the entry reads better as one story than as two features.

Four things to get right:

### 4.1 The board's error messages had never been checked

Three user-visible English sentences lived inside `static/board.js` — authored
as literals, for as long as this project has had a vocabulary guard.
`prose_scan` covers Rust under `components/` and `handlers/`; `static/*.js` was
outside it by construction. **Not excluded on purpose — unexamined.** They now
live in the message table, unchanged, and the guard has seen them.

Say that they were **not** reworded and that the vocabulary check passed. Copy
that had never been checked turning out to be fine is the true and slightly
anticlimactic outcome, and it is the one to report.

### 4.2 `static/*.js` is now scanned

New guard, `search.js` the one named exclusion. That exclusion has been RFC
006's stated position since 0.21.0; it is now enforced rather than assumed.

Name its blind spot: single-word copy is below the two-word threshold and is not
caught. A release note that describes a new check without its limit invites the
belief that it is total.

### 4.3 Status changes no longer reload the page

Issue detail, issue list and the board update in place, with a 5-second undo.
Any failure **before** the change lands falls back to the plain form submit that
0.25.0 introduced — so a user with JavaScript disabled or broken keeps the
working path, and the enhancement can never be the only way to act.

### 4.4 What a reader should not conclude

The JavaScript is **not** covered by the test suite. The harness drives HTTP; it
does not execute scripts. The in-place update, the toast, undo, and the
fallback are verified by reading and by hand.

This is the first release where that is true of new behaviour, and it is worth
one plain sentence rather than silence. Do not write it as a caveat that
undercuts the feature — write it as what it is.

`§1.7` applies under `§1.7.2`. **Run `find_violations` over the section**, as
the last five releases have.

## 5. Item 3 — final gate run, cold cache

`cargo clean`, the full `DEC-007` set, then three consecutive
`cargo test --workspace` runs.

| Target | Tests |
|---|---|
| `assignee_candidates` | 8 |
| `auth_boundary` | 11 |
| `board_keyboard` | 6 |
| `breadcrumb` | 2 |
| `calendar` | 7 |
| `calendar_surfaces` | 10 |
| `confirmation` | 11 |
| `health_explainability` | 9 |
| `inbox_refinements` | 7 |
| `issue_edit_url` | 3 |
| `optimistic_lock` | 9 |
| `search` | 9 |
| `smoke` | 11 |
| `sprint_plan` | 12 |
| `status_control` | 11 |
| `status_segment` | 2 |
| `sub_issues` | 7 |
| `today_panel` | 3 |
| `view_state` | 5 |
| `workload_privacy` | 4 |
| **integration total** | **147** |
| `peisear-web` lib | 11 |
| `peisear-i18n` | 11 |
| `peisear-notify` | 6 |
| `peisear-storage` lib | 2 |
| `peisear` facade | 1 |
| **workspace total** | **178** |

Stop and report on any difference.

## 6. Item 4 — the tarball

`git archive` at the release commit. Verify by extraction: files at the archive
root, `.git-exclude/` absent, the extracted tree builds, a representative sample
passes — and diff the archive's file list against
`git ls-tree -r --name-only <commit>`.

**The artefact goes in `evidence/`** with a package-relative checksum.

Sample: `status_control`, `board_keyboard`, and `smoke` — the surface that
changed most, the one that must not have changed, and the general path.

**Confirm `static/dm.js` and `static/board.js` are both in the archive.** They
are tracked, so they should be; this release is the first where a shipped
behaviour lives in a file the test suite never executes, and an artefact missing
one of them would run silently degraded rather than fail.

## 7. Item 5 — the post-publication check

After the owner approves and publication happens:

1. `git ls-remote --tags origin` shows `0.26.0`.
2. `max_version` == `0.26.0` for each of the seven crates.
3. Any crate that did not land, named, before the release is called done.

If publication is not authorised, say so and stop.

## 8. Acceptance

1. Version bumped; changelog written, accurate, passing `find_violations`.
2. Cold-cache `DEC-007` gates green, counts as §5; three consecutive workspace
   runs.
3. Tarball produced, in `evidence/`, extraction verified, file list identical to
   `git ls-tree`, SHA-256 reported, both `.js` files confirmed present.
4. Nothing tagged, nothing published.

## 9. Prohibited

Do not tag, publish, or `cargo publish`. No code, test, or `static/` changes. No
migration. No rewording of shipped copy — including the three sentences
`BOARD-001` just moved, which are byte-exact by design.

## 10. Required review-request format

Workflow §9.2. Include the changelog section as written, the cold-cache gate
log, the three-run transcript, the extraction and `git ls-tree` comparison, the
SHA-256, and confirmation that both `.js` files are in the archive.
