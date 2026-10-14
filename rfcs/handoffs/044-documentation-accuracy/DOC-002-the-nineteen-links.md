# DOC-002 — the nineteen links

**`RFC 019`, accepted.** Every link from documentation into `.git-exclude/`
is either **lack of documentation** or a **useless link**. Nineteen sit
inside documentation; **four of them are not defects**. Close them, then
narrow rule 5 to the rule the defect was actually about and switch it on.

**Target**: 0.51.0. **Do `DOC-001` first** — it edits two of the same files.

---

## §1 — the test, applied one line at a time

> **Does the reader need what is behind this link to understand the
> statement?** If yes, the substance is missing from the document — write
> it. If no, the link is noise — cut it.

**Apply it per citation and say which answer you got.** A blanket pass over
nineteen lines is how four of them get edited wrongly.

## §2 — the six in `docs/`, already classified

| Where | Do |
|---|---|
| `requirements.md:42` | **Cut the line.** The amendment note above it already states what changed and why |
| `requirements.md:2543` | **Cut the pointer, keep the clause.** The sentence already carries what matters — the architect verified directly, and three defects were found in the verification before the answer was believed |
| `requirements.md:3531` | **Reword**: *"a scratch copy outside the repository"*. A superseded-convention note narrating the document's own history |
| `development/changelog-and-releases.md:141` | **Cut the parenthesis.** The sentence gives the reasoning the policy was kept on — 4,154 lines, verbatim and reversible, checkable by the scan |
| `specification/README.md:41` | **Keep.** This row *defines* `.git-exclude/` as the private working area, in a table about what is normative. **The document is explaining the convention, not deferring to it** |
| `specification/history/…-0.20.0-…md:24` | **Leave.** A dated snapshot; editing frozen history to remove a link falsifies it |

**If any of these four edits turns out to lose something the document
needed, stop and say so** — that is the first classification being wrong,
which is more interesting than the edit.

## §3 — the eleven in RFCs, which you classify

`rfcs/done/003`, `004`, `004a`–`004d`, `005`, `011`, `012`, `013`, `015`,
and `rfcs/accepted/014`. Most cite a reconciliation, a measurement review or
a ruling — **decisions and their reasoning**, which is documentation.

**Per citation, §1's test:**

- **If the cited reasoning changed the decision**, it belongs in the RFC.
  Write the conclusion — not the review's prose, and not the process that
  produced it — and cut the citation.
- **If it only records where the work happened**, cut it and write nothing.
  `rfcs/done/005:875` — *"Added 2026-08-16, from `CONF-001-review.md`
  §4–§6"* — is the clearest of these: the substance is in the paragraph it
  annotates.

**Report the split.** How many were gaps and how many were noise is the
measurement this round produces, and it sizes `DOC-006`.

## §4 — one that is neither

`rfcs/done/011:88` states the CDP driver *"is kept untracked at
`.git-exclude/tools/cdp.mjs`."*

**It is committed, at `browser-checks/cdp.mjs`.** That is **a false
statement in documentation**, not a link. Fix the sentence to say where the
driver is and that it is committed. **Check the paragraph around it** — a
sentence that went stale rarely went stale alone.

## §5 — the two code comments

`components.rs:130` and `changelog_scan.rs:50` cite `.git-exclude/tmp/`
paths. **`components.rs:130` is a `///` comment and reaches docs.rs**, so it
is documentation by distribution; apply §1's test to it exactly as to a
document. `changelog_scan.rs:50` is a `//` implementation comment — **not
documentation**, and out of scope unless it is simply wrong.

## §6 — then rule 5, narrowed to the rule the defect was about

**Today's rule — *no committed file cites the path* — cannot pass**, and
should not: it bans `specification/README.md:41`, the sentence that
documents the convention, and it bans handoffs, which are execution records
and not documentation.

**The rule that matches:** *no document under `docs/`, no RFC, and no
published doc comment cites `.git-exclude/`.*

**Four exclusions, each named with its reason in the code:**

1. `docs_scan.rs` itself — the rule explains the string it counts.
2. `docs/specification/README.md`'s definition row.
3. `docs/specification/history/` — frozen snapshots.
4. **`rfcs/accepted/019-what-is-in-the-repository.md`** — it *defines* this
   rule and quotes the paths it governs. **The same self-reference as (1),
   one directory over**; you will hit it the moment the rule runs.

**Then remove the `#[ignore]` and let it run live.**

**If the exclusion list needs a fifth, stop and say so.** Four named
exceptions is a rule with edges; a list that keeps growing is a rule whose
subject is narrower than its name, and we have been here before (`§10.36`).

## §7 — gates

- **`DEC-007`**: rule 5 becomes live and its `#[ignore]`d enforcement test
  starts running. **State the arithmetic** — what the count was, what moved,
  what it is.
- **Rule 5 must be seen to fail.** Plant a citation in a document in scope
  and confirm it is caught; plant one in a handoff and confirm it is **not**.
  Both halves — a rule that catches everything is as wrong as one that
  catches nothing.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken), **`mdbook build`** — six documents
  change.
- **No migration.**

## §8 — escalate rather than deciding

- **If an RFC citation's reasoning is too large to distil into the RFC.**
  That is a gap bigger than a paragraph and it belongs to `DOC-006`.
- **If the exclusion list needs a fifth entry.**
- **If cutting a pointer leaves a statement that no longer stands up.**

## §9 — exit condition

Nineteen citations resolved, each with its classification stated; the false
statement in `rfcs/done/011` corrected; rule 5 narrowed to documents, RFCs
and published doc comments with four named exclusions, planted in both
directions, `#[ignore]` removed and **green**; the gap-versus-noise split
reported, because it sizes the next round.
