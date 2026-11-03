# DIA-004 — the comment, and the second query

**`dialect.rs` tells a reader that conditional compilation lives in two
files. It lives in eight.**

`crates/peisear-storage/src/dialect.rs:62`:

> **No `#[cfg(feature = "postgres")]` anywhere outside this module
> and `pool.rs`.**

and, of the five files it names as touching dialect-dependent types:

> None of the fourteen occurrences carries a `#[cfg]`.

**Both sentences are false.** The architect's measurement, 2026-11-03 —
`grep -rn 'feature = "postgres"' crates/peisear-storage/src --include=*.rs`,
which counts attribute **and** comment lines alike, so **re-measure
precisely rather than trusting this table**:

| File | sites | named by the comment? |
|---|---|---|
| `user_burnout.rs` | 3 | no |
| `personal_metrics.rs` | 3 | no |
| `metrics_snapshots.rs` | 1 | no |
| `optimistic_lock_guards.rs` | 2 | no |
| `sprints.rs` | 2 | **yes** |
| `issue_events.rs` | 1 | **yes** |
| `lib.rs` | 1 | **yes** |

The attribute is spelled `#[cfg(all(feature = "postgres", not(feature =
"sqlite")))]` rather than the bare form the comment quotes. **That is not
a defence of the comment**: a reader takes it to mean *you will not find
conditional compilation scattered around*, and they will.

## §0 — why this is not a tidiness item

**Three of those files carry a second, postgres-only SQL body** for
production queries — `const QUERY` in `user_burnout.rs` and
`metrics_snapshots.rs`, a `format!`ed `sql` in `personal_metrics.rs` —
and they are the **health and sustainability queries**, this product's own
domain logic. Two bodies per query, each needing the same change when the
query changes, **with published rustdoc saying they do not exist.** A
contributor who trusts the comment will edit one and ship the other
unchanged, and no test they run under default features will compile the
one they missed.

**`optimistic_lock_guards.rs`'s two sites are a different thing** — a test
helper (`migrated_pool`, line 75) outside its own `mod tests` (line 124).
Report it separately; do not fold it into the query finding.

## §1 — the design question, which is the architect's

**Why the second bodies exist, as far as the architect has read**: they
diverge on **date and time functions**, not placeholders.
`user_burnout.rs`'s sqlite body uses SQLite date arithmetic; its postgres
body uses `EXTRACT(EPOCH FROM (now() - …))`. `PG-004`'s `dialect::sql`
rewrites `?N` / `?` to `$N` — a parser, and it is why 284 call sites
needed no second spelling — **but a placeholder rewriter cannot rewrite a
date function.** So the paired bodies sit exactly where the existing seam
cannot reach.

**If that holds, the code is right and the comment is stale.** The
question then is whether date-function divergence deserves its own seam —
a `dialect::` helper the way placeholders got one — or stays as paired
bodies that are honestly labelled. **That is a design decision and it is
not this round's to make.** Establish the facts; the architect decides.

## §2 — what this round is

1. **Re-measure**, and report the table above as you find it, including
   whether any site is test-only.
2. **For each paired query body, state what the two bodies actually
   differ on** — date functions, or something else. One line each. This
   is the evidence the design decision needs.
3. **Correct `dialect.rs`'s comment** to what the code holds. Say where
   conditional compilation lives and *why it lives there*, since the
   reason (the placeholder seam cannot cover date functions) is the part
   that stops someone "cleaning it up" wrongly.
4. **Add the clippy step this gap was first proposed for**, beside
   `test.yml:997`'s existing postgres line:
   `cargo clippy --workspace --all-targets --no-default-features
   --features postgres -- -D warnings`.
   **It is green today — the architect ran it, exit 0, zero diagnostics.**
   Say so in the report. A guard that passes on arrival is not evidence of
   anything except that it is wired up; what makes it worth two lines is
   that `test.yml:39`'s only clippy invocation runs default features and
   therefore never compiles the bodies above.

**Not this round**: merging the paired bodies, introducing a date-function
helper, or changing any query. If a query looks wrong, **say so and
stop** — a wrong query is its own round.

## §3 — the trap

**A doc comment in this crate is published rustdoc.** Editing it means
the rustdoc gate applies: `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links"
cargo doc -p peisear-storage --no-deps` — and if you add a link to an
item, check it resolves and is public, the defect `CFG-002` round 2 hit in
`peisear-web`. The `peisear-web` baseline is 6; **measure
`peisear-storage`'s own baseline before your edit** so you can say it is
unchanged after.

**And the clippy step must be seen to cover what it exists for.** Do not
report it green and stop. **Plant a clippy violation inside one of the
postgres-only bodies, confirm the new step fails and the default-features
step at `test.yml:39` passes, then revert.** That single plant is the
entire argument for the two lines; without it they are decoration.

## §4 — gates

`cargo fmt --check`; `clippy --workspace --all-targets -- -D warnings`
**and** the new postgres invocation; `cargo doc` for `peisear-storage` with
its measured baseline; the full per-crate suite with `DEC-007` counted
from the runner — **and note that `DEC-007` is defined by `test.yml`'s own
block lines**: `-p peisear-storage --lib` excludes that crate's doctest,
a bare `-p peisear` includes its own. This round should move the count by
zero unless you add a test; say which.

## §5 — escalate rather than deciding

- If the two bodies differ on anything **other** than date/time functions,
  that breaks `§1`'s premise. **Report it and stop**; the design question
  changes shape.
- If correcting the comment would require describing a design you think is
  wrong, **write what the code does and say what you think is wrong
  separately.** Do not improve the design in a comment.

## §6 — exit conditions

1. The site table re-measured and reported, test-only sites distinguished.
2. Each paired body's point of divergence stated in one line.
3. `dialect.rs`'s comment true against the code, and carrying the *why*.
4. The postgres clippy step added, **seen red on a planted violation in a
   postgres-only body**, with the default-features step shown to miss it.
5. `§4`'s gates after the last edit, `peisear-storage`'s rustdoc baseline
   quoted before and after, `DEC-007` from the runner.
