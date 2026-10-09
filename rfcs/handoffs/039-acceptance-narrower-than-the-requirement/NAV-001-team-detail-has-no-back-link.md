# NAV-001 — team detail has no back link

**A defect, not a coverage gap**, and the distinction is the whole reason this
handoff exists separately. `COV-001` found it **while writing the test meant
to prove coverage**, and correctly declined to write either available test:
one would have passed against non-conformant markup, the other would have
failed against a defect.

**Requirement**: `FR-NAV-003` — *"Detail screens MUST provide a breadcrumb
trail **and a back link** to the parent context."* **Register**: `§10.35` —
and this is the **one live compliance defect** in that entry's twelve, now
recorded as such. **Target**: 0.46.0.

---

## §1 — what is wrong

`TeamDetailPage` (`components/teams.rs:205`–`379`) hand-rolls its trail:

```rust
<div class="breadcrumbs text-sm mb-2"><ul>
    <li><a href="/teams" class=grow("")>{t(MessageKey::NavLinkTeams)}</a></li>
    <li>{team_name.clone()}</li>
</ul></div>                                          // :306
```

Across that whole function: **no `render_breadcrumb`, no `render_back_link`,
no `aria-current`, no `/today` root, no back affordance of any kind.** I
verified each independently rather than from the report.

`FR-NAV-003`'s sentence has **two conjuncts**. That screen satisfies the
first and not the second. **The missing back link is the defect**; the trail's
non-conformance — no root, no `aria-current` — rides with it.

The other three detail screens route through `breadcrumb.rs`'s helpers, which
prepend `/today` (`:94`) and tag the current node `aria-current="page"`
(`:115`).

## §2 — what to do

**Route the page through the two shared helpers**, the way project, issue and
sprint detail already do (`sprints.rs:684`, `:693` is the closest model).
Delete the hand-rolled markup rather than adding a back link beside it.

**Trail shape**: Today → Teams → *this team*, with the team as the current
node. Confirm against what the other three produce rather than against this
sentence — **they are the precedent, and if they disagree with me, they win
and you report it.**

**Do not generalise.** Four detail screens, three already correct, one being
brought into line. **If you find a fifth screen that should be a detail screen
and is not in the set, report it** — do not fold it in. `FR-NAV-003`'s own
population is *"detail screens"*, and `§10.35` is the entry about requirements
whose population and mechanism disagree; adding a screen to the set is a
population question and it is mine.

## §3 — the test, which is why this was deferred and not rushed

**One test, in `breadcrumb` beside the three that exist.** `COV-001` left it
unwritten deliberately, so write it now against **what the requirement needs**
rather than what the page emits:

- the trail's full chain, including the `/today` root,
- `aria-current="page"` on the team,
- **and the back link's presence and target** — the conjunct that was missing,
  which is the one assertion that would have failed before this change.

**Seen to fail**: this one needs no planting. **Write the test first, watch it
fail against the current page, then fix the component.** Quote the failure.
That is the cleanest possible demonstration and this is the rare case where
the defect supplies it for free.

## §4 — gates

- **The overflow gate, at five widths.** A component changes, so it runs —
  **120 cells**. A breadcrumb trail gaining a `/today` root makes the row
  **longer**, and a team name is user text; `§10.25` and `LAYOUT-006` are
  about exactly that on a 320 px phone. **This is the most likely defect in
  this handoff.**
- **`NFR-A11Y-007`** on the back link and the new trail links: 44 × 44, no
  overlap. The hand-rolled markup used `grow("")`, so the existing links
  already declare it — confirm the helpers' output does too.
- **Both browser scripts**, since a component changes.
- `DEC-007` three consecutive runs. **Expect 392**, from 391.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.

## §5 — escalate rather than deciding

- **If the three correct screens disagree with each other** about the trail's
  shape. Then there is no single precedent and the shape is a decision.
- **If the overflow gate moves off 120.**
- **If routing through the helpers changes anything else on the page** — a
  heading level, a focus order, a landmark. It should not; if it does, that is
  a second finding.
- **If a fifth screen belongs in the population.**

## §6 — exit condition

Team detail routed through `render_breadcrumb` and `render_back_link`, the
hand-rolled markup gone, one test asserting the chain, `aria-current` **and
the back link** — **written before the fix and seen to fail against the
current page** — the overflow gate still 120, and `DEC-007` three times at
392.

**`FR-NAV-003` reaches Met on this**, which is mine to record. **`§10.35`
closes when this lands and 0.47.0's three are handed off.**
