# REL-0.24.0 — Prepare the 0.24.0 release candidate

**Issued by**: Architect
**Date**: 2026-08-16
**Covers**: RFC 003 (INBOX-001) — reviewed and approved, both rounds
**Priority**: release-blocking
**Depends on**: nothing outstanding

---

## 1. Purpose

Cut the minor release carrying RFC 003, the inbox refinements.

**Do not tag. Do not publish.** The owner approves first. A green review is an
input to that decision, not a substitute for it.

## 2. Change scope

- `Cargo.toml` — the workspace version, one line
- `CHANGELOG.md` — a new `[0.24.0]` section
- A release tarball, produced but not published **and placed in the review
  package**, per §6

Nothing under `crates/`. This is the smallest release since 0.20.1.

## 3. Item 1 — version bump

`Cargo.toml:24` — `"0.23.0"` → `"0.24.0"`.

**Minor.** Two new routes, one route removed, and a changed JSON field on
search results. No migration.

**The removed route deserves a moment.** `POST
/settings/notifications/ack-global` is gone — the email opt-in prompt moved to
`/inbox`, so its old form no longer renders. It was POST-only and reachable only
from that form, so nothing can be holding a link to it. Say so in the changelog
rather than leaving a reader to wonder whether their bookmark broke.

## 4. Item 2 — the changelog

Add a dated `[0.24.0]` section and open a fresh `[Unreleased]` above it.

Three user-visible changes, and they are genuinely small — resist inflating
them:

- **The silence-resume banner.** Silencing all notifications was previously a
  one-way trip through a settings page with nothing to tell you it had happened;
  now the inbox says so and offers the way back.
- **The email opt-in moved to the inbox**, after a first notification instead
  of before any. `FR-NTF-007` asked for that and the settings-page prompt could
  not honour it: it was reachable before the user had received anything, which
  is the state the requirement exists to prevent.
- **Sub-issue search results name their parent.** A result reading
  `Open issue · Project / Parent title` instead of `Open issue · Project`.

**No migration**, and say so plainly — 0.23.0's entry told users a downgrade
across `0016` fails to start, so a reader who upgraded cautiously last time
deserves to know this one carries no such constraint.

`§1.7` applies under `§1.7.2`. **Run `find_violations` over the section**, as
the last three releases did.

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
| `health_explainability` | 9 |
| `inbox_refinements` | 7 |
| `issue_edit_url` | 3 |
| `optimistic_lock` | 9 |
| `search` | 9 |
| `smoke` | 11 |
| `sprint_plan` | 11 |
| `status_segment` | 2 |
| `sub_issues` | 7 |
| `today_panel` | 3 |
| `view_state` | 5 |
| `workload_privacy` | 4 |
| **integration total** | **124** |
| `peisear-web` lib | 7 |
| `peisear-i18n` | 11 |
| `peisear-notify` | 6 |
| `peisear-storage` lib | 2 |
| `peisear` facade | 1 |
| **workspace total** | **151** |

Stop and report on any difference.

## 6. Item 4 — the tarball, in the package

`git archive` at the release commit. Verify by extraction: files at the archive
root, `.git-exclude/` absent, the extracted tree builds, a representative sample
passes — and diff the archive's file list against
`git ls-tree -r --name-only <commit>`.

**Put the artefact in `evidence/`, with a package-relative checksum.** REL-0.23.0
shipped only the `.sha256`, naming a path in a session scratchpad; I could verify
it only because the file happened to still exist. That was a habit carried by
precedent rather than written down, so it is written down now.

Sample: include `inbox_refinements` and `search` — they are what changed.

## 7. Item 5 — the post-publication check

After the owner approves and publication happens:

1. `git ls-remote --tags origin` shows `0.24.0`.
2. `max_version` == `0.24.0` for each of the seven crates.
3. Any crate that did not land, named, before the release is called done.

If publication is not authorised, say so and stop.

## 8. Acceptance

1. Version bumped; changelog written, accurate, passing `find_violations`.
2. Cold-cache `DEC-007` gates green, counts as §5; three consecutive workspace
   runs.
3. Tarball produced, **in `evidence/`**, extraction verified, file list
   identical to `git ls-tree`, SHA-256 reported.
4. Nothing tagged, nothing published.

## 9. Prohibited

Do not tag, publish, or `cargo publish`. No code or test changes. No migration —
there is nothing to migrate, and one appearing would mean something is wrong.
No rewording of shipped copy. Do not weaken a guard to make a gate pass.

## 10. Required review-request format

Workflow §9.2. Include the changelog section as written, the cold-cache gate
log, the three-run transcript, the extraction and `git ls-tree` comparison, and
the SHA-256.
