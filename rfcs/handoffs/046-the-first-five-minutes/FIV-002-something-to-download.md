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

---

# FIV-002 round 2 — the version nobody checks, and the dependencies nobody counted

Round 1 is accepted (see the review); the workflow works and the five
archives extract correctly. Two things it reported are not quite what
it did.

## §9 — the archive's version is `Cargo.toml`'s, and nothing ties it to the tag

Both packaging steps read the version out of `Cargo.toml`. Round 1's
own comment defends that for `workflow_dispatch`, correctly — a branch
has no version-shaped name. **On a `release` event it is a coupling
with nothing holding it.**

**Measured 2026-10-15, and it is not theoretical:**

```
main's Cargo.toml:      version = "0.52.0"
0.52.0 tag's Cargo.toml: version = "0.52.0"
```

**`0.52.0` shipped on 2026-10-14 and `main` is 0.53.0's tree** — yet
`main` still says `0.52.0`, because the bump happens when the release
is cut. So the two sources of truth agree at exactly one commit per
release and disagree the rest of the time, and **nothing in the
workflow checks which one it is standing on.** A Release cut from a
commit whose `Cargo.toml` had not been bumped would attach five
archives named for the previous version, to a Release named for the
new one, and every gate in this project would pass.

**Remedy**: on `github.event_name == 'release'`, compare the version
read from `Cargo.toml` against the triggering tag and **fail the job**
if they differ. A packaging step that cannot tell which release it is
building is `§10.36` in the one workflow whose output the public
downloads.

**This also corrects round 1's own §3.** It offered the archive name as
*"the strongest confirmation available that the historical tag was
actually built, not `main`'s current HEAD"*. It is not confirmation at
all — **`main` would have produced the identical name**. The claim is
nonetheless true, on the other two pieces of evidence the round
gathered: `/static/app.css` 404ing (impossible on post-`FIV-005`
`main`) and the binary being 20.8 MB against `main`'s 23.7 MB. **Say
which evidence carries a claim**, and drop the one that does not.

## §10 — two third-party actions were added, and the report says none were

`§2` reads: *"no cross-compilation, no third-party action or container,
so `§7`'s first escalation condition never arises."* The workflow uses
**`dtolnay/rust-toolchain@stable`** and **`Swatinem/rust-cache@v2`**.

**The escalation condition genuinely did not arise** — `§7` asks about
a *target* that cannot be built without one, and none needed either;
every target builds natively. That reasoning is right and the round
does not owe an escalation. **But the sentence is still false**, and
`§7`'s stated reason — *"a new dependency in the release path is a
maintenance cost and is the architect's call"* — applies to a
dependency added for convenience exactly as much as to one added from
necessity.

**Both are at moving refs** (`@stable` and `@v2` are tags that
advance), in the one workflow that holds `contents: write` and writes
to published Releases. That is third-party code, re-resolved on every
run, with permission to modify what the public downloads.

**The call, which is mine**: prefer removing both.

- GitHub's hosted images ship a rustup toolchain, so
  `rustup target add <target>` in a `run:` step does what
  `dtolnay/rust-toolchain` is doing here.
- `Swatinem/rust-cache` is a build-time saving on a workflow that runs
  a handful of times a release. An 18-minute cold matrix is
  acceptable; a supply-chain dependency on the release path is the
  more expensive side of that trade, by this project's own
  *"new dependencies are expensive"*.

**If a runner turns out not to carry a usable toolchain — the arm64
Linux image is the one I would actually check — keeping the action is
the right answer, and then pin it to a commit SHA**, not a moving tag.
Report what you find either way; this is a measurement, not a
preference.

## §11 — what this round does not reopen

`NFR-REL-002` is settled. `NFR-REL-009` is **new and is the
architect's**, written at this review from round 1's own escalation —
do not write a second one, and do not move its status: it closes on
the first Release that actually carries five assets, which is
`0.53.0`'s own.

## §12 — exit condition

A version check on the `release` event that fails the job on a
mismatch, with the comparison's handling of a leading `v` stated;
`§3`'s confirmation claim re-attributed to the evidence that carries
it; the two actions removed, **or** a measured reason a runner needs
one and that one pinned to a SHA; `DEC-007` unchanged at **486** and
said to be unchanged rather than assumed; and all four gates re-run
**after the round's last edit**.

---

# FIV-002 — closed at round 2, with one claim corrected

**Round 2 is accepted and `FIV-002` is closed.** No round 3: the
workflow is right, and what remains is a correction to the record,
which is the architect's to make rather than a round's to perform.

## §13 — round 1 did build against the pin; the divergence was not real

Round 2's own `§2` reported, as a discovery:

> *"round 1's removed `dtolnay/rust-toolchain@stable` action was
> building the public archives against `stable`, not this project's own
> `1.97.1` pin … the release workflow quietly didn't. Nobody had
> measured that divergence before this round went looking."*

**Checked at the review against round 1's own run log (38667002997),
and it is not what happened.** Two lines, seconds apart, in the log the
claim was read from:

```
info: note that the toolchain '1.97.1-x86_64-unknown-linux-gnu' is currently
      in use (overridden by '/home/runner/work/peisear/peisear/rust-toolchain.toml')
…
    - 1.97.1 x86_64-unknown-linux-gnu 8bab26f4f68e0e26f0bb7960be334d5b520ea452
…
rustc 1.99.0 (b940084d7 2026-09-28)
```

**`rust-toolchain.toml` overrode the action, exactly as it is designed
to.** The `rustc 1.99.0` line is the action reporting the toolchain
*it* installed; the `cargo build` that followed used `1.97.1`, and
rustup said so in the same log. **Round 1's archives were built with
the project's pin.**

**What was real** is the defect round 2 actually fixed: no toolchain
was explicitly active, so rustup reached its deprecated auto-install
path to resolve the pin. Removing the actions and naming the toolchain
with a bare `rustup install` is the right fix, and it is right on those
grounds alone. **The fix does not need the claim, which is why the
claim being wrong costs nothing but the record.**

**The lesson, which is the reason this section exists.** The claim was
drawn from one line of a log whose adjacent line said the opposite.
That is `§10.36`'s shape in its original form — a reading that cannot
distinguish *what ran* from *what was installed* — and it arrived
inside an otherwise exemplary round that measured three times before
settling. **A measurement taken while chasing something else deserves
the same second look as the one you went looking for.**

*The claim also survives in two commit messages (`f7b6f10`,
`224e390`). Those are not rewritten — history surgery costs more than
the error does, and this section is where the record is corrected.*
