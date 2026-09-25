# A11Y-002 — an arrow key commits a role change

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.41.0
**Related requirements**: `NFR-A11Y-002`, `FR-TEAM-003`.
**Source**: `A11Y-001` §0, measured. **Depends on**: nothing. **First.**

---

## 1. The defect

`components/teams.rs:468`:

```rust
<select name="role" onchange="this.form.submit()" …>
```

**`change` fires on every arrow key.** A keyboard user moving Admin → Viewer
passes through Member, and **the page submits the demotion to Member** before
they reach Viewer. Measured: `ArrowDown` navigates, focus lands on `body`.

**The intermediate state is a real change to a real person's access**, written
to the database and visible to them. The last-admin rule protects the floor,
not the step; `RACE-001` made that guard atomic, which does nothing here.

**Why this one and not the sprint `<select>` on issue detail**: that one has a
*Save* button. This is the only place a navigation key commits a state change.

## 2. The fix

**Give it an explicit commit**, the shape the rest of the product already uses:
a *Save* (or *Change role*) button beside the select, submitting the same form
to the same route with the same message keys.

- **The no-JavaScript path must keep working** — it does today, because the
  attribute is an enhancement over a real form. **Confirm it still does** with
  scripting off; `DEC-021` is the rule and `§10.15` is why it gets checked
  rather than assumed.
- **`onchange` comes out entirely.** An enhancement that submits is not one.
- **No new message key** unless the button genuinely has no existing label to
  use; if it does need one, say so rather than composing a string.
- **Do not change the route, the form fields, or the redirect.**
  `the_last_admin_is_refused_with_the_same_messages` and
  `two_admins_demoted_at_once_leave_one` pass unmodified.

## 3. Verification

- **A keyboard walk, measured**: focus the select, `ArrowDown` twice, confirm
  **nothing is submitted** and the role in the database is unchanged; then
  `Tab` to the button and `Enter`, and confirm the role changes once, to the
  chosen value. **Report both, with the values.**
- **Focus after the commit**: it is a native POST, so `body` is expected —
  `NFR-A11Y-009`, not this handoff. **Say where it lands**; do not fix it here.
- Scripting off: the form still submits and refuses the last admin.
- The touch-target floor holds on the new control.
- `DEC-007`: report the new count, last recorded **364**. A rendered test that
  the select carries no `onchange` and a submit control exists is worth one.
- `fmt`, `clippy`; **the overflow gate** — a control was added to a page it
  sweeps.

## 4. Escalate rather than deciding

- **If any other `<select>` or input commits on change.** `A11Y-001` found one;
  report a second rather than fixing it here.
- **If the button cannot go beside the select** without disturbing the members
  table's layout at 320 px.
- **If an existing user-visible string changes.**

## 5. Exit condition

No keyboard interaction with the role control changes a role until an explicit
commit; the no-JS path is unchanged; the last-admin guard and its tests are
untouched.
