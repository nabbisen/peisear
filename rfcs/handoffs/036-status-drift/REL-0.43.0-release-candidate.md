# REL-0.43.0 — release candidate

**Contents**: the identity newtype — `PRIV-002` (`259320e`), `PRIV-003`
(`d720db1`), `PRIV-004` and its round 2 (`a3650b9`, `6417ca6`), under
`DEC-058`/RFC 0014 with the `DEC-058` correction (`38e0f8c`); `DOCS-002`
(`61c7b96`) with the escalation (`7592fbf`) and round 2 (`219cbca`);
`REQ-003` (`fce0d87`) and the `NFR-A11Y-011` renumber (`8222ce7`); and the
specification amendment and its sweep (`5ae64ff`). **Every handoff reviewed
and closed.**

**`DEC-028` is already done** — both specifications cover `0.43.0` (`5ae64ff`),
including §3's new sweep. **Depends on**: nothing outstanding.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only.

**No migration** — `0020` remains the most recent, the **fourth** release
running.

**Expected: 367, and it is two fewer than 0.42.0's 369.** That is deliberate
and `PRIV-002` recorded it at the time: `untrusted_id_scan.rs` (212 lines, two
assertions) was **deleted**, because the property it approximated with text
patterns is now held by the type system everywhere rather than where the scan
happened to look. I verified the drop independently — 342 test attributes at
the `0.42.0` tag, 340 at `HEAD`, and the two missing are
`path_extracted_user_id_only_reaches_require_self` and
`every_path_extraction_binds_the_name_user_id`, both from that file.

**A falling test count is the one number a reader will read as a regression.**
Say in the changelog that the suite went 369 → 367 *and why* in the same
breath. Do not round it, do not omit it, and do not describe it as "unchanged".

**The overflow gate is 120 cells**, untouched by every handoff in this release
and not exercised by any of them. `test.yml`'s own comment described it as
*eighteen* pages; it holds **24** (20 explicit keys plus four `calendarUrls`),
and the comment is corrected in `5ae64ff`.

**One new CI job**: `rustdoc-links`, from `DOCS-002`. The local command is in
`.github/CONTRIBUTING.md` under **Doc links**; run it from there.

## 2. The changelog

**This release has two halves and they pull in opposite directions** — one
closes a long privacy thread, the other reports that the record was wrong
about a P0. Write both. Do not let the second hide behind the first.

1. **Lead with identity becoming a type.** `NFR-PRIV-005` reaches **Met**:
   personal-data storage now refuses anything but a cryptographically-verified
   identity **at compile time**, across all 36 functions that can take one.
   The sentence that makes this concrete for a reader is that
   **`user_id: &str` returns zero results across all six personal-data
   modules** — it was 11 of 36 still on `&str` after `PRIV-002`, and
   `PRIV-003`'s second sealed type closed the subject-with-no-requester gap.
   Say that the boundary it protects **already held** — two independent
   barriers, measured twice — so what changed is that **it can no longer be
   broken by inattention**, not that a hole closed. That distinction is the
   whole value of the release and it is easy to overclaim.
2. **The four that remain cannot close, and say so.** Three are the
   authentication path, where no caller identity exists yet because
   establishing one is what the call does; one is a job-side aggregate over all
   users. A reader who sees "36 of 40" and no explanation assumes four were
   missed.
3. **`DOCS-002`: ten broken links in the published API documentation, and the
   gate that would have caught them.** `docs.rs` builds these seven crates, so
   each unresolved link was live in the API docs of every released version.
   The honest framing is **nothing had ever looked**: `clippy --all-targets`
   does not check doc links and `cargo doc` was in no gate. Mention that the
   architect's own first count was wrong twice (nine, then 23 "issues") because
   it came from a run that **aborted before reaching `peisear-web`** — the
   release's own best argument for `--keep-going`.
