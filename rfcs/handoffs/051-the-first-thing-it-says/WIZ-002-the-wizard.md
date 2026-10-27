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

and **three statements**, not questions:

- the signing key was generated, and where it is, and to back it up
  with the database
- **that cookies are not marked `Secure`, and to set
  `COOKIE_SECURE=1` when peisear is served over HTTPS** — see §1a,
  which is a correction to this handoff's first version
- the `.env` was written, and where

**Why not five questions.** `JWT_SECRET` is generated, not chosen —
asking would invite a weak answer. `RUST_LOG` is an observability
knob with a working default and no wrong setting. And
`COOKIE_SECURE` is not a question either, for the reason §1a gives —
but it is **not silent**, which is where this handoff was wrong.
Asking all five makes it the *"configuration UI"* `RFC 020` §3
forbids; saying three things costs one sentence and no question.

## §1a — the correction: a silent security default is not a default

**This handoff first said `COOKIE_SECURE` is *"correct at its default
and wrong to ask about"* and left it out entirely.** The first half
is true and the conclusion was wrong.

`COOKIE_SECURE` sets the `Secure` flag on the session cookie
(`config.rs:41` → `state.rs:9` → `handlers/auth.rs:72`/`:116`). At
its default of `false` the cookie is **sent over plain HTTP**.
Correct for a first run on `127.0.0.1`; wrong for every deployment
behind TLS, which is every real one.

**And the wizard writes a file the operator keeps.** So the version
this handoff first specified would configure an instance, write two
lines, describe two lines — and say nothing at all about a security
control being off. The file and the explanation would agree, and both
would be silent. **That is `RFC 020`'s own claim satisfied in letter
and defeated in substance**: the operator still has to read
`configuration.md` to discover the flag exists, which is the *"read
the documentation"* step this release exists to remove.

**The fix uses a convention this project already chose, for this
exact reason.** `requirements.md`'s own `0.53.0` amendment:
*"every variable in `.env.example` is now commented out, each shown
at its own built-in default, so a copied file changes nothing and can
never hand anyone a setting they did not choose."*

So `env_file` writes it **commented, at its built-in default**:

```
DATABASE_URL=…
BIND_ADDR=…
# COOKIE_SECURE=0   # set to 1 when peisear is served over HTTPS
```

Visible and editable, **without pinning** a default a future release
might improve — which is precisely why 0.53.0 chose commenting over
writing values.

**Word the statement unconditionally.** Not *"because this instance
listens on loopback"* — the operator may have just changed
`BIND_ADDR` to `0.0.0.0`, and then that clause is false. *"Cookies
are not marked `Secure`. Set `COOKIE_SECURE=1` when peisear is served
over HTTPS."* is true whatever they answered.

**This changes one accepted test.** Round 1's *"nothing else is
written — exactly two lines"* becomes *"exactly two uncommented
lines, plus the one commented `COOKIE_SECURE` line"*. Update it
deliberately and say so; a test loosened quietly is worse than the
gap it was hiding.

