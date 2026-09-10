# LOCK-001 — assert the guarantee the JavaScript was covering for

**Governing RFC**: [011](../../done/011-browser-verification.md), **`DEC-052`**
— step 3's replacement. **Target release**: 0.33.0.

## 1. What this is

`board.js` defends against a dragged card with no `data-updated-at`:

```js
if (!clientUpdatedAt) { revert(); announceAssertive(copy.reloadMessage); return; }
```

**Nothing guarantees the server never renders such a card.** The attribute is
emitted at `components/issues.rs:847`. No test in `crates/peisear-web/tests/`
asserts it is present — I checked.

**Add that assertion.** One test, no browser, in the counted suite.

## 2. Why it matters, at its real size

**Not a crash — a silent no-op.** A card that cannot be dropped, on a page that
looks fine and reports nothing. `NFR-CONC-001`'s optimistic lock is what stops
two people overwriting each other's work, and this attribute is how the board
participates in it.

**Not urgent**: the JavaScript handles it correctly today and tells the user to
reload. **The point is that the handling is the only thing standing there.** If a
refactor of the card, or a new view, stops emitting the attribute, **nothing
fails** — the board quietly stops accepting drags.

## 3. The test

Every issue card the board renders carries a **non-empty** `data-updated-at`.

**Two things this must not become**, both of which this project has been bitten
by:

- **Not `body.contains("data-updated-at")`.** That passes when *one* card has it.
  **Walk every card and assert per card** — `§10.17` records eleven assertions
  that were passing on a neighbour's markup, and two were found only because
  someone planted against them.
- **Not "present".** An empty attribute is exactly the state `board.js` guards
  against — `!clientUpdatedAt` is false for `""`. **Assert non-empty.**

**Fixture: more than one issue**, and preferably in more than one column. A
single-card board cannot distinguish "every card" from "a card".

## 4. Plant it — three ways, separately

1. **Remove** `data-updated-at` from the card render → must fail.
2. **Empty it** (`data-updated-at=""`) → must fail. This is the one a
   presence-only assertion would miss.
3. **Remove it from one card of several** → must fail, and **name which card**.
   This is the `§10.17` case; a test that passes here is scoped wrong.

## 5. What is deliberately not in scope

**The list and detail surfaces.** `dm.js` reads a hidden `client_updated_at`
form input rather than a data attribute, and an empty one degrades to the native
form submit — a working, tested path (`DEC-021`). Different mechanism, different
consequence. **If you think it deserves the same assertion, say so** — but as a
finding, not folded in here.

**The JavaScript branch stays.** Do not remove it because the server is now
guaranteed. It still covers the case the server cannot: **a page cached from an
older build**. Defence in depth, with the depth now asserted rather than assumed.

## 6. Escalate rather than deciding

- **If the assertion fails on the current tree.** That would mean a card really
  can render without its lock value, and the finding outranks the handoff.
- **If a new test file is needed** — `dec_007_fs_scan` will require it in the
  `DEC-007` block and CI; that is the guard working, not an obstacle.

## 7. Exit condition

One test, planted three ways. `DEC-007` clean at **256** — this handoff adds
exactly one test, and if the count moves differently something was
misunderstood. Three consecutive `cargo test --workspace` runs.

**RFC 011 closes when this lands.** Its four steps will then be: three done, one
withdrawn with its reason recorded and replaced by this.

---

**Who holds what**: dev team — the test. **What's blocked**: RFC 011's move to
`done/`. **What's next**: review request.
