# I18N-007 — The completion sweep, and the test that replaces it

**Issued by**: Architect
**Date**: 2026-08-11
**Priority**: P1 — **0.21.0 is not complete without this**
**Governing RFC**: [006](../../done/006-i18n-architecture.md)
**Depends on**: I18N-006 (landed)

Pattern rules: RFC 006 §D6.

---

## 1. Why this exists

Four handoffs have each declared their surface complete. Each was followed by
another find. I18N-006 made the release-completion claim; a three-command sweep
of every string literal in `peisear-web/src` found five categories still
outside the table.

That is not a discipline problem. It is a **method** problem, and it is mine: I
asked for a global claim and supplied a local measurement. A survey scoped to a
handoff's categories confirms those categories and is silent about everything
else.

So this handoff has two halves. §2 converts what is left. §3 replaces the survey
with a test, so the claim stops depending on anyone's thoroughness — including
mine.

Do §3 **first**. Written first, it fails and names its own work list; you then
convert until it passes. Written last, it only confirms what you already did.

## 2. The five categories

All are pre-existing misses from I18N-005b/c/d, not I18N-006 defects. Byte-exact
conversion, **no rewording** — every wording concern here is already in the
copy-pass queue.

| # | Where | What |
|---|---|---|
| 1 | `components/issues.rs:255–267` | `render_trend_chip`'s match arms: `"flat"`, `"trend: roughly flat"`, `"trend: up by {delta} points"`, `"trend: down by {delta} points"` |
| 2 | `components/issues.rs:299` | `"Composite"` — visible text node in `composite_row` |
| 3 | `components/sprints.rs:752, 757` | `"Committed"` / `"Completed"` — burndown legend |
| 4 | `components/me.rs:271` | `aria-label="Current load"` |
| 5 | `components/me.rs:180, 181, 190` | `"{} / {} pt"`, `"{} pt · no limit"`, `"{} done in last {N}d"` |

Notes on three of them:

**#1** is prose in both `aria-label` and `title` — the `title` is a visible
tooltip, so this is not an accessibility-only surface. `Trend` is a closed
four-variant set with one arm returning early, and the `+{delta}` / `-{delta}`
labels are data, not copy. Follow the `IndicatorAriaLabel` shape you established
in I18N-006: compose the sentence in one `en.rs` arm from typed data. The glyphs
(`→` `↑` `↓`) stay put, as `glyph()`'s symbol did.

**#3** almost certainly needs no new keys. `CaptionWordCommitted` and
`CaptionWordCompleted` already exist from I18N-005c, for the caption directly
below this legend. If they render the right words, reuse them; that is D6 rule 3
working as intended. If reuse would force a casing or inflection compromise, say
so rather than bending the words — that is the fifth instance of the inflection
question and it should be recorded, not absorbed.

**#5** are chip values whose surrounding labels and tooltips are already
converted. `"pt"`, `"no limit"` and `"done in last …d"` are copy; the numbers are
data. Same shape as `SwitchingMedianValue` and `DriftValueLine`, which
I18N-005d already converted this way.

Then re-check `me.rs:172` (`"{} / {}"`) and `sprints.rs:155, 461` (`"{} → {}"`):
I read those as pure data with no copy word, so they stay. Say so if you agree,
and say so louder if you do not.

## 3. The scan test — the real deliverable

Add a test to `peisear-web` that reads its own sources and fails on user-visible
literals outside the table. Runtime `std::fs` walk rooted at
`env!("CARGO_MANIFEST_DIR")` — `include_str!` cannot glob.

**Scope**: `src/components/**.rs` and `src/handlers/**.rs`.

**Flag**:

- `aria-label="…"`, `title="…"`, `placeholder="…"` where the value is a string
  literal rather than `t(…)` or a binding.
- Quoted text nodes inside `view!` — a bare `"…"` on its own line, or `>"…"<`.

**Do not flag** class-attribute values, format strings with no alphabetic word
outside `{}`, `%`-style date patterns, SVG path data, or `#[cfg(test)]` code.

**Allowlist**: an explicit `const` of `(file, literal, reason)` triples. Every
entry needs a reason, and the reason must be a decision, not "not converted
yet". Expected at hand-off time:

- the 9 `onsubmit="return confirm('…')"` strings — awaiting the owner's decision
  on the confirmation pattern (I18N-005b review §2), a real defect but an
  architectural one, not an i18n gap;
- anything the seven `validator` literals surface, already covered by
  I18N-006 §6's scan test.

`static/search.js` is outside this test's scope entirely — it is not Rust. It
stays named in the queue README as the one permanent exclusion.

**Calibrate honestly.** If the test needs more than a handful of allowlist
entries beyond those two groups, the heuristic is wrong, not the codebase —
report that instead of growing the list. An allowlist that absorbs findings is
worse than no test, because it looks like coverage.

**Prove it works**: revert one §2 conversion, show the test fails naming that
literal, restore it. A guard nobody has seen fail is a guard nobody has tested.

## 4. The drift assertion for I18N-006's scan test

`VALIDATOR_DERIVE_MESSAGES` in `handlers.rs` duplicates seven literals that
cannot be referenced from the `#[validate]` attributes — correctly disclosed as
a drift risk. Close it: each entry already names its source file, so
`include_str!` that file and assert the literal still appears in it. Rewording
one side then fails the test instead of silently diverging.

## 5. Tests

1. The §3 scan test, demonstrated failing and passing.
2. The §4 drift assertion.
3. Guard covers every new key and passes; exhaustiveness holds in `en.rs` and
   the fixture locale.
4. Rendered output semantically identical per §D6 rule 5.
5. Existing suites unchanged apart from the two new tests.

## 6. Acceptance

1. All five §2 categories converted or explicitly dispositioned.
2. The scan test exists, passes, and has been shown to fail on a planted
   literal.
3. The allowlist has a reason per entry and no "not yet" entries.
4. The §4 drift assertion in place.
5. fmt and clippy exit 0; suite counts move only by the new tests.

## 7. Prohibited

No rewording — the five queued copy findings stay queued. Do not convert
`search.js`. Do not weaken the guard or widen the allowlist to make the test
pass. Do not change the `code`/`error`/`entity_type` JSON contract fields. Do
not touch `translate_trigger_error`'s needles.

## 8. On completing 0.21.0

When this lands, the completion claim stops being a claim. **Do not restate it
in prose.** Point at the passing scan test and its allowlist, and name what the
test cannot see: `static/search.js`, and anything the heuristic in §3 misses by
construction — which you are better placed than I am to describe, having just
written it.

If the sweep finds a sixth category I did not list, that is the most valuable
thing in this handoff. Report it before converting it.
