# BROWSER-002 — the gate's fixture, and 320 px

**Governing**: `DEC-048`. **Target release**: 0.33.0.
**Source**: `LAYOUT-003`'s §7, accepted in review; `§10.25`.

## 1. The gate is green on a tree with three overflow defects

`BROWSER-001` sweeps issue detail at 390 and 414. **Issue detail overflows by
624 and 600 px at those widths** on an issue whose title contains a long
unbroken run. The gate reports 48/48 clean.

**The reason is one character class.** The gate's fixture title — *"An issue
whose title is long enough on its own to…"* — has a space at every point, so no
element ever receives an unbreakable run to mishandle. `LAYOUT-001` was
conditional on an unbroken email; `LAYOUT-003` and `LAYOUT-004`'s three surfaces
are conditional on an unbroken title. **The gate's fixture cannot produce the
input that four of the five layout defects this project has found depended on.**

## 2. Two changes, and the order matters

**First — the fixture.** Give the fixture issue a title containing a **long
unbroken run** (64 characters or more), and give the project and team names one
too. This is the higher-value change: it makes issue detail fail **at widths the
gate already covers**, with **no new cells**.

**Second — 320 px.** Add it as a fifth width. This is what would have caught
the board (`LAYOUT-003`). Twelve more cells; `DEC-048`'s cost conditions are
unchanged by it.

**Land the fixture change first and watch the gate go red on issue detail before
`LAYOUT-004` lands.** That red run is the evidence that the fixture change works
— if the gate stays green with the new fixture on the unfixed tree, the fixture
is still not reaching the defect. **Do not order the commits so that the fix
lands before the fixture; that would make the fixture change unverifiable.**

## 3. What must not change

- **Still one assertion.** `scrollWidth <= clientWidth`. This handoff changes
  what the gate *sees*, not what it *asserts*.
- **No retry, no `continue-on-error`.** `DEC-048` condition 3.
- **Still not in `cargo test`.** `DEC-007` stays at 256.

## 4. Verification

- On the tree **before** `LAYOUT-004`: the gate **fails** on issue detail at
  390 and 414 with the new fixture. Report the cells.
- On the tree **after** `LAYOUT-004`: **60/60** (12 pages × 5 widths), zero
  non-local requests.
- Both of `BROWSER-001`'s original plants still fail the gate.

## 5. Escalate rather than deciding

- **If the new fixture surfaces an overflow on a page `LAYOUT-004` does not
  cover.** That is a sixth site, and it goes to `LAYOUT-004`'s author as a
  finding, not into this handoff.
- **If 320 fails on any page other than the board.** Same.

---

**Who holds what**: dev team. **Depends on**: nothing to start; **sequence
against `LAYOUT-004`** as §2 says. **What's next**: review request.
