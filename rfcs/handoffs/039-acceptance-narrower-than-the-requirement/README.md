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

| REQ-005 | [the four left unresolved](./REQ-005-the-four-left-unresolved.md) | **`REQ-004`'s own named limit, closed.** It resolved 44 of 48 entries and named **four** it triaged by text and test *name* only, without opening the code — as *unresolved*, not as `Covers`. **First in 0.46.0**, for two ordering reasons: anything found joins the same specification amendment (amending twice is `§10.34`'s churn), and **`NFR-PRIV-002` is a P1 privacy requirement at *plausible under-coverage, not confirmed***. **`FR-API-002` is the one to watch**: a **P0** prohibition whose cited acceptance is the *positive* case — *self can read own, 200*. | 0.46.0 |

| COV-001 | [the four small gaps](./COV-001-the-four-small-gaps.md) | **Seven tests, four requirements, no judgement calls.** `NFR-PRIV-002`'s tooltip, `FR-SUB-002`/`-003`'s three untested trigger conditions, `FR-PER-007`'s self-expanding panel, and `FR-NAV-003`'s two unasserted detail screens. **Carries a correction to `REQ-005`'s own reasoning**: `assert_no_capacity_leak` scans **raw HTML**, attributes included, so the tooltip was never invisible to it — the gap is its **finite vocabulary**, and the fix is a **positive** assertion rather than a fifth negative that would add nothing. | 0.46.0 |

| NAV-001 | [team detail has no back link](./NAV-001-team-detail-has-no-back-link.md) | **The one live compliance defect in `§10.35`'s twelve**, found by `COV-001` while writing the test meant to prove coverage. `TeamDetailPage` hand-rolls its trail and calls **neither** shared helper — no `/today` root, no `aria-current`, and **no back link at all**, which `FR-NAV-003`'s second conjunct requires. The architect's own citation fix had framed it as missing proof, written from a summary without opening `teams.rs`. **Write the test first and watch it fail** — the defect supplies its own demonstration. | 0.46.0 |

| REL-0.46.0 | [candidate](./REL-0.46.0-release-candidate.md) | **The release — and it is 0.46.0, not 0.47.0.** `0.46.0` was never cut: the last tag is `0.45.0` and this whole thread is one unreleased body of work, while the architect had been labelling amendments for two releases that do not exist. Relabelled at `f260b8b`. Expect **392** from 383 — nine net, with `TT-007` adding **none** because it widened a test in place. | 0.46.0 |

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
