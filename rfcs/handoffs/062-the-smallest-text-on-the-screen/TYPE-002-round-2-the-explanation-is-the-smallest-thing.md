# TYPE-002 round 2 — the explanation is the smallest thing on the screen

**Series:** [062 — the smallest text on the screen](./README.md)
**Release:** `0.64.0`, authorized 2026-11-17 — the row, not the cut
**Blocked on:** `TYPE-002`, closed 2026-11-17, **accepted on twenty of
its twenty-one decisions.**
**Register entry:** `§10.60`, still **open**

`TYPE-002` was a good round. The chip-label question was answered the
only way it could be answered — live, with every label raised at once,
because raising them one at a time measures eleven things that do not
happen. The fixture bug it caught in its own work is the sharpest
observation either series has produced: **a measurement that returns the
same value on the wrong fixture cannot detect its own fixture being
wrong.** None of that is reopened.

**One decision is wrong, and the round's own paragraph says why.**

---

## §1 The two reasons cannot both stand

`me.rs:367` — the `<section class="text-xs">` holding the *"What do
these mean?"* glossary — was **kept** at 12 px. `§10.60` gives two
reasons, in one paragraph:

> *"…for content that is **optional**, disclosure-gated, and already
> fully legible at its current size."*

> *"Removal … considered and rejected: … a hover tooltip has no touch
> equivalent, and **the glossary is the one place a first-time or
> touch-only reader can read every term at once** without discovering
> each tooltip by trial."*

**Both cannot be true of the same reader.** Removal was rejected because
this is the *only* route by which a reader who does not know the terms
can learn them. For that reader it is the least optional thing on the
page. **The round found the right reader and then decided against
them.**

Three facts settle it:

1. **This is now the smallest text on `/today`** — the only `text-xs`
   render site left in `me.rs`, confirmed at source. **The explanation
   of the terms is smaller than every term it explains.** However well
   the paragraph is argued, that outcome reads as an oversight.
2. **"Already fully legible at its current size" is the claim this
   release exists to test.** `0.64.0`'s premise is that 12 px may be
   unkind to an elder reader, and this panel is where a reader goes
   **because they do not already understand.** It is the one unmeasured
   assertion in an otherwise measured round, and it is in
   `CHANGELOG.md` too, where it is addressed to users.
3. **The density objection does not reach this content.** +208 px at
   390 is real and well measured — and it is paid **only inside a
   disclosure the reader deliberately opened.** Nothing above the
   `<details>` moves. The objection that correctly killed the
   2026-11-04 proposal was about what a reader is shown *before
   asking*; this is the one thing on the page they must ask for.

## §2 The ruling

**Raise `me.rs:367` to `text-sm`.** Accept the measured **+80 px at
1280 / +208 px at 390** as a deliberate cost and **record it as
accepted, not as avoided.** A reader who opens seven definitions is
reading prose — the content that benefits most from the extra two
pixels — and they are paying for it only because they asked for it.

**The bookkeeping argument is overridden, not dismissed.** The round
said keeping the container leaves `me.rs:373`'s touch-target comment
(`32px + 16px = 48px`) true without an edit, so *"the split costs
nothing in bookkeeping, which is part of why it is the right split."*
That was a fair point. It is also one comment and two gate runs against
leaving the explanation of a screen smaller than the screen.

**If you disagree, file a review request and the architect will rule
again.** Do not leave it unraised without one. What is not available is
keeping both reasons: **either raise the container, or withdraw the
removal argument.** `GRAM-002` round 3 is the precedent for stopping,
and it was right to stop.

## §3 What this round does

1. **Raise `me.rs:367`** from `text-xs` to `text-sm`, with the
   +80 px / +208 px recorded in `§10.60` as an accepted cost and the
   reason it is accepted.
