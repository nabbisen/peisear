# GATE-006 — one script, three properties

**`GATE-005`'s proposal, accepted.** Candidates 1–3 as **one** script,
candidate 4 **declined on the record** (`§10.15` now carries it as the named
remainder, with its capability gap, so nobody re-proposes it).

**Requirement**: `NFR-A11Y-008` (P1) — whose acceptance now **states its own
limit** pending this script. **Register**: `§10.15`, `§10.35` (closes on
this), `DEC-048`. **Target**: 0.47.0.

**Your own reasoning is the specification here.** This handoff adds the
ruling, the one risk you named, and the gates — it does not re-derive your
proposal.

---

## §1 — what is accepted, and the one thing reordered

**One script, three assertions, port `4175`.** Your §3 argument carried it:
the three share one setup, and widening `undo-mousedown-trap.mjs` in place
would be the shape `DEC-048` condition 3 warns against **even though its
letter names only `overflow-gate.mjs`** — reasoning from the rule's purpose
rather than its wording.

1. **The live region receives the outcome text** — `calendar.rs` and
   `sprint_plan.rs`, polite and assertive. **This is why the script exists**:
   `NFR-A11Y-008` cannot otherwise cite a mechanism matching its text.
2. **The undo toast's Tab order, measured.** I would have ranked this third
   and you ranked it second, correctly: it closes a **standing, written gap**
   — `COV-001`'s own *"neither instrument walks the Tab order in a browser,
   so 'Undo is the next Tab stop' remains an inference from DOM
   attachment"* — which has sat in the record since 0.46.0 with nothing
   against it. A cheap assertion against a gap we already wrote down beats a
   speculative one.
3. **The drop's stored effect**, read back over HTTP.

**Assert all three from one drop where that is honest.** If a surface needs
its own drop to make an assertion meaningful, use one and say why.

## §2 — the risk you named, now the handoff's

Your §6: *not measured — whether the synthetic-`DragEvent` technique fires
`calendar.js`'s and `plan.js`'s drop handlers without per-surface tuning*,
since the existing script exercises `dm.js` and `board.js` only.

**That is this handoff's main risk and it is named here so it cannot surprise
either of us.** Two consequences:

- **Establish the drop fires before asserting anything about its
  consequences.** A region that stays empty because the handler never ran is
  indistinguishable, from the assertion's side, from a handler that ran and
  announced nothing. **Assert the drop took effect first** — the stored
  change, candidate 3 — and only then the region's text. Ordering the
  assertions that way makes a setup failure legible instead of looking like a
  defect in the product.
- **If a surface cannot be driven honestly, stop and report it.** Do not
  approximate a drop by calling the script's own internal function or by
  POSTing the endpoint — either would make the gate assert something a Rust
  test already covers, which is `§10.35`'s defect in a new place.

## §3 — what this must not become

- **No new CDP capability.** Candidate 2's `Input.dispatchKeyEvent` goes
  through `cdp.mjs`'s existing generic `send`, the same way
  `undo-mousedown-trap.mjs` calls `Input.dispatchMouseEvent` (`:245-256`).
  **If you find yourself adding a domain wrapper, that is candidate 4
  arriving by the back door** — stop.
- **No widening of either existing script.**
- **Not a screen-reader assertion.** You drew that line yourself: the
  accessibility tree and real AT are out of scope, and `NFR-A11Y-008`'s entry
  now records it. **A gate claiming to prove an announcement was *heard*
  would be worse than the gap.**
- **Not `notifications.rs`.** Your §0 established it is server-rendered and
  out of this requirement's scope; its entry is corrected. **If you think a
  Rust test is owed there, say so** — do not build it here.

## §4 — gates

- **The script seen to fail, per assertion.** Three plants, not one: suppress
  the region write (candidate 1 fails, candidate 3 still passes — which is
  the proof the two assertions are independent); move the undo button out of
  Tab order; and break the drop's target resolution. **Quote each failure.**
- **Wired into the existing job** as a third step, after the trap script, with
  the job's comment extended. `browser-checks/README.md` gains it as a third
  gate, in the shape the second one has.
- **Both existing scripts still clean** in the same run, in order — three
  servers, three ports, each tearing down its own. **Say explicitly that the
  third does.**
- `DEC-007` unchanged at **405**; this adds no Rust test. **If the count
  moves, something unintended happened.**
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.
- **Overflow gate** only if markup changes; expected not to.

## §5 — escalate rather than deciding

- **If the synthetic drop cannot drive `calendar.js` or `plan.js`.**
- **If a region turns out never to receive text** on either surface — that is
  a defect in a P1, not a test problem, and it stops this handoff.
- **If the Tab order turns out not to put Undo next** — `FR-DM-006` is `Met`
  partly on the inference this assertion replaces, so a counter-example is a
  finding about that entry too.
- **If three servers in one job prove unreliable.**

## §6 — exit condition

One script at port `4175`, wired as the job's third step, asserting the
region's text on both untested surfaces, the Tab order, and the drop's stored
effect — **each seen to fail on its own plant**; both existing gates still
clean in the same run; `README.md` updated; `DEC-007` still 405.

**Then `NFR-A11Y-008`'s acceptance cites a mechanism matching its text**, and
**`§10.35` closes** — which is mine to record, on your evidence.
