# RFC 0019: What belongs in documentation, and what the links to `.git-exclude/` are

**Status**: **Proposed** (2026-10-12, rewritten) — the owner's framing,
replacing two earlier drafts of mine that proposed restructuring instead of
answering
**Target**: 0.51.0
**Raised by**: the owner — *"the issue is missing links: some statements in
documentation point to some in `.git-exclude/`… the issue possibly consists
of the two: lack of documentation, useless link to `.git-exclude/`."*
**Last updated**: 2026-10-12

## §1 — the issue, in the owner's two parts

Every link from documentation into `.git-exclude/` is one of two things:

1. **Lack of documentation.** The statement's claim depends on something
   only the private file says. **The substance belongs in a document and is
   not there.**
2. **Useless link.** The document already says the substance; the pointer
   only records where it came from. **Provenance, not content.**

**Measuring the issue at the right size matters.** 137 occurrences exist
repo-wide, but **83 of the 104 files holding them are handoffs** — execution
records, not documentation. **Inside documentation as defined in §2 there
are 19**, and they are listed in §4.

## §2 — what documentation is

**The owner's list, each category with the document that owns it.** Nothing
here is new; the point is that the list is now written down, so "which
document owns this?" has an answer before a citation is written.

| Category | Owning document |
|---|---|
| Service requirements | `docs/specification/requirements.md` |
| External design | `docs/specification/external-design.md` |
| System architecture | `docs/architecture/` |
| Threat model | `docs/security/hardening.md` |
| Data | `requirements.md §6`, `external-design.md §12` |
| Networking | `external-design.md §1` (boundary), `docs/operations/deployment.md` |
| **Decisions and their reasoning** | **nothing. §3** |

**And what documentation is not** — the owner's *"we don't have to record
all of the processes; noise and temporary things should be cut off"*:

- **Not documentation**: review packages, evidence logs, gate output,
  scratch trees, measurement working, round-by-round execution narration,
  and provenance (*"added from X-review §4"* — that is what git history is
  for).
- **Handoffs are execution records, not documentation.** They may be rough;
  documentation may not depend on them.

**The test for a citation**, which is the whole of this RFC in one line:

> **Does the reader need what is behind this link to understand the
> statement?** If yes, the substance is missing from the document — write
> it. If no, the link is noise — cut it.

## §3 — decisions: written where they take effect, not collected

**Owner, 2026-10-12, correcting a false choice I offered** (a standalone
register, or a section of `requirements.md`): *"It sounds strange to me if
you asked me where to collect the project decisions. Decisions should not be
written isolated from specification or requirement."*

**That is right, and a register would have recreated the defect one
directory over** — `requirements.md` → `decisions.md` instead of
`requirements.md` → `.git-exclude/`. Still a pointer; the reader still has
to go somewhere else.

**The project's own data proves the principle:**

| Decision | Where it lives | Legible? |
|---|---|---|
| `DEC-055` | its own section in the document it governs — *"## Keeping the file a readable size — `DEC-055`"* (`changelog-and-releases.md:137`) | **Yes** |
| `DEC-056` | stated in situ — *"**A release is four steps** (`DEC-056`, 2026-09-29): tag · push the tag · …"* | **Yes** |
| `DEC-013` | `requirements.md:2212`, *"Source: implementation decision `DEC-013`, migrations `0014`, `0017`"* — **and nothing else, anywhere** | **No** |
| `DEC-049` | discussed in **four** places in `requirements.md` (`:2728`, `:2751`, `:4859`, `:4876`), each about a different clause | **No** — I reconstructed it from six citations |

**`DEC-049` is the instructive one.** It *is* in the governing document, in
situ, four times — and still unreadable, because **no single place states
it**. In situ is necessary and not sufficient.

**So the rule:**

> **A decision is stated once, in the document it governs, at a location the
> other mentions point to.** Not collected elsewhere, and not only in
> passing.

