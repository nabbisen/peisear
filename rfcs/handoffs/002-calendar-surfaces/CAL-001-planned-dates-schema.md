# CAL-001 — Planned dates: schema, core, storage, form

**Issued by**: Architect
**Date**: 2026-08-13
**Priority**: P1 — first work of 0.23.0
**Governing RFC**: [002](../../done/002-calendar-surfaces.md)
**Depends on**: nothing

---

## 1. Scope

The data layer for RFC 002's calendars, and nothing that renders one.

**In scope**: migration `0016`, `planned_start_at` / `planned_end_at` on
`Issue`, the two storage window queries, and the issue edit form's date inputs
so a user can actually set them.

**Out of scope**: `/today/calendar`, `/projects/{id}/calendar`, view modes,
period navigation, the sprint band, the crowding chip. Those are CAL-002, which
is written after this is reviewed.

If you finish early, stop. A schema migration is the one thing here that cannot
be corrected by editing a file, and it deserves its own review.

## 2. Five corrections to RFC 002

RFC 002 was written 2026-05-04 and targeted 0.21.0. It predates the i18n
architecture, the compliance pass, RFC 009 and COPY-001. Where this handoff and
the RFC disagree, **this handoff wins**; the RFC is amended to match. A sixth
disagreement is a finding — report it.

### 2.1 The privacy footnote has two normative texts, and they differ

RFC 002 must-have 9 gives, "verbatim from the spec; do not paraphrase":

> Calendar note: this view shows planned issue work.
> Personal schedules are not aggregated here.

External design §10.3 gives, under Fixed texts:

> Calendar note: this view shows planned issue work for this project.
> Personal schedules are not aggregated here. Each member's individual
> calendar is private to that person.

**External design's text is normative.** It is the more complete one and its
third sentence is the only place the guarantee is actually stated to the user;
RFC 002's is a truncation that drops it.

This is the second time two of our documents have carried different "do not
paraphrase" versions of the same string. The first was the team footnote, found
at 0.21.0 *after* the code had already diverged from the spec. This one is
being settled before anything is built, which is the only difference that
matters.

The personal-axis footer stays "Private to you".

### 2.2 All copy goes through `peisear-i18n`

RFC 002 predates RFC 006. Both footers are `MessageKey`s, `prose_scan` will
fail on a literal in `components/` or `handlers/`, and **§D6 rule 7 applies** —
one key, one `en.rs` arm, no `format!`-assembled sentences.

Because §2.1's text is normative, add a byte-identity test in `peisear-i18n`
alongside `team_privacy_footnote_renders_byte_identically`. That test is what
makes "normative" mean something.

### 2.3 The trigger's `RAISE` text is user-facing copy naming database columns

RFC 002's migration raises:

```
'planned_end_at must be on or after planned_start_at'
```

Two problems, and they compound.

**It names columns, not fields the user has seen.** COPY-001 fixed exactly this
sentence shape three days ago: `"period_start must be on or before period_end"`
became `"The From date must be on or before the To date."`, because
`period_start` appears in no sentence a user reads. `planned_start_at` is the
same.

**Per `DEC-011`, the `RAISE` text *is* the rendered text.**
`translate_trigger_error` matches the trigger's string as a needle and returns
the `MessageKey` carrying identical text (`issues.rs:360`). So the wording is
not a detail you can fix later at the web layer — the migration is where the
user-facing sentence gets decided, and migrations do not get edited.

**So decide the sentence first.** Read the issue form's own labels for the two
new inputs, name them, and make the `RAISE` text, the `MessageKey`, and the
label agree. Report the wording you chose in the review request. If the labels
you add and the sentence disagree, that is the same defect one layer up.

### 2.4 The optimistic lock holds only if you use the existing UPDATE

RFC 002 says the existing lock covers these columns because they are edited
through the issue update form. True, **conditionally**: `issues::update`
(`issues.rs:~426`) sets `updated_at = CURRENT_TIMESTAMP` in its own `SET`
clause. `issues` has no `updated_at` trigger — `DEC-013`'s trigger machinery
covers `sprints`, `teams`, `team_memberships` and `user_capacities`, not this
table.

