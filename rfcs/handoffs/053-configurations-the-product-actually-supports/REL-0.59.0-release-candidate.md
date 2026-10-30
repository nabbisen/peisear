# REL-0.59.0 — release candidate

**0.59.0, the cut.** All four work rounds are closed. `SUP-001` closed
`§10.48`, `SUP-002` ran 33 of 33 integration binaries against a real
`postgres:17`, `SUP-003` documented the one install path that supports
PostgreSQL, `SUP-004` documented all seven `SMTP_*`. `RFC 021`'s four
conditions are met for both configurations, and `RFC 021` §5 — end the
backend — is off the table on `SUP-002`'s evidence.

**This cut has a second subject, and it is not a formality.** Assembling
it found that **both specifications' own record of what they cover has
drifted** — one of them has not been amended at all for two releases.
`§3` is that finding and its repair; `§4` is a section of
`external-design.md` that **this release makes wrong**, which is the
amendment `DEC-028` exists for. Neither is padding, and neither may be
absorbed into "the five passes were run".

## §0 — whose round this is, and in what order

**This is a dev-team round.** `§1`, `§3`–`§8` are its work. `§11` lists
the exit conditions, one per section.

**The two owner answers arrive with the authorization, not mid-round.**
`§5.1` and `§5.2` each need a decision that is the owner's; both are in
front of them now, each with the architect's recommendation. **Do not
start `§5` without them, and do not choose on the owner's behalf** —
`§10` is the escalation route. Nothing else in this round waits on
anyone.

**The four `DEC-056` legs are not part of it.** Tag, push tag,
`cargo publish --workspace` and `gh release create` are the architect's,
on the owner's approval, per
`docs/development/changelog-and-releases.md` (lines 59–60: *"The
architect, on the owner's approval… The dev team never runs it."*). The
order:

1. the dev team does `§1`, `§3`–`§8` and reports;
2. the architect reviews;
3. the owner approves the cut;
4. the architect runs the four legs and the `§9` asset check.

## §1 — the ordinary cut

1. **Version** — `0.59.0` in `[workspace.package]`, nowhere twice.
   Currently `0.58.1` at `Cargo.toml:24`.
2. **`CHANGELOG.md`** — a `[0.59.0]` section, dated. **Confirm
   `[Unreleased]` is empty first, by reading it** (it was at
   `475c961`; read it again rather than trusting this line).
3. **`ROADMAP.md`** — three edits, not one:
   - `## What ships next`: `0.59.0`'s row comes out and the shipped
     line is written. **Word the legs as pending until they are run** —
     `0.56.0` through `0.58.1` each carried that caveat, and `0.58.0`'s
     own first attempt at this paragraph was titled for a release that
     had not happened.
   - The `—` row still reads *"Nothing else is authorized beyond
     `0.58.1`"*. After this cut that sentence names the wrong release.
   - **`## Unscheduled work`'s first row is done.** *"Seven `SMTP_*`
     variables documented nowhere operator-facing"* was `SUP-004`'s
     subject; `docs/getting-started/configuration.md` now names all
     seven. The row comes out, and the shipped line is where it goes.
     The table's other two rows (calendar `POST` landing, the 117
     uncited entries) are untouched — leave them.
4. **Release cycle exit criteria 1–8**, each stated with what satisfied
   it. **There is no criterion 9.**

## §2 — the claim

**PostgreSQL is a configuration peisear says it supports, and now
means it; and an operator who wants email can read about all seven
variables that configure it.**

Stated at `RFC 021` §3's own four conditions, because that is the rule
this release was scoped against and the only honest way to say it:
reachable from `install.md`, variables documented where configuration
is documented, CI exercising the same 33-binary population as the
default, and a declared version range of one tested version —
`postgres:17`, said as one rather than implied as many.

## §3 — both specifications' own record of themselves has drifted

**This is a defect in a published, normative document, found by the
architect while scoping this cut, and the accounting belongs in the
release.** Measured at `475c961`:

| Where | What it says | What is true |
|---|---|---|
| `requirements.md:4` | **Covers release**: `0.53.0` | six releases back |
| `requirements.md:23` | **Amended**: 2026-10-15 (0.53.0) | last amended 2026-10-31 |
| `requirements.md:7901` | Appendix C's top row: `**0.56.0 (this baseline)**` | no rows for 0.57.0, 0.58.0, 0.58.1 |
| `external-design.md:4` | **Covers release**: `0.53.0` | six releases back |
| `external-design.md:12` | **Amended**: 2026-10-15 (0.53.0) | last amended 2026-10-24 |
| `external-design.md:2306` | Appendix C's top row: `**0.57.0 (this baseline)**` | no rows for 0.58.0, 0.58.1 |

