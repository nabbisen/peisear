# LAYOUT-011 — the assignee badge, which the gate has never rendered

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none — defect fix, plus a coverage decision.
**Source**: found by the dev team's survey in `LAYOUT-010` §2 — twelve pages
the gate does not visit, measured at five widths. Ruled in
`.git-exclude/reviewed/LAYOUT-010-and-GATE-001-review.md` §2.
**Depends on**: `LAYOUT-010` and `GATE-001`, both landed.

---

## 1. The defect

`<span class="badge badge-sm badge-ghost">` holding a **display name** —
`components/issues.rs:520`, `:818`, `:1820` — with no `min-w-0` and no
truncation. A display name is user text and can be an unbreakable run.
Measured with `GATE-001`'s fixture: **441 px of badge**, overflowing the
**team project's board** at 320, 390, **768 and 1280** (183, 114, 239, 50 px)
and **team issue detail** at 320 and 390 (137, 67).

**It overflows a 1280 px desktop.** That is the second time after `LAYOUT-008`,
and it is the part that says this is not a phone problem.

**Why nothing caught it.** The gate's issues are in the *personal* project and
**have no assignee**, so the board and issue-detail pages it already sweeps
cannot render an assignee badge at all. `GATE-001` closed one version of this
trap — an empty fixture making a page look covered — and this is the next layer
down: **a populated fixture whose content never exercises one branch.**

## 2. The fix

**`LAYOUT-007`'s remedy is the starting point** — the display name is user text
in a shrinkable position, `§10.25`'s shape A or B depending on the container —
**but measure which, and say so.** `LAYOUT-008` found a third shape by not
assuming the first two applied, and a `badge` is a shrink-to-fit container,
which is exactly where shape C was found.

- **All three sites**, and look for a fourth: any badge or chip holding user
  text. **Report what you find beyond these three; do not fix it here.**
- **The name must stay useful.** A badge clipped to nothing identifies nobody.
  Say what the narrowest width shows, as `LAYOUT-010` did.
- **Do not change what the badge contains** or its `title`.

## 3. The coverage decision — mine, and here it is

**The gate's fixture gets an assigned issue in the team project, and the
existing pages keep their content.**

`GATE-001` §4 correctly escalated that changing the fixture's assignee alters
existing pages. **So do not change an existing issue — add one**, assigned to
an account whose display name carries the unbreakable run, in the team project
that `GATE-001` created. The personal project's issues stay unassigned and
every page the gate already sweeps keeps what it had, plus one card.

**Expect the cell count to stay 95**: content on existing pages, no page added.
**If it moves, say why** before assuming it is fine.

**The team project's board and issue detail are not currently swept** — they
were in the survey, not the gate. **Adding them is a separate question and not
this handoff's**; the assigned issue makes the *existing* board and issue-detail
pages render a badge, which is what closes the gap.

## 4. Verification

- **The gate red before the fix, green after, with the fixture change.** Land
  them in one round as `LAYOUT-010` and `GATE-001` did — `main` does not carry
  a red gate (`DEC-048` condition 3).
- **Boxes at 320 / 360 / 390 / 414 / 768 / 1280** for each of the three sites,
  before and after, and what the badge reads at the narrowest.
- **Desktop is the point**: 768 and 1280 must be clean *after*, and the before
  numbers recorded — a desktop overflow is the finding.
- **Ordinary names unchanged** at every width, byte-identical if you can.
- `touch_target` green; `DEC-007` reported, last recorded **351**.
- `fmt`, `clippy`, three consecutive workspace runs.

## 5. Escalate rather than deciding

- **If a badge cannot be constrained without hiding the name** at 320.
- **If the fourth site turns out to be a different shape** — that is worth a
  `§10.25` amendment, which is mine.
- **If adding the assigned issue changes a page the survey found clean.**
- **If the gate finds a third surface** once the fixture lands. Report it; this
  has now happened twice, and the third time is a pattern worth recording
  rather than fixing in a hurry.

## 6. Exit condition

No assignee badge overflows at any of the five gate widths on any page, the
gate is 95 cells and green with an assigned issue in the fixture, and the
display name is still readable at 320.
