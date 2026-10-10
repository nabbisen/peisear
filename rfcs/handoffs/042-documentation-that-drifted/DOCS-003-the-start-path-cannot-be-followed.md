# DOCS-003 — the start path cannot be followed

**First of three, and the other two depend on it.** `DOCS-005`'s scan encodes
the state this handoff produces, so it lands after. **Target**: 0.49.0.

**Audit**: `.git-exclude/reviewed/DOCS-AUDIT-user-facing-documentation.md`
(§A) and `…-requirements-design-architecture.md` (§A).

**Six wrong sentences in three files, and nothing here is a product defect** —
every behaviour described wrongly is one that works. The documentation is
describing an older version of a working product.

---

## §1 — the Quick Start's first command cannot be run

`README.md`'s Quick Start opens with `tar xzf peisear.tar.gz`. **No release
carries any asset** — `gh release view --json assets` returns **0** for
`0.48.0`, `0.47.0` and `0.41.0` — and no document gives a download URL. **The
first command a new user runs references a file they cannot obtain.**

**And the block contradicts itself**: line 1 unpacks a distribution, line 4 is
`cargo run --release -p peisear`, which needs the source tree and a toolchain.

**The right answer already exists** at `docs/operations/deployment.md:24`:
`cargo install peisear`, and the crate does ship a `[[bin]]`. **It is in the
last document a new user would read.**

**Rewrite the Quick Start around what actually works.** Two paths, and say
which is which:

- **`cargo install peisear`** — the published binary, for someone who wants to
  run it;
- **`git clone` + `cargo run --release`** — for someone who wants to read or
  change it.

**Do not invent a third.** In particular **do not propose attaching tarballs
to releases** — that is a release-procedure change, `DEC-056`'s territory, and
mine. If you think it is the right answer, say so in the report and leave the
Quick Start on `cargo install`.

## §2 — `installation.md` names a toolchain that cannot build the project

> *"peisear needs a Rust toolchain with Edition 2024 support — that is,
> `rustc` **1.85 or later**."*

**`Cargo.toml:26` declares `rust-version = "1.88.0"`**, and `NFR-CMP-001`'s
own correction records that **1.85 was false**: *"the dependency tree had
drifted past it and 1.85 could not build the workspace at all."*

The *Verify* block then prints `rustc 1.85.x …` as the expected output, **so a
user installs 1.85, sees exactly what the guide promised, and cannot build.**

**Fix both the prose and the Verify block**, and take the number **from
`Cargo.toml`** rather than from this handoff — if it has moved since I wrote
this, `Cargo.toml` is right and I am stale.

**Option B's `apt` instructions install `rustc-1.91`**, which satisfies
1.88 — check whether anything else in that section needs the same correction.

## §3 — `installation.md` says the stylesheets come from a CDN

> *"daisyUI, Tailwind … **loaded from a CDN on first page load**."*

**They are vendored.** `static/daisyui.min.css` (2.9 MB) and
`static/tailwind.css` are in the tree, and `components/layout.rs:18` says so:
*"`static/`, not loaded from CDN"*, citing `NFR-CMP-002`'s self-hostability.

**This matters beyond accuracy**: it contradicts the README's own promise of
*"no external services"*, and an operator deciding whether the host needs
outbound network access would get the wrong answer. `ASSET-001` vendored them;
`docs/operations/tailwind-local.md` is the current story.

## §4 — the first-run walkthrough teaches the failure path

> *"Drag issue cards between the columns… the server persists and **the page
> reloads** to show the confirmed state."*

**The success path has not reloaded since 0.26.0.** `FR-DM-003` is
*"Implemented on all four surfaces"* — optimistic in place, with a five-second
undo. The one `window.location.reload()` in `board.js` is at **`:69`, inside
`applyOutcome`**, reached only for a conflict/unavailable/unconfirmed outcome.

**Describe what a user actually sees**: the card moves immediately, an undo
affordance appears for five seconds, and a conflict is the case that reloads.
**Read `board.js` rather than this paragraph** — it is the authority and I am
summarising it.

## §5 — the crate count is wrong

**Seven crates exist** and all seven are published. `README.md:61` says *"**Six**
crates"* and names six, **omitting `peisear-i18n`** — which arrived with its
own RFC (006) at **0.21.0** and owns every user-visible string, the
`MessageKey` enumeration and the vocabulary guard.

**Fix the README's count and its list.** The architecture documents are
**`DOCS-006`'s** (0.50.0), not yours — **do not touch `docs/architecture/`
here**, or the two handoffs collide.

## §6 — the crates.io landing page has no working links

`crates/peisear/Cargo.toml:4` sets `readme = "../../README.md"`. Because that
path points **outside the crate directory**, crates.io resolves the README's
relative links against `crates/peisear/`. **Measured** against the rendered
README from the crates.io API: **all ten** repo-relative targets come back
prefixed `crates/peisear/`, which does not exist — the logo is a broken image
and **nine links 404** (probed).

**Two shapes, and the choice is yours with the reasoning:**

- **Absolute URLs** in the README for the ten targets — `https://github.com/nabbisen/peisear/blob/main/…`.
  Works everywhere, costs a little readability in the repo.
- **Move `README.md` into `crates/peisear/`** and have the root one be a
  pointer, or set `readme` to a crate-local copy. Keeps relative links,
  introduces a second file to keep in step — **which is this release's whole
  subject**, so weigh it accordingly.

**My lean is absolute URLs**, precisely because the second shape creates the
duplication this handoff exists to clean up. **Say which you chose and why.**

## §7 — gates

- **`DEC-007`** confirmed once at **407**, unchanged; nothing here is a test.
- **`mdbook build` with linkcheck** — `README.md` is in the book and you are
  changing ten links.
- **`cargo fmt --check`**, and `clippy` only if you touch Rust (you should
  not).
- **Verify §2 against `Cargo.toml` and §4 against `board.js`**, and say in the
  report that you read the source rather than this handoff.
- **Re-fetch the crates.io rendered README after the fix is published?** You
  cannot — it only changes on the next publish. **Instead state what the
  rendered links will be** and why, so the next release's candidate can
  confirm it.

## §8 — escalate rather than deciding

- **If `cargo install peisear` does not actually work** from a clean machine —
  that would be a far bigger finding than any of these.
- **If the Quick Start cannot be honest in under ~8 lines.** Its value is that
  it is short; if correctness needs more, say so rather than growing it
  silently.
- **If §6's two shapes both have a problem I have not seen.**
- **Anything suggesting a release-artefact change** — mine.

## §9 — exit condition

A Quick Start a stranger can follow; `installation.md` naming the MSRV from
`Cargo.toml` and no CDN claim; a first-run walkthrough describing the
optimistic path with the conflict path named as the exception; seven crates in
the README; the ten crates.io links fixed with the shape and reasoning stated;
`mdbook build` clean; and **the report saying which sources you read rather
than trusting this handoff.**
