# `LBL-001` round 2 — The spelling the guard can read

**Round 1 is accepted.** All 15 sites changed, the scope held exactly, the
measurements are real, and the round **corrected the handoff on a
load-bearing point**: the `24px → 32px` control-height cost it was told to
measure **cannot happen**, because `grow()` composes `TOUCH_TARGET`
(`min-h-11` = 44 px) and that already dominates both `input-xs`'s and
`input-sm`'s own heights. The box never moves; only the control's
`font-size` does. That was measured, read back from the live element, and
reported plainly — and it makes the cluster raise better supported than the
evidence the handoff offered for it.

**Three items, none of which undoes round 1.** Two are repairs; one is a
decision the round is explicitly *not* asked to take.

---

## 1. Six sites were measured and left where the guard cannot see them

Round 1 measured contrast on all seven modifier-bearing hints and **kept the
spelling**: six remain bare `opacity-70`.

**`0.64.0` already decided this, for the identical mechanism, on measured
evidence.** `§10.60` records that bare `opacity-70` and
`text-base-content/70` composite the same at single nesting depth
(`5.49`/`5.19` against `5.51`/`5.21`) and that **all four** of that round's
sites moved to the guarded spelling — because a site `contrast_scan` can read
stays checked after the next change, and a site measured once does not.

**And the product already spells it that way almost everywhere**: measured in
`components/`, **149** render sites use `text-base-content/70` against **22**
using bare `opacity-70`. Six of those 22 are the hints this round just
raised — and `teams.rs:176`, in the same row family, already carries the
guarded spelling. **This is the same "two spellings for one job" shape the
round was opened to remove**, one level down.

**Convert the six** — `auth.rs:92`, `settings.rs:205`, `sprints.rs:531`,
`sprints.rs:1163`, `teams.rs:169`, `teams.rs:184` — from
`label-text-alt text-sm opacity-70` to
`label-text-alt text-sm text-base-content/70`, **subject to the condition
`§10.60` itself attached**:

- **Verify per site that the span is a single level deep with no faded
  ancestor.** Bare `opacity` fades an element *and its subtree*; the `/70`
  colour fades *text only*. The equivalence holds for a text-only span under
  an unfaded parent and does not hold in general. If a site fails that test,
  **leave it and say so** — a stated exception is the acceptable outcome, an
  unstated one is not.
- **Re-measure contrast after conversion** and report the ratio per site.
  Expect the same figures within pixel noise; a material change means the two
  spellings are not equivalent *here*, which is a finding worth more than the
  conversion.

---

## 2. State why the button stays small — do not raise it

In both dropdowns the card now reads **label 14 px, typed value 14 px,
button label 12 px**: the submit control kept `btn-xs`. Round 1's own
justification for raising the input — *so the value a reader types matches
the label naming it* — reaches the button in the same card, and nothing says
why it stopped there.

**Measured before deciding**: `btn-xs` has **19** render sites across the
product, `select-xs` **3** (`teams.rs:508`,
`notification_preferences.rs:217`), and `input-xs` now **0**. Raising that
tier is a product-wide control-density decision touching screens this series
does not read.

**Architect's decision: do not raise `btn-xs`, in this round or this
series.** What round 2 owes instead:

- **One clause in each of the two cards' own comments** saying the button
  tier was considered and left, and why — so the next reader does not "fix"
  it blindly, and the next size round does not take it as an oversight.
- Nothing else. The tier is recorded as a candidate in `ROADMAP.md` by the
  architect, together with the spelling split above; **a set is decided once,
  after the screens are read**, which is this series' own stopping rule.

---

## 3. One claim wider than its evidence, in a comment that outlives the round

`css_scan/tests.rs`'s new comment says `input-xs` *"was not used anywhere
else in the tree"*. **It is still in the tree** — `touch_target_scan.rs:244`,
in that guard's own `SIZING_CLASSES` list, where it is **correctly kept**:
the list is what stops the class being reintroduced.

The conclusion is right for a reason the sentence does not give: **`css_scan`
reads `components/**` only** (`css_scan.rs:22`). Correct the clause to say
*no longer rendered anywhere under `components/`*, which is the claim the
evidence supports and the one the next person to move that number needs.

**Checked and not a finding**: that guard's own prose is historical
(*"`TT-001`'s survey found"*, *"`TT-002` made every site of compose"*) and
remains true with zero sites. The ban outliving the use is the desirable
direction, not a gap.

---

## 4. Acceptance

1. **Six conversions, or fewer with a stated reason per exception**, each
   with its single-level check and its re-measured ratio over the background
   it sits on.
2. **Two comment clauses** on the button tier, one per card.
3. **One corrected clause** in `css_scan/tests.rs`.
4. **No new sites.** The `text-xs` census stays at **53** (18 `sprints.rs`,
   11 `calendar.rs`, 7 `teams.rs`, 4 `notification_preferences.rs`, 3
   `notifications.rs`, 3 `auth.rs`, 2 `sprint_plan.rs`, 2 `settings.rs`, 2
   `search.rs`, 1 `layout.rs`) — re-derive it with the command in
   `README.md §2` and say what it is.
5. **`css_scan`'s population should not move.** It stands at `(289, 2)`;
   both `opacity-70` (22 sites) and `text-base-content/70` (149) stay in use,
   so a conversion of six changes no token's presence. **If it moves,
   something other than this round moved it** — stop and report that rather
   than updating the number.
6. **Gates**: the `DEC-007` block with real exit codes (count stands at
   **687**), `fmt`, `clippy` both feature variants, `contrast_scan` and
   `touch_target_scan`, and the browser checks if any markup moved. If a
   gate's own source is edited, that gate is run.
7. **Privacy floor per artefact**, per `README.md §8.7`.
8. **Review request**:
   `.git-exclude/review-request/LBL-001-round-2-the-spelling-the-guard-can-read/README.md`.

**If a measurement contradicts anything above, the measurement wins** —
report it. Round 1 already did that once, correctly, and it improved the
decision.
