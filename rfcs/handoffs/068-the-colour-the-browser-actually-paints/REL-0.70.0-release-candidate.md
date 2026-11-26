# `REL-0.70.0` — release candidate

**Recommendation: cut it.** Exit criteria 1–6 are met, measured at this tree.
**Criterion 7 is the owner's**, and none of `DEC-056`'s four legs runs until
that approval.

Written 2026-11-26 against `main` at `245314f` plus this cut's own `DEC-028`
sweep.

---

## 1. What this release is

**Four releases taught `contrast_scan` to read a declaration. This one reads
the pixel.**

Every mechanism before this one reads a *declaration* — a muted class token,
a semantic family, a sheet rule, a border property — and **no declaration
resolves a cascade**. The project has paid for that twice: `§10.52`'s `<nav>`
that read as visible and computed `display: none` at four widths, and
`EDG-002`'s measurement of a recommended `/50` border at **2.73:1** where it
had been predicted at 3.16, because a translucent border composites over the
control's **own fill**. `EDG-003`'s guard was built **deliberately unable to
compute a ratio** for exactly that reason.

`browser-checks/contrast-gate.mjs` covers **WCAG 1.4.3 and 1.4.11 together**,
painting each background layer and then the foreground on a 1×1 canvas so the
**renderer** does every conversion and every composite.

**It found one defect four source mechanisms cannot see.** `issues.rs`'s
Board/List switcher drew its **selected** segment at **3.87:1** against 4.5 —
opaque-on-opaque, two tokens from two different rules. Repaired by **removing**
`btn-active`, not by adding a colour: plain `btn-primary` measures **4.83:1**,
the tier 37 other sites already use.

**And the rule it writes down matters more than the gate**, now in
`NFR-A11Y-005`:

> Source is total and approximate. Rendered is exact and fixture-bounded.
> Where they disagree, the renderer is right about what it reached, and silent
> about the rest.

The fixture builds **24** URLs against **68** `.route(` registrations, and the
brand dot is a **named exclusion** in one check and a **4.21:1 failure** in the
other. Both correct — which is why that sentence had to exist.

---

## 2. Exit criteria

| # | Criterion | State |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per handoff, in dependency order | **Met** — one round, `245314f`, reviewed and accepted |
| 2 | `fmt`, `clippy -D warnings`, the full per-crate suite, with captured logs | **Met** — §3 |
| 3 | Requirements amendments applied; nothing `Divergent` without a gap entry | **Met** — `NFR-A11Y-005` widened a fourth time, `NFR-A11Y-010` a third, `§10.65` opened and closed, 171 entries unchanged |
| 4 | `CHANGELOG.md` records the *why* | **Met** — `[Unreleased]` opened fresh after `0.69.0`'s cut |
| 5 | Affected documentation updated in the same release | **Met**, by reading — §5 |
| 6 | Tarball built and extraction verified | **Met** — §4 |
| 7 | Release recommendation issued; **owner approves** | **This document. Awaiting the owner.** |
| 8 | **What ships next** reconciled as of the tag | **Not yet** — in the release commit, with the `0.70.0` row removed |

---

## 3. Criterion 2, measured at this tree

```
commands (+++ markers):   42   (expected 42)
non-zero exits:            0
tests passed (summed):   699
tests failed (summed):     0
```

**`DEC-007`: 699, unchanged.** Every new assertion lives in
`contrast-gate.mjs`, which `browser-checks/README.md` states does not run
inside `cargo test --workspace` and is not counted in this block. Log: 1,831
lines.

| gate | result |
|---|---|
| `cargo fmt --all -- --check` | exit **0** |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** |
| `… --no-default-features --features postgres` | exit **0** |
| `cargo build --release -p peisear` | exit **0** |
| CI on `245314f` | **50/50, 0 not-success**, read per job |
| the browser job's **own step list** | **six** steps, `contrast-gate.mjs` sixth, `success` |

**I ran the new gate myself on this tree** rather than accept its report:

```
coverage: 24 pages x 5 widths, exclusions=["text-5xl font-bold text-error","text-primary"]
text (1.4.3): 4813 elements examined, 120 raw violations, 0 unacknowledged, 0 unpaintable
border (1.4.11): 217 examined, 97 raw (all EXEMPT focused-border, logged not failed), 0 unacknowledged
broader bordered population (count only): 2298
known-good control (/today <h1>): 14.05:1
0 failing cells, 0 sanity failures          exit 0
```

The known-good control reproduces my own prototype's figure exactly, by a
second implementation.

**Also re-verified at review**: `GATE-001`'s wiring guard failing by name on
the unwired gate; `spec_scan` 42/42, `contrast_scan` 8/8, `css_scan` 9/9 and
`changelog_scan` 9/9 after the round's last edit; and **a checked
`.checkbox`'s fill measured at 17.21:1** against the page, because the round's
own exemption moves 1.4.11's subject from the border to the fill and nothing
then looks there (§6).

---

## 4. Criterion 6 — the archive, extracted and walked

