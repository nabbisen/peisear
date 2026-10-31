# CFG-002 round 2 — the only `unsafe`

**`CFG-002`'s subject is accepted and is not reopened.** `SMTP_PORT`
and `SMTP_TLS_MODE` refuse by name, absence degrades, the cascade is
tested, and `TLS_MODE_SPELLINGS` is a better answer than the handoff
asked for. **This round is about one thing the round introduced.**

## §1 — what it is, measured

`crates/peisear-notify/src/config/tests.rs:186`:

```rust
// SAFETY: `cargo test -p peisear-notify` runs with
// `--test-threads=1` (`.github/CONTRIBUTING.md`), so no other
// test observes the environment concurrently with this one.
unsafe { std::env::remove_var(var); }
```

**`grep -rn 'unsafe {' crates/*/src` returns exactly one occurrence in
the whole workspace, and round 1 created it.** Re-measure that rather
than taking it from this sentence.

**Three reasons it does not stand, in order:**

1. **Its soundness rests on a flag a human remembers.**
   `--test-threads=1` is a convention in a contributing file, not
   something enforced. `cargo test -p peisear-notify` — the natural
   thing to type — does not meet the stated precondition. The test
   still passes, because no sibling test reads `SMTP_*`; that is a
   fact about today's test list, not an invariant.
2. **It mutates global process state to test a pure property.** *"A
   wholly unconfigured channel yields `Ok(None)`"* is a statement
   about seven input values.
3. **The codebase's own pattern already solves this, and round 1
   cited that pattern as its own rationale** for
   `resolve_smtp_port`/`resolve_tls_mode` taking values as
   parameters — then did not apply it to the one test that needed it.

## §2 — the change

**`from_env` becomes a thin wrapper over a pure function.** Shape, not
prose:

```rust
pub fn from_env() -> Result<Option<Self>, SmtpConfigError> {
    Self::from_values(
        env::var("SMTP_HOST").ok().as_deref(),
        // … the other six, in from_env's current order
    )
}

fn from_values(
    host: Option<&str>, /* … */
) -> Result<Option<Self>, SmtpConfigError> { /* today's body */ }
```

- **Every `env::var` call stays in `from_env` and nothing else moves.**
  The `.filter(|s| !s.is_empty())` absence treatment, the four
  early `Ok(None)` returns, the two resolver calls and their order,
  and the struct assembly all keep their current behaviour —
  **this is an extraction, not a rewrite.** If you find yourself
  changing what a value means, stop and escalate.
- **`from_values`' visibility is the smallest that lets the test call
  it.** `pub(crate)` if that suffices; do not make it public API.
- **The temporaries are fine in one call expression** — they live to
  the end of the enclosing statement — but if the borrow checker
  disagrees, bind the seven `Option<String>`s first and pass
  `.as_deref()`. Report which you needed.

## §3 — the test, and the `unsafe` going away

- **`an_entirely_unconfigured_channel_still_yields_ok_none` calls
  `from_values` with seven `None`s** and asserts `Ok(None)`. No env,
  no `unsafe`, no flag dependency.
- **Delete the `unsafe` block and its `SAFETY` comment.** Then
  re-measure: `grep -rn 'unsafe {' crates/*/src` must return **zero**,
  and say so with the command's output.
- **Keep the property, do not weaken it.** The test must still prove
  the *whole channel* degrades, not just that one resolver does.
- **Add the symmetric case while you are here**: all seven `None`
  except a present, malformed `SMTP_PORT` still returns `Err` —
  because the early `Ok(None)` returns sit *before* the port resolver,
  so a wholly unconfigured channel with a bad port returns `Ok(None)`
  and never sees the bad value. **State which it does in the report**;
  that ordering is a deliberate property of `RFC 022` §3's first half
  and should be recorded either way, not discovered later.

## §4 — what must not change

- **No behaviour changes at all.** This round is a refactor and two
  tests. If `DEC-007` moves by anything other than the tests you add,
  something else changed.
- The four early `Ok(None)` returns keep their order relative to the
  two resolvers (`§3`'s last point is to *record* that order, not
  alter it).
- `TLS_MODE_SPELLINGS`, both resolvers, the error type and every
  message stay exactly as round 1 left them.

## §5 — gates

- Full per-crate suite; `peisear-notify` was **17** at `8e9b7bd`,
  measured by the architect — re-measure rather than trusting it.
  `DEC-007` was **619**.
- `fmt`; `clippy --workspace --all-targets -D warnings`; `clippy
  -p <crate> --no-default-features --features postgres` for the four
  in that graph, as round 1 ran.
- `cargo test -p peisear-web --lib` — `docs_scan` reads
  `peisear-notify`'s source. **`smtp_port_default` now anchors on
  `fn resolve_smtp_port(`, which this round does not move** — so the
  rule should stay green. **If it goes red, that is the finding**: say
  what it anchored on and what moved.
- rustdoc with `.github/CONTRIBUTING.md`'s documented flag — 13
  warnings is the standing baseline; `from_values` gets a doc comment.
- `mdbook build docs` with the link check.

## §6 — `CHANGELOG.md`

**Probably nothing.** `[Unreleased]`'s `CFG-002` entry describes
behaviour, and this round changes none. **Decide and say which** — if
you add nothing, say that you considered it and why, rather than
leaving the omission unexplained.

## §7 — escalate rather than deciding

- If the extraction cannot be behaviour-preserving for a reason this
  handoff has not seen.
- If removing the `unsafe` turns out to leave another env-mutating
  test somewhere in the workspace — then the subject is wider than
  this round and that is the architect's call.

## §8 — exit conditions

1. `from_values` exists, pure, smallest sufficient visibility;
   `from_env` is a thin wrapper; every `env::var` call still in
   `from_env`.
2. The extraction is behaviour-preserving, and the report says how you
   checked rather than asserting it.
3. The test calls `from_values` with seven `None`s; the `unsafe` block
   and its `SAFETY` comment are gone.
4. **`grep -rn 'unsafe {' crates/*/src` returns zero, with the output
   quoted.**
5. `§3`'s symmetric case added, and the early-return ordering recorded
   either way.
6. `DEC-007` re-measured with the runner; any delta accounted for by
   the tests added and nothing else.
7. `§5`'s gates after the last edit, with numbers, and `docs_scan`'s
   behaviour stated.
8. The `CHANGELOG` decision made and explained (`§6`).
