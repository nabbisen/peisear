# JS-002 — Pin the shape the fallback boundary is made of

**Issued by**: Architect
**Date**: 2026-08-27
**Priority**: P1 — it protects the one thing `JS-001` proved the rest of RFC
011 does not
**Governing RFC**: [011](../../accepted/011-browser-verification.md), step 1b
**Depends on**: `JS-001`, closed. Its trace is the input; do not re-derive it.

---

## 1. Why this exists, and why it comes before the policy moves

RFC 011 planned to shrink the untestable surface by moving policy into Rust.
**`JS-001` established that the single most important rule cannot move.**

`dm.js`'s fallback boundary — *"falling back to a native form submit is correct
before the server has applied the change and wrong after"* — is held by
`applyChange` carrying its own `try { … } catch { announce; reload }`. An
exception raised inside it unwinds into **that** handler and therefore never
reaches the outer `.catch()` that calls `fallback()`.

**There is no boolean.** The property is a fact about which handler is nested
where, so no amount of Rust-side policy testing touches it.

**But the regression is a shape**, and this project has seven guards that
assert shapes in source text. `static_js_scan` already reads this file.

## 2. What to assert

Two things about `static/dm.js`:

1. **`applyChange` contains its own `try` and `catch`.**
2. **`fallback(` is not called inside `applyChange`'s body.**

Together those are the structure. Flattening the two handlers into one, or
deleting the inner `try`, breaks at least one.

**Where it lives**: `static_js_scan`'s family. A sibling module or an addition
to it — say which and why, as `dec_007_ci_scan` did when it split.

## 3. What this guard does **not** claim, and the doc comment must say so

**It does not test that the boundary works.** It pins the structure the
behaviour is made of. `test_harness_scan` makes exactly this kind of claim —
not that the harness is correct, but that the shape which made it incorrect
cannot return.

**A reader must not come away thinking the fallback path is tested.** `§10.15`
stays open; this narrows what is unguarded within it. Say that in the module
doc, in those terms.

## 4. Where this will be fragile, and I would rather hear it than have it worked around

The guard reads JavaScript as text. It has to find one function's body and ask
what is inside it — which is more structure than any existing scan in this
project attempts.

- **If locating `applyChange`'s body needs anything resembling a parser**, stop
  and report. `QA-008` and `QA-012` both drew that line, and a JS parser for
  one assertion is over the line.
- **A brace-counting scan is acceptable** if it is honest about what defeats it
  — a brace in a string or a comment. `dec_007_scan`'s history is the
  precedent for naming a limit rather than pretending to have none.
- **If the function is renamed**, the guard should fail loudly, not silently
  pass. `QA-021` shipped a guard that watched a name and was renamed around;
  the fix was to pin the name too. **Do the same here from the start**: assert
  a function named `applyChange` exists, before asserting anything about it.

## 5. Plant it three ways, separately

| # | Plant | Must fail |
|---|---|---|
| 1 | Remove `applyChange`'s inner `try`/`catch`, leaving the body | yes |
| 2 | Move a `fallback(` call inside `applyChange`'s body | yes |
| 3 | Rename `applyChange` to something else | yes — §4's last point |

One at a time, each reverted, `git diff` clean between — this project's
standing method, and `STATUS-001`'s test 6 is why.

## 6. Not in scope

- **No change to `dm.js`.** Not one line. The shape being pinned is the shape
  that is there.
- **No policy moved.** That is step 2, at 0.30.0.
- **No browser, no dependency.**
- **No guard on `board.js` or `search.js`.** Neither has this structure;
  `board.js`'s equivalent decisions were classified as movable and go to step 2.

## 7. Escalate rather than deciding

- **If a parser is needed** (§4), stop.
- If `applyChange`'s structure turns out to differ from `JS-001`'s trace — a
  third handler, an early return I did not see — stop and report. The trace is
  the premise.
- If pinning the name conflicts with the body assertion in some way I have not
  anticipated, report the shape rather than dropping one of them.

## 8. Acceptance

1. Both §2 assertions, plus the name pin from §4.
2. Three plants, separately, each reverted.
3. A module doc that says plainly what the guard does **not** claim (§3).
4. Its textual limits named, in the guard rather than only in the package.
5. `dm.js` unchanged.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 9. Required review-request format

Workflow §9.2. §4's fragility answer as prose — including the case that defeats
the scan. Each plant transcript separately.
