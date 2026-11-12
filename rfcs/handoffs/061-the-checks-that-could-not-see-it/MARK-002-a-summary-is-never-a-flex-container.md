# MARK-002 — a summary is never a flex container

**Series**: `061`, `0.63.0`'s scope **as re-authorized 2026-11-12**.
**Register**: `§10.58`. **Supersedes** the "fix the other four" shape
this round was first imagined as.

## 1. Why this is a rule and not four repairs

Ten of fourteen `<summary>` elements shipped without a marker, by **two
different routes**: `grow_text(`, and a literal `flex` / `inline-flex` /
`btn` inside `grow()`'s own argument. `MARK-001` fixed six and added a
scan for the **first** route; planted with the second, that scan passes.

**Fixing the remaining four the same way would buy the same thing
again**: four sites correct, one more shape checked, and the next
`<summary>` still free to compose a display class because nothing stops
it. The defect is not a set of call sites. It is that **a disclosure
control's affordance can be destroyed by a styling decision taken for an
unrelated reason, and no rule says otherwise.**

So the rule, and it is short:

> **A `<summary>` is never a flex container. If its contents need
> layout, the layout goes on a child.**

## 2. What to build

**`grow_summary(base)` — the only helper a `<summary>` may take.** It
appends the touch-target floor and symmetric padding, exactly as
`DISC-001` worked out by measurement, and it **never** appends a display
class. `grow_text()` and `grow()` keep their own jobs for everything
that is not a `<summary>`.

**Convert all thirteen real summaries to it** — the nine already correct
and the four still markerless. For the three that genuinely need two
children side by side (`notification_preferences.rs`, `teams.rs`, and
`me.rs`'s Rhythm label), **move the `flex` row onto one inner `<div>`**
and leave the summary itself a list item. Those three are the only ones
where this is more than a rename; they are measured, not guessed, and
their layout must look the same afterwards at both widths.

**`settings.rs`'s `btn btn-ghost btn-xs` summary is the hard one.**
`.btn` sets `display:inline-flex` in the vendored sheet, so it cannot
stay on the `<summary>` itself. Either move the button's appearance to
an inner element, or decide that control should not look like a button —
**say which you chose and why, and show the before and after**. If
neither works without changing what the control communicates, **stop and
report**: that is a design question and it is the architect's.

## 3. What this deletes, which is the point

- `alignment_scan::is_padding_centred_disclosure` — the exclusion
  `DISC-001` introduced and `MARK-001` widened — **goes away entirely**.
  Once no `<summary>` reaches `grow()`, nothing needs excluding.
- `alignment_scan::no_summary_composes_grow_text` is **replaced**, not
  kept beside the new one: the property *"every `<summary>` composes
  `grow_summary(`"* implies it and covers the shape it misses.

**A round that adds a rule and leaves both old exceptions behind has not
done this.** The measure of this round is that the tree gets *smaller*.

## 4. The one exception, named rather than listed

`issues.rs`'s *"How this is calculated"* toggle takes no helper, is
deliberately below 44 × 44, and is already a named escalation exclusion
in `touch_target_scan`. The new scan carries **exactly that one
exception, by cross-reference to the guard that already owns it** — not
a new list of its own. **If a second exception ever appears, the rule is
wrong and should be brought back to the architect**, not extended.

## 5. Acceptance

- All thirteen render `display: list-item` with a marker, **measured in
  a browser at 390 and 1280**, each ≥ 44 × 44, and
  `disclosure-marker-gate` extended to every page carrying one — report
  the page list you derived, not the one this handoff guesses.
- The three two-child summaries look the same after the change:
  screenshot or measured box comparison, before and after.
- **Two plants**: `grow_text(` on a summary, and a literal `inline-flex`
  inside its helper argument. **Both must fail by name** — the second is
  the one `MARK-001`'s scan could not see, and it is the reason this
  round exists.
- `touch_target_scan` and `css_scan` stay green; `DEC-007` from → to.
- `alignment_scan` is **shorter** than it was.

## 6. What this round must not do

- Do not touch the escalation exclusion, or lower any touch target.
- Do not change what any disclosure contains or what is open by default.
- Do not introduce a general "display class" linter for every element —
  the rule is about `<summary>` and the affordance it owns.
- Do not amend `requirements.md`; `§10.58` records this already.
