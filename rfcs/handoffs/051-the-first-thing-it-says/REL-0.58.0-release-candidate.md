# REL-0.58.0 — release candidate

**The series' last item.** Three rounds closed — `WIZ-001` (the skip),
`WIZ-002` (the wizard, four rounds), `WIZ-003` (what the documents are
for now).

`RFC 020` is built: **a first run with no configuration asks, writes
the file it describes, and is skipped entirely where a prompt would be
an outage.**

---

## §0 — one thing gates this round, and it is not a gate you can clear

**The owner has not settled the question count.** The implementation
asks **two** questions and prints **three** statements;
`ROADMAP.md`'s authorized row says the wizard *"asks five
questions"*.

`ROADMAP.md`'s own rule: *"a row that changes after authorization is
re-authorized, not quietly edited."*

**Do not start this round until the series README's State column says
the answer has arrived.** Do not edit the row to match the code, and
do not change the code to five questions on this handoff's word —
`WIZ-002` §1a argues two questions plus three statements on the
owner's own philosophy, and that argument is the proposal, not the
decision.

**`WIZ-002` round 3's report claims an answer that was never given.**
Round 4 corrected the record; this round must not re-import the
claim.

## §1 — the ordinary cut

1. **Version** — `0.58.0` in `[workspace.package]`, nowhere twice.
2. **`CHANGELOG.md`** — a `[0.58.0]` section, dated. Confirm
   `[Unreleased]` is empty first, by reading it.
3. **`ROADMAP.md`** — `## What ships next` moves on; the shipped line
   is written, **with the same caveat `0.56.0` and `0.57.0` both
   carried** while `DEC-056`'s legs were pending. `0.58.0`'s row
   comes out; what becomes next is whatever the table then shows.
4. **Release cycle exit criteria 1–8**, each stated with what
   satisfied it. **There is no criterion 9.**

## §2 — the claim, and the accounting `RFC 020` demanded of itself

**The claim**: the product says where its data is and whether it is
reachable, instead of a document asking the reader to find out — and
it says nothing at all where a prompt would hang a container.

**`RFC 020` §2 asked whether a wizard would have *prevented* the
three defects that motivated it or merely *moved* them, and told us
to answer honestly in review. `WIZ-003` did. The answer is one of
three, and the notes say so in these words:**

> *`RFC 020` asked whether a wizard would have **prevented** the three
> defects that motivated it or merely moved them. One of three.
> `§10.40` — a copied placeholder nobody was told to replace — could
> not have happened on a wizard path, because nothing on it copies
> `.env.example` at all; **that defect was already closed in 0.53.0**
> by the startup refusal and by commenting out every line of that
> file, so what this release adds is that the copy step is no longer
> needed, not a fix for something open. The other two it does not
> touch: documentation drifting from the binary's own output
> (`FIV-004` §1b, `FIV-001` §11b) is a prose/code gap, and `§10.41`
> was a packaging defect a wizard runs too late to see. This
> release's own housekeeping fixed the current instance of the first
> by reading the page against the real binary — the same means its
> original fix used, not anything the wizard does.*

**Do not soften that paragraph.** `RFC 020` §4 names the cheaper
alternative and says a project whose binding constraint is
maintenance *"should be able to say no to it."* The RFC was accepted
anyway; what it asked for is the accounting, not a justification.

**The claim to falsify**: *a configured instance is untouched.*
`peisear` with a `.env`, or with any of the four variables set,
starts exactly as before, silently. **Prove it** — the piped and
configured CI cases already assert it; quote the startup lines before
and after, and state that `WIZ-001`'s four skip cases are green with
the real body in place.

## §3 — what this release must not claim

- **Not that the documentation is now unnecessary.** `RFC 020` §2's
  own argument is that the pages change what they are *for* —
  reference for the operator writing a systemd unit, not a
  prerequisite for the newcomer. `configuration.md`'s table is
  intact and `.env.example` survives at ten live references.
- **Not that five questions were asked.** Two were. See §0.
- **Not that the wizard is reachable in a container.** It is
  deliberately unreachable there, and that is the release's safety
  property, proven by four CI cases including a real pty.

## §4 — `DEC-028`: both specifications, five passes

- **Pass 5 greps for the instruction, not the document.**
- **Appendix A recomputed from its own rows** — and since 0.57.0 this
  is read from rule (a) and rule (b) rather than re-added. Run them;
  do not re-derive by hand.
- `docs_scan`, `changelog_scan` and **`spec_scan`** green.
- `requirements.md`: does anything this release shipped move a
  `*Status*` line? A first-run wizard is new behaviour — check
  whether any `FR-*`/`NFR-*` entry should name it, and **if none
  should, say so rather than leaving the question unasked.**

## §5 — the gate table, re-measured against the candidate commit

- **`DEC-007` — 579** at `e91462f`; re-measure at the candidate.
  `549` at `0.57.0`, `+6` `WIZ-001`, `+3`/`+3`/`+11`/`+7` across
  `WIZ-002`'s four rounds, `+0` `WIZ-003`.
- `peisear-web --lib` — **185**; `wizard` — **20**.
- **`WIZ-001`'s four skip cases green**, and one of them seen red by
  a planted failure at least once this release — it was, twice:
  `WIZ-001` round 1's predicate plant and round 2's `listening`
  plant. Cite them; do not re-plant.
- `fmt`; `clippy --workspace --all-targets`, both profiles; `clippy`
  under `--features postgres` for all four crates that carry it;
  `rustdoc`; `mdbook` + linkcheck; `publish-dry-run`.
- CI on the candidate: all 47 jobs. **Quote the run's own conclusion
  and its non-success count; do not sum the jobs.**

## §6 — what this release does not prove, stated not implied

- **Seven `SMTP_*` variables are documented nowhere operator-facing**
  — only in `peisear-notify`'s own rustdoc. Measured in `WIZ-003`
  §4, recorded on `ROADMAP.md`, not fixed here.
- **The wizard is proven on a GitHub runner's pty, not in the
  published container image.** `WIZ-001` said a difference there is
  the case that matters most and that a runner cannot exercise it.
  Say so.
- **Two questions cover two of the four variables `Config::from_env`
  reads.** `JWT_SECRET` is generated and `COOKIE_SECURE` is a
  commented line with a sentence — deliberate, and not the same as
  configuring them.
- **`§10.36`'s instance 6 and `docs_scan` rule 4** were found and
  fixed inside this release by a round whose own refactor would have
  silenced the rule. That belongs in the notes: it is the register
  paying for itself.

## §7 — escalate rather than deciding

- **If the owner's answer is five questions.** Stop. That is a
  different wizard and a different round, not a cut.
- **If a configured instance's startup output differs at all** from
  before this release. Stop; §2's claim is the release's own gate.
- **Legs 1–4 of `DEC-056` are the architect's, on the owner's
  approval.** The dev team never tags, never publishes, never creates
  a release.

## §8 — exit condition

The owner's answer recorded before anything else; version bumped
once; `[0.58.0]` written and dated, carrying §2's accounting
paragraph **unsoftened** and §6's four limits; `ROADMAP.md`'s
`## What ships next` moved with the shipped line written under the
same pending-legs caveat and exit criteria 1–8 each satisfied in
writing; §2's claim falsified by a configured instance's unchanged
startup output; `DEC-028`'s five passes with Appendix A read from its
own rules; every gate in §5 re-run after the last edit and quoted
from its own output.

**No tag. No publish. No release.** Those are four legs the architect
runs when the owner approves the candidate.
