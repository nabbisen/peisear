# RFC 012 — Touch target conformance

**Status**: Accepted
**Author**: architect
**Target**: 0.30.0 (amendment + audit), 0.31.0 (application + guard)
**Supersedes**: nothing. Amends `SPEC §33.2` via `DEC-049`.
**Related**: `NFR-A11Y-007`, baseline `§8` Definition of Done item 5,
`touch_target_scan`, `§10.15` (for the one thing this cannot verify)

## Summary

`NFR-A11Y-007` requires a 44 × 44 px touch target on every interactive element.
**139 controls do not present one**, and the requirement has been open since
0.19.1 — the oldest condition in the Definition of Done.

This RFC does **not** resolve that by raising 139 controls. It resolves it by
fixing the requirement first, because `§33.2` as written is unverifiable in one
direction and misleading in another, and any pass run against it would inherit
both faults.

Three changes, in this order:

1. **`§33.2` states a *target* size, not a visual size** — hit-area expansion
   satisfies it.
2. **`§33.2` gains an adjacency rule** — expanded targets may not overlap.
3. **`touch_target_scan` enforces the whole requirement**, which it cannot do
   under any of the alternatives considered.

## Background — the requirement was never examined

`SPEC §33.2`'s 44 px is **WCAG 2.2 AAA** (2.5.5). The **AA** criterion is 2.5.8:
24 × 24, with a spacing exception. `§33.2` adopted the stricter figure and
dropped the exception that makes the standard workable in dense interfaces.
Nothing in the record indicates that was a considered choice rather than the
larger number looking safer.

Measured against the standard the product actually claims in its changelog —
AA — **zero controls fail on size**. All 139 failures are failures against a
self-imposed AAA bar.

That made "amend to AA" look like the obvious answer. It is not, for the reason
in §2 below.

## The decision

### `DEC-049` — `§33.2` is a target-size rule with an adjacency clause

**`§33.2` is amended, not relaxed.** The 44 px figure is kept. Three parts:

1. **Target, not visual.** *"Interactive elements MUST present a touch target of
   at least 44 × 44 CSS pixels. The target is the area that responds to a
   pointer or touch, which need not equal the control's visible bounds — a
   control may satisfy this by expanded hit area rather than by visible size."*
2. **Adjacency.** *"Touch targets of distinct interactive elements MUST NOT
   overlap."*
3. **Verification.** *"Conformance with (1) MUST be enforced by a structural
   guard over the component source. Conformance with (2) is verified by
   inspection per surface until rendered-geometry measurement is available
   (`§10.15`)."*

**`SPEC §33.2` amendment pending**, recorded the way `DEC-030` records `§28.1`'s.

### Why not the alternatives

**Amend to WCAG AA (24 px + spacing), or adopt a two-tier rule (44 px where a
mis-tap is costly, 24 px elsewhere).** Both were considered and both were
rejected for the same reason, which is not strictness:

**Both rest on the spacing exception, and the spacing exception cannot be
evaluated from source.** It is a claim about rendered geometry — whether a 24 px
circle centred on a target intersects another's. No test this project owns can
answer it, and whether to acquire one that could is `§10.15`'s open question,
deliberately deferred to 0.32.0.

Adopting either would write into the specification a conformance claim that can
only ever be **asserted**, never checked. That is `§10.15`'s known gap promoted
into the requirement itself. `touch_target_scan`'s own doc comment already
states the consequence: a rule that cannot be checked on the tree gets weakened
until it passes, which is worse than no rule. `QA-013` stopped `contrast_scan`
at `/60` for exactly this reason.

**The two-tier rule fails a second time, and worse.** It makes *conformance
itself* vary by surface — some controls permitted to be harder to hit — and
requires the judgement *"is a mis-tap costly here"* to be made for 139 controls
and re-made for every control added afterwards, with no guard able to check any
of it. Under `DEC-049`, only the **mechanism** varies by surface: grow the box,
or expand the hit area. Varying the mechanism has no accessibility consequence.
Varying the tier is nothing but accessibility consequence.

