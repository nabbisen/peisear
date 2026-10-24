# WIZ-003 — what the documents are for now

**0.58.0, round 3 of 3.** Blocked on `WIZ-002`: this round describes
what shipped, and cannot be written before it has.

Two jobs. One is re-pointing three documents. The other is the
question `RFC 020` asked of itself and told us to answer honestly in
review — **and the answer may not be the flattering one.**

---

## §1 — the claim `RFC 020` demands be tested

`RFC 020` §2, verbatim:

> *"**That is the claim to test before accepting this**, and it is not
> obvious. A project that answers a documentation defect by building a
> feature has chosen the more expensive repair. State honestly in
> review whether the three defects above would have been *prevented*
> by a wizard, or merely moved."*

The three defects it means, from its own Summary:

| Defect | What it was |
|---|---|
| `§10.40` | `.env.example` installed a placeholder nobody was told to replace |
| `FIV-004` §1b / `FIV-001` §11b | documentation described a tarball that did not exist, and an output line the binary no longer printed |
| `§10.41` | an install command that produced an application with no assets |

**Answer each one with *prevented* or *moved*, and say why.** This is
not a formality and the honest answer is probably mixed:

- A wizard that **writes** the configuration cannot install a
  placeholder — nothing copies a file with one in it. That reads as
  *prevented*.
- A wizard does not build a tarball and does not print the release
  notes' output line. Those defects were in documentation describing
  an artefact, and a wizard describes nothing about artefacts. That
  reads as *moved at best, untouched more likely*.
- An install command producing an application with no assets is a
  packaging defect. A wizard runs after install and cannot see it.
  *Untouched.*

**If that is the answer — one of three prevented — say so plainly in
the release notes.** `RFC 020` §4 already names the cheaper
alternative and says a project whose binding constraint is
maintenance *"should be able to say no to it."* The RFC was accepted
anyway; what it asked for is the honest accounting, not a
justification. **A release that claims three when it prevented one is
the defect class this whole project keeps closing.**

## §2 — `.env.example` survives, and that is measured not assumed

`RFC 020` §5's third open question: *"Does `.env.example` survive at
all once a wizard writes the real thing? It is currently referenced
by three documents and one scan."*

**It is referenced by ten live places, not four.** Counted over
tracked files, excluding closed handoffs and changelog history:

| Reference | What it is |
|---|---|
| `crates/peisear-web/src/config.rs` | **`PUBLISHED_PLACEHOLDER`** — the refusal that rejects the exact published string. A **security** behaviour |
| `crates/peisear-web/src/config/tests.rs` | its tests |
| `crates/peisear-web/src/docs_scan.rs` | **rule 7** — every uncommented key against `config.rs`'s own default |
| `.github/workflows/env-example-walk.yml` | a **dedicated workflow**, the copy-and-run walk |
| `.github/CONTRIBUTING.md`, `README.md` | contributor and reader entry points |
| `docs/getting-started/configuration.md`, `docs/architecture/workspace-layout.md` | documentation |
| `docs/specification/requirements.md`, `docs/specification/external-design.md` | both specifications |

**So the answer is: it survives, and the question is closed.**
Removing it would delete a security refusal, a scan rule, a CI
workflow and two specification references in a release whose subject
is a first-run prompt. That is not a tidy-up; it is a separate
proposal.

**Record the answer where the question was asked** — `RFC 020` is an
accepted RFC and its open questions should not stay open once
answered. Append the resolution; do not edit the question away
(`§10.47`'s convention).

## §3 — re-point the three documents

`RFC 020` §2: a wizard *"does not delete those … but it changes what
they are for"* — reference for the operator who wants it, rather than
a prerequisite for the newcomer who does not know they need it.

- **`docs/getting-started/configuration.md`** (55 lines, five
  variables): keep the reference table. Add, at the top, that a first
  run with no configuration writes this file for you and that this
  page is the reference for changing it afterwards. **The table stays
  — an operator writing a systemd unit must not have to run an
  interactive program to learn the variable names**, which is
  `RFC 020` §2's own argument.
- **`docs/getting-started/first-run.md`**: it explains what a
  successful start looks like. After `WIZ-002` the first start looks
  different. Rewrite it to match what the binary now prints, and
  **check it against the real output** rather than against the RFC's
  sketch — the sketch is explicitly *"a sketch, not a specification"*.
- **`README.md`**: its Quick Start is the sentence a newcomer reads
  first. If it still says to copy a file, it now says something the
  product has made unnecessary.

**And the seven `SMTP_*` variables are documented nowhere this round
surveyed.** `configuration.md` lists five; the workspace reads twelve.
That gap is **not this round's to fix** — it is a finding, and it goes
to the architect with the measurement rather than being absorbed into
a documentation pass about something else.

## §4 — gates

- `mdbook build` + linkcheck, and **every changed page read back as a
  newcomer**, not as a diff — `DEC-028` pass 5's own discipline.
- **`first-run.md` checked against the binary's real output**, with
  the output quoted in the round's report.
- `docs_scan` and `changelog_scan` green; **`spec_scan` green** — new
  since 0.57.0 and part of the sweep now.
- `DEC-007` unchanged unless a test is added; say which.
- No `.env*` file opened by a tool call anywhere in this round.

## §5 — escalate rather than deciding

- **If the answer to §1 is that the wizard prevented none of the
  three.** That is a legitimate finding and it belongs in the release
  notes. Bring it; it does not stop the release, and `RFC 020`
  anticipated it.
- **The SMTP documentation gap** (§3) — report it, do not take it.
- **If re-pointing `configuration.md` makes it read as two documents
  arguing** — a page that is both a prerequisite and a reference. Say
  so; that is a sign the re-pointing needs the page restructured, and
  restructuring is not in this round.

## §6 — exit condition

Each of the three defects in §1 answered *prevented* / *moved* /
*untouched* with its reason, and the accounting carried into the
release notes rather than only this report; `RFC 020` §5's three open
questions all resolved in the RFC itself by appending, with question
3 answered by §2's measured ten references; `configuration.md`
re-pointed with its reference table intact, `first-run.md` rewritten
against the binary's real quoted output, `README.md`'s Quick Start
consistent with a product that configures itself; the SMTP gap
reported and not absorbed; `mdbook`, `docs_scan`, `changelog_scan`
and `spec_scan` green; no `.env*` opened by a tool call; `DEC-007`
reconciled; all gates re-run after the last edit.
