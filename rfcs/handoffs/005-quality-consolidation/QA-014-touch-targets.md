# QA-014 — Touch targets: measure, and settle one unbacked claim

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: P1 — `NFR-A11Y-007`, *Not verified* since 0.19.1
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §6
**Depends on**: nothing.

**No control changes size in this handoff.** This is a measurement and two
small pieces of work. The decision it feeds is the owner's — see §5.

---

## 1. Read RFC 005 §6 first — it was rewritten today

The original described manual QA at narrow width plus screenshots in
`docs/src/mobile-checklist.md`. **Neither is possible here**: `QA-011`
established there is no browser, and `docs/src/` still has no `book.toml` with
`DEC-020` unresolved. **Do not create that file.**

The four flows' mobile behaviour stays open and named. What this handoff takes
is the part the original mentioned in half a clause and that is measurable from
source.

## 2. What I measured, for you to reproduce

DaisyUI's control heights are fixed values in the pinned stylesheet
(`daisyui@4.12.14`, the version `layout.rs` loads):

| Class | Resolved | Uses in `src/components/` | vs 44 px |
|---|---|---|---|
| `btn-sm` | 2rem / 32 px | 64 | ✗ |
| `btn-xs` | 1.5rem / 24 px | 18 | ✗ |
| `input-sm` | 2rem / 32 px | 29 | ✗ |
| `select-sm` | 2rem / 32 px | 21 | ✗ |
| `input-xs` | 1.5rem / 24 px | 5 | ✗ |
| `select-xs` | 1.5rem / 24 px | 2 | ✗ |
| `checkbox` | 1.5rem square | 10 | ✗ |

**Reproduce both halves** — the class counts from the source, and the resolved
heights from the pinned CSS. If either differs, stop and report.

**Exactly one control complies**: `issues.rs:661`, the board card's status
buttons, `min-h-11 min-w-11` from `DEV-002`.

## 3. Where the count is soft, and I want your reading

I say "approximately 149" deliberately. Three things could move it, and you
will see them and I have not:

- **A class on a non-interactive element.** `input-sm` on a display-only
  field, if any exist, is not a touch target.
- **A control whose padding or a wrapper already lifts it past 44 px.** The
  resolved `height`/`min-height` is the box, but an ancestor with `py-*` and a
  clickable area could exceed it. Check whether any do.
- **`checkbox` inside a `<label>`.** If the label wraps the input and is
  clickable, the *target* is the label's box, not the 24 px input. That is a
  real and common pattern and it may take several of the ten out.

**Report the number you arrive at and how it differs from mine.** A count that
matches mine exactly, on a question this soft, would make me think it was not
checked.

## 4. Two pieces of work, both small

**4.1 — A test for the one compliant control.** `issues.rs:661` carries
`min-h-11 min-w-11` and **nothing asserts it**. The baseline claimed
`board_keyboard` verified `NFR-A11Y-007`; it does not, and I corrected that
today. Deleting those two classes today is invisible to all 209 tests.

Assert them, in `board_keyboard` where the control belongs. Demonstrate it
against a plant removing **one class at a time** — `STATUS-001`'s test 6 passed
against a defect because a compound plant hid it.

**4.2 — Nothing else.** No control resized, no class swapped, no guard added.
A guard here would have to encode a rule the owner has not yet decided; see §5.

## 5. The question this feeds, which is not yours or mine

`SPEC §33.2` says 44 × 44. That is stricter than WCAG 2.2's **AA** criterion
(2.5.8: 24 × 24, with a spacing exception); 44 px is 2.5.5, which is **AAA**.

Raising 149 controls to 44 px changes this product's density fundamentally. A
Kanban card whose status buttons are 44 px tall is a different card, and this
is a tool whose screens are dense on purpose.

So the owner chooses between raising the controls and amending `SPEC §33.2`,
and the choice should be made against numbers. **What would help most is the
part I cannot supply**: for each class, say what raising it would actually do
to the screens it appears on — which layouts get taller, which wrap, which
tables stop fitting. You will be reading those components anyway.

**Do not recommend an answer.** Describe the consequence per class and stop.

## 6. Escalate rather than deciding

- If §2's counts or resolved heights do not reproduce, stop.
- If the `checkbox`-inside-`<label>` case takes a large share out, say so
  prominently — it changes the size of the decision, not just the number.
- **If you find a control that is already unusably small on a phone** —
  overlapping neighbours, a 24 px target with no spacing around it — flag it
  separately from the count. That is a live defect, not a threshold question.
- If any class turns out to be used only on non-interactive elements, that is a
  finding worth its own line.

## 7. Acceptance

1. §2 reproduced, both halves, with the source of the resolved heights named.
2. §3's soft cases examined and the arrived-at number reported with its
   difference from mine.
3. §4.1's assertion present, demonstrated against a one-class-at-a-time plant.
4. Per-class consequence description, no recommendation.
5. No control resized.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. §3's number with its reasoning, and §5's consequences as prose
per class. The plant transcript.
