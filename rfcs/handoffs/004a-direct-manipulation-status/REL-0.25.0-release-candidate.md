# REL-0.25.0 — Prepare the 0.25.0 release candidate

**Issued by**: Architect
**Date**: 2026-08-16
**Covers**: `CONF-001` (RFC 010), `QA-002` (RFC 005 §10), `STATUS-001`
(RFC 004a step 1) — all reviewed and approved
**Priority**: release-blocking
**Depends on**: nothing outstanding

---

## 1. Purpose

Cut the minor release carrying 0.25.0's three handoffs.

**Do not tag. Do not publish.** The owner approves first. A green review is an
input to that decision, not a substitute for it.

## 2. Change scope

- `Cargo.toml` — the workspace version, one line
- `CHANGELOG.md` — a new `[0.25.0]` section
- A release tarball, produced but not published, **placed in `evidence/`** with
  a package-relative checksum

Nothing under `crates/`.

## 3. Item 1 — version bump

`Cargo.toml:24` — `"0.24.0"` → `"0.25.0"`.

**Minor.** Three new `GET` routes, two new form-`POST` routes, and a behaviour
change on sprint deletion. No migration.

## 4. Item 2 — the changelog

Add a dated `[0.25.0]` section and open a fresh `[Unreleased]` above it.

**This release has an unusual shape and the entry should reflect it.** Nothing
here is a feature. Every item either removes a way for the interface to lie to
someone or gives a control to users who did not have one.

Four things to get right:

### 4.1 Lead with the confirmations, and say what was wrong

Four destructive deletes — project, issue, planned sprint, completed sprint —
previously confirmed through a JavaScript dialog attached to the submit event.
**With JavaScript unavailable that handler never ran and the delete proceeded
with no confirmation at all**, so the plain path was more dangerous than the
enhanced one. They now confirm on a server-rendered page.

Say that the five reversible confirmations — leave team, remove member, detach
project, remove capacity row, silence all — are deliberately unchanged, and why:
each is undoable through the interface, so a dialog that vanishes without
JavaScript costs nothing that cannot be undone.

### 4.2 An active sprint can no longer be deleted

A behaviour change, and the one most likely to surprise a user mid-flow. The
route accepted any sprint status; a team's running sprint could be deleted by
URL. It now refuses, and says to complete it first.

### 4.3 The status control is new, not improved

Issue detail rendered three status-shaped buttons that did nothing —
`type="button"`, `tabindex="-1"`, no handler. The issue list rendered status as
text. **Neither surface had ever had a working status control**; the board had
one and they did not. They do now, without JavaScript.

Do not describe this as making an existing control work. It did not exist.

### 4.4 What is not here

`change_status` still returns `204` with no body, so a second status change
without a page reload is not yet possible. That is step 2 and is not in this
release. A reader who tracks RFC 004a will look for it.

`§1.7` applies under `§1.7.2`. **Run `find_violations` over the section**, as the
last four releases have.

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
| `status_control` | 7 |
| `status_segment` | 2 |
| `sub_issues` | 7 |
| `today_panel` | 3 |
| `view_state` | 5 |
| `workload_privacy` | 4 |
| **integration total** | **143** |
| `peisear-web` lib | 9 |
| `peisear-i18n` | 11 |
| `peisear-notify` | 6 |
| `peisear-storage` lib | 2 |
| `peisear` facade | 1 |
| **workspace total** | **172** |

Stop and report on any difference.

## 6. Item 4 — the tarball

`git archive` at the release commit. Verify by extraction: files at the archive
root, `.git-exclude/` absent, the extracted tree builds, a representative sample
passes — and diff the archive's file list against
`git ls-tree -r --name-only <commit>`.

**The artefact goes in `evidence/`** with a package-relative checksum.

Sample: `confirmation`, `status_control`, and `smoke` — the two new targets and
the general path.

## 7. Item 5 — the post-publication check

After the owner approves and publication happens:

1. `git ls-remote --tags origin` shows `0.25.0`.
2. `max_version` == `0.25.0` for each of the seven crates.
3. Any crate that did not land, named, before the release is called done.

If publication is not authorised, say so and stop.

## 8. Acceptance

1. Version bumped; changelog written, accurate, passing `find_violations`.
2. Cold-cache `DEC-007` gates green, counts as §5; three consecutive workspace
   runs.
3. Tarball produced, in `evidence/`, extraction verified, file list identical to
   `git ls-tree`, SHA-256 reported.
4. Nothing tagged, nothing published.

## 9. Prohibited

Do not tag, publish, or `cargo publish`. No code or test changes. No migration.
No rewording of shipped copy. Do not weaken a guard to make a gate pass.

## 10. Required review-request format

Workflow §9.2. Include the changelog section as written, the cold-cache gate
log, the three-run transcript, the extraction and `git ls-tree` comparison, and
the SHA-256.
