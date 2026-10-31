# REL-0.60.0 — release candidate

**0.60.0, the cut.** All three work rounds are closed — two of them in
two rounds each — and `§10.51` is closed on a plant rather than a
feeling. `RFC 022`'s rule holds for the whole configuration surface:
**absence degrades; a value the operator supplied and the product
cannot use refuses startup, by name.**

**This cut is not an ordinary one, and `§3` is why.** `0.60.0` starts a
new changelog series, so `DEC-055`'s archive move falls due for the
first time since `0.50.0` — ten versions move out of `CHANGELOG.md`,
verbatim, in their own commit. It is mechanical and it is the item most
likely to be done approximately.

## §0 — whose round this is, in what order, and the one thing it does not wait for

**This is a dev-team round.** `§1`, `§3`–`§7` are its work. `§10` lists
the exit conditions.

**The four `DEC-056` legs are not part of it**, per
`docs/development/changelog-and-releases.md` (lines 59–60). The order:
the dev team does `§1`, `§3`–`§7` and reports; the architect reviews;
the owner approves; the architect runs the four legs and the `§8` asset
check.

**`QS-002` is open and this cut does not wait for it — the architect's
call, made rather than handed up.** `QS-001` landed the front-door fix
and `NFR-REL-007`'s new clause on 2026-11-01; `QS-002`, the `docs_scan`
rule that would enforce that clause, has not started. Holding a release
whose own subject is complete, for a guard on a documentation clause,
inverts the priority — and `0.59.0`'s own `REL` round recorded that a
cut blocking on something pending is the shape to avoid.

**So the clause ships enforced by a reader, and `§7` says so plainly.**
That is `§10.36`'s own class, deliberately accepted for one release and
named rather than left implicit. `QS-002` goes to the next proposal.

**Nothing else is pending.** No owner decision is owed this time —
`§5.1`'s question at `0.59.0` was answered and `NFR-CMP-007` written.

## §1 — the ordinary cut

1. **Version** — `0.60.0` in `[workspace.package]`, nowhere twice.
   Currently `0.59.0` at `Cargo.toml:24`.
2. **`CHANGELOG.md`** — a `[0.60.0]` section, dated. **`[Unreleased]`
   is not empty this time**: it holds `CFG-001`'s, `CFG-002`'s and
   `QS-001`'s own entries. **Read it, and fold its content into
   `[0.60.0]` rather than writing beside it** — `CFG-003` added
   nothing (an internal guard changes no behaviour), which is correct
   and should stay that way.
3. **`ROADMAP.md`** — three edits:
   - `## What ships next`: `0.60.0`'s row comes out, the shipped line
     written. **Word the legs as pending until they are run.**
   - The `—` row's *"beyond `0.59.0`"* becomes `0.60.0`.
   - **`QS-002` joins the candidate list** if it is not already there,
     named as the guard this release ships without.
4. **Release cycle exit criteria 1–8**, each stated with what
   satisfied it. **There is no criterion 9.**

## §2 — the claim

**A value the operator supplied and peisear cannot use stops the
server, by name, naming what would have been accepted — and absence
still degrades exactly as before.**

Stated at the four variables it changed: `COOKIE_SECURE`, `RUST_LOG`,
`SMTP_PORT`, `SMTP_TLS_MODE`. Three already behaved this way
(`PEISEAR_SETUP`, `JWT_SECRET`, `BIND_ADDR`) and `RFC 022` generalised
their shape rather than inventing one; the product now has **one** rule
for this input class where it had two.

## §3 — `DEC-055`: the series archive, due for the first time in ten versions

**`0.60.0` starts the 0.60–0.69 series, so 0.50–0.59 moves out.**
`docs/development/changelog-and-releases.md`'s own rules, which you
should read rather than take from this list:

1. **Create `changelog/0.50-0.59.md`** holding **ten versions**:
   `0.50.0`, `0.51.0`, `0.52.0`, `0.53.0`, `0.54.0`, `0.55.0`,
   `0.56.0`, `0.57.0`, `0.58.0`, `0.58.1`, `0.59.0` — **count them
   yourself from `CHANGELOG.md` rather than from this sentence**, which
   lists eleven section headings for ten minor versions because
   `0.58.1` is a patch. Getting that distinction wrong is the whole
   risk here.
2. **Moved, not copied, and verbatim.** Each version appears in
   exactly one file afterwards. Do not reword, retitle or tidy a
   single section on the way — the archive is the record of those
   releases, and `0.58.0`'s own wrong claim stays wrong there with its
   forward correction intact.
