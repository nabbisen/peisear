# POST-001 — where a POST leaves the reader

**An investigation. The deliverable is a measurement and a scoping
judgement — the fix shape decides whether this is a handoff or an RFC, and
that is the thing to establish.** **Target**: 0.48.0.

**Measured during `CAL-004` and never written anywhere a clone contains**
(that is `ROADMAP-001`'s subject, not this one's). The finding was: after the
calendar Move, *"you land at the top of the page"*, and the dev team
correctly identified it as **the ordinary Post/Redirect/Get behaviour every
mutation path in this product already has** — not something that route
introduced.

**`Redirect::to` appears at 51 call sites across six handler files**
(`teams` 12, `sprints` 11, `issues` 8, `settings` 6, `notifications` 4,
`projects` 3). So this is product-wide by construction, not a calendar
problem.

---

## §1 — the question, and why it is not obvious

> **What does a reader lose when a POST returns them to the top of a page
> they were partway down, and is it worth a mechanism?**

**On a desktop this is often nothing.** On a phone it can be the whole
context: `NFR-A11Y-011`'s own evidence records a reader at **605 px on a
1,697 px page** losing their place after a native POST, which is why the skip
link exists — **but a skip link solves reaching the content, not returning to
where you were.**

**And it is not uniformly bad.** Some POSTs *should* land at the top: a
create that lands on a new object, a delete that returns to a list. **The
measurement must distinguish them**, or the conclusion will overstate.

## §2 — what to measure

**Do not survey all 51.** Choose a spread and say why you chose it:

1. **A mutation that returns you to the same long page you were reading** —
   the sprint plan or a long issue list. This is the shape that loses
   context.
2. **A mutation that returns you somewhere new** — a create, a delete. Likely
   fine, and establishing *fine* is half the result.
3. **The calendar Move**, since it is what raised this and `CAL-006` already
   put a flash on its landing page.

For each: **on a phone-width viewport**, how far down was the reader, where
do they land, and **what is now off-screen that they were looking at?**
Numbers, like `NFR-A11Y-011`'s.

**The browser harness can do this** — `cdp.mjs` scrolls, and
`drag-outcome-gate.mjs` already drives a real mutation and reads the page
afterwards. **Use it rather than reasoning about it**; this project has
measured scroll positions before.

## §3 — the fix shapes, and why naming the right one is the deliverable

Three, roughly in order of cost, and **the choice decides what this becomes**:

- **A fragment anchor on the redirect** — `#issue-42`, so the browser lands
  on the acted-on element. Cheap, no JavaScript, works with the no-JS path
  this product protects. **But it changes the URL**, and `FR-NAV-005` makes
  the URL the carrier of list state — so check it does not collide.
- **Browser-native scroll restoration**, which applies to back/forward and
  **not** to a POST redirect — likely a dead end; **establish that rather
  than assuming it.**
- **Script-assisted restoration** — and this is the one to be careful about:
  every keyboard and no-JS path in this product works **without** script, by
  design and repeatedly defended. A fix that only works with JavaScript
  enabled would be the first. **If that is the only workable shape, say so
  plainly** — it is an owner decision, not a detail.

**If the answer is that the fragment anchor covers the cases that matter**,
this is a small handoff and you should say so. **If it needs a mechanism
across 51 sites, or needs script, it is RFC-sized** and this investigation
has done its job by establishing that.

## §4 — what this is not

- **Not a fix.** Write none. Measuring one case well beats proposing three.
- **Not a survey of 51 routes.** Three well-chosen cases with numbers.
- **Not an accessibility-conformance claim.** `NFR-A11Y-011` is Met and this
  is adjacent to it, not a reopening — **if your measurement suggests
  otherwise, that is an escalation**, not a line in the report.

## §5 — escalate rather than deciding

- **If the only workable fix needs JavaScript.**
- **If a fragment anchor collides with `FR-NAV-005`'s URL state.**
- **If any case turns out to lose something worse than scroll position** —
  focus, or a form's unsaved content.
- **If `NFR-A11Y-011`'s skip link already makes this a non-issue in
  practice** on the cases you measure. **That is a complete and welcome
  result**, and it would close the item rather than scope it.

## §6 — gates

Nothing is built: `DEC-007` confirmed once at **405**, unchanged; empty diff.
**Any harness script you write to take the measurement stays in the review
package**, not in `browser-checks/` — an unwired script beside gated ones
reads as coverage, which is the reasoning that wired `undo-mousedown-trap.mjs`
in the first place.

## §7 — exit condition

Three measured cases with numbers on a phone width, a named fix shape with
its cost, and **a plain statement of whether this is a handoff or an RFC**.
A result of *"the skip link already covers it"* closes the item and is worth
as much as a fix shape.