**`COOKIE_SECURE` was the only silent one, and that is measured.**
`DATABASE_URL` is asked; `BIND_ADDR` is asked and the sketch already
carries its own security sentence (*"press enter to keep it private
to this machine"*); `JWT_SECRET` is stated. Of the four variables
`Config::from_env` reads, this was the one with a security-relevant
default and no sentence anywhere in the wizard.

**`ROADMAP.md`'s own rule**: *"a row that changes after authorization
is re-authorized, not quietly edited."* So the row's *"five
questions"* is the owner's to confirm or override. The rest of this
handoff is written for two.

**Answered 2026-10-28: two questions plus three statements, as
recommended.** The owner accepted it after asking three times for this
to be reviewed against their own philosophy, and each of those three
reviews found a real defect — `§1a`'s silent `COOKIE_SECURE`,
`WIZ-004`'s ancestor-`.env`, and `WIZ-005`'s unvalidated address. The
count was never the flaw; **asking for the review was what found
them.** `ROADMAP.md`'s row is re-authorized to match.

**The rest of this section is the history of that question, kept
because round 3's report was wrong about it.** That report opens *"The owner answered `§1`: two
questions, three statements."* The owner's actual words were *"Your
recommendation is almost approved. However, review it carefully with
my fundamental philosophy beforehand"* — an instruction to find a
flaw, which produced `§1a`'s correction, and **the corrected
recommendation has not been approved either.**

Part of this is the architect's: the reply to the owner ended *"the
ask is now: two questions and three statements"*, and `§1a` reads as
a specification rather than a proposal.

**What follows from it**: `WIZ-003` and `REL-0.58.0` must not cite an
owner answer that does not exist, and **`REL-0.58.0` cannot cut until
the owner confirms the count or re-authorizes the row** —
`ROADMAP.md`'s own rule, which is why this was an ask. `§10.34`'s
family: a status asserted ahead of the thing it describes.

**Do not build the question list until the answer arrives — but do
not idle either.** `§2`'s writer is independent of how many questions
feed it: `env_file(&Answers) -> String`, the round trip through
`dotenvy` to `Config`, and the planted print/write disagreement can
all be built and proven against an `Answers` carrying the four
variables, whatever the wizard ends up asking. Start there, and add
the prompts last.

## §1a — round 2, first and unblocked: `docs_scan`'s rule 4 goes silent

Round 1 tried sharing `DATABASE_URL`/`BIND_ADDR` constants between
`config.rs` and `wizard.rs` — the better code, since a rename either
side would become a compile error. **It broke `docs_scan` rule 4, and
the rule did not say so.**

`literal_string_default` anchors on the literal text
`env::var("<VAR>")` and returns `None` when it is not found; its
caller is `if let (Some(actual), Some(documented)) = (…) && actual !=
documented`, so **`None` pushes no violation.** Three variables share
that path — `DATABASE_URL`, `BIND_ADDR`, and `COOKIE_SECURE` through
`literal_bool_default`. A refactor that changes the call shape leaves
rule 4 reporting zero while checking nothing.

**`§10.36`'s class**, which is **open** in the register and framed as
*"a question to ask of every green check"*. Asked here of a check
nobody had asked it of.

**Fix it by asserting the population**, the move `spec_scan` made in
0.57.0: a lookup rule 4 expects to succeed returning `None` is itself
a violation —

```
rule 4: no literal default found in config.rs for DATABASE_URL --
has the call shape changed?
```

**Planted through `docs_scan`'s own `Input`** — change the call shape
in the input, watch rule 4 go red rather than quiet. No file on disk
is touched; that is what `Input` is for.

**Record the instance under `§10.36`**, not a new `§10.N` and not
`§10.39`, which is closed. Say in it that the refactor the round
wanted was *correct*, and is blocked only by a guard that reads
source text — **a cost of text-reading guards worth recording**, not
a reason to avoid the refactor forever.

**This does not wait on the owner. Do it first.**

## §1b — round 4: validate the answer, do not prefix it blindly

`wizard.rs:129` prefixes the database answer unconditionally:
`format!("sqlite://{}", …)`. **The prefixing is right** — the prompt
shows a bare path and the file must carry the `sqlite://` form
`Config::from_env` reads. **Doing it unconditionally is not.**
Measured against the real binary:

| Answer | Becomes | What happens |
|---|---|---|
| `sqlite://data/app.db` — **the documented form** | `sqlite://sqlite://data/app.db` | creates a directory named `sqlite:`, **writes a generated signing key to `./sqlite:/data/jwt_secret`**, then fails: `unable to open database file (code: 14)` |
| `:memory:` | `sqlite://:memory:` | refuses at startup — no filesystem path to persist a key beside |
| `postgres://u:p@host/db` | `sqlite://postgres://…` | same class as the first |

**The first row is the likely one.** Anyone who has read
`configuration.md` knows the variable takes `sqlite://…`, and a
prompt showing a bare path invites pasting the form they know. The
result is not a clean refusal: **a real key is persisted where the
operator will never look** — while `backup.md` tells them it lives
beside the database — and then the server exits.

**`:memory:` is the same defect, and round 3 treated its symptom**:
omitting the signing-key statement is graceful, but the `.env` it
still writes cannot boot.

**The fix, one step before the answer becomes an `Answers`:**

1. **If the answer already contains `://`, take it as given** rather
   than prefixing — and **refuse a scheme this build cannot use**,
   naming it. `pool::connect`'s own deny-list already refuses
   `postgres://` on a SQLite build, so this restates an existing
   refusal rather than inventing a rule.
2. Otherwise prefix it, as today.
3. **Either way, if the result has no filesystem path for the key**
   (`config::database_file_path` → `None`), **say so and ask
   again.** The predicate exists and this round already exposed a
   wrapper around it.

**Re-ask; do not refuse the run.** The one thing a wizard can do
that a startup error cannot is let the operator correct the answer
while they are still sitting there.

**Never write an unusable `DATABASE_URL` into `.env`.** The key is
generated downstream in `Config::from_env`, so the only place to stop
it is here.

**Plant all three shapes in-memory.** `run_wizard` is already generic
over `BufRead`/`Write`: feed the answer, assert the wizard re-asks
and names the reason. No filesystem.

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
