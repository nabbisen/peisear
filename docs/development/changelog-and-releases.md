# Changelog, release notes and tags

This is how release notes are written and found. It is a development workflow,
not something an operator needs; the operator's view of a release is
[`CHANGELOG.md`](../../CHANGELOG.md).

## One place

**`CHANGELOG.md` is the only place release notes live.** Nothing is written
twice: each release has one section, in one file. There is **no GitHub Release
page** — a release is exactly three steps (tag, push the tag, `cargo publish
--workspace`), and nothing else is created. What a reader follows from a tag is
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
- **A tag is written once.** Once a tag is pushed it is a fixed reference:
  anyone who fetched it holds that object, and rewriting it means two people
  with the same tag name see different things. **This rule exists because it was
  broken**: `0.40.0`'s tag was force-pushed after its crates were published, on
  the same commit, to add the link above. The content was harmless and the act
  was not. Tags before `0.40.0` carry only `peisear X.Y.Z` and **are not
  rewritten** — 39 force-pushes to add a link is the same mistake at scale.
- **If a tag is wrong, the answer is the next version**, not a rewrite. A
  release whose tag names the wrong commit is a release to supersede.

## Keeping the file a readable size — `DEC-055`

**Decided 2026-09-25.** The arrangement below was implemented from another
project's policy before anyone here decided it (see
`.git-exclude/reviewed/CHANGELOG-POLICY-review.md`). It is kept **on this
project's own reasoning**: `CHANGELOG.md` had reached 4,154 lines, the move is
verbatim and reversible, and the scan makes the arrangement checkable rather
than remembered. What is *not* inherited is anything about how a section reads.


`CHANGELOG.md` holds **the current series only**. Older series are **moved**,
never copied, into one file each under [`changelog/`](../../changelog/), and
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
| From 0.41.0, a dated section opens with `### Highlights` | same | in place |
| **The tag's message carries the release-notes link** | **nobody — the release procedure above, by hand** | **not checked** |
| An external URL (the tag's link) resolves | nobody — nothing here uses the network | not checked |
| The Highlights are good | a reader | not checkable |

A green `changelog_scan` says the files are arranged as this document says. It
does not say the tag has its link.
