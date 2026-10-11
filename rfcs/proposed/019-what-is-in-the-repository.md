# RFC 0019: What is in the repository — key information, and where handoffs live

**Status**: **Proposed** (2026-10-12) — **§6's original proposal withdrawn
on the owner's instruction; §5 answers the question actually asked**
**Target**: none yet. Held for consideration
**Raised by**: the owner, 2026-10-12 — *"In my mental model, handoffs and
RFCs are also published and shared. To my understanding, it is the issue on
documentation. Key information should not be in `.git-exclude/`."*
**Sources re-read in full**: `.git-exclude/rules/`'s
`project-instructions-general-common.md`, `project-instructions-rust.md`,
`project-instructions-gui-common.md`, `000-rfc-lifecycle-policy.md`
**Last updated**: 2026-10-12

## Summary

**My own framing was wrong and the owner corrected it.** I proposed narrowing
rule 5 so that *"handoffs and RFCs may cite `.git-exclude/`, published
surfaces may not."* **Handoffs and RFCs are published** — they are committed
Markdown in a public repository. Narrowing the rule that way would have
blessed the defect: a published handoff pointing a reader at a review they
cannot obtain.

**The problem is not which files may cite the path. It is that key
information lives behind it.**

Re-reading the four rule documents turned up three further things, two of
which contradict a standing instruction and one of which reverses a ruling I
made last week.

## §1 — Finding A: the key information, and how much of it there is

**137** committed occurrences of `.git-exclude` outside the scan's own file,
across **104 files**, **83 of them handoffs**. What they point at:

| Class | What it is | Key? |
|---|---|---|
| `.git-exclude/reviewed/*.md` | every architect review — the rulings, the findings, the corrections | **Yes.** Cited by name from handoffs and changelogs |
| `.git-exclude/tasks/architect/*` | derivations cited from `requirements.md:42` and `:2543` as *"Full derivation"* | **Yes.** A normative document defers to them |
| `.git-exclude/rules/*.md` | the owner's standing instructions — design principles, test organisation, release duties | **Yes. §4** |
| `.git-exclude/review-request/<ID>/README.md` | the dev team's own account of each round | **Yes** |
| `.git-exclude/review-request/<ID>/evidence/*` | gate logs, screenshots, measurement output | **No** — bulky, and the package's README states their results |
| `.git-exclude/tmp/*` | scratch build trees | **No** |
| `.git-exclude/specs/*` | the owner's source documents | **Owner's call** |

**Every "Yes" row is a published document deferring to an unpublished one.**
That is one defect with 137 symptoms, and renaming the rule does not touch
it.

## §2 — Finding B: the handoff directories contradict the lifecycle policy

`000-rfc-lifecycle-policy.md` is explicit:

> *"A handoff's status is inherited from the matching RFC number."*
> *"Every `rfcs/handoffs/NNN-slug/` directory, if handoffs are used,
> corresponds to an existing RFC number."*

**Measured: 50 handoff directories. RFC numbers run 000–018.** So
`042-documentation-that-drifted`, `043-the-storage-contract` and
`044-documentation-accuracy` each imply an RFC that does not exist, and
`042`'s own README opens *"**Not RFC-governed.**"* — a sentence the policy
has no room for.

Two smaller breaches alongside it: the numbering is a **second sequence**
that has run past the RFCs it is supposed to mirror, and the padding is
inconsistent — `0013-`, `0014-`, `0015-` beside `001-`, where the policy
fixes three digits until 999.

**This is the policy's own named anti-pattern**, *"Turning handoffs into a
second RFC lifecycle"*, arrived at by accumulation rather than by decision.

**And the practice is not wrong.** `NFR-MNT-010` requires an RFC for
*significant* change; most rounds are not significant in that sense, and
they still need an execution document. **The policy did not anticipate a
round that has no RFC.** So this is a conflict to resolve deliberately, not
a mess to tidy.

## §3 — Finding C: `NFR-MNT-005`, and a ruling of mine to reverse

At `REG-001`'s review I wrote: *"The practice is right and the requirement
is wrong… a scan that reads the source tree needs
`env!("CARGO_MANIFEST_DIR")`, and a unit test of a private helper cannot live
in `tests/`."* I recommended amending the requirement.

**`project-instructions-rust.md` already gives the shape, and it is neither
of the two I considered:**

> ⭕ **Good:** `src/some_mod.rs` and `src/some_mod/tests.rs`.
> ❌ **Bad:** `#[test]` modules placed inside `src/some_mod.rs`.

**A sibling file under `src/`.** `CARGO_MANIFEST_DIR` still resolves, private
items are still reachable, and the test code is out of the implementation
file. **The carve-out I thought was necessary does not need to exist.**