4. **`FR-DM-002`, a P0, was recorded complete over half of what it governs.**
   Keyboard parity was marked *"Implemented for both shipped surfaces"* while
   **four** direct-manipulation surfaces ship — the sprint-planning drag since
   0.35.0 and the calendar block drag since 0.36.0. Neither has a keyboard
   test; `calendar.rs` has no form, button or select at all. `FR-DM-006`
   (undo) is in the same position. **Both are now `Partial`.** Write this as
   what it is: *the surfaces may well be fine — what is established is that
   nobody checked.* Do not write it as a newly-discovered inaccessibility, and
   do not bury it.
5. **The sweep that found it**, and `REQ-003`'s answer. Four text rules were
   tested against six historical instances and **none reached the bar**; the
   best scored 4 of 6 and the dev team argued it down because its zero false
   positives were measured **on zero opportunities**. So the remedy is a
   four-pass procedure in `docs/development/changelog-and-releases.md`, run by
   hand at each candidate, and its first run found **six** things — three stale
   entries and three stale rows of Appendix A, one of which was wrong by two
   whole requirements.
6. **`NFR-REL-008`** is new: no requirement governed the published API
   documentation, which is the project's only artefact it publishes without
   authoring. **`§10.33`** and **`§10.34`** open the register.
7. **A requirement identifier was used twice** — already under `[Unreleased]`,
   keep it.

**Do not write a `### Highlights` heading unless you want one.** There is no
such rule; the mandate was withdrawn before it took effect, and
`changelog-and-releases.md`'s own table said otherwise until `5ae64ff`.

## 3. Verification before the candidate commit

- **`DEC-028`: already done** (`5ae64ff`). Confirm both specifications say
  `0.43.0` and say so in the report.
- **The specification sweep is now a gate item of this section.** It was run
  for this release by the architect and its six findings are in `5ae64ff`.
  **You are not asked to re-run it** — confirm that the entries this release
  touched carry 0.43.0 amendments, and report *swept, nothing further found*
  or what you found. A pass with no output is indistinguishable from a pass
  not run.
- `DEC-007` **367**, three consecutive runs. Block and `test.yml`'s test jobs
  unchanged. **Report the figure as 367 with the two-test explanation beside
  it**, so the number never travels alone.
- **The overflow gate on the extracted tree: 120 cells.**
- **The new `rustdoc-links` gate from the extracted tree**: the command as
  `.github/CONTRIBUTING.md` writes it, exit 0, and confirm the log shows
  **seven** `Documenting` lines. A run that documents six crates and exits 0
  is the failure this gate exists to prevent.
- **`mdbook build` from the extracted tarball** — third release with the site.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`.
- **No migration to exercise**, but start the candidate on a database built by
  the released **0.42.0** binary and confirm it comes up at 20 with nothing
  applied.
- `cargo publish --workspace --dry-run` from a detached worktree; tarball
  sha256 and the `git ls-tree` comparison.
- **One behavioural check for this release in particular**: sign in and load
  `/me`, `/inbox`, `/settings/notifications` and a team's sprint page on the
  built artefact. Nine handlers had their identity parameter retyped; the
  reviews verified the SQL byte-identical, but **no reviewer has loaded the
  pages** since. Ten seconds each.

## 4. Escalate rather than deciding

- **If the count is not 367**, or the gate is not 120.
- **If the `rustdoc-links` log shows fewer than seven crates** even at exit 0.
- **If `mdbook build` fails from the tarball.**
- **If any of the four pages above renders an error or an empty panel.**
- **If the sweep turns up a seventh stale entry** — report it; do not amend the
  specification yourself. `DEC-028` work in a candidate is the architect's.

## 5. Exit condition

A candidate commit on `main`; the changelog written as a standalone
announcement that carries both halves, with the 369 → 367 drop explained
where it appears; 367 green three times; the gate at 120; `rustdoc-links`
clean with seven crates; the book building from the tarball; the four pages
loading on the artefact; the 0.42.0 database coming up clean; and the dry run
clean. Then stop.

---
**One decision is the owner's, and it is named in the report, not settled
here**: `FR-DM-002`'s P0 measurement — whether 0.43.0 ships with the honest
`Partial` and the measurement opens 0.44.0, or whether the release waits for
it. The architect's recommendation is **ship**: the condition has held since
0.36.0, holding the release does not shorten it, and a release that corrects
the record is the right place for the correction to appear.
