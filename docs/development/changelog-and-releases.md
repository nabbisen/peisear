# Changelog, release notes and tags

This is how release notes are written and found. It is a development workflow,
not something an operator needs; the operator's view of a release is
[`CHANGELOG.md`](https://github.com/nabbisen/peisear/blob/main/CHANGELOG.md) (on GitHub; outside this
book).

## One source, two copies

**`CHANGELOG.md` is the only place release notes are *written*.** Each release
has one section, in one file, and everything else is a copy of it — never a
second draft. Until `DEC-056` there was exactly one copy: a link. There are now
two, and both are derived from the section rather than written beside it.

**A release is four steps** (`DEC-056`, 2026-09-29): tag · push the tag ·
`cargo publish --workspace` · create the GitHub Release **with the version's
changelog section as its body**.

**Why a body and not only a link — and it is not verifiability.** The tag's
message carries a link pinned to the tag (below), and that link is
**immutable**: `blob/<tag>/CHANGELOG.md` is frozen at that commit, so archiving
older series later cannot change what it shows. **A Release body is the weaker
artefact on that axis** — it is editable at any time by anyone with write
access. What the body buys is **reach**: the Releases page, watch
notifications and the Atom feed show the body and never follow the link, and
anything reading releases programmatically reads the body. So the link is the
copy that cannot change and the body is the copy people actually see, and the
section both come from is the one thing anybody edits.

**Order matters.** Publish the crates *before* creating the Release, so the
Releases page never announces a version whose `cargo publish` failed. The tag
comes first because `cargo publish` runs from a worktree of it. **If the
publish fails, there is a tag and no Release** — which is the recoverable
state, since a tag can be superseded by the next version and a Release page
cannot be un-announced to a watcher.

**Who.** The architect, on the owner's approval, alongside the tag and the
publish. The dev team never runs it.

**Existing releases are not back-filled.** 0.1.0–0.40.0 have no Release page
and do not get one: forty of them, for versions nobody is reading, on the
`Highlights` precedent — new practice from here, no retrofit.

What a reader follows from a tag is
a link in the tag's own message, below.

## Each release's section

Work lands under `## [Unreleased]` as it merges. At release the heading is
renamed to the version and dated (`## [0.41.0] — 2026-10-12`), and the section
is written up for a reader.

Sections follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) —
`Added`, `Changed`, `Fixed`, `Security`, and this project's own `Internal`,
listing only those with something in them.

**There is no required opening heading.** A `### Highlights` rule was written
here and enforced by the scan; it came from another project's policy, no
decision here adopted it, and it was removed on 2026-09-25 before it took
effect. **What a section must do is what every release handoff already asks
of it**: lead with what a user can see, say plainly when nothing changed
rather than padding, and name this project's own mistakes rather than fixing
them quietly. Those are judgements a check cannot make, which is why the scan
checks the *arrangement* and not the prose.

If a Highlights convention is ever wanted it is one decision and ten lines —
see the note in `crates/peisear-web/src/changelog_scan.rs`.

## The tag carries the link

Release tags are **annotated**, and the message is two lines apart from the
subject: the subject `peisear X.Y.Z`, a blank line, then a link to
`CHANGELOG.md` **pinned to that tag**:

```sh
git tag -a X.Y.Z -F - <commit> <<'MSG'
peisear X.Y.Z

Release notes: https://github.com/nabbisen/peisear/blob/X.Y.Z/CHANGELOG.md
MSG
git push origin X.Y.Z
```

`git tag -n3 X.Y.Z` and GitHub's tag page then show where the notes are.

- **Pinned to the tag, not to `main`.** A link to `main` breaks the day older
  sections move (next part); a link to the tag never does, because at its tag a
  version is always the first dated section of `CHANGELOG.md`.
- **Write the tag after the changelog is final.** The link is only right if the
  tagged commit's `CHANGELOG.md` already holds the section. Tag the release
  commit, not one before it.
- **A tag moves only before its crates are published.** That is the whole
  rule, and **the boundary is publication, not the version number** — 0.x
  publishes to crates.io exactly as 1.x does, and the registry is immutable
  either way. A published `.crate` can be yanked but never replaced, so a tag
  that moves away from the commit it was built from leaves a permanent
  mismatch: a reader takes the version from crates.io, opens that tag in git,
  and gets different source.
  - **Before publishing**: re-make it freely. A pushed tag with nothing on
    crates.io is a candidate reference, not a release.
  - **After publishing**: the commit is fixed. **If the tag is wrong, the
    answer is the next version**, not a rewrite — a release whose tag names
    the wrong commit is a release to supersede.
- **Rewriting only a tag's message, after publishing, is the narrow middle
  case.** The commit correspondence survives, so the registry and the tag still
  agree; what breaks is smaller — two people with the same tag name holding
  different objects. **This is where `0.40.0` landed**: its tag was force-pushed
  after its crates were published, on the same commit, to add the link above.
  Harmless content, and still not something to do on purpose. It was left as it
  is because moving a tag twice is worse than the thing it would fix.
- **Tags before `0.40.0`** carry only `peisear X.Y.Z` and **are not rewritten**:
  39 force-pushes to add a link is the same mistake at scale, and every one of
  those releases is published.

## Keeping the file a readable size — `DEC-055`

