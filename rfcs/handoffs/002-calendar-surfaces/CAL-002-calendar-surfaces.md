# CAL-002 — The two calendar surfaces

**Issued by**: Architect
**Date**: 2026-08-14
**Priority**: P1 — completes 0.23.0
**Governing RFC**: [002](../../done/002-calendar-surfaces.md)
**Depends on**: CAL-001 (landed, both rounds)

---

## 1. Scope

Everything RFC 002 specifies that CAL-001 did not build: the two routes, the
three view modes, period navigation, the sprint band, the crowding chip, and
the mobile degradation.

CAL-001 gives you `planned_for_user`, `planned_for_project`, both footnote
keys, and the two `Issue` fields. `active_sprints_overlapping` does **not**
exist yet — it is yours, and it is the only new storage function here.

## 2. Five reconciliations

RFC 002's §Design and §Test plan were written 2026-05-04 and describe a repo
that no longer exists in a few specific ways. Where this handoff and the RFC
disagree, **this handoff wins**. A sixth disagreement is a finding — report it,
the way CAL-001 reported that one of my instructions was unsatisfiable.

### 2.1 `tests/calendar.rs` is taken, and RFC 002's test 1 is already done

CAL-001 created `crates/peisear-web/tests/calendar.rs` with seven
storage-level tests, including `migration_0016_adds_planned_columns_and_
existing_rows_are_null` — which is RFC 002's test-plan item 1.

**Do not re-add it.** Put the route-level tests in a new target,
`crates/peisear-web/tests/calendar_surfaces.rs`, with its own CI job and its own
line in `CONTRIBUTING.md`. Storage-shaped and route-shaped tests in one file
read as one thing and are two.

### 2.2 The cell label is one key, and counts are not inflected

RFC 002 §Accessibility asks for `aria-label` like *"May 3, 2 issues
scheduled"*. That is a composed sentence from typed data, so it is **one
`MessageKey`** taking the date and the count — §D6 rule 7, no `format!` at the
call site.

**Do not add pluralisation.** This project renders counts uninflected:
`WorkloadTitle` is `"{display_name} — {in_flight_issues} in-flight issues"`,
which reads "1 in-flight issues", and `PointsValue` is `"{points} pt"` for
every value. An `if count == 1` branch here would be the first inflection logic
in the codebase, and inflection is a locale problem RFC 006 deliberately does
not solve — one ad-hoc case sets a precedent the architecture is not ready to
honour. Match `WorkloadTitle`.

The date inside that label is rendered, not interpolated as a pre-formatted
string. Decide where the formatting lives and say so.

### 2.3 The crowding chip shows a state, never a number

RFC 002 offers it as a nice-to-have: a `Watch` chip when a day holds more than
`CROWDING_WATCH_THRESHOLD` (= 4) overlapping blocks.

Two binding constraints, because this is the single place in the release where
§16.6 is easiest to breach by accident:

- **The chip carries a state and no quantity.** No count, no ratio, no "6 of 4",
  no percentage. A chip that displays how full a day is *is* the fill rate that
  §16.6 prohibits; the difference between a signal and a metric is whether it
  invites comparison.
- **`Watch` is the ceiling.** `NFR-LANG-002` applies here as everywhere:
  nothing above `Watch`, and the state word comes from the message table via
  `DisplayHealthState::to_i18n_label()`, not a new literal.

If the chip seems to need a number to be useful, that is a finding about the
chip, not a reason for the number. Report it and ship without the chip — it is
a nice-to-have and RFC 002 says so.

### 2.4 Project-axis blocks must not name the assignee, and a test says so

RFC 002 must-have 6, as corrected: a block carries its title, its span, and
(personal axis) a project colour. **Not the assignee.**

A time-axis view of a project's planned work labelled by person is a per-person
schedule view — which is what the page's own footer tells the user it is not,
and what §10.2 rules out by refusing a team axis. The footer is a normative
string with a byte-identity test; the claim it makes has to survive the
implementation.

Test 7 asserts it directly: two users' issues on one project calendar, neither
display name in the response body.

### 2.5 The page says its times are UTC

CAL-001 recorded the limitation in `Issue::planned_start_at`'s doc comment.
CAL-002 is where a user first *sees* it: they typed 09:00 into a
`datetime-local` input and this page may show them something else.

