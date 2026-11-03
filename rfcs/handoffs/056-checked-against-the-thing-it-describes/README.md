# Handoffs — checked against the thing it describes

**No release number.** Authorized by the owner 2026-11-03, on the
**revised** proposal — the first version was withdrawn the same day after
the owner asked for it to be reviewed against this project's fundamental
philosophy, and its lead item did not survive. `main` is what a reader
gets; these land under `[Unreleased]` and whichever release is cut next
picks them up in its own `DEC-028` sweep.

**One rule, and it is a method rather than a prohibition**: every round
here exists because a sentence was checked against the thing it describes,
and three of them were wrong. **None was found by reading.** The dialect
comment was found by running a clippy invocation; `FR-TEAM-005`'s gap and
the eleventh `§10.34` instance were found by a ten-minute pattern over 185
status clauses followed by three greps of the test suite.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| DIA-004 | [the comment, and the second query](./DIA-004-the-comment-and-the-second-query.md) | `dialect.rs:62` says **"No `#[cfg(feature = "postgres")]` anywhere outside this module and `pool.rs`"** and that none of the five files it names carries a `#[cfg]`. Both are false: **thirteen sites in six files**, production code, and three of them carry a **second, postgres-only SQL body** for the health and sustainability queries. Two bodies per query with published rustdoc saying they do not exist. Carries the postgres clippy step as its regression guard | — first in the series | **Closed 2026-11-03.** **The round corrected the architect's own count**: **22 sites in eleven files**, not the thirteen in six the handoff claimed — four files missing from that table entirely — and it drew the split the table did not: **fourteen production, eight test-only**, with `lib.rs`'s site shown to be test-only by its own `all(test, …)` predicate. Re-measured by the architect without a `head` limit or a hardcoded file list: **the eleven-file set matches exactly**, and the single remaining difference — `pool/tests.rs` 5 against my 6 — is a doc-comment line quoting the attribute, so **their figure is the exact one.** **The `§1` hypothesis held for eleven of fourteen and the round reported the three that broke it rather than bending the comment to fit**: `lock_issue_set_for_sprint` (SQLite's `BEGIN IMMEDIATE` whole-database lock makes its body `Ok(())`, PostgreSQL needs `SELECT … FOR UPDATE OF i`), `capture_record` (`INSERT OR REPLACE` against `ON CONFLICT … DO UPDATE`, which must reset `captured_at` explicitly or it freezes at first capture), and `WIP_QUERY`'s decode cast, separated from the date translation it rides alongside. **The clippy step was seen red before being trusted**, and the architect re-planted it independently at a different site: `needless_return` in `optimistic_lock_guards.rs:79`'s postgres-only body — default features **exit 0, never sees it**; the new invocation **exit 101**, naming the line. Every figure re-run and exact: rustdoc **7** unchanged, `DEC-007` **632** unchanged with the CI block's own slices, `fmt` and both clippy invocations clean, `CI`/`Docs` `success` at `0f615bd`, and the diff two files with no query touched. **Architect's decision on the escalated design question, now in `dialect.rs` so it is not reopened: no seam helper, and the pairs stay.** Only the date/time group is mechanical and covering it needs a fragment DSL in place of a readable query; the other two are **semantic** differences a helper would hide. **What makes a paired body safe is the suite, not a helper** — `049`'s one-suite-two-backends runs all 33 integration binaries under postgres too (`test.yml:1022-1054`). *The architect went looking for a coverage risk here and there is none: the storage crate's own 31 `--lib` tests cover none of the paired modules, but the integration suite covers them under both backends* |
| REQ-005 | [the footnote nobody asserts](./REQ-005-the-footnote-nobody-asserts.md) | `FR-TEAM-005`, a **P1**, has an *Acceptance* clause naming an assertion that does not exist: the privacy footnote is byte-pinned in `peisear-i18n` and no `peisear-web` test asserts it on a screen. **The one round here that is for a user** | DIA-004, by order rather than dependency | **Closed 2026-11-03, and the handoff's premise was false.** **The test already existed** — `the_team_privacy_footnote_renders_on_the_team_screen_for_every_role` (`aggregate_privacy.rs:587`), added by `edea18c` and shipped at `0.41.0`, five weeks before the architect measured its absence. **No code changed; none was needed.** The round dated it with `git log -S` on the test's own name rather than inferring it, verified it green, then planted anyway because it already existed — removing the footnote paragraph fails it. **The architect re-planted independently at the role dimension**: rendering the footnote for admins only fails it **on the member's screen**, which is this requirement's own reader, and it catches a *present-but-empty* paragraph rather than only a missing one. **Why the architect's grep missed it**: it searched `peisear-web` for the literal words *"management role, not an oversight role"*, and the test renders the message key and compares against **that** — the words are in its output, never in its source. **A literal-string needle could not have found it, and the absence concluded from its silence was published twice**, in the handoff and in `§10.34`'s own worked example, where it illustrated *"the claim is true"*. **The round also corrected the second figure in that handoff**: prose-only `*Acceptance*` clauses are **6**, not 28 — the architect's `[^*]*` stopped at the first emphasis marker and truncated every clause to a fragment. Re-measured here paragraph-aware: **6, the same six they list.** **And the round's finding carried further than its own subject.** Instances **8, 9, 11 and 12 are one commit**: `edea18c` pinned four limbs and **all four statuses denied them for twenty-two releases** — one omission with four faces, not four misses. It also explains why the recorded remedy never caught them: a `DEC-028` sweep reads the release in front of it, and a status stranded by a previous release is invisible to every sweep after. `§10.34` now carries the table and the mechanism, and **`REQ-006` gains the instrument this implies** — the commit, not the phrase pattern, which could not have found instance 12 at all. Two corrections proposed in the report and applied by the architect; **`requirements.md` untouched by the round**, per the division. `DEC-007` **632 unchanged** — the handoff predicted +1 on the assumption a test would be written. No diff, no branch, no PR: *stated rather than silently skipped*, and the right outcome |
| REQ-006 | [pin the absence claims](./REQ-006-pin-the-absence-claims.md) | A `spec_scan` rule asserting an **enumeration** of the status clauses that claim an absence of coverage — the shape `docs_scan` rules 8 and 9 already use — so a twelfth `§10.34` instance cannot be written without someone noticing. **Not a census** | REQ-005, by order rather than dependency | **Closed 2026-11-03, with two findings — the first of this series — and one of them the architect's own claim.** `spec_scan` rule 3 pins six IDs and reports a *change* to that membership, judging nothing, with its own lookup asserted and **three** plants rather than two: a new claim, a broken anchor, and the **other** census direction — a known entry whose status stops matching is named rather than silently dropped. That third plant was not asked for and is the right instinct. The calibration read is the round's best work: the five suggested phrase additions produced **three false positives unbounded** — each hundreds of characters past its entry's own status — and **nothing at all** once capped at 500 characters on `docs_scan`'s own window convention, so they were **excluded with the reason recorded** rather than kept to look thorough. Architect's verification: the rule caught a claim planted in `NFR-SEC-001`, a **P0** outside the six; `DEC-007` **637** (226 + 335 + 75) with the CI block's slices; rustdoc **6** unchanged; `fmt`/`clippy` clean; `CI`/`Docs` `success` at `b975919`; diff two files, `requirements.md` untouched. **Finding 1, and its origin is the architect's**: the rule's doc comment said instance 12 (`FR-TEAM-005`) was one the phrase pattern *"could never have found at all"* — and `KNOWN_ABSENCE_CLAIM_IDS`, eight lines below it, **lists `FR-TEAM-005`**. The pattern shortlisted it correctly; what failed was the architect's **verification**, a grep of the suite for the status's literal words against a test that renders a message key. `REQ-006`'s `§2a` handed the round that false rationale and the round recorded it without checking it against the rule it had just built — in a series whose own subject is checking a sentence against the thing it describes. Corrected in the comment by the architect, since the error was the architect's. **Finding 2**: `rule_3_is_clean_on_the_real_tree` asserted `only(violations(..), "rule 3").is_empty()` **with no message**, so a future contributor learned only *that* the census had moved, never *which* entry — the one thing this rule exists to tell them, and `rule_1` and `rule_b` in the same file both name their findings. Fixed and re-planted: it now reads *"rule 3 found absence-claim membership changes: [… `NFR-SEC-001`'s governing *Status* clause now makes a coverage-absence claim this census does not know about …]"*. **One instruction was not executed and not flagged**: `§2a` asked for the commit pass to be **run and reported first**; `§5` instead answers whether to *automate* it, which was never asked. The architect ran it — every requirement ID cited in any test file, checked against that requirement's governing status — and it found **no instance the pinned six do not already hold**: `FR-API-006`'s citation explains a *withdrawn* test and `NFR-LANG-005` is cited as the *reason* for the fixture locale, both consistent with their statuses. So the deviation cost nothing, **and the rationale the architect gave for the instruction was wrong anyway** |

**One round in flight at a time**, in that order. The dependencies are
weak — the three touch disjoint files — but `054` showed what a series
costs when a round's findings arrive while another round is mid-flight on
the same documents.

## Closed 2026-11-03 — three rounds, and the architect was wrong in every one

**The series' own method worked, and it worked mostly against its author.**
Every round was commissioned on a measurement of the architect's, and all
three measurements were wrong:

| round | the architect's claim | what it was |
|---|---|---|
| `DIA-004` | 13 cfg sites in 6 files | **22 in 11**, with a production/test split never drawn |
| `REQ-005` | no web test asserts the footnote; 28 of 51 prose-only | **the test has existed since `0.41.0`**; **6** of 51 |
| `REQ-006` | the phrase pattern could never have found instance 12 | **it did** — `FR-TEAM-005` is in the rule's own pinned list |

**One root cause, three faces: a needle that does not match the shape of
the thing being searched.** A `head`-limited grep plus a hardcoded file
list; a literal-string search for a test that renders a message key; a
`[^*]*` that stops at the first emphasis marker. **Not one of them was a
reasoning error** — each was a measurement taken in a form that could not
have returned the right answer, then written up as though it had.

**What made the cost zero every time was one sentence**: *re-measure
precisely rather than trusting this table.* It is in each of these
handoffs, and each round obeyed it. **That hedge is the series' most
transferable output** — it belongs in every handoff that carries a number,
and it is worth more than any of the three guards built here.

**The round outputs, which stand on their own:** `dialect.rs` now says
where conditional compilation lives and the three reasons it is there;
a clippy invocation reaches the PostgreSQL-only bodies, proven by a plant;
`spec_scan` rule 3 pins the absence claims and names what moved. `REQ-005`
produced **no code at all**, which was the correct outcome.

**And one finding outgrew the series.** `§10.34`'s instances 8, 9, 11 and
12 are a single commit: `edea18c` pinned four limbs and all four statuses
denied them for twenty-two releases. **One omission with four faces, not
four misses** — and it explains why the entry's recorded remedy never
caught them, since a `DEC-028` sweep reads the release in front of it and a
status stranded by an earlier release is invisible to every sweep after.

## What this series is not

**Not a campaign against the instances.** `REQ-006` was first proposed as
*read all 185 `*Status*` clauses and report*, and that is the half the
owner's review removed: a one-time sweep is a snapshot that goes stale the
next time a status is written. **The rule is the deliverable; the read is
calibration.** If a round here starts producing a long list of corrections
instead of a guard, it has drifted back into the shape that was rejected.

**Not an overturning of `REQ-003`.** That investigation concluded no
machine can catch `§10.34`'s class, and it was right — it was asking
whether a rule could **judge** a status. `REQ-006`'s rule judges nothing;
it shortlists, and a person decides. **A rule that cannot be trusted to
decide can still be trusted to shortlist.**

**Not a redesign of the dialect seam.** `DIA-004` establishes why the
paired query bodies exist — the architect's reading is that they diverge on
**date and time functions**, which `PG-004`'s placeholder rewriter cannot
by design reach. Whether that divergence deserves its own seam helper is
**the architect's decision on `DIA-004`'s evidence**, not the round's.

## The trap this series will walk into, named once here

**Two of these three rounds end in a status or a comment that wants
editing, and neither is the round's to edit.** `REQ-005` will want to move
`FR-TEAM-005` to a status that cites its new test; `REQ-006` will form
verdicts on claims it shortlists. **Statuses are the architect's**, on
`CFG-001`'s division of labour — that round found a false claim of the
architect's by checking a premise instead of writing to it, and declined to
invent content nobody handed it. Write the proposed wording in the report
and stop there. **Eleven `§10.34` instances are what this division exists
to prevent**, and the architect produced all eleven.

`DIA-004` is the exception and only for its own comment: that comment
describes the code the round is measuring, so correcting it is the round's
work — but **describing the design, not improving it.**

## The standing exception applies

A defect found in the shipped product while this series is open earns a
round of its own rather than being folded into the round that noticed it.
`054` closed three rounds against one scoped that way, and each of its last
two was found by the round before it.
