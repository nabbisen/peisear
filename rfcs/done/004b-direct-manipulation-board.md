# RFC 0004b: Direct manipulation — the board (D-2)

**Status**: **Implemented (0.26.0)** — `BOARD-001`
**Target**: 0.26.0
**Umbrella**: [RFC 0004](./004-direct-manipulation.md) — substep D-2
**Governing decisions**: `DEC-021`, `DEC-018`
**Related requirements**: `FR-DM-001/002/005`, `NFR-LANG-001`, `NFR-CONC-001`
**Last updated**: 2026-08-25

## Summary

**The Kanban drag already ships.** `static/board.js` has done column-to-column
drag since before this RFC's umbrella was written, and the per-card form
`DEC-018` added at 0.20.0 gives the same outcome without JavaScript.

So D-2 is not "add drag". It is bringing the board up to the cross-cutting
contract the umbrella imposes and the board predates — and the item that matters
most is that **three user-visible English sentences live inside `board.js`,
outside the message table and outside every guard this project has built.**

Full reconciliation: `.git-exclude/tasks/architect/010-d2-reconciliation.md`.

## Background

RFC 004's D-2 sketch reads *"New view on project detail … Drag from one status
column to another → POST status change."* Both shipped, the view at or before
0.20.0 and the drag in `board.js` from the start.

What the sketch describes as the substep's content is therefore done. What it
does not describe — the umbrella's own requirements 4, 6 and 9 — is not.

**`DEC-021`'s note on the board is spent.** It records that `board.js` does not
meet the progressive-enhancement bar. That was true when written; `DEC-018` /
DEV-002 then gave the board a per-card form-POST control, so today there is full
degradation without JavaScript and a keyboard path to the same outcome. The
decision's *rule* stands; its *example* no longer holds and should stop being
cited as an open defect.

## Requirements

1. **No user-visible string is authored inside `board.js`.** The three it
   carries move into `peisear-i18n` and reach the script through the
   JSON-island pattern `dm.js` established at 0.25.0.
2. **The board gets the undo toast** the umbrella requires of every substep —
   5 seconds, inverse mutation, no celebratory language.
3. **The board stops reloading on success.** `change_status` returns the new
   lock value as of 0.25.0; the board should use it, as the issue detail and
   list surfaces now do.
4. **Failure handling matches the corrected rule.** Umbrella requirement 2a: a
   fallback is for failures *before the mutation lands*. The board has no form
   to fall back to mid-drag, so its correct degradation is what it already
   does — revert the card, announce, reload — and that must not change into a
   resubmit.
5. **Keyboard parity is preserved, not replaced.** The per-card form is the
   keyboard path today and stays. See open question 1 on whether a keyboard
   drag equivalent is built at all.
6. `board_keyboard`'s six tests pass unchanged, and the no-JS path is untouched.

## Design

### D1 — The strings, and why this is first

```js
var RELOAD_MESSAGE = "This page is showing an earlier version of the board. …";
var CONFLICT_MESSAGE = "Another member changed this issue first. …";
var UNAVAILABLE_MESSAGE = "This status change could not be completed. …";
```

`prose_scan` covers Rust under `components/` and `handlers/`. `static/*.js` is
outside it by construction, and `search.js` is RFC 006's one *named* permanent
exclusion. **`board.js` was never named** — it was not excluded, it was
unexamined. The vocabulary guard has never seen these three sentences.

They are the board's error copy, which is the category §1.7's failure-framing
rule most concerns. They happen to read well. Nothing checked that, and nothing
would have.

`dm.js` renders its copy into a `<script type="application/json">` island and
reads it once. Reuse that. After this, both scripts are string-free and
`static/search.js` is again the single named exclusion — which is what the queue
README already claims and is not currently true.

### D2 — Undo, and the returned lock value

STATUS-002 gave the other two surfaces an in-place update and a 5-second undo,
using the `updated_at` the endpoint now returns. The board reloads instead,
which is why it never needed that field.

