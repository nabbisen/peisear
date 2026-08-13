# RFC 0001: Sprint planning page

**Status**: Implemented (0.22.0) — minus the capacity hint, withdrawn on
privacy grounds

> **Implemented, with one component deliberately not built.** The page, both
> move routes, the filters and the three read-only shapes shipped in 0.22.0
> via `PLAN-001`. RFC 001's capacity hint did not: it sums each participating
> member's capacity on a page that names the participants, so at one
> participant the sum is that person's capacity. Withdrawn rather than shipped
> and regretted; the design that replaces it is not yet written.
>
> **This RFC needed eight corrections between acceptance and delivery**, five
> at handoff time and three after review, and two of those were defects rather
> than staleness — a §D1-adjacent test-plan status code that contradicted the
> shipped convention, and a positive privacy claim that was never true. It was
> written 2026-05-04 and delivered 2026-08-13. An accepted RFC is a design
> decision with a shelf life, and fifteen weeks exceeded this one's.
**Target**: 0.20.0 (Phase C PR2)
**Related spec sections**: §17 (Sprint Plan), §9 (Team / Sprint), §38.1 task 3
**Last updated**: 2026-08-13 — five corrections, plus three more from PLAN-001's review

> **Amendment note (2026-08-13).** This RFC was written fifteen weeks before
> the release that implements it, and predates the i18n architecture, the
> compliance pass, RFC 009, and `viewer`-role enforcement. Five things in it
> were wrong by the time `PLAN-001` was written and are corrected in place
> below, each marked *Corrected 2026-08-13*:
>
> 1. Non-members get **404**, not 403 (test plan, §Authorization).
> 2. `viewer` may read the page and may not move issues (§Must-have 1).
> 3. **The capacity hint is deferred** — it is reversible to an individual's
> capacity, which `NFR-PRIV-001` makes self-only (§Capacity hint formula,
> §Security).
> 4. All copy goes through `peisear-i18n` (§UI).
> 5. The assignee filter only became meaningful with RFC 009 (§Must-have 3).
>
> Points 1 and 3 are defects rather than staleness. Point 3 in particular was
> a positive claim — that the hint cannot be inverted — which was never true.

## Summary

Bulk-assign issues to a sprint from a single page rather than
one-at-a-time from each issue's detail. Adds the URL
`/teams/{slug}/sprints/{sprint_id}/plan` with a two-column
layout — a filterable backlog of unassigned issues on the
left, the sprint's currently-committed items on the right —
and button-driven moves between the two columns. Drag and
drop is intentionally deferred to Phase D-4.

## Background

