# RFC 020: The first thing it says — guiding setup instead of documenting it

**Status**: **Accepted** (owner, 2026-10-15). Raised by the owner while ruling on
`§10.40`: *"I don't think it is a serious issue. Just design issue: Give user
initialization wizard or warning etc."* **The refusal half is already
scheduled** — `FIV-001` round 2, 0.53.0: the published placeholder is
rejected at startup with a named remedy. **This RFC is the other half**, and
it is deliberately not in 0.53.0.
**Target**: **0.58.0**. §6 proposed 0.57.0 on the reasoning that this must
not displace `RFC 018`'s phases; that reasoning held and then applied again
to the guard row the owner authorized at 0.57.0 the same week, so this moved
one further. **`ROADMAP.md`'s *What ships next* is the number's home** — §6 is
left as it was written rather than back-edited, so the reasoning stays
readable and the number is read from one place.
**Raised by**: the owner, 2026-10-14.
**Related requirements**: `NFR-CMP-002` (self-hostable), `NFR-SEC-004`
(no secrets in repository or release artefact), `NFR-REL-007`
(documentation structure)
**Last updated**: 2026-10-14

## Summary

peisear's configuration story is *read the documentation, copy a file, edit
it*. Every one of those three steps is a place to stop, and the project has
now found defects in all three: a copied file that installs a placeholder
nobody is told to replace (`§10.40`), documentation that described a tarball
that did not exist and an output line the binary no longer prints (`FIV-004`
§1b, `FIV-001` §11b), and an install command that produces an application
with no assets (`§10.41`).

**Each of those was fixed by correcting a document.** This RFC asks a
different question: *how much of the documentation exists because the product
does not speak?*

## §1 — the shape

**On a start with no configuration, peisear asks.** Not a separate
`peisear init` subcommand the reader must first learn exists — the first run
*is* the wizard, because the reader who needs it is the reader who has not
read anything.

A sketch, not a specification:

```
peisear
  No configuration found. Setting up a new instance.

  Where should the database live?  [data/app.db]
  What address should it listen on?  [127.0.0.1:3000]
    (press enter to keep it private to this machine)

  A signing key has been generated and saved to data/jwt_secret.
  Back it up with the database.

  Written to .env — edit it any time, or delete it to start over.
  Starting on http://localhost:3000
```

**Three properties it must have:**

- **Every question has a working default**, so holding enter is a complete
  answer. The wizard's job is to *show* the decisions, not to require them.
- **It writes the `.env` it describes**, so the file and the explanation can
  never disagree — which is the class of defect this RFC comes from.
- **It is skipped entirely when configuration exists**, and whenever no one
  can answer it. A container, a systemd unit, or CI must never meet it.
  *Amended 2026-10-28 (§7): this property originally read "when the process
  has no terminal", which named one mechanism for "no one can answer" and was
  implemented as the whole of it. A background job has a terminal and no
  reader; 0.58.0 stopped dead there. The property is the requirement; "no
  terminal" is only one of its cases.*

## §2 — what it would replace, which is the actual argument

`docs/getting-started/configuration.md` is a 48-line page whose job is to
explain five environment variables. `first-run.md` explains what a successful
start looks like. `.env.example` is a file whose only purpose is to be copied
and edited.

**A wizard does not delete those** — an operator writing a systemd unit still
needs the reference table, and should not have to run an interactive program
to find out what the variables are called. **But it changes what they are
for**: reference for the operator who wants it, rather than a prerequisite
for the newcomer who does not know they need it.

**That is the claim to test before accepting this**, and it is not obvious.
A project that answers a documentation defect by building a feature has
chosen the more expensive repair. State honestly in review whether the three
defects above would have been *prevented* by a wizard, or merely moved.

## §3 — what this must not become

- **Not a configuration UI.** Five variables, with defaults, once. Anything
  that grows a second screen is a different proposal.
- **Not a required step.** `peisear` with a valid `.env`, or with the
  environment set, starts exactly as it does today, silently.
- **Not a reason to keep `.env.example`'s placeholder.** That line comes out
  regardless; this RFC does not depend on it and must not be cited to defer it.
- **Not a prompt in a container.** Detect a non-interactive stdin and use
  defaults. A wizard that blocks a `docker run` is an outage.

## §4 — the alternative, named so it is not rediscovered

**Do nothing beyond the refusal.** `FIV-001` round 2's startup refusal plus a
corrected `.env.example` closes the defect that raised this. Everything here
is improvement, not repair, and **a project whose maintenance budget is the
binding constraint should be able to say no to it.** The owner's words were
*"wizard or warning etc."* — the refusal is the floor and it ships in 0.53.0
either way.

If this RFC is declined, the thing worth keeping is §1's third property as a
rule: **the product should say, on a start a human is watching, where its
data is and whether it is reachable from the network.** That is two lines of
output and most of the value.

## §5 — open questions

1. **Does it write `.env`, or does it print what to write?** Writing is
   kinder and makes the file authoritative; printing never surprises anyone
   by creating a file. *Leaning: write, and say the path.*
2. **What happens on the second run after a partial answer?** A `.env` that
   exists but is missing a variable is the state the wizard would create if
   interrupted.
