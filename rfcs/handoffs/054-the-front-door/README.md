# Handoffs — the front door

**No release number.** `README.md` on `main` is what GitHub serves the
moment it merges, so this work does not wait for a cut and does not
get one of its own. It lands under `## [Unreleased]` in
`CHANGELOG.md` the normal way, and whichever release is cut next picks
it up in its own `DEC-028` sweep and Appendix C row.

**Raised by the owner, 2026-10-31:** *"Is there any plan to enhance
'Quick Start'? The current guide in `README.md` is far from 'quick'.
Also, not kind and not easy to users at all. Most users do not have
`cargo` client. Some of them do not have even `git`."*

There was no plan. The gap is real and it is narrower than a style
complaint: **we already fixed this in `install.md` and never carried
it to `README.md`.**

## What is actually wrong

Measured 2026-10-31 at `293df27`.

`docs/getting-started/install.md` offers three paths, ordered by what
a reader needs to have:

| | needs | commands |
|---|---|---|
| **Option A** — download a prebuilt binary | nothing | extract, run |
| **Option B** — `docker run` | Docker | **one** |
| **Option C** — `cargo install`, *"if you already have Rust"* | the Rust toolchain | two, plus a build |

`README.md`'s Quick Start offers **two paths, and both require
Rust** — the second also requires `git`:

```bash
cargo install peisear          # needs cargo
git clone … && cd peisear      # needs git and cargo
cargo run --release -p peisear
```

**So the product has shipped a no-toolchain path since `0.53.0` —
seven releases — and the front door never learned about it.** The
quickest start peisear has is one command
(`docker run -p 3000:3000 -v ./data:/data ghcr.io/nabbisen/peisear`)
and the section called *Quick Start* does not mention it.

**The cost of the path it does lead with**, measured rather than
asserted: CI's own `cargo install walk (NFR-REL-011)` job ran
`23:14:37`→`23:19:31` — **4m54s** — on a runner that already had the
toolchain. A reader without Rust installs Rust first. *Stated
bounded*: that figure is the whole job (checkout, toolchain, install,
walk), so the install alone is less; the point is the order of
magnitude against Option B's single pull-and-run.

## Why it survived seven releases

- **`FIV-004` (0.53.0) wrote `install.md`'s three options and did not
  carry them to the README.** Good work landing in one of two places,
  the same shape as `CON-004` writing operations pages one condition
  ahead of the others.
- **`NFR-REL-007` governs the README's *ordering*** — hero, overview,
  why/when, quick start, features or design notes, links — **and says
  nothing about what the quick start must contain.** So the five-pass
  sweep had nothing to catch, every release, correctly.
- **`§10.41` already quoted this exact block** as *"the instruction a
  first-time reader is most likely to follow"* and was closed on a
  different defect inside the same four lines. Nobody asked whether
  those were the right four lines.

**The lapse is the architect's** — `FIV-004`'s review and every sweep
since.

**This is `RFC 021`'s own class, one level closer to the reader.**
There, operations documentation promised a configuration
getting-started could not reach. Here, the front door offers the
hardest path while the product ships two easy ones.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| QS-001 | [the first command](./QS-001-the-first-command.md) | The Quick Start leads with what needs nothing, and `NFR-REL-007` gains the clause that would have caught this | — | **Open** |
| QS-002 | [the rule that keeps them together](./QS-002-the-rule-that-keeps-them-together.md) | A `docs_scan` rule: every command in the README's Quick Start names a path `install.md` offers. Seen red on a plant before it is trusted | QS-001 | **Open** |

Two rounds and no cut. `QS-002` does not start until `QS-001`'s shape
is settled, because the rule reads the thing `QS-001` writes.

## What this series is not

**Not a rewrite of the README.** `NFR-REL-007`'s ordering is met and
stays met; the hero, the overview, the why/when, the features and the
links are not this series' business. One section changes.

**Not a new install path.** All three already exist, are documented,
and are exercised — `NFR-REL-011` walks two of them in CI. Nothing
here builds an artefact; it changes which one a reader meets first.

**Not a claim that `cargo install` is wrong.** It is the right path
for someone who has Rust, it stays, and `install.md`'s own conditional
heading is the wording to mirror.
