# Handoffs — absence is not error

**0.60.0.** `RFC 022`, accepted 2026-10-31, authorized by the owner the
same day. **Read `RFC 022` §3 before any round** — it is one sentence
and every round is an application of it.

**One rule**: absence degrades; a value the operator supplied and the
product cannot use **refuses startup, by name, naming what would have
been accepted.**

Thirteen operator-facing variables. **Three already refuse**
(`PEISEAR_SETUP`, `JWT_SECRET`, `BIND_ADDR`). **Four discard a supplied
value in silence** — `COOKIE_SECURE`, `SMTP_PORT`, `SMTP_TLS_MODE`,
`RUST_LOG` — and the first two of those decide cookie security and
transport security. `§10.51` is this series' entry and closes with it.

**The defect is not four bad parses.** It is one rule that was never
written down and therefore never applied, so a new variable lands
wherever its author's habit puts it. `RFC 022` writes it down;
`CFG-003` makes it enforceable.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| CFG-001 | [the two that already have an error type](./CFG-001-the-two-that-already-have-an-error-type.md) | `COOKIE_SECURE` and `RUST_LOG`, both reachable from an existing `ConfigError`/`Result` path. The security-shaped one first | **Closed** — both refuse by name, absence still degrades, and the round's two best outputs were findings rather than code. **It proved `CFG-002`'s own trap a round early**: run against the resolver change alone, before `docs_scan.rs` was touched, `rule 4: no literal default found in config.rs for COOKIE_SECURE -- has the call shape changed?` fired — `§10.36` instance 6's recorded remedy demonstrated in force, then re-anchored and re-tested. **And it found `EnvFilter`'s grammar far more permissive than `RFC 022` §4 assumed** — `RUST_LOG=dbug` is accepted as a target name with an implicit `TRACE` level, so the rule's reach there is a malformed *level* after an `=` and little else; §4 is amended with the measured table, the decision unchanged and the claim corrected. **It also caught a false claim of the architect's in two published documents** — that `054`'s work had landed under `[Unreleased]` — by checking the premise instead of writing to it, and declined to invent content for a round nobody had handed it. Architect's verification: `DEC-007` **609** re-counted with the runner (274 per-crate + 335 integration, 0 failed), 49 CI check-runs green on `main`, `fmt`/`clippy -D warnings`/`mdbook` clean, rustdoc 13 real doc warnings unchanged — their figure exact where the architect's first count of 16 had caught two summary lines and a dependency notice. One instruction of the architect's was wrong and the round was right to decline it: restating the accepted set at all five documentation sites, against the project's own *state a fact once* rule (`docs/specification/README.md:27`) | **Open**
| CFG-002 | [the crate with no error channel](./CFG-002-the-crate-with-no-error-channel.md) | `SMTP_PORT` and `SMTP_TLS_MODE` — and `SmtpConfig::from_env() -> Option<Self>` becoming fallible, which is this release's real engineering content | CFG-001 | **Closed in full, both rounds.** `SMTP_PORT` and `SMTP_TLS_MODE` refuse by name, `from_env` is fallible — a signature and one `?`, exactly as predicted — absence degrades for both and for the whole channel, and the cascade is proven by composing the resolvers in `from_env`'s own order and asserting the first `?` stops the second. **The trap was walked in order and seen red first** (`left: None, right: Some(465)`), `§10.36` instance 6's remedy in force for the second time in this series. **`TLS_MODE_SPELLINGS` is better than the handoff asked for**: one const read by both the parser and the error message, so the two cannot drift at all. Architect's verification: `DEC-007` **619** with the runner (284 per-crate + 335 integration, 0 failed), 49 CI check-runs green, `fmt`/`clippy`/`mdbook` clean. **Round 2** ([the only `unsafe`](./CFG-002-round-2-the-only-unsafe.md)): round 1 added `crates/peisear-notify/src/config/tests.rs:186`, **the only `unsafe` block in the workspace** — measured, one occurrence — whose safety argument is `--test-threads=1`, a convention in a contributing file rather than anything enforced. It mutates global process state to test a property about seven input values, and the codebase's own parameter-passing pattern — which round 1 cited as its own rationale — already solves it. `from_env` becomes a thin wrapper over a pure `from_values`. `TTY-002` round 2's shape: the subject achieved, a property of the project's standard not met **Round 2 closed**: `grep -rn 'unsafe {' crates/*/src` returns zero and `set_var`/`remove_var` returns zero workspace-wide, both re-measured by the architect; `from_env` is a thin wrapper over a pure `from_values`; `DEC-007` **620**. The round re-measured the `unsafe` count rather than trusting the review's own sentence, and **broke one thing and fixed it before reporting green** — a public doc comment linking a `pub(crate)` item, caught by `rustdoc::private_intra_doc_links` against a zero baseline for that crate, de-linked with the reason now recorded in the comment itself. **One finding carried to `CFG-003`, and it is the architect's against the architect's own handoff**: `from_values` takes seven `Option<&str>` parameters, so a transposition of the two credential arguments compiles silently and — measured — no test would catch it, since every test call passes `None`s and none asserts a string field. Correct today; one test closes it |
| CFG-003 | [the rule, enforced](./CFG-003-the-rule-enforced.md) | A guard so a fifth variable cannot be added in the silent shape. Until it lands, `RFC 022` is prose | CFG-002 | **Round 1 built the rule; round 2 open, and `§10.51` stays open with it.** `docs_scan` rule 8 exists as a plantable predicate plus an asserted census — 12 distinct names and 16 literal `env::var` sites, both reconciling exactly with the architect's own independent count. Round 1 found its own predicate bug by running it (a `;`-bounded window let one planted fallback flag five innocent `from_values` arguments), re-bounded it and pinned the fix with an exact-match assertion; it named `RUST_LOG`'s unreachability in the doc comment; and it built the carried `§4a` test as asked, declining the stronger named-field struct with a reason. **But its central claim is false as measured.** Every real defect had `env::var("…")` on one line and `.unwrap_or(…)` three lines later — `SMTP_PORT`'s was four lines at `c53e7bc` — and the window is bounded to the current line, so the predicate is blind to the exact layout all four defects had. Round 1's coverage table reports three as caught; it tested one-line reconstructions. **Two architect plants**: the four-line shape added as a new site was caught only by the census, not the predicate; the same shape planted **in place**, keeping the site count at 16, gave `214 passed; 0 failed` — **nothing fired at all**, which is what a reintroduction actually looks like. **Round 2** ([the layout the defects actually had](./CFG-003-round-2-the-layout-the-defects-had.md)): bound the window by `;` *or* a same-depth `,`, whichever comes first — a wrapped chain has no same-depth comma before its fallback, while `from_values`'s arguments do, so the real layout is caught and round 1's false positive cannot return. The in-place plant is the round's own acceptance test |
| REL-0.60.0 | *not yet written* | The cut. **It will also carry `054`'s work, if `054` has merged by then** — the front door has no release of its own, so it lands under `[Unreleased]` and whichever release is cut next picks it up. *Corrected 2026-10-31 by `CFG-001`, which checked this premise instead of writing to it: at that round, `QS-001` and `QS-002` were both still open and `[Unreleased]` was empty. The original wording said the work had landed — a status asserted ahead of the thing it describes, the architect's* | CFG-001–CFG-003 | **Not yet written** — the architect writes it when the work rounds close |

**`CFG-001` before `CFG-002`** because the first works inside an error
type that already exists and the second has to build one; getting the
error *wording* settled on the easy path first means `CFG-002` copies a
shape rather than inventing one.

## What this series is not

**Not a change to graceful degradation.** An unconfigured email channel
still never blocks a boot. `RFC 022` §3's two halves exist precisely so
this is not in tension: **absence is not error.** A round that makes an
*unset* variable refuse has misread the rule.

**Not a widening of what is accepted.** `COOKIE_SECURE` must keep
accepting `true`, which works today — narrowing it would break working
deployments in the refusing direction, a worse failure than the one
being fixed (`RFC 022` §5).

**Not a rewrite of the three that are already right.** `PEISEAR_SETUP`,
`JWT_SECRET` and `BIND_ADDR` are the model. Read their error messages
and match them; do not improve them in passing.

## The trap this series will walk into, named once here

**`docs_scan` reads the code `CFG-002` changes.** `smtp_port_default`
(`docs_scan.rs:347`) finds the literal `env::var("SMTP_PORT")` and then
`.unwrap_or(` within the next 200 characters. `CFG-002` moves the
default out of `.unwrap_or(`, **so this lookup will stop matching** —
by design.

**The right outcome is that the guard goes red and is then updated.**
The wrong outcome — and the one that has happened here before — is that
it silently returns `None` and the rule examines nothing.
**`§10.36` instance 6 is this exact defect in this exact function**, and
that entry records the remedy: rule 4 now asserts its own lookup
succeeded rather than trusting it. **Verify that assertion fires. Do not
assume it does because the entry says it was added.**

## The standing exception applies

A defect found in the shipped product while this series is open earns a
round of its own rather than being folded into whichever round noticed
it. 0.58.0 closed five rounds against three scoped that way, 0.58.1 two
against two, and 0.59.0 four against four; every addition was real.
