# JS-003 — one authority for response classification

**Governing RFC**: [011](../../accepted/011-browser-verification.md), step 2
**Target release**: 0.30.0
**Depends on**: `JS-001` (the inventory), `JS-002` (the fallback boundary guard)

## 1. What this is, at its real size

`JS-001` measured what step 2 buys and the number came in **below** what
RFC 011 claimed for it. That correction stands and this handoff is written
against it, not around it:

> Steps 2–3 buy deduplication and about ten Rust-testable rules. That is worth
> doing and it is **not** what was claimed for it.

**The rule this RFC most wanted to move cannot move.** `dm.js`'s fallback
boundary is a fact about which `catch` is nested where; `JS-002` pinned its
shape with a guard instead. Nothing in this handoff touches it, and **the guard
must still pass unchanged when you are done**.

So: this is a deduplication task with one behavioural decision inside it. Do
not let it grow.

## 2. The duplication, precisely

One fact — *how to classify the response to a status-change request* — is
written **three times in JavaScript** and once in Rust (where it is true).

| Response | `dm.js` | `board.js` |
|---|---|---|
| `409` | conflict → announce assertive, reload (`240–244`, `256`) | conflict → revert, announce assertive, reload — **at two sites** (`121`, `204`) |
| other non-`ok`, or network rejection | unavailable → announce assertive, reload | unavailable → revert, announce assertive, **no** reload — at two sites (`127–129`, `212–216`) |
| `2xx`, body missing a non-empty string `updated_at` | main path: **fall back to native submit** (`218–220`) · undo path: announce unavailable, reload (`248–250`) | **silent `return`, no announcement** — both sites (`135`, `220`) |

Rows 1 and 2 are the same rule written twice and are pure deduplication.
**Row 3 is not** — see §4.

## 3. The move

The vehicle already exists. `render_status_enhancement_assets` and
`render_board_copy_assets` (`components/issues.rs:658`, `:686`) each render a
server-authored JSON island that the script reads once at load. Today each
island is **a bag of strings**. Make it carry the classification too.

Both islands gain one `outcomes` object, built by **one shared Rust function**
— not two similar literals:

```
"outcomes": {
  "conflictStatus": 409,
  "conflict":    { "message": <surface's conflict copy>,     "reload": true  },
  "unavailable": { "message": <surface's unavailable copy>,  "reload": false },
  "unconfirmed": { "message": <surface's unavailable copy>,  "reload": true  }
}
```

Then each script's branch becomes a lookup: classify the response to one of
three keys, read `message` and `reload`, act. The `409` literal disappears from
both `.js` files.

**Why this is the move and not a JS-side helper.** A shared JavaScript function
would deduplicate the text and leave the fact in a file no test executes —
`§10.15`'s whole problem. Putting it in the island means the classification is
**authored in Rust, tested in Rust, and consumed as data**, which is what
`QA-019` and `HLT-001` did for `updated_at` and for indicator membership. Same
pattern: move the fact to where it can be checked.

**`conflictStatus` is the one that matters.** `409` is currently a magic number
in two `.js` files that must agree with what the server actually returns.

The Rust side is already clean: `error.rs:75` maps both
`AppError::Conflict(_)` and `AppError::OptimisticLockConflict { .. }` to
`StatusCode::CONFLICT` — a named constant, not a literal. So derive the emitted
value **from that mapping**, not by writing `409` a fourth time. The literal
should appear nowhere in the change.

Note that `error.rs:378` performs the same mapping a second time. Look at both
before you pick a source, and if they can diverge, **say so** — that is a
finding about the error layer, not about this refactor, and it is worth more
than the refactor is.

## 4. The one decision, and it is behavioural

`JS-001` named this and deliberately did not settle it: a malformed `2xx` body
is announced as unavailable by `dm.js` and **passed over in silence** by
`board.js`.

**`dm.js` is right.** A mutation that failed and says nothing is the worse
default, and `NFR-A11Y-008`'s assertive region exists precisely so a failure
can be heard. `board.js`'s two silent `return`s change to the `unconfirmed`
outcome above.

**But the two surfaces do not act identically, and that is correct.** On the
primary mutation `dm.js` falls back to a native submit — it has a form, and
resubmitting fetches authoritative state. `board.js`'s drag has no form to fall
back to, so it reverts, announces, and reloads. The *rule* is one thing — *"a
`2xx` body without a usable `updated_at` means the mutation is unconfirmed"* —
and the *action* differs because the surfaces differ. Keep the rule in one
place; do not force the actions to match.

**One thing to verify before you build on it.** Reading `board.js:135` and
`:220`, the silent `return` happens **before** `card.dataset.updatedAt` is
updated, while the optimistic move has already been applied. If that reading is
right, the card sits in its new column carrying a **stale** lock value, and the
next drag of that card sends the stale value and takes the `409` path — so
today's silent failure surfaces later as an unexplained "someone else changed
this". That would make the asymmetry worse than a difference in tone, and it
would make this the most valuable line in the handoff.

**I have read the code but not executed it.** Confirm or refute it before it
goes in the changelog. If it is wrong, say so plainly — that outcome is worth
as much as the other.

## 5. What must not change

- **`JS-002`'s guard passes unchanged.** `dm_fallback_boundary_scan`'s three
  assertions still hold: `applyChange` exists, carries a `try` at its own
  depth, and never calls `fallback(`.
- **`static_js_scan` still covers both files**, and no new user-visible string
  is authored in `.js` — every message continues to come from the island.
- **`DEC-021`**: the no-JS path is untouched. Nothing here may make a form
  depend on scripting.
- **The `§1.7` vocabulary check** applies to any new message key.

## 6. Tests

Rust, in `peisear-web`:

1. Both islands carry `outcomes` with all three keys and a `message` on each.
2. `conflictStatus` equals the status an `AppError::OptimisticLockConflict`
   actually produces — asserted by constructing that error and reading its
   status, **not** by comparing against a literal `409` and **not** against
   `StatusCode::CONFLICT` written out again in the test. Route the assertion
   through the same path a real response takes, or it proves only that two
   spellings of the same constant match.
3. The board island's `unconfirmed` message is **non-empty** — the assertion
   that would have failed on today's silent `return`.

**Plant each one separately** to prove it. One at a time: a compound plant
hides a hole, which is `STATUS-001` test 6's precedent and `QA-004`'s,
`QA-005`'s, `QA-009`'s and `JS-002`'s lesson four times over. For test 2 in
particular, plant a *wrong* `conflictStatus` and confirm it fails — a test that
compares the constant to itself through two names proves nothing.

## 7. Exit condition

`§10.15`'s baseline entry updated with the **new** residue — what remains in
JavaScript that no test executes, measured the way `JS-001` measured it, not
estimated. Report the number even if it is disappointing.

**Also fold `JS-002`'s residual into `dm_fallback_boundary_scan`'s stated
limits** while you are in that file: a *narrowed* top-level `try` still passes
the guard, and closing it would need a parser or a rule that fails on the
current tree. It is recorded in RFC 011 only. Put it in the module's own doc
comment, where the next person to trust that guard will read it.

## 8. Escalate rather than deciding

- If `409` is not expressible from one Rust constant shared with the handler.
- If moving the classification would require a new message key whose copy you
  would have to author — copy is not yours to write.
- If the §4 stale-lock reading turns out to be wrong **or** turns out to be a
  live user-visible defect rather than a latent one — the second changes this
  from a refactor into a fix, and its release note with it.

---

**Who holds what**: dev team — implementation and tests. **What's blocked**:
nothing. **What's next**: submit a review request package; architect review
before it lands.