3. **Does `.env.example` survive at all** once a wizard writes the real
   thing? It is currently referenced by three documents and one scan.

**Resolved (`WIZ-003`, 0.58.0)** — all three, by what shipped rather
than by further proposal:

1. **Write, and say the path.** `WIZ-002` built `wizard::run_wizard`
   to ask, `wizard::write_env_file` to write, and `main.rs` prints the
   written path as part of the three statements. The leaning held.
2. **Existence, not completeness.** `wizard::configuration_exists`
   (`WIZ-001`) treats a `.env` that exists — regardless of which of
   the four variables it sets — as a configured instance, so a
   partial answer is never re-asked. `Config::from_env`'s own
   defaults already make a partial file a working one; asking again
   would surprise an operator who has already configured the
   instance.
3. **It survives, measured at ten live references, not three.**
   `config.rs`'s `PUBLISHED_PLACEHOLDER` refusal, `docs_scan` rule 7,
   a dedicated CI workflow (`env-example-walk.yml`), `CONTRIBUTING.md`,
   `README.md`, two getting-started pages, and both specification
   documents all still depend on it. Removing it would delete a
   security refusal, a scan rule, and a CI workflow in a release whose
   subject is a first-run prompt — a separate proposal, not a tidy-up
   this one performs.

## §6 — why 0.57.0 and not sooner

`ROADMAP.md`'s *What ships next* has 0.54.0 through 0.56.0 committed to
`RFC 018`'s four phases, authorized 2026-10-14. **Inserting this would push
each of them one release later** — the precise cost `0.51.0` incurred and
that `ROADMAP.md` records. The risk this RFC addresses is already carried by
0.53.0's refusal, so there is nothing here that justifies displacing the
PostgreSQL sequence.

**0.57.0 is a proposal, not a schedule.** It enters `ROADMAP.md` when the
owner accepts the RFC, per `RFC 000`, and release numbers live in that table
and nowhere else.

*Accepted 2026-10-15, and this section is what moved it.* 0.57.0 went to
`§10.36`'s citation guard — guard work, authorized the same day, and
0.52.0's precedent is that guard infrastructure does not queue behind a
feature. **The argument above did not change; it applied twice.** This RFC
is 0.58.0. See the header, and `ROADMAP.md` before either.

## §7 — amended 2026-10-28: the terminal nobody is reading

**`0.58.0` shipped this RFC and shipped a defect with it** (`§10.49` in
`requirements.md`, which carries the evidence). §1's third property above was
implemented as `stdin_is_terminal && !configuration_exists`. That predicate
answers *is a terminal attached*. The property needs *can anyone answer*.

The gap is not exotic. `./peisear > out.log 2>&1 &` from an interactive
shell — backgrounding, which is what `&`, `nohup` and a hand-rolled init
script all do — leaves `stdin` attached to the terminal while the process is
not the terminal's foreground job. Measured twice at tag `0.58.0`: stopped by
`SIGTTIN`, nothing listening, no `.env` written, the only trace a question in
a redirected log. On Windows the same cause cost the release a platform: the
`Release` workflow starts the binary with `Start-Process`, which creates a
fresh console nobody will ever type into, so the walkthrough timed out and the
packaged `.zip` was never attached.

**What this amendment settles, for `0.58.1`:**

1. **Where the operating system can answer the question exactly, ask it.** On
   Unix, "can I read this terminal" is precisely "am I the foreground process
   group of it" — the same condition the kernel evaluates to raise `SIGTTIN`.
   That is not a heuristic standing in for the property; it *is* the property.

2. **Where it cannot, the answer is declared, not guessed.** Windows has no
   foreground-process-group equivalent, and the signals that look promising
   are wrong in the case that matters most: a user who double-clicks
   `peisear.exe` gets a console holding only their own process, exactly like
   the `Start-Process` case, and that user *is* reading. So Windows keeps the
   `is_terminal` behaviour — a double-click and a terminal both prompt — and
   automation says so with one documented launch-time variable.

   The asymmetry is deliberate and has a reason worth stating: we use the
   exact answer where one exists and require a declaration where none does.
   The alternative — one heuristic applied everywhere for the sake of
   symmetry — would be wrong on both platforms instead of right on one.

3. **Being unable to prompt is not an error.** It means start the way a start
   with no terminal already starts: loopback, a generated signing key beside
   the database, `.env` untouched. `Config::from_env` already does exactly
   this and CI already proves it. A first run must never be *stopped* by the
   absence of a reader; it must be *unblocked* by it.

**The alternative, named so it is not rediscovered** (as §4 does for its own):
a deadline on the first prompt — wait *n* seconds, then proceed. It answers
the right question empirically and is platform-neutral, and it is still
refused: it makes correctness depend on a clock, so a person who pauses to
think about where their data should live gets a different program than a
person who types quickly. A rule nobody can predict is not a simple rule.

**What `0.58.0` got right and keeps.** A foreground first run still asks its
two questions, still writes the `.env` it describes, still makes its three
statements. Nothing in §1 is reversed; one of its properties is implemented as
the property instead of as one of its examples.
