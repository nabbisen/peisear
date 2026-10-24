# Handoffs — the specification checks itself

**0.57.0.** `§10.42`'s own argument, acted on: `requirements.md` is
this project's most-edited document and **nothing mechanical has ever
read it.** One subject — a specification that notices when it stops
being true.

Authorized 2026-10-15, placed ahead of `RFC 020` on `0.52.0`'s own
precedent that guard work does not queue behind a feature.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| SPEC-001 | [the citation that resolves](./SPEC-001-the-citation-that-resolves.md) | The two citation rules. **Rule 1 is already red: thirteen citations in live normative documents to four `§10.N` numbers that never existed.** Rule 2 is green and must be planted to be trusted | — | **Closed** — 2 rounds. Both rules planted and proven; rule 2’s first design found **23 real IETF citations** it would have flagged. The four promoted within budget, and **rule 1 is clean on the real tree** — re-derived in review, zero dangling citations |
| SPEC-002 | [the table and the vocabulary](./SPEC-002-the-table-and-the-vocabulary.md) | Appendix A summed from its own rows **and** checked against the entries; the status vocabulary closed. Two measured traps, both of which caught the architect while scoping | — | **Closed** — 1 round. Both rules built, both traps handled by construction, the vocabulary closed and the `Partial` spelling normalised. **Rule (b) is red in nine areas and right to be** — the table is stale, confirmed independently for Calendar and Privacy |
| SPEC-003 | [the counts nothing can check](./SPEC-003-the-counts-nothing-can-check.md) | *"Counts stated in prose checked against the workspace"* — **not mechanically decidable as written**, because a dated measurement and a present-tense claim look identical. A design question, not three more rules | — | **Closed** — 1 round. Declined to build the rule as named, surveyed the document to find out why that was right, and came back with **three wrong test counts of eight** as evidence. `§9.1`’s dated inventory table correctly excluded — verified in review |
| SPEC-005 | [the counts a step can check](./SPEC-005-the-counts-a-step-can-check.md) | Wire `SPEC-003`’s extractor to the runner **as a CI step, not a `cargo test` test** — the idiom `0.56.0`’s boot smoke and `backup.md` check already use. Three of eight citations were wrong, so a manual re-check at each sweep is the wrong answer | — | **Closed** — 2 rounds. Seven CI steps, each asserting its own expected citation count before comparing; absence planted and seen red, both paths reproduced in review. Round 2 also caught its own `zsh`-vs-`bash` testing error by checking the bytes rather than trusting the count |
| SPEC-004 | [the table re-derived](./SPEC-004-the-table-re-derived.md) | Appendix A’s nine stale areas re-derived from their own entries, and rule (b) flipped to clean. Plus the two prose fixes — `NFR-A11Y-009` → `Deferred`, and `FR-CAL-001`’s correction note withdrawn as superseded | — | **Closed** — 1 round. Nine areas re-derived, both grand totals recomputed from their member rows, rule (b) flipped to clean. **Re-derived independently in review and agreeing cell for cell across all 26 rows**, including the rows this round did not touch |
| REL-0.57.0 | [the release candidate](./REL-0.57.0-release-candidate.md) | The cut, `DEC-028`, `DEC-056`. **All five rounds are closed** — due now. Carries the claim to falsify (an empty production diff against `0.56.0`), what the release found, and the five things it does not prove | — | **Accepted** — at `692c20f`. Production diff is two `#[cfg(test)]` lines; `DEC-007` 549 re-measured; every rule green on the real tree. **Shipped** — `0.57.0` released 2026-10-24: signed tag at `12c94a2`, seven crates, five archives, `ghcr` at `0.57.0` and `latest` |

**Status lives in the table above, not in a message.**

---

## What scoping measured

Everything below was measured at `7c52515`, and two of the figures
corrected a pattern the architect used wrongly first.

- **45** `### 10.N` headings exist: `10.0`–`10.19`, `10.21`,
  `10.25`–`10.48`.