**Render a short note that times are shown in UTC**, as a `MessageKey`, on both
axes. Time-zone awareness stays out of scope (§34, Phase E) — but a page
showing unlabelled times that are not the reader's is misleading in a way one
sentence fixes.

If you think the note is unnecessary because your own machine is on UTC, that
is the reason it is required.

## 3. What stands unchanged

Both routes and their `?view=` / `?date=` parameters, week as default, no
per-user persistence. The half-open block treatment (`planned_end_at IS NULL` →
anchor at start). Top-level issues only. Sprint band on the project axis only,
and never on the personal axis. Blocks are `<a>` elements, not click-handled
`<div>`s. Percentage-positioned blocks so layout reflows without JavaScript.
Month view degrades to a chronological list at `max-width: 640px`. Arrow-key
navigation stays deferred to Phase E.

`CROWDING_WATCH_THRESHOLD = 4`, named as a const (open question 2's default).
Single colour on project axis, multi-colour on personal (open question 3's
default).

**Keep RFC 002's "no efficiency" comment** at the top of the component,
verbatim. It is aimed at a maintainer six months out and this handoff is not
that audience.

## 4. `active_sprints_overlapping`

The one new storage function. Project axis only.

`sprints.starts_on` / `ends_on` are `DATE`; widen to start-of-day and
end-of-day UTC for the overlap check, as RFC 002 says. Only sprints whose
status is active — a planned or completed sprint's band on a calendar would
assert something about time that is not true.

## 5. Tests

New target `crates/peisear-web/tests/calendar_surfaces.rs`.

| # | Check |
|---|---|
| 1 | `/today/calendar` renders only the viewer's assigned issues — two users' issues, one absent |
| 2 | `/projects/{id}/calendar` requires project read access; a non-member gets what `find_accessible` already gives |
| 3 | `?view=day\|week\|month` each render; an unknown value falls back to week rather than erroring |
| 4 | `?date=` anchors the window; prev/next links carry the adjusted date and the current view |
| 5 | A `planned_end_at IS NULL` issue appears as an anchor block, not dropped |
| 6 | Sprint band renders on the project axis for an overlapping **active** sprint, and not for a planned or completed one |
| 7 | **§2.4** — a project calendar with two users' issues contains neither display name |
| 8 | **§16.6** — the rendered page contains no percentage, no "of", no ratio between a count and the threshold; the crowding chip, if present, carries only its state word |
| 9 | Both footers render, byte-identical to their keys |
| 10 | Sub-issues appear on neither axis |

Test 8 is the one that will look pointless until someone adds a helpful number.
Write it so it fails if they do.

## 6. Escalate rather than deciding

- If percentage-positioned blocks cannot express overlapping spans without
  JavaScript, stop and report. `DEC-021` permits JS only as enhancement over a
  working no-JS path, and a calendar that needs JS to be readable is not that.
- If the month view's mobile degradation needs a second query shape, say so
  before writing one — that is a storage change wearing a CSS disguise.
- If the crowding chip needs a number (§2.3), report and ship without it.
- If any surface here would read better with the assignee on it, that is §2.4's
  finding, not a request.

## 7. Acceptance

1. All ten §5 tests pass, in `calendar_surfaces.rs`, with a CI job and a
   `CONTRIBUTING.md` line.
2. No assignee name on any project-axis block.
3. No quantity on the crowding chip; nothing above `Watch`.
4. The UTC note renders on both axes.
5. Every user-visible string a `MessageKey`; `prose_scan` passes with no new
   allowlist entries.
6. `active_sprints_overlapping` returns active sprints only.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs; `test_harness_scan` passes.
8. No efficiency metric of any kind, and RFC 002's comment is in place.

## 8. Prohibited

No team axis — permanently, and §10.2 is the reason this product has the shape
it has. No fill rate, free hours, comparison to a previous period, or any
number describing how full time is. No drag-and-drop, no create-from-cell, no
recurring events, no iCal — all Phase D-3 or later. No `start_date` /
`due_date`. No second migration. No per-user default view. No pluralisation
logic.

## 9. Required review-request format

Workflow §9.2. State where date formatting landed (§2.2), and whether the
crowding chip shipped or was reported out (§2.3).
