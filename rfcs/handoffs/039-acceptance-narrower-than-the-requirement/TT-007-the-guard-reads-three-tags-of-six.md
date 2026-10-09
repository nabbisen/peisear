# TT-007 — the guard reads three tags of six

**Register**: `§10.35` (opened for this), `§10.19`, `§10.28`.
**Requirement**: `NFR-A11Y-007`. **Target**: 0.46.0.

`NFR-A11Y-007` says **every interactive element** presents a 44 × 44 px touch
target — `DEC-050` removed the named limit *"because it was measured and it was
not narrow"* — and cites `touch_target_scan` as its mechanism. **That guard
reads three tag names.**

```rust
const TAG_NAMES: [&str; 3] = ["a", "button", "summary"];   // touch_target_scan.rs:649
```

Measured in `components/` on 2026-10-09: **161 elements inside that
population, 107 outside it** — `<input>` 70, `<select>` 28, `<textarea>` 9.
**Two of every five.**

**This is not an emergency and you should not treat it as one.** Of the 107:
**32 are `type="hidden"`** and are not touch targets; **58 declare a target
anyway** through `grow()`; **17 are visible with none** — 3 inputs, 5 selects,
and **all nine `<textarea>`s**.

---

## §1 — count by class before changing anything

`DOCS-002` is the precedent and it is the right one: the count came in wrong
twice there because it was taken from an aborting run. **Produce your own
numbers and do not trust mine.**

My 17 is a **heuristic**: `grow(` or `TOUCH_TARGET` appearing inside the
element's opening tag, scanning 600 characters forward. It bounds the
candidates; it does not name failures. Specifically it will be wrong about:

- an element whose classes are built in a `let` above the tag;
- an element inside a wrapping `<label>` that provides the hit area, which
  `NFR-A11Y-007` **explicitly permits** — *"a control may satisfy this by
  expanded hit area rather than by visible size"*, through an element that
  **participates in layout**;
- a `<textarea>`, which is multi-line and will usually pass on height while
  the heuristic says nothing about height at all.

**Report a table**: per tag, how many are hidden, how many declare a target,
how many are satisfied by a wrapping label or ancestor, and how many are
genuinely short. **Then** we know what the work is.

## §2 — widen the population

Add `input`, `select` and `textarea` to `TAG_NAMES`, and handle what follows.

**Three things the guard must get right that it does not face today:**

1. **`type="hidden"` is not an interactive element.** It must be excluded, by
   reading the `type` attribute rather than by an exception list — an
   exception list of 32 entries is not a list, it is a second implementation.
   **If the attribute is not statically readable at some site, report that
   site** rather than adding it to an exception list.
2. **`<input>` and `<select>` are void or self-closing in this codebase's
   markup.** The existing scanner finds a tag's span; check it handles `/>`
   and an attribute-only element without a closing tag. `§10.19` and `§10.28`
   are both this guard being wrong about its own rules.
3. **The wrapping-label case must be recognised or declared.** If the guard
   cannot see that an ancestor `<label>` provides the hit area, say so and
   propose the shape — an explicit declaration at the site is better than a
   guard that reports false failures, because a guard people learn to override
   is worse than no guard.

**Do not grow the exception list quietly.** It is *named and counted* today —
one site — and that is why `NFR-A11Y-007` can say what it says. **Every
addition is a decision and comes back to me**, with the reason, before it
lands.

## §3 — resolve the 17 (or however many you find)

For each: either it reaches 44 × 44 (apply `grow()`, or confirm an ancestor
provides it), or **it is a declared exception with a reason in the guard and
in the requirement.**

**Measure, do not assume.** `grow()` appends `min-h-11 min-w-11`
(`components.rs:139`) — a declaration, not a measurement. The `TT-004`
precedent and `NFR-A11Y-007`'s own words are clear that the guard proves *a
declaration is present*, not that a control renders at 44 px. **For any site
where the declaration is new, measure the rendered box in a browser and name
the browser.** `CAL-005` did this and it is why its figure is trustworthy.

**The nine `<textarea>`s are the interesting ones.** A multi-line box is
almost certainly over 44 px tall; the question is whether it is over 44 px
**wide** on a 320 px phone, and whether declaring `min-w-11` on something
already `w-full` changes anything. If the answer is *they all pass and need no
declaration*, that is a fine answer — **say it with the measurement**, and
then the guard needs to not fail them.

## §4 — gates

- **`DEC-007`** three consecutive runs. The count will rise.
- **The guard seen to fail**, per this project's standing discipline: plant a
  short `<input>` without a declaration, watch
  `every_interactive_element_declares_a_touch_target` fail, restore
  byte-identical, and quote the failure. A widened guard that has never been
  seen to fail on a newly-covered tag has not been shown to cover it.
- **The overflow gate at five widths**, if any component's markup changes.
  **120 cells.** Adding `min-w-11` to something in a narrow column is exactly
  how `§10.25`'s shape appears.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.
- Both browser scripts if components change.

## §5 — escalate rather than deciding

- **If the real count differs materially from my 17** — in either direction.
  Mine is a heuristic and yours is a measurement.
- **Before adding any exception**, with the reason.
- **If a site needs a wrapping-label declaration mechanism** that does not
  exist — that is a design question, not a patch.
- **If widening the guard would fail a site that passes in a browser**, which
  means the guard's rule is wrong rather than the site.

## §6 — exit condition

`TAG_NAMES` covering all six tags with `type="hidden"` excluded by attribute;
every newly-covered element either declaring a target, satisfied by a measured
ancestor, or a named exception with a reason; the guard **seen to fail** on a
newly-covered tag; the counted-population sentence in `NFR-A11Y-007` true
again — which is mine to write, on your table.

**`§10.35` does not close on this.** It closes when `REQ-004` reports, because
one instance found by accident says nothing about the rest.
