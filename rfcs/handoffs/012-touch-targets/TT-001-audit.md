# TT-001 — the touch-target audit

**Governing RFC**: [012](../../accepted/012-touch-target-conformance.md), step 2
**Target release**: 0.30.0
**Depends on**: step 1 (the amended `NFR-A11Y-007`) — done before this issues

## 1. What this is

**An audit. No code changes.** Not one class edited, not one control resized.

`DEC-049` amends `§33.2` to a *target*-size rule with an adjacency clause. 139
controls do not present a 44 px target. Before any of them is touched, this
handoff establishes **which mechanism each one takes** and **where expanding
them would create overlap**.

**Why it is separate work.** RFC 011's step 1 was commissioned because a plan
rested on an architect's estimate; it returned a correction that changed the
plan. This RFC has already had one estimate overturned by evidence that was in
the tree the whole time — I argued a uniform floor would ruin the board's
density, and the board's status buttons had been 44 px for releases. **The audit
exists to catch the second such error before 139 edits are made against it.**

If your findings contradict this handoff, **say so**. That has been the most
valuable outcome of every audit this project has run.

## 2. The two mechanisms

A control satisfies the amended `§33.2` either way:

- **Grow** — the visible control becomes 44 px. `min-h-11 min-w-11` on the
  existing size class is the shape already in the tree
  (`confirmation.rs:53`/`:58`, `issues.rs:825`).
- **Expand** — the visible control keeps its size; the *hit area* reaches 44 px
  by other means.

**Neither is preferred in the abstract.** Grow is simpler and should be the
default. Expand is for places where growing would break a layout that has a
reason to be dense.

**Do not decide the mechanism by counting effort.** Decide it by what the
surface is for, and say why in one line per surface.

## 3. Scope — where the 139 are

| File | Sub-44 class uses |
|---|---|
| `issues.rs` | 45 |
| `teams.rs` | 17 |
| `sprints.rs` | 17 |
| `settings.rs` | 16 |
| `projects.rs` | 10 |
| `auth.rs` | 7 |
| `sprint_plan.rs` | 6 |
| `notifications.rs` | 5 |
| `calendar.rs` | 4 |
| `notification_preferences.rs` | 3 |
| `layout.rs` | 3 |
| `search.rs`, `confirmation.rs` | 2 each |
| `me.rs`, `error_page.rs` | 1 each |

**That column counts `btn-sm`/`btn-xs`/`input-sm`/`input-xs`/`select-sm`/
`select-xs` occurrences. It is not the number of non-conforming controls**, and
the two quantities are easy to confuse because they happen to be the same
number. The arithmetic:

| | |
|---|---|
| Sizing-class occurrences (the column above) | **139** |
| — of which already overridden to 44 px by `min-h-11 min-w-11` | −3 |
| Checkbox controls, **not counted in that column at all** | +3 |
| **Interactive elements below a 44 px target** | **139** |

The three already-compliant controls are `confirmation.rs:53`,
`confirmation.rs:58` and `issues.rs:825`. The three checkboxes are all in
`notification_preferences.rs` — that file's `3` in the table above is
`btn-sm` ×2 and `select-xs` ×1, and its checkboxes are **additional**.

**Re-derive all of this yourself rather than trusting the table.** This exact
count has already been wrong on this project's record in three ways at once: the
baseline stated 139 in one place and 149 in another, counted `grep` matches
rather than controls for the checkbox row, and missed the three overrides
entirely — arriving at a correct total only because two of the errors cancelled.
A number that survives because its errors cancel stops surviving the moment
either is fixed alone.

**If your figure is not 139, that is a finding**, and so is a different
decomposition reaching the same total.

## 4. What to produce

### 4.1 Per surface, one row per control

| File:line | Control | Class today | Resolved px | Mechanism | Why |
|---|---|---|---|---|---|

**Group by surface, not by class.** The decision is "what is this screen for",
and a `btn-sm` in a confirmation dialog and a `btn-sm` in a filter bar are not
the same question.

### 4.2 The adjacency map — the part that matters most

For every cluster of two or more interactive elements that sit adjacent:

- where it is,
- the gap between them **today**,
- whether both reaching a 44 px target would make their hit areas **overlap**,
- and if so, what would have to change.

**This is the finding this audit exists for.** Clause (1) without clause (2)
manufactures mis-taps: two 32 px controls 4 px apart, each expanded to 44 px,
overlap — and a tap in the overlap resolves to whichever element is stacked
above. That is worse than the small target it replaced, because it is wrong
rather than merely difficult.

**Where you cannot tell from source, say so and name what you would need.**
Precision about what is unknowable from the component tree is worth more here
than a guess with a confident tone. RFC 012 already records that full
verification of clause (2) may want rendered geometry that this project does not
have.

### 4.3 The open question RFC 012 asks

**Can clause (2) be partially guarded from source?** Sibling interactive
elements inside one flex/grid container with a known gap utility may be
checkable without rendering — `gap-3` between two 32 px controls gives a 44 px
pitch, and that is a fact about class names.

Answer whether that pattern holds **often enough to be worth a guard**, or
whether adjacency is inspection-only until `§10.15` step 4. **Either answer is
useful.** "Not often enough" is a finding, not a failure.

### 4.4 Anything the amended rule does not cover

If you find an interactive element that is neither growable nor expandable
without breaking something, or a control whose target is ambiguous (a whole
card that is clickable, a link inside a paragraph), **name it**. The amendment
is two days old and has not met the codebase yet.

## 5. What must not change

- **No control is resized in this handoff.** If you find something so wrong it
  should be fixed immediately, report it and stop — do not fix it here.
- `touch_target_scan` keeps banning exactly `checkbox-xs` and nothing more. Its
  broader form is `TT-003`, and it must not land before the tree can pass it.
- No test is added or changed.

## 6. Escalate rather than deciding

- **If the amended rule is wrong.** You will be the first to apply it to real
  code. If clause (1) or clause (2) turns out to be unimplementable, or the
  target/visual distinction does not survive contact with DaisyUI's resolved
  CSS, that is a finding about `DEC-049` and it outranks the audit.
- **If the mechanism choice for a surface is really a product decision** — if
  growing a control changes what a screen is *for* rather than how it looks.
- **If the 139 is wrong**, in either direction.

## 7. Exit condition

The inventory of §4.1, the adjacency map of §4.2, and an answer to §4.3.

**`TT-002` will be written from this document.** Its scope is whatever this one
establishes — so an "I could not determine this from source" in here becomes a
named unknown in there, rather than a silent assumption.

---

**Who holds what**: dev team — the audit. **What's blocked**: `TT-002` and
`TT-003`, both of which are written against this output. **What's next**: a
review request package with the inventory and the adjacency map.
