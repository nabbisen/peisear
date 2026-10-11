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

**Added 2026-10-11, owner-raised, `RFC 0016` (accepted):** `README.md`'s
hero, overview, "Why peisear?" and feature list describe a self-hosted issue
tracker. The in-repo normative requirements (§1.1) describe a service whose
distinguishing purpose is keeping *"a project — and the people in it —
sustainable"*, and the README mentions health, sustainability, privacy,
`/today`, WIP, capacity and sprints **zero times each** — all shipped, two of
them visible in the hero screenshot the README now carries. **The
repositioning is 0.50.0's**, beside `DOCS-006`; the brand string and the four
published one-line descriptions went into 0.49.0 with `DOCS-008`, because the
release publishes them.

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