Bring it level. The inverse of a drag is a drag back — the same POST with the
previous status, which the card knows because it was just moved out of it.

**This is the item that could be dropped** if the board's reload-on-success is
judged good enough. It is a consistency argument, not a defect: a user who moves
a card and one who clicks a segment currently get different behaviour for the
same outcome.

### D3 — What must not change

The per-card form. `board_keyboard`'s six tests. The revert-announce-reload
posture on failure, which is correct for a surface with no form to fall back to
mid-gesture — requirement 2a says a fallback is for failures before the mutation
lands, and the board's is the right degradation for one that has no fallback
target at all.

## Test plan

| # | Check |
|---|---|
| 1 | No string literal in `board.js` reaches the user — the three constants are gone and the copy comes from the island |
| 2 | The three messages exist as `MessageKey`s and pass the vocabulary guard |
| 3 | A guard that fails if a user-visible string is authored in `static/*.js` — see open question 2 |
| 4 | Undo present on the board, 5 seconds, inverse mutation (if D2 is built) |
| 5 | `board_keyboard`'s six tests pass unchanged |
| 6 | The no-JS per-card form path is untouched |

**What cannot be tested here** is the same list STATUS-002 wrote down: the drag
itself, the toast, and the failure paths are verified by reading and by hand.
The harness drives HTTP. Say so; do not let a green suite stand in for it.

## Security and privacy considerations

None new. No endpoint changes, no authorisation surface, no new data. The
strings moving into the table is a correctness and vocabulary matter, not a
disclosure one.

## Out of scope

Adding drag — it exists. Changing the board's failure posture. D-3, D-4, D-5.
Any endpoint change. The `ServeDir` working-directory issue STATUS-002 reported,
which blocks an HTTP-level test that `static/*.js` is served — a real gap and a
separate one.

## Open questions — all settled at acceptance

**Settled 2026-08-25 by their stated defaults**, the owner having accepted
without varying them.

1. ~~**Is the keyboard pick-up/move/drop interaction built?**~~ — **No.**
   Requirement 1's parity is already met by the per-card form; Space-to-pick-up
   is a richer interaction, not a parity fix. Revisit if a user asks.
2. ~~**Should a guard cover `static/*.js` for authored copy?**~~ — **Yes, in
   this substep.** It is the only thing that stops this finding recurring in
   the next `.js` file, and it makes RFC 006's `search.js` exclusion true
   rather than assumed.
3. ~~**Is undo on the board worth building?**~~ — **Yes.** The inconsistency is
   visible to any user who uses both surfaces.

### Original wording, for the record

1. **Is the keyboard pick-up/move/drop interaction built at all?** The umbrella's
   requirement 1 — every action achievable by drag is achievable by keyboard —
   is **already met by the per-card form**. Space-to-pick-up is a richer
   interaction, not a parity fix. It is also the only genuinely new work in this
   substep. *Default-if-no-decision: not built. Revisit if a user asks for it.*
2. **Should a guard cover `static/*.js` for authored copy?** `prose_scan` cannot
   see it, and this RFC exists because nobody looked. A scan over `static/*.js`
   for string literals, with `search.js` allowlisted by name, would make the
   exclusion true instead of assumed. *Default: yes, and it belongs in this
   substep — it is the only thing that stops the same finding recurring in the
   next `.js` file someone writes.*
3. **Is D2 (undo on the board) worth building**, or is reload-on-success
   acceptable for a drag? *Default: build it — the inconsistency is visible to
   any user who uses both surfaces.*

## References

- `.git-exclude/tasks/architect/010-d2-reconciliation.md`
- `static/board.js`; `static/dm.js` (the JSON-island pattern)
- `DEC-021`, `DEC-018`; RFC 004 cross-cutting requirements 2a, 4, 6, 9
- RFC 006's queue README — the `search.js` exclusion this RFC makes true
