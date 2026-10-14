# Handoffs — documentation that drifted

**Not RFC-governed.** Four audits on 2026-10-11, at the owner's request, over
the documentation a user reads, the documentation a contributor reads, the
three design documents, and the security/privacy posture. **Every finding was
a correction that had landed in a specification and never travelled.**

**One was fixed on the spot and not handed off** (`61e7366`):
`docs/security/hardening.md` stated that `BIND_ADDR=127.0.0.1:3000` is *"the
default-safe choice"*. **The default is `0.0.0.0:3000`** — every interface —
and the same section delegates **TLS termination and rate limiting** to a
reverse proxy that the error makes **bypassable**. One sentence, the correct
wording already in a sibling document, and not worth a review cycle's delay.

**And the sweep gained a fifth pass** (`0424579`), over the documents a reader
outside this repository meets — the remedy all four audits reached
independently.

| ID | Link | What | Release |
|---|---|---|---|
| DOCS-003 | [the start path cannot be followed](./DOCS-003-the-start-path-cannot-be-followed.md) | **Six wrong sentences in three files.** The Quick Start's first command opens a tarball **no release provides** (0 assets, measured); `installation.md` names **rustc 1.85**, which `NFR-CMP-001` records as unable to build the workspace; it also says the stylesheets load **from a CDN** when they are vendored; the first-run walkthrough teaches the **conflict** path as the normal one; the README says **six crates** where seven exist; and the crates.io page's **ten relative links all 404** because `readme` points outside the crate. | 0.49.0 |
| DOCS-004 | [screenshots that regenerate](./DOCS-004-screenshots-that-regenerate.md) | **Owner request.** One image exists in the repository — the logo — for a product whose tagline says *"Good UI"*, and `first-run.md` walks a user through the board **in prose**. The harness already drives 24 pages at 5 widths with real fixtures, so these are **generated, not hand-captured** — a pasted PNG is precisely the artefact that goes stale. Two real decisions named: where they live, and how anyone knows they have gone stale. | 0.49.0 |
| DOCS-005 | [a scan for what a scan can check](./DOCS-005-a-scan-for-what-a-scan-can-check.md) | **Lands last; its rules encode what the other two produce.** Five rules comparing a document against a source of truth: the crate set against `crates/`, the MSRV against `Cargo.toml`, the deployment manifest against `static/`, documented defaults against `config.rs`, and committed citations against `.gitignore`. **Bounded by `REQ-003`**: no rule tries to judge prose, because none can. | 0.49.0 |

**Deferred to 0.50.0, the developer and architecture half:** the deployment
manifest's own fix, the **126** committed citations to gitignored paths (one of
them in a **published crate's** doc comment, on docs.rs), `CONTRIBUTING.md`
never linking to the system that drives all work, the three roles documented
nowhere, `peisear-i18n` missing from both architecture documents that
enumerate crates, and generated contents lists for two specifications totalling
**504 KB with no table of contents between them**.

