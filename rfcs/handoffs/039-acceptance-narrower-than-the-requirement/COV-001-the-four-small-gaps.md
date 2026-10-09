# COV-001 — the four small gaps

**`§10.35`'s coverage work, the half that is just tests.** Seven tests across
four requirements, all small, all independent of each other. **Target**:
0.46.0.

The other three gaps (`FR-PER-006`'s chain, `FR-HLT-007`'s three indicators,
`NFR-A11Y-008`'s live regions) are **deliberately not here** — each needs a
judgement about *what a test should assert* rather than just writing one, and
they go together at 0.47.0.

**Read first**: `.git-exclude/reviewed/REQ-005-review.md` §3 — **it carries a
correction to your own report that changes one of the four below.**

---

## §1 — `NFR-PRIV-002`'s tooltip (P1, privacy) — and the reason was wrong

**Your report said `assert_no_capacity_leak` *"scans body text only"* so a
`title=` attribute *"structurally cannot"* be seen by it. That is not right,
and I accepted it in review before checking — the error is both of ours.**

The helper takes `resp.text()` (`workload_privacy.rs:150`, `:178`, `:191`) —
**the raw HTML** — and substring-matches `"5/5 pt"`, `"over capacity"`,
`"strained"` and `"badge-error"` over all of it, **attributes included**. A
tooltip rendering any of those four would fail **today**.

**So do not write a fifth negative over the same four strings. It would add
nothing.**

**The real gap**: the helper's vocabulary is finite, and **nothing pins what
the tooltip *does* contain.** `WorkloadStrip` (`issues.rs:517`) renders
`"{name} — {n} in-flight issues"` (`en.rs:249`) — compliant, and unasserted. A
future change putting capacity-derived text there in **wording those four
strings do not match** passes every check that exists.

**Write a positive assertion**: on a page where another member's workload
strip renders, that member's `title=` attribute **is** the permitted
name-and-in-flight-count shape and contains no capacity denominator. Assert
the shape, scoped to that member's own element — `scoped_form`/`scoped_move_form`
are the precedent for locating one element rather than searching the page.

**If you conclude a positive assertion cannot be written without pinning copy
that `peisear-i18n` owns, say so** — asserting a message *key*'s presence may
be the right shape instead, and that is a judgement I would rather have
reported than guessed.

## §2 — `FR-SUB-002` / `FR-SUB-003` — three trigger conditions

`0015_sub_issues.sql` raises on four conditions. **One is tested**
(`cannot_create_sub_issue_under_a_sub_issue` — 1-level-only, `FR-SUB-001`'s
clause). **Three are not:**

- **same-project** — *"sub-issue must share project with its parent"*
- **self-parent** — *"an issue cannot be its own parent"*
- **demote-with-children** — *"cannot demote an issue that has its own
  sub-issues; promote them first"*

One test each, in `sub_issues.rs` beside its tested sibling.

**Assert the translated outcome, not the raw `RAISE` string.**
`translate_trigger_error` (`peisear-storage/src/issues.rs`) maps each message
to a `MessageKey`, so the user-visible path is *trigger → mapped key →
rendered sentence*. **A test asserting the raw SQL text would pass while the
mapping was broken** — which is the half the user actually meets. Assert
through the route a user takes.

**Note for the record**: when these land, `*Acceptance*: trigger-enforced`
stops naming a mechanism class and starts naming artefacts. **That edit is
mine**, on your evidence.

## §3 — `FR-PER-007` clause 3 — one test, both states

`me.rs:507` is `<details open=any_watch>`: the sustainability panel
self-expands when a watch-threshold signal is present. **Untested in either
state.**

One test, both directions: with a watch-threshold signal, `open` is present;
without one, it is absent. **Both halves matter** — an assertion that only
checks the expanded case passes against a panel that is always open.

## §4 — `FR-NAV-003` — team detail and sprint detail

`breadcrumb` covers project detail and issue detail. **Team detail and sprint
detail render breadcrumbs and neither is asserted.** Two tests, in the
`breadcrumb` crate.

**This is not `FR-AUTH-004`'s structural case** and the entry now says so:
each screen builds its own trail, so two untested screens are two untested
behaviours. Assert the trail's actual content — the screen's own ancestors —
not merely that a `<nav>` exists.

## §5 — gates

- **`DEC-007`** three consecutive runs. **Expect +7**, from 384 to **391** —
  and if it is not 391, say which test you did not write and why rather than
  reconciling the number.
- **Each new test seen to fail.** Four of the seven assert an **absence** or a
  **shape** (§1's tooltip, §3's `open`-absent half, §2's three refusals), and
  this project's standing discipline is that such a test is worth nothing
  until seen failing. **Plant, quote the failure, restore byte-identical.**
  §1's especially: plant capacity text into the tooltip and watch your
  positive assertion fail.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.
- **Overflow gate only if a component changes.** None should: these are
  tests. **If you find yourself editing a component, stop** — that means a
  gap is a defect and it is a different handoff.
- No new test file is expected, so no `DEC-007` registration. **If one lands,
  both halves.**

## §6 — escalate rather than deciding

- **If §1's positive assertion cannot be written without pinning i18n copy.**
- **If any of §2's three conditions turns out already tested** somewhere I did
  not look — two of us have now miscounted coverage in this thread by reading
  a name.
- **If a gap turns out to be a defect** rather than missing proof. All seven
  are believed compliant-by-inspection; **a failing test on first write is a
  finding, not a bug in your test**, and it stops this handoff.
- **If `FR-NAV-003`'s two screens render a trail that is wrong** rather than
  untested.

## §7 — exit condition

Seven tests, each seen to fail before it passed; `DEC-007` three times at
**391**; no component touched; and for §1, the positive-assertion shape
**named** so I can correct `NFR-PRIV-002`'s acceptance to cite it.

**`§10.35` closes when this lands and 0.47.0's three are handed off** — the
three that need a judgement, not a test.
