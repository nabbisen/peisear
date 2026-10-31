# RFC 022: Absence is not error — one rule for a value the operator supplied

**Status**: accepted 2026-10-31, with `0.60.0` (`ROADMAP.md`).
**Author**: the architect. **Supersedes nothing.** Closes `§10.51`.

## Summary

peisear reads thirteen environment variables. **Three of them refuse to
start when the operator's value is wrong, by name. Four discard it
silently and carry on with a default.** One of the four decides whether
the session cookie is marked `Secure`; another decides transport
security for mail.

This is not a parsing problem. It is **two rules for one input class**,
and the permissive half is the half that matters. This RFC states one
rule, applies it to the four, and says why it has no exceptions.

## §1 — the measurement this comes from

Measured 2026-10-31 at `f445e89`. **Thirteen operator-facing variables**
(`env::var` across `crates/*/src`, excluding doc comments and the scan
rules' own string literals).

*A correction belongs here rather than in a footnote: an earlier count
said sixteen. It double-counted `main.rs`'s four
`env::var(…).is_ok()` presence probes as variables of their own.*

**Refuses a bad value, by name — the shape this RFC generalises:**

| Variable | What it does |
|---|---|
| `PEISEAR_SETUP` | refuses, listing **both** accepted values (`wizard.rs`) |
| `JWT_SECRET` | refuses the published placeholder, an empty value, and a `DATABASE_URL` with no filesystem path to persist a generated key beside |
| `BIND_ADDR` | single parse, named error giving the expected form and an example |

**Discards a bad value silently:**

| Variable | The read | What an operator loses |
|---|---|---|
| `COOKIE_SECURE` | `.map(\|v\| v == "1" \|\| v.eq_ignore_ascii_case("true")).unwrap_or(false)` | `yes`, `on`, or `1` with a trailing space → **`false`**: the session cookie is not marked `Secure` |
| `SMTP_PORT` | `.ok().and_then(\|s\| s.parse::<u16>().ok()).unwrap_or(465)` | the port, **and the TLS mode with it**, since the mode derives from the port |
| `SMTP_TLS_MODE` | `.ok().and_then(from_str_loose).unwrap_or_else(default_for_port)` | transport security |
| `RUST_LOG` | `try_from_default_env().unwrap_or_else(\|_\| "info,sqlx=warn,…")` | the verbosity they asked for |

**Neither, and correct as they are**: `DATABASE_URL` (absent takes a
default; a malformed present value is surfaced by the connection
attempt, not swallowed) and the five remaining `SMTP_*`, which are
absence-shaped with no value to misparse.

## §2 — what is actually wrong, which is not "the parsing is sloppy"

**`PEISEAR_SETUP=nerver` is refused at startup, by name, with both
accepted values listed. `SMTP_TLS_MODE=startls` is discarded in
silence.** Same product, same launch environment, **documented two
sections apart on the same page** — and `wizard.rs`'s own comment
explains the first: *"A signal read as its own absence is `§10.49`'s
own class of defect."*

So the rule already exists here. It is applied to three variables and
not to four, and **nothing records which group a new variable belongs
in** — so the next one lands wherever its author's habit puts it. That
is the defect: not four bad parses, but **one rule that was never
written down, and therefore never applied.**

**`COOKIE_SECURE` is why this is urgent rather than tidy.** It fails
**open**, in a security control, in the one direction an operator
cannot detect from behaviour — `false` is also the legitimate default,
so nothing they can observe distinguishes *"my value was rejected"*
from *"I never set it"*. `docs/security/hardening.md` presents `Secure`
as conditional on exactly this variable.

Stated fairly, because it bounds the claim: **the documented value is
`1`, five places say so consistently, and the code accepts it.** This
is not a defect against the documentation. It is what happens to an
operator who types something reasonable the documentation did not
predict.

## §3 — the rule

**Absence degrades. A value the operator supplied and the product
cannot use refuses startup, by name, naming what would have been
accepted.**

One sentence, two halves, no third category.

- **Absence degrades**, exactly as today. An unconfigured email channel
  must not block a boot; an unset `COOKIE_SECURE` means `false`; an
  unset `RUST_LOG` takes the built-in filter. **Nothing in this RFC
  changes graceful degradation**, because absence is not error.
- **A supplied value that cannot be used refuses.** The operator stated
  an intent; carrying on with a different one is the failure. A
  refusal at startup is loud, immediate, and fixable in seconds.

**Why refuse rather than warn.** A warning would leave the product
with *two* rules again — refuse for some inputs, warn for others — and
someone would then have to classify every future variable into the
right half. That classification is the continuous maintenance cost,
and it is the thing that drifts. Refusing is also **less code**:
`ConfigError` already exists with named variants and its refusal path
is already walked by tests, where warning needs a mechanism, a
decision about where warnings go, and a reader who must learn which
bad values stop the server and which do not.

**The error message is part of the rule, not a nicety.** `PEISEAR_SETUP`
is the model: name the variable, quote what was given, and list what
would have been accepted. An error that says only *"invalid"* tells an
operator they are wrong without telling them what right looks like,
which fails the same standard the rule serves.

## §4 — no exceptions, and `RUST_LOG` is why that is a decision

`RUST_LOG` is the rule's hardest case: refusing to start because a log
filter is typo'd looks disproportionate, and logging is not a security
property.

**It takes the rule anyway.** Carving it out reintroduces the
two-rule split this RFC exists to end, and every future variable then
needs someone to decide which half it belongs in. One rule with an
uncomfortable edge is simpler to keep true than two rules with a
boundary to maintain — and the edge is not very uncomfortable: the
product already refuses a typo'd `BIND_ADDR`, and nobody has called
that wrong.

**Its own bound — measured 2026-10-31 against `tracing-subscriber`
0.3.23, and far narrower than this section first claimed.** `CFG-001`
found `EnvFilter`'s grammar more permissive than it looks while
writing the test for a malformed value; the architect then probed it
directly:

| Value | `EnvFilter` |
|---|---|
| `dbug` | **accepted** — a target name with an implicit `TRACE` level |
| `DBUG`, `not a valid directive!!`, `info,,`, `trace=info` | **accepted**, the same way |
| `sqlx=not_a_real_level` | refused — *"expected one of off, error, warn, info, debug, trace, or a number 0-5"* |
| `=info` | refused — *"invalid filter directive"* |

**So `RUST_LOG=dbug`, the likeliest operator typo there is, is
accepted in silence**, and this rule's reach on `RUST_LOG` is a
malformed *level* after an `=`, plus an empty target. Nearly all of
`RUST_LOG`'s silent-intent problem is out of reach — not half, as the
first version of this section said. That earlier sentence promised
more than the mechanism can deliver, which is `§10.35`'s own class
inside an RFC instead of a requirement.

**The decision stands and the claim is what changed.** `RUST_LOG`
keeps the rule: the bad-level shape is a real typo too, the cost is
nil once written, and an exception would restore the two-rule split
this RFC exists to end. What may not stand is a reader concluding
that a typo'd `RUST_LOG` refuses. Mostly it does not.

## §5 — applied, one line each

- **`COOKIE_SECURE`** — refuse anything outside the accepted set.
  **The set must keep accepting `true`**, which works today:
  narrowing to `1` alone would break working deployments *in the
  refusing direction*, which is a worse failure than the one being
  fixed. Document the whole set wherever the variable is documented.
- **`SMTP_PORT`** — absent takes `465`; present must parse as `u16` or
  refuse. The default survives; it moves out of `.unwrap_or(`.
- **`SMTP_TLS_MODE`** — absent derives from the port, as today;
  present must parse or refuse, listing the accepted modes.
- **`RUST_LOG`** — absent takes the built-in filter; a present value
  `EnvFilter` rejects refuses.

**`peisear-notify` has no error channel for this.**
`SmtpConfig::from_env() -> Option<Self>` cannot express *"a value was
given and it is wrong"* at all — `None` means *"not configured"*. It
becomes fallible. **One call site** (`crates/peisear/src/main.rs:119`),
inside a function that already returns `Result`, so this is a signature
and one `?`, not a refactor.

## §6 — the alternatives, named so they are not rediscovered

- **Warn instead of refusing.** Rejected in `§3`: two rules, a
  classification to maintain, and more code.
- **Document today's behaviour and change nothing.** The *less is
  more* reading, and the honest one to consider — rejected because a
  security control that fails open silently is not a documentation
  problem, and because the refusing half of this same product is two
  sections away on the same page.
- **Delete `COOKIE_SECURE` and derive it.** Considered properly:
  peisear sits behind a proxy and cannot know the scheme from its own
  socket, so deriving means trusting `X-Forwarded-Proto` — an
  attacker-controllable header unless the proxy is trusted, which is a
  worse decision than a boolean — and defaulting it on breaks first
  run on `http://localhost`. The variable earns its place.

## §7 — what this RFC does not claim

- **It does not claim the rule is new.** It is `PEISEAR_SETUP`'s,
  `JWT_SECRET`'s and `BIND_ADDR`'s rule, written down and applied to
  the rest.
- **It does not change graceful degradation**, and `§3` says why the
  two are not in tension.
- **It does not fix every silent loss of operator intent** —
  `RUST_LOG`'s well-formed-but-matching-nothing half is out of reach
  (`§4`), and an empty-string `SMTP_HOST` is deliberately still read
  as absence.
- **It does not settle `§10.36`.** A rule stated in a specification and
  a rule enforced by a check are different things; the guard is
  `CFG-003`, and until it lands the rule is prose.
