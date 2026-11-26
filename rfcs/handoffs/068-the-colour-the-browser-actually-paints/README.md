# 068 — The colour the browser actually paints

**Release `0.70.0`** · row proposed 2026-11-25, authorized that day, **revised
and re-authorized 2026-11-26** (**the row, not the cut**: exit criterion 7 is
a separate decision against a release recommendation this series does not
write).

**One round.** Four releases taught `contrast_scan` to read a *declaration* —
a muted class token, a semantic family, a sheet rule, a border property. **No
declaration resolves a cascade.** This series builds the check that reads the
pixel, and it covers **both** contrast criteria at once.

---

## 1. Why this exists, and why it is not a fifth mechanism

`§10.52` is the precedent: a `<nav>` that read as visible in the file
computed `display: none` at four widths in a real browser. `EDG-002` is the
cost: the architect's recommended `/50` border tier read **2.73:1** instead of
his predicted 3.16:1, because a translucent border composites over **the
control's own fill** (`background-clip: border-box`) — three facts a source
scan does not have. `EDG-003`'s guard was therefore built **deliberately
unable to compute ratios**.

**The relationship this series must write down, because the architect's own
proposal left it unstated:**

> **Source is total and approximate. Rendered is exact and fixture-bounded.
> Where they disagree, the renderer is right about what it reached, and
> silent about the rest.**

That is not a form of words. **The fixture builds 24 URLs**; `app.rs` carries
**68 `.route(` registrations** (how many of those render a page is
unmeasured — treat it as a bound, not a ratio). The four source mechanisms
read every class token in every component, rendered or not. **Neither
supersedes the other**, and the register has to say so once, or the next
person to meet a disagreement will guess.

**The disagreement already exists**: the brand dot
(`<span class="text-primary">●</span>`, `auth.rs`, `layout.rs`) is a **named,
counted exclusion** in `contrast_scan` and a **4.21:1 failure** in a rendered
sweep. Both are right. The rule above is what makes that sentence true rather
than confusing.

## 1.1 The round

| round | what it owes | state |
|---|---|---|
| [`SWP-001`](./SWP-001-the-colour-the-browser-actually-paints.md) | the gate, both criteria, its own population and sanity assertions, one exclusion home, the defect, the register, the wiring, and the close | **closed** 2026-11-26 — the gate wired and passing, `issues.rs`'s selected segment repaired (3.87:1 → 4.83:1, measured both times), the exclusion list moved to one shared file, two measurement artefacts (autofocus, `.checkbox:checked`) found and corrected before being trusted |

## 2. What was measured before this series was scoped

The architect built a prototype and ran it. **Its three failures are this
round's specification, and they matter more than its successes:**

| attempt | what it reported | why it was wrong |
|---|---|---|
| 1 | `examined=0, failures=0` | a regex colour parser matched nothing. **Zero examined reads exactly like "everything passes."** |
| 2 | `examined=1906, failures=1906` | **Chrome round-trips `oklch()` through canvas `fillStyle` unchanged** — the normalisation trick does not convert, so every colour became three meaningless numbers |
| 3 | `examined=4813, failures=130`, two shapes | correct: **paint each background layer, then the foreground, on a 1×1 canvas and read the pixel** — the renderer performs every conversion and every composite |

Attempt 3, at 24 pages × 5 widths = **120 cells**, in **16.5 seconds** (the
five-gate browser job currently takes 3m42s):

- **`3.87:1`** — `join-item btn btn-sm btn-active btn-primary`,
  `components/issues.rs`'s **selected** segment of the Board/List control,
  14px, needs 4.5. **No exclusion names it, and `contrast_scan` is 8/8 on the
  same tree** — it is opaque-on-opaque, two tokens applied by two different
  rules, which `§10.61` (semantic family as *text* colour) and `§10.62`
  (sheet colours *below* the muted floor) both miss by construction.
- **`4.21:1`** — the brand dot, already excluded by name, and the figure
  matches the exclusion list's own doc comment exactly. **An independent
  route reproduced the document's number**, which is the prototype's
  validation rather than a product defect.

**Re-measure all of it.** These figures are the architect's, and his last
three tables of this kind each contained an error he did not find himself.

## 3. What this round owes

### 3.1 The gate

`browser-checks/contrast-gate.mjs`, covering **WCAG 1.4.3 (text, 4.5:1, or
3:1 at ≥24px or ≥18.66px bold) and WCAG 1.4.11 (non-text, 3:1) together**.

- **Import `overflow-gate`'s own `WIDTHS`, `ADVERSARIAL_STRINGS`,
  `createFixtures` and `buildPages`** — never a second copy. `occlusion-gate`
  established this and says why in its own comment.
- **Measure by painting, never by parsing.** No colour-syntax parser, and do
  not assume canvas `fillStyle` normalises anything: attempt 2 above is what
  that assumption produces. Paint the ancestor background chain outermost
  first, then the foreground, and read `getImageData`.
- **1.4.11's own composite is the point.** A border or outline composites
  over the element's own fill, not the page behind it — the fact `EDG-002`
  had to measure by hand and the architect got wrong. The renderer answers it
  for free; make sure the gate asks it correctly, per side if the adjacent
  colour differs.
- **Inactive controls are exempt** from 1.4.3 by the criterion itself; say so
  in the code rather than in a commit message.

### 3.2 Its own population and sanity — the part that is not optional

Attempt 1 is why. **A sweep that cannot tell *nothing wrong* from *nothing
examined* is `§10.36` with a browser attached.**

1. a **floor on elements examined**, per run and ideally per page;
2. **`unpaintable == 0`** — any colour the gate could not paint is a hard
   failure, not a skip;
