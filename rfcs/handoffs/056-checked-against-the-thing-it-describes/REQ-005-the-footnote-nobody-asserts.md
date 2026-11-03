# REQ-005 — the footnote nobody asserts

**`FR-TEAM-005` is a P1 whose own *Acceptance* names an assertion that
does not exist.**

> *Acceptance*: the string "management role, not an oversight role"
> appears on the team detail screen.

Its status says so plainly and accurately: *"the footnote renders on a
member's team page; the string is byte-pinned in `peisear-i18n`.
**No web test asserts it on the screen**."*

**Checked, 2026-11-03**: the string is pinned in `peisear-i18n`
(`tests/rendering.rs:105`, `src/en.rs:592`), and the only `peisear-web`
occurrence of the words is a **comment** in `auth_boundary.rs:190-191`
citing the principle. **No assertion.** So the i18n pin guarantees the
words will not drift; nothing guarantees a user ever sees them.

## §0 — why a P1, and why this one

**This is the round that is for a user.** The footnote exists so a member
reading their team page is not left to infer that an administrator can see
their personal sustainability data — `SPEC §11.4` / `§29.6`, and
`NFR-PRIV-001`'s own inventory is what makes the claim true. **A privacy
reassurance that silently stops rendering is worse than one that was never
written**, because the product keeps the behaviour and loses the sentence
that tells the user about it.

## §1 — what this round is

One `peisear-web` test: a member (not an admin, not a non-member) loads
the team detail screen, and the footnote string is asserted on it.

**Write it to `§10.17`'s standard, and that entry is why this is not a
one-liner.** Eleven assertions in this suite were once passing on markup
other than their own subject, each confirmed by planting a real defect and
watching the existing assertion stay green. A bare
`body.contains("management role, not an oversight role")` is that shape:
it would pass if the string appeared in a `<title>`, a hidden element, an
unrelated card, or a second copy added later.

**So: assert the string in its own context** — the footnote element on
that screen — and **prove the assertion is load-bearing by planting**:
remove or relocate the footnote in the template, watch the test fail,
revert, watch it pass. Report the planted failure text.

## §2 — the bound on what this round closes

**`FR-TEAM-005` is one of 28.** `requirements.md` has **51**
`*Acceptance*` clauses and **28** of them name no test identifier at all
(architect's measurement: `*Acceptance*` clauses containing no backticked
identifier — **re-measure before quoting it**).

> **It is 6, not 28 — corrected by the round, and verified.** The six are
> `FR-AUTH-001`, `FR-AUTH-002`, `FR-NAV-001`, `FR-PROJ-002`, `FR-TEAM-005`
> and `FR-HLT-005` (whose clause says *"none — see below"* on purpose).
> **The architect's measurement used `grep -o '\*Acceptance\*:[^*]*'`, and
> `[^*]*` stops at the first `*`** — which in these wrapped prose
> paragraphs is an emphasis marker, so every clause was truncated to a
> fragment and a fragment with no backtick in it looked like a prose-only
> clause. **The regex did not match the document's structure, and the
> number it produced was out by a factor of four and a half.** Second time
> in this series: `DIA-004`'s site table was wrong for the same kind of
> reason. **Re-measure before quoting — as this round did.** This one was found because
the status census's first probe stumbled on it, not because the 28 were
audited.

**Do not audit the other 27 in this round.** Report the count as you
measure it and leave it; whether that becomes a round is the architect's
call, and `REQ-004`'s own 117 unaudited entries are the precedent for
naming a denominator rather than quietly adopting it.

## §3 — gates

`cargo fmt --check`; `clippy --workspace --all-targets -- -D warnings`;
`mdbook build docs` if any document changes; the full per-crate suite with
`DEC-007` from the runner — **this round adds one test, so the count moves
by exactly one**; say which figure it moves from and to, and remember the
count is defined by `test.yml`'s own block lines.

**`FR-TEAM-005`'s status will need amending once the test exists.** Write
what the new status should say in your report; **do not edit
`requirements.md` yourself.** Statuses are the architect's, on `CFG-001`'s
division of labour, and eleven `§10.34` instances are what that division
exists to stop.

## §4 — escalate rather than deciding

- If the footnote turns out **not** to render for some member case — a
  team with one member, a member who is also an admin — that is a defect
  in the product, not a test to bend around it. **Report it and stop.**
- If asserting the string in its own context needs a template change
  (no stable element to anchor on), **say what you would add and why**
  rather than adding a test id nobody designed.

## §5 — exit conditions

1. One `peisear-web` test asserting the footnote **in its own context** on
   a member's team detail screen.
2. **Seen red on a plant**, with the failure text quoted, reverted, green
   after.
3. The 28/51 `*Acceptance*` figure re-measured and reported, the other 27
   explicitly left alone.
4. The proposed new wording for `FR-TEAM-005`'s status, written in the
   report for the architect to apply.
5. `§3`'s gates after the last edit, `DEC-007` +1 from the runner.
