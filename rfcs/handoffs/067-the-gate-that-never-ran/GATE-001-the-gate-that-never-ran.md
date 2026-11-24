# `GATE-001` — The gate that never ran

Series [067 — the gate that never ran](./README.md), **round 1 of 3.**
Read `README.md` first: §1.1 is this round's measurement, §3 is what every
round owes.

**This round wires a passing gate and then makes the class impossible.** It
touches no component and no stylesheet.

---

## 1. The defect

`browser-checks/disclosure-marker-gate.mjs` **has never been run by
anything but a human typing its name.**

- `git log -S 'disclosure-marker-gate' -- .github/workflows/test.yml`
  returns **no commits**. It was never wired and then unwired; it was never
  wired.
- Walking **every** tracked script outside `crates/` (README §1.1's second
  command, not a sample) it is the **only** gate absent from every workflow.
- It is named as a required re-run by `components/me.rs:358`,
  `components/issues.rs:386`, five passages of
  `docs/specification/requirements.md`, and eight handoffs in series
  061–065.

**What it guards, that nothing else can.** Thirteen `<summary>` controls
must keep `display: list-item` and a ≥44×44 target at two widths.
`touch_target_scan` reads **which helper** a `<summary>` was given;
`grow_text()`'s `inline-flex items-center` suppresses the native `::marker`
box, because a marker exists only on `display: list-item`. **Only the
computed style answers**, and only this gate reads it.

**It passes today** — run at the proposal, exit `0`:

```
[disclosure-marker-gate] all thirteen checkable summaries (DISC-001 three +
MARK-001 six + MARK-002 four): list-item display, >=44x44, at both widths.
```

**Verify that yourself before wiring it.** If it fails on your tree, that
is the more important finding and this round changes shape: say so and stop.

---

## 2. Wiring it

The job is `browser-overflow-gate` in `.github/workflows/test.yml`
(line ~138), whose four steps today are `overflow-gate`,
`undo-mousedown-trap`, `drag-outcome-gate`, `occlusion-gate`.

**Three facts that make this safe, each checkable:**

1. **Ports.** This gate defaults to `4233`
   (`disclosure-marker-gate.mjs:73`); the four in the job use `4173`,
   `4174`, `4175`, `4177`. No collision. **But `PEISEAR_PORT` is read by
   all of them** — the README's own warning — so do not set it in the job.
2. **Isolation.** Each script starts and tears down its own server and
   scratch database, which is why they share one job. Confirm this one does
   the same rather than assuming it from the others.
3. **The job's name string** says what it runs
   (`name: browser checks (overflow gate + undo mousedown trap + drag
   outcome + occlusion)`). **It is now wrong in the other direction**: a job
   whose name omits a step it runs is the same defect in miniature. Update
   it.

**Also correct `browser-checks/README.md`.** Its *Running them locally*
block lists four scripts and its port paragraph says *"The four scripts
default to different ports"*. Both become five. The sentence that
`doc-screenshots.mjs` *"must not be added to the `browser-overflow-gate` CI
job"* is right and stays — it asserts nothing and is not a gate.

---

## 3. The guard that keeps gates wired — and the trap in its population

**Recommendation: one test asserting that every gate in `browser-checks/`
appears in a `test.yml` run line.** The machinery exists:
`dec_007_ci_scan::test_yml_run_lines()` already parses those lines, and
`css_scan` already walks `browser-checks/` and panics if it finds no
`*.mjs`. Nothing joins them.

**The trap, and it is the whole design of this test.** A `*-gate.mjs` glob
looks like the population and **is not**:

| file | a gate? | ends `-gate.mjs`? |
|---|---|---|
| `overflow-gate.mjs` | yes | yes |
| `drag-outcome-gate.mjs` | yes | yes |
| `occlusion-gate.mjs` | yes | yes |
| `disclosure-marker-gate.mjs` | yes | yes |
| **`undo-mousedown-trap.mjs`** | **yes, and wired** | **no** |
| `doc-screenshots.mjs` | no — asserts nothing, README says so | no |
| `cdp.mjs` | no — the shared CDP harness | no |

**A glob-based test would have passed while `undo-mousedown-trap.mjs` was
silently outside its population** — the same shape as the defect it is
built to prevent. **So the population is every `*.mjs` in the directory,
minus a named exclusion list carrying each file's reason**, the
acknowledgement shape `PLC-003` and `EDG-003` already use: two entries
today, `cdp.mjs` (library) and `doc-screenshots.mjs` (not a gate — quote
the README's own words).

**Decide, with reasons, and say which you chose**: a test inside
`dec_007_ci_scan` (reuses the parser; its module doc then covers two
inventories) or its own small module (one concern each; a second reader of
`test.yml`). I lean to the first — *less is more*, and the parser is the
hard part — but the round reads both and decides.

**Plant it both ways** (README §3.2): the test must fail when a gate is
removed from the job, **and** when a new `*.mjs` gate appears that nothing
runs. The second plant is the one that matters; the first is the easy
direction.

---

## 4. Acceptance

1. `disclosure-marker-gate.mjs` **runs in CI** and the job's own name
   string names it.
2. **It passed locally before it was wired**, with the output quoted — not
   wired on this handoff's word.
3. The new test exists, **planted both ways**, each failure quoted by name.
4. Its population is **not** a `*-gate.mjs` glob, or it is, with an argument
   for why `undo-mousedown-trap.mjs` is nonetheless covered.
5. `browser-checks/README.md`'s *four scripts* and *Running them locally*
   both updated.
6. `DEC-007`'s block in full; state the new count and what moved it
   (**694** at `0.68.0`).
7. CI green at the per-job level on the pushed commit — **read the jobs,
   not the summary**; and the new browser job is now one step longer, so
   confirm it ran five steps.
8. `CHANGELOG.md` under `[Unreleased]`: what a reader could not rely on.
   **`Fixed`, not `Added`** — the gate existed; what was missing was anyone
   running it. One bullet, in the register's plain-reader voice.

**What this round must not do**: touch a component, a stylesheet, or
`NFR-REL-010` (that is `IMG-001`), and must not add `doc-screenshots.mjs`
to CI.

---

## 5. One thing to disagree with, if the tree says so

This handoff asserts the gate **passes**. The architect ran it once, on one
machine, with `chromium` from `$PATH`. **The CI runner is the real
measurement**, and a gate that passes locally and fails there is a finding
about the gate's own assumptions — report it as this round's substance
rather than patching around it. `§10.54` is open for exactly this reason:
the browser gate failed on a documentation-only commit and nobody was
watching.
