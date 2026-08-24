# RFC 0004a: Direct manipulation — status change (D-1)

**Status**: Proposed
**Target**: 0.25.0, **after CONF-001**
**Umbrella**: [RFC 0004](./004-direct-manipulation.md) — substep D-1
**Governing decisions**: `DEC-021` (JavaScript posture), `DEC-018`
**Related requirements**: `FR-ISS-005/006`, `FR-DM-001/005`, `NFR-CONC-001/004`
**Last updated**: 2026-08-16

## Summary

Make an issue's status changeable from the issue list and the issue detail page
in two clicks or fewer — **in two steps, not one.**

**Step 1 makes status changeable on those two surfaces without JavaScript at
all.** Neither has any status control today; only the board does.

**Step 2 adds the pointer and keyboard affordance** on top of it: click a
segment to set that status, without a page load.

The umbrella's substep sketch describes step 2. Step 1 is not optional garnish —
the umbrella's own cross-cutting requirement 0 makes it mandatory and first, and
this RFC exists largely to say why that matters more here than the feature does.

## Background — the finding that shaped this

Verified against the code, not assumed:

| Surface | Status control today | Works without JavaScript? |
|---|---|---|
| Board (`?view=board`) | Per-card form POST to `/status/board` (`components/issues.rs:621`) | **Yes** — `DEC-018` / DEV-002 |
| Issue list | Rendered as **text** (`status_text`) | No control at all |
| Issue detail | **Inert** `<button type="button">` segments — `tabindex="-1"`, `cursor-default`, `aria-pressed`, no handler (`components/issues.rs:1504–1531`) | No control at all |

So D-1's two target surfaces are precisely the two with no no-JS path. Only the
board — which D-1 does not touch — already has one.

**Why this is the crux.** The issue detail page already renders three
status-shaped buttons that do nothing. Attaching a JavaScript click handler to
them and stopping there would produce a control that works for some users and
silently does nothing for others — **the exact shape of external design §17.4**,
which `CONF-001` is fixing in this same release for nine other controls.

Shipping that shape into a new place while removing it from an old one would be
the most expensive kind of inconsistency: one where the project has already
written down why it is wrong.

## Requirements

### Step 1 — the working path

1. **The issue detail status segment becomes a real form.** Three submit buttons
   in a form POSTing the chosen status, carrying `client_updated_at`. The
   `tabindex="-1"` and `cursor-default` go; `aria-pressed` stays and keeps
   carrying which is current.
2. **The issue list gains an equivalent control** on each row.
3. **Both work with scripting disabled**, end to end, and are keyboard-operable
   by virtue of being form controls — not by virtue of any script.
4. **The optimistic lock applies** through the existing shared
   `check_optimistic_lock`. No second lock check (umbrella requirement 5).
5. **A rejected change says what is true**, using the established conflict
   vocabulary — no failure framing (§1.7, `FR-DM-005`).

### Step 2 — the enhancement

6. **Clicking a segment sets that status without a page load**, and the row or
   segment updates in place.
7. **Keyboard parity is inherited, not rebuilt.** After step 1 the segments are
   real buttons in a real form, so Tab and Enter already work. The enhancement
   must not remove that, and must not introduce a keyboard path that exists only
   in script.
8. **The endpoint returns the new lock value** — umbrella requirement 6, and see
   §D3. Without it, a second change to the same issue conflicts every time.
9. **Undo toast**, 5 seconds, per the umbrella. No celebratory language.
10. **All copy through `peisear-i18n`**, RFC 006 §D6 rule 7 included.

## Design

### D1 — Step 1 first, and shippable alone

Step 1 is a complete, useful change on its own: two surfaces gain a status
control they have never had. If step 2 is never written, nothing is broken and
nothing is half-built.

That is the test for whether requirement 0 has been honoured — **not** "does the
no-JS path exist somewhere" but "would we be content to stop here".

### D2 — The three-button segment makes the dropdown unnecessary

The umbrella's sketch says *"Right-click / long-press (mobile) opens a dropdown
to pick directly"* and *"click cycles Open → InProgress → Done → Open"*.

