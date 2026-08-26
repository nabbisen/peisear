# REL-0.28.0 — release candidate

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: P0 — the release
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md), all
sections
**Depends on**: `QA-013` through `QA-020`, all closed and reviewed.

**Do not tag. Do not publish.**

---

## 1. What is in it

Twenty-three commits. **This release closes RFC 005 — every section.**

**User-visible:**

1. **Secondary text got darker, in 111 places.** The muted tier's floor moved
   to `/70`. This is the change a user will actually notice, across most
   screens.
2. **Conflict announcements now interrupt.** Success and conflict were sharing
   one polite live region; they are now two regions, chosen by outcome.
3. **Three checkboxes and two buttons got bigger.** The notification
   preferences checkboxes (16 px), and the confirmation screen's Cancel and
   Delete (32 px, adjacent, one irreversible).
4. **Sprint burndowns and velocity medians are hidden below two
   contributors.** The sprint-end totals stay.

**Behavioural, invisible unless it went wrong:**

5. **`updated_at` now has one authority.** Application code wrote it on three
   tables that had no trigger. A future mutation path forgetting the clause
   would have silently accepted a conflicting write.

**Internal**: one redirect encoder replacing four; five new guards.

## 2. Version, scope, and the migration

`0.28.0`. Minor.

**This release carries a schema migration — the first since `0016` at
0.23.0.** `0017_updated_at_single_authority.sql` adds three triggers.

**Rollback is therefore not "forward-fix only"**, and the 0.23.0 entry's
finding applies unchanged: `sqlx::migrate!` does not tolerate an applied
migration absent from its embedded list and the binary migrates
unconditionally, so **a downgrade to 0.27.0 fails to start**. Recovery is a
restore from a pre-migration backup.

State that plainly. **And state the part that is easy to get wrong**: the
triggers themselves are harmless to an older binary — `0014`'s `WHEN` clause
means a trigger stays inert while an application writes the column explicitly,
which is exactly what 0.27.0 does. The obstacle is `sqlx`'s migration
bookkeeping, not the schema.

Bump `Cargo.toml`, write `CHANGELOG.md`. Those two files and `Cargo.lock`.

## 3. Gates

`cargo clean` first, then the full `DEC-007` set. Expected:

| Target | Expected |
|---|---|
| `aggregate_privacy` | 6 |
| `assignee_candidates` | 8 |
| `auth_boundary` | 16 |
| `board_keyboard` | 7 |
| `breadcrumb` | 2 |
| `calendar` | 7 |
| `calendar_surfaces` | 10 |
| `confirmation` | 14 |
| `health_explainability` | 9 |
| `inbox_refinements` | 7 |
| `issue_edit_url` | 3 |
| `optimistic_lock` | 16 |
| `search` | 9 |
| `smoke` | 12 |
| `sprint_plan` | 12 |
| `status_control` | 13 |
| `status_segment` | 2 |
| `sub_issues` | 8 |
| `today_panel` | 3 |
| `updated_at_authority` | 4 |
| `view_state` | 5 |
| `workload_privacy` | 4 |
| **integration total** | **177** |
| `peisear-web` lib | 19 |
| `peisear-i18n` | 17 |
| `peisear-notify` | 6 |
| `peisear-storage` lib | 2 |
| `peisear-core` lib | 3 |
| `peisear` facade | 1 |
| **workspace total** | **225** |

Any difference: stop and report.

Then three consecutive `cargo test --workspace` runs.

## 4. The changelog — and the sentence this release must not contain

### 4.1 The claim to refuse

**Do not write that this release makes the product accessible, or WCAG AA
compliant, or that the accessibility work is done.** Contrast is now AA.
**Touch targets are not** — 138 controls remain below 44 px by this project's
own `SPEC §33.2`, measured and recorded, with the design pass scheduled for
0.30.0.

A release that fixes one axis and reads as fixing the category is the exact
misdirection `§10.15` and `§10.16` were written to prevent. **One sentence
naming what is still outstanding** is worth more than any of the rest.

### 4.2 Contrast, honestly

The theme's own tokens were never the problem — `base-content` on `base-100`
is 17.21:1. **The 130 opacity modifiers this project applied to them were**,
and 111 sites sat below AA. Say that: the failure was ours, not inherited.

Two details worth the words. `/60` passed on white by **0.04** while failing
on the page background at 4.23:1 — which is why the fix is a floor rather than
a repair. And **three of those failures were the login and register subtitle**:
the first text a new user reads.

**Name the cost.** The muted tier went from four steps of grey to two, and
places that used the lightest tier now look like places that used a middle one.
That is a deliberate trade, not a free improvement.

### 4.3 The lock, without claiming a bug users hit

Nobody lost data. Every conflict test passed throughout. What existed was a
**class**: two authorities for one value, on the two entities the optimistic
lock most protects, so the next mutation path that forgot a clause would have
accepted a conflicting write with no error.

**Do not describe it as a fixed bug, and do not describe it as nothing.** It
was found by auditing this project's own claims — not by a failure — and that
is worth one clause, because it is the only reason it was found at all.

### 4.4 The rest

The live regions and the touch-target items are small and should read small.
The burndown suppression needs its reason — an aggregate that resolves to one
person is that person's data — and must not name the predicate in a way that
tells a reader what a missing chart implies.

The encoder consolidation is internal. It changes nothing a user sees, **and
the reason it was safe was luck**: every flash string happened to be ASCII.
Say that or leave it out; do not say "improved".

### 4.5 Vocabulary

Run `find_violations` over the `[0.28.0]` section. Report characters and hits.

## 5. The tarball

`git archive` at the release commit, into `evidence/` with a package-relative
checksum. Verify by extraction: root-level files, no `.git-exclude/`, listing
identical to `git ls-tree`, archived `Cargo.toml` reads `0.28.0`, extracted
tree builds.

**Confirm `0017_updated_at_single_authority.sql` is in the archive.** A release
whose migration is missing from the artefact starts and then behaves like
0.27.0 with no trigger — which is the defect this release exists to close,
shipped silently.

Sample: `updated_at_authority` (4), `aggregate_privacy` (6), `optimistic_lock`
(16).

## 6. Post-publication

Not run unless authorised. Tag on the remote, `max_version` for all seven
crates, any crate that did not land named before the release is called done.

## 7. Escalate rather than deciding

- Any count in §3 differing — stop.
- If `find_violations` reports a hit — stop, report the hit.
- **If the extracted tree's migration list does not include `0017`** — stop.
- If writing §4.1's sentence proves awkward because the rest of the entry
  reads as a completion, **that is the entry's problem, not the sentence's**.
  Rewrite the rest.

## 8. Required review-request format

Workflow §9.2. Quote §4.1's outstanding-work sentence in the package, verbatim,
so it can be reviewed as text rather than as a claim about the text.