Until 0.19.0, putting issues into a sprint required opening
each issue and using its sprint dropdown. For a planning
session with 20 backlog candidates, that's 20 round-trips. The
spec called this out in §17.1 ("v1.0 では issue detail から
1 件ずつ割り当てる必要があり、 摩擦が大きかった"). This RFC
gives the planner a single screen, enough information per row
to decide, and enough scaffolding (capacity hint, totals) to
notice when they're over-committing the team.

The spec's §38.1 task 3 already names what we're shipping;
this RFC fills in the detail the spec deliberately leaves
open ("backlog filter scope", "capacity hint formula", etc.)
and pins the decisions that diverge from the spec because of
our prior decisions in Phase C PR1.

## Requirements

### Must-haves

1. URL `/teams/{slug}/sprints/{sprint_id}/plan` accessible to
   any team member (admin or otherwise) with read access to
   the team.
2. Two-column layout: backlog on the left, sprint items on
   the right.
3. Backlog filter: by project, by priority, by assignee. The
   filter state is reflected in the URL so it bookmarks and
   shares cleanly (matching the pattern in `view_states`).
4. Move actions implemented as form-POSTs (no DnD in this
   PR): one button per row "→ Sprint" / "← Backlog". Each
   POST handles one issue.
5. Sprint-side header shows `committed: N pts` with the sum
   of effort across in-sprint items. ~~`capacity hint: ~M` is
   computed from team capacity rows~~ — *corrected 2026-08-13:
   **the capacity hint is deferred**, see §Capacity hint formula.
   The committed total ships; it sums effort on issues already
   visible on the same page.*
6. Sub-issues do not appear in either column. Backlog and
   sprint items are top-level only. This matches Phase C PR1
   §8.5 ("sub-issue は parent に追従").
7. The sprint's lifecycle (planned / active / completed) is
   shown but not changeable from this page. The "Start
   sprint" button on the sprint detail page is still the
   single moment of state transition (§17.3).
8. Active and completed sprints render the page read-only
   (no move buttons). A planner editing a completed sprint
   would mean re-writing history.

### Nice-to-haves (not blocking)

- Per-assignee mini-rollup on the sprint side: "alice 12 pts /
  bob 8 pts / unassigned 4 pts". Useful for spotting that one
  person has half the sprint. Defer if implementation time is
  tight; capacity hint covers the worst case.
- Inline effort editing on a backlog row. The user can also
  click through to edit; deferring saves us a small form.

### Explicitly out

- Drag & drop. Phase D-4.
- Multi-select bulk move ("select 5 rows, hit → Sprint"). Same
  reason — the keyboard / DnD work that makes multi-select
  pleasant lives in Phase D.
- Search box inside the backlog. The filter widgets (project
  / priority / assignee) cover the common cases; full-text
  search in this surface adds a query path that's awkward to
  share with the global search infrastructure.

## Design

### Routes

```
GET  /teams/{slug}/sprints/{sprint_id}/plan
       ?project=<id>&priority=<level>&assignee=<id>
POST /teams/{slug}/sprints/{sprint_id}/plan/add
       Body: issue_id=<id>
POST /teams/{slug}/sprints/{sprint_id}/plan/remove
       Body: issue_id=<id>
```

GET renders the page. Both POSTs redirect (303) back to the
GET with the same filter query so the planner stays in
context after each move. Form submissions don't carry
`client_updated_at` because `sprint_issues` is a join table
and Phase B PR1 explicitly opted out of optimistic-lock for
joins; last-write-wins is acceptable here (concurrent
planners moving the same issue both end up with the same
table state).

### Authorization

- Team membership check on entry, same as `/teams/{slug}/...`
  routes — via the existing `resolve_team_membership`, which
  returns **404** for non-members (*corrected 2026-08-13*: the
  test plan below said 403. External design §9 uses 404 for team
  membership so a non-member cannot confirm a team exists; 403 is
  for personal data, where the URL already implies existence).
- *Corrected 2026-08-13*: **`viewer` may read and may not move.**
  `TeamRole::can_write()` (`Admin | Member`) gates both POSTs; a
  viewer gets 403 there — they are a member, so existence is not
  what is concealed. A viewer's page renders in the same
  read-only shape completed sprints already use.
- The page is **not** admin-only. Per V2.1 §11 the team
  members can collectively plan; admin role is for adding /
  removing team members, not for owning planning decisions.
- Read-only for completed sprints regardless of role.

### Storage layer changes (`peisear-storage::sprints`)

New helpers:

```rust
/// Top-level open issues across the team's projects that are
/// not in any active sprint. Optional filter facets.
pub async fn backlog_for_team(
    pool: &Pool,
    team_id: &str,
    filter: BacklogFilter,
) -> StorageResult<Vec<BacklogRow>>;

#[derive(Default)]
pub struct BacklogFilter {
    pub project_id: Option<String>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<String>,
}

pub struct BacklogRow {
    pub issue: Issue,
    pub project_name: String,
}
```

`issues_in_sprint` already exists from Phase C PR1 and already
filters to top-level only. No change needed.

### Capacity hint formula — **deferred, 2026-08-13**

**Do not implement this.** The formula below is reversible to an individual's
capacity, and `NFR-PRIV-001` makes capacity self-only (P0).

The hint sums `effective_for_user(today)` across members **who have at least
one open issue in the team's projects** — and a viewer of this page can see
exactly who those members are, because assignees are on the rows. One
participating member: the hint *is* their capacity. Two: subtract your own.

This is the first genuine `NFR-PRIV-007` case in the product — an aggregate
reversible to an individual — in a product built for teams of about five. It
needs an owner decision and a design that survives a two-person team. Deferred
to its own RFC rather than settled inside a feature handoff.

The original formula is retained below as the record of what was proposed.



The hint is the sum of `effective_for_user(today)` across
team members who have at least one open issue assigned to
them in the team's projects. The intent: "this is the rough
budget the people who are likely to take the work have."

We deliberately do not include people who have *no* open
issues in the team — they may exist on the membership list
but be on leave, in another team's planning, etc. Over-
counting here makes the hint less useful, not more.

The formula is exposed as `team_capacity_hint(pool, team_id,
date)` returning `Option<i64>` (None if no capacity rows
exist for any participating member).

### UI

`SprintPlanPage` component, new file
`crates/peisear-web/src/components/sprint_plan.rs` (the
existing `components/sprints.rs` is already long; a new
file is cleaner than wedging a fourth page-level component
in).

Page layout (semantic HTML, not table-based):

```html
<main class="grid grid-cols-1 md:grid-cols-2 gap-6">
  <section aria-labelledby="backlog-heading">
    <h2 id="backlog-heading">Backlog</h2>
    <!-- filter widgets, links updating ?project= ?priority= ?assignee= -->
    <ul class="divide-y">
      <li>… title … 5pt … <form>→ Sprint</form></li>
      …
    </ul>
  </section>
  <section aria-labelledby="sprint-heading">
    <h2 id="sprint-heading">Sprint Items</h2>
    <p>committed: <strong>24 pts</strong> · capacity hint: ~30</p>
    <ul class="divide-y">
      <li>… title … 8pt … <form>← Backlog</form></li>
      …
    </ul>
  </section>
</main>
```

Each `<li>` has an `aria-label` carrying the same triple
(title, points, current column) that the spec calls out in
§17.4 for screen-reader users.

### Filter widget pattern

Re-use the `view_states` pattern from Phase A: filter
controls update URL query parameters via a tiny form. The
backlog filter is **not** persisted to user view-state on
the server side — sprint planning is an episodic activity,
and saving "the project=X filter from last May's planning
session" would surface stale defaults the next time the
team plans.

### Read-only mode for completed sprints

*Corrected 2026-08-13, from PLAN-001's review.* There are **three** shapes,
not two. An earlier version of this section described only "read-only", and
`PLAN-001`'s handoff §2.2 compounded it by calling a viewer's page "the same
read-only shape completed sprints already use" — which would take the backlog
away from a viewer who is reading a live plan.

| Sprint status | Role | Backlog column | Move buttons |
|---|---|---|---|
| Planned | `admin` / `member` | shown | shown |
| Planned | `viewer` | shown | hidden |
| Active | any | shown | hidden |
| Completed | any | **hidden** | hidden |

A viewer is reading a plan in progress and needs to see what is not yet in it.
Nobody is planning a completed sprint — re-opening one to add issues is not a
flow we support, which is why the backlog goes away there and only there.

Two flags, not one: `can_move` (`role.can_write() && status == Planned`)
governs the forms; the backlog section is conditional on
`status != Completed`.

## Test plan

New test crate `crates/peisear-web/tests/sprint_plan.rs`
with at least:

1. `plan_page_renders_two_columns_for_planned_sprint` —
   page returns 200, has both `aria-labelledby` headings,
   has at least one `<form>` per column row.
2. `add_to_sprint_via_button_succeeds` — POST `/plan/add`
   with a backlog issue id returns 303, follow-up GET shows
   the issue under "Sprint Items" rather than "Backlog".
3. `remove_from_sprint_via_button_succeeds` — symmetric.
4. `sub_issues_do_not_appear_in_backlog` — create a
   top-level + a sub-issue, GET the page, assert sub-issue
   title is absent from both columns.
5. `completed_sprint_plan_is_read_only` — GET on a completed
   sprint returns 200, the **backlog section is absent**, and the
   sprint section contains no `/plan/add` or `/plan/remove` form
   action. *Corrected 2026-08-13*: the original wording ("no
   `<form>` tags inside either column section") was wrong twice — it
   reads as though both sections render on a completed sprint, and is
   vacuously satisfied when one is absent, so it never contradicted
   the prose it appeared to contradict. A blanket `<form>` assertion
   would also fail on any page, since the shell's logout form is a
   POST form. Assert the action targets, not the tag.
6. `non_team_member_gets_404` — auth boundary check
   (*corrected 2026-08-13*; was `_403`).
7. `committed_total_matches_sum_of_effort` — render with
   2 issues of effort 5 and 8, confirm **"13 pt"** appears.
   *Corrected 2026-08-13*: this said "13 pts". The product's unit is
   invariant singular (`PointsValue`, `PointsUnitSuffix`, `/me`'s
   load chip). A literal taken from this RFC's May HTML sketch became
   a test that then dictated shipped copy against the product's own
   convention — the test was the defect, not the copy.

Add a CI job `test-peisear-web-sprint-plan` mirroring the
existing per-test-crate jobs.

## Security & privacy considerations

- §11.5 boundary: the page exposes effort and assignee
  *aggregates* (committed pts, member rollups). The
  individual-level data (capacity, burnout) stays at
  `/today`.
- ~~The capacity *hint* is a sum-only number — no per-person
  breakdown — so a member can't infer another's capacity from
  the hint.~~ **False, corrected 2026-08-13.** Sum-only does not
  mean irreversible: the hint's own formula names its
  contributors, and the page shows who they are. At one
  contributor the hint is that person's capacity; at two, a
  member subtracts their own. The hint is deferred; see
  §Capacity hint formula.
- The per-assignee rollup **is** permitted: it shows each
  member's volume of committed work, which `NFR-PRIV-002`
  names as "workload distribution", and `ISSUE-003` ruled
  that holds regardless of how many members a surface lists.
  It carries no capacity value.
- §21.4 optimistic lock: the join-table mutations are not
  lock-checked, consistent with Phase B PR1 PR1's
  `assign_issue` decision. Document this explicitly in the
  handler comment so the next reviewer doesn't add it
  reflexively.
- The form-POST endpoints must verify team membership. The
  `Path` extractor gives the slug; `resolve_team_membership`
  (existing helper) returns the member's role and 403s on
  miss.

## Out of scope

- Phase D-4 drag & drop. The page's HTML structure
  intentionally puts moveable items in `<li>` siblings so
  D-4 can wire up DnD listeners later without restructuring.
- Sprint goal editing. Lives on the existing sprint detail
  page (`/teams/{slug}/sprints/{id}`).
- Cross-team backlog (planning a sprint with issues from
  another team's project). The spec doesn't ask for it and
  it complicates the auth boundary; revisit if it ever
  surfaces as a real need.

## Open questions

1. **Backlog scope across projects**: the team's projects
   include both team-scoped projects (those with
   `team_id = team.id`) and personal projects belonging to
   members. We restrict the backlog to **team-scoped
   projects only**. Personal projects are out of scope for
   team sprint planning. *Default-if-no-decision: team-
   scoped only.*
2. **Capacity hint when no capacity rows exist**: render
   "capacity hint: —" or omit the line entirely. *Default-
   if-no-decision: omit, with a tooltip on the committed
   total explaining "no capacity rows configured; ask
   members to set one in /settings".*
3. **Sprint Items default ordering**: by `assigned_at`
   ascending (the order they were added) or by priority
   descending? §17.2 sketch shows assigned_at order; the
   implementer can pick either, just be consistent.
   *Default: assigned_at ascending.*

## References

- Spec §17 — Sprint Plan
- Spec §38.1 task 3 — Phase C task list
- CHANGELOG entry for 0.19.0 — sub-issue follow-parent rule
  (informs §17 sub-issue handling here)
- RFC 0004 — direct manipulation, where the DnD version of
  this page is detailed