3. **An archive is written once and not edited afterwards.** Match the
   existing five archives' own shape — open `changelog/0.40-0.49.md`
   and follow it rather than inventing a header.
4. **Add the link** to `CHANGELOG.md`'s own archive list, in the same
   descending order the list already uses.
5. **In its own commit**, as `DEC-055` requires — separate from the
   version bump and the `[0.60.0]` section.

**`changelog_scan` enforces most of this and will tell you if you are
wrong** — each version in exactly one file, `CHANGELOG.md` holding only
the workspace version's series, each archive holding one whole series
with no `[Unreleased]`, every archive linked, every relative link
resolving, and **every released version that has a real `git` tag
having a section somewhere**. Run it, and quote what it said.

**What it does not check**: that the move was verbatim. That one is
yours, and `git diff` is how you show it — a moved section should show
as pure deletion in one file and pure addition in the other, with no
line changed in between.

## §4 — `DEC-028`: both specifications, five passes

The five passes in order, per
`docs/development/changelog-and-releases.md:206`. **Pass 5 greps for
the instruction, not the document.** Appendix A recomputed from its own
rows via `spec_scan` rules (a) and (b) — run them, do not re-derive by
hand.

**`external-design.md` has real content this release, not a
"nothing changed" row.** `§11.2 Degradation without SMTP`
(`external-design.md:1743`) has exactly two conditions:

| Condition | Externally observable behaviour |
|---|---|
| SMTP unconfigured | Warning at startup; in-app delivery continues normally |
| SMTP configured, unreachable | Send failure logged; in-app delivery unaffected |

**This release creates a third**: SMTP configured with a value the
product cannot use — the server **refuses to start**, naming the
variable. That is externally observable and it belongs in this table.

**And the sentence under it is now incomplete**: *"No user-facing error
is raised for mail failure; the in-app channel is authoritative."* True
of a *failure to send*; not true of a malformed `SMTP_PORT`, which is
not a mail failure and does stop the product. **Amend it to distinguish
the two**, and keep `0.59.0`'s own check-note above it intact — it
records a different question, already answered.

**Also in `external-design.md`:** the `0.60.0` Appendix C row, the
`(this baseline)` marker moved off `0.59.0`, and the header's
`Covers release` / `Amended` lines brought current. **`§1.1`'s own
entity rows are untouched by this release** — check that rather than
assume it.

**`requirements.md`:**

- **`§10.51` is closed and its entry already carries its own closing
  note**, with both bounds. **Cite it in the baseline row; do not
  restate it.**
- **`NFR-REL-007`** was amended by `QS-001` yesterday, `*Status*`
  unmoved. Cite it; the amendment already says the new clause is
  unenforced.
- **`RFC 022` is accepted and is this release's governing document** —
  check whether any `FR-*`/`NFR-*` entry should name it, and **if none
  should, say so rather than leaving the question unasked.**
  `NFR-SEC-*` is the likely neighbourhood for `COOKIE_SECURE`'s
  refusal.
- Appendix C's `0.60.0` row, the marker moved off `0.59.0`, and the
  header current.

## §5 — the gate table, re-measured against the candidate

- **`DEC-007` — 629** at `cb70f01`, measured by the architect;
  re-measure at the candidate. Progression: 601 at 0.59.0 → 609
  (`CFG-001`) → 619 (`CFG-002`) → 620 (round 2) → 625 (`CFG-003`) →
  629 (round 2). Sub-counts to reproduce: `peisear-web --lib` **218**,
  `peisear-notify` **19**, `peisear` **6**. **Count with the runner.**
- `fmt`; `clippy --workspace --all-targets`, both profiles; `clippy
  -p <crate> --no-default-features --features postgres` for the four
  crates in that graph; rustdoc with `.github/CONTRIBUTING.md`'s own
  documented flag (**13 warnings is the standing baseline**, 7
  `peisear-storage` + 6 `peisear-web`); `mdbook` + linkcheck;
  `publish-dry-run`.
- **`docs_scan` rule 8 green on the real tree**, and **say so as a
  measurement** — it is this release's own guard and the release that
  ships it should state that it holds.
- **The local `publish --workspace --dry-run` artefact should
  disappear at the bump.** It has held for four releases. If it
  survives, that is a real finding — report it.
- CI on the candidate: **quote the run's own conclusion and its
  non-success count; do not sum the jobs.** The postgres job's cost is
  part of the record — it has read 14m11s, 13m59s and 18m51s across
  this series; state this candidate's own figure rather than carrying
  one forward.

## §6 — the claims to falsify, with real binaries

