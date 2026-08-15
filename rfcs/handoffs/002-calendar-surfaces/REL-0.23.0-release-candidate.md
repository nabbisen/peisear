# REL-0.23.0 — Prepare the 0.23.0 release candidate

**Issued by**: Architect
**Date**: 2026-08-14
**Priority**: release-blocking
**Covers**: RFC 002 (CAL-001, CAL-002) — reviewed and approved
**Depends on**: nothing outstanding

---

## 1. Purpose

Cut the minor release carrying RFC 002, the calendar surfaces.

**Do not tag. Do not publish.** The owner approves first (workflow Phase 8).
A green review is an input to that decision, not a substitute for it — as
REL-0.22.0's own review request said, and said correctly.

## 2. Change scope

- `Cargo.toml` — the workspace version, one line
- `CHANGELOG.md` — a new `[0.23.0]` section
- A release tarball, produced but not published

Nothing under `crates/*/src`, `crates/*/tests`, or `crates/*/migrations`. Both
handoffs are reviewed and their corrections landed.

## 3. Item 1 — version bump

`Cargo.toml:24` — `"0.22.0"` → `"0.23.0"`.

**Minor.** Two new routes, two new nullable columns, two new form fields. No
breaking change to any existing shape.

## 4. Item 2 — the migration, which is new for this procedure

**0.23.0 is the first release since the release cycle was formalised to carry a
schema migration.** `0016_issue_planned_dates.sql` adds two nullable columns,
two triggers and a partial index. Every prior release candidate could say
"Migration considerations: none" and mean it.

Three things to establish and report, rather than assert:

1. **Forward, on a populated database.** Take a database created at `0.22.0`
   with real rows, run the `0.23.0` binary against it, confirm `0016` applies
   and existing rows read back with both columns `NULL`. CAL-001 could only
   test this within a single `migrate!` pass; here you have two versions and
   can do it properly. This is the check CAL-001's review said not to build a
   harness for — because at release time it is available for free.

2. **Downgrade.** Run the **0.22.0** binary against a database that has had
   `0016` applied. Report what happens. I do not know whether `sqlx::migrate!`
   tolerates an applied migration absent from its embedded list, and neither
   the changelog nor the rollback line should guess. If it errors, that is not
   a defect — it is a fact users need in the "Rollback / recovery" row, and it
   is the first release where that row cannot say "forward-fix only; no schema
   change to reverse."

3. **The trigger fires.** Confirm on the migrated database that an update
   setting `planned_end_at` before `planned_start_at` is rejected with the
   expected message. The tests cover this; do it once against a real migrated
   file so the release note's claim rests on the artefact.

## 5. Item 3 — the changelog

Add a dated `[0.23.0]` section and open a fresh `[Unreleased]` above it.

**Lead with the calendar itself** — two new surfaces, personal and project, on
a time axis, with plan dates settable from the issue form. That is the feature.

**Then two things a user needs and would otherwise discover the hard way:**

- **Times are shown in UTC.** They are not converted to the reader's zone, and
  a plan date typed as 09:00 will read as 09:00 only for a reader on UTC.
  Deferred deliberately to the locale work, not overlooked. The page says so;
  the changelog should too, because it is the single most likely surprise in
  this release.
- **Downgrade behaviour**, per §4.2, in the migration/rollback rows.

**Say what the calendar does not do, and mean it.** No team axis — permanently,
because a tool that lays out members' time against each other becomes an
oversight tool, and that is the line this product is drawn around. No fill rate,
no free hours, no comparison to last week. A changelog is the one place that
reaches a user who is wondering why their calendar has no productivity view,
and the answer is that it is deliberate.

`§1.7` applies under `§1.7.2`'s use-versus-mention rule. **Run
`find_violations` over the section** rather than reading it — one small binary
against `peisear-i18n`, as the last two releases established.

Do **not** list the `planned_for_user` sub-issue fix as a user-facing
correction. It never shipped: both halves were written and fixed inside this
release. Same handling as 0.22.0's cross-team removal boundary — describe it as
part of how the feature was built, or omit it, but do not present it as a fix to
something a user had.

## 6. Item 4 — final gate run, cold cache

`cargo clean`, then the full `DEC-007` set, then three consecutive
`cargo test --workspace` runs.

Expected counts — stop and report on any difference:

| Target | Tests |
|---|---|
| `assignee_candidates` | 8 |
| `auth_boundary` | 11 |
| `board_keyboard` | 6 |
| `breadcrumb` | 2 |
| `calendar` | 7 |
| `calendar_surfaces` | 10 |
| `health_explainability` | 9 |
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
| **integration total** | **117** |
| `peisear-web` lib | 7 |
| `peisear-i18n` | 11 |
| `peisear-notify` | 6 |
| `peisear-storage` lib | 2 |
| `peisear` facade | 1 |
| **workspace total** | **144** |

## 7. Item 5 — the tarball

`git archive` at the release commit. Verify by extraction: files at the archive
root, `.git-exclude/` absent, the extracted tree builds, a representative sample
passes — and **diff the archive's file list against
`git ls-tree -r --name-only <commit>`**, which is the check that actually proves
the archive is the tracked tree.

The sample should include `calendar` and `calendar_surfaces` this time; they are
what changed.

Report the SHA-256.

## 8. Item 6 — the post-publication check

As REL-0.22.0 established, and for the same reason: two releases were once
recorded here as complete while crates.io stayed at 0.19.1.

**After** the owner approves and publication happens:

1. `git ls-remote --tags origin` shows `0.23.0`.
2. `max_version` == `0.23.0` for each of the seven crates.
3. Any crate that did not land, named, before the release is called done.

If publication is not authorised, say so and stop — the check is on the
publication, not on a guess that it happened.

## 9. Acceptance

1. Version bumped; changelog written, accurate, and passing `find_violations`.
2. §4's three migration checks done and **reported**, including whatever
   downgrade actually does.
3. Cold-cache `DEC-007` gates green, counts as §6; three consecutive workspace
   runs.
4. Tarball produced; extraction verified; file list identical to `git ls-tree`;
   SHA-256 reported.
5. Nothing tagged, nothing published.

## 10. Prohibited

Do not tag, publish, or `cargo publish`. No code, test, or migration changes —
**a migration especially: `0016` has been reviewed, and editing an applied
migration is how a schema and its history stop agreeing.** No rewording of
shipped copy. Do not weaken a guard to make a gate pass.

## 11. Required review-request format

Workflow §9.2. Include the changelog section as written, the cold-cache gate
log, the three-run transcript, the migration checks from §4 with their actual
results, the extraction and `git ls-tree` comparison, and the SHA-256.