So `NFR-MNT-005` is a faithful restatement of a standing instruction, the 25
files and 79 in-file tests are a real violation of it, and **the remedy is
mechanical, not a requirement amendment**. `§10.37` should be re-read in that
direction and `DOC-004` rewritten accordingly.

**I was wrong because I reasoned from the constraint instead of reading the
rule.** The measurement was right; the conclusion drawn from it was not.

## §4 — Finding D: the rules are invisible to everyone they bind

`project-instructions-general-common.md` fixes the design principles, the
workflow, the README structure and the release duties.
`project-instructions-rust.md` fixes module style and test organisation.
`project-instructions-gui-common.md` requires i18n.

**None of them is in the repository.** A contributor reading
`CONTRIBUTING.md` cannot discover the test-organisation rule that
`NFR-MNT-005` restates, cannot discover the release audit duty, and cannot
discover that `docs/` is organised by reader persona.

**The one rule document that is published exists twice and the copies
differ**: `.git-exclude/rules/000-rfc-lifecycle-policy.md` records *"the
5-folder variant is adopted"* in its Status line; `rfcs/done/`'s copy says
only *"Implemented"*. **The authoritative sentence is in the private copy.**

## §5 — Finding E: why the documentation was not updated — the measurement

*The owner's actual question, 2026-10-12. Answered with numbers rather than
with a remedy, because the remedy is not the part that was missing.*

**There is no document that owns a decision.** Measured across every
committed file:

| | |
|---|---|
| Decision numbers assigned (`DEC-001`…`DEC-059`) | **59** |
| Cited anywhere in the repository | **35** |
| **Appearing in no committed file at all** | **24** — `DEC-002`, `-003`, `-005`, `-006`, `-009`, `-012`, `-014`, `-016`, `-017`, `-024`, `-026`, `-027`, `-029`, `-031`, `-033`…`-042` |
| Cited but appearing in no `docs/` file | **6** — `DEC-011`, `-018`, `-023`, `-025`, `-032`, `-043` |
| Files that **define** a decision rather than cite one | **0** |

**Nothing in this repository states what any of these decisions is.** They
are cited — *"per `DEC-049`"*, *"`DEC-013`'s own rule"* — and defined
nowhere. The definitions are in reviews and architect notes, which are
private, which is why the citations point there.

**Four causes, and they compound:**

1. **A ruling has no home.** `requirements.md` owns *what must be true*,
   `external-design.md` owns *what is presented*, `CHANGELOG.md` owns *what
   changed*. **A decision — "we parameterise rather than add an env var,
   because a lever that can silently disable a gate is the shape this
   project keeps finding" — belongs to none of them**, so it stays where it
   was made.
2. **The release check asks the wrong question.** The standing rule is
   *"verify all docs match changed requirements / design / specs"*, and the
   sweep's five passes implement it as *"is this document still true?"*
   **Absence is invisible to a check for contradiction.** A decision never
   written down makes no document wrong — it makes them incomplete, and
   nothing looks for incompleteness. **This is `§10.36` at the process
   level**: the sweep cannot tell *nothing wrong* from *nothing recorded*.
3. **Citing was cheaper than absorbing, and nothing charged for it.** One
   line to name a review; a paragraph and a judgement to decide which
   committed document owns the substance and write it there. **137
   citations is the accumulated bill.**
4. **The rule that would have caught it was itself private** (§4). It binds
   the architect and the dev team cannot check it was followed.

## §6 — the earlier proposal, withdrawn

**This RFC first proposed moving `.git-exclude/reviewed/` into the
repository. Withdrawn on the owner's instruction, 2026-10-12**: *"What is
important is not to store review result as-is. Respect my privacy and secret
in local environment."*

**That is right, and it sharpens §5.** A review is **working material** — it
carries local paths, machine details, and candid assessment of a round. It
is not a record. **The record is what gets extracted from it**, and the
reason the citations exist is that the extraction step was never
anybody's job.

## §7 — direction, held for consideration rather than scheduled

**Not a plan. The owner asked for consideration first, and §5 is the part
that had to be measured before anything is proposed.**

The shape the measurement points at:

- **Reviews stay private.** Unchanged.
- **The direction of reference inverts.** A committed document never defers
  to a private one. A private review may cite committed documents freely.
- **Decisions get a home** — a committed register where each `DEC-nnn` is a
  few distilled lines, not the review's prose. **That is the gap the 24
  invisible numbers measure.**
- **The sweep gains the absence question**: *what did this release decide,
  and which committed document now says it?* — distinct from all five
  existing passes, which ask only whether what is written is still true.
- **Rule 5 then becomes achievable as written**, as a consequence rather
  than as the goal.

**Nothing above is scheduled.** `DOC-002` stays blocked and the other four
0.51.0 rounds are unaffected.
