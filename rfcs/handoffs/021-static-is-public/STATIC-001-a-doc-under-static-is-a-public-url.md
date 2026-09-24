# STATIC-001 — a document under `static/` is a public URL, and one is about to ship

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.38.0 — **this blocks the tag.**
**Governing RFC**: none — defect fix.
**Source**: found by the dev team while assembling `REL-0.38.0`'s candidate
(§1c). Ruled in `.git-exclude/reviewed/REL-0.38.0-review.md` §1c.
**Depends on**: nothing. Lands **on top of `8a5fe5a`**, which is not discarded.

---

## 1. The defect

`app.rs:249` mounts `ServeDir::new("static")`, so **every file in that
directory is public on every deployment.** `static/` now holds eight assets and
`static/README.md`, added by my commit `4b14fcd`, and `GET /static/README.md`
returns 200 with 4,065 bytes on the built binary.

That document describes **what this project's test suite does not cover.** It
holds no secret and it is not dangerous, and it is also not something an
operator chose to publish alongside their tracker. Nobody opted into serving
this project's QA posture from their own domain.

**It has never been released.** `4b14fcd` is in 0.38.0, so tagging as-is would
publish it for the first time. Withdrawing something already shipped is a
changelog entry explaining an accident; not shipping it is a file move.

I placed it "beside the files it governs" without asking what `static/` *is*.
The adjacency was worth less than not publishing the document.

## 2. What to do

**2.1 — Move it to `docs/static-js-verification.md`.** Content unchanged; it is
a good document in the wrong directory. Add it to `docs/README.md`'s index
beside the other entries.

**2.2 — Leave a pointer where a developer will actually look.** A short comment
block at the top of `static/dm.js` — the shared module the other scripts wire
into — naming the document and its path. One pointer, not five. It is a comment,
not user-visible copy, so `NFR-LANG-001` is unaffected; run the guard anyway.

**2.3 — Stop this recurring, in the module that already walks the directory.**
`static_js_scan` (`crates/peisear-web/src/static_js_scan.rs`) already
`fs::read_dir`s `static/` and carries an exclusion list with a reason per
entry. Add one assertion there: **every entry under `static/` has an extension
on a short allow-list of servable asset types** — `.js` and `.css` today.

- Its failure message says *why*: the directory is served in full, so anything
  in it is public.
- **Extending the list is the intended way to add an asset type** — a `.svg` or
  a `.woff2` is a deliberate one-line change with a reason beside it, which is
  the same shape `is_named_escalation_exclusion` already uses in
  `touch_target_scan`. Follow that precedent rather than inventing one.
- `§10.28` is the warning to heed while writing it: **do not let the doc
  comment claim something stricter than the code does.**
- No new test file, no new CI job — it belongs in the module that is already
  there.

**Not proposed**: changing `ServeDir`'s root or filtering by extension at
request time. Both move a repository-layout question into the request path,
and the rule wanted here is about what the repository contains.

## 3. Then: candidate 2

**A second candidate commit on top of `8a5fe5a`**, not a rewrite of it. Keep
`8a5fe5a` in history — it is a sound cut that predates a finding.

The changelog's `[0.38.0]` section needs **no rewrite**; it is accepted as
written. Two amendments only:

- **Add the `§10.29` / `§10.30` citations**, one line, matching 0.37.0's
  practice.
- `static/README.md` needs **no changelog entry** — it is not shipping, so
  there is nothing for a reader to be told. If anything in the section
  currently mentions it, remove that.

**Note**: `§10.29` was corrected by me in `047931a` after your review — the
"read backwards, 22 sites" claim is now bounded to what was measured, with your
sprint finding recorded in it. Nothing in the changelog depends on that
sentence; check that it does not.

## 4. Verification

- **`GET /static/README.md` returns 404** on the binary built from the new
  tarball, and the eight assets still return 200 — name them and their sizes.
- **`static/tailwind.css` is byte-identical** to `0.37.0`, as candidate 1
  reported.
- The new assertion **fails on the tree as it is today** — plant it before the
  move, confirm it names `README.md`, then move the file and confirm it passes.
  **Report both runs**; an assertion never seen to fail is not yet evidence.
- `docs/README.md`'s index resolves.
- **Re-run in full, because the tarball changes**: `DEC-007` three consecutive
  (expect **297** — one new test in an existing file; if you add it elsewhere,
  say so and the block and `test.yml` travel together, authorised), `fmt`,
  `clippy`, the **overflow gate on the extracted tree** (a `static/` change is
  exactly what it watches), and `cargo publish --workspace --dry-run` from a
  detached worktree of the new candidate.
- **The migration check does not need repeating** — `0018` is untouched and
  candidate 1 exercised it from a 0.37.0 database on the final tree. Say that
  you are relying on that rather than silently skipping it.
- New tarball sha256 and the `git ls-tree` comparison.

## 5. Escalate rather than deciding

- **If the allow-list assertion trips on something other than `README.md`** —
  that is a second instance and I want to see it before it is fixed.
- **If moving the file breaks a link** anywhere in the repository. Grep for
  inbound references first; `4b14fcd` may have pointed at it from `§10.15` or
  from the handoffs.
- **If the overflow gate moves off 90/90.** Nothing here should touch it.
- **If the count is not 297.**

## 6. Exit condition

`static/` contains only served assets and a guard says so; the document lives
in `docs/` and is reachable from both the index and the code it describes;
`GET /static/README.md` is 404; a second candidate exists on top of `8a5fe5a`
with every gate re-run and a fresh tarball.