`DEC-055`'s own section heading is the project's worked example of this, and
it costs one heading.

**The counts stay the measurement of the gap**: 59 numbers assigned, 35
cited, **24 appearing in no committed file at all**, 0 that any document
states outright rather than mentions. **The 24 that nothing references may
simply be retired** — a number with no citation is not a decision anyone is
relying on.

## §4 — the 19, classified

**docs/ — six:**

| Where | Classification |
|---|---|
| `requirements.md:42` — *"Full derivation: …/004-…"* | **Useless link.** The amendment note above it already states what changed and why. Cut the line |
| `requirements.md:2543` — *"Details … in …/017-…"* | **Useless link.** The sentence already carries the part that matters: the architect verified directly, and three defects were found in the verification before the answer was believed. Cut the pointer, keep the clause |
| `requirements.md:3531` — *"pointed at a scratch copy in `.git-exclude/tmp/`"* | **Useless link.** A superseded-convention note narrating the document's own history. Reword to *"a scratch copy outside the repository"* |
| `development/changelog-and-releases.md:141` | **Useless link.** The sentence gives the reasoning it was kept on — 4,154 lines, verbatim and reversible, the scan makes it checkable. The parenthesis is provenance. Cut |
| `specification/README.md:41` | **Keep.** This row *defines* `.git-exclude/` as the private working area in a table about what is normative. The document is explaining the convention, not deferring to it — **and a blanket rule would ban the sentence that documents the rule** |
| `specification/history/…-0.20.0-…md:24` | **Leave.** A dated snapshot. Editing frozen history to remove a link would falsify it |

**RFCs — eleven, and they are where the real gaps are.** `rfcs/done/003`,
`004`, `004a`–`004d`, `005`, `011`, `012`, `013`, `015`, `accepted/014`.
Most cite a *reconciliation*, a *measurement review* or a *ruling* — that is
**decisions and their reasoning**, the category §3 has no home for.

**Per RFC, apply §2's test**: if the cited reasoning **changed the
decision**, it belongs in the RFC and the citation goes. If it only records
where the work happened, the citation goes and nothing replaces it.
`rfcs/done/005:875` — *"Added 2026-08-16, from `CONF-001-review.md` §4–§6"* —
is the clearest **useless link**: the substance is in the paragraph it
annotates.

**One is neither**: `rfcs/done/011:88` says the CDP driver *"is kept
untracked at `.git-exclude/tools/cdp.mjs`."* **It is committed, at
`browser-checks/cdp.mjs`.** That is not a link problem — it is **a false
statement in documentation**, and it should be fixed as one.

**Two code comments** (`components.rs:130`, `changelog_scan.rs:50`) cite
`.git-exclude/tmp/` paths. `components.rs:130`'s is a `///` comment and
reaches **docs.rs**, so it is documentation by distribution.

## §5 — what this makes the scan rule

**Rule 5 stops being "no committed file cites the path".** That version
bans `specification/README.md:41`, which is the sentence documenting the
convention, and it bans handoffs, which are not documentation.

**The rule that matches the issue**: *no document under `docs/`, no RFC, and
no published doc comment cites `.git-exclude/`* — with the frozen
`history/` directory and the definition row excluded by name, both for
stated reasons.

**In scope today: 19 occurrences, four of which are not defects.** That is a
rule that can pass, and it is the one the defect was actually about.

## §6 — what is not proposed

- **Reviews are not published.** They are working material and carry the
  owner's local environment.
- **Handoff directories are not restructured.** `RFC 000`'s numbering
  conflict is real and recorded; it is not this issue.
- **No new process.** §2's table and §3's register are documents, not
  procedure.

## §7 — nothing required from the owner

§3's question is withdrawn; the owner answered it by rejecting its premise.
What remains is editing nineteen lines against §2's test, and giving each
decision a document cites one stated location in that document — both mine
to schedule, as rounds of 0.51.0.