**Three different answers to "which release does the specification
cover", across two documents that are the normative source of truth.**

**And `external-design.md` was not amended at 0.58.0 or 0.58.1 at
all** — `git log` gives its last commit as `692c20f`, the 0.57.0 cut.
`DEC-028` requires *both* documents amended at each release and that
the candidate not tag until they are (`docs/specification/README.md:12`,
`docs/development/changelog-and-releases.md:192`). Every release from
0.42.0 to 0.57.0 has an `Amended:` header bump in that file's history;
the streak ends at 0.53.0 for the header and at 0.57.0 for the document.

**Why it was not visible.** `0.58.1`'s own candidate searched
`external-design.md` for the release's new identifiers, found zero
hits, and concluded no amendment was owed. The search was correct and
the conclusion does not follow: **Appendix C's row is owed by the
release happening, not by the content changing.** `0.54.0` through
`0.57.0` each wrote one saying *"No screen, route, state or copy
changed"* and explaining why. **A missing row and a row that says
nothing changed are different facts, and nothing in this project can
tell them apart** — which is `§10.36`'s own class, and is recorded
there as instance 7 rather than given a new number.

**Whose it is.** Mine. I wrote the `§4` that named both documents in
both candidates, reviewed both rounds, and accepted both without
checking that the second document had been touched. `requirements.md`
got its entry-level amendments every time; what lapsed is the index of
them.

**Why this is not a round of its own**, against the series' own
standing exception: the repair *is* `DEC-028`, the cut's own criterion,
in the two files the cut must edit anyway. A separate round would amend
these documents and then the cut would amend them again. The exception
exists so a defect is not buried inside a round scoped for something
else; `§3` and `§11`'s own conditions are how it stays unburied.

**The repair:**

1. **Both headers** to `0.59.0`, both `Amended:` dates to the cut's.
   `requirements.md`'s `Covers release` carries a release summary in
   parentheses; keep that shape, write 0.59.0's.
2. **`requirements.md` Appendix C** gains four rows: 0.57.0, 0.58.0,
   0.58.1 and 0.59.0, the top one marked `(this baseline)` and the
   old marker on 0.56.0 removed.
3. **`external-design.md` Appendix C** gains three: 0.58.0, 0.58.1 and
   0.59.0, same marker move.
4. **The three backfilled rows say they are backfilled**, with the date.
   Do not write them as though they had been written at their own cuts
   — that is the one thing this document's own 0.21.0 amendment note
   refused to do, and it is right: *"An external design document that
   lags the product stops being a specification and becomes a second,
   unreliable description of it."*
5. **Write them from the releases' own `CHANGELOG.md` sections and
   register entries**, not from memory. 0.58.0 and 0.58.1 are the
   wizard and its predicate; by `0.53.0`'s own precedent in that same
   table — *"terminal output at startup, not a screen"* — the
   `external-design.md` rows are very likely *"nothing for this
   document to amend"* **with the reason stated**. Reach that
   conclusion from the document's own scope sentence and say so; do
   not assume it because this paragraph does.

**Nothing here is a guard.** The remedy — a `spec_scan` rule reading
both headers and both Appendix C top rows against the version being
cut — is a candidate on `ROADMAP.md`, not this round's work. `spec_scan`
today has four rules (`§10.N` citations, the four-digit `RFC` form,
Appendix A's arithmetic, Appendix A's census) and **none reads a
document header**; that is measured, at `crates/peisear-web/src/spec_scan.rs`.

## §4 — `§1.1` is wrong, and this release is what made it wrong

**`external-design.md §1.1` describes a product with one database, and
says so three times:**

- the prose at `external-design.md:179–181` — *"no service-to-service
  dependencies at runtime beyond an optional outbound SMTP relay"*;
- the boundary diagram at `:183–203`, whose only datastore box is
  `SQLite (single file)`;
- the external-entities table at `:210` — `| SQLite file | out | file
  I/O | Single file — … → NFR-CMP-003 |`.

**0.55.0's own Appendix C row stated the condition under which that was
acceptable**, and this release removes it:

> *"The feature stays off by default — an operator who does not select
> it sees nothing this document would need to describe differently."*