### The density objection, and why it did not survive

This RFC's first draft recommended the two-tier rule, on the ground that a
uniform 44 px floor would change the product's density — with the board card's
status buttons as the lead example: *"a card whose status buttons are 44 px tall
is a different card."*

**Those buttons are already 44 px.** `components/issues.rs:825` carries
`btn btn-ghost btn-xs min-h-11 min-w-11 px-2`, two per card, shipped since
before the question was raised. The densest surface in the product is already
compliant and is not worse for it.

The argument was wrong twice: `§33.2` says *target*, so it never demanded visual
growth in the first place; and the one surface named as the worst case had
already taken the change without harm. **Recorded rather than deleted**, because
a recommendation reversed by evidence already in the tree is the more useful
thing to have written down.

## What this costs, honestly

**Overlap is a real hazard and it is created by the fix.** Two 32 px controls
4 px apart, each expanded to a 44 px target, now have overlapping hit areas —
and a tap in the overlap resolves to whichever element is stacked above. That is
**worse than a small target**, because it is wrong rather than merely difficult.
It is precisely why WCAG pairs the smaller size with a spacing exception instead
of stating a bare floor.

A uniform target floor **without** clause (2) manufactures the defect it was
adopted to prevent. Clause (2) is not decoration.

**The asymmetry that decides this RFC**: clause (2)'s full verification wants
rendered geometry, which arrives — if it arrives — with `§10.15` step 4 at
0.32.0. So `DEC-049` needs measurement to close an **edge case**. The rejected
options need it to establish the **baseline claim** for every control that is
not 44 px. One is a gap at the margin of an otherwise checkable requirement; the
other is a specification resting on it.

## The work

| Step | Release | What | Exit |
|---|---|---|---|
| **1** | 0.30.0 | **Amend the requirement.** `NFR-A11Y-007` rewritten; `DEC-049` recorded; external design's two acceptance-axis rows updated. **Architect's own work, no handoff.** | Baseline and external design carry the amended rule |
| **2** | 0.30.0 | **Audit** (`TT-001`). Per surface: which mechanism each control takes, and where adjacency risk exists. **No code changes.** | An inventory that names every one of the 139, and the adjacency risks by location |
| **3** | 0.31.0 | **Apply** (`TT-002`), then **guard** (`TT-003`). | `touch_target_scan` enforces clause (1) across the tree |

**The audit is separate from the application and comes first.** RFC 011's step 1
is the precedent: it was commissioned because a plan rested on an architect's
estimate, and it returned a correction that changed the plan. This RFC has
already had one estimate overturned by evidence in the tree (§ the density
objection). The audit exists to catch the second one before 139 edits are made
against it.

**The guard cannot come first.** It would fail on the current tree, and a guard
that fails on a tree everyone believes is correct gets weakened until it passes.
It lands with or immediately after the work that makes it true.

## Out of scope

- **`NFR-A11Y-006`** (mobile completion), the other open limb of Definition of
  Done item 5. Related surface, different requirement, not this RFC.
- **Acquiring rendered-geometry measurement.** That is `§10.15` step 4's
  decision at 0.32.0 and must not be pre-empted here.

## Open questions

- **Whether clause (2) can be partially guarded from source.** Sibling
  interactive elements in one flex/grid container with a known gap utility may
  be checkable without rendering. The audit should say whether that is true
  often enough to be worth a guard, or whether it is inspection-only until
  0.32.0.

## References

- `NFR-A11Y-007`, baseline `§8` item 5, baseline `§10.15`
- `QA-014` (the survey), `QA-015` (the four controls raised ad hoc)
- `touch_target_scan`, and its doc comment on why it bans one class only
- WCAG 2.2 SC 2.5.5 (AAA, 44 × 44) and SC 2.5.8 (AA, 24 × 24 with spacing)
