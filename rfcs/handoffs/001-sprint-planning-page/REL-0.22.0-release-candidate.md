# REL-0.22.0 — Prepare the 0.22.0 release candidate

**Issued by**: Architect
**Date**: 2026-08-13
**Priority**: release-blocking
**Covers**: RFC 001 (PLAN-001), RFC 009 (TEAM-001), RFC 005 §9 (QA-001),
RFC 006 follow-up (COPY-001) — all reviewed and approved
**Depends on**: nothing outstanding

---

## 1. Purpose

Cut the minor release carrying 0.22.0's four pieces of work.

**Do not tag. Do not publish.** The owner approves first (workflow Phase 8).
Producing an artefact is not releasing it.

## 2. Change scope

- `Cargo.toml` — the workspace version, one line
- `CHANGELOG.md` — a new `[0.22.0]` section
- A release tarball, produced but not published

**Nothing under `crates/*/src` or `crates/*/tests`.** Every correction from
all four handoffs has landed and been re-reviewed. If something looks wrong
while you are here, report it — a release candidate is not where code changes.

## 3. Item 1 — version bump

`Cargo.toml:24` — `version = "0.21.0"` → `"0.22.0"`. Member crates inherit via
`version.workspace = true`.

**Minor, not patch.** New user-facing routes (`/teams/{slug}/sprints/{id}/plan`
and its two POSTs), a behaviour change visible to every team (assignee
candidates), and `StorageError`-adjacent additions. `DEC-047` settled that all
seven crates publish, so the number is read by people outside this repository.

## 4. Item 2 — the changelog

Add a dated `[0.22.0]` section and open a fresh `[Unreleased]` above it. Write
it for someone deciding whether to upgrade.

**Lead with the assignment defect.** In a team-owned project only the owner
could be assigned an issue — every other member was rejected as an invalid
assignee. The product's central feature is per-person sustainability signals,
and those were therefore permanently empty for everyone except project owners.
It failed by showing nothing rather than by erroring, which is why it survived
two releases, an external design document, a requirements baseline and a
compliance pass. That is the entry a user needs first.

Then the planning page — the actual new feature, and the friction fix RFC 001
was written for: one screen instead of one round trip per issue.

**Two things to state plainly rather than bury:**

- **The cross-team removal fix.** The plan page's remove route takes an issue
  id, and `sprints::remove_issue` deletes by issue id with no sprint scoping.
  Any authenticated member of any team could have removed an issue from another
  team's sprint. It never shipped — it was found and closed inside the same
  handoff that created the route — so describe it as a hardened boundary rather
  than as a fixed vulnerability, and do not overstate it in either direction.
- **The capacity hint is not in this release, and that is deliberate.** RFC 001
  specified one; it sums each participating member's capacity and the page
  names the participants, so it is reversible to an individual's capacity,
  which `NFR-PRIV-001` makes self-only. Deferred pending a design that survives
  a two-person team. A reader who knows RFC 001 will look for it.

Also: the test-harness fix and `CONTRIBUTING.md` gaining `DEC-007`'s procedure.
Contributor-facing, not user-facing, but it is the first time that procedure
has existed outside an internal file.

`§1.7` applies to the changelog as it does to everything else, under `§1.7.2`'s
use-versus-mention rule: naming a prohibited term to describe a defect is
mention and is permitted; describing the user's work in those terms is not.
Check with `find_violations` rather than by eye — that is one small binary
against `peisear-i18n`, and the 0.21.0 entry needed it.

## 5. Item 3 — final gate run, cold cache

`cargo clean` first, then the full `DEC-007` set: per-crate tests, every
`peisear-web` integration target individually, fmt, workspace clippy.

**Then the repeated run**, per `CONTRIBUTING.md` as QA-001 wrote it:
`cargo test --workspace` three times, all passing. This is the first release
candidate to include it.

Expected counts — stop and report if any differ:

| Target | Tests |
|---|---|
| `assignee_candidates` | 8 |
| `auth_boundary` | 11 |
| `board_keyboard` | 6 |
| `breadcrumb` | 2 |
| `health_explainability` | 9 |
| `issue_edit_url` | 3 |
| `optimistic_lock` | 8 |
| `search` | 9 |
| `smoke` | 11 |
| `sprint_plan` | 11 |
| `status_segment` | 2 |
| `sub_issues` | 7 |
| `today_panel` | 3 |
| `view_state` | 5 |
| `workload_privacy` | 4 |
| **integration total** | **99** |
| `peisear-web` lib | 7 |
| `peisear-i18n` | 6 + 4 |
| `peisear-notify` | 3 + 3 |
| `peisear-storage` lib | 2 |

A count that moved during a version bump means something else moved too.

## 6. Item 4 — the tarball

`git archive` at the release commit, as REL-0.21.0 established — it excludes
everything untracked by construction rather than by an exclude list.

**Verify by extraction, not from the build command.** Extract into an empty
directory; confirm the tree builds and a representative test sample passes
there; confirm no `.git-exclude/` content is present. Then **diff the archive's
file list against `git ls-tree -r --name-only <commit>`** — I added that check
by hand at 0.21.0 and it is the one that actually proves the archive is the
tracked tree. Make it part of the procedure.

Report the SHA-256.

## 7. Item 5 — the post-publication check, new this release

Every release handoff so far has ended at "produce the artefact, do not
publish". What happens after the owner approves was never written down, so it
was never checked — and crates.io sat at **0.19.1** through 0.20.0 and 0.20.1
while both were tagged, pushed, and recorded here as complete. Registry
consumers stayed on a version carrying five P0/P1 violations those releases
existed to correct (`DEC-047`).

So this handoff ends differently. **After** the owner approves and publication
happens, verify and report:

1. `git ls-remote --tags origin` shows `0.22.0`.
2. For each of the seven crates, the registry's `max_version` is `0.22.0`:
   `curl -s https://crates.io/api/v1/crates/<name> | jq -r .crate.max_version`
3. Any crate that did not land, named, before anyone calls the release done.

If publication is not authorised, say so and stop there — the check is on the
publication, not on your guessing whether it happened.

## 8. Acceptance

1. Version bumped; changelog written, accurate about scope, and passing
   `find_violations`.
2. Cold-cache `DEC-007` gates green, counts exactly as §5.
3. `cargo test --workspace` three consecutive passes.
4. Tarball produced; extraction verified; file list identical to `git ls-tree`;
   `.git-exclude/` absent; SHA-256 reported.
5. Nothing tagged, nothing published.
6. §7's verification steps stated as pending, ready to run on approval.

## 9. Prohibited

Do not tag. Do not publish. Do not `cargo publish` — `DEC-047` settles *that*
the crates publish, not *when*, and the owner approves each release. No code
changes. No rewording of shipped copy. Do not weaken a guard to make a gate
pass; if a gate fails, the release waits.

## 10. Required review-request format

Workflow §9.2. Include the changelog section as written, the cold-cache gate
log, the three-run transcript, the extraction and `git ls-tree` comparison, and
the SHA-256.
