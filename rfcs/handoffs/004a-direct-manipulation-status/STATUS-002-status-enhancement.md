# STATUS-002 — The status enhancement (D-1 step 2)

**Issued by**: Architect
**Date**: 2026-08-16
**Priority**: P1 — 0.26.0
**Governing RFC**: [004a](../../accepted/004a-direct-manipulation-status.md),
step 2, under [RFC 004](../../proposed/004-direct-manipulation.md)'s
cross-cutting contract
**Depends on**: STATUS-001 (landed, both rounds)

---

## 1. Scope

Step 1 gave issue detail and issue list a status control that works with
scripting disabled. **Step 2 makes it not reload the page**, and adds the undo
the umbrella requires.

**In scope**: a vanilla-JS enhancement over both step-1 forms; the in-place
update; the 5-second undo toast; and the one endpoint change that makes a second
change possible without a reload.

**Out of scope**: the board (D-2), drag of any kind, the cycling interaction and
dropdown RFC 004a §D2 dropped, and any general revision of JSON response shapes
beyond §3's one field.

## 2. What step 1 already settled, and must not be undone

Keyboard operation is **inherited, not rebuilt**. After step 1 the segments are
real submit buttons in a real form, so Tab and Enter work natively. The
enhancement must not introduce a keyboard path that exists only in script — that
would re-create the dependency step 1 removed.

`neither_surface_depends_on_script` is the guard. It must still pass, and it must
still fail if a segment becomes `type="button"`.

## 3. The endpoint — one field, and it is backward compatible

`change_status` returns `StatusCode::NO_CONTENT` with no body
(`handlers/issues.rs:833`). An in-place update means a second change without an
intervening page load, so the client's `client_updated_at` goes stale the instant
the first succeeds and every later change would 409.

**Return the new `updated_at` on success.** External design §7.3 already
specifies it: compare, then either update and return the new timestamp, or 409.

**Checked before asking for it**: `board.js` treats the response as
`res.status === 409`, then `!res.ok`, then reloads — it never reads the body. A
`200` with a JSON body is still `res.ok`, so this change does not touch the
board. Confirm that yourself rather than taking my word; it is the one thing here
that could break a shipped surface.

One field. Do not widen this into an API-shape revision.

## 4. The enhancement must fail open

**This is the requirement that matters most, and it is the one this handoff
exists to get right.**

The enhancement intercepts a working form. If it calls `preventDefault()` and
then throws — a bad selector, a network refusal, an unexpected response shape —
the control does nothing, and we are back to §17.4's shape on the very surfaces
`CONF-001` and `STATUS-001` just cleared it from.

So: **any failure inside the enhancement falls back to submitting the form
natively.** Not an error toast, not a console message — the native submit, so
the user gets the page-load path that already works.

That includes the case where the fetch itself fails. `board.js` reverts and
announces; that is right for a drag, where there is no form to fall back to.
Here there is one. Use it.

Write this so a reader can check it by reading — the project has no way to
execute this code in a test (§7).

## 5. Undo

Umbrella requirement 4: a 5-second toast with an Undo button; undo issues the
inverse mutation; after 5 seconds the toast goes and the action is no longer
undoable *through the toast*.

- The inverse of a status change is the previous status, which the client knows.
- Undo needs the **new** lock value — §3's field. This is why §3 is a blocker
  rather than a nicety.
- **If undo conflicts (409)**, someone else changed the issue in those five
  seconds. Do not retry and do not force. Announce that the current state is now
  shown, and reload — the same posture `board.js` takes, and umbrella
  requirement 5.
- **No celebratory language** (umbrella requirement 7). "Moved to Done" and an
  Undo button. Nothing else.

## 6. Copy, announcements, and where the script lives

All new strings through `peisear-i18n` — umbrella requirement 9, RFC 006 §D6
rule 7 included. `prose_scan` covers `components/` and `handlers/`; a string
baked into a `.js` file is outside it, so **strings the script needs are
rendered into the page** as data attributes or a JSON island, not written in
JavaScript. `static/search.js` is the standing exception and stays the only one.

Announcements go through an `aria-live` region, and the same announcement fires
whether the change came from pointer or keyboard (umbrella requirement 8). The
board's `#board-status` region is the existing pattern.

`static/dm.js`, referenced with `defer`, matching `board.js` and `search.js`.
RFC 005's nice-to-have sets a budget of **under 8 KB uncompressed** — treat it as
a ceiling worth reporting against, not a hard gate.

## 7. Testing — and being honest about what cannot be tested

The project's harness drives HTTP; it does not execute JavaScript. So:

**Testable, and required:**

| # | Check |
|---|---|
| 1 | A successful `POST /status` returns the new `updated_at` |
| 2 | Two consecutive status changes both succeed, the second using the value the first returned |
| 3 | A stale value still returns 409 |
| 4 | `board.js`'s contract is intact: the endpoint still answers `res.ok` for success and 409 for conflict |
| 5 | `neither_surface_depends_on_script` still passes, and still fails on a planted `type="button"` |
| 6 | The no-JS path still works end to end on both surfaces — step 1's tests, unchanged |
| 7 | `dm.js` is served and referenced with `defer` on both surfaces |

**Not testable here, and to be stated plainly in the review request**: that the
enhancement actually updates in place, that the toast appears and expires, that
undo issues the inverse, and that §4's fail-open path works. Those are verified
by reading and by hand.

**Say so.** `board.js` and `search.js` carry the same gap and nobody has written
it down; this is the moment to. Do not describe the JS as "tested" because the
suite is green — the suite does not run it.

If you want to argue for a JS test harness, that is a real proposal and belongs
in RFC 005's audit, not smuggled into this handoff.

## 8. Escalate rather than deciding

- If the endpoint change turns out **not** to be backward compatible with
  `board.js`, stop and report. §3 says it is; if I am wrong, everything
  downstream of that needs rechecking.
- If failing open (§4) cannot be done for some path, report it. That is a design
  finding, not a licence to ship a control that can do nothing.
- If the undo's inverse mutation needs anything the client does not already
  hold, say so before inventing a way to get it.

## 9. Acceptance

1. All seven §7 tests pass; test 5 re-verified by planting `type="button"` on
   each surface separately, as STATUS-001's round 2 established.
2. The endpoint returns the new lock value; `board.js` untouched and unbroken.
3. Any enhancement failure falls back to a native form submit.
4. Undo present, 5 seconds, inverse mutation, 409 handled without retry.
5. No celebratory language; announcements via `aria-live`; no string authored
   inside `dm.js`.
6. The no-JS path unchanged on both surfaces.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 10. Prohibited

No change to the board or `board.js`. No drag. No cycling interaction, no
dropdown. No JSON shape change beyond §3's field. No hydration — `DEC-021` and
RFC 004's resolved open question 1 both settle that. No retry-on-409. No
keyboard path that exists only in script. No claim that the JavaScript is tested.

## 11. Required review-request format

Workflow §9.2. State the `dm.js` size, confirm the `board.js` compatibility check
you ran, and list exactly what §7 says is unverified — in those words, not
softened.
