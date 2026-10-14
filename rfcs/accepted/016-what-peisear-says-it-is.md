# RFC 016: What peisear is called, and what it says it is

**Status**: **Accepted** (2026-10-11) — both decisions taken by the owner;
implementation may begin
**Target**: the brand string and the published one-line descriptions in
**0.49.0** (inside `DOCS-008`, before the screenshots are captured); the
`README.md` repositioning in **0.50.0**
**Raised by**: the owner, 2026-10-11 — *"`README.md` said about themselves at
the top, 'Issue tracking' and 'issue management system'. However, this project
is not just an issue tracker."*
**Related requirements**: `docs/specification/requirements.md` **§1.1**
(Purpose), **§1.2** (core value commitments), **§1.4** (explicitly
non-intended users), `NFR-REL-007` (README ordering), `NFR-LANG-001`/**§1.7**
(prohibited vocabulary)
**Related inputs**: `peisear-0.19.1-requirements-en.md`,
`peisear-0.19.1-external-design-en.md`,
`peisear-development-instructions-v1-v0.1.md` (V2.1),
`peisear-development-instructions-v1-v0.19.md` (V3) — all four read in full
**Last updated**: 2026-10-11

## Summary

**The product's front door describes a different product from the one the
requirements define and the code implements.** `README.md`'s first line calls
it *"Issue tracking that fits in a single binary"*; the in-repo normative
requirements call it *"a self-hostable **project and issue management**
service"* whose *"primary aim is **not** 'getting tasks finished', but
delivering the observation and awareness needed to keep a project — **and the
people in it** — sustainable."*

**And the product's own interface never says its name.** The navbar and every
page title read **"Issue Tracker"**.

This RFC proposes two changes: the interface says **peisear**, and the README
says what the requirements say peisear is for.

## Background — measured, not impressions

### What the requirements commit to

`docs/specification/requirements.md` is normative and in this repository. Its
§1.1 is quoted above. §1.2 adds four commitments that *"constrain every
requirement in this document"*: **does not judge · does not chase · protects
the individual · is understandable**. §1.4 names three uses the product
deliberately refuses, one of which is **manager-led monitoring of members**.

The project's own priority vocabulary (§0.7) defines **P0** as
*"Product-defining. Violating it makes the product something else."* By that
definition this is a P0 surface: the README is where a stranger decides what
the product is.

### What the README says

| Where | Text |
|---|---|
| `README.md:3` | "**Issue tracking** that fits in a single binary." |
| `README.md:18` | "peisear is a self-hostable **issue management system** — projects, issues, a kanban board, and a handful of thoughtful edges." |
| `README.md:25-27` | "Most issue trackers either bloat into swiss-army platforms or thin themselves out into note apps. peisear stays **in the middle** on purpose" |

**The "Why peisear?" paragraph is the exact inverse of §1.1.** It compares
peisear to other issue trackers on **size**, when §1.1's whole claim is about
**purpose**. A reader who believes it concludes that peisear is a smaller
tracker — which is a reason to choose a different one.

### What the README never mentions

Occurrences in `README.md`, counted:

| Term | health | sustain | burnout | privacy | today | sprint | WIP | capacity | overload | notification |
|---|---|---|---|---|---|---|---|---|---|---|
| Count | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **0** |

("team" appears once, in the closing line. "kanban" three times, "issue" six.)

**These are shipped features, not plans.** Six project-health indicators
(`IndicatorKind` in `peisear-core/src/lib.rs:856-863`: Throughput, Staleness,
Activity, BusFactor, LongStale, WipCompliance), `/today`'s personal
sustainability panels, WIP limits and capacity periods, edge-triggered
notifications with an inbox, teams, sprints, the calendar, sub-issues.

**The README already publishes a picture of two of them.** The hero
screenshot added at `DOCS-007` shows the project board's **HEALTH** section
and its **WORKLOAD** chip. The image names features the text does not.

### What the interface says

| Site | Current |
|---|---|
| `crates/peisear-i18n/src/en.rs:205` | `MessageKey::AppBrandName => "Issue Tracker"` |
| `en.rs` × 7 more | page titles: `"Sign in — Issue Tracker"`, `"Projects — Issue Tracker"`, `"{project} — Issue Tracker"`, … |
| `components/layout.rs:181` | renders the brand in every authenticated page's navbar |
| `components/auth.rs:15` | renders it on sign-in and register |

**A user who installs peisear runs a product called "Issue Tracker".** The
name appears on no screen and in no window title. Someone who wants to search
for help, file a bug, or tell a colleague what they are using has not been
told what it is called.

Blast radius of changing it, measured: **eight string literals in one file**,
two render sites, and **zero tests assert the literal text**
(`git grep "Issue Tracker"` over `crates/*/tests` and `crates/*/src` outside
`en.rs` returns nothing).

### The one surface that already got it right

`docs/src/assets/logo.png` is **a hearth — a lit fireplace with people
gathered around it**, in warm colours, over the wordmark *Peisear*. Not a
checklist, not a board, not a ticket. **The logo, the requirements, and the
shipped features all describe the same product. Only the words disagree.**

## Why this is not a wording preference

§1.4 names the users the product refuses to serve. A README that presents
peisear as a small self-hosted issue tracker **mis-sorts its readers in both
directions**:

- Someone who wants a minimal tracker arrives, finds health indicators and
  burnout signals, and reads them as bloat — the opposite of the
  Minimal-by-Default pillar they were actually built under.
- Someone who needs exactly what peisear uniquely offers — *how is this
  project trending, am I overloaded, and who can see that about me* — never
  looks, because nothing on the page suggests the product has an opinion
  about it.

The second is the real cost. The requirements call these *"two questions
[ordinary trackers] make invisible until they become crises"*, and the one
place a stranger could learn that peisear answers them is silent.

## Proposal 1 — the interface says `peisear`

Change `AppBrandName` to **`peisear`**, and the seven title suffixes to
`"… — peisear"`.

**Why the bare name, with no descriptor**: it is what the product is called
everywhere else — the crate, the command, the repository, the documentation,
the logo's own wordmark. A navbar that says something else makes the running
instance unidentifiable to the person running it. One string, no cleverness;
the owner's rule that simple rules are preferred applies directly.

**Casing**: the specifications and the prose write **peisear** lowercase; the
logo wordmark and `README.md`'s `<h1>` write *Peisear*. The proposal is
lowercase `peisear` in the interface, matching the crate name and the
specifications, and this RFC does not touch the logo.

## Proposal 2 — the README says what the requirements say

Rewritten hero, overview, why, and features, keeping `NFR-REL-007`'s required
ordering. **Draft, for the dev team to refine against §1.7's vocabulary table
— no "velocity", no "performance", no achievement framing:**

**Hero.**

> **Project and issue tracking that also shows how the project — and the
> people in it — are holding up.**
> Sophisticated. Solid. Really easy. Good UI.

**Overview.**

> peisear is a self-hostable project and issue management service for
> individuals and small teams. It does the ordinary things — projects,
> issues, sub-issues, a kanban board, teams and sprints — and then answers
> two questions ordinary trackers leave invisible until they become crises:
> **how is this project trending?** and **am I overloaded?**
>
> It answers them quietly, and without turning the answer into a judgement.
> Personal sustainability data is visible only to the person it describes —
> a team administrator holds a management role, not an oversight role.
>
> One Rust binary, one SQLite file, no Node toolchain, no external services.

**Why peisear?** — replace the size comparison:

> Most trackers can tell you what is left to do. Few can tell you that one
> person is carrying the project, that three issues have not moved in a
> fortnight, or that you have been over your own capacity for a week — and
> fewer still will tell *you* first.

The three **"You own your data / uptime / roadmap"** bullets stay; they are
the self-hosting argument and they are good. They are simply not the
positioning.

**Features** — the list currently has five bullets, four of which describe
the implementation (auth, SSR, one-binary deploy, workspace layout). Add the
half that is missing, each one shipped and verifiable:

- **Project health in plain language** — six indicators; a non-healthy one
  explains itself in a sentence rather than a number (`FR-HLT-005`).
- **A dashboard that is yours alone** — `/today`'s load, rhythm and
  sustainability panels, private to you. No route exists by which anyone
  else, including a team administrator, can read it (`NFR-PRIV-003`,
  asserted by `team_admin_cannot_read_member_personal_data`).
- **Quiet by design** — notifications fire once, at the moment a threshold is
  crossed, with a cooldown (`FR-NTF-001`, `FR-NTF-002`).
- **Teams, sprints and planning** — with carried-over work described as a
  fact, never as a shortfall (`FR-SPR-003`).

## Alternatives considered

- **Keep "Issue Tracker" in the interface.** Rejected: it contradicts the
  logo, leaves the running product anonymous to its operator, and is the
  single most generic string the interface could carry.
- **A brand with a descriptor** (`peisear · project health`). Rejected for the
  navbar: it is a tagline in a place that has no room for one, and the
  tagline belongs on the README and the landing surface. Worth revisiting if
  a real landing page is ever built — today `/` only redirects.
- **Change the README and leave the interface.** Rejected: the screenshots
  the README now carries would keep publishing the other name.
- **Rename the product.** Out of scope; the name is the owner's and the logo
  already commits to it.

## Risks and sequencing

- **The screenshots.** `DOCS-008` is regenerating them now. If the brand
  changes afterwards, every committed image says "Issue Tracker" until the
  next regeneration — including the crates.io hero image for 0.49.0. **The
  brand string is one file and no test asserts it; folding it into `DOCS-008`
  costs one more capture run and gets it right the first time.**
- **Surfaces outside the repository.** The crates.io summary comes from
  `Cargo.toml` and is covered by §9. The **GitHub repository description** is
  the owner's to edit — and, measured on 2026-10-11, it says something
  different again: *"Accompanying assistant to promote project
  collaboration."* It is **not wrong in the README's direction** (it never
  says *issue tracker*), but it is a third distinct self-description, so the
  product now introduces itself three different ways depending on where it is
  met. **0.50.0's copy pass should produce one line and use it in all five
  places** — README hero, both `Cargo.toml` descriptions, the `//!` line,
  `book.toml`, and the GitHub description — rather than five lines that each
  sound reasonable alone.
- **No functional risk.** Neither proposal changes behaviour, routes, data or
  permissions.

## Decisions taken (owner, 2026-10-11)

1. **The interface brand string is `peisear`** — bare, lowercase, in the
   navbar and in all seven title suffixes.
2. **The brand string lands in 0.49.0**, inside `DOCS-008` and **before the
   screenshots are captured**, so the crates.io hero image for 0.49.0 carries
   the real name. **The `README.md` repositioning lands in 0.50.0** as its own
   round, with `DOCS-006`.

## §9 — found while scoping the decision: the same claim is in published metadata

Three further one-line descriptions say *issue tracker*, and all three are
**published by the 0.49.0 release itself**:

| File | Text | Published to |
|---|---|---|
| `crates/peisear/Cargo.toml:3` | "A minimal, sophisticated, solid, and easy issue tracker." | **the crates.io card** |
| `crates/peisear/src/lib.rs:1` | `//! peisear — a minimal, sophisticated, solid, and easy issue tracker.` | **the docs.rs front page** |
| `crates/peisear-web/Cargo.toml:3` | "…for peisear (issue tracker)." | the crates.io card for that crate |
| `docs/book.toml:17` | "Self-hostable issue tracker: …" | the book's metadata |

These are the same defect as the brand string, not the same work as the README
pass: one line each, mechanical, and **published at the next `cargo publish`**.
Leaving them would mean 0.49.0 ships a corrected hero image beside an
uncorrected crates.io description, which is a worse state than either.

**So they move with the brand string, into `DOCS-008`**, with the wording
taken from this RFC's own hero draft rather than invented separately — and
marked provisional, because 0.50.0's README pass owns the final phrasing and
these lines follow it.
