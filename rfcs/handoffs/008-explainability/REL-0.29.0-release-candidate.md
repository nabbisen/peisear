# REL-0.29.0 — release candidate

**Issued by**: Architect
**Date**: 2026-08-27
**Priority**: P0 — the release
**Governing RFC**: [008](../../done/008-explainability.md), and
[011](../../accepted/011-browser-verification.md) step 1b
**Depends on**: `HLT-001`, `HLT-002`, `JS-001`, `JS-002` — all closed.

**Do not tag. Do not publish.**

---

## 1. What is in it

Twenty-one commits. **RFC 008 ships in full**, and `§10.4` — open since
0.19.1 — closes.

**User-visible:**

1. **Every health indicator now shows how it is calculated.** A disclosure per
   indicator, giving the Good and Watch boundaries and the window.
2. **Five of the six link to what they are counting.** A new route,
   `/projects/{id}/health/{indicator}/basis`, renders exactly the issues behind
   that indicator's number.
3. **Both sprint charts gained a table and a plain-language summary**, and the
   completed-work chart's accessible name now describes its data rather than
   its type.

**Internal**: a guard pinning the shape `dm.js`'s fallback boundary is made of;
two doc-comment notes in `en.rs`.

**No migration.** `0017` remains the most recent; rollback is forward-fix only.

## 2. Version and scope

`0.29.0`. Minor — **one route added**, none removed, no schema change.

That route is the first HTML route added since 0.25.0's confirmation screens.
It is `GET`-only, it renders issues the viewer can already see, and it inherits
the project's access check — worth one line in the changelog because a new
route is the kind of thing a self-hoster with a reverse proxy notices.

Bump `Cargo.toml`, write `CHANGELOG.md`. Those two and `Cargo.lock`.

## 3. Gates

`cargo clean` first, then the full `DEC-007` set. Expected:

| Target | Expected | | Target | Expected |
|---|---|---|---|---|
| `aggregate_privacy` | 6 | | `optimistic_lock` | 16 |
| `assignee_candidates` | 8 | | `search` | 9 |
| `auth_boundary` | 16 | | `smoke` | 12 |
| `basis_route` | 5 | | `sprint_plan` | 12 |
| `board_keyboard` | 7 | | `status_control` | 13 |
| `breadcrumb` | 2 | | `status_segment` | 2 |
| `calendar` | 7 | | `sub_issues` | 8 |
| `calendar_surfaces` | 10 | | `today_panel` | 3 |
| `chart_equivalence` | 5 | | `updated_at_authority` | 4 |
| `confirmation` | 14 | | `view_state` | 5 |
| `health_explainability` | 9 | | `workload_privacy` | 4 |
| `inbox_refinements` | 7 | | | |
| `issue_edit_url` | 3 | | **integration total** | **187** |

| Crate | Expected |
|---|---|
| `peisear-web` lib | 24 |
| `peisear-i18n` | 17 |
| `peisear-notify` | 6 |
| `peisear-storage` lib | 2 |
| `peisear-core` lib | 3 |
| `peisear` facade | 1 |
| **workspace total** | **240** |

Any difference: stop and report. Then three consecutive
`cargo test --workspace` runs.

## 4. The changelog

### 4.1 The claim to refuse, again — and it is a different one this release

**Do not write that the product is now explainable, or that explainability is
done.**

`§10.4` closes **partially**. `FR-HLT-007` has three limbs and ships two:
basis and calculation. **History is deferred** — an indicator's history is a
time series, and for a project with one active contributor it is that person's
history, which 0.28.0 suppressed on the sprint screen for exactly that reason.

**And Definition of Done item 3 moves this release** — from *"Partially met"*
to *"Met, with one limb outstanding."* That is a real milestone and it is the
first of the five conditions to move outright since 0.19.1. **State the
qualification in the same sentence as the milestone**, not in a note below it.

### 4.2 The indicator with no link, explained rather than omitted

Five indicators link to their basis. **WIP compliance does not**, and a reader
who notices will ask why.

The answer is good: its basis is *which assignees are over their limit*, and a
WIP limit is personal data visible only to its subject. The indicator says
*"{count} active assignees are over their WIP limit"* — a count, deliberately
not names — and a basis route would have to name them.

**Say it.** A missing affordance with no explanation reads as an oversight;
this one is a decision, and the decision is the interesting part.

### 4.3 The tables mirror their charts, and why that is not a footnote

The burndown's table is hidden whenever the burndown is, and the completed-work
table's median row is hidden whenever the median line is.

**0.28.0's entry already gave the reason** — an aggregate that resolves to one
person is that person's data — so this entry can refer to it rather than
restate it. **Do not explain the predicate again**, and do not describe the
suppression in a way that lets a reader infer it from a missing table.

### 4.4 The internal guard, at its real size

A guard now pins the structure `dm.js`'s fallback boundary depends on. **It
does not test the boundary.** `§10.15` stays open, with a review date of
0.32.0.

Two sentences at most, and neither may imply the JavaScript became tested. This
release's own inventory found the opposite: the rule that matters most cannot
be moved into testable code, which is why its *shape* is pinned instead.

### 4.5 Vocabulary

Run `find_violations` over the `[0.29.0]` section; report characters and hits.
**Read `en.rs`'s module note before drafting** — it now names both vocabulary
rules and the fixture case, and two drafts in this cycle were lost to not
having done that.

## 5. The tarball

`git archive` at the release commit, into `evidence/` with a package-relative
checksum. Verify by extraction: root-level files, no `.git-exclude/`, listing
identical to `git ls-tree`, archived `Cargo.toml` reads `0.29.0`, extracted
tree builds.

Sample: `basis_route` (5), `chart_equivalence` (5), `peisear-web --lib` (24) —
the last because it now carries eight scan modules and is where this release's
internal work lives.

## 6. Post-publication

Not run unless authorised. Tag on the remote, `max_version` for all seven
crates, any crate that did not land named before the release is called done.

## 7. Escalate rather than deciding

- Any count in §3 differing — stop.
- If `find_violations` reports a hit — stop, report the hit rather than
  rewording around it.
- **If §4.1's qualification cannot be written into the same sentence as the
  milestone without reading as a hedge**, say so — that is worth a
  conversation, not a compromise.

## 8. Required review-request format

Workflow §9.2. Quote §4.1's milestone sentence verbatim in the package, so it
can be reviewed as text rather than as a claim about text.
