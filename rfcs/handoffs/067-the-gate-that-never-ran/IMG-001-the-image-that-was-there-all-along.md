# `IMG-001` — The image that was there all along

Series [067 — the gate that never ran](./README.md), **round 2 of 3.**
Read `README.md` §1.2 and §1.3 first. `GATE-001` is **accepted**.

`GATE-001` closed *a check nothing ran*. This round closes *a status
nothing re-read* — the same defect in the document instead of the CI file.

---

## 1. The defect

`NFR-REL-010 — Published container image` reads:

> *Status*: **Divergent** — no image has ever been published, and `0.53.0`
> is the first Release that can publish one.

**It is false, and has been for eight releases.** GHCR issues an anonymous
pull token for a public package, so this takes no credentials:

```bash
T=$(curl -s 'https://ghcr.io/token?scope=repository%3Anabbisen%2Fpeisear%3Apull&service=ghcr.io' | jq -r .token)
curl -s -o /dev/null -w '%{http_code}\n' -H "Authorization: Bearer $T" \
  https://ghcr.io/v2/nabbisen/peisear/manifests/0.68.0
curl -s -H "Authorization: Bearer $T" https://ghcr.io/v2/nabbisen/peisear/tags/list
```

At the proposal: `0.68.0`, `0.67.0` and `latest` each **200**, and the tags
list read `["0.61.0","0.62.0","0.63.0","0.64.0","0.65.0","0.66.0","0.67.0","0.68.0"]`.
**Re-run it; quote what you get.** It is the **only** genuinely `Divergent`
entry of the 171 — and that count came from parsing entries, because a
line-anchored `^\*Status\*:` grep cannot see a field wrapped onto the next
line, which is how the architect reported *"zero `Divergent` statuses"* at
`0.68.0`.

**This one is the architect's, and it is a repetition rather than a slip.**
Every release since `0.53.0` he reported the image as *"confirmable only via
`image.yml`, because the token lacks `read:packages`"* — one failed route
taken as proof that no route exists, repeated for months while two `curl`
calls stood unused. Write that into `§10.36`'s next instance in `ABS-001`,
not here; this round fixes the entry.

---

## 2. Establish what actually happened — do not write the obvious sentence

**The entry says `0.53.0` was the first Release that could publish. The
registry's oldest tag is `0.61.0`.** Something sits between those two facts,
and *"published since `0.53.0`"* would be a third wrong sentence in the same
entry.

What is known, and checkable:

- `image.yml` has **24** runs; the oldest are **2026-10-15**, on `main`, and
  the second onward **succeeded**.
- `git log --follow -- .github/workflows/image.yml` shows three commits, all
  `FIV-003`-era. The workflow has not changed since.
- The `0.68.0` run was **`release`**-triggered and succeeded.

**So the workflow was green for releases before `0.61.0` while no `0.61.0`-
or-earlier version tag exists in the registry.** Establish which: what tags
those earlier runs pushed (read the workflow's own tag rules and a run's own
log), whether they were `release`- or `push`-triggered, and whether anything
removed them. **If the record does not answer it, say so in those words** —
*"the registry's history does not show why the first version tag is
`0.61.0`, and these are the places I looked"* — and give the entry the
narrower claim you can prove. A requirement that overstates its own history
is how this one got here.

---

## 3. What to correct, and the coupling that will catch you

1. **`NFR-REL-010`'s `*Status*`** — to `Implemented`, with the measurement,
   the date, the tag range you observed, and **how it was measured** (an
   anonymous token, no credentials, because *public pullability* is the
   property the entry asserts; a push log proves only that a push happened).
   Keep the old text as a dated correction, the pattern every amended entry
   in this document uses — the previous status is part of the record.
2. **The header's `Amended` field** and `Covers release`, as every series
   close does.
3. **Appendix A's `| Release |` row**, today `11 | 9 | 1 | — | — | 1
   undefined`. **`spec_scan`'s `SPEC-002` rule (a) asserts that row's own
   arithmetic and maps every entry's status to one of its five columns**, so
   this is guarded — but **re-derive the row from all eleven entries' own
   status lines rather than decrementing one column**. A decrement is right
   only if every other entry is already right, which is the assumption this
   series exists to distrust.
4. **`§10.63`'s table is not this round's** — it records *how a colour
   escaped a guard*. The absence-claim pattern belongs in `ABS-001`.

---

## 4. The check that asks the registry

**Recommendation: a step in `image.yml`, after the push, that resolves the
manifest it just pushed — anonymously.**

- **Anonymous on purpose.** The entry's property is that the image is
  *published and pullable*, not that a credentialed client can see it. A
  step using the job's own token would pass on a private package and prove
  the wrong thing.
- **After the push, in the same run**, so a failed publish cannot be read as
  success. Assert the HTTP status, and assert the tag appears in
  `tags/list`.
- **Not a Rust test.** `cargo test --workspace` must not reach the network —
  the same reasoning `browser-checks/README.md` gives for staying out of
  `DEC-007`'s inventory.
- **Plant it**: point the step at a tag that does not exist and watch it
  fail by name, in a scratch branch or worktree — **never** by pushing a
  broken `image.yml` to `main`. Quote the failure.
- **Mind the registry's own shape**: a manifest request needs an `Accept`
  header listing the media types (OCI index, Docker manifest list, Docker
  v2); without it a registry may answer `404` for an image that exists.
  That is a real trap — the proposal's own command sends those headers.

**If a step in `image.yml` is the wrong home** — for instance because that
workflow also runs on `main` pushes, where no version tag exists — say so
and propose where it belongs. The requirement is *something runs and asks
the registry*, not *this file*.

---

## 5. Acceptance

1. The GHCR measurement **re-run and quoted**, not taken from this handoff.
2. `NFR-REL-010` reads `Implemented`, with the evidence, the method, and the
   old status kept as a dated correction.
3. **§2's question answered or explicitly declared unanswerable**, with the
   places looked named. No sentence about `0.53.0` that the record does not
   support.
4. Appendix A's `| Release |` row **re-derived from all eleven entries**;
   `spec_scan` green, and stated as green *after* the last edit.
5. The check exists, runs after the push, is **anonymous**, and was
   **planted failing** with the failure quoted.
6. `DEC-007`'s block in full — **696** at `GATE-001`; state the new count
   and what moved it.
7. `CHANGELOG.md` under `[Unreleased] → Fixed`: a reader could have run this
   image for eight releases while the project's own documentation said none
   existed. That is the reader-facing fact; no mention of `*Status*` fields
   or Appendix columns.
8. CI green **per job**, read from the jobs rather than the summary.

**What this round must not do**: touch `NFR-REL-009` or `NFR-REL-011` (both
`ABS-001`'s), use a credentialed token for the public-pullability check,
push a deliberately broken workflow to `main`, or cut a release.
