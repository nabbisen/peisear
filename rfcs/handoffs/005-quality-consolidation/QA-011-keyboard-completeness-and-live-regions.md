# QA-011 — Keyboard completeness, and a live region that is the wrong politeness

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: **P0** for §3 — `NFR-A11Y-001` is P0 and has read *Partial*
since 0.19.1. P1 for §2.
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §5
**Depends on**: nothing.

---

## 1. Read RFC 005 §5 first — it was rewritten today

The version you may have seen described `j`/`k` selection movement, a `?`
shortcuts modal, and a new `static/keynav.js`. **Do not build any of that.**

That is `NFR-A11Y-009` — *"SHOULD provide list navigation shortcuts"*, **P3**.
The requirement Phase E owes is `NFR-A11Y-001`: *"Every primary flow MUST be
completable with the keyboard alone"*, **P0**. A section that built the
convenience and never audited the completeness was inverted, and it has been
sitting that way since the RFC was written.

**`NFR-A11Y-009` is deferred out of Phase E.** If you find yourself writing a
key handler, stop.

## 2. The live finding — conflicts are announced politely

`components/issues.rs:117` renders one live region:

```html
<div id="status-announcements" role="status" …></div>
```

`role="status"` is a **polite** region. Both `board.js` and `dm.js` announce
**everything** through it — `announce(copy.movedTo[targetStatus])` for a
success, and `announce(copy.conflictMessage)` for a `409`.

`NFR-A11Y-008` (P1): *"Dynamic changes MUST be announced through an appropriate
live region; conflict notifications MUST use an **assertive** region."* Its
status read "Deferred with Phase D" — D-1 and D-2 shipped at 0.25.0 and 0.26.0,
so it is in force now.

**Confirm this before changing anything.** Read `issues.rs:117`, then
`announce()` in both scripts, then every call site.

**The design problem is one region serving two politenesses.** *"Moved to
Done."* should not interrupt; *"Another member changed this issue first."*
should, because the user's action did not take effect and the page changed
under them. A single region cannot be both.

**Two regions, then** — a polite one for success, an assertive one for
conflict and unavailable. Both server-rendered, both empty, both in the same
component. The scripts choose by outcome, which they already distinguish
(`STATUS-002` split conflict from unavailable and this project reviewed that
split twice).

**What to check that I have not:** whether `settings.rs:60` is the same class.
It renders `<div role="alert" … aria-live="polite">`, explicitly overriding the
assertive politeness `role="alert"` implies. It renders on a **fresh page load
after a redirect**, where a live region does little either way — so I think it
is defensible and possibly deliberate. **Nothing records why.** Determine
whether it is right and write the reason down; do not change it just to match
§2's rule, because it may not be the same situation.

## 3. The audit — `NFR-A11Y-001`, per primary flow

The requirement names four flows. Audit each **end to end with the keyboard
alone**, and record what you did, not what the markup suggests:

| # | Flow |
|---|---|
| 1 | List → detail → back |
| 2 | Edit → save, and edit → cancel |
| 3 | Marking a notification read |
| 4 | Changing an issue's status, on all three surfaces |

For each: every step reachable by `Tab`, every control operable by
`Enter`/`Space`, focus visible throughout, and no keyboard trap.

**This is a reading-and-driving audit, not a test-writing exercise** — the
harness drives HTTP and cannot press `Tab`. Say plainly which flows you
exercised in a browser and which you assessed from markup. **An assessment from
markup is not an audit and must not be reported as one.** If you cannot drive a
browser, say so and report the markup assessment as exactly that — it is still
useful and it is not the same thing.

**What is testable, and should be tested**: any structural precondition the
audit finds — a control that is a `div` where it should be a `button`, a form
input with no label, an `aria-hidden` on something focusable, a missing
`type="submit"`. `STATUS-001`'s test 6 passed with `type="button"` and the
defect was found only by planting one attribute at a time; expect that shape.

**Do not fix and audit in the same pass.** Produce the table first, then fix,
so the review can see what was found separately from what was changed.

## 4. Not in scope

- **No `keynav.js`, no `j`/`k`, no `?` modal.** §1.
- **No fourth JavaScript file at all.** `§10.15` — the shipped scripts are
  executed by no test, and `QA-003` named "a fourth file added later gets no
  reference test automatically" as an open residual.
- **No chart work.** `NFR-A11Y-003` is `§10.4` and RFC 008's, unwritten.
- **No mobile or touch-target work.** That is §6, a separate section.

## 5. Escalate rather than deciding

- **If §2 does not reproduce** — if conflicts already reach an assertive region
  by some path I did not read — stop and report.
- If the audit finds a flow that **cannot** be completed by keyboard, stop and
  report before fixing. A P0 completeness failure is a finding the owner should
  see before it moves.
- If `settings.rs:60` turns out to be the same class as §2 after all, say so
  and treat it the same way.
- If two regions turn out to need a change to `dm.js`/`board.js` beyond
  choosing a target element, report what and why before writing it.

## 6. Acceptance

1. §2 reproduced, then two regions, with the scripts choosing by outcome; the
   `settings.rs` question answered either way with its reason recorded.
2. §3's table complete for all four flows, with browser-driven and
   markup-assessed steps **labelled differently**.
3. Structural preconditions found by the audit covered by tests, each
   demonstrated failing against a one-attribute plant.
4. Findings reported separately from fixes.
5. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 7. Required review-request format

Workflow §9.2. §3's table with its two kinds of row plainly distinguished. Each
plant transcript separately.
