# I18N-004 — Clamp health summary prose; fix two broken explanation sentences

**Issued by**: Architect
**Date**: 2026-08-03
**Priority**: **P0** — `NFR-LANG-002`, currently violated in shipped code
**Governing RFC**: [006](../../accepted/006-i18n-architecture.md); ruling
[`ISSUE-006-decision.md`](../../../.git-exclude/reviewed/ISSUE-006-decision.md)
**Depends on**: I18N-002 (landed)
**Blocks**: I18N-005a–e
**Ships as**: **0.20.1**, owner-approved

---

## 1. Purpose

Three defects you found while surveying for I18N-002, plus the duplication
I18N-002 introduced. All four are in the same handful of functions, so they are
one unit.

Finding 1 is a **P0 severity-ceiling violation shipping today**. That is why
this comes ahead of the web conversions.

## 2. Finding 1 — clamp the prose, do not reword it

`summarize` renders `"{label} is a concern."` and
`"{first} is a concern; {second} also needs attention."` — naming the unclamped
state in the paragraph directly beneath the health heading, always visible.

**Rewording the two arms is not the fix.** That corrects the symptom and leaves
a sentence generator that can still see `HealthIndicator::Concern`.

**`summarize` takes `DisplayHealthState`, not `HealthIndicator`.** Then the
`Concern` arms **cannot exist** — the variant is not on the parameter type —
and they collapse into their `Watch` equivalents, which already read correctly:

- `"{label} is worth a glance."`
- `"{first} and {second} are worth a glance."`

No new wording. This is DEV-004's fix applied one layer down, and it is the
answer to why DEV-004 missed this: the clamp was attached to *a type used by
badges* rather than to *everything that renders a state*.

If the sort or selection logic in `summarize` needs the internal four-state
ordering to pick which indicators lead, keep that internal — clamp at the point
the state reaches a sentence, not before the ranking.

## 3. Findings 2 and 3 — two new message keys

The percentage template was never right for these values. Typed parameters make
each case structurally distinct.

**BusFactor, `active_assignees <= 1`** — currently *"solo of in-flight work is
concentrated on one person."*

> **"In-flight work is currently carried by one person."**

Plainly factual. `classify_bus_factor`'s own comment notes solo work is
*expected* for solo projects and is deliberately `Watch` rather than `Concern`.
**Do not add a suggestion** — "consider spreading the load" is a `you should` in
disguise, which §1.7 prohibits.

**WipCompliance, `wip_violators > 0`** — currently *"3 over of active assignees
are over their WIP limit."*

> **"{count} active assignees are over their WIP limit."**

The doubling disappears because the count is a typed parameter rather than a
pre-formatted `"N over"` string.

**Reproduce finding 3 live before fixing it.** You flagged it as source-derived
rather than verified, and were right to mark the difference. It needs a
configured WIP limit that is actually exceeded. **If it does not reproduce, say
so and leave it alone** — I would rather learn the case is unreachable than
ship a fix for a defect that cannot occur.

## 4. Absorb `IndicatorKind::label()`

I18N-002 §5.1 flagged that `label()` and `peisear-i18n`'s `IndicatorLabel` hold
the same six strings with nothing keeping them in sync.

That is a fifth prose-producing function — my I18N-002 §4 listed four. Convert
it here, because this handoff already restructures `summarize`'s label handling
and doing them separately means touching the same code twice.

After this, one place holds those six strings.

## 5. The ceiling test must be fixed, and must fail first

`health_presentation_clamps_concern_to_watch_vocabulary` checks
`body.contains("Concern")` — **case-sensitive**. Prose says "concern" lowercase
mid-sentence, so the test passed against a page that violated the requirement.
I approved that test.

Required:

1. **Case-insensitive.** A ceiling check that misses a word because it is not
   capitalised is close to no check at all.
2. **Cover the summary paragraph**, not only badge markup. Finding 1 lived in
   the sentence directly beneath the elements the test did examine.
3. **Fail before the fix.** Paste the run showing it red against current `main`,
   as every correction in this project has required.

Point 3 is the one that matters. A ceiling test that has never been observed
failing is exactly what let this ship.

## 6. Change scope

- `crates/peisear-core/src/lib.rs` — `summarize`'s signature and arms;
  `IndicatorKind::label()`
- `crates/peisear-i18n/src/` — new keys, removed keys, `IndicatorLabel`
- `crates/peisear-web/` — forced call sites only
- `crates/peisear-web/tests/health_explainability.rs` — the ceiling test

## 7. Non-change scope

- **No classifier, threshold or normalisation changes.** `classify_bus_factor`
  returning `Watch` for the solo case is correct and stays.
- No web surface conversion — that is I18N-005.
- Do not touch the guard's term list. These are not vocabulary violations, and
  extending the guard for them was correctly declined in ISSUE-006 §4.
- No other explanation sentence changes wording.

## 8. Required tests

1. The ceiling test, per §5 — case-insensitive, covering the summary, failing
   first.
2. A fixture whose indicators actually reach `Concern`, asserting the rendered
   summary contains no ceiling breach. DEV-004 established that a fresh project
   with one open issue drives `classify_throughput` to `Concern`.
3. The BusFactor solo sentence renders the new text, from the default state — a
   fresh project with one unassigned issue.
4. The WipCompliance sentence, **if §3 reproduces**.
5. `health_explainability`'s existing tests pass. If one needs editing, that is
   a behaviour move — report it.

## 9. Acceptance criteria

1. `summarize` cannot see `HealthIndicator::Concern` — enforced by its
   parameter type, not by convention.
2. No rendered health copy names a state above `Watch`, in any casing.
3. The two sentences render the new text; no other sentence changes.
4. Those six label strings exist in one place.
5. The ceiling test fails against current `main` and passes after.
6. fmt and clippy exit 0; suite counts unchanged apart from added tests.

## 10. Prohibited

- Do not reword the `Concern` arms and leave the type unchanged. The type
  change **is** the fix.
- Do not add advice or suggestion to either new sentence.
- Do not change classification to make a sentence easier to write.
- Do not weaken the ceiling test to accommodate anything.

## 11. Evidence

- Changed-file list.
- **The ceiling test failing against `main`**, then passing.
- Before/after rendered summary for a `Concern`-reaching fixture.
- Before/after for both corrected sentences.
- Whether finding 3 reproduced, and how you determined it.
- fmt, clippy, full suite.

## 12. Review focus to request

1. Whether the clamp is genuinely structural — could a future caller still get
   an unclamped state into a sentence?
2. The two new sentences: factual, no advice, no evaluation.
3. Anything the label consolidation forced that you did not expect.

**Escalate rather than deciding** if clamping `summarize` turns out to need the
internal state for ranking in a way the parameter type cannot express, or if
finding 3 does not reproduce.
