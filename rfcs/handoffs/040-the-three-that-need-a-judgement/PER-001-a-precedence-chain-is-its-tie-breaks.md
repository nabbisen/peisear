# PER-001 — a precedence chain is its tie-breaks

**Requirement**: `FR-PER-006` (P1). **Register**: `§10.35` — one of the three
coverage gaps held back from 0.46.0 because **each needs a judgement about
what a test should assert**, not just a test written against current
behaviour. **Target**: 0.47.0.

**The gap**: the entry cites one test,
`today_renders_no_callout_for_fresh_user` — **the case where nothing
applies.** The chain itself has never been exercised.

---

## §0 — the code says four callouts, not three tiers

The requirement names *"sustained burnout signal, then WIP over limit, then
long-stale assigned work."* `compute_read_first`
(`components/me.rs:85`–`136`) has **four** early returns, because **burnout is
two conditions**:

| order | condition | threshold |
|---|---|---|
| 1 | `b.overload_streak_days >= OVERLOAD_STREAK_WATCH` | `user_burnout::OVERLOAD_STREAK_WATCH` |
| 2 | `b.stalled_assigned_max_days >= STALLED_WATCH_DAYS` | `user_burnout::STALLED_WATCH_DAYS` |
| 3 | `current_wip > effective_wip_limit` | **strict `>`** |
| 4 | `long_stale_count >= 1` | **deliberately 1** |
| — | else | `None` |

**Read the constants from `peisear_core::user_burnout` rather than hardcoding
numbers.** A test that hardcodes a threshold passes after someone changes the
constant and the behaviour with it.

**Two boundaries the code comments call deliberate, and a chain test that
skips them is not testing the chain:**

- **`>` not `>=` on WIP.** The comment says so: *"being exactly at the limit
  is the limit, not over it."* **`current_wip == effective_wip_limit` must
  produce no WIP callout.** A test using *well over* passes equally against
  `>=`, so the boundary is the only assertion that pins the stated meaning.
- **`long_stale_count >= 1`.** *"even one stale issue is worth surfacing."*
  **0 and 1 both matter.**

## §1 — the judgement that is yours

`compute_read_first` is a **private `fn`** in `components/me.rs`, and `me.rs`
has **no `#[cfg(test)]` module**. So before writing anything, choose:

- **unit-test the pure function** — which needs a `#[cfg(test)]` module in
  `me.rs`, or widening it to `pub(crate)`; or
- **drive `/today` over HTTP** — which needs a fixture per case, and the
  `stalled_assigned_max_days` fixture is awkward (see §3).

**My view, offered and not binding**: a precedence chain is combinatorial, and
the pure function is where combinations are cheap — four conditions, their
pairwise overlaps, two boundaries. **Plus one HTTP test** proving the chosen
callout actually reaches the page, because a pure-function test proves nothing
about rendering. **If you disagree, say why and do it your way** — you will
see the fixture cost and I will not.

**Do not widen the function's visibility if a `#[cfg(test)]` module will
do.** `pub(crate)` for a test's convenience is a change to the code's shape
for the test's benefit, and `TT-007` set the precedent of reusing what exists
rather than exposing more.

## §2 — what must be asserted

1. **Each of the four in isolation** — only that condition true, that callout
   rendered.
2. **The tie-breaks, which are the requirement's actual content.** At minimum:
   **1 over 2** (both burnout conditions true → overload wins), **2 over 3**,
   **3 over 4**. A chain is only a chain where two conditions compete; four
   tests with no overlap test four independent conditions.
3. **At most one** — with several true, exactly one callout in the output.
   Assert the *count*, not the presence of the expected one: *"the right one
   is there"* passes against a page showing all four.
4. **The two boundaries** from §0.
5. **None** — already covered by the cited test; leave it alone.

## §3 — the fixture warning, from `COV-001`'s own work

`stalled_assigned_max_days` falls back to an issue's `updated_at` when there
is no `status_changed` event, and `updated_at` is trigger-maintained. **`COV-001`
already solved this**: `0017`'s trigger fires only
`WHEN OLD.updated_at = NEW.updated_at`, so a test-side `UPDATE` that sets the
column **explicitly** is not overwritten. That is the trigger's own documented
door, read before being relied on. **Reuse that, do not rediscover it**, and
cite `COV-001` where you do.

## §4 — gates

- **`DEC-007`** three consecutive runs; the count rises by whatever you write.
  **Report the composition**, not just the total.
- **Each test seen to fail.** For the tie-breaks this is cheap and
  essential: invert two arms of the chain and confirm the tie-break test
  fails. **A precedence test that passes against a reordered chain is
  asserting nothing**, and that is the specific way this test can be
  worthless.
- **If a new test file lands**, both halves of `DEC-007`.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.
- **Overflow gate only if a component changes.** It should not. **If you edit
  `me.rs` beyond adding a `#[cfg(test)]` module, say so** — that would mean a
  gap is a defect.

## §5 — escalate rather than deciding

- **If a tie-break turns out not to hold** — that is a defect in a P1, not a
  test you wrote wrongly.
- **If the WIP boundary turns out to be `>=` in effect** despite the comment.
- **If `OVERLOAD_STREAK_WATCH` or `STALLED_WATCH_DAYS` cannot be reached from
  a test** without reproducing production logic.
- **If driving `/today` needs a fixture you cannot build honestly.** Say so
  rather than approximating the signal.

## §6 — exit condition

The four conditions, at least three tie-breaks, the at-most-one count, and
both boundaries — each seen to fail; your choice of test level **with its
reasoning**; `DEC-007` three times at the new figure with the composition
named; and no component changed.

**`FR-PER-006`'s acceptance then names artefacts instead of one empty case**,
which is mine to write on your evidence.
