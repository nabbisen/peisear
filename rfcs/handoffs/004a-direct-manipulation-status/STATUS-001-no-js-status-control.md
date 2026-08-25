# STATUS-001 — A status control that works without JavaScript

**Issued by**: Architect
**Date**: 2026-08-16
**Priority**: P1 — 0.25.0, **after CONF-001**
**Governing RFC**: [004a](../../done/004a-direct-manipulation-status.md),
step 1 only
**Depends on**: CONF-001 landed

---

## 1. Scope — step 1 only

RFC 004a is two steps. **This handoff is step 1.**

**In scope**: the issue detail page's status segment becomes a real form; the
issue list gains an equivalent per-row control. Both work with scripting
disabled. Both go through the existing shared lock check.

**Out of scope**: step 2 entirely — no click handler, no in-place update, no
undo toast, no change to `change_status`'s `204`. Step 2 gets its own handoff
after this is reviewed.

**Also out of scope**: the board. Its per-card form and `board.js` are untouched.
That is D-2's territory, not this one's.

## 2. Why step 1 is its own handoff, and shippable alone

Two surfaces gain a status control they have never had. If step 2 were never
written, nothing here is broken or half-built.

That is the test RFC 004a §D1 sets for whether the umbrella's requirement 0 was
honoured — not "does a no-JS path exist somewhere" but **"would we be content to
stop here"**. Build it so the answer is yes.

## 3. The issue detail segment

`components/issues.rs:1504–1531` currently renders three
`<button type="button">` elements with `tabindex="-1"`, `cursor-default`,
`aria-pressed`, and no handler. The comment says "they are inert by design".

They stop being inert:

- Wrap them in a `<form method="post">` posting the chosen status.
- Carry `client_updated_at` as a hidden input, as the board's form already does
  (`:621`).
- **`tabindex="-1"` and `cursor-default` go.** They exist to signal "clicking
  does nothing here", which stops being true.
- **`aria-pressed` stays** and keeps marking which status is current. That is
  the segmented-control semantics screen readers already get; do not replace it
  with anything.

**Do not attach a click handler.** Not even a small one. This handoff's entire
point is that the control works before any script exists.

## 4. The issue list

Each row gains an equivalent control. Every row, not on hover or focus — hover
does not exist on touch, and RFC 004a open question 3 is settled as every row.

The row currently renders status as text (`status_text`). Keep the current status
legible whatever shape the control takes; a row that shows only three buttons
and no plain answer to "what is this issue's status" is worse than the text was.

## 5. The route shape — settled, with room

RFC 004a open question 1 is settled as **(b): one form-POST route per surface**,
mirroring `/status/board`'s pattern, because it is the smaller change. **(a)** —
one generalised route deriving its redirect target server-side — is recorded as
the better shape for whenever something else touches these routes.

**Take (a) instead if it turns out cheaper in the writing.** State which you
took and why. Both are acceptable; an unstated choice is not.

Either way: **the redirect target is derived server-side from the route's own
parameters.** Never from a caller-supplied value — the same reason `CONF-001`
§3.3 gives about open redirects, and the same reasoning applies to a status form
as to a delete confirmation.

## 6. The lock

Use the existing shared `check_optimistic_lock`. **Do not add a second lock
check** — umbrella requirement 5, and RFC 009 §D1's lesson about one definition
applies to lock checks as much as to queries.

A stale `client_updated_at` returns 409 through that function, with the
established conflict wording. **No failure framing** — §1.7 and `FR-DM-005` both
forbid it, and the umbrella explicitly withdrew an earlier draft's
*"status couldn't change: stale data"* example.

## 7. Copy

Everything new through `peisear-i18n`; RFC 006 §D6 rule 7 included. `prose_scan`
will catch a literal, so there is no need to be careful — only to not fight it.

Reuse `MessageKey::IssueStatusName` for the segment labels; they already render
through it.

## 8. Tests

New target `crates/peisear-web/tests/status_control.rs`, with a CI job and a
`CONTRIBUTING.md` line.

| # | Check |
|---|---|
| 1 | POST the issue-detail status form directly → status changes, redirect lands back on the detail page |
| 2 | Same for an issue-list row → redirect lands back on the list, preserving any view parameters the list had |
| 3 | A stale `client_updated_at` on either → 409 |
| 4 | `aria-pressed` still marks the current status on the detail segment |
| 5 | The detail segments are keyboard-reachable — no `tabindex="-1"` |
| 6 | **Regression guard**: neither surface's control depends on script. Assert the rendered markup is a form with submit buttons and no `onsubmit`/`onclick`. Written so it fails if one becomes script-only |
| 7 | The board's per-card control renders unchanged |

Test 6 is why this handoff exists. Write it before the forms work, and show it
failing — the failure is the current state, so it should fail on today's code
without any planting at all.

Test 2's "preserving view parameters" matters because the list is reachable with
a `?view=` and losing it on every status change would make the control annoying
enough to avoid.

## 9. Escalate rather than deciding

- If the shared lock check does not fit either form path, stop and report. A
  second lock check is not the answer.
- If keeping the current status legible on a list row (§4) forces a row redesign,
  report it — that is a finding about the row, not a licence to drop the text.
- If `/status/board`'s handler turns out to be reusable for both surfaces with
  only a redirect-target change, say so; that is (a) arriving for free.

## 10. Acceptance

1. All seven §8 tests pass; test 6 shown failing on today's code first.
2. Both surfaces work with scripting disabled, end to end.
3. No click handler, no `onsubmit`, no script added anywhere.
4. One lock check, the existing shared one.
5. `change_status`'s `204` unchanged — that is step 2.
6. The board untouched.
7. All new copy through `peisear-i18n`; `prose_scan` and `test_harness_scan`
   pass.
8. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 11. Prohibited

No JavaScript. No click handler. No in-place update. No undo toast. No change to
`change_status`'s response. No second lock check. No caller-supplied redirect
target. No change to the board or `board.js`. No cycling status interaction and
no dropdown — RFC 004a §D2 dropped both, and three segments make them
redundant.

## 12. Required review-request format

Workflow §9.2. State which route shape §5 landed on and why, and include test
6's failing-on-today's-code transcript.
