# ORD-001 — retire D-5: remove `position`, and stop ordering a list by a TEXT column

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.38.0
**Governing RFC**: [0004](../../done/004-direct-manipulation.md) — this
**retires** substep D-5 rather than implementing it. No `0004e` will be written.
**Owner approval**: the amendment of `FR-DM-001` from five surfaces to four was
approved 2026-09-24. Reasoning: `.git-exclude/tasks/architect/021-d5-decision.md`.
**Depends on**: nothing. Independent of `PLAN-003` — different queries, and they
can land in either order.

---

## 1. What is being decided, and why the column goes

D-5 was to give the issue list a manual order. It is retired: the product
already answers *what do we do next* with priority bands, sprint membership and
planned dates, and **the sprint is the better answer** — named, shared,
time-boxed, and on a page of its own. A manual order would be a second answer
to the same question with no name in the UI and no visible provenance. RFC
0004's cross-cutting requirement 10 settles the rest: a drag is not a touch
path, so on a phone D-5 would be up-and-down buttons on every row, which does
not order a long backlog at any width.

**Retiring is not doing nothing.** `position` is assigned `MAX+1` per
`(project, status)` on insert and **never recomputed** — `update_status` writes
`status` alone. A status change carries the old band's number into the new
band. So the list's default sort and every board column currently present an
order **nobody chose and nothing explains.** That is what this change removes.

## 2. The second defect, found while enumerating — and it is the same class as `PLAN-003`

Both list queries order:

```sql
ORDER BY status ASC, position ASC, created_at DESC
```

`status` is **TEXT** (`open` | `in_progress` | `done`), so `ASC` is
alphabetical. SQLite's own ascending order of those three values is:

```
done   in_progress   open
```

**The default issue list therefore shows Done issues at the top and Open at the
bottom.** Traced end to end: `apply_filter_and_sort`'s default branch preserves
storage order (`handlers/issues.rs:150`), the flat `Vec` goes to
`render_project_detail` at `:281`, and the component does not re-sort.

The board is **not** affected — it builds its columns from
`IssueStatus::all()` (`handlers/issues.rs:254`), which is
`[Open, InProgress, Done]`, the lifecycle order. Only the list reads the SQL
status order, and only the list is wrong.

**This is `PLAN-003`'s defect in a second place**: SQL asked to order a TEXT
column as though its alphabetical order carried meaning. Two independent
instances found in two days is the finding, not the coincidence.

**Scope note, checked unbounded**: `grep` over every migration shows `position`
appears **once** in the entire schema — `0001_initial.sql:39`, with **no index
and no trigger** on it. And **no test anywhere references it**, which is why
neither defect was ever caught.

## 3. The new ordering

**`ORDER BY created_at DESC` — newest first, and nothing else.**

- **The status term is removed, not corrected.** Fixing it means a `CASE` in
  SQL or a Rust re-sort, to reintroduce a grouping the **board already
  provides properly**. The list is a list; the board is the status view; the
  status filter narrows either. One term beats two, and no rule needs
  explaining.
- **Deliberately not "priority then newest."** That would put severity ordering
  into two more SQL queries on a TEXT column — `PLAN-003`'s defect multiplied —
  and would promote the stored-ordinal column `PLAN-003` §3 declines. The list
  already offers `sort=priority`, which sorts correctly in Rust.

**This is a visible behaviour change and it is the one thing here the owner did
not explicitly approve.** What was approved is removing `position`; removing
`status ASC` is an addition, made because rewriting these exact clauses while
leaving a known defect in them is not defensible. **If it is unwanted, stop and
say so** — `ORDER BY status ASC, created_at DESC` is the smaller change and
this handoff still works with it.

## 4. Every site — enumerated, not sampled

**Schema** — `crates/peisear-storage/migrations/`
- `0001_initial.sql:39` — `position INTEGER NOT NULL DEFAULT 0`. The only
  schema occurrence; no index, no trigger.
