# CAL-005 — the window becomes a date

**Review:** `.git-exclude/reviewed/CAL-004-review.md` (§4, §5)
**Round 1:** `fa9d2ab`, accepted. 385 green, the identical-stored-row test
passing, the lock agreeing across all three paths, both false comments
corrected. **The handler is not changing.**

**Your §5 escalation was right, and the answer is not the one it anticipated:
the fix is to stop having a window, not to widen one.** The reasoning is in
the review's §4, including an answer to your own comment's argument for a
bounded window — read that before starting, because it is the part you may
disagree with.

---

## §1 — the substitution

Replace the day `<select>` with a native date input:

```html
<input type="date" name="target_date" value="…">
```

**`change_schedule_move` does not change.** It already takes
`body.target_date` and parses `%Y-%m-%d` (`issues.rs:1369`), which is exactly
what `type="date"` submits. The delta arithmetic, the lock, the redirect and
every test from round 1 stand.

**What goes:**

- `day_options` from `ScheduleMoveContext`
- the option loop at `handlers/issues.rs:719-722`
- the `window_days` call at `:718` and the `parse_view`/`parse_anchor` calls
  **only if nothing else needs them** — check, because `from_view` is still
  parsed for the return path (§2)
- the option rendering in `components/issues.rs`

**What stays:** `from_view`/`from_date`/`from_surface` threading and the
server-side redirect rebuild. **`FR-NAV-005` still needs it** — the context
is carried to return the user where they came from, which is the only job it
ever had to do that survives.

**`value`**: the issue's current `planned_start_at` day is the sensible
default — it makes the field show where the block is now, and a user changing
one field sees what they are changing from. If you think an empty field is
better, say which you chose and why.

**No `min` or `max`.** The edit form accepts any `datetime-local` today;
bounding this input would invent policy this work has no mandate for, and the
server validates through the same path either way.

## §2 — the comment that carries my wrong figure

`handlers/issues.rs:710` reads *"one day for a day-view visit, **up to 35 for
month**"*. **That figure is mine and it is wrong** — the grid pads to whole
weeks and can show 35 cells; `window_days(Month, …)` returns 28 to 31, as
your own report measured. §1 deletes most of that comment anyway.

**Whatever survives must not carry the figure**, and should say what is now
true: the target is **any** day, and the calendar context is carried **only**
to return the user to the view and date they came from. One or two sentences.

## §3 — the gates, and the one place §1 is not free

- **`NFR-A11Y-007`, re-measured.** The control's geometry changes when a
  `<select>` becomes a date input, and round 1's `113 × 44` does not carry
  over. Measure the rendered input and button again: 44 × 44 minimum, no
  overlap. **A date input's rendered width is browser-dependent** — report
  what you measured and in which browser.
- **The overflow gate, re-run at five widths.** It passed at **120 cells**
  with the old control; this changes the control on a page the fixture
  sweeps. §1 should *reduce* the risk — a fixed-width input cannot stretch
  the way a `<select>` sized to "Thursday 9 October" can — but a reduction
  argued is not a reduction measured.
- **`DEC-007`** three consecutive runs. **Expect the count to fall**: round
  1's day-count tests (the day/week/month option assertions) are testing a
  thing that will no longer exist. **Delete them rather than adapting them**
  — an assertion kept alive past its subject is `§10.17`. Report the new
  figure and name every test you removed.
- `fmt`, `clippy -D warnings`, `rustdoc-links`.
- **Both browser scripts**, since a component changes.
- **The identical-stored-row test must still pass**, unchanged. If it needed
  editing, something in §1 went further than specified — stop and report.

## §4 — escalate rather than deciding

- **If `NFR-A11Y-007` fails** on the date input in any browser you can
  measure.
- **If the overflow gate moves off 120.**
- **If you disagree with §4 of the review** — your comment at
  `issues.rs:713-714` argued for a bounded window, and the review answers it
  rather than overriding it. If the answer does not persuade you, say so with
  your reasoning; I would rather re-decide now than ship a control you think
  is worse.
- **If removing the option list turns out to break the return path**, which
  would mean `from_view` was doing two jobs and only one was documented.

## §5 — exit condition

A date input in place of the day `<select>`; the handler untouched; the
figure-carrying comment corrected or gone; `NFR-A11Y-007` re-measured and the
overflow gate re-run at 120; `DEC-007` three times at the new figure with
every removed test named; and the identical-stored-row test still green
without edits.

**`FR-DM-002` reaches Met on this**, which is the first P0 this project has
closed on a clause written specifically to make it falsifiable. Recording
that is mine, on your evidence.
