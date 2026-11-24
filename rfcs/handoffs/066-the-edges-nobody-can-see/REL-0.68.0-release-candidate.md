# `REL-0.68.0` — release candidate

**Recommendation: cut it.** Exit criteria 1–6 are met, measured at this tree
rather than carried from the rounds' reports. **Criterion 7 is the owner's**,
and none of `DEC-056`'s four legs runs until that approval.

Written 2026-11-24 against `main` at `bbf44c1` plus this cut's own `DEC-028`
sweep.

---

## 1. What this release is

**Three releases taught the contrast check to read *text* colour. WCAG
1.4.11 governs the rest, and this product had never measured it.**

- **67 focus rings** — every `.input`, `.select` and `.textarea` — drew at
  `base-content/20`: **1.52:1** against a 3:1 requirement. **A keyboard
  reader could not see where they were.**
- **70 control boundaries** — the same 67 plus three checkboxes — drew at
  the same `/20`. On a card, where the control paints its own white fill,
  **that line at one and a half to one was the only thing saying a field
  existed.**

Rings now draw at **full `base-content`** (17.21:1) and boundaries at
**`/70`** (6.42:1 on a card, 5.24:1 on the page body) — **both tiers the
product already had**, so the release adds no colour and leaves the theme
alone.

**`contrast_scan` gains a fourth mechanism**, and **every one of the four
exists because a colour escaped a guard whose requirement's words covered
it**: a muted class, a semantic family used as text, a colour the
stylesheet applies with no class involved, and now a non-text property.
`§10.63` carries that table, because the pattern is the finding.

**The new mechanism deliberately does not compute ratios.** That arithmetic
needs `background-clip` semantics, the element's own fill, and which side of
the border the adjacent colour sits on — which is exactly where my own
recommended tier went wrong (§8). The measured figure is a human's, recorded
beside the selector; the test asserts the acknowledgement and its override
exist.

**`NFR-A11Y-010` was widened, not duplicated** — see §8.

---

## 2. Exit criteria

| # | Criterion | State |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per handoff, in dependency order | **Met** — `5878974`, `07b2151`, `4c14167`, each reviewed and accepted, each its own commit |
| 2 | `fmt`, `clippy -D warnings`, the full per-crate suite, **with captured logs** | **Met** — §3 |
| 3 | Requirements amendments applied; nothing `Divergent` without a gap entry | **Met** — `NFR-A11Y-010` widened, `§10.63` opened and closed, zero `Divergent` statuses, Appendix A re-derived and unchanged |
| 4 | `CHANGELOG.md` records the *why* | **Met** — three `Fixed` bullets, saying what a reader could not see rather than naming ratios |
| 5 | Affected documentation updated in the same release | **Met**, checked rather than assumed — §5 |
| 6 | Tarball built and extraction verified | **Met** — §4 |
| 7 | Release recommendation issued; **owner approves** | **This document. Awaiting the owner.** |
| 8 | **What ships next** reconciled as of the tag | **Not yet** — in the release commit, with the `0.68.0` row removed |

---

## 3. Criterion 2, measured at this tree

```
commands (+++ markers):   42   (expected 42)
non-zero exits:            0
tests passed (summed):   694
tests failed (summed):     0
```

**`DEC-007`: 691 → 694**, the three new guard tests. Log: 1,833 lines.

| gate | result |
|---|---|
| `cargo fmt --all -- --check` | exit **0** |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** |
| `… --no-default-features --features postgres` | exit **0** |
| `cargo build --release -p peisear` | exit **0** |
| CI on `4c14167` (the last round) | **50/50 jobs, 0 failed** |
| CI on `bbf44c1` (the series closing) | watched to green |

**Two of the series' claims re-verified by me rather than accepted:**

- **the new guard's first plant** — `class="checkbox toggle"` added in a
  throwaway worktree made `every_reachable_boundary_leak_is_acknowledged_or_exempt`
  fail, naming `.toggle:focus-visible outline-color -> …oklch(var(--bc)/.2)`;
  baseline 8 passed, tree clean afterwards;
- **`EDG-001`'s cascade**, verified without a product build by serving the
  three real stylesheets with a one-element page and reading the computed
  values: `outline-color: oklch(0.223899 0.031305 278.072)` (the override
  winning) and, at that point, `border-color: … / 0.2` — which is what
  `EDG-002` then repaired.

---

## 4. Criterion 6 — the archive, extracted and walked

