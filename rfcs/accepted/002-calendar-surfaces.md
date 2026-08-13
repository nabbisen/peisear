# RFC 0002: Calendar surfaces

**Status**: Accepted
**Target**: **0.23.0** (was 0.21.0 — the release slipped while the compliance
pass and RFC 006 took precedence)
**Related spec sections**: §16 (Calendar), §10.2-10.4 (calendar
concept), §11 (privacy boundary), §38.1 task 2
**Last updated**: 2026-08-13 — five corrections, see the amendment note

> **Amendment note (2026-08-13).** Written 2026-05-04 and dispatched fifteen
> weeks later, after the i18n architecture, the compliance pass, RFC 009 and
> COPY-001. Five things were wrong or under-specified by dispatch time and are
> corrected in place below, each marked *Corrected 2026-08-13*:
>
> 1. The privacy footnote has a **different normative text** in external design
> §10.3, which wins (must-have 9).
> 2. All copy goes through `peisear-i18n` (must-have 9, §UI).
> 3. The migration's `RAISE` text is **user-facing copy** and names database
> columns (§Migration).
> 4. The optimistic-lock claim holds only if the columns join the existing
> `UPDATE` (§Security & privacy).
> 5. Project-axis blocks must not name the assignee, or must-have 9's own
> claim stops being true (must-have 6).
>
> Points 1, 3 and 5 are defects rather than staleness. This is the second
> accepted RFC to need substantial correction between acceptance and delivery;
> see RFC 001's status note. **RFC 003 is the same vintage.**

## Summary

Two new surfaces that lay issues out on a time axis:

- `/today/calendar` — personal axis. The viewer's assigned
  issues across all projects.
- `/projects/{id}/calendar` — project axis. Every issue in the
  project, with an overlay band for the active sprint.

There is **no team axis** by design (§10.2; teams aggregating
members' time would push the tool toward oversight, which
§2.5 rules out). Read-only display in this PR; drag-and-drop
to reschedule lives in Phase D-3.

A schema migration adds `planned_start_at` and
`planned_end_at` to `issues` so that the calendar has
something to lay out.

## Background

So far peisear has timestamps for "when work happens" only as
a side-effect: `created_at`, `updated_at`, sprint dates, the
`issue_events` log. None of these answer "when do I plan to
do this?" The user has either kept that knowledge in a
separate calendar or not had it at all.

The spec §10.2 asked us to add it back as a *peisear* concept,
specifically rejecting team-axis aggregation. The
non-aggregation isn't an oversight — it's the spec's main
design point: "the tool gives you a time-shape view of your
own work and your project's work, not a way to see whether
the team is putting in the hours."

The spec §38.1 task 2 lists `start_date`, `due_date`,
`planned_start_at`, `planned_end_at` as four columns to add.
This RFC narrows that to two. See open question 1.

## Requirements

### Must-haves

1. New columns on `issues`: `planned_start_at`,
   `planned_end_at`, both nullable `DATETIME` (UTC). Migration
   `0016_issue_planned_dates.sql`.
2. New routes:
   - `GET /today/calendar` — personal axis, requires login.
   - `GET /projects/{id}/calendar` — project axis, requires
     project read access (existing
     `projects::find_accessible`).
