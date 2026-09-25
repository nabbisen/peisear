# A11Y-001 — two P1 requirements whose status says nothing

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.41.0
**Governing RFC**: none — an audit, like `REQ-001`.
**Related requirements**: **`NFR-A11Y-002`** and **`NFR-A11Y-004`**, both P1.
**Depends on**: nothing. **Do this first** of the two 0.41.0 handoffs.

**Report, do not fix.** Findings become handoffs; the wording is mine.
**If you find a violation you can fix in a line, still report it** — I want the
shape of the whole before any of it moves.

---

## 1. Why these two

`REQ-001` found six requirements whose text admits two readings. **These two are
worse: their status is `Partial.` and nothing else.** No statement of what is
partial, so there is nothing to check against and nothing that could ever be
wrong. They have been unfalsifiable since they were written.

**Both are P1, and both are about whether the product is usable by someone who
cannot see it the way its author does.**

> **`NFR-A11Y-002` — Focus management.** *After a mode change, focus MUST move
> to a defined, visible location, and MUST NOT be sent off-screen.*
>
> **`NFR-A11Y-004` — Meaning not carried by colour alone.** *State MUST be
> conveyed by label and icon in addition to colour. Colour-blind-safe
> patterning MUST be used in charts.*

**`NFR-A11Y-002` is the urgent one**, because this product has spent four
releases adding **mode changes**: `D-1`'s in-place status change, the board
drag, the sprint-plan drag, the calendar drag, and now reopen. Each replaces
part of a page under the user. **Nobody has ever checked where focus goes**,
and `RFC 0004`'s requirement 1 gives every drag a keyboard equivalent — which
is worth nothing if using it strands the caret.

## 2. `NFR-A11Y-002` — what to establish

**Every place the page changes under the user**, and for each: where focus was,
where it went, and whether it is visible. Start from these and say if there are
more:

- **`D-1` in-place status change** (issue list, issue detail) — the control is
  replaced after the POST.
- **The board drag and its move buttons**; **the sprint-plan drag and buttons**;
  **the calendar drag**. Per `RFC 0004` requirement 10, the buttons are the
  touch and keyboard path, so **the button path matters more than the drag**.
- **Undo toasts** (`D-1`, `D-4`) — they appear, take an action, and vanish.
- **Reopen, start, complete** — a form POST that re-renders the page.
- **The confirmation interstitials** (`RFC 010`) — a full page between intent
  and effect.

**Measure, do not read.** `document.activeElement` before and after, its
bounding box, and whether it is inside the viewport. The browser tooling from
`browser-checks/` is there; a scratch probe is fine and this is not a gate
change.

**`MUST NOT be sent off-screen` is the testable half** — a focused element
outside the viewport, or `body` when it was on a control, is a finding. **A
defined, visible location** needs judgement: say where focus went and whether a
person would find it.

## 3. `NFR-A11Y-004` — what to establish

**Every place state is shown.** Issue status, priority, sprint lifecycle, team
role, health indicators, the workload strip, calendar blocks, notification
kinds.

For each: **is the state readable with colour removed?** A badge whose text
names the state passes; a badge distinguished only by `badge-error` versus
`badge-success` does not. **Report per surface**, not as a verdict.

**Charts are the second limb and a different question**: *colour-blind-safe
patterning MUST be used*. The burndown and the completed-work chart — is
anything distinguished by hue alone? **`NFR-A11Y-003` already requires a
tabular equivalent for every chart**, which may be the answer the product
already has; **say whether it is**, because if a table carries the same
information then patterning is a different and weaker obligation than the
requirement's wording suggests. **That is a finding, not a decision** — do not
conclude the requirement is met because an equivalent exists.

## 4. Verification

- **No code changes.** If something is one line and obviously wrong, report it
  with the line; I will schedule it.
- **Say what you could not check**, as `REQ-001` did — a surface you could not
  reach, a state you could not produce.
- **Distinguish measured from read**, per site. `REQ-001`'s value came from
  marking which was which.
- `DEC-007` unchanged at **360**. `fmt`, `clippy` if anything is written.

## 5. Escalate rather than deciding

- **A focus violation on a path with no keyboard alternative.** That is
  `FR-DM-002` as well, and it stops being an audit finding.
- **If either requirement cannot be checked as written** — that is the most
  useful thing you can report, and it is what `REQ-001` §5 called the more
  valuable list.
- **If the count of mode changes is larger than §2's list.** Say how many.

## 6. Exit condition

A report saying, per surface, where focus goes after every mode change and
whether state survives the removal of colour — enough that both statuses can be
written as something a later reader could find wrong.
