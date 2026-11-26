# `SWP-001` — The colour the browser actually paints

Series [068 — the colour the browser actually paints](./README.md), **the
only round.** Read `README.md` first: §1 is the rule this round must write
down, §2 is the prototype whose **failures** are the specification, §3 is the
work.

**This round is larger than one of this series' usual rounds.** It was three
rounds in the architect's first proposal; the owner's review collapsed it,
because the three were paying for the same harness twice and keeping two
exclusion lists in step. **If it does not fit one round, say so and propose
the split** — the one thing it must not become is a round that ships the gate
without §3.2's own assertions, since a sweep that measures nothing looks
exactly like a product that passes.

---

## 1. Acceptance, in the order it will be reviewed

1. **The gate exists and runs in CI**, in the `browser-overflow-gate` job,
   covering **1.4.3 and 1.4.11 together**, and the job's `name:` names it.
2. **`GATE-001`'s wiring guard failed before you wired it**, and you quote
   that failure. It is the first new gate since that guard shipped; **if it
   does not fail, that is a finding about the guard** and it outranks the
   rest of this round.
3. **The gate's own population and sanity assertions are present and
   planted** — §3.2's four, each watched failing.
4. **Both criteria planted**: a text colour broken, and a border colour
   broken, each failing by name, each in a scratch worktree with `git status`
   clean either side.
5. **The exclusion list has one home, or one list with a stated reason.**
6. **`issues.rs`'s selected segment measures ≥ 4.5:1 in the browser**, with
   the figure quoted from a run.
7. **The authority rule is in `NFR-A11Y-005`**, and `§10.65` is opened and
   closed.
8. `DEC-007`'s block in full (**699** at `0.69.0`), `fmt`, both `clippy`
   variants, CI green **read per job**, `CHANGELOG.md` under `[Unreleased]`.

## 2. The three things most likely to go wrong

**They are the three that went wrong for the architect**, in order of how
convincing each looked while being wrong.

### 2.1 A parser instead of the renderer

`getComputedStyle(el).color` can come back as `oklch(...)`. Chrome
**round-trips that through canvas `fillStyle` unchanged**, so the classic
normalisation trick returns the same syntax it was given, and a numeric scan
of it yields three meaningless numbers. The second prototype reported
**1906 failures out of 1906 elements** this way, and the figure looked like a
catastrophic product defect rather than a broken probe.

**The route that works**: `ctx.fillStyle = '#ffffff'; ctx.fillRect(…)`, then
each background layer outermost-first, then the foreground, then
`getImageData(0,0,1,1).data`. The renderer performs the conversion and the
alpha composite. **Reset `fillStyle` to a known value before each
assignment** and detect the no-op, so an unsupported value cannot silently
repaint the previous layer.

### 2.2 Measuring nothing, and reporting zero failures

The first prototype's parser matched nothing, every element was skipped, and
it printed `examined=0, failures=0`. **Had that been a gate, it would have
been green for the rest of the product's life.** §3.2 of the README is the
answer, and criterion 3 above is not negotiable.

### 2.3 A border's own composite

1.4.11's subject composites over **the element's own fill**, not the page
behind it. That is the fact that made the architect's `/50` recommendation
read 2.73:1 when he predicted 3.16:1 — `background-clip: border-box`, which
is the default. The renderer gets this right; **the gate has to ask the right
question**, which is the colour on each side of the boundary it is judging,
not the page background.

## 3. What to decide, and say which you chose

1. **The non-text population.** `EDG-001`/`EDG-002` repaired the boundary and
   focus ring of `.input`, `.select`, `.textarea`, `.checkbox` — 67 rings and
   70 boundaries. Does the gate judge **those families** (matching the
   repaired set and the acknowledgement lists), or **every element with a
   visible border**? The second is more honest and will find card and alert
   borders that `0.68.0` deliberately ruled out. **The architect's lean: the
   repaired families, with the broader sweep reported as a count only** — so
   the decision stays where `0.68.0` made it rather than being reopened by a
   tool. Argue if the code says otherwise.
2. **Shared exclusion file or one list.** README §3.3. The counts
   (`[1, 2]`) may be meaningful only to the Rust side, in which case a shared
   file carries a field one reader ignores — which is a reason to keep one
   list and say so.
3. **The port**, and whether `PEISEAR_PORT` interacts badly with five
   siblings — `browser-checks/README.md`'s own warning.

## 4. What this round must not do

- **Must not** widen `overflow-gate`'s single assertion, or change any
  existing gate's own scope.
- **Must not** reopen `0.68.0`'s decision that card and alert borders are
  decoration (see §3.1).
- **Must not** touch identity, authentication, or anything in
  `crates/peisear-auth`.
- **Must not** cut a release.

## 5. The honest note this round inherits

**Three of the architect's figures in this series' own documents were wrong
before they reached you**, and the owner's review found the scope's shape
wrong after it was authorized. The rule in README §1 exists because of the
first; this round's single shape exists because of the second.

**So: the prototype's numbers are a starting point, not a baseline.**
Re-measure 120 cells, re-derive the two shapes, and if the count is not 130
or the shapes are not two, **that is the round's first finding** and the
README is wrong rather than the tree.