3. Three view modes — day, week, month — selectable via a
   `?view=day|week|month` URL parameter (week as default).
   The selection is in the URL so it bookmarks; per-user
   preference is **not** persisted (matches the rationale in
   RFC 0001's filter section: episodic activity).
4. Period navigation: `?date=YYYY-MM-DD` URL parameter
   anchoring the visible range. Prev/next buttons advance or
   retreat by the current view's span and update the URL.
   Today's date is the default.
5. Issue blocks: rendered for any issue with a non-NULL
   `planned_start_at`. If `planned_end_at` is NULL, the
   block is treated as a half-hour anchor at
   `planned_start_at` (the user added a "when I'll start"
   without committing to a duration; we shouldn't drop the
   issue from the calendar).
6. Personal-axis: only issues where
   `assignee_id = current_user.id`. Project-axis: every
   top-level issue in the project (sub-issues follow parent;
   they don't appear separately on the calendar).

   *Corrected 2026-08-13*: **project-axis blocks must not name
   the assignee.** A time-axis view of a project's planned work,
   with each block labelled by person, is a per-person schedule
   view — which is what must-have 9's footer tells the user this
   page is not, and what §10.2 rules out by refusing a team
   axis. A block carries its title, its span, and (personal
   axis) a project colour. The assignee is one click away on the
   issue itself, where it has always been.
7. Sprint band overlay on **project axis only**. Renders at
   the top of the calendar grid for any active sprint whose
   span overlaps the visible window. Personal axis does not
   render sprint bands (the personal axis can span multiple
   teams' sprints; surfacing them all turns the page into a
   sprint dashboard).
8. Click an issue block → navigate to the issue detail page.
9. Project-axis privacy footer (literal text, on the page).
   *Corrected 2026-08-13*: external design §10.3 carries a
   different "do not paraphrase" version of this same string,
   and **it wins** — it is the complete one, and its third
   sentence is the only place the guarantee is stated to the
   user:

   > Calendar note: this view shows planned issue work for this
   > project. Personal schedules are not aggregated here. Each
   > member's individual calendar is private to that person.

   Personal-axis footer is just "Private to you". Both per
   §16.4. Both are `MessageKey`s (RFC 006, which this RFC
   predates); the project one gets a byte-identity test, as
   `FR-TEAM-005`'s footnote has.
10. **No efficiency metrics** (§16.6). The page does not
    display fill rate, comparison-to-last-week, or "free
    hours". This is non-negotiable; it is the difference
    between peisear's calendar and a load-tracking
    calendar.

### Nice-to-haves

- Crowding chip: when a day has more than N (= 4) overlapping
  issue blocks, show a small `Watch` chip near the date
  cell. Quiet signal that something is over-scheduled.
  Default-decision: ship in PR3 — it's small.

### Explicitly out

- DnD reschedule. Phase D-3.
- Adding an issue from a calendar cell. Phase D-3 (the new
  issue would inherit the cell's date as
  `planned_start_at` / `planned_end_at`).
- Recurring issues / events. peisear isn't a calendar app;
  we're adding a calendar *view* on top of an issue tracker.
- iCal export. Possible later as a read-only `.ics`
  endpoint, but out of scope here.
- Team axis. Permanently out.

## Background — schema decision recap

The spec lists four columns: `start_date`, `due_date`,
`planned_start_at`, `planned_end_at`. We collapse to two
because the other two would be redundant:

- `start_date` would mirror `planned_start_at::date` for
  most uses.
- `due_date` already lives in the user's mental model as "a
  hard ceiling on `planned_end_at`". Carrying both invites
  a bug class ("which one is the real deadline?").

If a real distinction emerges (e.g. `due_date` becomes a
sprint-style firm commitment vs. `planned_end_at` as a soft
estimate), add it then. Until then, two columns is enough.

## Design

### Migration `0016_issue_planned_dates.sql`

```sql
ALTER TABLE issues
    ADD COLUMN planned_start_at TIMESTAMP;

ALTER TABLE issues
    ADD COLUMN planned_end_at TIMESTAMP;

-- Sanity constraint: end after start when both are set.
-- Trigger rather than CHECK because SQLite CHECK can't
-- reference NULL semantics cleanly.
CREATE TRIGGER issues_planned_range_check_insert
BEFORE INSERT ON issues
FOR EACH ROW
WHEN NEW.planned_start_at IS NOT NULL
 AND NEW.planned_end_at IS NOT NULL
 AND NEW.planned_end_at < NEW.planned_start_at
BEGIN
    SELECT RAISE(ABORT, 'planned_end_at must be on or after planned_start_at');
END;

-- *Corrected 2026-08-13*: the RAISE text above is USER-FACING
-- COPY and must not ship as written. Per DEC-011,
-- translate_trigger_error matches the trigger's string as a
-- needle and returns the MessageKey carrying identical text --
-- so this sentence IS what the user reads, and a migration
-- cannot be edited afterwards. It also names database columns,
-- the exact defect COPY-001 corrected on the capacity form
-- ("period_start must be on or before period_end" -> "The From
-- date must be on or before the To date."). Choose the wording
-- against the form's own labels before writing the migration.
-- See CAL-001 §2.3.

CREATE TRIGGER issues_planned_range_check_update
BEFORE UPDATE OF planned_start_at, planned_end_at ON issues
FOR EACH ROW
WHEN NEW.planned_start_at IS NOT NULL
 AND NEW.planned_end_at IS NOT NULL
 AND NEW.planned_end_at < NEW.planned_start_at
BEGIN
    SELECT RAISE(ABORT, 'planned_end_at must be on or after planned_start_at');
END;

-- Index supporting calendar window queries.
CREATE INDEX idx_issues_planned_window
    ON issues(planned_start_at)
    WHERE planned_start_at IS NOT NULL;
```

The partial index keeps the ordering small — issues without
plan dates make up most of the table for any normal project,
and the calendar query needs only the planned ones.

### Issue struct (peisear-core)

```rust
pub struct Issue {
    // ... existing fields ...
    pub planned_start_at: Option<DateTime<Utc>>,
    pub planned_end_at: Option<DateTime<Utc>>,
}
```

`IssueRow` and `into_issue` carry the columns through, same
pattern as `parent_issue_id` in 0.19.0.

The existing `issues::update` handler should accept these in
its form payload; otherwise the user can't set them. Adding
them to the edit form is part of this PR. The form fields use
`<input type="datetime-local">` which posts `YYYY-MM-DDTHH:MM`;
the handler parses and converts to UTC at the boundary.

### Storage queries

```rust
/// Issues planned to overlap the [from, to] window for the
/// given assignee. Used by /today/calendar.
pub async fn planned_for_user(
    pool: &Pool,
    user_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> StorageResult<Vec<Issue>>;

/// Issues planned to overlap the window in the given project.
/// Top-level only — sub-issues inherit their parent's
/// position on the calendar by way of the parent appearing
/// in the result.
pub async fn planned_for_project(
    pool: &Pool,
    project_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> StorageResult<Vec<Issue>>;
```

The "overlap" predicate:

```sql
WHERE planned_start_at IS NOT NULL
  AND (
    -- block ends in or after the window start
    (planned_end_at IS NOT NULL AND planned_end_at >= ?from)
    OR (planned_end_at IS NULL AND planned_start_at >= ?from)
  )
  AND planned_start_at <= ?to
```

Why the OR: NULL `planned_end_at` is treated as "anchor at
start, half-hour width" (must-have #5). Including those
points in the overlap predicate keeps the rendered view
consistent with the data.

### Sprint band query

```rust
pub async fn active_sprints_overlapping(
    pool: &Pool,
    project_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> StorageResult<Vec<Sprint>>;
```

Project axis only. Sprint dates are full-day (`starts_on`,
`ends_on` are `DATE`); we widen them to start-of-day UTC and
end-of-day UTC for the overlap check.

### UI

`CalendarPage` component, new file
`crates/peisear-web/src/components/calendar.rs`. Re-uses
`AppShell` and the existing breadcrumb component.

The component takes a `mode: CalendarMode` (Personal /
Project) and a `view: CalendarView` (Day / Week / Month).
Layout per the spec sketch §16.2.

Cells are an HTML table for month view, a flex column for
week view, a flex column with hour-marks for day view.
Issue blocks are absolute-positioned inside cells using
percentages so the layout reflows on screen size without
JavaScript.

Mobile: month view degrades to a chronological list of cells
(spec §16.5). Use a single CSS query
`@media (max-width: 640px)` to switch between grid and list.

### Accessibility (§16.5)

- Each calendar cell has an `aria-label` like "May 3, 2 issues
  scheduled". When the cell holds issues, the label includes
  count rather than spelling them out — issue titles are read
  in their `<a>` text inside.
- Issue blocks are `<a>` elements (links), not `<div>`s with
  a click handler — so they're keyboard-focusable by default,
  and the link text is the issue title.
- Cell focus outline uses the existing `focus-visible`
  Tailwind class.
- Arrow-key navigation between cells: deferred to Phase E
  (the keyboard-nav consolidation pass). For PR3, tab order
  follows DOM order and the user can navigate with tab.

### Render note: "no efficiency"

Add a top-of-component comment that bluntly restates §16.6.
This is a recurring temptation when a calendar view exists,
and a maintainer six months from now is the right audience
for the comment:

```rust
// peisear-feature-spec-v2.1 §16.6 — this calendar
// deliberately renders no efficiency metrics. No fill rate,
// no "free hours", no comparison to last week. These look
// helpful and act as pressure. If you find yourself adding
// one, the answer is no; if it survives review and you ship
// it anyway, you owe a CHANGELOG entry explaining the
// override.
```

## Test plan

New test crate `crates/peisear-web/tests/calendar.rs`:

1. `migration_0016_adds_planned_columns` — query
   `pragma_table_info('issues')` and assert both columns
   exist with the right types.
2. `personal_calendar_renders_only_my_issues` — create
   issues for two users, GET `/today/calendar` as user A,
   confirm only A's titles in the response.
3. `project_calendar_renders_all_top_level_issues` —
   parent + sub-issue + sibling top-level; project calendar
   shows parent + sibling, not the sub-issue.
4. `project_calendar_overlays_active_sprint_band` —
   project with an active sprint whose dates overlap the
   default week view; assert the sprint name appears in the
   band region (use a distinguishing aria-label).
5. `personal_calendar_does_not_show_sprint_band` —
   negative of #4.
6. `view_param_switches_between_day_week_month` — three
   GETs with `?view=day`, `?view=week`, `?view=month`;
   each returns 200 and contains a distinguishing marker
   (e.g. day view's hour ruler).
7. `date_param_navigates_window` —
   `?date=2026-06-15&view=week` returns a page whose week-
   range covers June 15.
8. `null_planned_end_at_renders_as_half_hour_anchor` —
   issue with start but no end appears in the rendered
   range.
9. `unauthorised_project_calendar_returns_403` —
   project-axis surface uses `find_accessible`, so a
   non-member gets 403.
10. `calendar_does_not_render_efficiency_metrics` — guard
    against regressions: scan the response for the strings
    "fill rate", "free hours", "% busy" and assert all are
    absent.

Add CI job `test-peisear-web-calendar`.

Storage-side test: extend
`crates/peisear-storage/src/issues.rs` (or a new test file)
with one unit test that exercises the trigger:

```rust
// Inserting an issue with end_at < start_at must fail.
let err = issues::insert_with_planned(...).await.unwrap_err();
assert!(matches!(err, StorageError::Validation(_)));
```

## Security & privacy considerations

- §11.5: personal axis is self-only. Test #2 enforces this.
  No new auth code needed; the route uses `AuthUser` and
  filters by `user.id`.
- §11.5 (project): project-axis uses `find_accessible`,
  which already enforces team membership. Test #9
  re-confirms.
- §16.6 efficiency-metrics absence: directly enforced by
  test #10.
- §21.4 optimistic lock: editing
  `planned_start_at`/`planned_end_at` happens via the
  existing issue update form, so the existing lock check
  covers it. No new path. *Corrected 2026-08-13*:
  **conditionally.** `issues` has no `updated_at` trigger —
  `DEC-013`'s machinery covers `sprints`, `teams`,
  `team_memberships` and `user_capacities`, while
  `issues::update` sets `updated_at` in its own `SET` clause.
  The two columns must join that statement. Given their own
  `UPDATE`, the row's `updated_at` never moves and concurrent
  plan-date edits silently overwrite each other —
  `NFR-CONC-004` violated with no error and no symptom.
- The privacy footer text (must-have #9) is verbatim from
  the spec; do not paraphrase. The wording is part of the
  contract with the user.

## Out of scope

- DnD on cells. Phase D-3.
- ICS export. Possible future, separate RFC.
- Time-zone awareness in display. We render in UTC for now;
  user-local rendering needs an i18n discussion that fits
  better with Phase E §34 (Locale).
- Default view per user. The URL parameter is the truth; if
  someone wants their default to be `month`, they bookmark
  `/today/calendar?view=month`.

## Open questions

1. **Drop `start_date` / `due_date` from spec**: this RFC
   ships only `planned_start_at` / `planned_end_at`. If a
   reviewer wants the four-column shape from the spec, raise
   it before this RFC is accepted; otherwise the
   implementation lands with two columns. *Default-if-no-
   decision: two columns.*
2. **Crowding threshold**: 4 overlapping blocks per day was
   pulled from a guess. Could be 5, could be parametric.
   *Default-if-no-decision: 4, with a const named
   `CROWDING_WATCH_THRESHOLD` so it's easy to tune.*
3. **Project-axis colour-coding**: §16.5 calls for
   project-coloured blocks on the personal axis (which
   spans projects). Do we also vary on the project axis to
   distinguish, say, sprint-attached vs. backlog? *Default-
   if-no-decision: single colour on project axis, multi-
   colour on personal axis.*

## References

- Spec §10.2-10.4 — calendar concept
- Spec §16 — calendar screen
- Spec §38.1 task 2 — Phase C tasks
- RFC 0001 — sprint planning page (sprint band overlap
  semantics referenced from there)
- RFC 0004 — direct manipulation, where calendar DnD lands
