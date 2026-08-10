# I18N-005b — Project and issue surfaces

**Issued by**: Architect
**Date**: 2026-08-10
**Priority**: P1
**Depends on**: I18N-005a (pattern settled)
**Parallel with**: 005c, 005d, 005e

The six pattern rules are in the queue README. They are not restated here; this
handoff covers only what is specific to this group.

---

## 1. Scope

`components/issues.rs` (~35 literals) and `components/projects.rs` (~13), plus
any user-visible strings in the matching handlers.

`issues.rs` is the largest component file in the crate and has been edited by
six handoffs this cycle. Expect the survey to be harder than the count suggests.

## 2. Already converted — do not re-convert

Parts of this file already go through the table:

- Health explanation sentences and the summary — I18N-002 and I18N-004.
- Indicator labels — I18N-004, via `IndicatorLabel`.

Leave them. If something looks half-converted, check whether it is one of these
before treating it as an omission.

## 3. This is where rule 1 bites hardest

> A `String` parameter carries user data only. Anything that is our own copy is
> a key.

This group is the reason that rule exists. On these surfaces:

- **User data**: issue titles, project names, descriptions, assignee display
  names. `String` parameters, correctly.
- **Our copy**: "New issue", "Add sub-issue", "Edit", "Delete", column headings,
  empty-state text, status and priority words.

The two look identical at the call site — both arrive as `&str`. Ask of every
parameter: *if this text were wrong, would a user have written it, or would we?*

## 4. A fifth and sixth prose function in `peisear-core`

`IndicatorKind::label()` was absorbed by I18N-004. Two siblings remain:

- `IssueStatus::label()` — `peisear-core/src/lib.rs:78`
- `Priority::label()` — `:125`

Both produce user-visible words rendered on these surfaces. Same shape as the
one already absorbed, same reason to absorb them: otherwise those words live
outside the table and the guard never sees them.

**Convert both here.** They belong to this group's surfaces, and doing them
elsewhere would mean touching this file twice.

Follow `IndicatorLabel`'s precedent — a closed enum in `peisear-i18n`, with a
`to_i18n_*` conversion at the boundary.

## 5. Watch for

- **The status segment** (`FR-ISS-005`) carries `aria-pressed` and an accessible
  group name. Attributes are copy.
- **The board card** — DEV-001 and DEV-002 both edited it. Its status-control
  buttons need accessible names that distinguish one card from another
  (`FR-DM-002`); those names embed an issue title, which is user data.
- **Empty states** (`FR-SUB-*`, external design §5.4) must not imply emptiness
  is a deficiency. If the guard objects to existing empty-state copy, that is a
  finding, not a licence to reword.
- **Workload chips** — DEV-003 narrowed these. Do not widen what they display.

## 6. Tests

Guard covers the new entries; exhaustiveness holds; rendered output
semantically identical per rule 5; existing suites unchanged —
`optimistic_lock`, `sub_issues`, `status_segment`, `board_keyboard`,
`workload_privacy` and `view_state` all exercise this file.

## 7. Acceptance

1. No user-visible literal left in the two components or their handlers,
   attributes included.
2. `IssueStatus::label()` and `Priority::label()` absorbed; those words exist in
   one place.
3. Guard passes; rendered output semantically identical.
4. fmt and clippy exit 0; suite counts unchanged.
5. The survey reported per 005a §4.1 — count, method, exclusions.

## 8. Prohibited

No rewording; report instead. No widening of what workload chips show. No
change to what the status segment does — it is display-only until Phase D
(`FR-ISS-006`).

## 9. Review focus to request

1. Every `String` parameter you introduced, and why each is user data.
2. Anything that looked half-converted and turned out to be §2.
3. Whether absorbing the two `label()` functions forced call sites you did not
   expect.
