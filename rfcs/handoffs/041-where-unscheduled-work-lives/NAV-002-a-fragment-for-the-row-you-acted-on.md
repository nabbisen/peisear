# NAV-002 — a fragment for the row you acted on

**`POST-001`'s case 1, commissioned.** **Target**: 0.48.0.

`POST-001` measured it: on the sprint plan at a 390 × 844 phone width, a move
discards **1292 px of 3078**, and a second row the reader could see ends up
**1990 px below the fold**. The fix it named is a **fragment anchor** on the
row's existing identifier — cheap, **no script**, and no collision with
`FR-NAV-005`, whose carrier is the query string and never the fragment.

**Read `POST-001`'s report before starting.** Its central finding is not the
fix but the **split**, and this handoff exists only for the half that has
one.

---

## §0 — what this must not claim, stated first because it is the whole risk

> *"A uniform 'add a fragment to every redirect' fix would appear to work on
> the case that is easy to test and silently do nothing on the case that
> motivated this investigation."*

**Case 3 — the calendar Move — is not in scope and is not fixed by this.** The
subject **leaves the landing page**, so a fragment has nothing to target;
`CAL-006`'s flash naming the new day is already that case's remedy.

So: **scope this to redirects that land on a page the acted-on row still
appears on**, and **say in the commit message and the report which redirects
you changed and which you deliberately did not.** A handoff that quietly
fragments all 51 `Redirect::to` sites would be worse than this gap, because
it would read as solved.

## §1 — what to build

1. **An id on the row.** The row already carries `data-plan-issue-id`
   (`components/sprint_plan.rs:395` and `:497` — **both render functions, so
   both columns**). A fragment needs an `id` attribute, which a `data-`
   attribute is not. Add one derived from the same value, in both functions.
   **Prefix it** — a bare issue id as a DOM id is a collision waiting on a
   page that may render other ids from the same source.
2. **The fragment on the redirect.** `plan_add` and `plan_remove` return to
   the plan page; append `#<prefix>-<issue-id>`. **Preserve the existing query
   string** — those handlers already carry the backlog's filter fields, and
   `FR-NAV-005` is why. **The fragment goes after the query, never instead of
   it.**
3. **Nothing else.** Do not touch the other 49 call sites in this handoff.
   **If you find a second site that obviously qualifies** — a redirect back to
   a list the row is still on — **name it in the report and leave it**; a
   second site is a second decision about scope and it is mine.

## §2 — the thing most likely to go wrong

**A fragment that points at nothing fails silently**, which is exactly how
case 3 misleads. So:

- **After a move, the row is in the *other* column** — still on the page, with
  the same id. Confirm that, because the id must be rendered by **both**
  functions for the fragment to resolve after either direction.
- **When the row leaves the page entirely** — a filter that excludes it, a
  removal that drops it from the view — the fragment resolves to nothing and
  the browser lands at the top, as today. **That is acceptable and must be
  stated**, not papered over.

## §3 — the test, and why a Rust test is not enough on its own

**A Rust test can assert the redirect's `Location` carries the fragment and
that the landing page renders that id.** Write that — it is the cheap, exact
assertion.

**It cannot assert the browser scrolled.** That is the `§10.35` trap one
release after closing it: an assertion cited as if it covered the behaviour.
So:

- **The Rust test asserts the two halves that are server-side facts**: the
  `Location` header's fragment, and the id present in the response body.
- **State plainly, in the test's own doc comment, that the scroll itself is
  not asserted** — and whether `POST-001`'s harness could assert it is worth
  one sentence of your judgement. **Do not build a browser gate for it in
  this handoff**; if it is worth one, that is a `GATE-00x` decision and mine.

## §4 — gates

- **`DEC-007`** three consecutive runs; the count rises by whatever you write.
- **The test seen to fail** — remove the fragment from the redirect and watch
  it fail on the `Location` assertion; restore byte-identical.
- **The overflow gate at five widths, 120 cells.** A new `id` attribute does
  not change layout, but the fixture sweeps the sprint plan and `§10.25` is
  about that page.
- **Both other browser scripts**, since a component changes.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.
- **A phone-width confirmation of the actual benefit.** `POST-001`'s harness
  measured the loss; **re-run that measurement with the fix in place** and
  report the new `scrollY`. The number that made this worth doing is the
  number that should show it is done. **Keep the harness in the review
  package**, not `browser-checks/`.

## §5 — escalate rather than deciding

- **If the id has to be derived from something other than the issue id** to
  avoid a collision.
- **If the fragment and the query string interact** in any way `FR-NAV-005`
  would care about.
- **If a second call site obviously qualifies.** Name it, leave it.
- **If the measured `scrollY` after the fix is not the row's position** —
  that means the fragment resolved to the wrong element, which is worse than
  not resolving.

## §6 — exit condition

An `id` on the plan row in both render functions, the fragment on both plan
redirects with the query string preserved, a Rust test asserting the
`Location` fragment and the rendered id **with its own limit stated in its doc
comment**, the pre/post `scrollY` measured at phone width, the overflow gate
at 120, and **a report naming which redirects were changed and which were
deliberately not.**

**`ROADMAP.md`'s Unscheduled work row then loses its case-1 half** and keeps
case 3's — which is mine to record.