3. a **known-good control pair** asserted every run (the prototype used an
   `<h1>`: `14.05:1`, `[24,26,42]` on `[232,232,232]`, which are `--bc` and
   `--b2`);
4. **a 0% or a 100% failure rate is itself an error.** Both of the architect's
   broken attempts would have been caught by this one assertion.

### 3.3 One exclusion list, two readers

**Today**: `NAMED_EXCLUSION_CLASS_VALUES` holds **2 entries**
(`"text-5xl font-bold text-error"`, `"text-primary"`) with
`NAMED_EXCLUSION_COUNTS` `[1, 2]`.

**Move it to one small JSON file that both readers read** — `contrast_scan`
in Rust (`serde_json` is already a `peisear-web` dependency) and the gate in
Node. **Not** two lists plus a test that compares them: that was the
architect's proposal and it is the flaw the owner's review found.

**State the cost honestly in the round's report**: this moves a **working,
shipped list** and touches a guard that currently passes. If, on reading it,
a shared file turns out to cost more than it saves — for instance because the
counts are meaningful only to the Rust side — **say so and keep one list,
with the reason**. The instruction is *one home if one home is cleaner*, not
*a shared file whatever it costs*.

### 3.4 The defect

Repair `issues.rs`'s selected segment. **Measure the repair in the browser,
not in a calculator.** The product already has tiers that pass; prefer one of
them over a new colour, as `EDG-001`/`EDG-002` did.

The brand dot stays excluded; the gate must honour the shared list so it does
not fail on its first run for a reason the project decided deliberately.

### 3.5 The register

- **`NFR-A11Y-005`** gains the authority rule from §1, in one sentence.
- **`§10.65`** opens and closes: *a check that reads a declaration cannot
  resolve a cascade*, with the `/50` error, the `oklch` round-trip, and the
  `examined=0` run as its instances.
- `NFR-A11Y-010`'s scope gains the rendered half, since 1.4.11 is now
  measured rather than argued.

### 3.6 Wire it, and close the series

- Add it to `test.yml`'s `browser-overflow-gate` job — **and `GATE-001`'s own
  guard will fail until you do**, which is the first time that guard earns
  its keep on a new gate rather than a planted one.
- Update the job's `name:` string and `browser-checks/README.md`'s *Files*
  list and *Running them locally* block: **five scripts becomes six**, and
  the port paragraph gains this gate's port.
- `DEC-007`'s block in full — **699** at `0.69.0`; state the new count and
  what moved it.
- `CHANGELOG.md` under `[Unreleased]`, in the register's plain-reader voice.
- Rounds table and a closing summary here.

## 4. What every round owes, in common

1. **Measure at the tree, quote the command.** A figure in this file is a
   claim to check.
2. **Plant before trusting.** A new check must be watched failing on a
   deliberate break, in a scratch `git worktree` or by file copy — never
   `git checkout`/`git stash` on this tree — with `git status` clean either
   side. For this gate, plant **both criteria**: a text colour and a border
   colour.
3. **Real exit codes**: `cmd | tail -1; echo $?` reports `tail`'s status.
4. **Scratch under `.git-exclude/tmp/`** with `CARGO_TARGET_DIR` there, and
   removed when the round closes.
5. **No release leg.** `DEC-056`'s four are the architect's, on the owner's
   approval at exit criterion 7.
6. **Nothing published carries local information** — no screenshot, no
   rendered capture, no private path in `docs/` or `rfcs/`.

## 5. Deliberately not in this series

- **The 17 in-scope smallest-text sites**, the **19** `opacity-N` sites, the
  **nine** `link link-hover` sites, the **`xs` control tier** — text and type
  questions, each figure re-derived at `0.69.0`'s scoping.
- **Card and alert borders** — decoration beside a shadow or a filled
  surface, out by 1.4.11's own test, measured and recorded at `0.68.0`.
- **`FR-HLT-005`'s unverified status**, `proc-macro-error2`, and the Pages
  deployment wedged since 2026-11-18.
- **Anything about identity or authentication.** A relying-party obligation
  list arrived from an upstream project this week; it is a candidate, not
  this series.

---

## 6. Series closed — the one round, stated plainly

**`SWP-001` was real exactly as scoped**, and it was larger than this
series' own usual round, as §0 warned it might be — it did not need
splitting. The gate renders, covers both criteria at once, and found
one real defect this project had shipped for an unknown number of
releases: the project board's own selected view (Board or List) read
**3.87:1**, below the 4.5:1 this requirement has asked for since
`0.27.0`. Repaired on an existing tier, no new colour, re-measured in
the browser at **4.83:1**.

**Two things found along the way, neither a product defect, both
recorded rather than silently worked around**: an autofocused field's
own "resting" border is already its focused state by the time anything
reads it, and a checked `.checkbox`'s border is painted the same
colour as its own now-opaque fill by construction — both are
measurement artefacts in the gate's own first draft, found by
debugging a surprising figure rather than trusting the acknowledgement
list's shape, the discipline `§1`'s own authority rule asks for.

**The population choice went the architect's own way**: the repaired
families, not every bordered element, with the broader population
(2298 elements) counted and logged rather than asserted on, so
`0.68.0`'s own decision about card and alert borders stays made.

**The exclusion list moved to one shared file**, with its one real
cost — `liveCount` unread by one of its two readers — stated in the
round's own report rather than hidden, per this file's own §3.3
instruction not to pay for a shared file "whatever it costs."

`§10.65` opens and closes this round. `NFR-A11Y-005` and
`NFR-A11Y-010` are each widened again; no new register number spent.
`Covers release`/`Amended` in `requirements.md` moved to `0.70.0`,
nesting `0.69.0`'s own text, at this round's own close.
