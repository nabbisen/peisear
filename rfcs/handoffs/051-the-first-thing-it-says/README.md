# Handoffs — the first thing it says

**0.58.0.** `RFC 020`, accepted 2026-10-15. One subject: **a product
that says where its data is and whether it is reachable, instead of a
document that asks you to find out.**

Three rounds, ordered by risk rather than by the RFC's sections. The
only thing in this release that can hurt a user is a prompt in a
container, so that is built and proven first, on a stub.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| WIZ-001 | [the skip](./WIZ-001-the-skip.md) | **Not the wizard.** `std::io::IsTerminal` plus *configuration already exists*, proven by a CI walk over four cases including a real pty — with a stub body that prints one line. *"A wizard that blocks a `docker run` is an outage"* | — | **Closed** — 2 rounds. Four cases green with case 3 through a real pty and case 4 now unable to pass on a server that never started; the plant reproduced in review in both directions. **The safety floor is in place before the feature exists** |
| WIZ-002 | [the wizard](./WIZ-002-the-wizard.md) | Two questions, two statements, and `env_file()` as a **pure function** so the written file can be tested as a string. **Carries one question for the owner** (§1) | WIZ-001 | **Closed** — 4 rounds. Two questions, three statements, the cookie flag written commented at its default, and the database answer validated rather than blindly prefixed — all three shapes reproduced through a real pty in review, with **no key written for a rejected answer**. **`REL-0.58.0` still waits on the owner for the question count** |
| WIZ-003 | [what the documents are for now](./WIZ-003-what-the-documents-are-for-now.md) | `RFC 020`'s own test of itself — *prevented or merely moved?* — answered honestly for all three defects, plus re-pointing three documents | WIZ-002 | **Closed** — 1 round. `first-run.md` checked against the real binary line for line in review. **One of three defects prevented**, reasoned from primary sources and written plainly; the SMTP gap reported, not taken |
| WIZ-004 | [the skip that agrees with the loader](./WIZ-004-the-skip-that-agrees-with-the-loader.md) | **The wizard says *“No configuration found”* when a `.env` sits in an ancestor directory, then writes a second one and shadows it** — measured. Detection is cwd-only; `dotenvy` searches ancestors. The standing exception applies: a defect that blocks the release is not a scope question | — | **Closed** — 1 round. Detection now asks the loader; the ancestor-`.env` case reproduced in review, binding the parent’s **3999** with no wizard and no second file. CI case 5 asserts the address, not just the absence |
| REL-0.58.0 | [the release candidate](./REL-0.58.0-release-candidate.md) | The cut, `DEC-028`, `DEC-056`. **All three rounds are closed.** Carries the accounting paragraph for the notes, the claim to falsify (a configured instance is untouched) and four limits | the owner’s answer on the question count | **Blocked — awaiting the owner** |

**Status lives in the table above, not in a message.**

---

## One thing needs the owner before `WIZ-002` acts

**`ROADMAP.md`'s authorized row says the wizard *"asks five
questions"*. `RFC 020` §1's own sketch asks two.** Both are in the
record and they disagree.

Measured at `e5b0ad5`:

| | Count |
|---|---|
| Variables `Config::from_env` reads | **4** — `DATABASE_URL`, `JWT_SECRET`, `BIND_ADDR`, `COOKIE_SECURE` |
| Variables `configuration.md` documents | **5** — those plus `RUST_LOG` |
| Variables the workspace reads | **12** — plus seven `SMTP_*` |

So *"five"* is the **documented** set, not the surface, and the
surface is twelve.

**The architect recommends two questions** — where the database
lives, what address it listens on — plus **three statements**: the
signing key generated and where, **the cookie flag**, and the `.env`
written and where.

**Corrected 2026-10-25, after the owner asked for this to be reviewed
against the project's philosophy.** The first version recommended
two statements and left `COOKIE_SECURE` out, on the grounds that it
is *"correct at its default and wrong to ask about."* The first half
is true; the conclusion was wrong. `COOKIE_SECURE` sets the `Secure`
flag on the session cookie, so at its default of `false` the cookie
travels over plain HTTP — correct on `127.0.0.1`, wrong behind TLS,
which is every real deployment. **And the wizard writes a file the
operator keeps**, so that version would have configured an instance
and said nothing about a security control being off, leaving the flag
to be discovered by reading `configuration.md` — the *"read the
documentation"* step this release exists to remove.

**Still two questions**, so still not a configuration UI. The flag
becomes a statement and a **commented line at its built-in default**,
which is the convention `0.53.0` already chose for `.env.example`
*"so a copied file … can never hand anyone a setting they did not
choose"* — visible and editable without pinning a default a future
release might improve. `WIZ-002` §1a carries it.

