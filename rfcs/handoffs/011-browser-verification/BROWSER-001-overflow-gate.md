# BROWSER-001 — the horizontal-overflow gate

**Governing RFC**: [011](../../accepted/011-browser-verification.md), **step 4**
**Target release**: 0.33.0
**Binding**: `DEC-048`'s four conditions. Read them before anything else — they
were written to constrain exactly this handoff, and one of them is why step 4
took three releases to become possible.

## 1. What this is, and what it is deliberately not

**One assertion, on rendered pages: `scrollWidth <= clientWidth`.**

That is the whole gate. It is the check that found `LAYOUT-001` and
`LAYOUT-002` — two defects that had shipped for many releases and that **nothing
this project owns could observe**.

**It is not** the target-size gate (step 4 ranked that second, and it is more
sensitive to browser version). **It is not** the overlap gate — `§10.19` and
`§10.21` are real, and the measurement artefacts around them are not yet
understood well enough to gate on. **It is not** a JavaScript test; `DEC-048`
condition 2 forbids that and `JS-001` measured the value as thin anyway.

**Do not widen it.** A first browser check that grows a second assertion before
the first has proved stable is how `DEC-048`'s condition 3 starts costing
attention.

## 2. It does not go in `cargo test --workspace`

**This is the most important design constraint in the handoff.**

If the gate were a Rust integration test it would need a CDP crate, and
**every contributor's `cargo test --workspace` would then require Chromium** —
including `DEC-007`'s three-consecutive-run gate, run before every release. That
is an imposition on everyone for a check that serves CI.

**Build it as a separate CI job** driving a tracked Node script, the way `fmt`
and `clippy` are gates without being tests. Node and Chromium are both present
on `ubuntu-latest`; add no dependency to any `Cargo.toml`.

**It is a gate, not a test, and the distinction is load-bearing**: the project's
inventory (255) counts `cargo test` results. This check is not in that number and
must not be reported as though it were — the same footing `fmt` and `clippy`
already have.

## 3. The harness becomes tracked

`.git-exclude/tools/cdp.mjs` is untracked scratch. It has now driven four
investigations and this gate depends on it, so **it moves into the repository**
— location yours, but tracked, and with its own limits documented the way this
project's scan modules document theirs.

**Its known limits, to carry across**: it drives one page at a time, it waits on
`document.readyState`, and it has no notion of animation completion.

## 4. `DEC-048`, condition by condition

1. **No external network.** The harness spawns a local instance, as `TestApp`
   does. **`ASSET-001` is what made this possible** — until 0.32.0 every page
   fetched CSS from two CDNs, and a gate that fails because jsdelivr is slow is
   the gate this condition forbids. **Confirm no page requests any external
   origin** — `Network.setBlockedURLs` on `*` for everything but `127.0.0.1`,
   and the run must be unaffected.
2. **No wall-clock dependence.** **Wait on a signal, never on a duration.**
   `readyState === 'complete'` plus the presence of a specific element; no
   `setTimeout` as a synchronisation primitive. If a page needs a settle delay to
   measure stably, **that is a finding about the page**, not a licence to sleep.
3. **A flake is a defect with an owner.** Decide *now*, in the job's own
   documentation, where a quarantined check is recorded and who owns it. **No
   silent retries** — no `continue-on-error`, no re-run step, no matrix retry.
   `§10.13`'s defect survived four releases because re-running was the response.
4. **A quarantined check is not coverage.** If anything is quarantined at a
   release, the release candidate names it.

## 5. Fixtures — and one lesson to build in

Create a user, a project and issues **through the real forms**, as the
investigation scripts did.

**Include content, and include a long issue title.** `§10.21` — a 314 px overlap
— is invisible until a project has issues, and my own ten-page sweep missed it
**because the fixtures were empty**. A layout gate with empty fixtures measures a
layout nobody sees.

**Also include a long signed-in email.** `LAYOUT-001` was conditional on exactly
that, and a short-email fixture would not have caught it.

## 6. Coverage

Pages: `/today`, `/inbox`, `/today/calendar`, `/projects`, a project detail with
issues, the board, an issue detail, `/settings`, `/settings/notifications`,
`/teams`, `/search`, a delete interstitial.

Widths: **390, 768, 1280, 1920**.

**Report every failing cell, not the first.** A gate that stops at the first
failure hides the shape of the problem.

## 7. Verification

**Plant the two defects this gate exists to catch**, separately:

1. Restore `LAYOUT-001`'s cause — remove `min-w-0`/`overflow-hidden` from the
   account menu's `menu-title` — **with a long-email fixture**. The gate must
   fail and name the page.
2. Restore `LAYOUT-002`'s cause — put `shrink-0` back and drop `flex-wrap` on the
   project toolbar. The gate must fail at 390 and pass at 1280.

**Then confirm it passes on the current tree**, at every cell.

**And run it three times.** Not as a determinism proof — `§10.13` is on record
that a repeated run is not that — but because condition 3 makes an unstable
first result something to find now rather than after it has cost attention.

## 8. Escalate rather than deciding

- **If any cell is non-zero on the current tree.** That is a defect, and it is
  the gate working before it exists.
- **If a page needs a delay to measure stably** (condition 2).
- **If the runner's Chromium version turns out to change a result.** Overflow was
  chosen partly for version-stability; if that proves wrong, the choice of first
  check is wrong and I would rather know than pin a browser.
- **If making the harness tracked pulls in a dependency.**

## 9. Exit condition

The job exists, runs on every push, has no retry, and fails on both plants. Zero
overflow on the current tree across twelve pages × four widths. `DEC-007`
unchanged at **255** — **this handoff adds no Rust test**, and if the count
moves, something was misunderstood.

---

**Who holds what**: dev team — the gate. **What's blocked**: nothing.
**What's next**: review request.
