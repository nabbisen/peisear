# TT-006 — the touch-target guard's doc says it has no exception list; it has one

**Target release**: 0.34.0. **Governing RFC**: RFC 012 (`done/`), `DEC-050`.
**Source**: found 2026-09-12 while reviewing `LAYOUT-009`; `§10.28`.

## 1. The defect

`crates/peisear-web/src/touch_target_scan.rs`'s module doc says, in bold:

> **No exception list.** `TT-002` round 2 converted the last three hardcoded
> `min-h-11 min-w-11` literals specifically so this guard would need none. If
> a future change needs one, that is a finding to report, not a line to add —
> this module's own `checkbox-xs` history already states why: a rule that
> fails on a correct tree gets weakened until it passes, and an exception list
> is how that weakening looks in practice.

**The module has had an exception list since `TT-004`.**
`is_named_escalation_exclusion` carries two class-string arms covering the
three call sites `DEC-050` excluded by design — the calendar's event chips and
the per-indicator "why" toggle.

**Nothing is broken, and the list is not the problem.** It is correct, it was
decided by `DEC-050` rather than added quietly, and it is documented
thoroughly — at the predicate, **470 lines below the paragraph that says it
does not exist**. The module doc never mentions it.

**The paragraph's own words describe what then happened.** A future change did
need one; it *was* reported as a finding (`TT-004`, three rounds); the decision
*was* taken by the owner; and a line *was* added. The doc should say that
rather than deny it.

This is the `§10.14`/`§10.16` class — a comment that reads as stricter or more
complete than the code — sitting in the guard a reader consults to learn what
the rule is. That is the reason it is worth a handoff at all.

## 2. What to do

Correct the module doc. Roughly:

- **Keep the paragraph's reasoning**, which is right and is why the list is
  two entries rather than twelve: a rule that fails on a correct tree gets
  weakened until it passes.
- **Say what is actually true**: the size-clause assertion
  (`every_sizing_class_site_composes_the_touch_target`) needs no exceptions,
  and that is what `TT-002` round 2 bought. The coverage assertion
  (`every_interactive_element_declares_a_touch_target`) has **one declared
  exception** (`data-inline-text-link`, `DEC-050`) and **two named escalation
  exclusions** covering three call sites, each with a recorded reason and each
  removable by deleting an arm.
- **Point at `is_named_escalation_exclusion`** so a reader of the module doc
  reaches the list rather than discovering it 470 lines later.
- **Keep the distinction the code already draws**: a declared exception is a
  property of the rule; a named escalation exclusion is a decision about three
  specific call sites. Collapsing them into one word would lose what `DEC-050`
  settled.

**Scope: the doc comment only.** Do not change the predicate, the arms, the
assertions, or any call site. If while reading you find a *third* place the doc
and the code disagree, report it rather than folding it in.

## 3. Verification

- The corrected paragraph names both assertions, both kinds of allowance, and
  the predicate.
- `cargo doc -p peisear-web --no-deps` builds without a broken intra-doc link
  if you add one.
- `DEC-007` **256**, three consecutive workspace runs, `fmt`, `clippy`. **No
  behaviour changes**, so the count must not move.

## 4. Escalate rather than deciding

- **If the doc and the code disagree anywhere else** in this module.
- **If correcting it honestly requires saying the exclusions should close.**
  Whether they close is `TT-004` §5's decision and mine to revisit, not
  something a doc fix settles.

## 5. Exit condition

One paragraph, accurate, pointing at the list; nothing else touched; 256 on
three runs.

---

**Who holds what**: dev team. **Depends on**: nothing. **What's next**: review
request.