**Both are dropped.** The shipped markup already renders three segments, one per
status. Three buttons that each set their own status need neither a cycle nor a
dropdown — the direct choice the dropdown was for is already on screen, and a
cycling click is a worse interaction than the one the markup affords.

This is a simplification the shipped design handed us, not a scope cut. It also
removes the substep's only right-click interaction and its only mobile
long-press, which between them would have needed their own keyboard equivalents.

### D3 — The 204 blocker is real, and narrow

`change_status` returns `StatusCode::NO_CONTENT` with no body
(`handlers/issues.rs:833`). Step 2's in-place update means a second change
without an intervening page load, so the client's `client_updated_at` is stale
the instant the first succeeds — every subsequent change would 409.

External design §7.3 already specifies the right behaviour: compare, then either
update and return the new timestamp, or return 409.

**Change the success response to carry the new `updated_at`.** This belongs to
step 2, not step 1 — step 1's form POST does a page load and reads a fresh value
from the server anyway.

Do not widen this into a general API-shape change. One endpoint, one field.

### D4 — `/status/board`'s redirect is board-shaped

Two routes exist: `/status` returns 204, `/status/board` returns
`Redirect::to("/projects/{id}?view=board")`.

Step 1's forms need a redirect back to *their* surface, and `/status/board`
hardcodes the board's. Options, and the substep should state which it took:

- **(a)** Generalise: one form-POST route whose redirect target is derived from
  where the form was submitted. Requires deciding how it knows.
- **(b)** One route per surface, mirroring `/status/board`'s pattern.

**(b) is the smaller change and (a) is the better shape.** I lean (a) with the
target derived server-side from the route's own parameters — never from a
caller-supplied parameter, for the reason `CONF-001` §3.3 gives about
open redirects. But this is a judgement the implementer is closer to; state it.

### D5 — What must not regress

The board's existing per-card control and `board.js` are untouched. D-1 does not
go near them; D-2 does.

## Test plan

| # | Check |
|---|---|
| 1 | **No-JS, issue detail**: POST the status form directly; the status changes |
| 2 | **No-JS, issue list**: same on a list row |
| 3 | A stale `client_updated_at` on either returns 409 — through the shared lock check, not a new one |
| 4 | `aria-pressed` still marks the current status after the segment becomes a form |
| 5 | The segments are keyboard-reachable — `tabindex="-1"` is gone |
| 6 | **Regression guard**: no status control on either surface is script-only. Written so it fails if one becomes so |
| 7 | Step 2 only: a successful change returns the new `updated_at`, and two consecutive changes both succeed |
| 8 | Step 2 only: the board's existing control and `board.js` behaviour are unchanged |

Test 6 is the one that keeps §17.4's shape out. Test 7 is the one that would
otherwise be found by a user making two changes in a row.

## Security and privacy considerations

- No new authorisation surface: both steps use the existing status endpoints and
  their existing write-access checks.
- No caller-supplied redirect target — §D4.
- Nothing new is disclosed; status is already visible on both surfaces.

## Out of scope

Drag (D-2). The cycle interaction and the dropdown (§D2). Any change to the
board. Any general revision of the JSON API's response shapes beyond §D3's one
field. Undo beyond the umbrella's 5-second toast.

## Open questions

1. **§D4's (a) or (b)** — one generalised form route or one per surface. *Default
   if the implementer has no preference: (b), as the smaller change, with (a)
   recorded as the intended shape.*
2. **Does step 2 ship in 0.25.0 at all?** Step 1 is complete alone, and the
   release already carries `CONF-001`. *Default: yes, both — but if step 1 lands
   late, step 2 slips rather than compressing its review.*
3. **Does the issue list's control render on every row, or only on hover/focus?**
   Every row is simpler and works without pointer hover, which matters for
   touch. *Default: every row.*

## References

- RFC 0004 §Requirements cross-cutting 0 and 6
- External design `§17.4` (the shape to avoid), `§7.3` (the response contract)
- `.git-exclude/tasks/architect/009-rfc-004-reconciliation.md` §2.1
- `components/issues.rs:1504–1531` (the inert segment), `:621` (the board's
  working form), `handlers/issues.rs:833` (the 204)