- **16 citations to four numbers with no heading** — `§10.20` ×5,
  `§10.22` ×1, `§10.23` ×2, `§10.24` ×8. **Thirteen are in
  `requirements.md` and `external-design.md`**; three are in closed
  handoffs, which `SPEC-001` §3 scopes out with its reason.
- **No `### 10.N` heading was ever written for the four** — but
  **the entries exist**, in an older compact register at
  `requirements.md:4355`–`:4376`, dated and naming their closing
  handoff. Round 1 found that; the architect had searched for
  headings and concluded about entries. Measured: 45 heading
  entries, 28 compact rows, **24 in both forms**, 4 table-only.
  A migration **24 of 28 complete**, and `SPEC-001` §1a finishes it.
- **45** four-digit `RFC 0NNN` references over tracked files, of
  which **two were in `SPEC-001`’s own text** — a census-taking
  document instantiating the pattern it counts, which round 1 hit
  independently from the other side. Rule 2 is green **as scoped**
  (maintained `docs/` plus published doc comments) and passes the
  day it is written, which is why `SPEC-001` §4 requires it planted.
- **168 requirement entries** — 99 `FR-*`, 69 `NFR-*` — matching
  Appendix A's own totals, whose arithmetic is also consistent.
- **180 `*Status*` lines against those 168 entries.** Five entries
  carry more than one; `NFR-CMP-006` carries **nine**.
- **The vocabulary has already drifted**: `Partial` ×4 against
  `Partially implemented` ×1, and `Met` (19, plus 12 `Met at`) has no
  column in Appendix A at all.

## Two traps this series exists to avoid building

**`NFR-A11Y`.** An ID whose area segment contains digits. The obvious
pattern `(FR|NFR)-[A-Z]+-[0-9]+` matches **none** of the eleven
accessibility requirements. **The architect used that pattern while
scoping**, got 58 non-functional entries against Appendix A's 69, and
noticed only because the two disagreed. A scan carrying it would be
green having examined 58 of 69 — `§10.36` inside the guard. Hence
`SPEC-002`'s requirement that **the rule assert the population it
examined.**

**A status line is not one per entry.** 180 against 168. A rule that
counts status lines counts twelve things that are not entry statuses,
eight of them in `NFR-CMP-006` alone.

Both were found by measuring rather than by reasoning, and both would
have produced a guard that passes while reading less than it claims.
**That is the specific failure this whole release is about**, so a
round that builds one has defeated its own purpose.

---

## What this series is not

**Not a rewrite of `requirements.md`.** The rules read it; they do not
reorganise it. A round that finds itself restructuring the document
has left its scope.

**Not a fix by fiction.** `SPEC-001` §2 is explicit: a dangling
citation is never resolved by minting a register entry to match it.
The register records defects this project actually had.

**Not a claim that prose is checkable.** `SPEC-003` exists to state
the limit precisely and document it where a reader will find it, not
to approximate it with something that looks thorough.

**Not open to a fourth round.** Anything else found goes to the
architect and waits, with the standing exception: a defect that
blocks the release is not a scope question — `0.56.0` used that
exception twice and both times it was right.

---

## What `0.56.0` left for whoever reads this first

- **Quote result lines verbatim; do not sum them in prose.** Six
  published figures across that release were off by one or two while
  the measurement underneath was right every time. `48 passed; 4
  failed; 1 ignored` is already in the output.
- **A `file:line` citation is the most fragile form this project
  uses** (`CON-004`'s review): `Cargo.toml:51` was correct when
  written and became `:60` because a comment block was added above
  it, with nothing touching the cited fact. `ROADMAP.md`'s 0.57.0 row
  carries this as a design input — prefer a symbol or an anchor, and
  if line citations stay, they are what a scan most needs to check.
- **Plant every rule.** `CON-006`'s predicate was seen red with all
  28 offenders named before it was trusted; `CON-005` round 3's own
  first proof passed while reverted and was investigated rather than
  taken. Both are the standard here.
- **A gate's stated purpose can outlive what it verifies.** A
  documentation fix in `0.56.0` left a CI step claiming to prove
  commands the document no longer gave. Rules in this series describe
  what they check, so the same drift is available to them.