`SUP-003` wrote `docs/getting-started/install.md:61` — *"Running
against PostgreSQL instead"* — with a real `cargo install
--no-default-features --features postgres` command and a
`DATABASE_URL` in PostgreSQL's form. **An operator can now follow this
project's own getting-started documentation to a running instance whose
boundary is not the one `§1.1` draws**, and on that path the database
is a service the process did not start and does not own, which is a
service-to-service dependency.

**This is `RFC 021`'s own defect one document up.** The RFC's negative
half says operations documentation may not exist for a configuration
that fails the rule. Here it is the reverse and worse: the
**normative design document** describes a single-backend product while
getting-started explains how to run the other one. Same shape, higher
stakes, and it is this release's own doing rather than an inheritance.

**What to write.** `§1.1` describes both backends, with the default
named as the default and the second named as what it is — selected at
build time, by the one documented install path. The diagram shows one
datastore position with two possibilities rather than two boxes, if
that reads more cleanly; the entity table gets a row per backend, or
one row that names both. **The shape is the round's to choose and to
justify in its report**; what is not optional is that a reader of
`§1.1` can no longer conclude the product has one backend.

**Two more places in the same document, to check rather than assume:**

- **`§11.2 Degradation without SMTP`** (`:1720–1725`) — the row reads
  *"SMTP unconfigured | Warning at startup; in-app delivery continues
  normally"*. `SUP-004` changed what that warning says: it no longer
  names four variables, it names none and points at the documentation
  page. The row does not quote the message, so it may need nothing —
  **answer it either way, in writing.**
- **`§1.1`'s SMTP entity row** (`:209`) — *"Optional. Absence degrades
  gracefully → `FR-NTF-008`"*. Accurate, and incomplete in exactly the
  way `§10.51` names: absence degrades, and a malformed present value
  is discarded just as quietly. **A citation to `§10.51` is the whole
  of what is owed here, if anything is**; do not write the fix, which
  is unscheduled.

## §5 — `DEC-028`: five passes, and two decisions that are the owner's

The five passes in order, per
`docs/development/changelog-and-releases.md:206`. Passes 1–4 over both
documents, pass 5 over the documents a reader outside this repository
meets. **Pass 5 greps for the instruction, not the document.**

**Appendix A recomputed from its own rows** — read from `spec_scan`
rules (a) and (b); run them, do not re-derive by hand.

**Entries this release touched, at minimum** — this list is a floor,
not the population:

- **`§10.48`** — closed by `SUP-001`, its entry already carrying its
  own closing note and the decayed-evidence finding. **Cite it in the
  baseline row; do not restate it.**
- **`§10.50`**, **`§10.51`** — both open, both unscheduled. The
  baseline row names them as what the release did not fix.
- **`NFR-REL-011`** — amended by `SUP-003`, status unchanged.
- **`NFR-CMP-004` / `§6.5`, measured and both stale.** `§6.5 Migration
  inventory at this baseline` lists `0001`–`0015`; `NFR-CMP-004`'s
  status reads `Implemented (0001–0015)`. At `475c961` there are **22**
  migrations under `crates/peisear-storage/migrations/` and **23**
  under `migrations-postgres/`, the extra one being
  `0023_rowid_surrogate_for_ordering`. **Pre-existing drift, and this
  release's business**: `§6.5` is where a PostgreSQL operator would
  look to learn what schema they get, and the specification records
  nowhere that the two backends' migration sets differ. Amend both,
  and state the asymmetry and its reason rather than only the numbers.
- **`§9.1 Test inventory at this baseline`** is explicitly marked
  *"Measured per target on 2026-08-27 at `0.28.0`, not carried
  forward"* — that caveat is honest and **stays**. Do not refresh it.

### §5.1 — owner's decision: where PostgreSQL's persistence requirement lives

`NFR-CMP-003` reserved this for the owner on a named trigger — *"the
day a PostgreSQL install path is documented"* — and that day is
2026-10-30, `SUP-003`.

**The entry carries two shapes for the same content, one day apart,
and they have never been put side by side** — which is why this is
still open:

- **(a)** the 0.56.0 amendment's proposed **clause inside
  `NFR-CMP-003`**, reading *"the operator's own instance, backed up by
  the operator's own means (`pg_dump`, WAL archiving, or a managed
  provider's own snapshot), and `docs/operations/backup.md` MUST say
  so"*;
- **(b)** the `*Open at 0.56.0*` paragraph's proposed **sibling entry
  `NFR-CMP-007 — Persistent state on the PostgreSQL backend`**,
  proposed to the owner 2026-10-21, with fuller text that also
  requires `backup.md` to state that no generated signing-key file
  exists on this backend.

**The architect recommends (b), the sibling entry — and this reverses
the recommendation I gave the owner on 2026-10-30.** I had read the
clause and not weighed it against the sibling proposal in the same
entry. The reasons for (b):

1. `NFR-CMP-003` is titled **"Single-file database (SQLite backend)"**
   and its 0.56.0 amendment scoped it to SQLite *deliberately*. A
   PostgreSQL clause inside it contradicts its own title, which is the
   confusion the owner's standard exists to prevent.
2. One entry per backend, each titled for what it governs, is the
   simpler rule. `NFR-CMP-003` stops growing; it is already carrying
   four amendment notes.
3. (b)'s text is the more complete one — it covers the signing-key
   asymmetry, which is real and which (a) omits.

If (b): `NFR-CMP-007` is written, and `NFR-CMP-003`'s `*Open at
0.56.0*` paragraph **closes** by pointing at it. If (a): the clause goes
in as written and the sibling proposal is withdrawn **by name**, so it
is not rediscovered. **Either way both proposals stop being open**, and
`external-design.md:210`'s `→ NFR-CMP-003` citation follows the answer.

### §5.2 — owner's decision: `§10.51`

`§10.51` — `SMTP_PORT` and `SMTP_TLS_MODE` each discard a malformed
value silently, where `PEISEAR_SETUP` refuses one by name — is **open
and unscheduled**, opened by the architect's review of `SUP-004` on
2026-10-31.

**The architect's recommendation is that it stays unscheduled for this
release and is named in it.** The repair is a design call with a
security dimension (refuse, warn, or document the current behaviour),
`SUP-004` was right not to touch channel behaviour, and this release is
at the cut. **What the owner decides is whether it gets a `ROADMAP.md`
row next**, which comes with the architect's next proposal rather than
here.

**This round's only obligation is `§8`**: name it as something the
release does not fix. Do not fix it.

## §6 — the gate table, re-measured against the candidate commit

- **`DEC-007` — 601** at `475c961`, measured during `SUP-004`'s review;
  **re-measure at the candidate.** Progression: 586 at 0.58.0, 601 at
  0.58.1, unchanged across all four `SUP` rounds. Sub-counts I
  measured and you should reproduce: `peisear-web --lib` **205**,
  `peisear-notify` **7**. `peisear-i18n` is **17 under its own
  invocation and 0 under `--lib`** — the per-crate block in
  `.github/CONTRIBUTING.md` lines 96–115 is the authority. **Count with
  the runner, never a grep.**
- `fmt`; `clippy --workspace --all-targets`, both profiles; `clippy`
  under `--features postgres` for all four crates that carry it —
  **and now also under `--no-default-features --features postgres`**,
  which is the combination `install.md` actually tells an operator to
  build and which `SUP-003` found is required rather than optional.
  State whether `test.yml` already covers that combination; if it does
  not, say so rather than adding a job in a cut.
- rustdoc with `.github/CONTRIBUTING.md`'s own documented flag, not a
  stricter one invented here; `mdbook` + linkcheck; `publish-dry-run`.
- **The local `publish --workspace --dry-run` artefact should disappear
  at the bump** — it fails locally at an unchanged version because the
  sandbox's registry cache resolves a stale `peisear-web@0.58.1`. The
  same prediction held at 0.58.0 and 0.58.1. **If it still fails after
  the bump, that is a real finding — report it.**
- **The postgres job's cost is part of the record**: `SUP-002`
  measured **+6m09s**. Re-state it from the candidate's own run rather
  than carrying that figure forward.
- CI on the candidate: **quote the run's own conclusion and its
  non-success count; do not sum the jobs.** One `SKIPPED` (`Deploy to
  GitHub Pages`) is correct on a PR and is not a failure.

## §7 — the claims to falsify, with real binaries

**Two, because this release has two configurations.**

1. **An upgrade is a non-event for a configured SQLite instance.**
   Install `0.58.1` from crates.io into a scratch root, register a real
   account, create a project, keep the session cookie. Run this
   candidate against that same database. **Assert**: the migration
   count does not move; the 0.58.1-issued cookie still authenticates;
   startup output for a configured instance is byte-identical to
   0.58.1's. **This release adds no migration** — show the count rather
   than asserting it.
2. **`install.md`'s PostgreSQL section reaches a running instance, from
   outside the checkout, exactly as written.** `SUP-003` walked this in
   CI; walk it again against **the published candidate rather than the
   branch**, because `RFC 021` condition (1) is about what a reader who
   has read nothing can do. Copy the command out of the document —
   including `--no-default-features` — and record what happens if it is
   omitted, which `SUP-003` found silently rebuilds the dual-driver
   binary `SUP-001` eliminated. Register, reach an authenticated
   `/projects`.

**And one negative check**: the three SQLite install paths in
`install.md` are unchanged by this release. Confirm one of them still
works rather than assuming the PostgreSQL section did no harm.

## §8 — what this release does not fix, stated not implied

Each of these is real, open and named here so the `[0.59.0]` section
can carry it in the release's own words. **None is a footnote.**

- **`§10.51`** — `SMTP_PORT` and `SMTP_TLS_MODE` discard a malformed
  value silently, and because the TLS mode derives from the port, a
  malformed port silently changes transport security. Opened by this
  release, unscheduled, and the only one of these the release itself
  created the conditions for noticing.
- **`§10.50`** — no CI job compiles for any platform but Linux outside
  the release tag. All 47 `test.yml` jobs are `ubuntu-latest`,
  measured.
- **An attended start prompting is unproven on Windows.** Five
  diagnosed attempts, the account in `release.yml`'s own comment.
- **PostgreSQL's declared range is one version.** `postgres:17`, which
  is what CI exercises and what `backup.md`'s commands were proven
  against. Honest, and narrower than the ecosystem.
- **The first-run wizard writes a SQLite `.env` only** —
  `install.md:87` says so. A PostgreSQL operator writes configuration
  by hand, which is a documented gap rather than a hidden one.
- **The container image's wizard is still proven on a runner's pty and
  in CI, not inside the published image.**

## §9 — the legs, and the check `0.58.0` taught us to run

`DEC-056`'s four legs are the architect's, on the owner's approval:
signed annotated tag · push tag · `cargo publish --workspace` from a
detached worktree of the tag commit · `gh release create` with the
changelog section as its body and `--verify-tag`.

**Then, before any record says this shipped**: `gh release view 0.59.0
--json assets` and count. **Five archives, not four.** 0.58.0 shipped
four while its record was one template sentence away from claiming
five; 0.58.1 was verified at five by reading the asset list. Read it
again. Verify the crates by per-crate query and the container tags
against the registry's own manifest API, not against the workflows'
exit codes.

## §10 — escalate rather than deciding

- **If `§5.1` or `§5.2` has not been answered when you reach it.**
  Stop at that item and report; do the rest.
- If a backfilled Appendix C row in `§3` would require claiming
  something about 0.58.0 or 0.58.1 that the `CHANGELOG` and the
  register do not support.
- If `§4`'s `§1.1` rewrite turns out to need a `§17` divergence entry
  — that is a specification question, not release mechanics.
- If `publish --dry-run` survives the version bump.
- If the `--no-default-features --features postgres` clippy
  combination is red for a reason this release introduced.

## §11 — exit conditions

One per section, each with a stated measurement rather than an
assertion.

1. Version bumped; `[0.59.0]` section written and dated;
   `[Unreleased]` confirmed empty **by reading it**.
2. `ROADMAP.md`'s three edits made — shipped line with the legs worded
   as pending, the `—` row's release number corrected, and the
   `Unscheduled work` SMTP row removed.
3. **`§3`'s repair complete and its accounting in `[0.59.0]`**: both
   headers at `0.59.0`, both Appendix C markers moved, seven rows
   written in total, the three backfilled ones marked as backfilled and
   dated, and `0.58.0`/`0.58.1`'s `external-design.md` conclusion
   reached from that document's own scope sentence and stated.
4. **`§4`'s `§1.1` no longer describes a single-backend product** —
   prose, diagram and entity table all addressed, the chosen shape
   justified, and `§11.2` and the SMTP entity row answered either way
   in writing.
5. `DEC-028`'s five passes run; Appendix A recomputed from `spec_scan`
   rules (a) and (b); `§10.48` cited rather than restated;
   `NFR-CMP-004` and `§6.5` amended with both migration counts and the
   asymmetry named; `§9.1`'s own caveat left alone.
6. `§5.1` written as the owner answered it, and both proposals in
   `NFR-CMP-003` closed — the chosen one adopted, the other withdrawn
   by name.
7. `§6`'s gate table re-measured at the candidate, counted with the
   runner, CI's own conclusion quoted, the postgres job's cost
   re-stated from this candidate's run.
8. `§7`'s two falsifications walked with real binaries, plus the
   negative check, with the migration count shown.
9. `§8`'s six limits stated in the candidate's own words, `§10.51`
   among them and not fixed.
