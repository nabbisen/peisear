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
| REQ-005 | [the footnote nobody asserts](./REQ-005-the-footnote-nobody-asserts.md) | `FR-TEAM-005`, a **P1**, has an *Acceptance* clause naming an assertion that does not exist: the privacy footnote is byte-pinned in `peisear-i18n` and no `peisear-web` test asserts it on a screen. **The one round here that is for a user** | DIA-004, by order rather than dependency | **Open, the dev team's** |
| REQ-006 | [pin the absence claims](./REQ-006-pin-the-absence-claims.md) | A `spec_scan` rule asserting an **enumeration** of the status clauses that claim an absence of coverage — the shape `docs_scan` rules 8 and 9 already use — so a twelfth `§10.34` instance cannot be written without someone noticing. **Not a census** | REQ-005, by order rather than dependency | **Open, the dev team's** |

**One round in flight at a time**, in that order. The dependencies are
weak — the three touch disjoint files — but `054` showed what a series
costs when a round's findings arrive while another round is mid-flight on
the same documents.

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