**Three, and the first is new because `QS-001` changed the front
door.**

1. **`README.md`'s own first command works, exactly as written.** The
   Quick Start now leads with download-and-run, and
   `NFR-REL-011` requires every documented install path to be
   exercised end to end. Take the archive from the `0.59.0` Release,
   extract it with the README's own `tar` line, run `./peisear` in an
   empty directory, reach `http://localhost:3000`, register. **Use the
   published archive, not a local build** — the point is the path a
   reader takes.
2. **An upgrade is a non-event for a configured instance.** Install
   `0.59.0` from crates.io into a scratch root, register, create a
   project, keep the cookie. Run this candidate against the same data
   directory. **Assert**: the migration count does not move; the
   0.59.0-issued cookie still authenticates. **Do not assert
   byte-identical startup output** — two independently `tokio::spawn`'d
   jobs log their own "started" lines in a racing order, found at
   `0.59.0`, so that property is not provable; assert the lines that
   are ordered, and say which you compared.
3. **The release's own claim, against a real binary.** For each of the
   four variables, with a malformed value set: the server **refuses,
   naming the variable**, and with the variable **unset** it starts
   normally. Eight starts, `COOKIE_SECURE`, `RUST_LOG`, `SMTP_PORT`,
   `SMTP_TLS_MODE`. **Capture the refusal text.** This is the release
   claim; prove it on the artefact, not in a unit test.

**This release adds no migration.** Say so, and show the count rather
than asserting it.

## §7 — what this release does not fix, stated not implied

- **`NFR-REL-007`'s new clause ships enforced by a reader, not a
  check.** `QS-002` is written and open; `§0` records the decision to
  cut without it. This is `§10.36`'s class, accepted for one release
  and named.
- **`RUST_LOG` is fixed but not guarded** — no literal
  `env::var("RUST_LOG")` for `docs_scan` rule 8 to anchor on — and its
  reach is narrow even fixed: `RUST_LOG=dbug` is accepted as a target
  name with an implicit `TRACE` level. Both bounds are in `§10.51`'s
  closing note and `RFC 022` §4.
- **`§10.50`** — no CI job compiles for any platform but Linux outside
  the release tag.
- **`§10.32`** — `BROWSER-001`'s fixture cannot render four branches
  its pages contain.
- **An attended start prompting is unproven on Windows.**
- **`SMTP_TLS_MODE` accepts seven spellings for two modes**, five of
  them undocumented; deliberately not narrowed, on `RFC 022` §5's own
  constraint.

## §8 — the legs, and the check `0.58.0` taught us

`DEC-056`'s four legs are the architect's, on the owner's approval:
signed annotated tag · push tag · `cargo publish --workspace` from a
detached worktree of the tag commit · `gh release create` with the
changelog section as its body and `--verify-tag`.

**Then, before any record says this shipped**: `gh release view 0.60.0
--json assets` and count. **Five archives.** Read the asset list;
verify the crates by per-crate query — `0.59.0` taught that the publish
log batches crates onto one line, so six `Published` lines covered
seven crates — and the container tags against the registry's own
manifest API.

## §9 — escalate rather than deciding

- If `changelog_scan` objects to the archive in a way `§3` does not
  predict.
- If `§4`'s `§11.2` amendment looks like it needs a `§17` divergence
  entry, or if an `FR-*`/`NFR-*` status looks like it should move.
- If `publish --dry-run` survives the version bump.
- If `§6.1`'s README walk fails — that is a defect in a path shipped
  yesterday, and it outranks this round's own subject.

## §10 — exit conditions

1. Version bumped; `[0.60.0]` written and dated; `[Unreleased]`'s
   three entries folded in, **read first**.
2. `ROADMAP.md`'s three edits, legs worded as pending.
3. **`§3`'s archive move: `changelog/0.50-0.59.md` created with the
   right versions, moved verbatim, linked, in its own commit**, with
   the verbatim-ness shown by diff and `changelog_scan` quoted.
4. `DEC-028`'s five passes; **`§11.2`'s third condition written and
   the sentence under it amended**; both headers and both Appendix C
   rows current; `§10.51` and `NFR-REL-007` cited not restated; the
   `RFC 022` status question answered either way.
5. `§5`'s gate table re-measured at the candidate, counted with the
   runner, rule 8's own green stated, CI's conclusion quoted, the
   postgres job's figure from this run.
6. `§6`'s three falsifications walked with real binaries — including
   eight starts with captured refusal text — and the migration count
   shown.
7. `§7`'s six limits in the candidate's own words, the `QS-002`
   decision among them.
