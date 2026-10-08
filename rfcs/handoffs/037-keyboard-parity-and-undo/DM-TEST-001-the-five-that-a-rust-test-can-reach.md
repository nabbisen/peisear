# DM-TEST-001 — the five a Rust test can reach, and one that needs the browser

**The measurement is closed** (`.git-exclude/reviewed/FR-DM-002-measurement-review.md`)
and its findings are amended into the specification (`df2fa9a`). This handoff
builds the tests your own §4 proposed, **with two of the seven dropped and one
deferred on the record** so the reasons are not re-litigated later.

**Background**: `FR-DM-002` (P0, Partial), `FR-DM-003`, `FR-DM-004`,
`FR-DM-006` (Met) as amended at `df2fa9a`; `§10.15`; `§10.17`; `DEC-007`.

**Do not amend either specification.** Report, and I amend.

---

## §1 — what is in, what is out, and why

**In (five):**

1. **Sprint plan field parity.** Assert the rendered hidden-input set per row:
   **add has five** (`issue_id`, `project_id`, `project`, `priority`,
   `assignee`), **remove has four** (no `project_id`, because
   `PlanRemoveForm` does not declare it). Markup assertion.
2. **Sprint plan stored-row equivalence.** POST `/plan/add` then
   `/plan/remove` with exactly the row form's fields; assert the
   `sprint_issues` membership appears and disappears, **and that
   `issues.updated_at` is untouched by both**. Your curl sequence ports
   directly.
3. **Calendar lock parity.** A stale `client_updated_at` through **both**
   `update` and the `/schedule` endpoint returns the **same** conflict
   status. This is the one that earns its keep: `-004`'s agreement between
   those two call sites exists **only because two authors made the same
   choice**, which your own §2 established. Pin it.
4. **Undo DOM order, all four surfaces.** Assert that the undo control is the
   acted-on element's own next focusable sibling — a **DOM-order assertion**,
   not a Tab-walk. `FR-DM-006` is now Met on the strength of a reading; this
   is what keeps it Met.
5. **`can_move = false` yields neither affordance.** Assert a row that cannot
   move renders **no** `draggable`, **no** `data-plan-move` and **no** move
   form. One shared gate today; this asserts they cannot drift apart.

**Out (two), and these are rulings, not omissions:**

- **The calendar duration-shrink guard.** Dropped. Ruled **not a defect**: a
  pair of absolute `datetime-local` fields cannot carry *preserve the
  length*, and the ordering guard already rejects the incoherent case. There
  is nothing to assert.
- **A real drag gesture producing the same stored effect.** Deferred, not
  dropped. §1's structural finding (the drag's body *is* the form's body) and
  the calendar ruling took most of its value, and it is the most expensive
  item on your list. It stays recorded under `§10.15`.

**In, but browser-only (one):**

6. **The `mousedown`-focus trap**, parameterised over the three draggable
   surfaces. `A11Y-005` is the precedent that this class needs **real pointer
   events** — a `mousedown` plus a small movement on the undo button must
   still fire `click` and not begin a drag. One script in `browser-checks`,
   three pages.

## §2 — the trap I am most worried about

**Four of the five Rust tests assert on rendered markup, and markup
assertions are where `§10.17` lives.** A test that greps for
`name="project_id"` passes if the string appears **anywhere** on the page,
including inside a different row's form, a comment, or a `data-` attribute.

So for items 1 and 5, **scope every assertion to the row under test** — find
that row's form, then assert on its inputs — and **include a negative that
has been seen to fail.** Item 5 is a test about an absence; an absence test
that passes against a page where the thing is present is worthless. Before
you report, plant the affordance on a `can_move = false` row by hand, watch
item 5 fail, and restore byte-identical. **Say in the report that you did,
and what the failure said.**

Item 3's negative is free: a stale stamp must produce the conflict status on
**both** paths, so assert the status rather than just "not 200".

## §3 — `DEC-007` and the gate

If any of this lands in a **new** test file, `DEC-007` requires **both** the
command block in `.github/CONTRIBUTING.md` **and** a matching `test.yml` job
— and `dec_007_fs_scan` will fail until both exist, which is the intended
behaviour, not an obstacle.

**Prefer extending existing files** where the subject fits: items 1, 2 and 5
are sprint-plan concerns; item 3 is an optimistic-lock concern; item 4 spans
four surfaces and may genuinely want its own file. Your call, but say which
you chose and why.

Expect the inventory to **rise**. Report the figure and the delta, and if a
new file lands, confirm both halves of `DEC-007`.

## §4 — verification before reporting

- `DEC-007`, three consecutive runs, the new figure stable across all three.
- Item 5's planted failure, with the message, and a byte-identical restore
  shown by `diff`.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`, `rustdoc-links`.
- The overflow gate: **only if** a component's markup changed. If no
  component changed, say so rather than running it.
- For item 6: the script seen to fail against a deliberately broken
  `mousedown` handler, then restored. Same discipline as the Rust plant.

## §5 — escalate rather than deciding

- **If item 3 finds the two paths return different statuses** — that is a
  defect in a P0-adjacent guard and it stops being a test task.
- **If item 4 finds a surface where the undo control is not the next
  focusable sibling** — `FR-DM-006` was just recorded Met on all four, and a
  counter-example means I amended it wrongly for the second time. Report it
  that plainly.
- **If item 1 finds a field asymmetry** between what the form renders and
  what `plan.js` would send.
- **If any test cannot be written without asserting something the handoff did
  not ask for**, say so instead of widening it.

## §6 — exit condition

Five Rust tests and one browser-checks script, each with its negative
demonstrated; `DEC-007` three times at the new figure; the two dropped items
not written; and a report naming, per test, **what it would catch that
nothing catches today.** That last line is the one I will read first — a test
whose answer to it is vague is a test that passes for the wrong reason.
