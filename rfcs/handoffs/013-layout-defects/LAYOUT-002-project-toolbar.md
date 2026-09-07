# LAYOUT-002 — the project-detail toolbar does not fit a phone

**Target release**: 0.32.0
**Source**: `LAYOUT-001`'s escalation, §5 — found while carrying out that
handoff's own verification, and correctly refused as out of its scope.
**Governing RFC**: none. Defect fix.

## 1. The defect

**Project detail overflows horizontally by 6 px at 390 px**, and by nothing at
768, 1280 or 1920. The outermost offender is the view/action toolbar in
`components/issues.rs` — `<div class="flex items-center gap-2 shrink-0">`
carrying Board / List / Calendar / Edit / New issue — measured **5.7 px** past
the viewport.

The surrounding row (`flex flex-wrap items-start justify-between gap-3`) already
wraps. **The inner toolbar does not**: it has no `flex-wrap`, and `shrink-0`
forbids it from shrinking, so its five controls stay on one line at any width.

## 2. Two things established by measurement, so nobody re-derives them

**It is not caused by RFC 012.** I suspected the `min-w-11` that `grow()` adds
to each button had pushed the toolbar's minimum width past the viewport.
**Measured on a running instance: stripping `min-w-11` from all ten elements
leaves the overflow at exactly 6 px, and stripping `min-h-11` as well changes
nothing.** The touch-target work is not implicated. It is a pre-existing
inability to wrap.

**`flex-wrap` fixes it.** Adding `flex-wrap` and removing `shrink-0` on that one
container, applied live: **overflow 6 px → 0 px.**

That is the whole diagnosis. **Confirm both before relying on them** — they were
measured in one sitting and one of the two contradicted what I expected.

## 3. What to do

Let the toolbar wrap below the point where it stops fitting.

External design `§5.6` already says what narrow screens do: *"< 640 px — single
column; tables become stacked rows."* A toolbar that refuses to wrap is the
exception to a rule the document already states, so this is bringing one
container into line rather than inventing a policy.

**`flex-wrap` plus dropping `shrink-0` is the measured fix**, but do not take it
on my say-so: `shrink-0` was presumably added for a reason, and if removing it
lets the toolbar compress in a way that looks wrong at some width between 390
and 1920, **say so and propose the alternative** — a wrap at a breakpoint, or
`min-w-0` on the sibling heading block instead.

## 4. Verification

`.git-exclude/tools/cdp.mjs` — the same harness `LAYOUT-001` used; its README
has the invocation.

- **`scrollWidth - clientWidth` is 0** on project detail at **390, 768, 1280 and
  1920**. Report the numbers.
- **The four other pages from `LAYOUT-001` stay at 0** at all four widths.
- **The toolbar's five controls remain reachable and operable** at 390 px —
  wrapping must not push anything off-screen or under another element.
- **Each still presents a 44 px target** — `NFR-A11Y-007` applies to them and
  they carry `grow()` today. A wrap that shrinks them fails the requirement.

**No test.** Nothing in the suite can observe this. Do not add a browser to CI —
that is RFC 011 step 4's open decision.

## 5. Escalate rather than deciding

- **If `shrink-0` turns out to be load-bearing** at some width.
- **If wrapping puts the primary action ("New issue") somewhere it reads as
  secondary.** That is a design question about the screen, not a layout fix.
- **If the 6 px turns out to have a different cause than §2 says.**

## 6. Exit condition

Zero overflow at four widths on five pages, the toolbar operable at 390 px with
its targets intact, `DEC-007` clean, three consecutive `cargo test --workspace`
runs.

---

**Who holds what**: dev team — the fix. **What's blocked**: nothing. **What's
next**: review request.
