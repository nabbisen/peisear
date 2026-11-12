# GRAM-002 — every counted sentence, not the five

**Series**: `061`, `0.63.0`'s authorized scope. **Register**: `§10.59`.
**Blocked on**: nothing.

## 1. What is wrong

Five English strings still disagree with themselves at one:

| key | at `count == 1` |
|---|---|
| `IssuesCountText` | *"1 issues"* |
| the sprint committed summary | *"… committed across 1 issues"* |
| the velocity chart's summary | *"Completed points across the last 1 sprints"* |
| `TrendDirectionLabel`'s announced form | *"trend: up by 1 points"* |
| the capacity announcement | *"Capacity 1 points, period …"* |

`GRAM-001` fixed the sixth — the one `VIEW-001` had promoted to a
project screen's headline — and left these by instruction.

**Fixing five strings is the smaller half of this round.**

## 2. The decision, which is the architect's and is made here

**Re-scoped 2026-11-12, with `MARK-001`'s lesson applied to this round
before it starts.** The first version of this handoff asked for a guard
that renders every message and checks the result, with an allowlist for
legitimate cases like *"1 pt"*. That is the same shape that just failed:
**fix the instances, then check for recurrence** — and a check with an
allowlist grows an exception list for as long as the project lives.

**Agreement becomes structural instead.** A small private helper in
`en.rs` — not a public API, not a plural-rules mechanism, not an i18n
feature — takes the count and both forms and returns the right one:

```rust
fn counted(n: i64, singular: &str, plural: &str) -> String
```

**Every counted sentence in this locale goes through it.** Then a
sentence cannot disagree with its own number, because the number and the
two forms are chosen in the same place. `"1 pt"` needs no allowlist
entry: it never asks the helper, because a unit abbreviation has no
plural form to get wrong.

**`MessageKey::all()` still gains a singular sample per count-bearing
variant**, and `page_title_scan`'s population pin still moves — that
part stands, because the pin is what makes the *next* counted variant
announce itself. Re-measure the pin with the runner, in the same commit.

## 4. What to measure, and report as a table

The **real population of count-bearing variants**, read from the enum
definition rather than from this handoff's list of five. If it is larger
than six, say so and fix what you find — this handoff's table is what
the architect measured on 2026-11-12, not a guarantee.

Report: variants carrying a count, how many `all()` now samples at 1,
allowlist size, and `DEC-007` from → to.

## 5. Acceptance

- Each of the five asserted **byte-exact at 1 and at 2**, not by a
  substring on the shared tail — that is why three existing tests could
  quote one of these sentences for releases without seeing it.
- `count == 0` checked and reasoned about per string, not assumed:
  *"0 issues"* is correct English, *"0 issue"* is not.
- **Planted**: make one call site pass the plural where the singular
  belongs, and show the test failing by name. The helper makes the
  defect hard to write; the test is what proves it is still caught.
- `peisear-i18n`'s own suite including the fixture locale; any existing
  test that quotes one of these five updated with a stated reason after
  checking which count its fixture drives.
- `DEC-007` from → to, and `MessageKey::all()`'s pin re-measured.

## 6. What this round must not do

- Do not restructure the message table, add a pluralisation library, or
  introduce a general plural-rules mechanism. **English agreement in
  this locale's own text is the whole scope**; a locale that needs more
  than two forms is a different decision, and `FR-I18N` is not amended
  here. The helper is private to `en.rs` for exactly that reason.
- Do not route uncounted strings through the helper to be tidy. It
  exists where a number and a noun meet, nowhere else.
- Do not amend `requirements.md`; `§10.59` records this already.
