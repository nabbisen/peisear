# Handoffs — where unscheduled work lives

**Not RFC-governed.** Found while scoping 0.48.0: the architect has been
reciting the unscheduled-work list **from memory**, and two of the recited
items were wrong in different ways.

| ID | Link | What | Release |
|---|---|---|---|
| ROADMAP-001 | [the current plan ended twenty releases ago](./ROADMAP-001-the-current-plan-ended-twenty-releases-ago.md) | `ROADMAP.md` says *"the current plan is the Milestones, Release sequence and Release cycle sections below"*. **That release sequence ends at `0.27.0`; we shipped `0.47.0`.** Not a stale-file complaint: **the way work is chosen changed** — handoffs, the register, the sweep — and the roadmap did not move with it. **A decision handoff**: what is the document now, and where does *unscheduled quality work on shipped things* live? The register and `§11` already handle defect classes and deferred features; that fourth row has **no home**. | 0.48.0 |
| POST-001 | [where a POST leaves the reader](./POST-001-where-a-post-leaves-the-reader.md) | **An investigation; the fix shape decides handoff-versus-RFC.** `Redirect::to` at **51 call sites across six handlers** — so landing at the top of a page is product-wide by construction, not the calendar's. Measure three chosen cases on a phone width with numbers, name the fix shape, and say which this is. **"The skip link already covers it" closes the item** and is worth as much as a fix. | 0.48.0 |

| NAV-002 | [a fragment for the row you acted on](./NAV-002-a-fragment-for-the-row-you-acted-on.md) | **`POST-001`'s case 1, commissioned.** A fragment anchor on the sprint-plan row's own id, so a move stops discarding **1292 px of 3078** on a phone. No script, no `FR-NAV-005` collision. **Scoped to redirects landing on a page the row still appears on** — the other 49 `Redirect::to` sites are untouched, and the handoff's first section is the reason: a uniform fix would read as solved while doing nothing for the calendar case that raised this. | 0.48.0 |

## The two recited items that were wrong

**One did not exist.** *"`FR-DM-005`'s keyboard undo (RFC-sized)"* was carried
across many releases. **`FR-DM-005` is *Conflict message vocabulary* and is
`Implemented`**; no keyboard-undo requirement exists anywhere —
`NFR-A11Y-009` is list navigation shortcuts, and undo's keyboard path was
settled at 0.41.0 and is now **measured** on two surfaces by `GATE-006`. **A
label carried across releases without once being checked against the thing it
named** — this thread's own recurring class, in the planning record instead of
in code.

**One is recorded nowhere a clone contains.** The POST landing lives only in
`.git-exclude/reviewed/`, which is gitignored.

**Not in 0.48.0, with the reasons rather than silence:** `§10.32` is **open
by decision with a stopping rule** and is doing what it should; `NFR-MNT-004`
was **re-measured** at 11 of 86 files over 500 effective lines against 10 of
85 six releases earlier and stays P3; the **117 entries citing no mechanism**
is a third audit and `§10.35`'s remedies should settle first; the **real
OS-level drag gesture** is declined with its capability gap recorded in
`§10.15`, and reopening it is the owner's.