- New `0018_remove_issue_position.sql`. **Do not edit `0001`** — it is applied
  history.

**SQL in `peisear-storage/src/issues.rs`**
- `ORDER BY` (both become `created_at DESC`): `:81` `list_in_project`,
  `:104` `list_all_in_project`
- `SELECT` lists dropping the column: `:75`, `:99`, `:124`, `:142`, `:949`, `:981`
- `INSERT`: `:191` (the `MAX(position)+1` subquery goes entirely), `:205`
  (column list), `:285` (`insert_sub_issue`, the literal `0`)
- Doc comments at `:259-262` and `:968` describe the sub-issue counter and are
  removed with it.

**SQL in `peisear-storage/src/sprints.rs`**
- `:799` — `i.position` in `backlog_for_team`'s `SELECT`. It is selected and
  never ordered by; it goes with the column.

**Rust**
- `peisear-core/src/lib.rs:185` — `pub position: i64` on `Issue`
- `peisear-storage/src/issues.rs:24` and `:52` — `IssueRow` field and mapping
- `peisear-storage/src/sprints.rs:725` and `:751` — `BacklogIssueRow` field and
  mapping
- `peisear-web/src/handlers/issues.rs:146` and `:150` — doc comments stating
  the old ordering; rewrite both to the new one.

**`Issue.position` is `pub` on a published crate.** Removing it is a breaking
change to `peisear-core`'s API. Under 0.x a minor bump carries it, but it
belongs in the release note explicitly rather than being discovered by a
downstream build.

## 5. The migration

`ALTER TABLE issues DROP COLUMN position;` — SQLite has supported it since
3.35, there is no index or trigger on the column, and it is not in any view.

**Confirm the bundled SQLite actually supports it before writing the rest**,
rather than assuming the toolchain matches the CLI on this machine (3.53.4). If
it does not, the table-rebuild form is the fallback and needs the triggers from
`0017` re-created afterwards — say so rather than improvising it.

## 6. Verification

- **The list's order changes, and a test pins the new one.** No test references
  `position` today, which is why two defects lived here undisturbed. Add one:
  issues created across all three statuses, asserted **newest first regardless
  of status**, by relative position — not `body.contains`, which passes on any
  order (`PLAN-002` round 2).
- **Assert Done is no longer first.** The specific old behaviour, named, so the
  fix cannot silently revert.
- The board's columns are unchanged: Open, In progress, Done, and within a
  column newest first. Check before and after.
- `sort=priority`, `sort=created`, `sort=updated` and the status/assignee
  filters unchanged.
- Sub-issue creation and listing unchanged — sub-issues order by creation time
  and always did.
- Analytics/health (`list_all_in_project`) and the sprint-plan backlog still
  render; the backlog's own ordering is `PLAN-003`'s business, not this one's.
- Migration applies to a populated database and to an empty one; the app starts
  on both.
- `DEC-007`: report the new count, last recorded **272**. A new test file means
  editing `.github/CONTRIBUTING.md`'s block in the same change.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched.

## 7. Escalate rather than deciding

- **If the bundled SQLite cannot `DROP COLUMN`** (§5).
- **If anything outside this list reads `position`.** The enumeration in §4 is
  from a grep that was widened after an earlier filter of mine silently
  excluded every `position:` struct field — so treat it as thorough, not as
  complete, and report anything it missed.
- **If removing the status term changes a page this handoff does not name.**
- **If a user-visible string changes.** None should.

## 8. What the architect does, not the dev team

`FR-DM-001` amended to four surfaces with D-5 recorded as retired by decision;
RFC 0004's table and status updated; `021`'s revisit trigger carried into the
requirement. Those land alongside this change, not in it.

## 9. Exit condition

`position` exists nowhere in the schema, the queries, the structs or the
published API; the issue list reads newest-first with Done no longer at the
top; a test fails without the change and passes with it; the board is
untouched.
