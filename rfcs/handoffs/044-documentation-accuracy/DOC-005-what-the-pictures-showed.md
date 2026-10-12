# DOC-005 — what the pictures showed

**Two things the screenshots exposed, in this order**: the alignment defect
first, the empty sprint plan second, **so the fourth screenshot is captured
after the fix rather than before it** — the ordering `DOCS-008` used for the
brand string, for the same reason.

**Reviews carried**: `.git-exclude/reviewed/DOCS-008-review.md` §4,
`.git-exclude/reviewed/DOCS-007-review.md` §6.
**Target**: 0.51.0, last round before the candidate.

---

## §1 — the alignment defect, and its actual root

**The symptom**, published in two of the three committed screenshots:
*"(set your capacity)"* sits **above** `WORKLOAD` rather than beside it
(`components/issues.rs:533-539`), and a sub-issue's status badge sits
**below** its title (`:1611-1619`).

**The mechanism.** `grow()` (`components.rs:148`) appends
`TOUCH_TARGET = "min-h-11 min-w-11"` **and nothing else**. Inside a
`flex items-center` row, the element is blockified, takes a 44px box, and
**its text sits at the top of that box** while the parent centres the box.
A shorter sibling centres. The two labels end up 10–14px apart.

**The root is not the helper. It is `DEC-049`.** Reconstructed from its
citations, because nothing defines it: the 44px target as
`min-h-11 min-w-11`, `Grow` as the default pattern, and —

> *"a `Grow` cluster inside a positive-`gap` container is **presumed** to
> satisfy clause (2) with no further verification."*

**A presumption written into a rule**, applied everywhere, and the first
time anyone looked at the result was when a screenshot was published two
weeks ago. **Revisit the decision; do not only centre the content.**

## §2 — measure before changing anything

**`TT-002` recorded 136 call sites. Measured today: 219** `class=grow(…)`
sites across the components — `issues.rs` 56, `sprints.rs` 33, `teams.rs`
26, `settings.rs` 18, `projects.rs` 16, `layout.rs` 10, `auth.rs` 9, and the
rest. **The number grew with the product and the record never moved**; that
is worth one line in your package independent of the fix.

**Sort the 219 into three kinds**, because the fix differs per kind:

1. **Already centring** — anything whose class list makes it a flex
   container (`btn` is `inline-flex` with `items-center` in daisyUI). The
   44px box centres its own label. **No defect, no change.**
2. **Form controls** — `input`, `select`, `textarea`. These centre their
   own content natively and **must not** be given `inline-flex`.
3. **Plain text elements** — a bare `<a>` or `<span>` with a label. **This
   is the defect class**, and both published instances are in it.

**Report the three counts.** The fix you choose should follow from them, not
from my guess — and if kind 3 turns out to be two sites rather than fifty,
say so, because that changes the answer.

## §3 — the fix, with the shape I expect and room to disagree

**If kind 3 is substantial**: `grow()` gains `inline-flex items-center`, and
**kind 2 gets a separate path** — a `grow_control()` or an explicit
`TOUCH_TARGET` composition at those sites — because adding `inline-flex` to
an `<input>` is wrong.

**If kind 3 is a handful**: fix them at the call site and leave `grow()`
alone. **Two names for one idea is a cost this project has refused before**
(`DOCS-007` §1, the fixture), and it is only worth paying if the measurement
says so.

**The scan does not constrain you**: `touch_target_scan` reads
`components::TOUCH_TARGET`/`grow` **as a symbol**, not as a class pair at
219 sites, so changing what they emit keeps it passing. **It also will not
catch a regression here** — it checks the touch target, not the alignment.

## §3.1 — ruling on the escalation (2026-10-13)

**Your count stands, mine does not.** 212, not 219 — mine was a raw
`class=grow(` grep that swept in the scan's own fixture strings. **Two
instances of the same drift in one round, and both are mine.**

**Your split: 98 already centring · 67 form controls · 47 plain text.**

**Ruling: `grow()` is unchanged. Add a second helper for kind 3.**

**And the reason is not that 47 is substantial.** It is that **one helper
cannot be correct for all three kinds**, which I did not know when I wrote
§3 and which your split makes checkable:

