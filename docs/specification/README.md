# Specification

**The normative source of truth for peisear, in English.** Two documents:

- [**Requirements**](./requirements.md) — what the product must do, every
  requirement with an identifier, a status and the evidence for that status;
  and `§10`, the compliance register, which records what checking has found.
- [**External design**](./external-design.md) — what the product presents:
  screens, states, status codes, copy rules, and `§17`, the record of where the
  design and the implementation have diverged.

They are amended **at each release**, not afterwards (`DEC-028`), and the
release candidate does not tag until they are.

[History](./history/) holds the superseded baselines, retained unedited as the
record of their own releases rather than rewritten.

## English only, and what that settles

These two documents are the single source of truth (`DEC-020`, closed
2026-09-24; `DEC-015`). **There is no other specification, in any language.**

That decision closes a question the project carried for two months. `DEC-020`
had two halves: restore a canonical specification to `docs/`, and promote these
English baselines to normative. The first half was recorded as *blocked* on a
document nobody could produce. **There was no such document**; the block was
chasing something that did not exist, and the second half was never blocked at
all. Both are now closed by placing these files here.

## How the citations resolve

These documents cite three different kinds of thing, and only two of them are
things you can open.

| Citation | What it is | Can you open it? |
|---|---|---|
| `SPEC §11.5.3`, `KICK`, `BRIEF`, `V3`, `GUI §3.4` | **Provenance.** The source material these requirements were derived from, named so a reader can see where a rule came from | **No.** Not published, and not a source of truth — this directory is |
| `LAYOUT-003`, `PLAN-002`, `TT-004`, `CAL-003` … | Implementation handoffs | **Yes** — [`rfcs/handoffs/`](../../rfcs/handoffs/) |
| `…-review.md`, `.git-exclude/…` | The project's private working area: reviews, architect notes, scratch | **No** |

**A citation of the first kind is a label on an explanation, not a substitute
for one.** Every requirement states its own content; the tag records ancestry.
That is the same resolution the changelog reached at 0.34.0 when it stopped
citing section numbers a reader could not follow and kept the explanations.

## What a status in here means

A requirement marked `Implemented` names the test that asserts it. Where no
test exists, the status says so. Where a status was found to be wrong, `§10`
records the correction **and keeps the wrong wording visible** — a document
that quietly acquires the right answer teaches nothing about how the wrong one
was reached.

That practice is the reason these documents are long. It is also the reason
they are worth reading.