**Add the two columns to that existing statement.** If they get their own
`UPDATE`, the row's `updated_at` will not move and every concurrent edit to a
plan date will silently win — `NFR-CONC-004` violated with no error and no
symptom.

A test asserts a stale `client_updated_at` on a planned-date-only edit returns
409, in `optimistic_lock.rs` where its siblings live.

### 2.5 UTC display is out of scope, and must be named as a decision

RFC 002 defers time-zone awareness: "we render in UTC for now". That stands —
it needs the §34 locale discussion.

But it means a user in Tokyo sets "09:00" in a `datetime-local` input and, once
CAL-002 renders it, sees a time that is not the one they typed. **Say so in the
review request** rather than letting it be discovered on the calendar. It is a
deliberate limitation, not an oversight, and the difference is whether anyone
wrote it down.

## 3. What stands unchanged

Two columns, not the spec's four (open question 1's default, and the reasoning
in §Background — schema decision recap is still right). The partial index. The
`IssueRow` / `into_issue` pattern, same as `parent_issue_id` in 0.19.0. Both
storage query shapes. `datetime-local` inputs parsed and converted to UTC at
the handler boundary.

## 4. The migration

`0016_issue_planned_dates.sql`. Current head is `0015`, so the number is free.

This is the project's first migration since before the compliance pass. Two
things to hold to:

- **Both triggers, insert and update**, as the RFC specifies. A constraint
  enforced on one path is a constraint that holds until someone uses the other.
- **The partial index** — `WHERE planned_start_at IS NOT NULL`. Most issues in
  a real project will never have plan dates.

Run the migration forward against a database with existing rows and confirm it
applies cleanly. `ALTER TABLE ... ADD COLUMN` with a nullable column is safe in
SQLite; confirm it rather than assume it, because this is the step with no undo.

## 5. Tests

| # | Check |
|---|---|
| 1 | Migration applies to a populated database; existing rows get `NULL` for both columns |
| 2 | Trigger rejects `planned_end_at < planned_start_at` on **insert** |
| 3 | Trigger rejects it on **update** |
| 4 | Either column `NULL` is accepted — the constraint is only about both being set |
| 5 | `translate_trigger_error` maps the new `RAISE` text to its `MessageKey`; the rendered text and the needle are identical |
| 6 | The two window queries return issues overlapping the window and exclude those outside it, including the half-open case (`planned_end_at IS NULL`) |
| 7 | Personal query returns only the given assignee's issues |
| 8 | **§2.4** — a stale `client_updated_at` on a planned-dates-only edit returns 409 |
| 9 | The footnote key renders byte-identically to §2.1's text |

Test 8 is the one that fails silently if §2.4 is got wrong, so write it before
the form change and watch it fail.

## 6. Escalate rather than deciding

- If `ALTER TABLE` on `issues` interacts badly with an existing trigger or
  index, stop and report. Do not work around it in the migration.
- If the window query needs an index the RFC did not specify, say so — do not
  add one silently. An index is a schema commitment.
- If the two footnote texts turn out to differ from *both* documents somewhere
  else in the repo, report all three rather than picking.

## 7. Acceptance

1. Migration `0016` applies cleanly forward against a populated database.
2. All nine §5 tests pass; test 8 demonstrated failing first.
3. The two columns join `issues::update`'s existing `SET` clause.
4. The trigger text, the `MessageKey`, and the form labels all agree, and the
   chosen wording is reported.
5. Both footer keys exist and the project-axis one matches external design
   §10.3 byte for byte.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs; `prose_scan` and `test_harness_scan` pass.
7. Nothing renders a calendar.

## 8. Prohibited

No calendar routes, components, or view modes — that is CAL-002. No team axis,
ever (RFC 002 §Explicitly out, and §10.2 is the reason the product exists in
this shape). No efficiency metric of any kind. No `start_date` / `due_date`
columns. No rewording of external design §10.3's footnote. Do not edit a
migration after it applies anywhere — add another.

## 9. Required review-request format

Workflow §9.2. State the wording chosen in §2.3 and the labels it matches, and
name the UTC limitation per §2.5.