`JWT_SECRET` is generated rather than chosen; `RUST_LOG` has a
working default and no wrong setting. Asking all five makes it the
*"configuration UI"* `RFC 020` §3 forbids; saying three things costs
one sentence and no question.

`ROADMAP.md`'s own rule — *"a row that changes after authorization is
re-authorized, not quietly edited"* — is why this is an ask and not
an edit. `WIZ-002` §1 carries it and tells the round to wait.

---

## What scoping measured, and what it answers

`RFC 020` §5 left three open questions. **Two are answered by
measurement, here.**

- **Q2: *"What happens on the second run after a partial answer?"***
  `Config::from_env` (`config.rs:26`–`:50`) already gives every one
  of its four variables a working answer without a `.env` — three
  defaults and a generated key. **So a partial `.env` is already a
  working configuration**, and the skip condition is **existence,
  not completeness**. Asking again because one line is missing would
  be the wizard surprising an operator who has already configured
  the instance.
- **Q3: *"Does `.env.example` survive at all? It is currently
  referenced by three documents and one scan."*** It is referenced
  by **ten live places**, including `config.rs`'s
  `PUBLISHED_PLACEHOLDER` **security refusal**, `docs_scan`'s rule 7,
  a **dedicated CI workflow**, and both specifications. **It
  survives**; removing it is a separate proposal, not a tidy-up.
- **Q1: *"Does it write `.env`, or print what to write?"*** Write,
  and say the path — the RFC's own leaning, and the only option
  consistent with its central claim. A printed block can be pasted
  wrong, and then the file and the explanation disagree again, which
  is the class this comes from.

**`std::io::IsTerminal` needs no dependency** — standard library
since 1.70, MSRV here is 1.88, and nothing in the workspace uses it
yet.

---

## The constraint that shapes how this is built

**`.env` and `.env.example` are behind a standing deny rule on file
tools.** Neither the dev team nor the architect can open one.

**That is not worked around. It is designed around, and the idiom is
already in the repository** —
`.github/workflows/env-example-walk.yml`'s own comment:

> *"the one walk nobody here can perform by reading the file. This job
> never reads `.env.example` either: it copies it, as the documented
> path instructs, and asserts on whether the server then runs — the
> job's own log is the entire answer, never a tool here opening the
> file."*

So: **`WIZ-001` proves the skip by behaviour in CI**, and
**`WIZ-002`'s writer is a pure `String`-returning function** tested as
a string, with the file write a thin shell. The distinction that
makes both possible: **Rust code may read a `.env`; a tool call may
not.** A test needing to check a written file uses
`fs::read_to_string`.

And **no test writes into the repository** — temp directories
throughout. `.gitignore` carries `.env*`, but that is the backstop,
not the plan: `§10.45` is the register entry for a generated secret
reaching a commit, and it is this project's own.

---

## What this series is not

**Not a configuration UI.** `RFC 020` §3. Two questions, once. A
round that grows a second screen has left its scope.

**Not SMTP.** Seven optional variables with a startup warning already.
`WIZ-003` reports that they are documented nowhere, as a finding — it
does not fix it.

**Not a replacement for the refusals.** `config.rs`'s `EmptySecret`
and `PublishedPlaceholder` (`FIV-001`) stay exactly as they are. The
wizard is an additional path to a good configuration, never a
substitute for refusing a bad one.

**Not a release that overstates itself.** `RFC 020` §2 asks whether a
wizard would have *prevented* the three defects that motivated it or
merely *moved* them, and `WIZ-003` §1 must answer per defect. **The
likely honest answer is one of three**, and the release notes say so.
`RFC 020` §4 already names the cheaper alternative; what it asked for
is the accounting, not a justification.

---

## What 0.57.0 left for whoever reads this first

- **`spec_scan` is now part of the sweep.** Six rules read
  `requirements.md` and `external-design.md`; a round that edits
  either meets them.
- **Plant every guard, and plant the right direction.** `SPEC-005`
  round 1 planted a wrong *number* and shipped a step that passed on
  an *absence*; round 2 planted the absence. Ask which direction your
  plant exercises.
- **The interactive shell here is not the shell CI runs.** `zsh` does
  not word-split an unquoted variable on IFS; `bash`, which Actions
  uses, does. `SPEC-005` round 2 lost a measurement to this and
  caught it by checking the bytes.
- **Count tests with the runner, never with a grep over source
  text.** `#[tokio::test(flavor = …)]` and `NFR-A11Y` have each
  defeated a pattern in the last two releases.
