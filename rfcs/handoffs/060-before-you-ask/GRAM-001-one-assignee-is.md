# GRAM-001 — one assignee *is*

**Series**: `060`, inside `0.62.0`'s authorized scope as finishing this
release's own work. **Blocking**: the `0.62.0` cut. **Register**:
`§10.59`. **Small.**

## 1. What is wrong

`MessageKey::IndicatorExplanationWipCompliance { count }` renders

```
{count} active assignees are over their WIP limit.
```

so at `count == 1` a reader meets **"1 active assignees are over their
WIP limit."**

**This release is why it matters.** `VIEW-001` made the highest-precedence
non-Good indicator's own sentence the **headline** of a project board and
list — the one line above the work. When `WipCompliance` leads, that
sentence is the first thing on the screen. The string itself is older
than this release; its prominence is not.

## 2. What to build

**The singular reads `1 active assignee is over their WIP limit.`** —
numeral kept, noun and verb agreeing. The plural is unchanged.

- Keep the numeral (`{count}`), not a spelled-out *one*: every sibling
  indicator explanation uses numerals, and this round is not a style
  change.
- `their` stays in both forms. It is already correct for a singular
  person of unknown gender and it is what the plural uses.
- **Only this one string.** The other five in `§10.59` are a separate
  round; do not touch them here, and do not build the general guard
  here either.

## 3. What to measure

Render the key at **0, 1, 2** and read all three. The count can be zero
on a Good indicator, and a zero form that reads *"0 active assignees
are"* is correct English — check it rather than assuming.

## 4. Acceptance

- A unit test in `peisear-i18n` that renders this key at `count == 1`
  and asserts the exact singular sentence, **and** at `count == 2`
  asserting the plural. Not a substring match on the shared tail: the
  defect lives in the words a substring match skips, which is exactly
  why three existing tests assert this sentence and none of them saw it.
- The three existing assertions that quote the plural tail
  (`basis_route`, `health_explainability` ×2) still pass, or are
  updated with a stated reason if their fixture drives `count == 1`.
  **Check which they drive before editing either side.**
- `peisear-i18n`'s own suite including the fixture locale; `DEC-007`
  from → to, re-derived with the runner.
- **Plant**: put the plural back for `count == 1` and show the new test
  failing by name.

## 5. What this round must not do

- Do not change any other string, including the five siblings in
  `§10.59`.
- Do not change the precedence chain, what is collapsed, or any
  indicator's computation or basis.
- Do not amend `requirements.md` — `§10.59` already records this.
