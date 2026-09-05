# TT-002 — apply the touch-target rule

**Governing RFC**: [012](../../accepted/012-touch-target-conformance.md),
step 3, and `DEC-049` **as amended after `TT-001`**
**Target release**: 0.31.0
**Depends on**: `TT-001` — delivered and reviewed. **Read the review**
(`.git-exclude/reviewed/TT-001-review.md`) before this handoff: it withdraws
six of the audit's own recommendations.

## 1. Scope

**139 controls reach a 44 px target.**

| | Count | Mechanism |
|---|---|---|
| Controls carrying a sizing class | **136** | **Grow** |
| Checkbox controls (`notification_preferences.rs`) | **3** | **Expand, by `<label>` wrap** |

**`Grow` everywhere else. There is no other `Expand` in this handoff.**
`TT-001` recommended ten; the review withdrew six (`settings.rs` ×2,
`calendar.rs` ×4) and resolved the four escalations to `Grow`. If you believe a
surface needs `Expand`, that is an escalation (§6), not a local choice.

**No gap value changes.** `TT-001` §3.1 reported two overlap hazards; both exist
only under `Expand`. Under `Grow` the layout engine keeps the boxes apart at any
positive gap, including `gap-1`. **If you find yourself widening a gap, stop —
something has gone wrong with the mechanism choice.**

## 2. The 44 px fact gets one home

**Do not write `min-h-11 min-w-11` 136 times.** That is 136 copies of one fact,
which is the defect `RFC 006`, `QA-019`, `HLT-001` and `JS-003` each existed to
remove. This project's rule is *move the fact to where it can be checked*, and a
magic pair of utility classes scattered across fifteen files is the opposite.

**Give it one name in Rust** — a `const` in `components/` — and compose class
strings from it. `TT-003`'s guard then checks for **one symbol** rather than
pattern-matching a class pair, and a future change to the target size is one
edit.

**If composing the class string makes the markup materially worse** — Leptos
`class=` attributes are mostly literals, and `format!` at 136 sites may read
badly — **say so and propose the alternative before doing it 136 times.** A
constant that makes every call site uglier is a real trade and I would rather
hear the argument than receive it applied. What is not acceptable is 136 bare
literals with no single home.

## 3. What is proven, and what is not — read this before touching an input

Verified from the pinned bundle (`.git-exclude/tmp/daisy.css`, the convention
`TT-001` established):

**Buttons — proven.** `.btn-sm{height:2rem;min-height:2rem}` and
`.btn-xs{height:1.5rem;min-height:1.5rem}`. A `min-height` utility resolves
above both, and **three controls already ship this way** — `confirmation.rs:53`,
`:58`, and the status button in `IssueCard`. `.btn` is `inline-flex` with
centred items, so a taller box keeps its label centred. Apply and move on.

**Inputs and selects — NOT proven, and this is the trap.**

```
.input-sm  { height:2rem;   ...; line-height:2rem }
.input-xs  { height:1.5rem; ...; line-height:1rem; line-height:1.625 }
.select-sm { height:2rem; min-height:2rem; ...; line-height:2rem }
.select-xs { height:1.5rem; min-height:1.5rem; ...; line-height:1rem }
```

**Each pins `line-height` to its own old height.** Grow the box to 44 px and the
line box stays 32 px or 24 px. Whether the text still sits centred is a
**rendering** question, and it depends on the element: `.select` is `inline-flex`
(likely fine), a text `<input>` is not (browser-dependent).

**This project cannot answer a rendering question** — that is `§10.15` /
external design `§17.6`, and it does not resolve before 0.32.0.

**So:** determine the correct utility pairing for inputs and selects and
**justify it from the pinned CSS**, not from how it looks in your editor. If the
justification requires knowing how a browser lays out a text input whose
`line-height` is shorter than its box, **that is exactly the escalation in §6**
— say so rather than guessing, and say what you would need.

**57 of the 139 are `input-*` or `select-*`.** This is not a corner case.
*(Corrected 2026-09-05: this read 65, which was wrong — `input-sm` 29 +
`select-sm` 21 + `input-xs` 5 + `select-xs` 2 = 57. The figure propagated
into the review record and the 0.31.0 changelog before it was re-derived.)*

## 4. The three checkboxes

`.checkbox{height:1.5rem;width:1.5rem}`, and `TT-001` confirmed they are **not**
`<label>`-wrapped — the bare 24 px box is the target, which is why they are
genuine failures.

Wrap each in a `<label>` that reaches 44 px, keeping the box at 24 px. This is
the sanctioned `Expand` under `DEC-049` as amended: the label **participates in
layout**, so it keeps the container gap's protection and stays guardable. A 44 px
checkbox glyph would be wrong; a 24 px box inside a 44 px label is correct.

**The `aria-label` on each input must survive.** They name the notification kind
and channel, and `QA-009` is this project's precedent for `aria-label` variants
going unread by a guard that was believed to cover them. If wrapping changes how
the accessible name is computed, **say so** — that is an accessibility
regression dressed as an accessibility fix.

## 5. Re-derive `TT-001` §3.4's inspection-only list

`TT-001` listed several clusters as inspection-only. **Most of that list was
built while the mechanism was still open**, and under `Grow` inside a positive
gap they are now covered by `DEC-049`'s clause (4) instead.

Re-derive which genuinely remain inspection-only after this handoff's mechanism
choices, and report that shorter list. **Do not carry the old list forward** —
an inspection-only list that is longer than it needs to be reads as unresolved
risk and will be treated as such by whoever writes `TT-003`.

## 6. Escalate rather than deciding

- **The input/select line-height question (§3)**, if it cannot be settled from
  the pinned CSS.
- **Any surface where `Grow` genuinely breaks the layout** — not "makes it
  taller", which is expected and accepted, but *breaks*.
- **If the `<label>` wrap changes the checkboxes' accessible name.**
- **If the single-constant approach (§2) makes the call sites materially
  worse** — propose, do not apply.
- **If `TT-001`'s inventory is wrong anywhere.** It was reviewed, not re-run.

## 7. Tests

`TT-003` writes the exhaustive source guard. **This handoff proves the pattern
renders**, which a source scan cannot:

1. **One integration assertion per mechanism.** A rendered page carrying a grown
   button, one carrying a grown input or select, and the notification-preferences
   page carrying a wrapped checkbox — asserting the rendered markup, not the
   source.
2. **One assertion that the constant is the source of the value** — plant a
   changed constant and confirm the rendered output changes with it. A constant
   nothing observes is 136 literals with extra steps.

**Plant each separately.** One at a time — `STATUS-001` test 6, and `QA-004`,
`QA-005`, `QA-009`, `JS-002` and `JS-003` since.

**Do not add a guard that fails on the current tree.** `TT-003` lands after
this, for exactly that reason.

## 8. Exit condition

139 controls at a 44 px target. `DEC-007` clean, three consecutive workspace
runs. The re-derived inspection-only list from §5. And a statement of **what
`TT-003` can now check** — the guard is written from that, not from this
document.

**One thing that is deliberately not in scope**: interactive elements carrying
no sizing class — plain `<a>` links, breadcrumbs, whole-card links. `TT-001` §5
established they sit **outside** the counting method rather than inside it and
passing, and `NFR-A11Y-007` now records that as a named limit. **Do not extend
coverage to them here.** It is a separate decision and folding it in silently
would make the limit untrue.

---

**Who holds what**: dev team — the application. **What's blocked**: `TT-003`.
**What's next**: review request; then `TT-003` is written from §8's statement.
