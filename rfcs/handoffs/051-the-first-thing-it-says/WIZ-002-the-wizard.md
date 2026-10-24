# WIZ-002 — the wizard

**0.58.0, round 2 of 3.** Blocked on `WIZ-001`: the skip is the
safety floor and this round replaces the stub it proved.

`RFC 020` §1's sketch, built. **Two questions, two statements, and a
file that says what the statements said.**

---

## §1 — how many questions: the one thing the owner must answer first

**`ROADMAP.md`'s authorized row says *"asks five questions"*.
`RFC 020` §1's own sketch asks two.** Both are in the record and they
disagree.

Measured, so the choice is made on numbers:

| | Count |
|---|---|
| Variables `Config::from_env` reads | **4** — `DATABASE_URL`, `JWT_SECRET`, `BIND_ADDR`, `COOKIE_SECURE` |
| Variables `docs/getting-started/configuration.md` documents | **5** — those four plus `RUST_LOG` |
| Variables the workspace actually reads | **12** — the five plus **seven** `SMTP_*` |

So *"five"* is the **documented** set, not the configuration surface,
and the surface is twelve.

**The architect recommends two questions**, which is `RFC 020`'s own
sketch:

1. *Where should the database live?* — `[data/app.db]`
2. *What address should it listen on?* — `[127.0.0.1:3000]`, with the
   note that enter keeps it private to this machine

and **two statements**, not questions:

- the signing key was generated, and where it is, and to back it up
  with the database
- the `.env` was written, and where

**Why not five.** `JWT_SECRET` is generated, not chosen — asking
would invite a weak answer. `COOKIE_SECURE` is correct at its default
and wrong to ask about: answering it requires knowing whether a
reverse proxy terminates TLS, which the reader this wizard exists for
does not yet know. `RUST_LOG` is an observability knob with a working
default. Asking all five makes it the *"configuration UI"* `RFC 020`
§3 forbids, and asking about TLS termination on a first run is the
opposite of what this release is for.

**`ROADMAP.md`'s own rule**: *"a row that changes after authorization
is re-authorized, not quietly edited."* So the row's *"five
questions"* is the owner's to confirm or override. The rest of this
handoff is written for two.

**Do not build the question list until the answer arrives — but do
not idle either.** `§2`'s writer is independent of how many questions
feed it: `env_file(&Answers) -> String`, the round trip through
`dotenvy` to `Config`, and the planted print/write disagreement can
all be built and proven against an `Answers` carrying the four
variables, whatever the wizard ends up asking. Start there, and add
the prompts last.

## §2 — the writer is a pure function, and that is not a style choice

**`.env` is behind a standing deny rule on file tools** — nobody in
this pipeline can open one to check what the wizard wrote.

So: **`fn env_file(answers: &Answers) -> String`**, pure, returning
the file's whole text, with the write a thin shell around it. Then
the content is tested as a string by ordinary unit tests, and no test,
reviewer or agent ever needs to read a `.env` back.

**What the tests must assert about that string**, because this is
where `RFC 020`'s central claim lives — *"it writes the `.env` it
describes, so the file and the explanation can never disagree"*:

- every variable the wizard printed a value for appears in the file
  with **that same value**
- the file is loadable by the same path that reads it — `dotenvy`,
  per `config.rs:28` — so a round trip through `Config::from_env`
  yields the answers given
- **nothing is written that the wizard did not say.** A line the
  operator was not shown is the defect this RFC exists to close,
  pointing the other way.

**The round trip is the test that matters**: answers → string →
`dotenvy` → `Config` → the same answers. Write it so it fails if a
key is renamed on either side.

## §3 — write, and say the path

**`RFC 020` §5's first open question — write `.env`, or print what to
write?** Answered: **write, and say the path.** The RFC's own leaning,
and the only option consistent with its central claim: a printed
block the operator pastes can be pasted wrong, and then the file and
the explanation disagree again, which is the class this comes from.

**Then the last line of the wizard is the path**, as the sketch has
it — *"Written to `.env` — edit it any time, or delete it to start
over."* That sentence is also the answer to *"how do I change this?"*,
which is the question a wizard usually leaves unanswered.

## §4 — what must not happen

- **No second screen.** `RFC 020` §3: five variables with defaults,
  once. A wizard that grows a step is a different proposal.
- **No SMTP.** Seven variables, all optional, and the startup log
  already warns when the email channel is unconfigured. Asking about
  mail on a first run is the configuration UI again.
- **No change to the default start.** `peisear` with a valid `.env`
  or a set environment starts exactly as today, silently. State the
  before/after output.
- **No weakening of the refusals.** `config.rs`'s `EmptySecret` and
  `PublishedPlaceholder` refusals (`FIV-001`) stay exactly as they
  are. The wizard is an additional path to a good configuration, not
  a replacement for refusing a bad one.
- **Never write into the repository.** Temp directories in every
  test; `.gitignore`'s `.env*` is the backstop, not the plan.

## §5 — gates

- The round-trip test of §2, and **a test that fails if the wizard
  prints a value it does not write** — plant the disagreement and
  watch it go red. That is this release's own subject; a guard for it
  that has only been seen green proves nothing.
- `WIZ-001`'s four skip cases **still green**, unchanged, with the
  stub replaced by the real body.
- The default start unchanged, output quoted before and after.
- `DEC-007` delta stated.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`,
  `publish-dry-run`, CI; `test.yml`'s own gate if touched.

## §6 — escalate rather than deciding

- **The question count is the owner's** (§1). Do not pick it.
- **If a default turns out to be wrong** for a first run — if
  `127.0.0.1:3000` or `data/app.db` is not what a newcomer should
  get. That is a requirement question and `RFC 017` §5 chose the
  loopback bind deliberately.
- **If writing `.env` collides with `dotenvy`'s own search path** in
  a way that makes the written file not the file that gets read.
  Bring it; that would falsify §3's answer.

## §7 — exit condition

Two questions and two statements per §1, after the owner has settled
the count; `env_file` a pure function with the write a thin shell;
the round trip answers → string → `dotenvy` → `Config` asserted, and
a planted print/write disagreement seen red; nothing written that the
wizard did not say; `WIZ-001`'s four skip cases still green with the
real body in place; the default start unchanged with its output
quoted; the `FIV-001` refusals untouched; no test writing into the
repository; `DEC-007` reconciled; all gates re-run after the last
edit.