**Added 2026-10-11, from `DOCS-007`'s review (§6):** the overflow gate's own
`sprint_plan` page sweeps **empty**. `createFixtures` moves every
team-project issue it creates into the *completed* sprint
(`sprint_issues`' single-sprint-per-issue invariant), so the still-planned
sprint's plan page has an empty Backlog and empty Sprint Items at all five
widths — **five of the 120 cells measure a page with no list in it**, which
is `GATE-001` and `GATE-002` repeating. The fix is one or two more
team-project issues left out of the completed sprint; **the fourth
documentation screenshot falls out of it for free**. Held out of 0.49.0
deliberately: populating a never-swept surface can turn the gate red, and
that does not belong inside a release candidate.

**Added 2026-10-11, owner-raised, `RFC 016` (accepted):** `README.md`'s
hero, overview, "Why peisear?" and feature list describe a self-hosted issue
tracker. The in-repo normative requirements (§1.1) describe a service whose
distinguishing purpose is keeping *"a project — and the people in it —
sustainable"*, and the README mentions health, sustainability, privacy,
`/today`, WIP, capacity and sprints **zero times each** — all shipped, two of
them visible in the hero screenshot the README now carries. **The
repositioning is 0.50.0's**, beside `DOCS-006`; the brand string and the four
published one-line descriptions went into 0.49.0 with `DOCS-008`, because the
release publishes them.

**Added 2026-10-11 from `REL-0.49.0`'s review, for 0.50.0:**

- **`§10.36`, a new register entry**: *a check that cannot tell "nothing
  wrong" from "nothing examined."* Three instances in two releases — rule 5
  reading "no git here" as zero citations (found by running the release
  gate, `9dd7900`), rule 6 passing by matching nothing (`11f7704`), and the
  self-path test passing vacuously after a rename (`DOCS-008` §3). Adjacent
  to `§10.17` but distinct: there the assertion is wrong, here the input is
  empty for a reason the check never asks about.
- **`README.md:77`'s PostgreSQL claim**, for `RFC 016`'s repositioning
  round: it calls swapping SQLite for Postgres *"a localised change"* while
  `ROADMAP.md:582` says it *"is larger than the crate-shaping above
  suggests, and needs its own RFC before it is scheduled or promised."* The
  type boundary is clean; the contracts are not — **11 triggers** carrying
  `NFR-CONC-003` and `FR-SUB-001..003`, and **8 `RAISE(ABORT)`** messages
  string-matched by `translate_trigger_error`.

**Added 2026-10-11, found while clearing scratch space:**

- **`.github/CONTRIBUTING.md:21` still says "Install Rust 1.85+".**
  `Cargo.toml` and `installation.md` say **1.88**, and `NFR-CMP-001` records
  that 1.85 cannot build the workspace. `DOCS-003` fixed `installation.md`
  and `docs_scan`'s **rule 2 reads that file only** — so the scan built to
  catch MSRV drift has the drift one document over. **Fix the number and
  extend rule 2 to `CONTRIBUTING.md`**; the rule already knows how to read a
  version claim, it is reading one file too few.
- **A scratch-hygiene convention, written down.** Every round builds under
  `.git-exclude/tmp/<round>-target` via `CARGO_TARGET_DIR` and nothing ever
  removed them: **65 of them, 808 GB apparent**, back to `rel38`, cleared
  2026-10-11 on the owner's instruction. **Add one line to the handoff exit
  conditions and to `CONTRIBUTING.md`'s development setup**: a round's target
  directory is removed when the round closes. Both files are scanned, so the
  edit belongs in a round that runs the gates, not in a drive-by.

**Added 2026-10-11 from `RFC 018` §2, for 0.50.0:**
`docs/operations/scaling.md:91-94` tells an operator the migrations hold
*"no SQLite-specific tricks beyond a few `julianday` calls and
`datetime('now', '-7 days')` modifier strings, both of which have direct
PostgreSQL equivalents."* **Measured: 24 `julianday` uses, 10 `datetime('now'`
modifiers — and the 11 triggers and 8 `RAISE(ABORT)` messages the sentence
excludes, which have no direct equivalent at all.** The same paragraph points
at `ROADMAP.md`'s *"Pluggable backends beyond relational"* (line 792) when it
means *"PostgreSQL backend"* (line 569). **An operator reading this page
today is told the port is smaller than it is** — the third README-family
overclaim about the same work, after `README.md:77` and ahead of whatever the
backend documentation will say.

**Added 2026-10-11 from `RFC 018` §2's security assessment, for 0.50.0:**

- **The threat model has no data-at-rest entry.** 194 lines across
  `docs/security/`, **zero occurrences of "encrypt", "at rest" or "disk"**.
  `hardening.md:11-38` takes three threats seriously and the database file is
  not one of them; *What the operator does* covers TLS, `JWT_SECRET`,
  headers, rate limiting, OS hardening and backups, and not encryption.
  **Add the asset, name volume encryption as the control, and say plainly
  that peisear does not encrypt its own storage.**
- **`main.rs:22-27` logs the DSN at `INFO` with no redaction** anywhere in
  the workspace. Harmless while it is a file path; **a credential in journald
  the day `RFC 018` P3 lands**. One line and a test, worth doing now.

**Added 2026-10-12 from `REL-0.50.0`'s own sweep:** `first-run.md`'s sample
log line shows the tracing target `peisear_web` where the code's own target
is `peisear`. Named by pass 5 and deliberately not picked up there — a
documentation release is next and that is where it belongs.

**Two decisions are the owner's**, recorded here so they are not lost:

1. **Do reviews become committed artefacts?** `.git-exclude/reviewed` is cited
   **52 times** from committed files and is the project's richest design
   record; the cost is repository size and a public audit trail. The present
   state cites a thing the repository neither contains nor explains.
2. **Should the threat model name the authorised colleague?** It lists
   external attackers, shell users and lost tokens — and **eight
   `NFR-PRIV-*` requirements, `DEC-058`'s sealed types and the
   `workload_privacy` crate** exist to defend against a *legitimate teammate*,
   which it never mentions. The defence is built; only the document is silent.
