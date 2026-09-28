# DOCS-001 — publish `docs/` as an mdbook site

**Issued by**: Architect
**Date**: 2026-09-29
**Target release**: 0.41.0
**Governing RFC**: none — `DEC-057`, owner-decided 2026-09-29.
**Depends on**: nothing. **The owner has already configured the repository's
Pages settings**; what is missing is the book and the workflow.

---

## 1. Shape — decided, so you are not choosing it

**Book root `docs/`, with `src = "."` in `book.toml`.** The owner asked to
*extend* `docs/`, not restructure it: every existing page stays where it is and
keeps its path, so no inbound link in the repository breaks.

- **`build-dir` must point outside the source tree.** With `src = "."` the
  default `docs/book/` would be inside the source and mdbook would recurse.
  Put it in `target/` and gitignore it.
- **`docs/src/assets/logo.png` already exists** — from the badges commit, not
  from mdbook. With `src = "."` it is just a subdirectory and nothing collides,
  but **the name is now misleading**: `docs/src/` is not the book's source. Say
  so in `book.toml`'s comment, or propose moving it and I will rule; **do not
  move it silently** — the README references it.
- `docs/SUMMARY.md` is new and lists every page. **`SUMMARY.md` is the
  navigation, so its order is an editorial decision**: getting-started,
  guides, operations, security, architecture, development, specification. Say
  if you would order it differently.

## 2. The blocking problem: four links point outside `docs/`

On a published site these 404. Measured — `.github/SECURITY.md` (3),
`ROADMAP.md` (2), `CHANGELOG.md` (1), `rfcs/handoffs/` (1).

**Rewrite them as absolute `https://github.com/nabbisen/peisear/blob/main/…`
URLs.** They are repository artefacts, not documentation pages, and pulling
them into the book would publish the RFC and handoff tree as a website, which
nobody has decided.

- **Pin to `main`, not to a tag** — these are living documents and the site is
  built from `main`.
- **`CHANGELOG.md` is the one to look at twice**: `changelog_scan` rule 5
  checks every markdown link into a changelog file. **Confirm the rule still
  sees it, or still passes, after the rewrite** — an absolute URL may fall
  outside what it parses, which would silently drop a check rather than break
  it. **Report which.**

## 3. The specification pages

`requirements.md` is 235 KB and `external-design.md` 119 KB. mdbook renders
each as one page — it will work and it will be heavy.

**Include both; they are the normative documents and the point of publishing.**
**Do not split them** — their section numbers are cited from everywhere and a
split changes every anchor.

**`docs/specification/history/` holds three superseded baselines.** **Link
them, do not include them** in `SUMMARY.md`: they are archival, they triple the
book's weight, and a search hit in a superseded baseline is worse than no hit.

## 4. What is published, and one thing I want stated rather than discovered

Everything in `docs/` becomes a public website. **It is already public in the
repository**, so this discloses nothing new — but say plainly in the PR which
pages are now a *site* rather than files someone would have to go looking for.

**`docs/static-js-verification.md` in particular.** `STATIC-001` moved that
file out of `static/` because it would have been served from **every
operator's own deployment**, which nobody chose. **Publishing it on the
project's own documentation site is a different thing and is fine** — the
objection was to every self-hoster serving it, not to the project doing so.
I am saying this so nobody reads `DOCS-001` as reversing `STATIC-001`.

## 5. The workflow

A second workflow beside `test.yml`: build the book and deploy to Pages on push
to `main`.

- **The build must fail on a broken internal link.** This project does not ship
  checks that cannot fail; `mdbook-linkcheck` or an equivalent, and **say which
  and what it does not cover** (external URLs are a network call — I do not
  want the docs build depending on github.com being up).
- **Deploy only from `main`**, and only after the build succeeds.
- **Do not touch `test.yml`.** `dec_007_ci_scan` reads it and a new job in the
  wrong file will fail the suite.
- Pin the mdbook version. An unpinned toolchain that reformats the site on
  someone else's push is the kind of thing that is discovered months later.

## 6. Verification

- `mdbook build` clean, **with the link check failing on a planted broken
  link** — restored byte-identical. A link checker never seen to fail is not
  evidence.
- **Every page in `SUMMARY.md` renders**, and the count matches the file count
  under `docs/` minus what §3 excludes. State both numbers.
- **The four rewritten links resolve** (by eye on the built site or by one
  `curl`; say which).
- `changelog_scan` after the `CHANGELOG.md` rewrite (§2).
- `DEC-007` unchanged at **369** — no Rust should change. If it does, say why.
- `fmt`, `clippy`, three consecutive workspace runs. The overflow gate is
  untouched.

## 7. Escalate rather than deciding

- **If `src = "."` does not work** as described — mdbook's behaviour with a
  non-`src` source directory is the assumption this whole shape rests on, and
  I have not run it. **Check that first**, before writing `SUMMARY.md`.
- **If the link rewrite drops a `changelog_scan` check** (§2).
- **If the specification pages make the build or the site unusably slow.**
  Report the numbers; splitting is mine to rule and §3 says why I would resist.
- **If Pages needs repository settings changed.** The owner configured them;
  if something else is required, say what rather than changing it.

## 8. Exit condition

`https://nabbisen.github.io/peisear/` serves the documentation tree, built from
`main` by a workflow that fails on a broken internal link; no existing file
moved; the four outbound links resolve; `changelog_scan` still checks what it
checked.
