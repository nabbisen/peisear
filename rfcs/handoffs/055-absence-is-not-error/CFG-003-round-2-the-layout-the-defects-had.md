# CFG-003 round 2 — the layout the defects actually had

**Round 1's rule is kept, not rebuilt.** The census half, the
`RUST_LOG` bound, the doc comment's honesty and the carried `§4a` test
all stand. **One thing is wrong and it is the central one**: the
predicate is blind to the line layout every real defect had, so rule 8
cannot fail on a reintroduction of `§10.51`'s own subject.

**`§10.51` stays open until this round lands.**

## §1 — the measurement, which you should reproduce before fixing

The real `SMTP_PORT` defect at `c53e7bc` was **four lines**
(`git show c53e7bc:crates/peisear-notify/src/config.rs`):

```rust
let port = env::var("SMTP_PORT")
    .ok()
    .and_then(|s| s.parse::<u16>().ok())
    .unwrap_or(465);
```

`COOKIE_SECURE`'s and `SMTP_TLS_MODE`'s pre-fix shapes were three
lines each, the same way. **Round 1's coverage table reports all three
as caught; it tested one-line reconstructions.** As those shapes
existed in this repository, the predicate catches none of them.

**Two plants, both run by the architect at `8b96a32`, both reverted:**

1. **The four-line shape added as a new site.** The predicate said
   nothing; the **census** fired — *"examined 17 … expected 16 … update
   the census after checking what changed against `RFC 022` §3 by
   hand"*. Correct, but it is the census doing the work.
2. **The same shape in place, keeping the site count at 16.**
   `214 passed; 0 failed`. **Nothing fired at all.**

Plant 2 is the one that matters: an in-place regression is what a
reintroduction actually looks like, and it is invisible to both halves
today.

## §2 — the change

**Widen the discard-then-fallback window: bound it by the next `;`
*or* the next `,` at the same parenthesis depth, whichever comes
first.**

Why this is the right bound, and why it does not bring back round 1's
false positive:

- A method chain spanning lines contains **no same-depth comma**
  before its fallback, so the window reaches `.unwrap_or(` — the
  four-line defect is caught.
- `from_values(a, b, c)`'s arguments **are** separated by same-depth
  commas, so each argument's window closes before the next
  argument's fallback — the five innocent flags round 1 found cannot
  return.

**Verify that reasoning rather than taking it from this handoff.** If
depth tracking over `(`/`)` turns out to need care this handoff has
not seen — string literals containing brackets, turbofish, closures
with commas inside nested parens — **say what you found and what you
did about it.** A comma inside `|s| s.parse::<u16>().ok()` is at a
deeper depth than the chain's own, which is the case that makes depth
tracking necessary rather than a plain `find(',')`.

**Keep everything else.** The census, both asserted counts, the
`RUST_LOG` bound, the message wording, `KNOWN_ENV_VAR_NAMES` — all
untouched unless the window change forces it, in which case say so.

## §3 — seen red on the real layout, not a reconstruction

**Round 1's plant passed because it used a form the defects never
took. Do not repeat that.**

1. **Plant the four-line shape as a new site** and confirm the
   *predicate* now names the variable — not only the census. Capture
   both messages.
2. **Plant it in place, keeping the site count at 16**, and confirm
   the predicate names it. **This is the acceptance test for this
   round**; it is the plant that passed silently at `8b96a32`.
3. **Re-run round 1's own false-positive plant** — one fallback
   planted on a `from_values(...)` argument line — and confirm
   **exactly one** violation, not six. Round 1's exact-match assertion
   already pins this; keep it.
4. **Plant the three-line `COOKIE_SECURE` shape**
   (`git show 564bd6f~1:crates/peisear-web/src/config.rs`) in
   `config.rs` and confirm it is caught. Three of the four historical
   shapes should be reachable after this change; say which are and
   which are not.

**Each plant: captured output, reverted, green after.** Revert by
file copy, never by `git checkout` of the tree.

## §4 — the doc comment, reversed

Today's comment presents the predicate as the protection and the
census as a backstop. **It is the other way round in the one case that
matters**: an in-place edit keeping the site count is seen *only* by
the predicate, so the predicate's reach is the whole of the protection
exactly where a regression would occur.

**State that.** A future reader deciding whether to trust rule 8 needs
it, and after `§2` it is a statement about a predicate that can
actually see the shape.

## §5 — gates

- `cargo test -p peisear-web --lib` — the gate this round's source
  *is*. `DEC-007` was **625** at `8b96a32`, `peisear-web --lib`
  **214**, both measured by the architect; re-measure rather than
  trusting them.
- `fmt`; `clippy --workspace --all-targets -D warnings`; the full
  per-crate suite; rustdoc with `.github/CONTRIBUTING.md`'s own flag
  (13 is the standing baseline); `mdbook build docs` with the link
  check.
- CI on the branch: **quote the run's own conclusion and its
  non-success count.**

## §6 — escalate rather than deciding

- If same-depth comma tracking cannot be made reliable at this cost —
  **then say so and propose what is**, including the possibility that
  the census must become stronger instead (for example asserting a
  hash or a normalised shape per site, so an in-place edit moves a
  number). That is a legitimate outcome; a predicate that cannot see
  the real layout is not.
- If widening the window turns any **currently safe** call site red.
  That is information about the codebase, not an obstacle — report it.

## §7 — exit conditions

1. The window is bounded by `;` or a same-depth `,`, whichever comes
   first — or `§6`'s escalation is reported with a concrete proposal.
2. **`§3`'s plant 2 goes red on the predicate** — the in-place
   four-line shape, site count unchanged. This is the round's own
   acceptance test.
3. `§3`'s plants 1, 3 and 4 all behave as stated, each with captured
   output, each reverted, green after.
4. Round 1's exact-match assertion kept, and the false-positive plant
   still yields exactly one violation.
5. Which of the four historical shapes the predicate now reaches,
   stated as a measurement.
6. `§4`'s doc comment states that the predicate is the protection in
   the in-place case.
7. `§5`'s gates after the last edit, `DEC-007` re-measured with the
   runner, CI's own conclusion quoted.
