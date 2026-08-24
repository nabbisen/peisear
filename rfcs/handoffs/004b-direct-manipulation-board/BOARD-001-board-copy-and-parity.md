# BOARD-001 — The board's copy, its undo, and a guard for `static/*.js`

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P1 — 0.26.0
**Governing RFC**: [004b](../../accepted/004b-direct-manipulation-board.md),
under [RFC 004](../../proposed/004-direct-manipulation.md)'s cross-cutting
contract
**Depends on**: STATUS-002 (landed, both rounds)

---

## 1. Scope, and what this substep is not

**Do not add drag.** It ships. `static/board.js` has done column-to-column drag
since before RFC 004 was written. If you find yourself writing a `dragstart`
listener, stop — something has gone wrong with the premise, and that is a
finding to report before writing anything.

Three things, in this order:

1. **The three English sentences inside `board.js`** move into the message
   table.
2. **A guard over `static/*.js`** so this cannot recur silently.
3. **The board's undo toast**, and using the returned lock value instead of
   reloading.

Item 2 is the durable one. Item 1 is what made it necessary.

## 2. Item 1 — the strings

```js
var RELOAD_MESSAGE = "This page is showing an earlier version of the board. …";
var CONFLICT_MESSAGE = "Another member changed this issue first. …";
var UNAVAILABLE_MESSAGE = "This status change could not be completed. …";
```

User-visible copy authored in JavaScript. `prose_scan` covers Rust under
`components/` and `handlers/`; `static/*.js` is outside it by construction.
`search.js` is RFC 006's one **named** permanent exclusion — `board.js` was
never named. It was not excluded, it was unexamined, and the vocabulary guard
has never seen these sentences.

**Use `dm.js`'s pattern**: three `MessageKey` variants, rendered into a
`<script type="application/json">` island, read once at script load. That
pattern is one release old, reviewed, and already carries `dm.js`'s copy.

**Byte-exact.** These three read well and this is not a copy pass — convert the
text unchanged. If the vocabulary guard rejects any of them, **stop and report**:
that would be a genuine §1.7 finding on copy that has been shipping unchecked,
and it is worth more than the conversion.

## 3. Item 2 — the guard, and its allowlist

A test that scans `static/*.js` for authored user-visible strings.

**`search.js` is allowlisted, by name, with a reason** — it needs a JS-side
rendering mechanism that does not exist, and inventing one for a type-ahead
dropdown is disproportionate. That has been RFC 006's stated position since
0.21.0; this makes it true rather than assumed.

**Calibrate the way `prose_scan` was.** A string literal in JavaScript is not
automatically copy: URL fragments, class names, `data-` keys, event names and
selector strings all look alike to a scanner. If the heuristic needs more than
`search.js` plus a handful of entries, **the heuristic is wrong, not the
codebase** — report that instead of growing the list. An allowlist that absorbs
findings is worse than no guard, because it looks like coverage.

**Prove it works**: after item 1 lands, plant one of the removed sentences back
into `board.js`, watch the guard fail naming it, remove it. A guard nobody has
seen fail is a guard nobody has tested — and `prose_scan`, `test_harness_scan`
and `dm.js`'s own tests were each verified that way.

Two things to carry over from `prose_scan`'s history, so this guard does not
repeat them:

- **Comments.** `prose_scan` read comments as code until QA-002 ported
  `strip_line_comments` into it. Decide deliberately whether this one strips
  `//` comments, and say which.
- **Where it lives.** `prose_scan` and `test_harness_scan` are both
  `#[cfg(test)]` modules in `peisear-web`. Follow that unless there is a reason
  not to; if you share a helper with either, say so — QA-002 chose duplication
  over sharing for a six-line function and gave its reason.

## 4. Item 3 — undo, and not reloading

STATUS-002 gave issue detail and issue list an in-place update and a 5-second
undo, using the `updated_at` the endpoint now returns. The board reloads on
success instead, which is why it never needed that field.

Bring it level. The inverse of a drag is the same POST with the previous status,
which the card knows because it was just moved out of it.

**The failure posture does not change.** Umbrella requirement 2a says a fallback
is for failures *before the mutation lands*; the board has no form to fall back
to mid-gesture, so revert-announce-reload remains correct. Do not turn it into a
resubmit, and do not extend `dm.js`'s `fallback()` to a surface with no form.

If undo's own request fails, take `board.js`'s existing posture — and note it
already distinguishes `CONFLICT_MESSAGE` from `UNAVAILABLE_MESSAGE`, which is
the split STATUS-002's round 2 had to add to `dm.js`. The board got that right
first.

## 5. What must not change

The per-card form control (`DEC-018` / DEV-002) — it is the no-JS path and the
keyboard path, and RFC 004b open question 1 settled that no keyboard drag
equivalent is built. `board_keyboard`'s six tests pass unchanged. No endpoint
change. No authorisation surface.

## 6. Tests

| # | Check |
|---|---|
| 1 | No user-visible string is authored in `board.js`; the three come from the island |
| 2 | The three `MessageKey`s exist, render byte-exact, and pass the vocabulary guard |
| 3 | The `static/*.js` guard exists, passes, and has been **demonstrated failing** on a planted sentence |
| 4 | The guard's allowlist is `search.js` plus whatever §3's calibration honestly requires — each with a reason |
| 5 | Undo present on the board: 5 seconds, inverse mutation, no celebratory language |
| 6 | `board_keyboard`'s six tests pass unchanged |
| 7 | The no-JS per-card form path is untouched |

**Not testable here**, and to be stated plainly: the drag itself, the toast's
appearance and expiry, and the failure paths. The harness drives HTTP, not
JavaScript. STATUS-002 wrote that disclosure; reuse its shape and do not let a
green suite stand in for it.

## 7. Escalate rather than deciding

- **If any of the three sentences fails the vocabulary guard**, stop and report
  before rewording. That is a §1.7 finding on shipped copy.
- **If the `static/*.js` heuristic needs more than a handful of allowlist
  entries**, report rather than grow it.
- **If you find a fourth authored string** anywhere under `static/`, report it
  before converting — the count is a fact about how long this went unlooked-at.
- If drag turns out **not** to be shipped as RFC 004b claims, stop. Everything
  in this handoff rests on that.

## 8. Acceptance

1. All seven §6 tests pass; test 3 demonstrated failing on a planted sentence.
2. `board.js` authors no user-visible string; neither does `dm.js`.
3. `search.js` is the only named exclusion, and the guard says so.
4. Undo on the board, using the returned lock value; failure posture unchanged.
5. `board_keyboard` unchanged; per-card form untouched.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs; `prose_scan` and `test_harness_scan` pass.

## 9. Prohibited

No drag work. No keyboard pick-up/move/drop interaction — RFC 004b open question
1 settled it as not built. No rewording of the three sentences. No endpoint
change. No change to the per-card form. No weakening of any existing guard, and
no allowlist entry without a reason that is a decision rather than "not done
yet".

## 10. Required review-request format

Workflow §9.2. Include test 3's planted-failure transcript, state whether the
new guard strips comments and why, and give the honest count of authored strings
found under `static/`.
