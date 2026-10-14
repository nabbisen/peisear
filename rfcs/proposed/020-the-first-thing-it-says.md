# RFC 020: The first thing it says — guiding setup instead of documenting it

**Status**: **Proposed**, 2026-10-14. Raised by the owner while ruling on
`§10.40`: *"I don't think it is a serious issue. Just design issue: Give user
initialization wizard or warning etc."* **The warning half is already
scheduled** — `FIV-001` round 2, 0.53.0 — because it is one function and it
carries the risk mitigation. **This RFC is the other half**, and it is
deliberately not in 0.53.0.
**Target**: proposed for **0.57.0**, after `RFC 018`'s four phases. See §6.
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
- **It is skipped entirely when configuration exists**, and when the process
  has no terminal. A container, a systemd unit, or CI must never meet it.

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

**Do nothing beyond the warning.** `FIV-001` round 2's startup warning plus a
corrected `.env.example` closes the defect that raised this. Everything here
is improvement, not repair, and **a project whose maintenance budget is the
binding constraint should be able to say no to it.** The owner's words were
*"wizard or warning etc."* — the warning is the floor and it ships in 0.53.0
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

## §6 — why 0.57.0 and not sooner

`ROADMAP.md`'s *What ships next* has 0.54.0 through 0.56.0 committed to
`RFC 018`'s four phases, authorized 2026-10-14. **Inserting this would push
each of them one release later** — the precise cost `0.51.0` incurred and
that `ROADMAP.md` records. The risk this RFC addresses is already carried by
0.53.0's warning, so there is nothing here that justifies displacing the
PostgreSQL sequence.

**0.57.0 is a proposal, not a schedule.** It enters `ROADMAP.md` when the
owner accepts the RFC, per `RFC 000`, and release numbers live in that table
and nowhere else.
