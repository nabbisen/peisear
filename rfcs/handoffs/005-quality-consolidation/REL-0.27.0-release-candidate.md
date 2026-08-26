# REL-0.27.0 — release candidate

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P0 — the release
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §1-§3,
§12-§14
**Depends on**: `QA-003` through `QA-010`, all closed and reviewed.

**Do not tag. Do not publish.** Produce the candidate and the review-request
package; the owner approves, the architect executes.

---

## 1. What is in it

Twenty-four commits since `0.26.0`. **Two of them change what a user sees**, and
the changelog must not imply otherwise.

**User-visible (both `QA-006`, RFC 005 §2):**

1. **Project delete and issue delete now take an optimistic lock.** A
   confirmation screen that has gone stale answers `409` instead of deleting
   something that changed while it was on screen. Sprint delete and capacity
   delete already did this; two of four did not.
2. **The issue delete confirmation names its cascade.** *"This issue has 2
   sub-issues. Deleting it deletes all of them too. This cannot be undone."*
   `issues.parent_issue_id` has been `ON DELETE CASCADE` since `0015`; the
   screen whose purpose is to name what will be deleted did not name them.

**Everything else is internal**: five guard modules and twenty-six tests. It is
still worth a changelog section — see §4.

**No migration.** Nothing under `crates/*/migrations` changed.

## 2. Version and scope

`0.27.0`. Minor: no route added or removed, no migration, but the two delete
routes changed their request contract (§4.2).

Bump `Cargo.toml` and write `CHANGELOG.md`. **Those two files and `Cargo.lock`,
nothing else.** If you find yourself editing anything under `crates/`, stop and
report.

## 3. Gates

`cargo clean` first, then the full `DEC-007` set — **including the two lines
`QA-004` and `QA-005` added**, `cargo test -p peisear` and
`cargo test -p peisear-web --lib`. Expected:

| Target | Expected |
|---|---|
| `assignee_candidates` | 8 |
| `auth_boundary` | 16 |
| `board_keyboard` | 6 |
| `breadcrumb` | 2 |
| `calendar` | 7 |
| `calendar_surfaces` | 10 |
| `confirmation` | 13 |
| `health_explainability` | 9 |
| `inbox_refinements` | 7 |
| `issue_edit_url` | 3 |
| `optimistic_lock` | 16 |
| `search` | 9 |
| `smoke` | 12 |
| `sprint_plan` | 12 |
| `status_control` | 12 |
| `status_segment` | 2 |
| `sub_issues` | 8 |
| `today_panel` | 3 |
| `view_state` | 5 |
| `workload_privacy` | 4 |
| **integration total** | **164** |
| `peisear-web` lib | 14 |
| `peisear-i18n` | 17 |
| `peisear-notify` | 6 |
| `peisear-storage` lib | 2 |
| `peisear-core` lib | **3** |
| `peisear` facade | 1 |
| **workspace total** | **207** |

`peisear-core`'s row was **0 in every previous release**; `QA-010` put a guard
there. If any number differs, stop and report before proceeding — a wrong
expected count here would manufacture an escalation about work that was
correct.

Then three consecutive `cargo test --workspace` runs.

## 4. The changelog

### 4.1 The two user-visible changes, not inflated

Lead with them. Two entries, plainly stated, with the *reason* rather than only
the mechanics — the reason is what a self-hoster acts on.

For the lock: the confirmation screen introduced a gap between reading and
acting that a single `POST` never had. For the cascade: the count is the
consequence, and a confirmation that names the issue but not its children names
the smaller half.

### 4.2 The compatibility note — this is the one that can cost someone

**`POST /projects/{id}/delete` and `POST /projects/{id}/issues/{iid}/delete`
now expect a `client_updated_at` form field.**

- With a form body and no such field: `400` with a readable message.
- With no body at all: `415` from the extractor, before the handler runs.

Anyone who scripted a delete against these routes will find it stops working.
That is a small audience and possibly an empty one, but this project named a
removed route and why in `0.24.0` for exactly this reason, and a **changed
request contract** deserves the same treatment as a removed one. State the
field, state both failure modes, and say the value is rendered as a hidden
field on the confirmation page — which tells a scripted caller where to get it.

Do not apologise for it and do not call it a fix. It is a deliberate narrowing.

### 4.3 The internal work, told truthfully

The temptation is "hardened the guards" or "improved test coverage". Both are
true and neither says anything. What actually happened:

- **A P0 vocabulary guard had never seen five live strings.** `MessageKey::all()`
  is a hand-maintained list, five `aria-label` variants were absent from it, and
  the check that exists to hold all copy to §1.7 had therefore never read them.
  **They turned out to be fine.** Say that — copy that had never been checked
  turning out to be correct is the true and anticlimactic outcome, and this
  project has now reported it twice.
- **Four structural guards never ran in CI.** `prose_scan`, `static_js_scan`,
  `test_harness_scan` and `dec_007_scan` all live in a target no CI job invoked.
  They ran in the release gate, so no release shipped without them; a pull
  request that reintroduced any of those defect classes would have passed.
- **The enumerations those guards walk are now checked**, in `peisear-i18n` and
  `peisear-core`.

**Do not claim the guards found bugs in the product.** They found gaps in the
guards. The distinction is the whole point, and blurring it would sell the
release on a defect that did not exist.

### 4.4 What the release does not do

RFC 005's accessibility axes — colour contrast, keyboard navigation, mobile —
are **not** in this release. If the changelog implies a quality pass happened,
a reader will assume those were part of it. One sentence naming them as still
outstanding is worth more than any of §4.3.

### 4.5 Vocabulary

Run `find_violations` over the `[0.27.0]` section as a throwaway test and report
the character count and hit count. Eyeballing it does not count.

## 5. The tarball

`git archive` at the release commit, into `evidence/` with a package-relative
checksum. Verify by extraction:

- Files at the archive root, no intermediate directory; no `.git-exclude/`.
- File list identical to `git ls-tree -r --name-only <commit>`.
- Archived `Cargo.toml` reads `0.27.0`.
- The extracted tree builds clean.
- **Confirm `.github/workflows/test.yml` is in the archive.** New this release:
  two `dec_007` guards now read that file at test time, so an artefact missing
  it fails in a way that looks like a broken guard rather than a missing file.
- Sample: `optimistic_lock` (16) and `confirmation` (13) — what changed — plus
  `peisear-core --lib` (3), which had no tests at all before this release.

## 6. Post-publication

**Not run unless publication is authorised.** If it is not, say so and stop.
Otherwise: the tag on the remote, `max_version` for all seven crates
(`DEC-047`), and any crate that did not land named before the release is called
done.

## 7. Escalate rather than deciding

- Any count in §3 differing from the table — stop.
- If `find_violations` reports a hit on the changelog — stop, and report the
  hit rather than rewording around it.
- If the two delete routes turn out to accept a bodyless `POST` after all,
  §4.2 is wrong and I want to know before it ships.

## 8. Required review-request format

Workflow §9.2. §4.2 as prose in the package, not only in the changelog — I want
to read the compatibility note twice, once as a changelog entry and once as your
description of who it affects.
