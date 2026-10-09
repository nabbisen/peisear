# Handoffs — an acceptance narrower than the requirement

**Not RFC-governed.** `§10.35`, opened at 0.46.0. `NFR-A11Y-007` says **every
interactive element** presents a 44 × 44 px touch target and cites
`touch_target_scan` as its mechanism. **That guard reads three tag names of
the six that matter** — 161 elements in its population, **107 outside it**.
The requirement reads `Met`, the guard is green, and both are true.

Found by accident, at the edge `CAL-004` walked past when it added an
`<input>` at 0.45.0 — recorded then with the gap's size unmeasured, and
therefore understated.

| ID | Link | What | Release |
|---|---|---|---|
| TT-007 | [the guard reads three tags of six](./TT-007-the-guard-reads-three-tags-of-six.md) | Widen `TAG_NAMES` to all six and resolve what it newly covers. **Exposure is 17 visible elements with no declared target** — 3 inputs, 5 selects, **all nine `<textarea>`s** — since 32 of the 107 are `type="hidden"` and 58 declare one anyway. **The architect's 17 is a heuristic**: count by class first, `DOCS-002`-style, and do not trust it. `type="hidden"` must be excluded **by attribute, not by an exception list**, and the existing one-site exception list does not grow without a ruling. | 0.46.0 |
| REQ-004 | [does the acceptance cover the requirement?](./REQ-004-does-the-acceptance-cover-the-requirement.md) | **An investigation; the deliverable is a count and a list.** `REQ-001` audited every **status** that was a claim and found sixteen stale. **Nothing has ever audited an *acceptance*.** For every entry citing a mechanism: does the mechanism exercise what the requirement's own text says, over the population it names? **If `§10.35` turns out to be the only instance, that is a complete and valuable result.** | 0.46.0 |

**Why these two together.** `TT-007` closes the instance; `REQ-004` asks
whether the instance is a class. Fixing one guard and declaring the question
answered is how `§10.34` happened — a correction made from the record rather
than from the code, and no check on whether the same thing sat next door.
`§10.35` therefore **does not close on `TT-007`**.

**Not in 0.46.0, and the reasons rather than silence:**

- **`NFR-MNT-004`** (file size) — **re-measured and left P3.** 11 of 86 files
  over 500 effective lines against 10 of 85 six releases ago; the largest grew
  18 lines. The figure barely moved and the entry says so now. *A raw
  `wc -l` gives 18, not 11 — this entry counts **effective** lines, and the
  definition is part of the number.*
- **The top-of-page landing after any POST** — measured during `CAL-004`,
  product-wide, and genuinely unscoped: the fix might be a fragment anchor,
  scroll restoration, or JavaScript, and that difference decides whether it is
  a handoff or an RFC. **Two investigations plus a fix is enough for one
  release**; this is next.
- **`FR-DM-005`'s keyboard undo** and **`§10.32`'s remaining fixture
  branches** — both still RFC-sized or decision-gated, neither blocked by
  anything here.
