# COPY-001 — The six deferred wording items, ruled

**Issued by**: Architect
**Date**: 2026-08-11
**Priority**: P2 — 0.22.0, not release-blocking
**Governing RFC**: [006](../../done/006-i18n-architecture.md) (implemented)
**Depends on**: nothing

---

## 1. What this is

Across I18N-005b through I18N-007, six wording items were found and correctly
**not fixed**, because every conversion handoff prohibited rewording. Deferring
them was right: a conversion that also rewords cannot prove it changed nothing.

They are ruled here. **Three are defects and three are not**, which is itself
the finding — see §8.

Each ruling is mine as design authority, within `§1.7` and external design
§10.4. Where I have specified replacement text, it is normative: convert it
byte-exact and report if it does not fit, rather than adjusting it.

## 2. Defect — `SwitchingAriaLabel` says "per active day" twice

`en.rs:697`. The sentence embeds `switching_median_text`, which already renders
`"2 / active day"`, into a template that appends `"pickups per active day"`.
A screen reader hears:

> Switching pattern: median 2 / active day pickups per active day (14 total
> events over 30 days). …

The chip value and the sentence are different registers — a compact value
versus prose — and the sentence should not reuse the value's rendering.

**Fix**: `SwitchingAriaLabel` formats the median itself, with the same
one-decimal rule `switching_median_text` uses, and without the `/ active day`
suffix. `SwitchingMedianValue` is unchanged; the chip still reads
`"2 / active day"`.

Resulting sentence, normative:

> Switching pattern: median 2 pickups per active day (14 total events over 30
> days). For context only — high or low here is not a quality judgement.

Keep the shared helper for the *number* formatting so the chip and the
sentence cannot disagree about decimals. That was the right instinct; only the
suffix travelled where it should not have.

## 3. Defect — the two sub-issue nesting messages

`en.rs:923` and `:928`. The same rejection, two wordings:

- `SubIssueCannotNestLongMessage` — "Sub-issues cannot have their own
  sub-issues. Promote the parent to a top-level issue first, or add this work
  as a sibling sub-issue under the same parent."
- `SubIssueCannotNestShortMessage` — "Sub-issues cannot have their own
  sub-issues."

**Unify on the long form. Delete the short key.**

External design §10.4 rule 4: *errors describe what happened and what would
resolve it.* The long form does both. The short form is the same error with
the resolution removed, and there is no surface here that cannot hold a second
sentence — both are validation rejections on a full-width error surface.

This is not a preference for longer copy. A user who has just been told they
cannot do something needs to know what they can do instead, and the project
already wrote that sentence.

## 4. Defect — `PeriodStartMustPrecedeEndMessage` names Rust identifiers

`en.rs:1001` renders `"period_start must be on or before period_end"`. The
words `period_start` and `period_end` appear in no sentence a user reads; they
are struct fields.

**Replacement, normative**:

> Start date must be on or before the end date.

**Check the form's own labels before converting.** If the capacity-row inputs
are labelled something other than "Start date" and "End date", match them and
tell me — a message naming a field differently from the field's label is the
same defect one layer up.

## 5. Not a defect — the two burnout / `me.rs` "overlaps"

I18N-006 reported two wording pairs as possible inconsistencies. **Both are
correct as they stand. No change.**

- `EstimationDrift{Up,Down}SignalMessage` — "Recent issues are taking longer
  per point than older ones." — against `DriftValueLine`'s `"recent 2.10 vs.
  1.40 d / pt"`.
- `CognitiveSwitchingSignalMessage` — "Switching between 2.4 issues per active
  day on average." — against `SwitchingMedianValue`'s `"2.4 / active day"`.

In each pair one is a **sentence** in a JSON response body and the other is a
**compact value** in a chip. `"d / pt"` in a sentence would be jargon;
`"days per point"` in a chip would not fit. Different registers for different
surfaces is what good copy looks like.

Recorded as ruled so nobody re-opens it. Note that §2 *is* a defect of an
adjacent shape — there, a compact value was embedded **inside** a sentence.
That is the line: registers may differ across surfaces, and must not mix
within one string.

## 6. Not a defect — the burndown legend's casing

`BurndownLegendCommitted` renders `"Committed"`; `CaptionWordCommitted`
renders `"committed"` for the caption below it. I18N-007 kept them as separate
keys rather than forcing a casing compromise, and filed it as an open
question.

**Correct as it stands. No change, and the question is closed.** A legend
label is a noun phrase standing alone; the caption embeds the word
mid-sentence. English requires the difference, and a shared key would have to
render one of the two positions wrongly.

This has been called the fifth instance of "the inflection question". It is
not. `requirements.md` §1.7.1 concerns whether a *prohibited* term's
inflections are also prohibited — a rule about the vocabulary list. This is
ordinary sentence casing. **Two different questions were being counted
together, and I repeated the miscount in the I18N-007 review.**

## 7. Tests

1. Guard passes; exhaustiveness holds in `en.rs` and the fixture locale.
2. `SubIssueCannotNestShortMessage` is gone — the compiler proves no caller
   remains.
3. `prose_scan` and the validator scan still pass.
4. Rendered output changes **only** at the three §2–§4 sites. Capture
   before/after for the `/me` switching chip's aria/title, the sub-issue
   rejection page, and the capacity-row validation error.

This handoff **does** change rendered output — the first in the series that
does. Say so plainly in the review request rather than reusing the
"semantically identical" language from the conversion handoffs, where it would
be false.

## 8. What to say in the review request

Three of six were defects. **Report that ratio**, because it is the honest
measure of what collecting copy in one place bought: it made six differences
visible, and half of them turned out to be correct.

A queue of "found, not fixed" items is not a defect list. Treating it as one
would have produced three unnecessary changes to shipped copy — including
flattening two register distinctions that are doing real work.

## 9. Prohibited

No wording changes beyond §2–§4. Do not touch the nine `onsubmit` dialogs
(external design §17.4, owner decision pending). Do not adjust my replacement
text to fit a layout — report instead. Do not weaken any guard.

## 10. Required review-request format

Workflow §9.2, with the before/after captures from §7 item 4.
