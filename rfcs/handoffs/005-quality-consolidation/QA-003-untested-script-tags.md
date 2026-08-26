# QA-003 — The script tags nothing asserts

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P1 — 0.27.0
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §12
**Depends on**: nothing. `BOARD-001` and `STATUS-002` are both shipped in
0.26.0; this corrects a gap that predates them.

---

## 1. The defect, verified by planting

Delete line 142 of `crates/peisear-web/src/components/issues.rs` —

```rust
<script src="/static/board.js" defer=true></script>
```

— change nothing else, and the suite is still green:

```
status_control: 11 passed    board_keyboard: 6 passed
smoke:          11 passed    view_state:     5 passed
cargo test --workspace: 178 passed, 0 failed
```

The board would ship with no drag-and-drop and no undo. Every gate would
pass. **Reproduce this before you fix it** — the transcript is the
evidence that the tests you add are the tests that were missing, and it
costs one build.

`search.js` has the same gap and a wider blast radius: its tag is in the
app shell (`components/layout.rs:72`), so it is on every page.

| File | Tag emitted at | Asserted by |
|---|---|---|
| `dm.js` | `components/issues.rs:564` | `status_control::dm_js_is_served_with_defer_on_both_surfaces` |
| `board.js` | `components/issues.rs:142` | **nothing** |
| `search.js` | `components/layout.rs:72` | **nothing** |

## 2. The comment that says otherwise

`crates/peisear-web/tests/status_control.rs:485`:

> ...not the board, which loads `board.js` instead —
> `boards_per_card_control_renders_unchanged` above already pins that.

It does not. That test asserts the board posts to `/status/board` and does
not pick up `/status/detail` or `/status/list`. It never looks for
`board.js`.

**Correct the comment as part of this handoff.** Say what that test
actually pins — the board's route, unchanged — and point at the new test
for the script tag. This is the second time in this project a comment has
described a neighbouring test's coverage and been wrong about it; the
first cost `RFC 003` a rewrite. A comment is the one artefact here with no
guard, so the only defence is not writing claims into it that a reader
cannot check in the same screen.

## 3. What to add

Two tests, both HTTP-level, both mirroring the shape of
`dm_js_is_served_with_defer_on_both_surfaces` — which is the one that got
this right and is the model to copy, including its doc comment explaining
why it does not `GET /static/dm.js` itself.

**`status_control::board_js_is_referenced_on_the_board_view`** — `GET
/projects/{id}?view=board` contains `<script src="/static/board.js" defer`.
Next to the `dm.js` test, because the pair reads as one fact about the
three surfaces.

**`smoke::search_js_is_referenced_in_the_app_shell`** — any authenticated
page contains `<script src="/static/search.js" defer`. `smoke` is the right
home: the claim is about the shell, not about search.

Assert the tag **with `defer`**, as the `dm.js` test does. A tag that loses
`defer` runs before the DOM it reaches for exists, which is a real
regression and free to catch here.

## 4. What not to build

**No scan over `static/`.** A test walking the directory and asserting each
filename appears somewhere under `crates/peisear-web/src/` would extend
itself to a fourth file for free — and would pass on a tag emitted inside a
branch that never renders, which is most of what can actually go wrong.
Three assertions for three files is complete today.

The residual gap is real and is not being built for: **a fourth JavaScript
file added later gets no reference test automatically.** Note it in the new
`board.js` test's doc comment so the next person adding one sees it. Do not
add machinery.

**Do not touch `static/*.js` itself.** No file in `static/` changes in this
handoff.

**Do not fix `ServeDir`'s path resolution.** The `dm.js` test's comment
explains that `ServeDir::new("static")` resolves against the crate root
under `cargo test`, not the workspace root, so no test can `GET` these
files. That is a separate concern and it is not this one.

## 5. Tests

| # | Check |
|---|---|
| 1 | Board view references `board.js` with `defer` |
| 2 | Any authenticated page references `search.js` with `defer` |
| 3 | `boards_per_card_control_renders_unchanged` still passes, unmodified |

Expected counts after this handoff:

| Target | Before | After |
|---|---|---|
| `status_control` | 11 | **12** |
| `smoke` | 11 | **12** |
| integration total | 147 | **149** |
| workspace total | 178 | **180** |

## 6. Escalate rather than deciding

- If either new test passes with its script tag deleted, stop and report.
  That would mean the assertion is matching something other than the tag.
- If `search.js`'s tag turns out to be conditional — rendered on some
  authenticated pages and not others — stop and report which. §12's table
  assumes it is unconditional in the shell, and if that is wrong the
  requirement changes shape.
- If correcting §2's comment reveals other comments in the same file making
  claims about neighbouring tests, list them. Do not fix them in this
  handoff.

## 7. Acceptance

1. The §1 plant reproduced and its transcript included, then reverted.
2. Both new tests present and demonstrated failing with their tag removed.
3. `status_control.rs:485`'s comment corrected.
4. Counts match §5 exactly.
5. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 8. Required review-request format

Workflow §9.2. Include the §1 plant transcript and each new test's failing
transcript as first-class evidence — not a summary of them.