2. **Update `me.rs:373`'s touch-target comment** to `32px + 20px =
   52px`, pinned from `static/tailwind.css`'s own rule
   (`.text-sm{font-size:.875rem;line-height:1.25rem}`) exactly as this
   round already did for `me.rs:332` — not assumed, and not carried
   over from `332`'s comment either.
3. **Re-run `touch_target_scan` and `disclosure-marker-gate.mjs`, and
   report `what_do_these_mean`'s new measured height.** It should be
   **52 px**. Saying the number is what makes this measured rather than
   predicted — and `TYPE-002` already set the standard here by reporting
   `rhythm` at 52 px matching its own recomputed sum.
4. **Delete the unmeasured legibility claim** — *"already fully legible
   at its current size"* from `§10.60`, and *"information that is
   already fully legible"* from `CHANGELOG.md`. Both are rewritten
   anyway once the container is raised; what must not survive is the
   assertion itself.
5. **Name the cross-screen step in `§10.60`**, one sentence (§4).
6. **Re-measure the subtree after the change** and state its height at
   both widths, so the record carries the after as well as the before.
   The round gave 328 → 408 at 1280 and 520 → 728 at 390 as a
   *prediction* of raising; confirm or correct those as the *measured*
   result.

## §4 One thing to record rather than change

**`/today`'s headings are now 14 px while the identical
`text-xs uppercase tracking-wide` pattern on the project screens and
in `sprints.rs` is still 12 px.** That is not a defect — it is the
direct consequence of the owner's own instruction to read the screens
*"carefully one by one"*, and `TYPE-003` holds the next screen.

**But nothing in the record names it**, and an unnamed inconsistency is
indistinguishable from a drift six weeks later. One sentence in
`§10.60`: that it exists, that it is expected, that `TYPE-003` inherits
`/today`'s step as **a precedent rather than a rule**, and that whether
it becomes a rule is still decided after `TYPE-003`.

## §5 What this round must not do

- **Must not adopt a type floor, write a type requirement, or add a
  scan over text sizes.** Still `TYPE-003`'s, still after it.
- **Must not revisit any of `TYPE-002`'s other twenty decisions.** They
  are accepted. The headings, the eleven chip labels, the three italic
  lines and the two value lines are closed.
- **Must not touch `issues.rs`, `projects.rs` or `sprints.rs`** — 36
  sites belong to `TYPE-003`, the rest to the 65 excluded.
- **Must not touch `issues.rs:350` or `sprints.rs:438`/`:985`'s
  touch-target comments.** They are still true; nothing this round does
  reaches them.
- **Must not lower any ambient size.** `issues.rs:350` records that
  `py-3` would total only 40 px — the margin over the 44 px floor is
  4 px, and it is not this round's to spend.
- **Must not reopen `NFR-A11Y-005`, `NFR-A11Y-007`, `MARK-002`'s
  helper, `contrast_scan` or `touch_target_scan`.**

## §6 Acceptance

1. `me.rs:367` raised, or a review request disagreeing with §2's
   ruling.
2. `me.rs:373`'s sum updated and pinned from source.
3. `touch_target_scan` and `disclosure-marker-gate.mjs` run, with
   `what_do_these_mean`'s **measured** height reported.
4. The subtree's after-height measured at 390 and 1280 (§3.6).
5. The unmeasured legibility claim gone from both documents.
6. The cross-screen sentence in `§10.60` (§4).
7. **`node browser-checks/overflow-gate.mjs`** — a rendered class
   changes, so run it and report it.
8. **Tests per `.github/CONTRIBUTING.md` → *Before you open a pull
   request* → *Tests***, read there rather than copied from here.
   Baseline: `DEC-007` **687**, `cargo test -p peisear-web --lib`
   **246**, both confirmed independently by the architect. Count the
   artefact with `wc -l`.
9. **Clippy both ways**, as CI runs it:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   cargo clippy --workspace --all-targets --no-default-features --features postgres -- -D warnings
   ```
10. **`CHANGELOG.md`** amended — the glossary sentence is now wrong in
    two ways, not one.
11. **Every non-text artefact checked for local information before it
    is committed** — a path under `/home`, a real display name, a
    hostname. Per artefact, because nothing scans them.

**`§10.60` stays open.** `TYPE-003` has not run.

## §7 Where to file a question

`.git-exclude/review-request/TYPE-002-round-2-…/README.md` — the
question, what was tried, and what you are declining to do without a
ruling. **A blocking question goes in a file, never in chat.**