```
archive: peisear-0.67.0-x86_64-unknown-linux-gnu.tar.gz   8,396,562 bytes
tar tzf → peisear      ← one member, at the root (NFR-REL-002)
```

The archive says `0.67.0` because `Cargo.toml` still does — the bump is held
for the release commit (§7), and `release.yml:113` refuses to attach an
archive whose version does not match the tag.

**Walked the extracted binary**: unattended start, `/health` **200**,
register → authenticated `/projects` **200**, and the **served `app.css`
carries both of this release's rules** — the focus-ring rule at full
`base-content` and the `/0.7` boundary rule — with the server stopped by pid
and **0 listeners** afterwards.

**What this walk does not cover, stated rather than implied.** It confirms
the artefact *carries* the rules; it does not re-measure a rendered ring or
border. Those were `EDG-001`'s and `EDG-002`'s measurements, each with its
cascade planted, and `EDG-002`'s own figures corrected mine. One backend,
one platform, one user, as always.

---

## 5. Criterion 5 and the `DEC-028` sweep

`requirements.md` was already at `0.68.0` with its `Amended` field moved —
**the round did that itself**, which is the first time a round has carried
that part of the sweep; this cut added its Appendix C baseline row.

**`external-design.md` is unamended, and the check names what it read**:
`§5.5 Focus behaviour` is a table of focus *destinations* — page load, skip
link, validation failure, disclosure, post-action redirect — and `§5.6`
requires every flow to complete with the keyboard. **Neither states how a
focus indicator looks**, and no section describes a control's border. The
repairs change the appearance of a state the document requires to exist,
not the state itself.

Both Appendix C tables **re-rendered with the pinned `mdbook`**: one table
each, 18 and 12 rows, **zero** stray pipe paragraphs.

---

## 6. What is not fixed

- **Card and alert borders** (~1.3:1) — decoration beside a shadow or a
  filled surface, out by 1.4.11's own "required to identify" test.
- **`.radio`/`.toggle`** — zero sites today, and now **guarded against**
  rather than pre-empted: the plant above is what that looks like.
- The **21** smallest-text sites, the **nine** `link link-hover` sites, the
  **fifteen** bare-`opacity` sites, the **`xs` control tier** — recorded
  candidates.
- **The browser-rendered contrast sweep** — the only check that resolves a
  cascade in every case, **named for the third time** and still not built.
- **A Pages deployment wedged since 2026-11-18**, which GitHub refuses to
  cancel by any route this token has.
- **`proc-macro-error2`**, still arriving via `syn_derive 0.2.0`.

---

## 7. What the release commit will contain, and why it is held

`Cargo.toml`'s bump, `Cargo.lock`, `CHANGELOG.md`'s `[Unreleased]` heading
becoming `## [0.68.0] — <cut date>`, and `ROADMAP.md`'s criterion-8
reconciliation, in one commit — `changelog_scan` derives the version from
`Cargo.toml` and **requires a dated section for it**.

---

## 8. The honest part

**Two of my instructions in this series were load-bearing and wrong.**

1. **The tier.** I recommended `/50` as *"the first tier that passes"*. It
   does not: a semi-transparent border composites over **the control's own
   white fill**, so `/50` reads **2.73:1** against the page body. My figure
   composited the border over the page instead. `EDG-002` measured both
   candidates, re-derived the formula against a second implementation
   before trusting the discrepancy, and shipped `/70`. **Had the round
   followed its instruction, this release would have shipped a fix that
   fails on the one site that tests it.**
2. **The requirement.** I recommended adding `NFR-A11Y-013 — Non-text
   contrast`. **It already exists as `NFR-A11Y-010`**, since `0.42.0`, with
   the same title and the same criterion, its first sentence naming *"the
   boundaries of interactive controls"*. I had checked which numbers were
   taken and never read what 010 said — the namespace, not the semantics.
   `EDG-003` read the document, widened that entry, and spent no number.

**Twelve of my figures or instructions have been corrected across four
series**, and these two are the first that would have changed what shipped.
Each was caught the same way: by a round reading the thing itself — the
rendered pixels, the document — rather than the sentence handed to it.

**One thing that has stopped**: the `rfcs/` citation count, which reddened
`main` three times at `0.67.0`, now shares `docs_scan` rule 5's own
exclusion and asserts zero.

**One question, and it is the only one this document asks:** cut `0.68.0`,
or hold it?
