# RFC 0019: What is in the repository — key information, and where handoffs live

**Status**: **Proposed** (2026-10-12) — one decision is the owner's (§7)
**Target**: 0.51.0, replacing `DOC-002`'s sketch
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

## §5 — Proposal

**One principle, stated once**: *a published document may defer only to
another published document.*

1. **`.git-exclude/reviewed/` → `reviews/`** in the repository, prose only.
   ~60 documents, a few hundred KB. The citations become resolvable links.
2. **The cited architect derivations → `docs/specification/derivations/`**,
   or the citations go. A normative requirement may not defer to a file
   nobody can open.
3. **`.git-exclude/rules/` → `docs/development/project-rules.md`**, as the
   project's own standards, with `CONTRIBUTING.md` linking to it. **And the
   two copies of the lifecycle policy become one.**
4. **Review-request READMEs → committed with the round's handoff**; evidence
   logs stay out. The README is the account; the logs are its working.
5. **Handoffs: resolve §2 deliberately.** Either the project's copy of the
   policy gains a *round handoff* alongside the RFC companion, or
   non-RFC handoffs move out of `rfcs/` to a sibling `handoffs/`.
   **Recommend the former** — the directories are cited from 83 files and
   the policy is ours to amend, where the citations are not ours to break
   cheaply.
6. **Then rule 5 stands exactly as written**, no carve-out: no committed
   file cites a gitignored path, because nothing key is gitignored. What
   remains behind it is scratch, evidence and the owner's private inputs.

**Sequencing**: 1–4 are mechanical and independent; 5 is a policy amendment;
6 is one `#[ignore]` removed and should land last, as the proof the rest
worked.

## §6 — what this costs, stated plainly

- **Every review becomes public**, including my criticisms of the dev team's
  work and my own reversals. **That is the point of the change and also its
  price**, and it is the owner's to accept.
- **Repository size** grows by a few hundred KB of prose. Evidence stays
  out, which is where the bulk is.
- **~137 citations get rewritten** once, mechanically.

## §7 — decision required

**Does the project's working record become part of the repository?** The
owner's own sentence — *key information should not be in `.git-exclude/`* —
answers it in principle; §5 is what it costs in practice, and §6 is the one
consequence worth saying out loud before it is irreversible.

Everything else in §5 follows from a yes and is mine to schedule.