```
archive: peisear-0.69.0-x86_64-unknown-linux-gnu.tar.gz   8,396,537 bytes
tar tzf → peisear      ← one member, at the root (NFR-REL-002)
```

The archive says `0.69.0` because `Cargo.toml` still does — the bump is held
for the release commit (§7), and `release.yml` refuses an archive whose
version does not match the tag.

**Walked the extracted binary**: unattended start, `/health` **200**,
register → authenticated `/projects` **200**, the served `app.css` carrying
the three `/0.7` rules, and **`btn-primary` served with zero `btn-active`** on
the project page — the repair present in the artefact, not only in the source.
Server stopped by pid; **0 listeners** afterwards.

**What this walk does not cover**: it confirms the artefact serves the
repaired class, not that the rendered ratio is 4.83:1 — that is the gate's
job, and the gate ran (§3). One backend, one platform, one user.

---

## 5. Criterion 5 and the `DEC-028` sweep

`requirements.md` was already at `0.70.0`: the round moved the header fields
at the series close, which is where `0.69.0` established they belong. This cut
added its Appendix C baseline row.

**`external-design.md` is unamended, and this release's check was a reading
rather than an assumption.** The repair changed a **rendered appearance** —
the selected segment of `SCR-08`'s Board/List switcher. `SCR-08`'s own rows
state what that screen renders and in what order; **no section states how a
selected segment is filled**, and the selected state still reads as a filled
segment beside transparent siblings, the treatment two other controls in the
same component already use. No screen, route, state or externally observable
copy changed.

Both Appendix C tables **re-rendered with the pinned `mdbook`**: one table
each, 20 and 14 rows, **zero** stray pipe paragraphs.

---

## 6. What is not fixed, and one gap I found while reviewing

- **CSS `opacity` is still not composited** — it dims an element and its
  subtree as one unit and is never folded into `color`. The same named limit
  `contrast_scan` already carries, and the **19** bare-`opacity` sites stay
  unmeasured by either check.
- **Card and alert borders stay decoration**, `0.68.0`'s decision, with the
  broader **2,298**-element bordered population logged and never asserted on
  so that tooling cannot reopen it.
- **A checked `.checkbox`'s fill is unchecked by anything.** The round skips
  its resting border for a good reason — once checked, the filled square is
  the boundary a reader perceives — which **moves 1.4.11's subject to the
  fill, where no check looks.** I measured it: **17.21:1** on this theme, so
  nothing is hidden today, and my own prediction beforehand (that the fill
  would be `--p`, about 4.2:1) was wrong. **Worth one sentence in the gate's
  comment; not worth a change.**
- **Two stale doc references** to the constants this round replaced, and
  rustdoc does not flag them for a reason I could not establish.
- **CI's `rustdoc-links` job cannot see private items' doc links**, and **two
  are genuinely broken on this tree** — `crate::static_js_scan` and
  `super::super::handlers::issues::parse_planned_datetime`. **Pre-existing,
  not this round's**, and a candidate: a gate whose reach is narrower than its
  name.
- The **17** in-scope small-text sites, the nine `link link-hover` sites, the
  `xs` tier, `FR-HLT-005`'s unverified status, `proc-macro-error2`, and the
  **Pages deployment wedged since 2026-11-18**.

---

## 7. What the release commit will contain

`Cargo.toml`'s bump, `Cargo.lock`, `CHANGELOG.md`'s `[Unreleased]` becoming
`## [0.70.0] — <cut date>`, and `ROADMAP.md`'s criterion-8 reconciliation, in
one commit — `changelog_scan` derives the version from `Cargo.toml` and
requires a dated section for it.

---

## 8. The honest part

**This release's scope was wrong when the owner authorized it, and he caught
it by asking.** He authorized the row, then asked for it to be reviewed
*"carefully with my fundamental philosophy"*; the review found three flaws,
all in my shape rather than in the measurement: a fifth mechanism with a
sixth piece of machinery to police a two-entry list; **no stated answer to
which check is right when two disagree**; and a text/non-text split that paid
for one harness twice. The row was revised and re-authorized, and what
shipped is one round instead of three, with the authority rule written down.

**The gate's hardest requirement came from my own failures, not my success.**
Three prototypes: `examined=0, failures=0` from a parser that matched nothing
— which reads exactly like a product that passes — then **1906 of 1906**
because Chrome round-trips `oklch()` through canvas `fillStyle` unchanged,
then the pixel route that works. That is why the gate asserts an examined
floor, `unpaintable == 0`, a known-good pair, and **treats a 0% or 100%
failure rate as an error in itself** — and the round planted each of the four
and watched it fail.

**Two of my own guesses were wrong again in this cycle, both caught by
measuring**: the checked checkbox's fill colour, and `NAMED_EXCLUSION_*`'s
doc links, where I expected rustdoc to flag what it does not. Neither reached
a document.

**One question, and it is the only one this document asks:** cut `0.70.0`, or
hold it?