**Decided 2026-09-25.** The arrangement below was implemented from another
project's policy before anyone here decided it (see
`.git-exclude/reviewed/CHANGELOG-POLICY-review.md`). It is kept **on this
project's own reasoning**: `CHANGELOG.md` had reached 4,154 lines, the move is
verbatim and reversible, and the scan makes the arrangement checkable rather
than remembered. What is *not* inherited is anything about how a section reads.


`CHANGELOG.md` holds **the current series only**. Older series are **moved**,
never copied, into one file each under [`changelog/`](https://github.com/nabbisen/peisear/tree/main/changelog)
(on GitHub; outside this book), and
`CHANGELOG.md` ends with links to them.

- **Before 1.0.0 a series is ten minor versions** and their patches: 0.1–0.9,
  0.10–0.19, 0.20–0.29, and so on. It ends when the next series' first release
  ships: when 0.50.0 is released, 0.40–0.49 moves to `changelog/0.40-0.49.md`.
- **From 1.0.0 a series is one major version**: `changelog/1.x.md` when 2.0.0
  ships.
- **An archive is written once**, when its series ends, and is not edited
  afterwards. Each version appears in exactly one file.
- **The move is done with the release that starts the new series**, in its own
  commit, by moving the old sections verbatim.

**Known limit:** a link someone outside the project made to a section of
`CHANGELOG.md` on `main` stops working when its series moves. Links pinned to a
tag — including the one in every tag message — are not affected.

## What checks this

The rules that can be read from the files are checked by
`crates/peisear-web/src/changelog_scan.rs`, which runs under `cargo test` (and so
in CI's `peisear-web --lib` step). Each rule has a test that breaks it.

| Rule | Checked by | Status |
|---|---|---|
| Each version appears in exactly one file across `CHANGELOG.md` and `changelog/` | `changelog_scan` | in place |
| `CHANGELOG.md` holds only the current series (the workspace version's) | same | in place |
| Each archive holds one whole series, the one its name says, and no `[Unreleased]` | same | in place |
| Every archive is linked from `CHANGELOG.md`, and every relative link there resolves | same | in place |
| No markdown file links to a changelog section that is not in the file it names | same | in place |
| The workspace version has a dated section | same | in place |
| ~~From 0.41.0, a dated section opens with `### Highlights`~~ | **nothing — withdrawn 2026-09-25 before it took effect** | **not a rule** |
| **The tag's message carries the release-notes link** | **nobody — the release procedure above, by hand** | **not checked** |
| An external URL (the tag's link) resolves | nobody — nothing here uses the network | not checked |
| The section is worth reading | a reader | not checkable |

A green `changelog_scan` says the files are arranged as this document says. It
does not say the tag has its link.

## The specification sweep — `REQ-003`, `§10.34`

**Every release candidate sweeps the two specifications before it tags.**
`DEC-028` already requires that both are amended *at each release, not
afterwards*, and that the candidate does not tag until they are. This is the
part of that work which has repeatedly been skipped: not amending the entries
the release *touched*, but finding the ones it **closed without being
touched**.

Seven times a requirement's status has said one thing while the shipped code
said another — all seven the architect's. `REQ-003` tested four text rules
against six of them and **none reached the bar**; the full result is `§10.34`.
So this is a procedure, by hand, and the reason to trust it over a scan is
that **four of the seven were caught exactly this way**: by a person reading
the specification against what had just shipped, which is what assembling a
candidate is.

Four passes, in this order, over `requirements.md` and `external-design.md`:

1. **Every entry the release touched.** The easy half, and the only half that
   has reliably been done. Amend the status, and date the amendment.
2. **Every entry whose status cites a scope that the release changed.** The
   failure mode in full: `FR-DM-001` was amended from five surfaces to four
   and reached Met, and `FR-DM-002` — a **P0**, one screen below it — kept a
   status written when two surfaces shipped, for five releases. Nothing in
   `FR-DM-002`'s own text was wrong. Its *subject* had changed.
3. **Rule (e), by hand**, which is what survives of `REQ-003`'s best scan:
   *does this stale-looking status cite a sibling requirement that is already
   clean?* It scored 4 of 6 and was rejected as a scan because its zero false
   positives were measured **on zero opportunities** — but by hand, on the
   handful of entries a release actually puts in play, it is a minute's work
   and it catches a real class.
4. **Anything that counts or summarises statuses.** A second record of the
   same facts drifts, and three of Appendix A's rows were stale at 0.43.0 —
   Accessibility by two whole entries. If a count cannot be re-derived, say in
   the document that it was not re-derived rather than leave it reading as
   current.

**Two things this procedure must not become.** It is not a re-audit of all 162
entries — `REQ-001` did that once and it is not a per-release cost. And a
sweep that finds nothing is reported as *swept, nothing found*, never left
silent: a pass with no output is indistinguishable from a pass not run, which
is the shape this whole class comes from.

**It is a `§3` gate item of every release-candidate handoff**, with its
findings named in the candidate report. `§10.34` closes when two consecutive
releases sweep clean; the first run, at 0.43.0, found **six** things — three
stale entries and three stale appendix rows — which is why it is not closed
on one trial.

**Nothing checks this.** It is a procedure, it is the architect's, and it is
recorded here so that skipping it is visible.
