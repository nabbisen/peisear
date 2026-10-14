# FIV-002 — something to download

**Target**: 0.53.0, after `FIV-001`. `RFC 017` proposal 1.

**Barriers 1 and 2** in the RFC's own table — *install a compiler*,
*wait for a release build* — are removed by there being a file to
download. Every release page today carries **zero assets**; I confirmed
it again while scoping, for `0.48.0`, `0.50.0`, `0.51.0` and `0.52.0`.

---

## §1 — what ships

A per-platform archive on each GitHub Release, built in CI on the tag:

| Target | |
|---|---|
| `x86_64-unknown-linux-gnu` | |
| `aarch64-unknown-linux-gnu` | |
| `aarch64-apple-darwin` | |
| `x86_64-apple-darwin` | |
| `x86_64-pc-windows-msvc` | |

**Naming and layout are `NFR-REL-002`'s**: version-suffixed, files at
the archive root, **no intermediate directory**. That requirement has
read `Implemented` throughout while no release has ever carried an
archive — the candidate process builds and extracts a tarball locally,
which is what the status was resting on. This round is what makes it
checkable against a thing a user actually receives, so **verify the
layout by extracting each produced archive**, not by reading the
workflow.

**`DOCS-003` found this from the other end**: the Quick Start's first
command opens a tarball no release provides. The instruction was right;
the artefact was missing. Do not change the instruction.

## §2 — the workflow never gates the release

**This is the design, not an implementation preference** (`RFC 017`
proposal 1, and the owner's standing concern is continuous cost):

- the workflow runs **after** the Release exists
- a failed matrix leaves a Release with **no assets** — exactly
  today's state — and is fixed by **re-running one job**, not by
  re-cutting a release
- **`DEC-056` stays four legs.** Nothing is added to what the
  architect runs by hand.

A cross-compilation matrix is the part of this release most likely to
break, and it breaks at the worst moment. Build the recovery path in
from the start rather than discovering it on a tag.

## §3 — how to verify it before there is a tag to verify it on

The upload path cannot be exercised without cutting a release, and
this round must not cut one. So:

- the workflow takes a **`workflow_dispatch`** with a ref input, and
  on that path **produces the five archives as workflow artifacts and
  uploads nothing to any Release**
- exercise it on a branch against **`0.52.0`**'s commit, download all
  five, and **extract each one** — five archives, correct names, files
  at the root
- **say plainly that the Release-upload step is the one path this
  round could not exercise**, and what would show it working. A step
  nobody has run is not evidence; naming it is the honest alternative
  to pretending otherwise.

**Do not attach assets to `0.52.0`'s published Release** to test the
upload. It is published, and a release page is not a scratch surface.

## §4 — what the archive contains

**`FIV-005` runs first and answers most of this** *(added 2026-10-15)*.
It embeds `static/` into the binary and amends `NFR-REL-002`'s layout
once, there. **Read that round's outcome before this section** — if
`static/` is in the binary, the question below is already settled and
what remains is the binary plus whatever documentation belongs beside
it. Do not re-amend `NFR-REL-002` here. If `FIV-005` landed differently
from what it proposed, this section is live again and **say so rather
than following it blindly.**

The binary, and whatever a running instance needs that is not compiled
in. **Check `static/` against `deployment.md`'s *What to ship*
manifest** — `docs_scan` rule 3 already holds that manifest to
`static/`'s real contents in both directions, so the manifest is
trustworthy; the question this round answers is whether the archive
matches the manifest. If it does not, say which is wrong.

## §5 — specification work (`DEC-028`)

- **`NFR-REL-002`**: its status stops resting on a local tarball.
  State in the entry what it is now checked against and how — and if
  the Release-upload path is still unexercised at the candidate, the
  entry says so rather than claiming a verification nobody performed.
- **A new published artefact may want its own entry.** Five
  per-platform archives are a release obligation this document does
  not currently name. **Do not invent one**: report whether you think
  one is missing, with what it would say. That is the architect's
  call at the candidate.

## §6 — gates

- `DEC-007` from a fresh block, against `FIV-001`'s post-round
  baseline. **No new tests expected** — this round is CI and
  packaging. A changed count means something unintended moved.
- `fmt`, `clippy -D warnings`, `mdbook build` with linkcheck.
- **`dec_007_ci_scan` is the one to watch.** It asserts every `DEC-007`
  block line has a matching CI `run:` step. A new workflow file is not
  a test target and should not disturb it — **confirm it is green and
  say you checked**, because a new `.github/workflows/*.yml` is exactly
  the shape that scan reasons about.
- The five archives, extracted, named, flat.

## §7 — escalate rather than deciding

- **If a target cannot be built in CI without a third-party action or
  a cross-compilation container.** A new dependency in the release
  path is a maintenance cost and is the architect's call, not a
  workaround to adopt quietly.
- **If `NFR-REL-002`'s naming convention turns out to be ambiguous for
  Windows** (`.zip` versus `.tar.gz`, `.exe` at the root). Report the
  ambiguity; the requirement is the thing to fix, not the archive.

## §8 — exit condition

A workflow that builds five targets, attaches them to a Release that
already exists, and cannot fail a release; exercised on a branch with
all five archives downloaded and extracted; `NFR-REL-002` verified by
extraction and its entry restated to say what it now rests on; the
upload path named as unexercised with what would show it working;
`DEC-007` unchanged from `FIV-001`'s baseline and accounted for;
`dec_007_ci_scan` confirmed green.
