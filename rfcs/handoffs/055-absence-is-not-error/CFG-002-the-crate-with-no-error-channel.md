# CFG-002 — the crate with no error channel

**`SMTP_PORT` and `SMTP_TLS_MODE` refuse a value they cannot use** —
and to do that, `peisear-notify` gains an error channel it does not
have. **That is this release's real engineering content.**

Read `RFC 022` §3 and §5, the series `README.md`'s named trap, and
`CFG-001`'s accepted error wording — **copy its shape rather than
inventing a second one.**

## §0 — the structural change, measured so it is not over-estimated

```rust
pub fn from_env() -> Option<Self>     // today: None means "not configured"
```

`None` cannot express *"a value was given and it is wrong"*. It becomes
fallible.

**One call site**: `crates/peisear/src/main.rs:119`, inside `run()`,
which already returns `Result` and already uses `?` at line 102. So
this is **a signature and one `?`**, not a refactor — and the round's
report should say so plainly rather than describing it as a large
change.

**The return shape is a design choice; propose it with a reason.** My
reading, which you may argue with: `Result<Option<Self>, SmtpConfigError>`
keeps the two facts separate — `Ok(None)` is *not configured*,
`Err(_)` is *configured wrongly* — and that separation is exactly what
`RFC 022` is about. Collapsing them into one type would re-merge the
distinction the release exists to draw.

## §1 — `SMTP_PORT`

Today, in `crates/peisear-notify/src/config.rs`:

```rust
let port = env::var("SMTP_PORT")
    .ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(465);
```

**Absent → `465`, unchanged.** Present must parse as `u16` or refuse.
**The default survives** — it moves out of `.unwrap_or(`, which is the
whole of the trap in `§3`.

## §2 — `SMTP_TLS_MODE`

```rust
let tls_mode = env::var("SMTP_TLS_MODE")
    .ok().and_then(|s| TlsMode::from_str_loose(&s))
    .unwrap_or_else(|| TlsMode::default_for_port(port));
```

**Absent → derived from the port, unchanged.** Present must parse or
refuse, **listing the accepted modes** — `from_str_loose` already knows
them, so derive the list from it rather than writing a second copy that
can drift.

**Say what `from_str_loose`'s looseness actually accepts**, in the
round's report. The rule is that a value the product cannot use
refuses; if `from_str_loose` already accepts several spellings of each
mode, that is fine and stays — but the error message must list what
genuinely works, not a tidier set than the parser implements.

**And name the cascade in the changelog**: a malformed `SMTP_PORT` used
to change transport security silently, because the mode derives from
the port. After this round it cannot, and that is the user-visible
point of the pair.

## §3 — the trap, which is this round's and is not optional

**`docs_scan::smtp_port_default` (`docs_scan.rs:347`) reads the code
you are about to change.** It finds the literal `env::var("SMTP_PORT")`
and then `.unwrap_or(` within the next 200 characters. **`§1` moves the
default out of `.unwrap_or(`, so that lookup will stop matching.**

**The right outcome is that the guard goes red and you then update
it.** The wrong outcome is that it silently returns `None` and the rule
examines nothing — **`§10.36` instance 6 is this exact defect in this
exact function**, caused last time by a refactor that was itself an
improvement.

Three things, in order:

1. **Make the change, run the suite, and report what `docs_scan` did** —
   red with which message, or green. Capture it.
2. **If it went green, that is the finding**, and it is more important
   than the round's own subject: the rule's lookup is not asserting
   itself, and `§10.36` instance 6's recorded remedy is not in force.
   **Report it; do not paper over it by keeping `.unwrap_or(` alive for
   the scanner's benefit.**
3. **Update the rule to read the new shape**, and make its lookup
   assert it found something, so the next person who moves this code
   gets a red rule rather than a vacuous green one.

**`docs_scan` rule 7 also knows these variables** — `SMTP_TLS_MODE` is
in a match arm at `docs_scan.rs:679` whose comment says it *"derives
from `SMTP_PORT` rather than naming its own literal"*. Check whether
that comment is still true after this round and correct it if not; a
stale comment inside a guard is how the next instance starts.

## §4 — what must not change

- **Absence still degrades, and this is the round where that matters
  most.** An unconfigured channel must not block a boot; `Ok(None)`
  must still be the answer when `SMTP_HOST` is unset.
- **The five absence-shaped `SMTP_*` keep reading an empty string as
  absence** (`RFC 022` §7). Do not tighten them.
- **No channel behaviour changes.** Nothing about how mail is sent,
  retried or logged is in scope.

## §5 — tests

- A refusal test per variable, **asserting the message names the
  variable, quotes the value, and lists what was accepted.**
- **An absence test per variable**, and one asserting an entirely
  unconfigured channel still yields `Ok(None)` and a booting server.
- **A test for the cascade**: a malformed `SMTP_PORT` no longer
  silently selects a TLS mode.
- `DEC-007` moves. Count with the runner; `peisear-notify` was **7**
  at `602ae80`, measured — re-measure rather than trusting that.

## §6 — gates

As `CFG-001` §5, plus: **this round edits a gate's source
(`docs_scan.rs`), so run that gate** — not the other call sites.
`clippy` under `--no-default-features --features postgres` as well,
since `peisear-notify` is in that graph. State numbers, not
adjectives.

## §7 — escalate rather than deciding

- If the return shape in `§0` should differ.
- **If `docs_scan` goes green in `§3.2`** — report and stop there for
  that item; whether the remedy is in this round or its own is the
  architect's call.
- If `from_str_loose`'s accepted spellings turn out to be wider or
  narrower than the documentation claims.

## §8 — exit conditions

1. `from_env` is fallible, the shape justified, the one call site
   updated, and the report says plainly how small the change was.
2. `SMTP_PORT`: absent `465`; present must parse or refuse by name.
3. `SMTP_TLS_MODE`: absent derives from the port; present must parse
   or refuse, listing modes **derived from `from_str_loose`**, not
   hand-copied.
4. **`§3`'s three steps done in order, with `docs_scan`'s behaviour
   captured** — and if it went green, that reported as the finding it
   is.
5. Rule 7's `SMTP_TLS_MODE` comment checked for staleness.
6. Absence still degrades, proven by test, including a wholly
   unconfigured channel.
7. The cascade named in `[Unreleased]`, added to `054`'s and
   `CFG-001`'s entries rather than replacing them.
8. `§6`'s gates after the last edit, `DEC-007` re-measured with the
   runner.
