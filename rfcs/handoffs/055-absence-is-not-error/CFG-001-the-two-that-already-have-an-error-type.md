# CFG-001 — the two that already have an error type

**`COOKIE_SECURE` and `RUST_LOG` refuse a value they cannot use**, both
on paths where a named error type and a `Result` already exist.

Read `RFC 022` §3 and §5 first. Read the series `README.md` for the
trap that belongs to `CFG-002`, not this round.

## §0 — why these two first

Neither needs new plumbing. `Config::from_env` already returns
`Result<Self, ConfigError>` and `ConfigError` already has named
variants (`crates/peisear-web/src/config.rs`); `main.rs:28–33` builds
the `EnvFilter` inside `main` itself. **So this round settles the error
*wording* on the easy path, and `CFG-002` copies the shape rather than
inventing one.**

`COOKIE_SECURE` goes first within the round: it is the security-shaped
one, and it fails **open**.

## §1 — `COOKIE_SECURE`

Today, at `crates/peisear-web/src/config.rs:48–50`:

```rust
let cookie_secure = env::var("COOKIE_SECURE")
    .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    .unwrap_or(false);
```

**Absent → `false`, unchanged.** A present value must be one of the
accepted set or **refuse**, with a `ConfigError` variant naming the
variable, quoting what was given, and listing the set.

**The accepted set is the design decision in this round, and it has one
hard constraint**: it **must keep accepting `1` and `true`
case-insensitively**, because both work today and narrowing them would
break working deployments *in the refusing direction* — a worse failure
than the one being fixed (`RFC 022` §5).

**Propose the set and justify it; do not widen it casually.** My own
reading, which you may argue with: add the three negatives that
correspond to what is already accepted — `0`, `false`, and nothing
else — so the set is `1`/`0`/`true`/`false`, case-insensitive, and the
error lists all four. Adding `yes`/`no`/`on`/`off` would be kinder to
the operator who typed `yes` and worse for the one reading the
documentation, because a set with eight spellings is a set nobody
remembers. **If you disagree, say so with the reason rather than
implementing the other one.**

**Documentation**: every place the variable appears must name the
accepted set — `docs/getting-started/configuration.md:60`,
`docs/getting-started/first-run.md:46`,
`docs/operations/deployment.md:149,154`,
`docs/security/hardening.md:105,149`. **That list is from a grep and it
is a floor, not a census** — run your own and say what you found.

## §2 — `RUST_LOG`

Today, at `crates/peisear/src/main.rs:28–33`:

```rust
tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| "info,sqlx=warn,hyper=warn,tower_http=info".into())
```

**Absent → the built-in filter, unchanged.** A present value
`EnvFilter` rejects must refuse, naming the variable and the parse
error `EnvFilter` itself produced — it is more specific than anything
this round would invent, so **pass it through rather than replacing
it**.

**Two things make this one awkward, and both need stating in the
round's report rather than solving quietly:**

1. **It runs before the subscriber exists**, so there is no `tracing`
   to report through. `main` returns `ExitCode` and already prints
   `Error: {e}` to stderr for the `run()` path — **use that same
   shape**, and say in the report how you kept the two consistent.
2. **`try_from_default_env` reads `RUST_LOG` *or* a default env
   variable name.** Confirm which names can reach this call before
   writing an error that names `RUST_LOG` specifically; if more than
   one can, the error must name the one actually set.

**Its bound is `RFC 022` §4's and it is not this round's to fix**: a
well-formed directive naming a target that does not exist is valid and
matches nothing. Do not try to detect it. **Do** make sure the round's
own report does not imply it was fixed.

## §3 — what must not change

- **No unset variable starts refusing.** If a round makes absence an
  error it has misread `RFC 022` §3.
- **`PEISEAR_SETUP`, `JWT_SECRET` and `BIND_ADDR` are untouched.** Read
  their error messages as the model; do not improve them here.
- **No warning mechanism is added.** The rule is refusal; a warning
  would be the two-rule split `RFC 022` exists to end.

## §4 — tests

- **A test per refusal, asserting the message names the variable and
  lists what was accepted** — not merely that an error occurred. The
  message is part of the rule (`RFC 022` §3), so it is part of the
  test.
- **A test that absence still degrades**, for both variables. This is
  the half most likely to regress silently and the half `RFC 022`
  promises.
- **A test that `1` and `true` still work** — the constraint in `§1`,
  pinned so a later tidy-up cannot narrow the set.
- `DEC-007` moves this round. **Count with the runner**, give before
  and after, and name the crate each delta is in.

## §5 — gates

`fmt`; `clippy --workspace --all-targets -D warnings`; the full
per-crate suite; `docs_scan`/`prose_scan`/`changelog_scan` via
`cargo test -p peisear-web --lib`; rustdoc with
`.github/CONTRIBUTING.md`'s own documented flag; `mdbook build docs`
with the link check. **Run them after the last edit**, and state the
numbers rather than that they passed.

**`docs_scan` rule 4 reads `COOKIE_SECURE`** — its population is
`["DATABASE_URL", "BIND_ADDR"]` plus `COOKIE_SECURE`
(`docs_scan.rs:582`). Changing this code may move what that rule
anchors on. **If the rule goes red, that is information, not an
obstacle**: report what it anchored on and what moved.

## §6 — `CHANGELOG.md`

Under `## [Unreleased]`, which **already holds `054`'s front-door
entry** — add to it, do not replace it, and **read it first.**

## §7 — escalate rather than deciding

- If the accepted set in `§1` should differ from the four proposed.
- If `try_from_default_env` can be reached by a variable name other
  than `RUST_LOG` (`§2.2`).
- If refusing on `RUST_LOG` turns out to break a documented workflow —
  that is a finding about `RFC 022` §4, which is a design question.

## §8 — exit conditions

1. `COOKIE_SECURE`: absent still `false`; `1`/`true` still accepted,
   pinned by a test; anything else refuses with a message naming the
   variable, the given value, and the accepted set.
2. The accepted set is stated, justified, and documented in every
   place the variable appears — with your own census, not this
   handoff's floor.
3. `RUST_LOG`: absent still takes the built-in filter; a value
   `EnvFilter` rejects refuses, carrying `EnvFilter`'s own error text;
   `§2`'s two awkward points answered in writing.
4. Nothing warns; nothing new refuses on absence; the three existing
   refusers untouched.
5. `§4`'s four kinds of test present, `DEC-007`'s delta counted with
   the runner.
6. `§5`'s gates run after the last edit, with numbers, and
   `docs_scan`'s behaviour on this change reported either way.
7. `[Unreleased]` added to, with `054`'s entry intact.