- **`static/tailwind.css` emits `.flex{display:flex}` immediately before
  `.inline-flex{display:inline-flex}`.** Later rule wins, **class order on
  the element is irrelevant** — so adding `inline-flex` to `grow()` would
  flip every kind-1 site that carries a bare `flex` class from block-level
  to inline-level flex. **Measured: 19 of them do.**
- On the **67 form controls** it is meaningless at best.

**So the trade I warned about — "two names for one idea is a cost this
project has refused before" — is not the trade.** The alternative to two
names is not one name; it is one name that is wrong at 86 sites.

**Shape:**

- **`grow()`** keeps its meaning and its 165 sites: the touch target, and
  nothing else.
- **`grow_text()`** = `TOUCH_TARGET` **+ `inline-flex items-center`**, at
  the 47 kind-3 sites. The name says the kind, not the mechanism.
- **`TOUCH_TARGET` stays one symbol**, composed by both, so
  `touch_target_scan` keeps reading one thing.

**Verify the stylesheet claim yourself before relying on it** — I read the
emitted order out of `static/tailwind.css` and counted the bare-`flex`
sites with a grep. **Both are my measurements and this round has already
corrected two of them.**

**§4's guard gets simpler, and is still yours to decline**: with two
helpers the rule is *a bare text element uses `grow_text()`*, rather than a
taxonomy inside one name.

## §4 — a guard, because the screenshots cannot be the only control

`touch_target_scan` would not have caught either instance, and neither would
the overflow gate: both measure whether content escapes its box, not whether
two labels in a row share a baseline.

**Proposed, in the existing scan style**: every `grow()` call site's class
string either makes the element a flex container, **or** the element is a
form control that centres natively. A text scan over the same sites the
touch-target guard already enumerates.

**If that taxonomy cannot be expressed without a list of exceptions longer
than the rule, say so and do not build it.** A guard nobody can read is
worse than the screenshot review it replaces.

## §5 — then the sprint plan, which sweeps empty

`sprint_plan` is one of the overflow gate's own 24 pages
(`overflow-gate.mjs:515`), and **both its panels are empty at all five
widths**: `createFixtures` moves every team-project issue it creates into the
*completed* sprint (`sprint_issues`' single-sprint-per-issue invariant), so
the still-planned sprint has an empty Backlog and empty Sprint Items.
**Five of the 120 cells measure a page with no list in it** — `GATE-001` and
`GATE-002` repeating.

**Add team-project issues so both panels have content**: at least one left
out of the completed sprint (the backlog), and at least one added to the
planned sprint (sprint items). **Through the real routes**, as every other
fixture step is.

**The gate may newly fail, and that is the point.** A surface swept with no
content for this long may be hiding a defect. **If it goes red, escalate
with the measurement rather than adjusting the fixture until it passes.**

**Adversarial and plain, as usual**: the new strings join
`ADVERSARIAL_STRINGS` and `PLAIN_STRINGS` both.

## §6 — then the fourth screenshot

**After §1–§3, not before.** `doc-screenshots.mjs` gains the sprint plan as
a fourth page, captured at 1× like the others.

- **`§4.1`'s per-image privacy confirmation, for all four** — the three
  existing ones are recaptured by the alignment fix, so none of the old
  confirmations carry over.
- **Byte cost reported.**
- **`first-run.md`** gains it where the walkthrough reaches planning.
- **Read all four at full size** and confirm the alignment defect is gone
  from the two that showed it.

## §7 — gates

- **`DEC-007`** — §4's guard if built, and nothing else should move it.
  State the arithmetic.
- **All three browser gates**, and the overflow gate's count said plainly:
  **120 cells**, and whether any newly failed.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken), `mdbook build`.
- **No migration.**

## §8 — escalate rather than deciding

- **If kind 3 is large enough that `grow()` must change** — tell me the
  count before changing 219 sites.
- **If the overflow gate goes red** on the newly populated plan page.
- **If §4's guard needs an exception list longer than its rule.**
- **If the fourth screen still photographs badly** once populated. That was
  `/today`'s answer and it remains available.

## §9 — exit condition

The 219 sites sorted and counted; the alignment defect fixed in whichever
shape the counts justify, with `DEC-049`'s presumption named in the
package; a guard built or declined with its reason; both plan-page panels
populated and the gate's result stated; four screenshots captured after the
fix, confirmed by eye and by the privacy check; `first-run.md` showing the
fourth.
