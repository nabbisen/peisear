# peisear — Software Requirements Specification

**Document status**: Baseline
**Covers release**: `0.38.0` (**`FR-DM-001` amended from five
direct-manipulation surfaces to four and reaches Met** — D-5 retired by owner
decision rather than built; four ordering defects and one concurrency defect
closed, `§10.29` and `§10.30`; implementation through `0.38.0`)
**Supersedes**: [`history/peisear-0.20.0-requirements-en.md`](./history/peisear-0.20.0-requirements-en.md),
and through it
[`history/peisear-0.19.1-requirements-en.md`](./history/peisear-0.19.1-requirements-en.md).
Both retained unedited as the record of their releases
**Language**: English (normative)
**Prepared**: 2026-07-27 · **Amended**: 2026-09-24 (0.38.0)
**Location**: `docs/specification/requirements.md`. **This document is the
normative source of truth** — English only, placed 2026-09-24 (`DEC-020`,
closed). See [the directory README](./README.md) for how its citations
resolve

> **Amendment note (0.20.0).** This baseline differs from 0.19.1 in two ways
> that deserve to be read before the requirements themselves.
>
> **Nine recorded statuses were wrong.** Not out of date — wrong when written.
> Five P0 or P1 requirements were annotated `Implemented` or `Deferred` while
> the code did the opposite, and nothing checked. The release this baseline
> covers exists to correct them. They are marked below and catalogued in §10.
>
> **A status is a claim, and claims need evidence.** Where a requirement is now
> marked `Implemented`, it is because a named test asserts it, not because it
> was believed. Where no test exists, the status says so.
>
> Full derivation: `.git-exclude/tasks/architect/004-requirements-baseline-amendments.md`.

> **Amendment note (0.21.0).** §1.7 gains two clarifications (§1.7.1, §1.7.2).
> Both were found by implementers who hit an ambiguity in the requirement and
> escalated it rather than resolving it locally — which is the behaviour the
> constraint needs in order to stay enforceable, and the reason both are
> recorded with their instances rather than as abstract wording changes.
>
> §10.8 closes. §10.13 opens: the integration-test harness collides with
> itself, and the isolation procedure adopted to make gate results trustworthy
> is what kept it invisible.

> **Amendment note (0.22.0).** `§10.11` and `§10.13` both close. `§10.11` had
> been open across four releases: in a team-owned project only the owner could
> be assigned an issue, so per-person sustainability signals were empty for
> every other member. It failed by showing nothing rather than by erroring,
> which is how it survived a compliance pass and this baseline's own first two
> editions.
>
> `NFR-PRIV-002`'s caveat — that it described something which had never fully
> existed — is discharged.
>
> **This file is now the single living baseline, not one snapshot per
> release.** `0.19.1` and `0.20.0` stay frozen: the first is the owner's
> handoff artefact, the second is the compliance-pass record the owner
> reviewed. Beyond those, a fresh 2,200-line near-duplicate per release costs
> more than it returns — nothing has ever been read out of the `0.21.0`
> snapshot, and Appendix C plus the gap register already carry what changed
> and when. This file is renamed at each release instead of copied.

> **Amendment note (0.23.0).** No gap opens or closes. `§10.5` — the calendar
> schema reduced from the spec's four date columns to two — moves from a
> recorded intention to a shipped fact, and its "revisit if a firm deadline
> must be distinguished from a soft estimate" clause becomes the live question
> rather than a hypothetical.
>
> Two facts this release established that the register has no column for.
> **A downgrade across a schema migration fails to start**:
> `sqlx::migrate!` does not tolerate an applied migration absent from its
> embedded list, and the binary migrates unconditionally, so recovery is a
> restore from a pre-migration backup rather than a live downgrade. And **the
> calendar surfaces carry no per-person aggregate at all** — which is why
> RFC 001's withdrawn capacity hint did not recur on a surface that lays
> people's planned work on a time axis.

> **Amendment note (0.25.0-0.26.0).** Two releases, one theme: **the
> plain path was the dangerous one, and it had to be built before the
> enhanced one.**
>
> Four destructive deletes confirmed through a JavaScript dialog bound to
> the submit event. Without JavaScript that handler never ran and the
> delete proceeded **unconfirmed** — the degraded path was more dangerous
> than the enhanced path, which is the inverse of what progressive
> enhancement is for. External design `§17.4` closes with server-rendered
> interstitials. The five reversible confirmations are deliberately
> unchanged: each is undoable through the interface, so a dialog that can
> vanish costs nothing that cannot be undone.
>
> `FR-DM` moves from one shipped surface out of five to two, and from
> `Deferred` to `Implemented` on five of its seven requirements — but
> **only for the surfaces that ship**. The register's habit of reading a
> requirement's status as global is exactly what let `FR-DM-002` be
> violated unnoticed at 0.19.1; the statuses below name their surfaces.
>
> **Two things were found by planting rather than by reading**, and
> neither would have failed a test: the board's three error sentences had
> never been reached by the vocabulary guard, because that guard only ever
> read Rust; and the board's `<script>` tag could be deleted with the whole
> suite green. Both are closed. The second is recorded as `§10.14`.
>
> Two more are open. `§10.15`: **the JavaScript that now carries the
> in-place update, the undo, and the fallback is not executed by any
> test** — the harness drives HTTP and does not run scripts. And
> `§10.16`, found writing this amendment: **none of the four structural
> guards has a CI job.** `prose_scan`, `static_js_scan`,
> `test_harness_scan` and `dec_007_scan` all live in
> `peisear-web --lib`, which `.github/workflows/test.yml` never runs and
> `DEC-007`'s own command block omits. They run under
> `cargo test --workspace`, which is in the release gate three times —
> so no release has shipped without them — but a pull request that
> reintroduces any of those four defect classes passes CI.

> **Amendment note (0.27.0).** Two user-visible changes, and a release spent
> checking the checkers.
>
> **The finding this release exists for is about this document's own
> apparatus.** `find_violations` — the guard behind `NFR-LANG-001`, **P0** —
> walks `MessageKey::all()`, a hand-maintained list. **Five live `aria-label`
> variants were absent from it**, so the check that holds all copy to §1.7 had
> never read them. They were correct. Nothing would have said otherwise.
>
> That is the same defect this baseline's first edition was written to correct
> — a status asserted and never checked — relocated from the requirements into
> the machinery that checks them. `§10.16` closes and four more entries in the
> register now describe the verification apparatus rather than the product.
>
> **Two changes a self-hoster can see**: a stale delete confirmation now
> answers `409` rather than deleting what changed while it was on screen, and
> the issue confirmation names the sub-issues that go with it. The first
> narrows a request contract — see `§7.1`.
>
> **What this release is not.** `RFC 005`'s accessibility axes — colour
> contrast, keyboard navigation, mobile completion — remain unstarted.
> Definition of Done item 5 stays **Not verified**, and eight consecutive
> quality handoffs went to guard infrastructure instead. That is a scheduling
> fact, recorded here because a register of what checking found says nothing
> about what was never checked.

> **Amendment note (0.28.0).** RFC 005 completes. Fourteen sections, twenty-one
> handoffs, and the release that closes it is the one whose changelog says the
> product is **not** WCAG AA compliant.
>
> **What Phase E was for, in the end.** It began as an audit of the product and
> became an audit of the apparatus that checks it. Nine of its fourteen
> sections turned out to describe work already done, already reverted, or aimed
> at the wrong requirement — including one that proposed re-adding a
> suppression this project had deliberately removed. **Reconciling each section
> against the code before dispatching it is the single practice that produced
> the most value**, and it produced it by preventing work rather than by
> completing it.
>
> **The findings that mattered were about claims, not code.** A P0 vocabulary
> guard that had never read five live strings. Four structural guards that had
> never run in CI. An `Implemented` status on `NFR-CONC-003` while the
> application wrote the column two authorities were supposed to prevent. A
> cited acceptance test that cannot fail. `§10.3`'s entry describing a codebase
> that has not existed for some time.
>
> **`§9.2` now opens with a warning rather than a list**, and that is the
> honest state: 125 requirements marked `Implemented`, 40 citing evidence, 15
> of those 40 overstating it, and a 15-item sample of the 85 uncited finding 8
> with no test at all. The correction is not scheduled, deliberately — the
> owner's direction was to diagnose before editing, and a document made
> consistent before it is understood would lose the only evidence of how it
> drifted.
>
> **Two commitments outlive RFC 005 and are named rather than absorbed**: the
> touch-target design pass (139 controls, and the requirement itself
> may be what changes), and the citation-drift diagnosis.

> **Amendment note (0.29.0).** RFC 008 ships and `§10.4` closes — **partially**,
> after nine releases open and three slips.
>
> **The word "partially" is doing real work and this document should not let it
> soften.** `NFR-A11Y-003` is met in full. `FR-HLT-007` is met on two limbs of
> three; **history is deferred**, and one of its six indicators is **excepted**
> rather than met. Definition of Done item 3 moves — the first of the five
> conditions to move outright since 0.19.1 — and moves to *"Met, with one limb
> outstanding"*, not to *"Met"*.
>
> **Two designs in this release were wrong first and are recorded that way.**
> The basis route was to be query filters on the issue list; three of six
> indicators could not be expressed that way, and building the missing filters
> would have put the staleness clock in two places. And RFC 011's plan to shrink
> the untested JavaScript rested on a count of mine that turned out to be
> `return` and `throw` statements counted as decisions — the rule it most wanted
> to move is the one rule that cannot move.
>
> **Both were caught by an implementer reproducing an architect's table before
> building on it**, which is the practice this project has now had produce its
> largest single saving twice.
>
> **`§10.3` also closed this cycle**, narrowed and guarded after nine releases
> of an entry describing a codebase that no longer existed.

---

## 0. Document control

### 0.1 Purpose of this document

This is the consolidated, English-language requirements baseline for
peisear. Until now the requirements existed across six documents in two
languages — two Japanese kickoff briefs, a 2,388-line Japanese feature
specification, an English GUI external specification, and two
progress-reflecting documents. That spread makes it difficult to answer
"what must be true of this product?" without cross-reading all six.

This document consolidates them into one normative, traceable
requirements set. It does not replace the feature specification, which
remains the canonical description of *screens and behaviour* and is
cited throughout as `SPEC §N`. It replaces the ad-hoc requirement lists
that were embedded in briefs and handoff summaries.

### 0.2 Audience

- Implementers picking up unstarted work.
- Reviewers checking whether a change satisfies or violates a requirement.
- QA deriving test cases.
- The project owner, when deciding whether a proposed feature belongs.

### 0.3 Source documents

| Ref | Document | Language | Role |
|---|---|---|---|
| `KICK` | Project development brief — Minimum Issue Management System | JA | Original scope, tech stack, MVP, project conventions |
| `BRIEF` | Extension brief — project-health optimisation and human burndown prevention (V2.1) | JA | Purpose, non-goals, design principles, metric intent |
| `SPEC` | peisear feature specification v2.1 (2,388 lines) | JA | **Canonical** screen/behaviour specification |
| `GUI` | peisear GUI External Specification v2.1 | EN | Component hierarchy, form contracts, URL/permission matrix, naming rules |
| `V3` | Extension brief V3 (state at 0.19.0) | JA | Implementation-reflecting revision of `BRIEF` |
| `DIGEST` | Project state digest (0.19.0) | JA | Decisions, open questions, implicit assumptions |

Where `SPEC` and an implementation decision diverge, this document
records the divergence explicitly in §10 rather than silently
preferring one.

### 0.4 Normative keywords

Interpreted per RFC 2119:

- **MUST** / **MUST NOT** — absolute requirement. A violation is a defect.
- **SHOULD** / **SHOULD NOT** — strong recommendation; deviation requires
  a recorded rationale.
- **MAY** — genuinely optional.

### 0.5 Requirement identifier scheme

`<CLASS>-<AREA>-<NNN>`

- `CLASS` — `FR` (functional) or `NFR` (non-functional).
- `AREA` — three-to-four letter domain code (`AUTH`, `ISS`, `PRIV`, …).
- `NNN` — zero-padded sequence within the area.

Identifiers are permanent. A withdrawn requirement is marked
`Withdrawn` and retained, never renumbered or deleted.

### 0.6 Status vocabulary

| Status | Meaning |
|---|---|
| `Implemented` | Shipped and covered by at least one automated test. |
| `Implemented (untested)` | Shipped, but no automated test asserts it. |
| `Partial` | Some but not all acceptance criteria are met. |
| `Specified` | Fully specified, design accepted, not yet built. |
| `Deferred` | Accepted as a requirement, deliberately postponed to a named phase. |
| `Divergent` | Implementation exists but contradicts the requirement — see §10. |
| `Withdrawn` | No longer a requirement; retained for history. |

### 0.7 Priority vocabulary

| Priority | Meaning |
|---|---|
| `P0` | Product-defining. Violating it makes the product something else. |
| `P1` | Required for the product to be usable and trustworthy. |
| `P2` | Materially improves the product; schedulable. |
| `P3` | Desirable; may never be built. |

---

## 1. Product overview

### 1.1 Purpose

peisear is a self-hostable project and issue management service for
individuals and small teams. Its distinguishing purpose is stated in
`SPEC §1`:

> The primary aim is **not** "getting tasks finished", but **delivering
> the observation and awareness needed to keep a project — and the
> people in it — sustainable**.

Ordinary issue trackers make two questions invisible until they become
crises: *how is this project trending?* and *am I overloaded?* peisear
exists to make both visible, quietly, without converting the answers
into judgement.

### 1.2 Core value commitments

From `SPEC §1`, restated as product-level commitments that constrain
every requirement in this document:

| Commitment | Meaning |
|---|---|
| **Does not judge** | No "on track", "behind", "grade", or ranking vocabulary anywhere. |
| **Does not chase** | Notifications are edge-triggered and cooled down; the product is quiet by default. |
| **Protects the individual** | Personal sustainability data is visible only to its subject. |
| **Is understandable** | Every number exposes its basis; every state transition has a visible human trigger. |

### 1.3 Intended users

- **Solo developers** who want visibility into a single project's state.
- **Small to mid-sized teams** (roughly 5–30 people) where an
  administrator can reasonably know every member.
- **Organisations that value sustainability** over short-term sprint
  completion.

### 1.4 Explicitly non-intended users

`SPEC §1` names these as uses the product deliberately does not serve.
They are recorded here because they bound the requirement set: a feature
request that only makes sense for one of these users is out of scope by
construction.

- **Large-enterprise resource management.** A different class of tool
  fits better.
- **Individual productivity maximisation.** It conflicts with the
  non-evaluative design.
- **Manager-led monitoring of members.** The administrator role is a
  *management* role, never an *oversight* role.

### 1.5 Scope

**In scope.** Self-hosted single-binary deployment; user accounts and
sessions; personal and team projects; issues with one level of
sub-issues; teams, membership roles, and sprints; project-health
indicators; personal sustainability indicators; edge-triggered
notifications with an inbox; global search; a read-only JSON API for
personal data; server-side-rendered HTML UI.

**Out of scope.** Performance review, appraisal, or ranking features;
continuous monitoring; automatic reassignment or work throttling;
multi-tenant SaaS operation; SSO / IdP / IdaaS integration (future
phase); CI/CD and cloud-platform integration (future phase); a
team-axis calendar (permanently excluded — see `FR-CAL-006`).

### 1.6 Design pillars

`SPEC §2` defines three pillars. They are normative and appear as
non-functional requirements in §5.

1. **ABDD — Accessible by Default.** Accessibility is an acceptance
   criterion of each screen specification, not a final inspection pass.
2. **Minimal by Default.** One screen, one purpose. Secondary
   information is collapsed and hidden by default.
3. **Non-evaluative.** The product states facts and leaves
   interpretation and judgement to the user.

`BRIEF §0.3` adds three further operating principles:

4. **Insight on Demand.** Advanced analysis is opened explicitly.
5. **Human-in-the-loop.** The system proposes; a human decides and acts.
6. **Privacy-aware.** Personal logs are disclosed minimally.

### 1.7 Prohibited and preferred vocabulary (normative)

`SPEC §3` makes vocabulary a hard constraint, not a style preference.
This table is normative for all user-visible strings, including empty
states, tooltips, notification bodies, error messages, and conflict
toasts.

**MUST NOT appear in user-visible text:**

- "performance is increasing / decreasing"
- "good progress", "bad pace"
- "you should X", "you must X"
- "concerning trend", "underperforming", "failing to meet"
- "velocity" (industry term carrying evaluative connotation)
- emphasis on completion rate or achievement rate
- "ranking", "top performer"
- "Failed to update", "Error: outdated version" (`SPEC §21.4.6`)

**Preferred forms:**

- "trending higher" / "trending lower" (direction only)
- "completed work this period" (instead of velocity)
- "X carried over" (factual description)
- "in flight" (work continuing)
- "this is a management role, not an oversight role"
- "private to you"

#### 1.7.1 The prohibition covers the term, not the exact string (normative)

*Added at 0.21.0.*

A term in the list above is prohibited **in every inflection and casing**:
plural, possessive, participle, gerund, and any capitalisation.
`"top performers"`, `"ranked"`, `"rankings"` and `"Velocity"` are prohibited
because `"top performer"`, `"ranking"` and `"velocity"` are.

This is a clarification, not a widening. It states what the requirement always
meant; the base forms were written as examples of a concept, not as a match
list.

**Why it needed saying.** I18N-001's guard transcribed the list literally —
correctly, because a guard that guesses at inflections diverges from the
requirement it enforces, which is worse than a known gap. The implementer
reported the gap rather than closing it in the guard, and that was the right
call: **the enforcement must not get ahead of the rule.** So the rule moves
first, here, and the guard follows.

*Consequence for `peisear-i18n`*: `find_violations` matches case-insensitively
and covers regular inflections of each listed term. Irregular forms are listed
explicitly. Where a listed term has a legitimate non-evaluative homograph, that
is a finding to escalate, not a suppression to add.

#### 1.7.2 Prohibited in use, permitted in mention (normative)

*Added at 0.21.0.*

§1.7 governs text that **addresses the user in the product's own voice**. It
does not prohibit *naming* a term in order to discuss it.

- **Prohibited (use)**: any user-visible string that describes the user's work
  in these terms — screens, empty states, tooltips, notification bodies, error
  messages, conflict toasts, API message fields.
- **Permitted (mention)**: text whose subject is the vocabulary rule itself —
  this document, `CHANGELOG.md` entries describing a §1.7 defect and its fix,
  code comments, RFCs, review packages, and the guard's own term list.

**Why it needed saying.** The rule was written for the product's voice and then
applied to everything the project writes, which made three defect-corrections
awkward to describe honestly:

1. **DEV-009** — a changelog entry for a fixed §1.7 violation could not name
   the phrase that was fixed without appearing to violate the rule.
2. **REL-0.20.1** — the implementer paraphrased two corrected sentences rather
   than quoting them, and noted that neither old wording actually contained a
   prohibited term, so quoting would not have been a violation either way.
   They paraphrased anyway rather than make that judgment in shipped copy, and
   flagged it. Their own review package *did* quote the wording, on the
   grounds that a review package is not user-facing — drawing exactly the
   distinction this clause now makes.
3. **REL-0.21.0** — a changelog draft used two prohibited terms as
   illustrations inside the guard's own description. Reworded, because an
   equivalent phrasing existed, and rewording beat leaving an open question
   that did not need to exist.

Three implementers hit the same ambiguity and none of them resolved it
locally. That is the behaviour a normative constraint needs, and it is why the
clause is written here rather than settled in a review.

**The boundary, stated so it is not stretched.** A changelog is a mention
surface *when its subject is the rule*. It is a use surface when it describes
the user's work. "Corrected a badge that displayed a completion score" is
mention; "improved your completion score display" is use, and prohibited. If a
sentence is arguable, it is use — the default protects the reader.

*Consequence*: the guard covers the message table, which is use by
construction. Mention surfaces are not scanned, and `prose_scan.rs` does not
scan them either. Neither is an oversight.

---

## 2. Stakeholders and roles

| Role | Definition | Key limitation |
|---|---|---|
| Anonymous | Unauthenticated visitor | Sees only landing, login, register |
| Personal user | Authenticated user acting outside a team | Owns personal projects |
| Team viewer | Team member with read access | Cannot mutate team content |
| Team member | Team member with write access | Cannot manage membership or sprint lifecycle |
| Team admin | Team member with management rights | **MUST NOT** read another member's personal sustainability data |

The final row is the single most important constraint in this document.
`SPEC §11.3` and `§11.5.2` state it as an invariant: administrative
authority over a team does not include authority to read members'
personal data. It is enforced at the API layer, not only in the UI.

---

## 3. Glossary

| Term | Definition |
|---|---|
| **ABDD** | Accessible by Default by Design. The six-axis accessibility acceptance framework (`SPEC §30`). |
| **Composite** | The aggregate health indicator derived from the individual indicators. |
| **Edge-triggered** | Fires only at the moment a threshold is first crossed, not while the condition persists. |
| **Effort** | Estimated size of an issue in story points. |
| **In flight** | An issue in `open` or `in_progress` status; deliberately used instead of evaluative alternatives. |
| **Insufficient** | Health state meaning "not enough data to classify" — not a negative judgement. |
| **Optimistic lock** | Concurrency control comparing a client-supplied `updated_at` against the stored value, rejecting mismatches with 409. |
| **Personal sustainability data** | Capacity, WIP limit, burnout streaks, estimation drift, cognitive switching, `/today`, personal calendar, notification history and preferences. |
| **Sub-issue** | An issue nested exactly one level beneath a parent issue in the same project. |
| **Top-level issue** | An issue with no parent. |
| **Watch** | The highest severity peisear displays. See `NFR-LANG-002` (Watch ceiling). |
| **WIP** | Work in progress; count of the user's in-flight assigned issues. |

---

## 4. Functional requirements

Each requirement gives: statement, rationale, source, acceptance
criteria, status, priority. Verification references appear in §9.

### 4.1 Authentication and session — `FR-AUTH`

**FR-AUTH-001 — Account registration**
The system MUST allow a visitor to register with an email address, a
display name, and a password. Passwords MUST be stored only as an
Argon2 hash.
*Source*: `KICK §3.1`, `GUI §5`. *Acceptance*: registration succeeds
with password ≥ 8 characters; no plaintext password is persisted or
logged. *Status*: Implemented. *Priority*: P0.

**FR-AUTH-002 — Login and session establishment**
The system MUST authenticate a registered user by email and password and
establish a server-verified session carried in an HTTP-only cookie.
*Source*: `KICK §3.1`, `KICK §4.4`. *Acceptance*: valid credentials
produce a session; invalid credentials do not disclose which field was
wrong. *Status*: Implemented. *Priority*: P0.

**FR-AUTH-003 — Logout**
The system MUST allow a user to end their session, after which protected
routes are inaccessible with the prior credentials.
*Source*: `KICK §3.1`. *Acceptance*: `logout_clears_session_and_redirects`.
*Status*: Implemented. *Priority*: P0.

**FR-AUTH-004 — Unauthenticated HTML access**
Unauthenticated requests to protected HTML routes MUST redirect to the
login page (HTTP 303), not return an error page.
*Source*: `GUI §6`. *Acceptance*: `unauthenticated_projects_redirects_to_login`.
*Status*: Implemented. *Priority*: P1.

**FR-AUTH-005 — Unauthenticated API access**
Unauthenticated requests to `/api/*` MUST return HTTP 401 with a JSON
body. They MUST NOT return an HTML redirect.
*Rationale*: a JSON client receiving HTML markup cannot parse the
response and cannot distinguish authentication failure from corruption.
*Source*: `SPEC §11.5.1`, `SPEC Appendix E.2`. *Acceptance*:
`unauthed_api_users_returns_401_not_redirect`. *Status*: Implemented.
*Priority*: P0.

### 4.2 Navigation and information architecture — `FR-NAV`

**FR-NAV-001 — Five primary entry points**
The primary navigation MUST expose exactly five entry points: Today,
Inbox, Projects, Teams, Search.
*Rationale*: `SPEC §4.1` records that the previous seven-entry
navigation diluted the user's sense of where to start.
*Source*: `SPEC §4.2`. *Acceptance*: navigation renders five entries.
*Status*: Implemented. *Priority*: P1.

**FR-NAV-002 — Renamed route compatibility**
Routes renamed during redesign MUST continue to resolve via HTTP 308
permanent redirect. HTTP 301 MUST NOT be used.
*Rationale*: 308 preserves the request method, so a bookmarked or
externally-issued POST is not silently downgraded to GET.
*Source*: `SPEC Appendix D.1`; decision `DEC-004`. *Acceptance*:
`/me`→`/today`, `/notifications`→`/inbox`, and
`?edit=1`→`/edit` each return 308 with a correct `Location`.
*Status*: Implemented. *Priority*: P1.

**FR-NAV-003 — Breadcrumb and back affordance**
Detail screens MUST provide a breadcrumb trail and a back link to the
parent context.
*Source*: `SPEC §4.4`. *Acceptance*: `breadcrumb` test crate.
*Status*: Implemented. *Priority*: P2.

**FR-NAV-004 — Parent-aware breadcrumb for sub-issues**
A sub-issue's breadcrumb MUST include its parent issue as a navigable
link, between the project and the sub-issue itself.
*Source*: `SPEC §15.2`. *Acceptance*:
`detail_page_omits_sub_issues_section_for_sub_issue` asserts the parent
title is present. *Status*: Implemented. *Priority*: P2.

**FR-NAV-005 — Context retention for filter and sort**
List filter and sort state MUST persist across navigation. The URL is
the primary carrier; a per-user server-side default is secondary.
*Rationale*: `SPEC §5.3` — returning from a detail screen to a list that
has forgotten its filter breaks the user's train of thought.
*Source*: `SPEC §5.3`. *Acceptance*: `view_state` test crate (5 tests),
including that an explicit filter becomes the user's default and that a
URL parameter overrides the saved default. *Status*: Implemented.
*Priority*: P2.

### 4.3 Projects — `FR-PROJ`

**FR-PROJ-001 — Project lifecycle**
Users MUST be able to create, view, edit, and delete projects.
*Source*: `KICK §3.2`. *Status*: Implemented. *Priority*: P0.

**FR-PROJ-002 — Personal and team projects**
A project MUST be either personal (owned by a user, no team) or
team-scoped (associated with a team). Team features such as sprints
MUST be unavailable on personal projects.
*Source*: `SPEC §7.3`. *Acceptance*: assigning a sprint to an issue in a
personal project is rejected with a validation error.
*Status*: Implemented. *Priority*: P1.

**FR-PROJ-003 — Access control**
A user MUST only access projects they own or that belong to a team they
are a member of.
*Source*: `GUI §6`. *Status*: Implemented. *Priority*: P0.

**FR-PROJ-004 — One decision per screen**
The project detail screen MUST present the project's state such that a
single primary judgement is supported, with secondary analysis collapsed.
*Source*: `SPEC §13.1`, pillar "Minimal by Default".
*Status*: Implemented. *Priority*: P2.

### 4.4 Issues — `FR-ISS`

**FR-ISS-001 — Issue lifecycle**
Users MUST be able to create, list, view, update, and delete issues
within an accessible project.
*Source*: `KICK §3.3`. *Status*: Implemented. *Priority*: P0.

**FR-ISS-002 — Issue attributes**
An issue MUST carry: title, description, status
(`open` / `in_progress` / `done`), priority, position, effort
(optional), assignee (optional), timestamps, and parent reference
(optional).
*Source*: `SPEC §8.1`. *Acceptance*: title 1–200 characters,
description ≤ 10,000 characters (`GUI §5`); effort, when present, is a
positive integer. *Status*: Implemented. *Priority*: P0.

**FR-ISS-003 — Unassigned is a normal state**
An issue without an assignee MUST be a valid, unremarkable state, and
deleting a user MUST NOT delete their issues — ownership returns to the
pool.
*Rationale*: backlog items are routinely unassigned; cascading deletion
would destroy project history when a person leaves.
*Source*: `SPEC §8.1`. *Acceptance*: assignee foreign key uses
`ON DELETE SET NULL`. *Status*: Implemented. *Priority*: P1.

**FR-ISS-004 — Separate read and edit URLs**
The issue detail screen MUST expose read mode and edit mode at distinct
URLs (`…/issues/{id}` and `…/issues/{id}/edit`). The legacy `?edit=1`
query form MUST 308-redirect to the edit URL.
*Rationale*: edit mode is a place, not a hidden state — refresh,
browser-back, and open-in-new-tab must all behave predictably.
*Source*: `SPEC §14.3`. *Acceptance*: `issue_edit_url` test crate
(3 tests). *Status*: Implemented. *Priority*: P2.

**FR-ISS-005 — Status segment control**
The issue detail screen MUST display all three statuses as a segmented
control with the current status indicated, conveying the active state
through `aria-pressed` in addition to visual styling.
*Source*: `SPEC §22.2`. *Acceptance*: `status_segment` test crate; all
three labels present; exactly one segment carries `aria-pressed="true"`.
*Status*: Implemented. **The segment became a mutation path at 0.25.0**
(`STATUS-001`); it renders as a form whose buttons post, so the control
that was display-only is now the control that changes status.
*Priority*: P2.

**FR-ISS-006 — Status segment is display-only until Phase D**
Until the direct-manipulation phase, the status segment MUST NOT mutate
state on click; status changes go through the edit form.
*Rationale*: shipping the affordance early without the interaction lets
the layout settle without promising behaviour that does not yet exist.
*Source*: decision `DEC` in `SPEC §27` phasing. *Acceptance*:
`edit_form_keeps_existing_select_widget`. *Status*: **Expired at 0.25.0 —
satisfied for its whole term.** This requirement carried its own end
condition, and Phase D's first substep reached it. The segment mutates
state now, deliberately.
*What survives its expiry*: the edit form is a **separate, older** path and
still uses `<select name="status">`. `edit_form_keeps_existing_select_widget`
was kept and re-purposed to guard against a future change that collapses the
two paths by putting the segment into edit mode — so the test outlives the
requirement it was written for, with its reason rewritten rather than
inherited. *Priority*: P2.

**FR-ISS-007 — Explicit state transitions only**
Issue and sprint state transitions MUST result from an explicit user
action. The system MUST NOT transition state automatically.
*Source*: `BRIEF §0.2`, `GUI §9`. *Status*: Implemented. *Priority*: P0.

### 4.5 Sub-issue hierarchy — `FR-SUB`

**FR-SUB-001 — One level of nesting**
An issue MAY have sub-issues. A sub-issue MUST NOT have sub-issues of
its own. This MUST be enforced at the database layer, not only in
application code.
*Rationale*: `SPEC §8.3` chose the parent-id model precisely to keep the
hierarchy shallow and promotions cheap; deep trees would reintroduce the
navigation complexity the model avoids.
*Source*: `SPEC §8.3`, `SPEC Appendix C`. *Acceptance*: database
triggers reject nesting on both INSERT and UPDATE;
`cannot_create_sub_issue_under_a_sub_issue` returns 400.
*Status*: Implemented. *Priority*: P0.

**FR-SUB-002 — Same-project constraint**
A sub-issue MUST belong to the same project as its parent.
*Source*: `SPEC §8.3`. *Acceptance*: trigger-enforced.
*Status*: Implemented. *Priority*: P1.

**FR-SUB-003 — No self-parenting and no demotion with children**
An issue MUST NOT be its own parent. An issue that already has children
MUST NOT be demoted into a sub-issue (which would create a two-level
chain); its children must be promoted first.
*Source*: derived from `FR-SUB-001`. *Acceptance*: trigger-enforced.
*Status*: Implemented. *Priority*: P1.

**FR-SUB-004 — Independent attributes**
A sub-issue MUST be able to carry its own assignee, status, priority,
and effort, independent of its parent.
*Source*: `SPEC §8.3`, `SPEC §38.2`. *Status*: Implemented. *Priority*: P1.

**FR-SUB-005 — No automatic parent completion**
When every sub-issue reaches `done`, the parent MUST NOT automatically
become `done`.
*Rationale*: `SPEC §8.6` — the parent frequently carries integration or
review work that no child represents. A suggestion affordance is a
future extension; automatic transition is not.
*Source*: `SPEC §8.6`. *Status*: Implemented (by absence). *Priority*: P0.

**FR-SUB-006 — Top-level-only aggregate views**
Project list, board, and kanban views MUST show only top-level issues.
Sub-issues appear in their parent's detail screen.
*Source*: `SPEC §8.5`. *Acceptance*:
`list_in_project_returns_top_level_only`. *Status*: Implemented.
*Priority*: P1.

**FR-SUB-007 — Sub-issues follow the parent's sprint**
A sub-issue MUST take its sprint membership from its parent. Direct
sprint assignment to a sub-issue MUST be rejected.
*Rationale*: scheduling a parent and its children separately would
double-count effort against a sprint's commitment.
*Source*: project decision recorded in `DIGEST` (answer to design
question Q3), refining `SPEC §8.3`. *Acceptance*:
`sub_issue_inherits_parent_sprint`;
`cannot_assign_sprint_directly_to_sub_issue` returns 400.
*Status*: Implemented. *Priority*: P1.

**FR-SUB-008 — Sub-issues excluded from sprint listings**
Sprint detail listings MUST include only top-level issues, so each unit
of work is counted once.
*Source*: derived from `FR-SUB-007`. *Status*: Implemented. *Priority*: P1.

**FR-SUB-009 — Parent deletion cascades**
Deleting a parent issue MUST delete its sub-issues.
*Rationale*: matches the user's mental model ("delete the parent, the
whole subtree goes") and avoids dangling references. Users who want to
keep children promote them first.
*Source*: implementation decision, migration `0015`.
*Acceptance*: `sub_issues::deleting_a_parent_issue_cascades_to_its_sub_issues`
— a parent with two sub-issues deleted through the real `POST` route, all three
rows gone. *Status*: **Implemented and verified (0.27.0)**. *Priority*: P2.
*Note*: the test was written before the copy that describes the cascade, on the
reasoning that a confirmation naming a consequence must not be written until
the consequence is demonstrated. Had the cascade not fired, the defect would
have been orphaned sub-issues — worse than the missing sentence, and invisible
to a reader of either.

**FR-SUB-010 — Sub-issue search visibility**
Sub-issues MUST be discoverable through search. Search results SHOULD
indicate the parent context.
*Source*: `SPEC §38.1`. *Status*: Partial — sub-issues are searchable;
parent breadcrumb in results is Specified (RFC 0003). *Priority*: P2.

### 4.6 Teams and membership — `FR-TEAM`

**FR-TEAM-001 — Team lifecycle and membership**
Users MUST be able to create teams, and administrators MUST be able to
add members and change roles.
*Source*: `KICK`, `SPEC §9.1`, `GUI §5`. *Status*: Implemented.
*Priority*: P1.

**FR-TEAM-002 — Role model**
The system MUST support the roles admin, member, and viewer, with
increasing restriction on mutation.
*Source*: `GUI §6`. *Status*: Implemented. *Priority*: P1.

**FR-TEAM-003 — Last-admin guard**
The system MUST prevent a team from losing its last administrator.
*Source*: `GUI §5`. *Status*: Implemented (untested). *Priority*: P2.

**FR-TEAM-004 — Non-member concealment**
A non-member requesting a team detail screen MUST NOT learn whether the
team exists.
*Source*: `GUI §6`. *Status*: Implemented (untested). *Priority*: P1.

**FR-TEAM-005 — Privacy footnote**
The team detail screen MUST display, as fixed text, a note stating that
project trends and workload distribution are visible to all members,
that personal sustainability data is visible only to the individual, and
that admin is a management role rather than an oversight role.
*Source*: `SPEC §11.4`, `SPEC §29.6`. *Acceptance*: the string
"management role, not an oversight role" appears on the team detail
screen. *Status*: Implemented (untested). *Priority*: P1.

### 4.7 Sprints — `FR-SPR`

**FR-SPR-001 — Sprint lifecycle**
A sprint MUST progress through `planned` → `active` → `completed`, with
each transition triggered by an explicit administrator action. **A completed
sprint MAY be returned to `active` by an explicit administrator action
(*reopen*)**; no other backward transition exists.
*Source*: `SPEC §9.1`, `SPEC §17.3`. *Status*: Implemented; **reopen added at
0.39.0** (`DEC-053`). *Priority*: P1.
*Amendment (0.39.0, `DEC-053`) — why a backward transition exists at all.*
`SPRINT-001` and `SPRINT-002` close `FR-SPR-004` in all three directions, and
closing it fully means a sprint completed **by mistake** can never be
corrected: its figures are wrong and frozen. The owner accepted reopen as the
answer rather than leaving a completed sprint's membership editable. The
distinction the decision rests on: **a correction becomes a deliberate,
visible state change someone performed, instead of a silent edit to a finished
record.** Reopen therefore does not weaken `FR-SPR-004` — it is what makes
enforcing it tolerable. `FR-SPR-002` binds it: a reopen is refused while the
team has another active sprint, on the same rule and the same atomicity as
`start`.

**FR-SPR-002 — One active sprint per team**
A team MUST have at most one active sprint at a time.
*Source*: `GUI §5`. *Status*: **Met** — enforced atomically and tested since
0.39.0 (`RACE-001`). *Priority*: P2.
*Correction (0.39.0)*: this read `Implemented (untested)`, and the second word
was doing more work than it could carry. The rule was enforced by a read on
one connection followed by a write on another, with no transaction between
them, so **twelve simultaneous starts left six active sprints** — measured, not
inferred. `RACE-001` put the check and the write under one `BEGIN IMMEDIATE`;
`race_guards` now covers both the concurrent case and the sequential refusal
naming the active sprint. **`untested` was the accurate half of the old
status**, and an untested P2 invariant held for four months by a check that
did not hold it is the argument for `§10.17`'s entry.

**FR-SPR-003 — Carry-over is factual, not failure**
Work not completed within a sprint MUST be described neutrally
("X carried over"), never as failure or shortfall.
*Source*: `SPEC §3` vocabulary table. *Status*: Implemented.
*Priority*: P1.

**FR-SPR-004 — A completed sprint's record is fixed**
**What a sprint reported when it completed MUST NOT change afterwards**: its
committed and completed totals, its counts, its carried-over figures and its
burndown series. The record MUST be captured at completion and read from the
capture thereafter; it MUST NOT be recomputed from current data
(`DEC-054`, RFC 0013). Only `FR-SPR-001`'s explicit reopen discards it, and a
reopened sprint captures afresh when completed again.
**Membership is deliberately not frozen.** An issue may leave a completed
sprint — that is `FR-SPR-003`'s carry-over, the designed path for unfinished
work — and the captured record is unaffected by its leaving.
*Rationale*: editing a completed sprint rewrites history and corrupts
trend data.
*Source*: derived; RFC 0001 requirement 8. *Status*: **Met at 0.39.0**
(`SPRINT-001`, `SPRINT-002`). *Priority*: P2.
*Correction (0.39.0) — twice, and the second is larger.* The old status read
*"Specified (the planning screen enforcing this is unbuilt)"*, locating the
omission in one unbuilt screen when **three shipped routes** reached it, none
of them that screen: the issue form's unassign branch, `plan_remove`, and
`add_issue`'s upsert, which moves a membership and so removes it from whatever
sprint held it.
**Then the requirement itself turned out to be the wrong mechanism.** Its old
text protected *membership*, which is **one of four inputs** to the figures it
exists to protect: `summary` computes `completed_points` from each member's
**current** `status`, and `burndown` from `status`, `effort` and `updated_at`.
**A completed sprint's record therefore drifted with no membership change at
all** — carry an issue over, finish it a month later, and the completed
sprint's completed total rises. Two handoffs written against the old wording
(`SPRINT-001`, `SPRINT-002`) would have blocked the product's designed
carry-over flow to buy a guarantee neither could deliver; the dev team stopped
the second before writing code, having measured the flow it would break. RFC
0013 replaced the mechanism. **A rationale is worth more than the sentence
built on it**, and the rationale here — *"editing a completed sprint rewrites
history and corrupts trend data"* — was right throughout. Measured on the shipped product, a completed sprint recording
`committed 15 / completed 8 / 3 issues` read `0 / 0 / 0` after three ordinary
unassigns, and `12 / 2` after one issue was assigned elsewhere. **A requirement
whose status blames an unbuilt screen is not read as describing routes that
exist** — the same failure `FR-DM-001` recorded twice, in a second
requirement. The status now names what was checked rather than what was
missing.
*Sequencing note*: `SPRINT-001` closed two of the three routes and the third
stayed open for one handoff, during which a user was told *"Cannot remove
issues from a completed sprint"* by one control while another did exactly
that. **A protection that can be walked around is worse than none**, because
the message asserts a guarantee the product does not keep; `SPRINT-002` is
scheduled in the same release for that reason.

### 4.8 Sprint planning screen — `FR-PLAN`

**FR-PLAN-001 — Bulk sprint assignment screen**
The system MUST provide `/teams/{slug}/sprints/{sprint_id}/plan` with a
backlog column and a sprint-items column, allowing issues to be moved
between them without visiting each issue.
*Rationale*: `SPEC §17.1` — assigning twenty candidates one at a time
during a planning session is prohibitive friction.
*Source*: `SPEC §17`. *Status*: Specified (RFC 0001, target 0.20.0).
*Priority*: P2.

**FR-PLAN-002 — Button-based moves before drag**
The first implementation MUST provide button-driven moves. Drag and drop
is deferred to the direct-manipulation phase.
*Source*: `SPEC §38.1`. *Status*: **Implemented in both halves** — buttons at
0.22.0 (`PLAN-001`), drag at 0.35.0 (`PLAN-002`, RFC 0004c / D-4).
*Priority*: P2.

**This requirement was fulfilled rather than superseded, and the order it
named is the reason the drag was cheap.** Because the button path already
existed and worked, the drag is an enhancement over an endpoint that was
already correct, and RFC 0004's requirement 0 — a working no-JavaScript path
ships first — was satisfied before `PLAN-002` began. **The buttons remain**,
as the keyboard path, the no-JavaScript path and the touch path: HTML
drag-and-drop does not fire for touch input (RFC 0004's cross-cutting
requirement 10), so the drag reaches a pointer and nothing else.

**FR-PLAN-003 — Backlog filtering**
The backlog column MUST be filterable by project, priority, and
assignee, with filter state reflected in the URL.
*Source*: `SPEC §17.2`, RFC 0001. *Status*: Specified. *Priority*: P2.

**FR-PLAN-004 — Capacity hint is advisory**
The sprint column MUST show committed effort and MAY show a capacity
hint. The hint MUST be presented as guidance, never as a hard limit or
a target to hit.
*Rationale*: `SPEC §17.3` explicitly marks this "not a hard limit"; a
limit would convert planning into a quota.
*Source*: `SPEC §17.3`. *Status*: Specified. *Priority*: P2.

**FR-PLAN-005 — Team-scoped backlog**
The backlog MUST draw only from team-scoped projects, excluding members'
personal projects.
*Source*: RFC 0001 open question 1, resolved. *Status*: Specified.
*Priority*: P2.

### 4.9 Project health — `FR-HLT`

**FR-HLT-001 — Indicator set**
The system MUST compute a set of project-health indicators covering, at
minimum: work completion trend, staleness of in-flight work, recent
activity, concentration of work on individuals, long-stale work, and
WIP-limit compliance.
*Source*: `BRIEF §1.1`, `SPEC §28.1`. *Status*: Implemented —
see §10.1 for divergence from the indicator names in `SPEC §28.1`.
*Priority*: P1.

**FR-HLT-002 — Normalisation**
Each indicator MUST normalise to a 0.0–1.0 scale where 0.0 is the
problematic end and 1.0 the healthy end, absorbing any direction
inversion during normalisation, and MUST clip extreme values so a single
outlier cannot dominate.
*Source*: `BRIEF §4.3`. *Status*: Implemented. *Priority*: P1.

**FR-HLT-003 — Insufficient data is not a negative state**
When data is inadequate, the indicator MUST report `Insufficient` rather
than failing to compute or rendering as a poor score.
*Source*: `BRIEF §4.2`. *Status*: Implemented. *Priority*: P1.

**FR-HLT-004 — Trend direction**
Each indicator MUST expose a trend derived from a 7–14 day median
window, expressed as direction only (higher / stable / lower) in neutral
colours.
*Source*: `SPEC §28.5`. *Status*: Implemented. *Priority*: P2.

**FR-HLT-005 — Human-language explanation**
When an indicator is in a non-healthy state, the system MUST present a
plain-language sentence describing what is happening, in place of raw
computed values.
*Rationale*: `SPEC §28.6.2` — "3 issues haven't moved in over two weeks"
is actionable; "long_stale_ratio = 0.30 (−15)" is not.
*Source*: `BRIEF §4.4`, `SPEC §28.6`. *Acceptance*: **none — see below.**
*Status*: **Implemented, unverified** (was `Implemented` with a citation that
did not hold). *Priority*: P1.

*Citation withdrawn 2026-08-26 (`QA-018`).* This cited
`health_explainability`'s `human_explanation_omits_good_indicators`. After its
fixture that test's body is:

```rust
if body.contains("Throughput") {
    // Either way the test passes; this is a smoke check.
    let _ = body;
}
```

Beyond a status assertion it **asserts nothing about explanations and cannot
fail for any response the server could produce.** The behaviour is implemented
and correct on inspection; the evidence named for it was not evidence.

*What a real test would assert*, recorded so the next attempt does not repeat
the original's hesitation — its own comment declined to pin wording, which is
why it pinned nothing: force a deterministic non-healthy indicator; assert an
explanation row exists for it; assert healthy and insufficient indicators
produce none; and assert the row contains no `identifier = value` form, which
is the *"in place of raw computed values"* clause as an absence.

*This needs no browser.* The explanations are server-rendered and the existing
HTTP harness already receives them — it is a fixture-control problem, not a
rendering one, and must not be scheduled behind `§10.15`'s browser question.

**FR-HLT-006 — Explanation neutrality**
Explanation text MUST be descriptive and MUST NOT contain evaluative
words or directives.
*Source*: `SPEC §3`, `NFR-LANG-001`. *Status*: Implemented (untested —
no automated vocabulary guard exists). *Priority*: P0.

**FR-HLT-007 — Indicator basis is traceable**
Each indicator MUST offer a route to its basis: the underlying issue
list, the calculation, and recent history — not only a tooltip.
*Source*: `SPEC §28.3`, Definition of Done `§41.3`. *Acceptance*:
`basis_route` (5 tests). *Status*: **Partially implemented (`HLT-001`,
post-0.28.0) — two of three limbs.** Basis and calculation ship; **history is
deferred.** *Priority*: P2.

*Amended (RFC 008 §2, owner-approved 2026-08-27)*: this read *each* indicator
MUST offer a route to its basis. **WIP compliance is excepted**, because its
basis is *which assignees are over their limit* and a WIP limit is named in
`NFR-PRIV-001`'s own inventory as visible only to its subject. A blanket MUST
that cannot be met for one of six cases is a requirement that gets quietly
not-met; the exception is carved, its reason recorded, and a test asserts the
absence.

**The exception is structural rather than special-cased.** The design returns
each indicator's basis *set* from the computation that produced its count; WIP
compliance's basis is users, not issues, so its computation returns nothing and
the route has nothing to render. A future contributor cannot helpfully "fix"
the asymmetry without inventing a users-shaped basis first.

*Why the design is not a filter.* The first attempt was to link to the issue
list with query parameters. **Three of six indicators cannot be expressed that
way** — `status` matches exactly one value where in-flight is two, every sort is
descending, and staleness uses an event-aware clock rather than a column
compare. Reproducing that logic in the web layer would have put the staleness
clock in two places and let them drift invisibly: the indicator says four, the
link shows three. **The count is now the length of its own membership**, so the
two cannot disagree.

*History, deferred and why.* An indicator's history is a time series, and for a
project with one active contributor it is that person's history — the shape
`NFR-PRIV-007` suppressed at 0.28.0. Building it needs `QA-017`'s contributor
predicate and its own audit.

**FR-HLT-008 — Composite is not a headline score**
The composite indicator MUST be displayed alongside the individual
indicators at equal weight. The system MUST NOT present a single
aggregate score as the headline representation of project health, and
MUST NOT render a 0–100 gauge or progress bar.
*Rationale*: `SPEC §28.4` — a headline score invites the user to stop at
the number instead of reading the indicators that would tell them what
to do.
*Source*: `SPEC §28.4`, `SPEC §28.6.2`. *Acceptance*:
`health_presentation_has_no_headline_score`,
`composite_renders_beside_indicators_not_as_a_headline`. *Status*:
**Implemented (0.20.0)** — the score badge is retired; the composite
renders as one chip among the six, inside the same disclosure, keeping its
trend and summary sentence but carrying no number. *Priority*: P1.

**FR-HLT-009 — Computation and presentation are separate concerns**
Internal computation MAY use arbitrary statistical sophistication.
Presentation MUST NOT expose scores, gauges, percentages of achievement,
rankings, or evaluative colour contrast.
*Rationale*: `SPEC §28.6` — accuracy is the source of the product's
value and is invisible to the user; the constraint applies to
presentation only.
*Source*: `SPEC §28.6`. *Status*: Partial — the separation holds for
explanation text but not for the score badge (see `FR-HLT-008`).
*Priority*: P1.

### 4.10 Personal sustainability — `FR-PER`

**FR-PER-001 — Personal dashboard**
The system MUST provide `/today`, showing the authenticated user's own
current load, work rhythm, and sustainability signals.
*Source*: `BRIEF §2.2`, `SPEC §12`. *Status*: Implemented. *Priority*: P0.

**FR-PER-002 — WIP limit and overage detection**
The system MUST track the user's in-flight assigned issue count against
an effective WIP limit and indicate when the count exceeds it. The
default limit is 3, adjustable per user.
*Source*: `BRIEF §1.2`. *Status*: Implemented. *Priority*: P1.

**FR-PER-003 — Capacity periods**
The user MUST be able to define capacity in points, optionally scoped to
a date period, with overlapping periods rejected.
*Source*: `BRIEF §1.2`, `GUI §5`. *Status*: Implemented. *Priority*: P1.

**FR-PER-004 — Burnout signals**
The system MUST derive, for the individual only: an overload streak
(consecutive snapshots over capacity) and a stalled-assignment measure
(age of the oldest in-flight assigned issue).
*Source*: `BRIEF §1.2`, `SPEC §11.1`. *Status*: Implemented. *Priority*: P1.

**FR-PER-005 — Estimation drift**
The system MUST expose the user's tendency toward under- or
over-estimation, framed as an aid to planning realism and never as a
performance measure.
*Source*: `BRIEF §1.3`. *Status*: Implemented. *Priority*: P2.

**FR-PER-006 — Single "what to read first" callout**
`/today` MUST surface at most one prioritised callout, chosen by a
strict precedence chain: sustained burnout signal, then WIP over limit,
then long-stale assigned work. If none applies, no callout is rendered.
*Rationale*: `SPEC §12` and the Minimal-by-Default pillar — two
simultaneous callouts dilute both; a manufactured callout in a healthy
state is noise that trains the user to ignore the slot.
*Source*: `SPEC §12.2`, `BRIEF §0.3`. *Acceptance*:
`today_renders_no_callout_for_fresh_user`. *Status*: Implemented.
*Priority*: P1.

**FR-PER-007 — Secondary panels collapsed by default**
On `/today`, the rhythm panel MUST be collapsed by default; the current
load panel MUST remain always visible; the sustainability panel MUST
self-expand only when a signal reaches its watch threshold.
*Source*: `SPEC §12.2`, Definition of Done `§41.4`. *Acceptance*:
`today_folds_rhythm_panel_by_default`. *Status*: Implemented.
*Priority*: P1.

**FR-PER-008 — Pace framed as approximate**
Any pace or cycle-time figure MUST be presented as a coarse
approximation with an explicit caution against over-interpretation.
*Source*: `BRIEF §1.3`, `SPEC §12`. *Status*: Implemented. *Priority*: P2.

### 4.11 Notifications and inbox — `FR-NTF`

**FR-NTF-001 — Edge-triggered dispatch**
Notifications MUST fire only at the moment a threshold is first crossed,
not repeatedly while the condition holds.
*Source*: `SPEC §29.1`. *Status*: Implemented. *Priority*: P0.

**FR-NTF-002 — Cooldown**
The system MUST suppress repeat notifications of the same kind to the
same user within a 24-hour cooldown window.
*Source*: `SPEC §29.1`. *Status*: Implemented. *Priority*: P1.

**FR-NTF-003 — Severity ceiling**
Notification severity MUST be limited to `Info` and `Watch`. A
`Concern` severity MUST NOT be introduced.
*Source*: `SPEC §29.1`, Definition of Done `§41.2`. *Status*:
Implemented. *Priority*: P1.

**FR-NTF-004 — Inbox**
The system MUST provide `/inbox` listing the user's notification
history, visually distinguishing unread from read.
*Source*: `SPEC §19`. *Status*: Implemented. *Priority*: P1.

**FR-NTF-005 — Mark all read**
The inbox MUST offer a prominent action to mark all notifications read,
hidden when there are none unread.
*Source*: `SPEC §19.3`. *Status*: Specified (RFC 0003). *Priority*: P2.

**FR-NTF-006 — Silence-all and its release affordance**
The user MUST be able to silence all notifications. When silenced, the
inbox MUST display a pinned banner offering single-click resume.
*Rationale*: `SPEC §29.3.2` — a user who silenced notifications months
ago should not have to remember where the setting lives.
*Source*: `SPEC §29.3.2`. *Status*: Partial — silence-all exists in
settings; the pinned inbox banner is Specified (RFC 0003). *Priority*: P2.

**FR-NTF-007 — Deferred email opt-in**
The email opt-in prompt MUST appear only after the user has received
their first in-app notification, not at registration.
*Rationale*: `SPEC §29.3.1` — asking before the user has seen what a
notification looks like requests a decision without information.
*Source*: `SPEC §29.3.1`, `SPEC §19.4`. *Status*: **Implemented (0.24.0)**
— RFC 003 as rewritten, handoff `INBOX-001`. *Acceptance*:
`inbox_refinements` tests 5 and 6. *Priority*: P2.
*Note*: the settings-page prompt this replaced could be answered before the
user had received anything, which is the state this requirement exists to
prevent. Moving it was therefore a correction, not only a relocation — and
the old route was removed rather than left as a second, non-compliant path.

**FR-NTF-008 — Graceful degradation without SMTP**
With SMTP unconfigured, the system MUST log a warning at startup and
continue operating with in-app notifications. With SMTP configured but
unreachable, send failures MUST be logged without disrupting in-app
delivery.
*Source*: `SPEC §29.5`. *Status*: Implemented. *Priority*: P1.

**FR-NTF-009 — Recipient-only access**
Notification records and actions MUST be accessible only to their
recipient.
*Source*: `SPEC Appendix E.1`, `GUI §5`. *Status*: Implemented.
*Priority*: P0.

### 4.12 Search — `FR-SCH`

**FR-SCH-001 — Global search**
The system MUST provide search across projects and open issues the user
can access, with a type-ahead affordance and a full results screen.
*Source*: `SPEC §4.5`, `SPEC §6.7`. *Status*: Implemented. *Priority*: P1.

**FR-SCH-002 — Access scoping**
Search MUST NOT return projects or issues the requesting user cannot
access.
*Source*: `SPEC §11.5`. *Acceptance*:
`typeahead_does_not_leak_other_users_projects`. *Status*: Implemented.
*Priority*: P0.

**FR-SCH-003 — Completed work excluded from type-ahead**
Type-ahead MUST exclude issues in `done` status.
*Rationale*: type-ahead serves navigation to active work.
*Source*: `SPEC §4.5`. *Acceptance*: `typeahead_excludes_done_issues`.
*Status*: Implemented. *Priority*: P2.

**FR-SCH-004 — Literal matching of special characters**
Search MUST treat pattern meta-characters in user input literally.
*Source*: security hardening; `KICK §7`. *Acceptance*:
`typeahead_handles_like_meta_characters`; storage-level escape unit
tests. *Status*: Implemented. *Priority*: P1.

### 4.13 Calendar — `FR-CAL`

**FR-CAL-001 — Personal calendar**
The system MUST provide `/today/calendar`, showing only issues assigned
to the authenticated user, across all their projects.
*Source*: `SPEC §16`, `SPEC §10.2`. *Status*: Specified (RFC 0002,
target 0.21.0). *Priority*: P2.

**FR-CAL-002 — Project calendar**
The system MUST provide `/projects/{id}/calendar`, showing the
project's top-level issues on a time axis.
*Source*: `SPEC §16`. *Status*: Specified. *Priority*: P2.

**FR-CAL-003 — Planned time attributes**
Issues MUST carry optional planned start and planned end timestamps to
support calendar placement.
*Source*: `SPEC §38.1`. *Status*: **Implemented** — migration `0016` at
0.23.0; **written by a second path since 0.36.0**, the day view's
drag-to-reschedule (`CAL-003`, RFC 0004d / D-3), which moves both timestamps
by one delta through a narrow endpoint beside the edit form. Note the scope
reduction recorded in §10.5. *Priority*: P2.

**The ordering constraint lives in the schema, not in a handler**, and this is
worth recording because a handoff of mine asserted the opposite. Migration
`0016` carries `issues_planned_range_check_insert`/`_update`, which abort when
both dates are set and the end precedes the start; `translate_trigger_error`
maps that to a worded message. **Neither writing path re-checks it** — the edit
form because the trigger is the authority, and the reschedule because moving
both timestamps by one delta cannot invalidate an ordering that was already
valid.

**FR-CAL-004 — Sprint band on project axis only**
The project calendar MUST overlay a band for active sprints overlapping
the visible window. The personal calendar MUST NOT show sprint bands.
*Source*: `SPEC §16.3`, `SPEC §16.4`. *Status*: Specified. *Priority*: P2.

**FR-CAL-005 — Empty time is not penalised**
The calendar MUST NOT fill, highlight, or comment on unscheduled time.
Crowding MAY be indicated quietly; emptiness MUST NOT be.
*Rationale*: `SPEC §10.4` — an asymmetric design. Flagging gaps converts
a planning aid into a pressure instrument.
*Source*: `SPEC §10.4`, `SPEC §16.3`. *Status*: Specified. *Priority*: P1.

**FR-CAL-006 — No team-axis calendar**
The system MUST NOT provide a team-axis calendar.
*Rationale*: `SPEC §11.3` — aggregating members' schedules is exactly
the oversight capability the product refuses to build. This exclusion is
permanent, not deferred.
*Source*: `SPEC §11.3`, `SPEC §10.2`. *Status*: Implemented (by
absence). *Priority*: P0.

**FR-CAL-007 — No efficiency metrics**
The calendar MUST NOT display occupancy rate, week-over-week comparison,
free-hours totals, or any derived efficiency figure. Factual enumeration
of scheduled items is permitted.
*Rationale*: `SPEC §16.6` lists these explicitly, including the
seemingly benign "free time: 4 hours", because a positive framing is
still pressure.
*Source*: `SPEC §16.6`. *Status*: Specified — RFC 0002 mandates a
guard test asserting these strings are absent. *Priority*: P0.

**FR-CAL-008 — Calendar privacy footnote**
The project calendar MUST display fixed text stating that the view shows
planned issue work and that personal schedules are not aggregated.
*Source*: `SPEC §11.4`. *Status*: Specified. *Priority*: P1.

### 4.14 Settings — `FR-SET`

**FR-SET-001 — Self-scoped settings**
All settings screens MUST operate only on the authenticated user's own
configuration.
*Source*: `GUI §6`, `SPEC Appendix E.1`. *Status*: Implemented.
*Priority*: P0.

**FR-SET-002 — Profile and settings separation**
Identity-facing profile data and behavioural settings SHOULD be
presented as distinct areas.
*Source*: `SPEC §20`. *Status*: Implemented. *Priority*: P3.

**FR-SET-003 — Notification preferences**
The user MUST be able to configure notification delivery per kind and
channel.
*Source*: `SPEC §20.1`. *Status*: Implemented. *Priority*: P2.

### 4.15 JSON API — `FR-API`

**FR-API-001 — Personal data endpoints**
The system MUST expose read-only JSON endpoints for the authenticated
user's burnout signals, capacity, and notifications, under
`/api/users/{user_id}/…`.
*Source*: `SPEC §11.5.1`. *Status*: Implemented. *Priority*: P1.

**FR-API-002 — Self-only authorisation**
These endpoints MUST return data only when the path `user_id` equals the
authenticated user's id.
*Source*: `SPEC §11.5.1`. *Acceptance*: `self_can_read_own_*` return
200. *Status*: Implemented. *Priority*: P0.

**FR-API-003 — Cross-user requests return 403**
A request for another user's personal data MUST return 403, including
when the requester is a team administrator, and including when the
target user does not exist.
*Rationale*: `SPEC Appendix E.2` — returning 404 for a non-existent user
would leak account existence to a probing client, so all refusals
present identically.
*Source*: `SPEC §11.5.2`, `SPEC Appendix E.2`. *Acceptance*:
`*_walls_off_other_users`, `team_admin_cannot_read_member_personal_data`.
*Status*: Implemented. *Priority*: P0.

**FR-API-004 — Stable machine-readable codes**
JSON responses MUST carry stable field and code identifiers that clients
can branch on without parsing human-readable labels.
*Source*: implementation decision `DEC-008`. *Status*: Implemented.
*Priority*: P2.

**FR-API-005 — Wire shape decoupled from internal types**
API response types MUST be defined independently of internal domain
types so that internal refactoring does not silently change the
contract.
*Source*: implementation decision. *Status*: Implemented. *Priority*: P2.

**FR-API-006 — WIP-limit and calendar endpoints**
`GET /api/users/{user_id}/wip-limit` and
`GET /api/users/{user_id}/calendar` are specified in `SPEC Appendix E.1`.
*Status*: Specified — not implemented. The absence of a user-scoped
endpoint of this shape is why one authorisation test remains disabled
(see §9.3). *Priority*: P2.

### 4.16 Direct manipulation — `FR-DM`

**FR-DM-001 — Four direct-manipulation surfaces**
The system SHOULD provide direct manipulation for: status change from a
list, kanban column drag, calendar block drag, and sprint-planning drag.
*Amended 2026-09-24, owner-approved*: this read **five** surfaces and
included *issue list reordering*. That fifth surface is retired — see the
Status note. The other four are unchanged and all four ship, so the
amendment closes this requirement rather than narrowing it to stay open.
*Source*: `SPEC §21.2`, `SPEC §39`. *Acceptance*: `status_control` (12),
`board_keyboard` (6). *Status*: **Met** — all four surfaces ship. Kanban column
drag has shipped since approximately 0.6.0; status change from the issue
list and issue detail shipped at 0.25.0 (no-JS path) and 0.26.0 (in-place),
RFC 004a; sprint-planning drag (D-4) shipped at 0.35.0, RFC 0004c; calendar
block drag (D-3) shipped at 0.36.0, RFC 0004d. **Issue list reordering (D-5) is
retired by owner decision, 2026-09-24** — four of the five surfaces ship and
the fifth will not be built. The product already answers *what is next* with
priority bands, sprint membership and planned dates, and the sprint is the
better answer: named, shared, time-boxed, and on a page of its own. A manual
order would be a second answer to the same question with no name in the UI
and no visible provenance, and RFC 0004's cross-cutting requirement 10 means
that on a phone it would be per-row buttons rather than a drag. `ORD-001`
(0.38.0) removes the `position` column the surface would have used.
**Revisit if** a user asks for manual ordering, or if the sprint-plan
backlog's filters prove insufficient for grooming; reversing costs the same
migration, inverted.
*Priority*: P2.
*Correction*: recorded `Deferred` at 0.19.1 while one of its five surfaces
was in production. That misstatement is what allowed `FR-DM-002` to be
violated unnoticed — a requirement believed dormant is not checked. The
per-surface breakdown above exists so the same reading cannot recur.
*Correction (2026-09-24)*: **it recurred anyway, in this requirement, and
the per-surface breakdown did not prevent it.** Between 0.35.0 and 0.38.0
this entry read `two of five` and described D-3 and D-4 as Deferred with
`no substep RFC written`, while both RFCs were written, accepted and
shipped — and their RFCs sat in `accepted/` for four releases after the
fact. A finer-grained record is still a record: it prevents a wrong
*reading*, not a stale *writing*. What the first correction should have
added, and this one does, is the obligation: **a release that ships a
surface amends this line in the same change.**

**FR-DM-002 — Keyboard parity**
Every direct-manipulation action MUST have a keyboard equivalent
producing the identical effect. A mouse-only action MUST NOT exist.
*Rationale*: `SPEC §32` treats the keyboard path as the contract and the
pointer path as an enhancement.
*Source*: `SPEC §21.3.1`, `SPEC §32`. *Acceptance*: `board_keyboard` (6),
`status_control` (12). *Status*: **Implemented for both shipped surfaces.**
The board's keyboard path landed at 0.20.0. D-1's surfaces shipped their
no-JavaScript form path **first**, at 0.25.0, and the 0.26.0 enhancement
was layered over it — so the keyboard path was never the thing being added
afterward. In force and unmet for any future surface until that surface
ships its keyboard path first. *Priority*: P0.
*Correction*: recorded `Deferred` at 0.19.1. A deferred requirement cannot
be violated — but because `FR-DM-001`'s drag surface had shipped, this one
was in force and unmet: the board's status change had no keyboard path at
all. `DEC-021` now requires the no-JS path to ship *before* the pointer
affordance, so the ordering cannot recur. **0.25.0 and 0.26.0 are the first
release pair to execute that ordering deliberately.**

**FR-DM-003 — Optimistic update with rollback**
Direct manipulation SHOULD apply the change to the interface
immediately and reconcile with the server afterward, reverting the
interface if the server rejects the change.
*Source*: `SPEC §21.3.2`. *Status*: **Implemented for both shipped
surfaces** (0.26.0) — `board.js` for the drag, `dm.js` for the issue list
and issue detail. **Not executed by any test** (`§10.15`): the endpoints,
the response shape and the fallback path are asserted; the scripts are not
run. *Priority*: P2.

**FR-DM-004 — Conflict handling on 409**
On a conflict the interface MUST revert the optimistic change, notify
the user in neutral language, refetch the entity, and re-render the
current state. It MUST NOT retry automatically.
*Source*: `SPEC §21.4.5`. *Status*: **Implemented for both shipped
surfaces** (0.26.0). The `409` path is asserted at the endpoint
(`optimistic_lock`, `status_control`); the client's revert-and-reload is
subject to `§10.15`. *Priority*: P1.
*Note*: `STATUS-002`'s review added cross-cutting requirement 2a to RFC 004
— falling back to a native form submit is correct **before** the server has
applied the change and wrong after. A blanket fail-open would have re-sent
an applied change.

**FR-DM-005 — Conflict message vocabulary**
Conflict messages MUST state that another member changed the item first
and that the latest state is now shown. They MUST NOT use failure or
error vocabulary, and MUST NOT use danger colouring.
*Source*: `SPEC §21.4.6`, `SPEC §21.4.8`. *Status*: **Implemented**
(0.26.0). All conflict copy now lives in `peisear-i18n`'s message table
and is subject to `NFR-LANG-001`'s guard. *Priority*: P1.
*Correction*: the board's three sentences were authored **inside
`static/board.js`** from before this project had a vocabulary guard, and
the guard only ever read Rust — so they were never excluded from the check,
merely never reached by it. Moved byte-exact at 0.26.0 (`BOARD-001`) and
they passed without a reword. `static_js_scan` now covers `static/*.js`,
with `search.js` excluded pending a rendering mechanism, and with one named
blind spot: it does not catch a single word standing alone.

**FR-DM-006 — Undo window**
A completed direct manipulation SHOULD offer a brief undo affordance
(approximately five seconds).
*Source*: `SPEC §39.2`. *Status*: **Implemented for both shipped surfaces**
(0.26.0) — a 5-second toast on the board, the issue list and issue detail.
Undo's own failure modes are split: a `409` inside the window says another
member changed the issue; any other failure says the change could not be
completed. Conflating them told users someone else had changed the issue
when nobody had (`STATUS-002` round 1). Subject to `§10.15`.
*Priority*: P2.

**FR-DM-007 — No celebratory feedback**
Completion feedback MUST remain factual. Celebration, congratulation,
or achievement framing MUST NOT be used.
*Source*: `SPEC §23.5`, `SPEC §25.2`. *Status*: **Implemented** (0.26.0) —
the shipped completion copy is `Moved to Done.` and its siblings, held to
§1.7 by the message table's guard rather than by review. *Priority*: P1.

---

## 5. Non-functional requirements

### 5.1 Privacy and authorisation — `NFR-PRIV`

This is the product's defining constraint set. A violation here is not a
bug of degree; it changes what the product is.

**NFR-PRIV-001 — Personal data inventory**
The following MUST be treated as personal data visible only to its
subject: capacity settings; WIP limit; burnout streaks; estimation
drift; cognitive switching; the entire `/today` dashboard; the personal
calendar; notification history; notification preferences.
*Source*: `SPEC §11.1`. *Status*: Implemented. *Priority*: P0.

**NFR-PRIV-002 — Aggregate data inventory**
The following MAY be shared within a project or team: project health
indicators and trends; workload distribution; project-calendar issue
listings; sprint committed / completed / carried-over counts; sprint
charts.

"Workload distribution" means each member's volume of in-flight work. It
MUST NOT include another member's capacity value, WIP limit, or any state
derived from either — including badge colour, glyph, tooltip, or
annotation. Where this inventory and `NFR-PRIV-001` appear to overlap,
**`NFR-PRIV-001` governs**: an explicit P0 inventory beats a general P1
permission.
*Source*: `SPEC §11.2`; scope clarified by `DEC-019`. *Acceptance*:
`workload_privacy` test crate (4 tests). *Status*: Implemented (0.20.0).
*Priority*: P1.
*Correction*: the unclarified wording was resolved in the code's favour
without a decision — the project detail screen and both issue forms
rendered `{in_flight}/{capacity} pt`, an over-capacity annotation, and a
capacity-derived danger badge to anyone with project access. See §10.7.
*Caveat discharged 0.22.0*: this requirement described something that had
never fully existed — `project_workload` could return only the project
owner (§10.11). RFC 009 corrected the query, so "workload distribution"
now means what the requirement says and what the shipped `FR-TEAM-005`
footnote already promised team members.

**NFR-PRIV-003 — Administrative authority excludes personal data**
No route, UI or API, MUST exist by which an administrator can read
another member's personal data.
*Source*: `SPEC §11.3`, `SPEC §11.5.2`, Definition of Done `§41.1`.
*Acceptance*: `team_admin_cannot_read_member_personal_data`.
*Status*: Implemented. *Priority*: P0.

**NFR-PRIV-004 — API is the security boundary; UI is not**
Authorisation MUST be enforced at the API layer. Hiding an element in
the interface is a usability optimisation and MUST NOT be relied upon as
a protection.
*Rationale*: `SPEC §11.5` — browser developer tools, `curl`, and direct
HTTP requests bypass the interface entirely.
*Source*: `SPEC §11.5.4`. *Status*: Implemented. *Priority*: P0.

**NFR-PRIV-005 — Defence in depth at the storage layer**
Storage functions handling personal data SHOULD additionally accept the
requesting user's identity and verify it, so that a handler-layer
oversight does not become a disclosure.
*Source*: `SPEC §11.5.4`. *Status*: **Not implemented** — verification
exists at the handler layer only. See §10.3. *Priority*: P2.

**NFR-PRIV-006 — Refusals do not disclose existence**
Authorisation refusals MUST NOT reveal whether the requested resource
exists.
*Source*: `SPEC Appendix E.2`. *Status*: Implemented. *Priority*: P0.

**NFR-PRIV-007 — Aggregates must not be reversible to individuals**
Aggregate displays MUST NOT permit reconstruction of individual personal
data. Where a team is small enough that an aggregate resolves to one
person, the aggregate SHOULD be suppressed.
*Rationale*: `SPEC §11.5.3` — a workload chip covering exactly one
member is that member's personal data wearing an aggregate's clothing.
*Source*: `SPEC §11.5.3`. *Acceptance*: `aggregate_privacy` (6 tests).
*Status*: **Implemented (`QA-016`, `QA-017`, post-0.27.0)** — the first
instance of this requirement ever to ship. *Priority*: P2.
*Scope correction (0.20.0)*: a suppression was added to the workload chips
during DEV-003 on the strength of this requirement, then withdrawn. It was
a misapplication: this requirement concerns *aggregates* that inadvertently
resolve to an individual. **A chip labelled with a person's name is not an
aggregate** — it is individual workload, governed by `NFR-PRIV-002`.
Suppressing it at one member would not have protected privacy; because
`project_workload` returns at most one row (§10.11), it would have
silently disabled the surface on every project in existence.
The requirement itself stood unimplemented until after 0.27.0. **Its
prediction was correct**: "when one is built — sprint charts and health trends
are the likely candidates — this is the requirement it must satisfy." Both
candidates existed by 0.27.0 and neither had been looked at.

*Implemented after 0.27.0* (`QA-016` audit, `QA-017` implementation).

**What ships**: below two distinct contributors, the sprint burndown does not
render and the velocity chart's median reference line does not render. **The
sprint-end totals stay** — the aggregate survives; only the trajectory and the
computed statistic go.

**The predicate is distinct contributors, not team size**, and the distinction
is the substance. In a one-person team the only viewer of the burndown is its
subject; hiding it protects nobody. The case that engages this requirement is a
five-person team where one person completed all of a sprint's work and their
day-by-day pattern is shown to the other four — including anyone holding
`viewer`, which the audit confirmed sees both charts. Every prior discussion,
including this entry's own scope correction, reached for team size because it
was the number available.

**Unknown counts as fewer than two.** An unassigned completed issue makes the
true count unknowable, so the trajectory is suppressed. Whether that fires on
the common case **could not be checked** — no production data exists — and the
absence of the check was reported rather than a finding invented from it.
Revisit when there is real usage.

**Corrected from the audit's own first framing**: it is not true that nothing
else exposes completion timing. The issue list renders each issue's
`updated_at`, so a crude completed-by-day series is assemblable by hand. What
the burndown adds is **reliability** — `updated_at` moves on any edit and has
no guaranteed relation to the done-transition. The velocity chart's median is
the stronger case: no proxy exists for it at all, and unlike the burndown it
has no degenerate instance.

**Nothing explains the absence, by design.** Copy naming the reason would
disclose what the suppression withholds; a test asserts no such copy appears.
The principle, which took two review rounds to state properly:
**conditionally rendered copy is not the trap — condition-revealing copy is.**

**NFR-PRIV-008 — Automated authorisation regression tests**
Every endpoint in the personal-data inventory MUST have automated tests
asserting 403 for another user, 403 for an administrator, and 401 for an
unauthenticated caller.
*Source*: `SPEC §11.5.5`, `SPEC Appendix E.5`. *Status*: **Met** (`PRIV-001`,
0.37.0) — every endpoint whose cross-user request is *constructible* now
carries the assertions this requirement asks for.
*Priority*: P1.

*The count was five, not the seven I first reported.* My audit said the
administrator case existed only against `burnout`; it has covered all three
`/api/users/{id}/…` endpoints since 2026-05-07, and I missed it because my
search window stopped four lines short of the assertions. The implementer
checked rather than inherited, and declined to add a duplicate that would have
made the count match the claim while changing no coverage.

**The five that were genuinely missing**: the unauthenticated case for
`capacity` and `notifications`, and the cross-user attempt on all three
`/settings/capacity/{id}` mutations. **Every one covered behaviour already
measured correct** — regression tests for properties that held, not fixes.

**They are load-bearing, and the plant that showed it also measured `§10.3`'s
claim for the first time.** With `user_capacities::find`'s ownership scoping
removed the three tests still pass; with every `user_id` predicate in that file
removed they all fail, `303` where `404` belongs. So the handler's check and
the storage layer's own scoping refuse independently — `§10.3` asserted *"two
independent barriers"* from reading the code, and this is that claim
demonstrated.

**The earlier wording pointed at an unfinished audit and that was misleading.**
RFC 005 §1's audit completed at 0.27.0 and found no reachable boundary; what it
did not leave behind was a test for every endpoint it checked. **Every
behaviour the missing assertions would cover was measured in 2026-09-23 and is
correct** — they are regression tests for properties that hold.

**The shortfall, and the classification that makes it finite.** A cross-user
test is writable only where a cross-user request is *constructible*:

| class | how data is named | cross-user constructible | missing |
|---|---|---|---|
| identity in the path (`/api/users/{id}/…`) | another user's id | yes | the administrator and unauthenticated cases for `capacity` and `notifications`; both exist once, against `burnout` |
| a **resource** id in the path | another user's row | yes | the cross-user attempt on all three `/settings/capacity/{id}` mutations; `/inbox/{id}/read` has one |
| nothing in the path | impossible to express | **no** | nothing — the unauthenticated case is the only assertion this requirement can ask of them |

**The middle class was missed because `auth_boundary.rs`'s own module doc
describes it as the third**, saying cross-user POST *"isn't expressible"*
against the settings mutations. It is: they take a row id, and naming another
user's row is the attempt. True of `/settings/wip-limit`, false of the capacity
routes. `PRIV-001` corrects it.

### 5.2 Concurrency and data integrity — `NFR-CONC`

**NFR-CONC-001 — Optimistic locking on owned entities**
Every mutation of an entity owned by a single record MUST carry the
client's observed `updated_at` and MUST be rejected with 409 if it does
not match the stored value.
*Source*: `SPEC §21.4.2`. *Acceptance*: `optimistic_lock` test crate (**16**
tests), `optimistic_lock_atomicity` (**14**), `board_keyboard`,
`confirmation`. *Status*: Implemented (0.20.0); **extended to all four
destructive deletes at 0.27.0**; **made atomic at 0.39.0** (`RACE-002`).
*Priority*: P0.
*Extension (0.39.0, `RACE-002`) — the comparison happens inside the write.*
Until 0.39.0 the handler read `updated_at`, compared it, and then wrote, with
nothing atomic across the three, so two requests carrying the same valid stamp
both passed. **Measured, the outcome differed by route**: a project edit
answered all eight racing saves `303` and stored one — seven people told they
had saved — while issue edits answered `500`, their deferred transaction dying
on the write's upgrade rather than returning the conflict it owed. All thirteen
locking paths now compare atomically: six as a `WHERE` predicate on the single
statement, seven — those with a state check, an overlap check or an event diff
to keep consistent — by holding the write lock from the stamp's read to the
commit.
*Limit of the stamp, stated because this requirement states a guarantee.*
`updated_at` has **one-second resolution**, and `0017`'s trigger sets it to
`CURRENT_TIMESTAMP`, which within the same second equals the value it replaces.
**So a row whose previous write landed in the current second does not move its
stamp, and a second save holding that stamp is indistinguishable from a fresh
one and is accepted.** Reproduced directly. This is a property of the version
value, not of how it is compared; it predates 0.39.0 and is unchanged by it.
The lock refuses a **stale page** — the case it exists for, minutes or hours
old — every time. It does not refuse **two saves inside one request window** on
a row someone else has just written. Closing it needs a finer version stamp: a
millisecond column or a counter, a schema and stored-format change declined for
now on the reasoning `ORD-002` §3 used for `created_at`.
*Extension (0.27.0, `QA-006`)*: two of the four destructive deletes —
project and issue — took no lock at all, while sprint and capacity delete
did. `RFC 010`'s confirmation interstitial had widened the window this
protects: a delete used to be one `POST` from a page in front of the user,
and became `GET` → read → `POST`, with nothing binding the second to the
state the first displayed. All four now lock. The audit that found it also
found **five** routes carrying a lock and no test naming that route, and one
route (`/settings/capacity/{id}/close`) absent from the audit table and the
suite alike.
*Correction*: recorded `Implemented` at 0.19.1 and was not — the kanban
status endpoint accepted and applied lock-free mutations, and the shipped
client never sent a lock value, so every board drag bypassed the contract
for four releases. See §10.6.

**NFR-CONC-002 — Reuse of `updated_at`; no version column**
Conflict detection MUST use the existing `updated_at` column rather than
introducing a separate version column.
*Rationale*: `SPEC §21.4.3` — the column exists on every major entity
and is already trigger-maintained, so no migration or dual-write path is
needed.
*Source*: `SPEC §21.4.3`. *Status*: Implemented. *Priority*: P1.

**NFR-CONC-003 — `updated_at` maintained by the database**
The application MUST NOT write `updated_at`; database triggers MUST
maintain it, so the lock value has exactly one authority.
*Source*: implementation decision `DEC-013`, migrations `0014`, `0017`.
*Acceptance*: `updated_at_authority` (4 tests) and
`peisear-web`'s `updated_at_authority_scan`. *Status*: **Implemented
(`QA-019`, post-0.27.0)**. *Priority*: P1.

*Correction*: this read `Implemented` from 0.17.0 and **was not**. `0014`
added the column and a bump trigger to the four entities that lacked the
column; `projects` and `issues` had carried it since `0001`, so the trigger
convention arrived after those two tables and never went back for them.
Application code wrote `updated_at` in **five** places across three tables —
four `UPDATE … SET` clauses and one `INSERT … VALUES` — and no trigger
existed for `issues`, `projects` or `user_view_states`. **Two authorities, on
the two entities `NFR-CONC-001` most protects.** Found by `QA-018`'s audit of
this document's own claims, not by anything failing.

*Why it mattered while the lock still worked.* Every `optimistic_lock` test
passed throughout. The exposure was prospective: `issues.rs` already had two
`SET` sites, and a third mutation path added without the clause would leave the
column stale — so a later stale-lock check **passes when it must reject**. That
is `§10.6` exactly: a silent bypass, no error, no log line. Demonstrated during
review by dropping the new trigger, at which point a stale `client_updated_at`
returns `303` where it must return `409`.

*A mechanism the fix nearly broke.* `issues::update_status` returned the new
timestamp via `RETURNING updated_at`, and `STATUS-002` hands that value to the
client as its next lock. **`RETURNING` reflects the row as modified by its own
statement, not by an `AFTER` trigger's nested `UPDATE`** — verified directly in
`sqlite3` — so removing the application write would have returned a value the
row had already moved past, and the next in-place status change would have
taken a spurious `409`. The dev team hit this before writing the migration and
stopped. `update_status` now reads the value back in the same transaction; the
round trip `RETURNING` was introduced to avoid is the price of one authority.

*Guarded*: `updated_at = CURRENT_TIMESTAMP` may not appear anywhere under
`peisear-storage/src/`.

**NFR-CONC-004 — No force overwrite**
The system MUST NOT provide an option to override a detected conflict.
*Rationale*: `SPEC §21.4.4` — a privileged overwrite would sit badly
with the management-not-oversight principle, and "re-read, then decide"
is a simpler and safer invariant.
*Source*: `SPEC §21.4.4`. *Status*: Implemented. *Priority*: P0.

**NFR-CONC-005 — Missing lock value is rejected**
A mutation request omitting the lock value MUST be rejected rather than
processed unguarded.
*Source*: `SPEC Appendix E.3.1`. *Acceptance*:
`issue_update_with_missing_client_updated_at_is_rejected` (form path),
`issue_status_change_with_missing_client_updated_at_is_rejected` and
`…_with_empty_…` (JSON path), `keyboard_status_change_with_missing_token_is_rejected`.
*Status*: Implemented (0.20.0); the two delete routes joined this class at
0.27.0 — `client_updated_at` is `#[serde(default)]`, so an absent field
deserializes to `""` and is rejected as a `400`, and a request with no body at
all is rejected as `415` by the extractor before the handler runs.
*Priority*: P1.
*Correction*: as `NFR-CONC-001`. The 0.19.1 acceptance test covered the
form path only; the JSON path — the one the shipped client used — was
tested for a *stale* value but never a *missing* one. Coverage claims are
now recorded per path where an endpoint has multiple entry points.

**NFR-CONC-006 — Structured conflict response**
A 409 response MUST include the entity type, entity identifier, and the
current `updated_at`, so a client can present accurate state.
*Source*: `SPEC Appendix E.3.3`. *Status*: Implemented. *Priority*: P2.

**NFR-CONC-007 — Join tables exempt**
Pure association tables MAY be exempt from optimistic locking, with the
exemption documented at the point of mutation.
*Rationale*: an association carries no independent `updated_at`, and
convergent last-write-wins yields a coherent pairing either way.
*Source*: implementation decision. *Status*: Implemented. *Priority*: P2.

### 5.3 Accessibility — `NFR-A11Y`

Accessibility requirements are acceptance criteria per screen, not a
final audit (`SPEC §30`).

**NFR-A11Y-001 — Keyboard completeness**
Every primary flow — list to detail to back, edit and cancel, marking a
notification read — MUST be completable with the keyboard alone.
*Source*: `SPEC §30.1`. *Status*: **Audited (`QA-011`, post-0.27.0) — no
structural failure found**, with the qualification below. *Priority*: P0.
*How it was audited, which matters as much as the result.* No browser was
available, so **every row was assessed from real server-rendered markup, not
driven with a keyboard.** That establishes necessary conditions — native
interactive elements, explicit `type`, no `tabindex="-1"`, no `aria-hidden` on
anything focusable, no `onclick`-only stand-in — across all four flows, and it
is sound here for a reason worth recording: **`src/components/` contains no
ordering classes at all**, no `order-N`, no `flex-row-reverse`, so DOM order
equals visual order and tab order can be inferred from markup. In a reordering
codebase the same assessment would be worth little.
*What it cannot establish*: that the focus ring is actually visible against
each background at each control's size. That is the open residue.
*Why nothing was found.* Not luck. `DEV-002`/`FR-DM-002` built the board's
keyboard equivalent because a mouse-only drag was not enough, and
`STATUS-001`/`STATUS-002` did the same for the list and detail surfaces under
`DEC-021`'s no-JavaScript-path-first rule. The audit corroborates work already
done rather than discovering that nothing was wrong.
*Adjacent, not in the four flows*: `/settings` and sign-out are reachable only
through the header's DaisyUI dropdown, which opens on `:focus-within` and is
therefore keyboard-reachable — but it is their only path.

**NFR-A11Y-002 — Focus management**
After a mode change, focus MUST move to a defined, visible location, and
MUST NOT be sent off-screen.
*Source*: `SPEC §30.1`. *Status*: Partial. *Priority*: P1.

**NFR-A11Y-003 — Screen reader equivalence for charts**
Every chart MUST provide a one-sentence summary label, a two-to-three
sentence textual summary, and a tabular equivalent of its data.
*Source*: `SPEC §31.1`. *Acceptance*: `chart_equivalence` (5 tests).
*Status*: **Implemented (`HLT-002`, post-0.28.0).** *Priority*: P1.

*Correction*: this read *"Not implemented"* while **the first of its three
parts already existed** — both charts carried a `role="img"` with an
`aria-label`. What was missing was the textual summary and the tabular
equivalent, and the bar chart's label named *what kind of chart it is* rather
than what it shows.

*A caption is not a summary.* The completed-work chart already carried prose
explaining its **encoding**; this requirement asks for the **finding**. Both
now exist and do different jobs.

*What the tables had to not do.* A tabular equivalent is the easiest possible
way to undo `NFR-PRIV-007`'s suppression, in three distinct ways: the
burndown's table would outlive its chart, the velocity table's median row would
restore the statistic its line was suppressed to withhold, and a summary
sentence could state a median that neither shows. Each is gated by **the same
predicate as the thing it accompanies** — the burndown's table lives inside
`render_burndown`, and the median row and clause both read `show_median`. No
second predicate exists to drift.

**NFR-A11Y-004 — Meaning not carried by colour alone**
State MUST be conveyed by label and icon in addition to colour.
Colour-blind-safe patterning MUST be used in charts.
*Source*: `SPEC §30.1`, `SPEC §31.2`. *Status*: Partial. *Priority*: P1.

**NFR-A11Y-005 — Contrast**
Text and background combinations MUST meet WCAG AA (4.5:1).
*Source*: `SPEC §40.1`. *Acceptance*: `peisear-web`'s `contrast_scan` —
`text-base-content/{10..60}` may not appear under `src/`. *Status*:
**Measured and repaired (`QA-012`, `QA-013`, post-0.27.0)** — was *Not
verified* from 0.19.1 through 0.27.0. *Priority*: P1.

*What was found.* The theme's own tokens were never the problem —
`base-content` on `base-100` is 17.21:1. **The 130 opacity modifiers this
project applied to them were**, and 111 sites sat below AA. `/70` is now the
floor: 6.36:1 on `base-100`, 5.76:1 on `base-200`, 5.15:1 on `base-300`.

*Why a floor rather than a repair.* `/60` measured **4.54:1 on `base-100` — a
pass by 0.04** — while failing at 4.23:1 on `base-200`, where three real uses
sat: the login and register subtitle, the first text a new user reads. A muted
tier that passes by four hundredths is one theme adjustment from failing with
nothing to report it.

*A correction to the rule, found by the implementer.* Bare `opacity` composites
the element's whole rendering **including an inherited colour that already
carries alpha**; `text-base-content/N` does not, since a nested colour replaces
an inherited one. Three nested sites therefore fell outside the arithmetic —
one at **2.64:1, worse than the `/30` tier the rule bans outright**. Found by
checking all 21 bare-opacity sites individually rather than sampling.

*The limit, stated rather than implied.* The guard bans a modifier range. It
does **not** know which background a passing modifier sits over, and does not
cover bare `opacity` at all — `opacity` is not a text property, and separating
text from non-text needs rendering (`§10.15`). Every tinted background in the
tree is enumerated and passing, and no sub-floor bare `opacity` remains, but
**neither fact is guarded**. Both drift the moment someone adds a `bg-*`
container.

*The measurement expires.* It holds for `daisyui@4.12.14`'s `corporate` theme
and nothing else. Full table, resolved token values, and the tightest margin —
**4.97:1 on a `bg-primary/25` hover state**, which no screenshot or static
analysis would catch — in RFC 005 §4.

**NFR-A11Y-006 — Mobile completion of key flows**
The following MUST be completable on a phone: reviewing and reading
notifications; viewing `/today`; changing issue status from the issue
detail; viewing today's calendar.
*Source*: `SPEC §33.1`. *Status*: **Met** — verified 2026-09-10.
*Priority*: P1.

**Verified by driving each flow to completion on a phone-sized viewport**, not by
confirming the pages render. Three viewports (320 × 568, 390 × 844, 414 × 896),
touch events rather than synthetic clicks, with `elementFromPoint` asserting the
tap lands on the intended control. **Status change verified on both paths** —
`dm.js`'s in-place update *and*, with scripting disabled, the native form
(`DEC-021`): a phone user with a broken script still completes it. Every check
was **broken first** and confirmed to notice, and the run reproduces.

*Route to this status, recorded because the instrument mattered more than the
answer.* `A11Y-006` was handed out three times and its results never reproduced
between environments — 15/15 in one, 7/15 in another, unchanged across two
correct-looking fixes. **A verification whose answer depends on where it runs
certifies nothing, and that is as true of its passes as its failures.** The
architect verified it directly instead. Details, including three defects found in
the architect's own checks before the answer was believed, in
`.git-exclude/tasks/architect/017-…`.

**What is not covered**: only the four flows this requirement names; one browser
engine; emulated touch rather than a real device, so a soft keyboard, focus-time
viewport resizing and pointer imprecision are unmodelled.

**NFR-A11Y-007 — Touch target size**
Interactive elements MUST present a touch target of at least 44 × 44 CSS
pixels. **The target is the area that responds to a pointer or touch, which
need not equal the control's visible bounds** — a control may satisfy this by
expanded hit area rather than by visible size.
Touch targets of distinct interactive elements **MUST NOT overlap**, **except
where two controls are members of one segmented group whose adjacent borders are
deliberately collapsed** — a `join` cluster — in which case they share a 1 px
column by design. *Amended 2026-09-16 (`TT-005`, `§10.19`).* The carve-out is
narrow on purpose: it requires that the pair belong to the same group, so a tap
on the seam reaches an adjacent member of the group the user was aiming at
rather than an unrelated control. Measured: `elementFromPoint` assigns the
column to one member at all 25 sample points across it.
Conformance with the size clause MUST be enforced by a structural guard over
the component source. **A control that reaches 44 px by growing, inside a
container with a positive CSS `gap`, satisfies the adjacency clause without
further verification** — the layout engine applies the gap independent of the
child's box size. **That argument covers gap-separated clusters and nothing
else**, which is narrower than this entry claimed until 2026-09-06: measured in
a browser, `join`-grouped buttons overlap by 1 px across their full 44 px height
because `join` collapses adjacent borders with a **negative margin**, not a gap.
That is the one overlap in the product, and the clause above now carves it out.
*This entry also claimed two `<summary>`/control pairs overlapping by larger
amounts; they were **withdrawn 2026-09-16** as unreproducible on the tree they
were measured from. See `§10.19`.* A control that reaches 44 px by expanded hit area MUST do so
through an element that participates in layout (a wrapping `<label>`, or padding
on the control), never an overlay that escapes flow, and MUST come with a
demonstrated clearance. Remaining cases are verified by inspection per surface
until rendered-geometry measurement is available (`§10.15`).

**Amended 2026-09-08 by `DEC-050` (RFC 012). The named limit is removed, because
it was measured and it was not narrow.**

The requirement now reads: **every interactive element** presents a 44 × 44 px
target; **the sole exception is a link inside a block of running text, which MUST
be declared as such in the markup**.

*What the limit used to say, and why it went.* It said the count and the guard
covered controls carrying a DaisyUI sizing class, and that everything else was
*"unassessed, not passing"* — with WCAG's inline-link exception *probably*
covering the text links. Measured across ten pages, **exactly one** sub-44
control was a link inside running text. The rest were the account menu on every
page (four links at 34 px, sign-out at 19 px), `<summary>` disclosure toggles at
**16 px** — which external design `§5.7` names by hand — breadcrumb and back
links at 20 px, the navbar brand at 28 px, and **`FR-HLT-007`'s own indicator
basis links at 17 px**. A limit that size is a hole.

**Verification rests on two things and the guard is only one of them.**
`touch_target_scan` proves a **declaration is present** in the source. It cannot
prove the declaration **works**: `min-h-11` on an inline element does nothing,
and a source scan would pass it while the rendered target stayed 20 px.
Conformance therefore requires the guard **and** measurement in a browser, and
this entry says so rather than letting a green guard read as a claim about the
rendered product — which is the shape `QA-009` found when `MessageKey::all()`
was missing five live `aria-label` variants the P0 guard had never read.
*Source*: `SPEC §33.2`. *Status*: **Met with a named, counted population** (`TT-004`, 0.32.0). Every
`<a>`, `<button>` and `<summary>` in `src/components/` reaches 44 × 44 or carries
the one declared exception — **one site**, the notifications footer link that
completes a sentence.

**Three call sites are excluded, both by design, and both named in the guard as
well as here:**

- **The calendar's event chips** (`calendar.rs`, two sites). The day view's block
  height is `top:{}%;height:{}%`, proportional to the appointment's real
  duration. **A 44 px floor would make a fifteen-minute meeting occupy the same
  space as a two-hour one — misrepresenting the schedule the view exists to
  show.** That is not a density objection; it is the control's meaning.
- **The per-indicator "why" toggle** (`issues.rs::indicator_row`, one site,
  rendering up to seven times per project at 134 × 16 px each). At 44 px the
  disclosure controls become the row's dominant element rather than the health
  data they annotate.

*Route to this status, recorded because it took three corrections.* At 0.31.0
this read *met with a named limit*; the limit was measured (2026-09-08) and was
not narrow — an account menu on every page, `<summary>` toggles at 16 px,
breadcrumbs, and `FR-HLT-007`'s own basis links. `DEC-050` removed it. The guard
then had to be corrected three times: a name-based allowlist that accepted
`class=cls` on sight, a binding-follow that checked whether `grow(` appeared
rather than whether every path took it, and finally a **code shape** that makes
the property structural — `grow(if … { "a" } else { "b" })`. **Each bypass was
demonstrated by planting it and watching the suite stay green.** Was *not verified, failing at 139 controls* — re-measured 2026-08-27 against `daisyui@4.12.14`'s own resolved
control heights, not estimated. *Priority*: P1.

| Class | Resolved | Uses | Overridden to 44 | **Below 44** |
|---|---|---|---|---|
| `btn-sm` | 2rem / 32 px | 64 | 2 | **62** |
| `btn-xs` | 1.5rem / 24 px | 18 | 1 | **17** |
| `input-sm` | 2rem / 32 px | 29 | 0 | **29** |
| `select-sm` | 2rem / 32 px | 21 | 0 | **21** |
| `input-xs` | 1.5rem / 24 px | 5 | 0 | **5** |
| `select-xs` | 1.5rem / 24 px | 2 | 0 | **2** |
| `checkbox` | 1.5rem square / 24 px | 3 | 0 | **3** |
| **Total** | | **142** | **3** | **139** |

**Closed by RFC 012** (built, releasing in 0.31.0) — `TT-001` (audit), `TT-002` (application),
`TT-003` (guard). All 139 controls reach a 44 px target: 136 by growing, three
by a `<label>` wrap that keeps each checkbox's own 24 px box. The 44 px fact has
**one home** (`components::TOUCH_TARGET`), and `touch_target_scan` makes a
sizing-class site without it, and a bare checkbox without its wrap,
**unconstructible** — with no exception list, because `TT-002` round 2 removed
the last three hardcoded literals precisely so none would be needed.

**"With a named limit" is doing real work.** The guard keys off sizing classes
and the `checkbox` class. Plain `<a>` links — breadcrumbs, whole-card links,
inline text links — carry neither, so they are **outside the counting method,
not inside it and passing**. WCAG's inline-link exception probably covers the
text links; that is asserted and not verified. The release note says so rather
than claiming WCAG conformance the guard does not cover.

*Historical, from before the closure:*

**Three controls in the product complied**, each a small size class overridden by
`min-h-11 min-w-11`: `confirmation.rs:53` and `:58` (Cancel and Delete, raised
by `QA-015`) and the status button inside `IssueCard`
(`components/issues.rs:827` at the time of writing). Nothing asserts any of them
— see §9.1.

*Cited by symbol as well as line since 2026-08-27.* This one control has been
cited at three different line numbers across three documents — `:760`, `:825`
and `:827` — none of them wrong when written, all shifted by unrelated commits.
`TT-001` caught it. A bare line number in a durable document is a citation that
rots on every commit above it.

*This entry was re-measured 2026-08-27, before the touch-target design pass was scoped
on it. It was wrong in three places, two of which cancelled.*

1. The status line read **139**, the prose read **149**, and the table summed
   to 149. They could not all be right.
2. The `checkbox` row counted **string matches, not controls** — `grep` returns
   the doc comment at `notification_preferences.rs:20`, and matches
   `type="checkbox"` and `class="checkbox"` separately on each of the same
   three inputs. There are **three** checkbox controls, not ten.
3. **Three controls carrying a small size class are overridden to 44 px** by
   `min-h-11 min-w-11`, and were counted as failing. The old table had no
   column for an override, so it could not express them.

**The corrected total is 139, and the previously stated 139 was right by
accident** — reached by omitting the checkboxes entirely (−3) while also
missing the three overrides (+3). Two errors of equal size in opposite
directions.

**This is why the figure was re-derived rather than carried forward.** A number
that survives because its errors cancel stops surviving the moment either is
fixed alone, and the design pass was about to be scoped on it. The claim
*"exactly one control complies"* was wrong for the same reason: it predates
`QA-015` raising two more, and was never re-checked.

**The published 0.28.0 changelog's figure of 139 is therefore correct** and
needs no amendment.

**Amended 2026-08-27 by `DEC-049` (RFC 012). `SPEC §33.2` amendment pending**,
recorded the way `DEC-030` records `§28.1`'s.

`SPEC §33.2`'s 44 px is **WCAG 2.2 AAA** (2.5.5). The **AA** criterion is 2.5.8
— 24 × 24 with a spacing exception. `§33.2` took the stricter figure and dropped
the exception that makes the standard workable in dense interfaces, and nothing
in the record shows that was considered rather than assumed. Measured against
AA, **zero controls fail on size**; all 139 failures are against a self-imposed
AAA bar.

**The 44 px is kept and the rule is repaired, rather than the bar being
lowered.** Amending to AA — or to a two-tier rule, 44 px where a mis-tap is
costly and 24 px elsewhere — was considered and rejected, and **not for
strictness**. Both rest on the spacing exception, which is a claim about
**rendered geometry** and cannot be evaluated from source. No test this project
owns can answer it, and whether to acquire one is `§10.15`'s open question,
deferred to 0.32.0. Either would write into this document a conformance claim
that can only be asserted and never checked — `§10.15`'s gap promoted into the
requirement. `touch_target_scan`'s own doc comment states the consequence: a
rule that cannot be checked gets weakened until it passes.

The two-tier rule fails again and worse: it makes **conformance itself vary by
surface**, and needs the judgement *"is a mis-tap costly here"* re-made for
every control ever added, checkable by nothing. Under `DEC-049` only the
**mechanism** varies — grow the control, or expand its hit area.

*One argument in this entry's previous version was wrong and is recorded rather
than removed.* It read: *"a Kanban card whose status buttons are 44 px tall is a
different card."* **Those buttons are already 44 px** — the status button inside
`IssueCard` (`components/issues.rs:827` at the time of writing),
`btn-ghost btn-xs min-h-11 min-w-11`, two per card, shipped since before the
question was raised. The densest surface in the product was already compliant
and is not worse for it. The objection also mistook *target* for *visual size*,
which the amended clause now makes explicit.

**What the amendment costs, stated because clause 2 is not decoration.** Two
32 px controls 4 px apart, each expanded to a 44 px target, have **overlapping**
hit areas, and a tap in the overlap resolves to whichever is stacked above —
worse than a small target, because it is wrong rather than merely difficult. A
size floor without an adjacency clause manufactures the defect it was adopted to
prevent.

**NFR-A11Y-008 — Live region announcements**
Dynamic changes MUST be announced through an appropriate live region;
conflict notifications MUST use an assertive region.
*Source*: `SPEC §21.4.8`. *Acceptance*:
`status_control::both_surfaces_render_a_polite_and_an_assertive_status_region`.
*Status*: **Implemented (`QA-011`, post-0.27.0)**. *Priority*: P1.
*Correction*: this read "Deferred with Phase D" while D-1 and D-2 had shipped
at 0.25.0 and 0.26.0 — so it was in force and unmet, the same shape as
`FR-DM-002` at 0.19.1: a requirement believed dormant is not checked. Both
`board.js` and `dm.js` announced **every** outcome, success and conflict alike,
into one `role="status"` region — polite. Now two regions, chosen by outcome.
*Note*: `settings.rs`'s conflict block keeps `aria-live="polite"` deliberately.
It renders on a fresh `GET` after a redirect and nothing ever mutates it, so
`aria-live` is inert there; the reason is recorded in the code rather than only
here.

**NFR-A11Y-009 — Keyboard shortcuts**
The system SHOULD provide list navigation shortcuts (`j` / `k` for
movement, `Enter` for detail), which MUST NOT intercept keystrokes while
focus is in a text input.
*Source*: `BRIEF §3.4`, `SPEC §32`. *Status*: Not implemented —
Phase E. *Priority*: P3.

### 5.4 Language, tone, and locale — `NFR-LANG`

**NFR-LANG-001 — Non-evaluative vocabulary**
All user-visible text MUST conform to §1.7. This applies to numbers,
charts, notification bodies, empty states, validation messages, and
conflict notices.
*Source*: `SPEC §3`, `SPEC §28.6.3`. *Acceptance*: `peisear-i18n`'s
`find_violations` over the message table; `prose_scan` and
`static_js_scan` in `peisear-web`'s library tests. *Status*:
**Implemented, with an automated guard** — the "Implemented by
convention; no automated guard exists" this line carried until 0.26.0
had been wrong since **0.21.0**, when RFC 006 shipped the message table
and the vocabulary guard. *Priority*: P0.
*What the guard covers, precisely*: copy is held in `peisear-i18n`'s
message table and checked against §1.7 there, so a violation is
unconstructible rather than merely absent. `prose_scan` fails the build
on user-visible English authored in Rust instead; `static_js_scan` does
the same for `static/*.js` from 0.26.0. Two named limits: `search.js` is
excluded pending a rendering mechanism, and the JavaScript scan does not
catch **a single word standing alone** — it looks for two or more.
*Correction*: this status is the reason `§10.16` matters. The guard is
real, and until it has a CI job it does not run on a pull request.

**NFR-LANG-002 — Watch ceiling in presentation**
No user-visible severity label MUST exceed `Watch`. The vocabulary
`Concern`, `danger`, and `failing` MUST NOT appear in the interface, and
danger colouring MUST NOT be used to represent health state.
*Rationale*: `SPEC §28.2` and `GUI §9` state this as an absolute
interface ceiling. Internal computation may model finer gradations
(`SPEC §28.6.1`); presentation must not expose them.
*Source*: `SPEC §28.2`, `GUI §9`. *Acceptance*:
`health_presentation_clamps_concern_to_watch_vocabulary` —
case-insensitive, isolates the summary paragraph, sweeps the whole page,
and asserts the fixture actually reaches `Concern` before checking
anything, so it cannot pass hollow. **Observed failing before the fix.**
*Status*: **Implemented (0.20.1)**. *Priority*: P0.

*Correction 1 (0.20.0)*: the 0.19.1 record named one violating surface
(the project health strip, §10.2). There was a second — `/today` could
render `✗` and danger colouring for a user's own WIP or long-stale
state, on the page every user lands on after login, and that surface
appeared in no gap record. Both are now clamped at the render boundary:
the four-state internal model is preserved for computation, but
`HealthIndicator` no longer carries a badge method, so an unclamped
**badge** render fails to compile. See §10.2, §10.10.

*Correction 2 (`ISSUE-006`)*: there was a third. `project_health::summarize`
rendered `"{label} is a concern."` into the summary paragraph beneath the
health heading — prose, not a badge, so the `DisplayHealthState` clamp
never touched it. **Closed in 0.20.1** (`DEC-046`, handoff I18N-004) by
deleting the two `Concern`-shaped message keys from `peisear-i18n`
outright, so a sentence naming a state above `Watch` cannot be
constructed by any caller in any crate.

> **This requirement has been recorded as satisfied while violated three
> times.** 0.19.1 named one surface when there were two; 0.20.0 named two
> when there were three; and the 0.20.0 release was recommended on that
> basis. The first was inherited. The second and third are mine.
>
> The cause each time was the same: the clamp was applied to *the sites
> someone thought to check* rather than to *the type every rendering path
> must pass through*, and the test guarding it —
> `health_presentation_clamps_concern_to_watch_vocabulary` — matched a
> **capitalised** substring, so it passed against a page rendering
> "is a concern" in lowercase prose. A guard never observed failing is
> not evidence.
>
> Do not mark this `Implemented` again without a test that has been seen
> to fail.

**NFR-LANG-003 — Single interface language**
Interface strings MUST NOT mix languages. Code, comments, and
documentation MUST be English.
*Source*: `SPEC §34.1`, `KICK §7`. *Status*: Partial — audit is Phase E.
*Priority*: P2.

**NFR-LANG-004 — Locale-aware formatting**
Dates and numbers SHOULD be formatted per locale.
*Source*: `SPEC §34.2`. *Status*: Not implemented. *Priority*: P3.

**NFR-LANG-005 — Full internationalisation deferred**
Complete multi-language support is a future major-version concern; the
architecture SHOULD avoid decisions that would preclude it.
*Source*: `SPEC §34.1`, `KICK §7`. *Status*: Deferred. *Priority*: P3.

### 5.5 Security — `NFR-SEC`

**NFR-SEC-001 — Injection resistance**
All database access MUST use parameter binding. String interpolation of
user input into SQL MUST NOT occur.
*Source*: `KICK §3.4`. *Status*: Implemented. *Priority*: P0.

**NFR-SEC-002 — Output escaping**
User-supplied content MUST be escaped on render.
*Source*: `KICK §3.4`. *Status*: Implemented (framework-provided).
*Priority*: P0.

**NFR-SEC-003 — Password storage**
Passwords MUST be stored only as Argon2 hashes.
*Source*: `KICK §2.1`. *Status*: Implemented. *Priority*: P0.

**NFR-SEC-004 — No secrets in repository or release artefact**
Credentials MUST NOT appear in source, configuration, or release
archives. Deployment secrets are supplied by the environment.
*Source*: project convention. *Status*: Implemented; automated scanning
is advisory and not yet wired. *Priority*: P0.

**NFR-SEC-005 — No `unsafe` code**
Project code MUST NOT use `unsafe`. Any future use requires explicit
review and recorded justification.
*Source*: decision `DEC-010`. *Status*: Implemented. *Priority*: P1.

**NFR-SEC-006 — Vetted cryptographic dependencies**
Authentication and cryptographic primitives MUST come from maintained
upstream crates rather than bespoke implementations.
*Source*: `KICK §2.1`. *Status*: Implemented. *Priority*: P0.

**NFR-SEC-007 — Dependency review**
New dependencies SHOULD be reviewed for licence, maintenance status, and
security posture. `Cargo.lock` MUST be committed.
*Source*: project convention. *Status*: Implemented. *Priority*: P2.

### 5.6 Performance and scale — `NFR-PERF`

**NFR-PERF-001 — Target scale**
The system MUST perform acceptably for teams of roughly 5–30 users on a
single node with SQLite. Horizontal scaling is out of scope.
*Source*: `SPEC §1`, `KICK §1`. *Status*: Implemented. *Priority*: P1.

**NFR-PERF-002 — Server-rendered first paint**
Screens MUST be server-rendered so that first paint does not depend on
client-side hydration.
*Source*: `KICK §2.2`. *Status*: Implemented. *Priority*: P1.

**NFR-PERF-003 — Query support for hierarchy and status**
Frequent access paths — top-level issues by project and status, children
of a parent — MUST be index-supported.
*Source*: migration `0015` design. *Status*: Implemented (partial
indices). *Priority*: P2.

**NFR-PERF-004 — Analysis is not on the default path**
Expensive analysis MUST be disabled or simplified by default, and
computed on demand.
*Source*: `BRIEF §8`. *Status*: Implemented. *Priority*: P1.

### 5.7 Maintainability — `NFR-MNT`

**NFR-MNT-001 — Pure, testable computation**
Metric computation MUST be implemented as pure functions in the domain
crate, free of database dependencies.
*Source*: `BRIEF §3.1`, decision `DEC-001`. *Status*: Implemented.
*Priority*: P1.

**NFR-MNT-002 — Crate boundaries**
The workspace MUST separate domain computation, storage, authentication,
notification, and web presentation.
*Source*: decision `DEC-001`. *Status*: Implemented (six crates).
*Priority*: P2.

**NFR-MNT-003 — Module layout**
Rust 2018+ module style MUST be used (`foo.rs` beside `foo/`); `mod.rs`
MUST NOT be introduced.
*Source*: project convention. *Status*: Implemented. *Priority*: P3.

**NFR-MNT-004 — File size discipline**
A source file exceeding 300 effective lines SHOULD be considered for
splitting; exceeding 500 effective lines, strongly so.
*Source*: project convention. *Status*: Partial. *Priority*: P3.

**NFR-MNT-005 — Test organisation**
Tests MUST be separated from implementation files. In-file `#[test]`
modules inside implementation source MUST NOT be used.
*Source*: project convention. *Status*: Implemented. *Priority*: P2.

**NFR-MNT-006 — Tests validate specification, not code shape**
Test cases MUST derive from design specifications rather than mirroring
the implementation.
*Source*: project convention. *Status*: Implemented. *Priority*: P1.

**NFR-MNT-007 — Formatting and linting gates**
`cargo fmt --check` and `cargo clippy … -D warnings` MUST pass.
Formatting MUST run once after implementation completes, and the
formatted output MUST NOT then be hand-revised.
*Source*: `KICK §5.3`, project convention. *Status*: **Implemented
(0.20.0)** — and true for the first time. *Priority*: P1.
*Correction*: recorded `Implemented in CI` at 0.19.1. CI had **never
passed**: five runs, all failures, `fmt` and `clippy` red on every push
since the workflow was introduced, while all fourteen test and build jobs
passed throughout. 44 files had never been run through `cargo fmt` at all.
Cleared across DEV-006 (formatting), DEV-007 (`peisear-storage`, 21
findings), DEV-008 (`peisear-web`, 4) and DEV-005 item A (7 more unlocked
by the MSRV raise). See §10.9.
*Note*: "clippy clean" means clean modulo eight pre-existing
`#[allow(clippy::too_many_arguments)]` suppressions in `peisear-web` —
not that no function in the crate has too many arguments.

**NFR-MNT-008 — Error propagation**
`unwrap()` SHOULD be avoided in favour of propagated `Result` values.
*Source*: `KICK §5.3`. *Status*: Implemented. *Priority*: P2.

**NFR-MNT-009 — Decisions recorded with rationale**
Changelog entries MUST record why a change was made, not only what
changed.
*Source*: project convention. *Status*: Implemented. *Priority*: P2.

**NFR-MNT-010 — RFC lifecycle for significant change**
Significant changes MUST be specified in an RFC before implementation,
and the RFC MUST be retained after implementation as a rationale record.
*Source*: project convention; `rfcs/README.md`. *Status*: Implemented.
*Priority*: P2.

### 5.8 Release and packaging — `NFR-REL`

**NFR-REL-001 — Semantic versioning**
Releases MUST follow semantic versioning, with a single version shared
across the workspace.
*Source*: project convention. *Status*: Implemented. *Priority*: P1.

**NFR-REL-002 — Archive naming and layout**
Release archives MUST be named with the version suffix and MUST unpack
with files at the archive root, without an intermediate directory.
*Source*: project convention. *Status*: Implemented. *Priority*: P2.

**NFR-REL-003 — Release cadence**
Releases MUST be cut at logical boundaries — a resolved RFC, a completed
theme, or a finished compliance pass — rather than per work session. A
release MAY span multiple sessions.
*Source*: project convention. *Status*: Implemented. *Priority*: P2.

**NFR-REL-004 — Gate evidence before release**
Formatting, linting, and the full test suite MUST pass before a release
is cut.
*Source*: project convention. *Status*: Partial — see §9.4 on the
evidence limitation. *Priority*: P1.

**NFR-REL-005 — Reproducibility**
The release procedure SHOULD be reproducible and verifiable.
*Source*: derived. *Status*: Not defined. *Priority*: P2.

**NFR-REL-006 — Required project files**
The repository MUST contain `LICENSE`, `NOTICE`, `TERMS_OF_USE.md`,
`ROADMAP.md`, `CHANGELOG.md`, and CI configuration. Licence text MUST
NOT be reproduced in `README.md`.
*Source*: project convention. *Status*: Implemented. *Priority*: P2.

**NFR-REL-007 — Documentation structure**
`README.md` MUST remain concise and follow the ordering: hero, overview,
why/when, quick start, features or design notes, links to full
documentation. Full documentation MUST live under `docs/src` in an
mdbook-compatible structure, organised by reader persona.
*Source*: project convention. *Status*: Implemented. *Priority*: P2.

### 5.9 Compatibility and portability — `NFR-CMP`

**NFR-CMP-001 — Rust edition and MSRV**
The project MUST target Rust edition 2024 and MUST declare a minimum
supported Rust version. The toolchain SHOULD be pinned in-repository, and
the declared MSRV MUST be exercised by an automated build.
*Source*: `KICK §7`; extended by `DEC-044`/`DEC-045`. *Status*:
**Implemented (0.20.0)** — MSRV `1.88.0`, `rust-toolchain.toml` pins
`1.97.1`, and a dedicated CI job builds the workspace at the MSRV.
*Priority*: P2.
*Correction*: the declared MSRV of `1.85` was **false** — the dependency
tree had drifted past it and 1.85 could not build the workspace at all.
Nothing had ever exercised the claim.

Two distinctions this requirement now carries deliberately:

- **The pinned toolchain and the MSRV are different things.** The pin
  (`1.97.1`) exists for determinism of `fmt` and `clippy`; the MSRV
  (`1.88.0`) is the floor for building from source. Pinning development to
  the MSRV would put every contributor on the oldest supported compiler.
- **The MSRV is set by the dependency tree, not chosen.** It drifts upward
  on `cargo update`. The CI job exists to surface that drift the day it
  happens rather than three months later, which is how the false claim
  survived. Per `DEC-045`, MSRV is a documentation fact rather than a
  compatibility commitment — but it is also a **tooling input**: raising it
  unlocked seven MSRV-gated clippy lints, which is not obvious in advance.

**NFR-CMP-002 — Self-hosted cross-platform operation**
The system MUST be self-hostable and MUST NOT depend on a specific
cloud provider.
*Source*: `KICK §7`. *Status*: **Implemented, and verified without network
egress** (`ASSET-001`, 0.32.0). *Priority*: P1.

*This status rested on an untested assumption until 2026-09-08.* Measured with
both CDNs blocked, the application rendered **unstyled**: `.btn` 44 px → 17 px,
the type falling back to Times New Roman, the account menu unable to collapse.
Every guarantee RFC 012 established was contingent on `cdn.jsdelivr.net` and
`cdn.tailwindcss.com` being reachable **by the end user's browser** — which a
self-hoster does not control and an air-gapped deployment does not have.

**Nothing was broken** — every link, form and flow worked; `DEC-021`'s posture
held and the no-CSS path was a working document. **The claim was simply never
checked**, and the requirement said Implemented.

`DEC-051` vendored both into `static/`. Verified across fourteen pages with both
CDNs blocked: identical rendering, three stylesheets, 44 px controls, no
overflow.

**NFR-CMP-003 — Single-file database**
Persistent state MUST reside in a single SQLite file so that backup is a
file copy.
*Source*: `KICK §2.3`. *Status*: Implemented. *Priority*: P2.

**NFR-CMP-004 — Forward-only migrations**
Schema migrations MUST be forward-only and numbered sequentially. A
defective migration MUST be corrected by a further forward migration.
*Source*: implementation decision. *Status*: Implemented (0001–0015).
*Priority*: P1.

**NFR-CMP-005 — Additive schema evolution**
Existing tables SHOULD be extended with nullable columns rather than
restructured, so that existing rows remain valid without backfill.
*Source*: migration `0014` / `0015` practice. *Status*: Implemented.
*Priority*: P2.

**NFR-CMP-006 — Licence**
The project MUST be distributed under Apache-2.0, attributed to the
author.
*Source*: project convention. *Status*: Implemented. *Priority*: P1.

---

## 6. Data requirements

### 6.1 Core entities

| Entity | Key attributes | Notable invariants |
|---|---|---|
| User | id, email, display name, password hash, timestamps | Email unique; hash never plaintext |
| Project | id, owner, name, description, optional team, timestamps | Personal (no team) or team-scoped |
| Issue | id, project, author, title, description, status, priority, position, effort?, assignee?, parent?, timestamps | One level of nesting; same-project parent |
| Team | id, name, slug, description?, timestamps | Slug unique |
| Team membership | team, user, role, joined, timestamps | Last admin protected |
| Sprint | id, team, name, goal?, start, end, status, timestamps | At most one active per team |
| Sprint–issue association | sprint, issue, assigned timestamp | Top-level issues only |
| User capacity | id, user, points, period start?, period end?, note?, timestamps | Periods must not overlap |
| Issue event | id, issue, project, actor?, kind, from, to, timestamp | Append-only |
| Metrics snapshot | project or user scope, computed values, timestamp | Enables trend recomputation |
| Notification | id, user, kind, severity, body, read state, timestamps | Recipient-only access |

### 6.2 Event capture

`BRIEF §4.1` requires that indicators derive from recorded events. The
system MUST record: issue creation; status change; comment addition;
assignee change; completion; effort change; deadline change.
*Status*: Implemented for creation, status, assignee, effort, and
completion. Comment and deadline events are not yet applicable (no
comment feature; no deadline attribute until `FR-CAL-003`).

### 6.3 Recomputation

Aggregations MUST be recomputable from retained history rather than
depending solely on incrementally maintained totals.
*Source*: `BRIEF §3.3`. *Status*: Implemented via snapshot tables.

### 6.4 Retention

Audit-oriented logs MUST have a defined retention period, distinct from
operational-improvement history.
*Source*: `BRIEF §2.5`, `BRIEF §8`. *Status*: Not defined — proposed
default is 30 days for audit records and 90 days for issue events
(RFC 0005). *Priority*: P2.

### 6.5 Migration inventory at this baseline

`0001` initial schema · `0002` issue effort · `0003` issue assignee ·
`0004` user capacity · `0005` personal limits · `0006` issue events ·
`0007` metrics snapshots · `0008` user metrics snapshots ·
`0009` user capacities · `0010` notifications · `0011` teams ·
`0012` sprints · `0013` user view states ·
`0014` `updated_at` columns and auto-bump triggers ·
`0015` sub-issue column, partial indices, and constraint triggers.

`0016` calendar planned-date columns (`FR-CAL-003`) — **applied at 0.23.0**.

*Corrected at 0.24.0*: an earlier version of this line also reserved `0017`
for the deferred email opt-in state (`FR-NTF-007`). **No such migration
exists or is needed.** RFC 003's first version proposed
`users.email_opt_in` / `email_opt_in_prompted_at`; its rewrite found both
facts already held by the `notification_preferences` global row, and
`FR-NTF-007` shipped at 0.24.0 with no schema change at all. `0017` is
unreserved and free.

The reservation was mine, and it survived in this document for four
releases after the design that needed it was discarded — which is the same
shape as the defects §10 records, in the register's own housekeeping.

*At 0.28.0*: **`0017_updated_at_single_authority.sql`** — the first migration
since `0016` at 0.23.0. Three triggers, on `issues`, `projects` and
`user_view_states`, giving `updated_at` one authority (`NFR-CONC-003`).

**Rollback is not forward-fix.** A downgrade to 0.27.0 **fails to start**:
`sqlx::migrate!` does not tolerate an applied migration absent from its
embedded list, and the binary migrates unconditionally. Recovery is a restore
from a pre-migration backup. The triggers themselves are harmless to the older
binary — `0014`'s `WHEN` clause keeps a trigger inert while application code
writes the column, which is what 0.27.0 does — so the obstacle is `sqlx`'s
bookkeeping, not the schema.

`0018` is the next free number.

*At 0.27.0*: **no migration.** None of 0.25.0, 0.26.0 or 0.27.0 touched
`crates/*/migrations`. `0016` remains the most recent, and `0017` remains
free. All three are forward-fix only, with no schema change to reverse.

`0015`'s `ON DELETE CASCADE` on `issues.parent_issue_id` is unchanged and was
**demonstrated** for the first time at 0.27.0 (`FR-SUB-009`). It had been
relied on since 0.19.1 and asserted by nothing — a schema guarantee the
application depended on, with `foreign_keys(true)` set on the pool and no test
proving either half.

---

## 7. External interface requirements

### 7.1 URL and permission summary

Authoritative matrix: `GUI §6`. Summarised, with post-redesign paths:

| URL | Anonymous | Personal user | Team viewer | Team member | Team admin |
|---|---|---|---|---|---|
| `/`, `/login`, `/register` | View | Redirect | Redirect | Redirect | Redirect |
| `/today` (was `/me`) | No | Own only | Own only | Own only | Own only |
| `/today/calendar` | No | Own only | Own only | Own only | Own only |
| `/inbox` (was `/notifications`) | No | Own only | Own only | Own only | Own only |
| `/projects` | No | Accessible | Accessible | Accessible | Accessible |
| `/projects/{id}` | No | If accessible | Read | Read/write | Manage |
| `/projects/{id}/issues/{iid}` | No | If accessible | Read | Read/write | Manage |
| `/projects/{id}/issues/{iid}/edit` | No | If accessible | No | Yes | Yes |
| `/projects/{id}/issues/{iid}/status` | No | If accessible | No | Yes | Yes |
| `/projects/{id}/issues/{iid}/status/{board\|detail\|list}` | No | If accessible | No | Yes | Yes |
| `/projects/{id}/delete` **(GET confirms, POST deletes)** | No | Owner only | No | No | Manage |
| `/projects/{id}/issues/{iid}/delete` **(GET confirms, POST deletes)** | No | If accessible | No | Yes | Yes |
| `/teams/{slug}/sprints/{sid}/delete` **(GET confirms, POST deletes)** | No | Member only | No | No | Manage |
| `/projects/{id}/issues/{iid}/sub-issues/new` | No | If accessible | No | Yes | Yes |
| `/projects/{id}/calendar` | No | If accessible | Read | Read | Read |
| `/teams/{slug}` | No | Member only | Read | Read | Manage |
| `/teams/{slug}/sprints/{sid}` | No | Member only | Read | Read/write | Manage |
| `/teams/{slug}/sprints/{sid}/plan` | No | Member only | Read | Read/write | Read/write |
| `/settings*` | No | Own only | Own only | Own only | Own only |
| `/search` | No | Own scope | Own scope | Own scope | Own scope |
| `/api/users/{uid}/*` | 401 | Self only | Self only | Self only | Self only |

**On the three `delete` rows (0.25.0, RFC 010).** `GET` renders a
server-side interstitial naming the specific project, issue or sprint;
`POST` performs the delete and is unchanged. The `GET` half exists because
the previous confirmation was a JavaScript dialog bound to the submit
event: **without JavaScript no confirmation ran at all**, so the degraded
path was more dangerous than the enhanced one. The sprint route additionally
refuses an `Active` sprint (`FR-SPR-002`).

**On the four status rows (0.25.0-0.26.0, RFC 004a/004b).** One route per
surface rather than one shared route, so a surface's fallback and its
redirect target are its own. `/status/board` predates the others.
`change_status` returns `200` with the new `updated_at` where it returned
`204`, because a client that continues without reloading needs that value
to keep its `client_updated_at` fresh.

### 7.2 HTTP status semantics

| Situation | Status | Notes |
|---|---|---|
| Unauthenticated, HTML route | 303 | Redirect to login |
| Unauthenticated, `/api/*` | 401 | JSON body |
| Authenticated, another user's personal data | 403 | Including administrators |
| Authenticated, non-existent user's personal data | 403 | Deliberately indistinguishable from the above |
| Validation failure | 400 | Neutral message; input preserved |
| Optimistic-lock mismatch | 409 | Structured body per `SPEC E.3.3` |
| Renamed route | 308 | Method preserved |

### 7.3 Mutation contract

Every owned-entity mutation MUST carry the client's observed
`updated_at`. Absence MUST be rejected (`NFR-CONC-005`). The server
compares, then either updates and returns the new timestamp, or returns
409 with the current timestamp, entity type, and entity identifier.

### 7.4 Form contract

`GUI §5` is authoritative. Validation MUST preserve user input on
failure (`GUI §9`), and messages MUST follow §1.7.

### 7.5 Naming conventions

`GUI §8` is authoritative: page components `{Domain}{Purpose}Page`;
forms `{Entity}{Action}Form`; read-only panels `{Entity}{Purpose}Panel`;
small status elements `{Meaning}Badge` / `{Meaning}Chip`; route
parameters in snake_case; boolean UI state prefixed `is_` / `has_` /
`can_`.

---

## 8. Definition of Done

`SPEC §41` fixes five completion conditions. Current assessment:

| # | Condition | Requirement anchors | Assessment |
|---|---|---|---|
| 1 | **Privacy maintained** — personal data boundaries hold in UI and API | `NFR-PRIV-001..008` | **Met** (0.37.0), having read *Largely met* since 0.19.1. `NFR-CONC-003` **corrected after 0.27.0** — it read `Implemented` while the application wrote `updated_at` on the two entities the optimistic lock most protects. `NFR-PRIV-007` **implemented after 0.27.0** — its first instance since the requirement was written. **RFC 005 §1's authorisation audit completed at 0.27.0 and found no reachable boundary** — every endpoint carrying personal data or a per-user mutation now has its check named and, where a cross-user attempt is constructible, a test. *Gaps, restated 2026-09-23 because the previous list had gone stale in both
directions.* **Storage-layer defence in depth is not a gap** — `§10.3` was reconciled at 0.28.0 and its entry now reads *"entry was wrong"*: the outcome `SPEC §11.5.4` prescribes was already achieved by scoping, with two independent barriers, and the guard that was genuinely missing was added. **Aggregate suppression is not a gap** — `NFR-PRIV-007` is Implemented with six acceptance tests. **`NFR-PRIV-008` closed at 0.37.0** (`PRIV-001`): five assertions — not the seven I first counted, see that requirement — every one covering behaviour already measured correct, and demonstrated load-bearing by planting. **With it, this row reaches Met**: the boundaries were audited at 0.27.0 and found unreachable, and every constructible cross-user request now has a test that fails if the barriers come down |
| 2 | **Quiet by design** — edge-triggered, cooled down, not chasing | `FR-NTF-001..003`, `FR-NTF-006..007` | **Met.** Silence-resume and the opt-in prompt shipped at 0.24.0 (RFC 003) |
| 3 | **Explainable** — indicators state what they do and do not show | `FR-HLT-005..009`, `NFR-A11Y-003` | **Met, with one limb outstanding** (0.29.0). Basis routes and calculation disclosures ship; chart tables and summaries ship. **`FR-HLT-007`'s history limb is deferred** on `NFR-PRIV-007` grounds, and WIP compliance is excepted from the basis route for the same reason. `FR-HLT-005`'s own acceptance citation was withdrawn at 0.28.0 — its cited test cannot fail — so the condition rests on implemented-and-inspected behaviour there rather than on a test |
| 4 | **Light by default** — analysis does not obstruct basic use | `FR-PER-006..007`, `FR-PROJ-004` | **Met.** One-click status change shipped at 0.25.0 (no-JS) and 0.26.0 (in place, with undo) on the issue list, issue detail and board |
| 5 | **Reach** — keyboard, screen reader, and mobile flows succeed | `NFR-A11Y-001..007` | **Met** (2026-09-10) — **the first time this row has read "Met", and it has been open since 0.19.1.** Mobile completion (`NFR-A11Y-006`) verified by driving all four named flows to completion at three phone viewports, both status paths. `NFR-A11Y-007` is met **with three call sites excluded by design and recorded** — a met requirement with documented exceptions, not an unmet one. *Prior wording, and the route:* Touch targets (`NFR-A11Y-007`) are met with a named, counted population — three call sites excluded by design, one declared exception — and the rule is now structurally guarded across every `<a>`, `<button>` and `<summary>`, not the class-carrying subset. **`NFR-A11Y-006` remains open**: a browser inspection found all four flows render and operate at 390 × 844, which is evidence the Phase E audit will probably pass and **is not that audit**. *This row was moved to this wording at 0.31.0 on an unmeasured claim, held on 2026-09-08 when the claim was measured and failed, and moved here once — deliberately not twice.* Earlier detail: Keyboard (`NFR-A11Y-001`) audited with no structural failure — from markup, not browser-driven. Contrast (`NFR-A11Y-005`) measured and repaired across 111 sites. Live regions (`NFR-A11Y-008`) implemented. Chart equivalence (`NFR-A11Y-003`) met in full at 0.29.0 (§10.4). **Touch targets (`NFR-A11Y-007`) met with a named limit and guarded** in 0.31.0 (RFC 012): all 139 controls reach 44 px, the fact has one home, and the guard makes its absence unconstructible — plain `<a>` links stay outside the counting method, unassessed rather than passing. **Still open: mobile completion (`NFR-A11Y-006`).** *Open since 0.19.1 and the oldest condition in this table; this is the first time it has moved.* It moves to **met-with-an-outstanding-limb, not to met** — the same distinction item 3 drew at 0.29.0, and for the same reason: the word is worth nothing if it is spent early |

---

## 9. Verification and traceability

### 9.1 Test inventory at this baseline

Measured per target on 2026-08-27 at `0.28.0`, not carried forward.

| Suite | Tests | Primary requirements verified |
|---|---|---|
| `auth_boundary` | 16 | `FR-AUTH-002/005`, `FR-API-002/003`, `NFR-PRIV-003/006`; **bulk-route parity** — `mark-all-read` asserted in both directions (`QA-007`, `QA-008`) |
| `optimistic_lock` | 16 | `NFR-CONC-001/005` — **per entry point**, and from 0.27.0 per *route*: every locking route has a `409` test naming it |
| `confirmation` | 13 | RFC 010, external design `§17.4`; the four interstitials, the active-sprint refusal, and the `POST`-without-`GET` boundary per route |
| `smoke` | 12 | `FR-AUTH-003/004`, `FR-NAV-002`, `FR-PER-001`; `search.js` referenced in the app shell |
| `status_control` | 12 | `FR-ISS-005/006`, `FR-DM-001/003/004/006`, RFC 004a |
| `sprint_plan` | 12 | `FR-SPR-*`, RFC 001 |
| `calendar_surfaces` | 10 | `FR-CAL-*`, RFC 002 |
| `search` | 9 | `FR-SCH-001..004` |
| `health_explainability` | 9 | `FR-HLT-005/008`, `NFR-LANG-002` |
| `assignee_candidates` | 8 | `FR-ISS-004`, `NFR-PRIV-002`, RFC 009 |
| `sub_issues` | 8 | `FR-SUB-001/006/007`, **`FR-SUB-009`** — the cascade, verified at 0.27.0 |
| `calendar` | 7 | Migration `0016`, both triggers, the two window queries |
| `inbox_refinements` | 7 | `FR-NTF-005..007`, RFC 003 |
| `board_keyboard` | 6 | `FR-DM-002`, `NFR-A11Y-001/004` — **not `NFR-A11Y-007`; see below** |
| `view_state` | 5 | `FR-NAV-005` |
| `workload_privacy` | 4 | `NFR-PRIV-001/002` |
| `issue_edit_url` | 3 | `FR-ISS-004` |
| `today_panel` | 3 | `FR-PER-006/007` |
| `breadcrumb` | 2 | `FR-NAV-003` |
| `status_segment` | 2 | `FR-ISS-005/006` |
| `board_card_lock_value` | 1 | `NFR-CONC-001`, `DEC-052` — every board card renders a non-empty lock value, per card; RFC 011 step 3's replacement (`LOCK-001`, 0.33.0) |
| `peisear-web` library | 14 | `NFR-LANG-001` (`prose_scan`, now over **all** of `src/`; `static_js_scan`, now recursive), `FR-NTF-003`, `§10.13` (`test_harness_scan`), `DEC-007` block parity (`dec_007_scan`) and **CI parity** (`dec_007_ci_scan`) |
| `peisear-i18n` | 17 | `NFR-LANG-001`, `NFR-LANG-003`, `FR-HLT-006`, `NFR-PRIV-001`; **`MessageKey::all()`'s own completeness**, the fourteen label enums, and non-Latin script in the English renderer |
| `peisear-notify` | 6 | `FR-NTF-001/002/003` |
| `peisear-core` library | **3** | The notification kind and channel constant lists — **this crate had zero tests in every release before 0.27.0** |
| `peisear-storage` library | 2 | `FR-SCH-004` |
| `peisear` facade | 1 | binary wiring — a doctest, not a unit test |

Totals: **187 web integration test functions, 0 disabled**, plus 53 library and
crate-level tests — **240 active at the 0.29.0 tag**, **254 at the 0.31.0 tag**,
**255** at the 0.32.0 tag (`TT-004`'s one guard test), and **256** at the time of
writing (`LOCK-001`'s one test).

**The count is flat across 0.32.0 and that is the point of the release.**
`LAYOUT-001`, `LAYOUT-002` and `ASSET-001` added **no tests at all**, because
nothing in this suite can observe rendered layout — which is `§10.18`'s finding
and the reason RFC 011 step 4 exists.

**Across 0.37.0 it grew by four, to 272, and nothing else changed at all** —
`PRIV-001`'s authorisation assertions, in two existing files. **Every one
covers behaviour that was already correct**, which is what a regression test
is for; the plant that proved them load-bearing is in that handoff's review.

**Across 0.36.0 it grew by eleven, to 268** — `CAL-003`'s, across the two
existing files it extended, so `DEC-007`'s command block did not change. They
assert the new endpoint (including a conflict routed through the real shared
lock function rather than a hand-written `409`), the day view's markup, and the
copy island. **None performs a drag**, for the same reason as `PLAN-002`'s six.

**Across 0.35.0 it grew by six, to 262** — all of them `PLAN-002`'s, in the
existing `sprint_plan` file, so `DEC-007`'s command block did not change.
**None of the six performs a drag**: they assert the rendered markers, the copy
island and the server. That is the most this suite can assert about a pointer
gesture, and `§10.15` is why it is not more.

**Across 0.34.0 it did not move at all, and neither did product behaviour.**
`LAYOUT-009` changed three class strings; `TT-005` was a measurement and
deliberately produced no guard (`§17.8` records why overlap is not gated);
`TT-006` changed a doc comment. A release can be worth cutting and add nothing
to this table.

**Across 0.33.0 it grew by one while nine layout defects were fixed**, and that
is also the point. `LAYOUT-003` through `LAYOUT-007` added no tests, because the
layout is now observed by `BROWSER-001`'s gate — eighteen pages at five widths,
one assertion, a CI job outside this inventory by design (external design
`§17.8`). A `cargo test` that reads markup cannot see what those fixes changed;
the gate can, and was watched go red on each of them.

(225 at 0.28.0; 207 at 0.27.0; 178 at 0.26.0; 172 at 0.25.0; 151 at 0.24.0;
144 at 0.23.0; 125 at 0.22.0; 104 at 0.21.0; 82 at 0.20.0; 65 active with 1
disabled at 0.19.1.)

**Pinned-CSS questions are answerable from the repository.** `contrast_scan` and
`touch_target_scan` assert values resolved from `daisyui@4.12.14`, and since
`ASSET-001` (0.32.0) **that exact file is `static/daisyui.min.css`** — tracked,
shipped, and the same bytes the product serves. A resolved-value question is a
`grep` away and is never *"unanswerable without network access"*.

*Superseded convention, kept because it explains the guards' older wording.*
Before vendoring, the guards reasoned about a file that was not in the
repository, and this section pointed at a scratch copy in `.git-exclude/tmp/`.
`TT-001` had found sub-passes reporting resolved-value questions as unanswerable
without checking whether that copy existed. **The guards now name the shipped
file**, so the question cannot arise in that form again.

**`peisear-web`'s library target now carries twelve scan modules** —
`contrast_scan`, `dec_007_ci_scan`, **`dec_007_fs_scan`**, `dec_007_scan`,
`dm_fallback_boundary_scan`, `one_encoder_scan`, `prose_scan`, `static_js_scan`,
`test_harness_scan`, `touch_target_scan`, `untrusted_id_scan`,
`updated_at_authority_scan`. Each makes one defect class unconstructible rather
than merely absent, and each was added because a defect of that class had
already been found.

`dec_007_fs_scan` is new in 0.31.0 (built, unreleased) and closes `§10.16`'s reopening.
`touch_target_scan` grew from banning one class to enforcing the whole of
`NFR-A11Y-007`'s size clause — see `§10.17` for what its own review found.

**A coverage claim in this table was unbacked, corrected 2026-08-26.**
`board_keyboard`'s row listed `NFR-A11Y-007` (44 × 44 touch targets) among what
it verifies. It does not. Its six tests assert the form's route, the lock
token, the reachable target statuses, accessible names and vocabulary —
**nothing anywhere in the suite asserts a touch-target dimension**, including
on `issues.rs:661`, the single control in the product that carries
`min-h-11 min-w-11`.

The claim predates this edition and I carried it forward when rewriting §9.1 at
0.27.0 rather than checking it. It is the defect class this baseline's first
edition was written to correct, committed in the section that records how
verification is claimed.

**209 at the time of writing, not 207.** The two beyond the release are
`QA-011`'s live-region test and `QA-013`'s `contrast_scan`, both landed after
the tag. The table above is the release state; this line is the tree state, and
the difference is named rather than folded in — a baseline that silently
absorbs post-tag work stops being able to say what shipped.

**Twenty-nine added since the `0.26.0` tag.** Recorded from the tag rather than
from any intermediate state, because a release counts from the last release —
an earlier draft of `REL-0.27.0`'s handoff took its figure from the post-tag
count in this section's previous edition and was three short. Caught in review
by the implementer, who queried it rather than reverse-engineering a subset
that would fit.

**The 0.21.0 caveat on this inventory is withdrawn at 0.22.0.** It read: these
counts come from per-crate and per-target runs, and under a single
`cargo test --workspace` the suite fails roughly one run in two for reasons
unrelated to any requirement (§10.13). That is fixed. The counts above hold
under both procedures, and three consecutive `cargo test --workspace` runs are
now part of the release gate.

**A convention adopted at 0.26.0 from `QA-004`, discharged at 0.27.0.** The
facade's single test is a doctest, so `cargo test -p peisear --lib` reports
zero for it; that crate had no line in the `DEC-007` command block at all, and
two release candidates' per-target tables carried a count no command produced.
Both that line and `cargo test -p peisear-web --lib` are now in the block, and
**both are now in CI** — see `§10.16`.

Two conventions adopted at 0.20.0, both from defects this release found:

1. **Coverage is recorded per entry point, not per requirement.** An
   endpoint reachable by more than one route (`change_status` takes JSON
   and form) needs a test per route. Recording it per requirement is what
   let `NFR-CONC-005` read as covered while the path the shipped client
   used was untested. **0.27.0 tightened this**: `QA-006` found five routes
   with a working lock and no test naming that route, plus one route
   (`/settings/capacity/{id}/close`) in neither the audit table nor the suite.
2. **A test crate without a CI job does not exist.** All thirteen were
   swept against `.github/workflows/test.yml` during DEV-009; all thirteen
   have one. **This convention was itself unenforced until 0.27.0** — see
   `§10.16` — and is now asserted by `dec_007_ci_scan`, target by target.

### 9.2 Requirements without automated verification

> **This list is known to be badly incomplete. Audited 2026-08-26 (`QA-018`);
> not yet corrected, deliberately.**
>
> This document carries **162** requirement blocks. **125** are marked
> `Implemented`: **40** cite an acceptance test, **85** cite nothing. This
> section names **three**.
>
> **Of the 40 citations, 15 do not verify what they claim** — 8 partial, 7 not
> at all. In every case traced, the underlying code is correct and the citation
> overstates what the test checks. `FR-HLT-005` is the extreme: its cited test
> cannot fail (see §4.9).
>
> **Of the 85 without a citation, a 15-item sample found 8 with no test at all,
> 4 partial, 2 holding.** That is a sample and is labelled one; it is not
> extrapolated. But it points the opposite way from the comfortable
> explanation, which was that most have tests nobody cited.
>
> **Why this is recorded rather than fixed.** Correcting 15 citations and
> expanding this list would make the document consistent without establishing
> why it became inconsistent. The owner's direction was to diagnose before
> acting, and that is the right order: every one of the 15 was written once,
> when its requirement was annotated, and nothing has ever re-read a citation
> against the test it names. **This document has guards for its claims about
> the code and none for its claims about itself** — the same asymmetry `§10.16`
> had, one level up.
>
> **The diagnosis landed 2026-08-27 and its rule is adopted — see `§9.5`.** The
> 15 are not yet corrected: the rule governs from adoption forward, and this
> list is still a floor rather than a count. **Read a citation written before
> 2026-08-27 as a pointer, not as evidence.**

The following are implemented but unverified, and are the highest-value
targets for new tests: `FR-TEAM-003` (last-admin guard), `FR-TEAM-004`
(non-member concealment), `FR-TEAM-005` (privacy footnote).

**`NFR-A11Y-005` left this list post-0.27.0** and is the first entry ever to
leave it by **measurement** rather than by a test being written — the guard
that now holds it (`contrast_scan`) enforces a range derived from an audit, and
the audit itself is not reproducible by the suite. Recorded that way in §5.3:
what is guarded is narrower than what was measured.

**`FR-SUB-009` left this list at 0.27.0** — the cascade is now demonstrated by
a test written *before* the copy that describes it. Three of the original seven
have now gone, each for a different reason: a guard shipped at 0.21.0 and this
section did not notice for five releases (`FR-HLT-006`, `NFR-LANG-001`); a
guard shipped with the feature (`FR-SPR-002`, 0.25.0); and one was demonstrated
because a handoff required it before dependent copy could be written.

**Three left this list.** `FR-HLT-006` and `NFR-LANG-001` were listed as
having no vocabulary guard; one shipped at **0.21.0** (RFC 006) and this
line did not notice for five releases. `FR-SPR-002` gained one at 0.25.0 —
the `confirmation` suite asserts an `Active` sprint cannot be deleted.

A list of "implemented but unverified" that is itself unverified is the
register's own defect class, and this is its second instance after §6.5's
`0017` reservation.

### 9.3 Disabled tests

**None.** `cross_user_settings_post_returns_403` was **withdrawn** at
0.20.0 with recorded cause: no user-scoped POST endpoint exists, because
settings mutations are self-scoped by session rather than addressed by
`user_id` in the path, so the boundary it asserted cannot exist. It should
be reinstated if `FR-API-006` lands.

Withdrawn rather than left `#[ignore]`d, because **an ignored test on a
privacy boundary reads like coverage that does not exist.**

### 9.4 Evidence

A clean cold-cache run of every gate was captured at 0.20.0: `cargo fmt
--check` (exit 0), `cargo clippy --workspace --all-targets -- -D warnings`
(exit 0), and the full per-crate test suite (0 failures). The release
artefact was verified by extraction, checksum, and file-count parity with
the committed tree, and the extracted copy builds.

This closes `RSK-001`, outstanding since 0.19.1.

**Correction to the 0.19.1 record.** That baseline stated the missing
cold-cache run "does not indicate a known failure". **That was false.**
Continuous integration had failed on every push since it was introduced —
five runs, zero successes — with `fmt` and `clippy` red throughout. The
risk was never "evidence not captured"; it was "gates red and unobserved".
The sentence propagated through handoff documents for three months.

The lesson is recorded rather than the incident: *absence of evidence was
reported as evidence of absence.* Where this baseline now says a gate
passes, a captured log says so.

**Known verification limits at 0.20.0**, stated rather than implied:

- Drag rollback (`FR-DM-004`) and keyboard operability (`FR-DM-002`) are
  verified through the HTTP contract each path produces, plus code review
  — not by driving real drag or Tab/Enter input in a browser. No such
  tooling is available in the development environment.
- The `peisear-notify` suite passes only single-threaded; a shared SQLite
  file makes it flake under parallel execution. CI's `--test-threads=1`
  masks this, which makes it a trap for anyone running `cargo test`
  naively.

---

### 9.5 What an acceptance clause must name (normative)

*Adopted 2026-08-27 by the owner, from the `QA-018` audit and its diagnosis.*

> **An acceptance clause MUST name something that is not the requirement.**
>
> A test function, a test crate, a file, an exact rendered string, a concrete
> observable value. **If the clause can be derived from the requirement text
> alone, it is not evidence — it is the requirement, twice.**

**Naming the mechanism does not satisfy this.** *"trigger-enforced"*,
*"`validator` derives"*, *"the FK carries `ON DELETE CASCADE`"* all name an
implementation. An implementation's existence is not proof that anything
exercises it — `QA-006` found `ON DELETE CASCADE` correct and untested, and
wrote the test before the copy that described it.

#### Why this rule and not a rule about care

The 40 clauses in this document divide cleanly, and not by how carefully they
were written:

| | |
|---|---|
| *"navigation renders five entries"* | the requirement, restated — **does not hold** |
| *"title 1–200 characters, description ≤ 10,000"* | restated — **does not hold** |
| *"trigger-enforced"* | the mechanism — **does not hold** |
| `breadcrumb` test crate | locatable — **holds** |
| `self_can_read_own_*` | locatable — **holds** |
| the exact string *"management role, not an oversight role"* | locatable — **holds** |
| `308` with exact `Location`s | locatable — **holds** |

A restatement cannot be checked by anyone, including its author, because
checking it means checking the requirement against itself. **The failing
clauses were not written carelessly; they were written in a form no amount of
care could make checkable.**

#### What this rule does not reach

It constrains the **shape** of a citation, not its **truth**. `FR-HLT-005`
named a real suite and a real function whose body was
`if body.contains("Throughput") { let _ = body; }` — locatable, and evidence of
nothing. **No textual rule reaches that**, and this section does not pretend
otherwise. Whether a named test asserts its requirement stays a review
question.

#### Status of the two recommendations this rule arrived with

- **Correcting the 15 clauses that restate their requirements**: not done. The
  rule governs from adoption; the backlog is separate and is the architect's.
- **A guard enforcing the rule**: not built, and not to be built before
  `DEC-020`. *(Superseded 2026-09-24: this document is now tracked at
  `docs/specification/`, so a scan over it can reach it.)* When it lived in
  the private working area a scan over it could not
  run in the workspace suite as the seven code guards do. **Building one that
  cannot run in CI would be the fifth instance of the defect Phase E spent
  itself closing** (`§10.16`).

Until then this is enforced the way every other document rule in this project
is: at review.

## 10. Compliance gaps

Divergences between this requirements baseline and the shipped
implementation, discovered by direct comparison of `SPEC §28` and
`GUI §9` against the code. They are recorded here rather than resolved
silently in either direction.

### 10.1 Indicator set differs from the specification

`SPEC §28.1` names six indicators: WIP load, Long-stale, Pace, Effort
balance, Dependency tightness, Composite. The implementation provides
Throughput, Staleness, Activity, Bus factor, Long-stale, and WIP
compliance, with the composite carried as an aggregate score.

Four map closely (WIP load ↔ WIP compliance; Long-stale ↔ Long-stale;
Pace ↔ Activity and Throughput; Effort balance ↔ Bus factor).
**Dependency tightness has no implementation** and no supporting data
model — issues carry no dependency relationships.

*Assessment*: the divergence is substantive but defensible; the shipped
set measures what the available event data supports. Two resolutions are
open: revise `SPEC §28.1` to the implemented set, or specify an issue
dependency model and add the indicator. `FR-HLT-001` is written to the
implemented set pending that decision.

### 10.0 How to read this register at 0.29.0

Gaps are recorded rather than resolved silently in either direction, and
**closed gaps stay here with their resolution**. A register that only lists
open items cannot show whether a class of defect recurs.

| Gap | State at 0.37.0, updated as work lands |
|---|---|
| §10.1 indicator set differs from `SPEC §28.1` | Decided (`DEC-030`), amendment pending |
| §10.2 health presentation exceeds the ceiling | **Closed** |
| §10.3 storage-layer authorisation absent | **Narrowed and guarded** — 0.28.0+; entry was wrong |
| §10.4 explainability affordances incomplete | **Closed, partially** — 0.29.0; history deferred |
| §10.5 calendar schema scope reduced | Open, deliberate — **shipped as designed** at 0.23.0 |
| §10.6 kanban endpoint bypassed the optimistic lock | **Closed** |
| §10.7 capacity disclosed to non-subjects | **Closed** |
| §10.8 domain crate generates user-visible prose | **Closed** — 0.21.0 |
| §10.9 `fmt`/`clippy` had never passed | **Closed** |
| §10.10 `/today` rendered danger colouring | **Closed** |
| §10.11 workload and assignee queries return only the owner | **Closed** — 0.22.0 |
| §10.12 health **summary prose** names the unclamped state | **Closed** — 0.20.1 |
| §10.13 the integration-test harness collides with itself | **Closed** — 0.22.0 |
| §10.14 the board's script tag was guarded by nothing | **Closed** — after 0.26.0 |
| §10.15 the shipped JavaScript is executed by no test | Open; **the gap is permanent, its size is not** — 820 lines in three files when declared permanent at 0.33.0, **1,663 in five** at 0.37.0. The old rationale ("a failure is immediate and local") was falsified by `PLAN-002` round 1, which passed every gate and reached review. No tool is bought; two required evidence runs are recorded instead, each having caught one defect no gate could — see `docs/static-js-verification.md` |
| §10.16 the four structural guards have no CI job | **Closed** — 0.27.0; **reopened and re-closed** in 0.31.0 |
| §10.17 assertions keep passing while no longer testing what they name | Open, **recorded not scheduled** — 11 instances found and fixed in 0.31.0 |
| §10.18 authenticated pages scroll horizontally when the signed-in email is long | **Closed** — `LAYOUT-001`, 0.32.0 |
| §10.20 the project-detail toolbar does not fit a phone | **Closed** — `LAYOUT-002`, 0.32.0 |
| §10.21 an empty-text anchor overlaps the issue-row link by 314 px | **Closed, not a defect** — my own measurement artefact, 2026-09-10 |
| §10.22 the touch-target guard was bypassable three ways | **Closed** — `TT-004` rounds 1-3, 0.32.0; each bypass demonstrated by planting |
| §10.23 the product's CSS is loaded from two CDNs at runtime | **Closed** — `ASSET-001`, 0.32.0; vendored, verified with both CDNs blocked |
| §10.24 the board's columns overflow at 320 px on an unbreakable issue title | **Closed** — `LAYOUT-003`, 0.33.0; `min-w-0` + `break-words`, each alone insufficient |
| §10.25 unbreakable user text overflows **thirteen** surfaces, in **three shapes** with non-interchangeable remedies | **Closed in tree** — found 2026-09-12 by `LAYOUT-003`'s sweep; `LAYOUT-004` five sites, `LAYOUT-005` two, `LAYOUT-007` three, `LAYOUT-008` five (the sprint pages — one overflowing at 1280 — and the issue form's workload chips, the **third shape**, reached only by `overflow-wrap: anywhere`); each handoff's gate pages watched red first; the rule recorded once with three rows; the gate's fixture carries an unbroken run in every user-text field and its list is eighteen pages. 0.33.0 |
| §10.26 every page overflows a 320 px phone when the display name is 21 characters | **Closed in tree** — `LAYOUT-006` (`468e7bd`), 0.33.0; a **third mechanism** (our own `flex-none`, not `min-width: auto`); the name truncates only when the row cannot fit; 320 px landed (`188056e`, 70/70) and plant D holds it: 14 cells, all at 320 |
| §10.27 page header rows squeeze the title instead of wrapping — sprint detail's `h1` is 54 px wide and eleven lines deep at 320 px with an ordinary name | **Closed in tree** — found 2026-09-12 by `LAYOUT-008`'s package (§5); `LAYOUT-002`'s mechanism on a header. `LAYOUT-009` (`b458943`, `fbe5a6b`, both after the 0.33.0 cut) wraps all **three** headers of this shape — issue detail, sprint detail, team detail: the actions move below only when the row cannot fit, and the title keeps the full content width. A sweep of every `justify-between` row found no fourth. **Passed the overflow gate throughout**, before and after. 0.34.0 |
| §10.28 the touch-target guard's module doc denies the exception list the module has carried since `TT-004` | **Closed** — `TT-006`, 0.34.0; doc only, the count unmoved, and the old paragraph's own prediction recorded rather than overwritten |
| §10.19 the adjacency guarantee is narrower than recorded | **Closed** — `TT-005`, 0.34.0. The `join` 1 px seam is real, deliberate and now carved out of the clause; the two `<summary>` pairs are **withdrawn**, unreproducible on `221e074`, the tree they were measured from, with the probe validated by a plant on that same tree |

Twenty-three closed; §10.1, §10.3 and §10.5 decided or deliberate; **two open: §10.15 and §10.17** — both open by decision rather than by schedule, and both recorded as such. Of the six closed by 0.20.0/0.20.1, **five were not
recorded as gaps at
0.19.1 at all** — they were requirements annotated as satisfied. That is the
pattern this release exists to break, and §10.0 exists so the next reader can
see whether it held.

**It did not hold at 0.20.0.** §10.12 was found days after this baseline was
written, by the same means as the others — someone reading the code for an
unrelated reason. §10.2 and §10.10 were closed while a third instance of the
same violation, on the same screen, went unexamined.

**At 0.21.0 it held in one direction and not the other.** No new instance of
the ceiling violation appeared, and §10.8 closed. But §10.13 is the same
pattern in a new place: a defect sitting under a green gate, invisible because
of how the gate was run rather than because nobody looked. Six of the thirteen
entries in this register were found by someone reading code for an unrelated
reason; §10.13 was found by running a command nobody had been asked to run.

The register is a list of what checking found. It is not evidence about what
checking would find next.

**At 0.26.0 the pattern shifts, and not in a comfortable direction.** §10.14
and §10.16 were both found by looking at what the guards do *not* cover, rather
than at what the code does — and both are cases where the project's own
apparatus reads as more complete than it is. A guard with no CI job, and a
comment claiming a test that does not exist, are the same defect as a
requirement annotated `Implemented` while the code does the opposite. This
baseline's first edition was written to correct twelve of those in the
requirements. Three of the last three entries are that same shape in the
verification apparatus itself.

§10.15 is different and worth separating: it is not a false claim of coverage
but a true statement of its absence, recorded before anyone could infer
otherwise.

It was corrected in **0.20.1**, and the correction differs from its
predecessors in a way worth recording: the constraint now lives on the message
table rather than on the render sites, so the violation is unconstructible
rather than merely absent, and the guarding test was **observed failing** before
it was trusted. Those two properties — constraint on the type, guard seen red —
are what distinguish a closed gap from one that will reopen.

### 10.2 Health presentation contradicts the Watch ceiling and the no-score rule

Three related contradictions, all in project-health presentation:

1. **A headline score is rendered.** The project detail screen displays
   "Score N / 100" as a badge. `SPEC §28.4` and `SPEC §28.6.2`
   explicitly prohibit this form, naming "Total Health Score: 72/100"
   and 0–100 gauges as examples of what not to build. Violates
   `FR-HLT-008`.
2. **A severity above `Watch` is reachable in presentation.** The health
   state model includes `Concern`, and explanation text is generated for
   it. `SPEC §28.2` limits displayed vocabulary to
   Insufficient / Good / Watch, and `GUI §9` states "No UI label exceeds
   `Watch` severity". Violates `NFR-LANG-002`.
3. **Danger colouring is used.** `Concern` maps to an error badge
   class. `SPEC §28.5` requires neutral colours only and forbids
   red/green win-lose contrast. Violates `NFR-A11Y-004` and
   `NFR-LANG-002`.

*Assessment*: these are the most significant gaps in this document.
They sit precisely on the product's defining commitment — item 1 invites
the user to stop at a number, and items 2 and 3 reintroduce the
alarm vocabulary the design excludes. `SPEC §28.6` anticipated exactly
this failure mode by separating computation from presentation: the
internal model *may* keep `Concern` for accuracy, but the presentation
layer must clamp it.

*Recommended resolution* (implementation change, not specification
change): retain the internal four-state model; clamp presentation to
three states by mapping `Concern` to `Watch` at the render boundary;
replace the score badge with the composite indicator rendered at equal
weight beside the others, carrying a note that the composite alone is
not the answer. This is a contained change in the health strip component
and the badge-class mapping.

### 10.3 Storage-layer authorisation not implemented

`SPEC §11.5.4` prescribes defence in depth: storage functions handling
personal data should accept the requesting user's identity and verify
it, so that a handler oversight does not become a disclosure. The
implementation verifies at the handler layer only (`require_self`).

*Assessment at 0.19.1*: the current boundary is correct and tested; this is a
resilience gap rather than an open hole. Scheduled with the Phase E
authorisation audit (`NFR-PRIV-005`).

*Reconciled 2026-08-26 (`QA-021`). **The assessment above was wrong in the
direction that matters, and the gap is narrower than nine releases of this
entry have implied.***

"Verifies at the handler layer only" is not what the code does. **Every storage
function handling this document's own personal-data inventory takes the
subject's identity and scopes on it** — `user_capacities` 10/10, `notifications`
14/14, `view_states` 3/3, `personal_metrics` 2/2, `user_burnout` 1/1,
`user_metrics_snapshots` 2/3. The exceptions are a job-side aggregate and the
three auth-path functions in `users.rs`, where no caller identity exists yet.

**And no handler passes a caller-supplied identity to personal-data storage.**
The three `/api/users/{user_id}/*` endpoints call `require_self` and then pass
the **session** identity; the path value is validated and discarded, reaching
neither storage nor the response body.

So the outcome `SPEC §11.5.4` prescribes was already achieved by a different
route, with **two independent barriers** rather than the one this entry
described.

*What was actually missing*: scoping is not verification —
`for_user(pool, user_id)` returns whatever user it is handed — so nothing
**guarded** that a future handler would keep passing the session id. Closed by
two assertions: every `Path(` extraction in `api_users.rs` binds the name
`user_id`, and that name reaches nothing but `require_self`.

*The direction, for whoever opens this entry next.* Both the narrow guard's
weakness and the impossibility of a broader one have **one cause**: nothing in
this codebase distinguishes an identity `Path` parameter from a resource one.
Every extraction is a `String`, and identity-ness lives in a variable's name. A
**newtype** closes both at once — unconfusable with a resource id, findable
structurally, and refusable by a storage signature at compile time. It touches
every handler signature and is not scheduled. **It is a better answer than
threading a requester parameter through thirty storage functions**, which is
what this entry's original framing implied and what nine releases of readers
would have built.

*A scheduling failure, recorded because the register is where it would
otherwise be invisible.* This entry said "scheduled with the Phase E
authorisation audit". RFC 005 had no section for it, `QA-007` excluded it
explicitly, and Phase E completed without it. It was picked up only because
`QA-020` closed the last section and the carry-overs were counted.

*Status*: **Partial — the barriers hold and are now guarded; the type-level
remedy is not built.**

### 10.4 Explainability affordances incomplete — **closed, partially**, at 0.29.0

Two `SPEC` requirements in the explainability family were unimplemented:

- `FR-HLT-007` — no "what this is based on" link or detail view exists;
  indicators offer a sentence but no route to the underlying issue list,
  calculation, or history.
- `NFR-A11Y-003` — existing charts do not provide tabular equivalents or
  textual summaries.

*Assessment at 0.19.1*: both are named in Definition of Done item 3, so this
condition cannot be considered met until they land.

*Closed at 0.29.0* (RFC 008, handoffs `HLT-001` and `HLT-002`), **and the
word to read carefully is "partially".**

`NFR-A11Y-003` is met in full. **`FR-HLT-007` is met on two limbs of three** —
basis and calculation ship; **history is deferred**, because an indicator's
history is a time series and for a one-contributor project it is that person's
history, which `NFR-PRIV-007` suppressed at 0.28.0 on the sprint screen. It
returns with `QA-017`'s predicate or not at all.

**And `FR-HLT-007` was narrowed rather than met for one indicator.** WIP
compliance offers no basis route; its basis is personal data. That amendment
was the owner's and is recorded in the requirement itself.

**So this entry is closed and the condition it blocks is not fully satisfied.**
That is an uncomfortable pair to write down and it is the accurate one:
Definition of Done item 3 moves from *"Partially met"* to *"Met, with
`FR-HLT-007`'s history limb outstanding"*, not to *"Met"*.

*Two things this closure cost, recorded because neither is visible from the
outcome.* The first design — link the issue list with query filters — was
**wrong on three of six indicators** and would have produced links pointing at
sets the indicators do not count. It was caught by the implementer reproducing
the architect's own table before building, which the handoff asked for and
which is the practice that prevented the defect. The second design returns each
indicator's membership from the computation that produced its count, so a count
and its basis **cannot** disagree.

### 10.5 Calendar schema scope reduced — **shipped as designed**, 0.23.0

`SPEC §38.1` lists four date columns (`start_date`, `due_date`,
`planned_start_at`, `planned_end_at`). RFC 0002 reduces this to two
planned-time columns, reasoning that `start_date` duplicates the date
part of `planned_start_at` and that carrying both `due_date` and
`planned_end_at` invites ambiguity about which is authoritative.

*Assessment*: a deliberate, recorded narrowing. It is listed here so the
divergence from `SPEC` is not mistaken for an oversight. Revisit if a
firm deadline must be distinguished from a soft estimate.

*Shipped at 0.23.0* (RFC 002, handoff CAL-001) with the two columns as
designed. The entry stays open rather than closing: the divergence from
`SPEC §38.1` is now a shipped fact rather than a plan, and the revisit
condition — a firm deadline distinguished from a soft estimate — is a live
product question a user can now actually run into, having plan dates but no
due date. Closing this would suggest the question was answered; it was
deferred, and the schema is now the thing deferring it.

### 10.6 Kanban status endpoint bypassed the optimistic lock — **closed**

`POST /projects/{id}/issues/{issue_id}/status` accepted a mutation with no
lock value and applied it, behind a comment describing a "Phase A rollout
window" that had closed three releases earlier. The shipped `board.js`
never sent one, so **every board drag in production bypassed the contract**.

`NFR-CONC-001` and `NFR-CONC-005` were both recorded `Implemented`. The
acceptance test covered the form path only.

*Closed at 0.20.0* (DEV-001): the bypass is removed, both entry points
share one lock check, and the client sends and handles the value. Four
tests, two of which fail against the prior code with a silent `204` where
a rejection belongs.

### 10.7 Capacity disclosed to non-subjects — **closed**

The project detail screen and both issue forms rendered another person's
capacity value, over-capacity annotation, and a capacity-derived danger
badge to anyone with project access. `NFR-PRIV-001` makes capacity
self-only; `NFR-PRIV-002` permits "workload distribution". The ambiguity
was resolved in the code's favour without a decision.

*Closed at 0.20.0* (DEV-003, `DEC-019`): only in-flight load remains.
`NFR-PRIV-002`'s scope is now explicit.

### 10.8 The domain crate generates user-visible prose — **closed**, 0.21.0

`peisear-core::Indicator::human_explanation()` returned English sentences,
so the computation crate performed presentation. This sat badly with
`FR-HLT-009` and `NFR-MNT-001`, and it meant a vocabulary guard over the
web layer alone would have a hole exactly where `FR-HLT-006` applies.

*Closed at 0.21.0* (RFC 006 §D3, handoffs I18N-002 and I18N-006): the domain
emits a message key plus typed parameters; presentation renders it.
`peisear-storage` followed in I18N-006 — its user-facing `Validation` and
`Conflict` variants carry a `MessageKey` rather than a `String`, rendered at
the `peisear-web` boundary.

The gap was wider than this entry said. It named one function; the survey
found `IndicatorKind::description()`, `DisplayHealthState::glyph()`'s state
word across four call sites, fifteen `peisear-storage` strings, and the
burnout endpoint's JSON label prose. Each was found by someone converting the
surface next to it, which is the argument for collecting copy in one place
restated as a fact about this register: **an entry naming one instance of a
pattern is an estimate, and this one was low by an order of magnitude.**

### 10.9 Formatting and lint gates had never passed — **closed**

`NFR-MNT-007` was recorded `Implemented in CI`. CI had failed on every push
since introduction — five runs, zero successes — with `fmt` and `clippy`
red throughout while every test and build job passed. 44 files had never
been run through `cargo fmt`.

*Closed at 0.20.0* across DEV-006, DEV-007, DEV-008 and DEV-005 item A: 32
findings in total, and the first workspace-wide `clippy -D warnings` exit 0
in the project's history.

Recorded despite closing immediately, because a P1 gate was red and
unobserved for three months across four releases, and the register is where
that stays visible.

### 10.10 `/today` rendered a severity above `Watch` — **closed**

§10.2 named the project health strip. It was not the only surface:
`classify_wip` and `classify_long_stale` reach `Concern` under ordinary
conditions, so **`/today` could render `✗` and danger colouring for a
user's own sustainability state** — the page every user lands on after
login, and the surface the product's thesis rests on. Recorded nowhere.

*Closed at 0.20.0* (DEV-004), structurally: `HealthIndicator` no longer
carries a badge method, so an unclamped render cannot compile.

### 10.11 Workload and assignee queries return only the project owner — **closed**, 0.22.0

`project_workload` and `list_assignee_candidates` both join
`projects p ON p.owner_id = u.id`, so neither can return a non-owner. The
second is used as a **write-path validator**, so an issue in a team project
**cannot be assigned to a team member**.

Consequences, none previously recorded:

- `FR-ISS-002` — the assignee attribute is inoperative for team projects.
- `FR-HLT-001` — the concentration indicator (bus factor) computes over a
  population that can only contain one person.
- `NFR-PRIV-002` — "workload distribution" describes something that has
  never existed.
- `FR-PER-002`/`FR-PER-004` — personal sustainability signals cannot
  receive team work for anyone but the project owner. **The product's
  differentiating feature does not function for non-owners in a team
  context.**

Plausibly stale since teams landed: `ROADMAP.md` records
`list_assignee_candidates` shipping in 0.4.0 as *"today: the project owner;
when team support lands, all team members"*. Teams landed; the query was
never revisited.

*Not corrected in 0.20.0.* Pre-existing, not a regression, and fixing it is
a behaviour change with open design questions — do members with no work
appear with a zero row? Do personal projects differ? Does the validator
widen to team membership, or to a distinct "may be assigned" concept?

**Requires an RFC, and bears on RFC 001** (sprint planning, 0.22.0), which
filters the backlog by assignee and shows capacity hints across team
members. Both are close to meaningless while only one person can hold an
assignment.


*Closed at 0.22.0* (RFC 009, handoff TEAM-001). Both queries now derive from
one shared candidate expression — the project's owner plus any `admin` or
`member` of its team — so they cannot drift apart again. A user removed from a
team keeps issues already assigned to them, so the candidate set is a subset of
the workload set rather than equal to it.

**What the delay cost.** The defect was reachable by every non-owner in every
team project, and it made the product's central feature — per-person
sustainability signals — permanently empty for them, because nothing could ever
be assigned to them. It failed by showing nothing rather than by erroring,
which is why it survived two releases, an external design document, this
baseline, and a compliance pass. `list_assignee_candidates`'s doc comment
promised team support was coming; it had been saying that since before teams
shipped in 0.11.0, so the defect read as a scheduled limitation.

**Two defects in the governing RFC were found by the implementer**, both by
following the handoff rather than the RFC and reporting the difference: §D1's
sample SQL had no role filter and would have made every `viewer` an assignee,
and the privacy section forbade widening `project_workload`'s consumers while
requirement 2 forced it. Both corrected in the RFC.

### 10.12 Health summary prose named the unclamped state — **closed**, 0.20.1

`project_health::summarize` renders `"{label} is a concern."` and
`"{first} is a concern; {second} also needs attention."` into the summary
paragraph beneath the health heading — always visible, not inside the
collapsed disclosure.

**Third instance of the §10.2 violation, on the same screen.** §10.2 named
the score badge and the badge-level `Concern` mapping; §10.10 named
`/today`'s badges. Both were closed by `DisplayHealthState`, which governs
**badge and glyph rendering**. `summarize` produces prose and never passed
through it.

Two reasons it survived the release that was correcting it:

1. **The clamp was attached to a type used by badges**, not to every path
   that renders a state. DEV-004's scope named the badge layer; the
   function writing the sentence directly beneath it was not in scope, and
   nothing forced the question.
2. **The guarding test matched a capitalised substring.**
   `health_presentation_clamps_concern_to_watch_vocabulary` checks
   `body.contains("Concern")`. The prose says "concern" lowercase,
   mid-sentence. The test passed against a page that violated the
   requirement, and was accepted in review on that basis.

Two further defects were found in the same survey and are fixed alongside,
being template/value mismatches rather than ceiling breaches:

- BusFactor's `active_assignees <= 1` case rendered *"solo of in-flight
  work is concentrated on one person."* — a percentage template fed a
  non-percentage value. Reachable from the **default state of any freshly
  created project**.
- WipCompliance rendered *"N over of active assignees are over their WIP
  limit."* — the same mismatch, milder.

*Closed in 0.20.1* (`DEC-046`, handoff I18N-004) — and the fix went further
than specified. I asked for `summarize` to take `DisplayHealthState`, which
would have made the violation unrepresentable in **one function's input**.
The implementer instead **deleted the two `Concern`-shaped message keys from
`peisear-i18n` entirely**, making it unrepresentable in the **message
table's output** — so no caller, in any crate, present or future, can
construct such a sentence.

The ceiling test is now case-insensitive, isolates the summary paragraph,
sweeps the whole page, asserts the fixture genuinely reaches `Concern` so it
cannot pass hollow, and **was observed failing** against the defect before
the fix landed.

Both sentence defects fixed; the WipCompliance case was reproduced live
first, having been reported as source-derived rather than verified.

### 10.13 The integration-test harness collides with itself under parallel runs — **closed**, 0.22.0

`TestApp::spawn` (`crates/peisear-web/tests/common/server.rs:35`) names its
temporary database directory from `SystemTime::now().as_nanos()` alone. Two
tests entering `spawn` within the same clock tick — across threads in one
target, or across targets running concurrently — receive the **same directory
and the same `test.db`**. `create_dir_all` succeeds on an existing directory,
so nothing signals the collision; the second arrival fails on `connect` or
`migrate` with `SqliteError { code: 5, message: "database is locked" }`.

Reproduced at 0.21.0 and, on the same command, at **0.20.1** — so it predates
this release and no shipped code is involved. Roughly one failure per two
`cargo test --workspace` invocations, on a varying test each time
(`issue_status_change_with_empty_client_updated_at_is_rejected`,
`cannot_assign_sprint_directly_to_sub_issue`, and others).

**Why it was never seen.** `DEC-007` requires per-crate runs and every
`peisear-web` integration target individually, precisely for isolation. That
procedure never triggers the collision, so every gate log this project has
captured is honest and green. The defect is only visible under the one command
nobody was asked to run.

That is the finding worth keeping: **an isolation procedure adopted to make
results trustworthy also hid a defect in the thing producing them.** A gate set
should include at least one run under the conditions a contributor will
actually use.

**The failure rate is a property of the machine, not of the defect.** Measured
at 3/6 by the implementer and observed repeatedly by the reviewer on the same
day; on the same machine hours later, with the pre-fix harness restored, it
reproduced **0 times in 24 runs** across four command shapes. The difference is
load — the failing runs were interleaved with compilation.

That matters for how the gap is closed. A repeated-run gate is a
**probabilistic** detector whose sensitivity depends on conditions nobody
controls, so it cannot be the guard for this class; a structural test that the
harness does not derive a path from the clock is deterministic and is. It also
means **CI could never have caught this**: every integration target runs in its
own job with `--test-threads=1`, so CI has no concurrency to expose.

**Not release-blocking for 0.21.0** — test-only, pre-existing, no user-visible
surface.

*Closed at 0.22.0* (RFC 005 §9, handoff QA-001), in both crates —
`peisear-notify`'s `fresh_pool` carried the identical defect and was found
while fixing the first. `tempfile::TempDir` replaces both clock-derived names
and removes the directories on drop, closing the `/tmp` accumulation that came
with the same code.

**The guard is structural, not a repeated run.** The handoff asked for a
repeated `cargo test --workspace` in the gate set; measurement showed that
detects this defect at a rate set by machine load, so a test now scans every
`crates/*/tests/**.rs` file for the co-occurrence of `SystemTime::now()` and
`create_dir_all` and fails deterministically. Proven against a planted
regression in four separate files across three review rounds, including two
that no watch list named.

`DEC-007`'s procedure was also written into `.github/CONTRIBUTING.md` — the
first time it has existed outside an internal, gitignored handoff. That is the
part of this entry most likely to still matter in a year. Fix at the opening of 0.22.0: derive the suffix from process id plus
an atomic counter as well as the clock, or use a crate that guarantees a unique
directory, and add a repeated-workspace-run check to the gate set so the class
cannot return silently.

**A second property of that repeated-run gate, found 2026-08-27 during
`JS-003` round 2 and recorded here because this is the entry that created the
gate.** Three consecutive `cargo test --workspace` runs failed at the same
point with `SIGSEGV` in `peisear-notify`'s `dispatch_integration` — a target
untouched by that round. It is **not a defect and not noise**, and both halves
matter:

- **It is free memory, not load and not concurrency.** With ~3 GB free the same
  test binary segfaulted 5 times out of 5 run standalone, including under
  `--test-threads=1`. Once free memory rose to ~14 GB it passed 6 of 6 — with
  the machine's load average *higher* than during the failures. Under memory
  pressure a thread-stack page allocation can fail and surface as `SIGSEGV` in
  whatever frame happened to touch the new page; here it faulted inside
  `PathBuf::as_ref` under `tokio::fs::create_dir_all`, an innocuous frame with
  no unsafe code near it.
- **`cargo test -p <crate>` builds a different binary from
  `cargo test --workspace`** — different hash, different feature resolution.
  So *"it passes under `-p`"* can never by itself clear a workspace failure.
  That reasoning was offered in good faith and compared two artefacts.

**The distinguishing evidence is whether the same binary passes once memory
frees** — not whether it passes in isolation, and not whether it passes on a
fourth attempt. Retrying to green without establishing which of those is true
would convert this gate into the thing §10.13 exists to prevent: a green result
that means less than it appears to. Stopping and reporting was the correct
response, and is recorded as such.

---

### 10.14 The board's script tag was guarded by nothing — **closed**, after 0.26.0

`crates/peisear-web/src/components/issues.rs:142` emits
`<script src="/static/board.js" defer>`. Deleting that line and changing
nothing else left **`cargo test --workspace` reporting 178 passing**. The board
would have shipped with no drag-and-drop and no undo, and every gate green.

Found by planting during `REL-0.26.0`'s review, not by reading. `search.js`
carried the same gap with wider reach — its tag is in the app shell, so it is
on every authenticated page.

**A comment asserted the opposite.** `status_control.rs:485` said the board's
tag was pinned by `boards_per_card_control_renders_unchanged`; that test
asserts the board's route and never looks for `board.js`. This is the second
time in this project a document has described a test's coverage and been wrong
about it — the first cost RFC 003 a rewrite. **A comment is the one artefact
here with no guard**, which is the whole reason this entry names the mechanism
rather than only the defect.

*Closed after 0.26.0* (RFC 005 §12, handoff `QA-003`): three HTTP-level
assertions, one per file, each asserting `defer` as well as `src`, each proven
against its own planted deletion. A scan over `static/` was rejected — it would
extend itself to a fourth file for free but would pass on a tag emitted inside
a branch that never renders, which is most of what can go wrong. The residual
— a fourth JavaScript file added later gets no assertion automatically — is
recorded in the guarding test's own doc comment rather than built for.

---

### 10.15 The shipped JavaScript is executed by no test — **open; the gap is permanent, its size is not**

0.26.0 is the first release where user-visible behaviour lives in code the test
suite never runs. `static/dm.js` and `static/board.js` carry the in-place
status update, the 5-second undo, the conflict path, and the fallback to the
plain form. The harness drives HTTP; it does not execute scripts.

What the suite does assert: the endpoints these scripts call, the response
shape they depend on (`200` with `updated_at`), the copy they render, the
`<script>` tags that load them (§10.14), and the no-JavaScript path that works
without them. What it cannot assert is that the scripts do what they say.

**This is stated in the changelog as a fact rather than a caveat**, and it is
recorded here rather than left implicit, because the alternative is a reader
inferring coverage from a green gate — the same inference §10.13 punished.

> **Amended 2026-09-24. The paragraph below is kept as written and it is no
> longer accurate.** It called the residue "DOM mechanics, where a failure is
> immediate and local rather than subtle and conditional", and it described
> three scripts. Both halves have been overtaken, and the second one was
> falsified rather than merely outgrown.
>
> **The body doubled in four releases.** At 0.33.0, when this was declared
> permanent: three files, 820 lines. At 0.37.0: **five files, 1,663 lines** —
> `plan.js` (`PLAN-002`, 0.35.0) and `calendar.js` (`CAL-003`, 0.36.0) together
> are larger than the three that preceded them.
>
> **And a failure in it was neither immediate nor local.** `PLAN-002` round 1
> shipped a row marker that never flipped, so a second drag of the *same* row
> was silently refused until a page reload. It passed `fmt`, `clippy`, 262
> tests, `BROWSER-001` at 90/90 and a four-screenshot evidence run, and arrived
> as a finished package. Behind it sat a stale form action that would have
> diverged the client from the server **with a toast reporting success** —
> subtle and conditional, the two words the old paragraph used to say this
> could not happen.
>
> **What does not change: the decision not to buy a tool.** RFC 011 weighed a
> JavaScript harness and declined it, and nothing here reverses that. What
> changes is the honest description of what is being carried, and the
> recording of what has actually caught something.
>
> **The compensating controls, and the evidence for each.** Two required
> evidence runs have each caught one defect that no gate could:
>
> | control | what it caught |
> |---|---|
> | **the no-reload sequence run** — act on the same element twice, then undo, then reload once and confirm the server agrees | `PLAN-002` round 1's marker, above. A run that reloads between actions verifies two first actions, not a round trip |
> | **the before-and-after rendering check** — computed style for the same data, before and after a markup change | `CAL-003`'s `h-full`, a class never used in this codebase, so the purged stylesheet had never emitted it and a block's height silently collapsed from 59.875 px to 20 px |
>
> Both are now written down in `docs/static-js-verification.md`, alongside
> the rest of the documentation, rather than living in whichever handoff
> last remembered to ask. *It lived beside the files it governs until
> `STATIC-001` (0.38.0): `static/` is served in full, so a document about
> what the suite does not cover would have been a public URL on every
> deployment.*
>
> *One figure corrected in passing: I said in review that the sequence run had
> caught a defect "twice". It has caught one. The second catch was the
> rendering check — a different instrument.*

*Scheduled 2026-08-27, RFC 011; reviewed at 0.32.0; the programme complete at
0.33.0.* **The entry stays open, permanently and by decision.** What remains in
the three scripts is DOM mechanics: the movable policy moved (`JS-003`), the
fallback boundary's shape is pinned (`JS-002`), step 3 was withdrawn because the
rule it would have covered was a client-side cover for a server guarantee now
asserted directly (`DEC-052`, `LOCK-001`), and step 4 bought a browser for
layout, not for this residue. A residue that cannot shrink further is recorded
as such rather than re-scheduled; the earlier wording below is kept as the
route.

**The approach is to shrink it rather than buy a tool to point at it.** Policy
becomes data in the JSON island the scripts already read, tested in Rust. What
remains uncovered is DOM mechanics, where a failure is immediate and local
rather than subtle and conditional.

That is this project's own pattern inverted back: RFC 006 moved copy to a
message table, `QA-019` moved `updated_at` to one authority, `HLT-001` returned
the set rather than re-deriving it. **Move the fact to where it can be checked
— do not add a checker where the fact is.** A browser harness would be the
first time this project chose the second.

#### The argument this entry used to make was wrong

*Corrected at 0.29.0 by `JS-001`, and left visible rather than quietly
rewritten.* This entry read:

> `dm.js` measures 36 decision points against 24 DOM operations — it is more
> *policy* than mechanics.

**The 36 was mine and it was wrong.** It counted `return` and `throw`
statements as decisions. `JS-001` traced every branch in all three scripts:

| | decisions | guard clauses | mechanics | **movable** | DOM ops |
|---|---|---|---|---|---|
| `dm.js` at 0.28.0 | 23 | 12 | 5 | **6** | 24 |
| `board.js` at 0.28.0 | 22 | 12 | 1 | **9** | 29 |
| `dm.js` at 0.29.0 | 23 | 12 | 9 | **2** | 25 |
| `board.js` at 0.29.0 | 25 | 12 | 12 | **1** | 29 |

So these files were **more than half guard clauses**, not "more policy than
mechanics." The premise the shrink plan was sold on did not survive its own
first audit — and the audit was commissioned precisely because the plan rested
on an estimate, with the handoff saying in as many words that finding the
movable fraction small would be the most useful outcome it could have.

**And the one rule this entry most wanted to move cannot move.** `dm.js`'s
fallback boundary — *falling back to a native submit is correct before the
server has applied the change and wrong after* — is not a value anywhere. It is
a fact about which `catch` is nested inside which. Extracting it to a flag would
make it legible and no more testable, because the branch would stay in
JavaScript and a Rust test would assert something no shipped code consults.
**Legibility is not testability**, and that distinction is the most useful thing
this gap has produced.

`JS-002` therefore pinned the *shape* instead: `dm_fallback_boundary_scan`
asserts that `applyChange` exists, carries a `try` at its **own** depth, and
never calls `fallback(`. It does not test that the boundary works — it makes the
flattening that would break it unconstructible, the same claim
`test_harness_scan` makes about clock-derived temp paths. **Named limit**: a
*narrowed* top-level `try` still passes; closing that needs a second
depth-counting pass, and it is recorded in the module's own doc comment rather
than only in the RFC.

#### What actually moved

**0.29.0, `JS-003`** — the `409`/other-failure/malformed-body classification was
written three times across the two scripts and once in Rust, where it is true.
It now lives in the copy island both scripts already read, built by one shared
function, with `conflictStatus` derived from a real `AppError::
OptimisticLockConflict` rather than written as a literal. **The literal `409`
appears in neither `.js` file.**

**Combined movable sites: 15 → 3.** A real reduction, not a recount — what
remains is correctly mechanics, because once the policy is data the script only
looks up, the lookup *is* mechanics.

Two of the three residual sites sit behind the fallback boundary that does not
survive extraction; the third is `board.js`'s stale-lock-value guard at drop
time, a separate concern. **None is a realistic candidate for a further move**,
which is a more useful thing to know than the count.

`JS-003` also closed a latent defect it did not set out to find: `board.js`
passed a malformed `2xx` body over in silence, leaving the card moved and its
lock value stale, so the *next* drag would surface an unexplained conflict.
`dm.js` announced the same case. One rule now, two actions — different because
only one surface has a form to fall back to.

**That fix required new copy, and the reason is worth keeping.** The obvious
move was to reuse the existing *"could not be completed"* message. It asserts
two things the code cannot support: the server returned `2xx`, so the mutation
may well have applied, and the reload that same outcome triggers can show the
card in its **new** column moments after announcing it was returned to the old
one. `BoardUnconfirmedMessage` and `StatusChangeUndoUnconfirmedMessage` say only
what is known — *"may not have completed"*. An assertive live region stating a
false outcome is a worse failure than the silence it replaced, and this product
is scrupulous about that everywhere else (`FR-HLT-005`/`006`, `NFR-LANG-002`).

**On the counts in the table above.** The DOM-operation figure is
**measurement-method sensitive**: `dm.js` reads 24 or 25 depending on the grep,
across a re-read in which no DOM call was added or removed. Do not treat a ±1
as a regression. It is recorded because the alternative is someone re-deriving
it later, getting the other number, and looking for a change that never
happened.

#### Remaining schedule

| Step | Release | What |
|---|---|---|
| 1, 1b | 0.29.0 | ✅ Inventory; pin the fallback boundary's shape |
| 2 | 0.30.0 | ✅ One authority for response classification (`JS-003`) |
| 3 | ~~0.31.0~~ **withdrawn** (`DEC-052`) | `board.js`'s stale-card rule was **mis-classified**: not policy, but a client-side cover for a server guarantee — every board card carries a non-empty lock value. `LOCK-001` (0.33.0) asserts that guarantee directly, one test, scoped per card. **`search.js` remains excluded** — its two "movable" rules fail the *purpose*: the server has no query-length floor, so moving `MIN_QUERY_LENGTH` would **invent** a second authority rather than remove one |
| 4 | 0.32.0 (decided), **0.33.0 (shipped)** | ✅ **Answered, and the premise inverted.** A browser buys little for the JavaScript residue and a great deal for **layout** — four defects in three days, none reachable otherwise. `BROWSER-001` gates horizontal overflow: **eighteen pages × five widths** after `BROWSER-002`, `LAYOUT-005` and `LAYOUT-008`, one assertion, **a CI job rather than a `cargo test`**, so no contributor's workspace run needs a browser. Watched red by four planted defects before it was trusted. **What it sees is exactly its fixture and its page list** — four times in 0.33.0 that was the gap (`§10.25`) |

Step 2 shipped in 0.30.0. *An earlier edit to this entry said it "landed
in 0.29.0 rather than 0.30.0 — the only item on this plan to arrive early."
That was wrong*: `JS-003` merged after the 0.29.0 release commit. The original
0.30.0 target was correct and the correction introduced the error, in both this
entry and RFC 011's own schedule table.

**One question step 4 should carry, added 2026-08-27.** Every reloading outcome
announces into an assertive live region and then immediately tears the document
down. Whether a screen reader finishes speaking first is not answerable by
source reading or by any test here, it is **pre-existing** rather than
introduced, and `NFR-A11Y-008` is the requirement those announcements exist to
satisfy. *Announced* is not *heard*. It is exactly the sort of thing a harness
could settle and this project's current tools cannot, so it belongs in the
evidence when the browser question is re-asked rather than in a backlog nobody
re-reads.

The mitigating property, unchanged throughout, is `DEC-021`: every JavaScript
path is an enhancement over a server-rendered path that is tested, so a total
failure of these files degrades to tested behaviour rather than to nothing.

---

### 10.16 The four structural guards have no CI job — **closed** 0.27.0, **reopened and re-closed** in 0.31.0

§9.1 states the convention adopted at 0.20.0: **a test crate without a CI job
does not exist.** `.github/workflows/test.yml` has a job for each of the
twenty `peisear-web` integration targets, and for `peisear-core`,
`peisear-auth`, `peisear-storage`, `peisear-i18n` and `peisear-notify`. It has
**no job running `cargo test -p peisear-web --lib`**, and none running
`cargo test -p peisear`.

That is where every structural guard this project has built actually lives:

| Guard | What it makes unconstructible | Runs in CI |
|---|---|---|
| `prose_scan` | user-visible English inside Rust (RFC 006) | **No** |
| `static_js_scan` | the same inside `static/*.js` (`BOARD-001`) | **No** |
| `test_harness_scan` | §10.13's clock-derived temp paths | **No** |
| `dec_007_scan` | the `DEC-007` block drifting from the workspace (`QA-004`) | **No** |

`DEC-007`'s command block in `.github/CONTRIBUTING.md` omits the same line, so
a contributor following the documented procedure does not run them either. They
execute only under `cargo test --workspace` — which **is** part of the release
gate, three times, so no release has shipped without them. The exposure is
per-pull-request and per-contributor, not per-release.

**`dec_007_scan` does not catch this, and the reason is already recorded.**
RFC 005 §13 notes the guard asserts each member appears as `-p <name>` but not
that the flags on that line are right for the crate. `peisear-web` appears
twenty times, via `--test` lines. This is that limit's first live instance
rather than a hypothetical, and it argues the limit is worth less tolerance
than it was given.

*Closed at 0.27.0* (RFC 005 §14, handoff `QA-005`). `peisear-web --lib` gained
a dedicated job named for the four guards it runs, so a red check names the
cause rather than the command; the facade joined the existing `test-libs` job,
whose comment already anticipated doctests. Both lines were added to the
`DEC-007` block.

**The fix that mattered was not the job.** `dec_007_scan` passed throughout,
because it asserted a crate's name appears in the block and `peisear-web`
appeared twenty times via `--test` lines — none of which run its own library
target. The guard now requires a line that is **not** a `--test <target>` line,
which generalises: any future member reached only through `--test` lines trips
the same assertion.

**And the block is now pinned to CI**, target by target (`QA-008`, `QA-009`).
Deleting the lib job, deleting any one of the twenty per-target jobs, or
disabling one with `if: false` each fail a test. `continue-on-error: true` does
not, deliberately: it means the job *runs* and its failure is not enforced,
which is a question about enforcement rather than coverage, and folding it in
would silently redefine what this guard claims.

Each step of that was found by planting the next-most-realistic way the
apparatus could lie, and each was found only because the previous one was
closed.

**Reopened 2026-08-27, found by `TT-002` and verified in review.** The guards
that closed this entry check **block → CI** — every target named in
`CONTRIBUTING.md`'s `DEC-007` block has a matching CI run line. **Nothing
checked filesystem → block.**

So a new `crates/*/tests/*.rs` file got no loop entry and no CI job, *silently*,
and ran only under `cargo test --workspace` — **which is the exact exposure this
entry was opened to describe.** Verified by deleting `touch_target`'s lines from
`CONTRIBUTING.md`: all three existing scans stayed green.

`TT-002` hit this and wired its own test file in by hand. The gap was that
nothing would have said otherwise.

**Re-closed in 0.31.0** by `dec_007_fs_scan`: every `crates/*/tests/*.rs`
appears in the `DEC-007` block. Combined with the existing block→CI scan, that
gives **filesystem → CI transitively**, which is the actual guarantee and is
stated in the module's own doc comment because it is not obvious from either
scan alone.

**One exemption, and it was verified rather than asserted.** A crate whose block
line is a bare `-p <crate>` needs no per-file entry, because that command
already runs every integration binary it has; only `peisear-web`'s per-target
shape needs each file named. The dev team confirmed this was load-bearing by
commenting out `peisear-i18n`'s bare line, watching all four of its files report
missing, and restoring it. **An exemption nobody has tried to break is a hole
with a comment next to it.**

*What this entry has now demonstrated twice:* a guard's own scope is a claim
about the product, and the scope is where the next gap opens — not in the rule.

---

### 10.17 Assertions keep passing while no longer testing what they name — **open**, recorded at 0.31.0

**Eleven integration-test assertions were passing on markup other than their
own subject.** Found by `TT-003`'s sweep, each confirmed by planting a real
defect and watching the existing assertion stay green.

**Every one of them was correct when written.** None was broken by a commit.
They decayed because the page grew a *second* source of the string they checked:

| The check | What started also producing it |
|---|---|
| `contains("5") && contains("pt")` — a user's capacity | the Tailwind CDN URL `cdn.tailwindcss.com/3.4.15`, on every authenticated page |
| the Cancel link's `href="/projects"` | the navbar's own brand link, on every authenticated page |
| the breadcrumb's `/today` entry | the navbar's account-dropdown menu |
| the status segment's "Open"/"In Progress"/"Done" labels | `JS-003`'s copy island, added to the same page |
| the list row's `name="status"` | the filter toolbar's own `<select name="status">` |
| the Rhythm chip's "Throughput" | the same page's glossary section |
| a burndown cell's value `8` | the chart SVG's own y-axis tick labels |
| the board card's `min-h-11` | `TT-002`'s 139 other controls carrying the identical pair |

**This is a distinct class from anything else in this register.** `§10.13` is
tests colliding with each other. `§10.15` is code no test executes. This is
**tests that execute, pass, and no longer test what their name says** — and the
suite reports the same number either way.

**No gate detects it and none plausibly could.** A test that passes is
indistinguishable, from the outside, from a test that passes for the right
reason. The only detector found is the one this project already uses for
everything else: **plant the defect the assertion exists to catch, and confirm
it fails.** Every one of the eleven was found that way; two further candidates
were planted, found correctly scoped, and reported as clean.

**Recorded, not scheduled.** There is no work item here — the eleven are fixed,
and a rule saying *"scope every assertion"* would be advice, not a guard. What
this entry is for is the next person writing `body.contains(...)`: the string
you are checking may not be unique to the thing you are checking, it may not
have been unique for some time, and **nothing will tell you.**

*The two mechanisms seen so far are worth naming*, because both are ordinary
and neither looks like a hazard while you are doing it: **adding copy to a
shared page** (the copy island, the glossary, the navbar), and **making many
controls identical** (`TT-002`'s own 139). A feature that makes the product more
consistent makes its assertions less specific, and that is not a trade anyone
weighs at the time.

---

### 10.18 Authenticated pages scroll horizontally when the signed-in email is long — **closed** by `LAYOUT-001`, 0.32.0

**Horizontal overflow on every authenticated page, at every viewport width** —
but **only for accounts whose email address is long and has no break
opportunity.** Measured on a live instance by stripping the fix:

| Address length | Overflow |
|---|---|
| 21 chars | **0** |
| 33 chars | **49 px** |
| 47 chars | **137 px** |

*The conditional half of that sentence was missing when this entry was first
written, and the omission was mine.* The original read *"~17 px on every
authenticated page, at every viewport width"* and *"it has shipped in every
release"*, generalised from **one** scratch account whose address happened to be
~30 characters. I saw a stable number across three viewport widths and read
"independent of viewport" as "independent of everything." **The dev team varied
the input and found the dependence** — the discipline this project applies to
guards, applied to a finding.

**Cause**: `li.menu-title` in the account dropdown (`components/layout.rs`)
wraps the user's email. It is a flex item, and **a flex item's default minimum
width is its content's minimum**; an email has no spaces, and `.`/`@` are not
break opportunities, so its minimum is the whole string. When that exceeds the
menu's `w-48`, the `<li>` escapes the `<ul>` instead of shrinking.

**`visibility: hidden` is why the escape reaches the scroll extent, and is not
why it escapes.** A closed dropdown keeps its box; that part of the original
diagnosis held. `dropdown-end` was suspected and is **not** at fault —
`inset-inline-end: 0` right-aligns the closed menu correctly at every width.

**Fixed** by `min-w-0 overflow-hidden` on the `<li>` and `truncate` on the span,
which also stops a long email spilling past the menu's border when it is *open*.

It is worth being exact about why nothing caught it:

- **No test in this project can observe it.** It is a layout fact, and the suite
  drives HTTP and reads markup.
- **`§10.15` is not it.** That gap is about JavaScript being unexecuted. This is
  CSS, executed correctly, doing what it is specified to do.
- **Source review cannot find it.** Nothing in the markup is wrong. The defect
  is in what `visibility: hidden` does and does not do, which is a fact about
  the layout engine, not about this file.

*Severity is low* — every affected page is usable and nothing is unreachable.
**Its value is as evidence**: it is the first defect this project has found that
no discipline already in use could have found, and it was found in the first
hour of looking at the rendered product.

**And its correction is the second piece of evidence.** A browser found the
defect; a browser *and a varied input* found what actually caused it and how
narrow it was. Looking once is better than not looking. Looking once and
generalising from a single account is how the overclaim in the first version of
this entry happened.

---

### 10.19 The adjacency guarantee is narrower than recorded — **closed**, `TT-005`, 0.34.0

> **Closed 2026-09-16, and half of the entry below is withdrawn.** The `join`
> row is real and reproduces on every tree tried. **The two `<summary>` rows
> are withdrawn**: they could not be reproduced on `HEAD`, and — the step that
> settles it — they could not be reproduced on **`221e074`, the 2026-09-06 tree
> they were measured from**, rebuilt and served with the CDN stylesheets it
> shipped with, styling confirmed applied before each measurement. Two
> independent probes, four widths, `<details>` closed and open, board and list.
> Zero non-`join` pairs on either tree.
>
> The instrument was validated on **both** trees by planting a `<summary>`
> overlap of computed size and finding it — 244 × 22.5 px on `HEAD`, 244 × 16 px
> on `221e074`, the second being 16 px because `TT-004` had not yet grown that
> summary. So the probe sees this exact pair shape on that tree when one exists.
>
> **This is `§10.21`'s shape a second time**, and the lesson is the same one:
> an overlap number produced by a geometry sweep is not evidence until the
> sweep has been shown to find a known positive on the same tree. `§10.21` was
> one artefact; two makes it a pattern, and it is why `§17.8` now records a
> decision not to gate overlap.
>
> **What produced the original numbers is still not identified.** `TT-005`'s
> package offers hidden content inside a closed `<details>`, which matches the
> heights (16 px and 5 px) and not the widths (34.7 and 20.2 against 95.1,
> 112.5, 133.9). A 16 px height is simply an ungrown `<summary>`, so that half
> of the match is weak evidence. Recorded as unexplained rather than explained,
> because the difference matters.
>
> **The remedy is in the requirement, not the product.** `NFR-A11Y-007`'s
> adjacency clause now carves out members of one segmented group with
> deliberately collapsed borders. A rule that fails on a correct tree gets
> weakened until it passes; naming the case is the alternative to that.
>
> *Original entry, left as written:*

`DEC-049` clause 4 says a grown control inside a container with a positive CSS
`gap` satisfies the adjacency clause with no further verification. **That is
true, and it is not the whole tree.**

Measured across eight pages, comparing every pair of interactive bounding boxes:
five pages have **zero** overlaps; three have real ones.

| Overlap | Where |
|---|---|
| `join-item` ↔ `join-item`, **1 px × 44 px** | project detail, board, issue detail |
| `<summary>` ↔ `a.block`, **34.7 × 16 px** | project detail, board |
| `<summary>` ↔ `button.btn-xs`, **20.2 × 5 px** | project detail, board |

**Why the reasoning missed it.** `TT-001` §3.2 established that a positive
flex/grid `gap` cannot be consumed by a growing child. Correct — and **`join`
does not use a gap. It collapses adjacent borders with a negative margin**, so
two 44 px targets share a 1 px column by design. The argument was promoted into
`DEC-049` as though it covered every adjacency in the tree; it covers
gap-separated clusters.

**Practical severity is very low.** A 1 px seam between visually contiguous
segmented buttons harms nobody, and this is what WCAG's own spacing exception
exists to tolerate. **The recorded guarantee was still wrong**, and a
requirement that overstates what has been verified is the specific failure this
register exists to prevent.

The `<summary>` pairs are larger and less obviously benign; they are worth a
look before anyone asserts they are harmless.

---

### 10.21 An empty-text anchor overlaps the issue-row link — **closed, not a defect**

*Recorded 2026-09-08 as a 314 × 36.5 px overlap, content-dependent and open.*
**Re-measured 2026-09-10: it is not a defect. It was my measurement.**

**The anchor is a health-indicator basis link inside a closed `<details>`.** Its
`innerText` reads empty because it is not rendered — which is why it looked like
an "empty-text anchor" — and `getBoundingClientRect` **still reports the geometry
it would occupy if opened.** `TT-004` had already found this exact artefact and
corrected for it with `checkVisibility({checkVisibilityCSS: true,
checkOpacity: true})`. **My sweep predated that correction and never gained it.**

Measured both ways, same page, same fixture:

| | 390 px | 1280 px |
|---|---|---|
| No visibility filter (as I ran it) | 4 overlaps | 8 overlaps |
| With `checkVisibility` | **1** | **1** |

The survivor at both widths is `join-item ↔ join-item`, 1 × 44 px — `§10.19`'s
known, benign border collapse. **So the tree has exactly one overlap, and it is
the one already understood.**

*This is the third false reading my own measurements have produced.* The other
two were escaping errors during `ASSET-001`, caught before they reached a record.
**This one reached the register and stood for two days.** The pattern is the same
each time: a matcher or filter I had not validated against a known positive, used
to check someone else's work — and here the correction was already in the project,
found by `TT-004`, and I did not apply it.

**It changes a downstream judgement.** RFC 011 step 4 ranked an overlap gate last
because *"its measurement artefacts are not understood."* They now are: one
artefact class — unrendered content reporting layout geometry — with a one-call
fix, and with it applied the tree is clean but for a 1 px seam. **An overlap gate
is more tractable than step 4 assumed.**
---

### 10.25 Unbreakable user text overflows thirteen surfaces, in three shapes — **closed in tree**, 0.33.0; found 2026-09-12

A title, project name, team name or display name containing a long run with no
space widened the page on nine surfaces. **The defects come in two shapes whose
remedies are not interchangeable**, and that — not the count — is the finding:

| Shape | What is too wide | Remedy | Sites |
|---|---|---|---|
| **A** | the *box* — a flex or grid item whose `min-width: auto` lets its content set its size | `min-w-0` **and** a break opportunity; `break-words` alone is defined not to affect min-content size | board columns (`§10.24`), issue detail `h1`, team detail's wrapper, the navbar's menu title (`§10.18`) |
| **B** | the *text* — an ordinary block whose box is already right | `break-words` only; `min-w-0` is a no-op | both delete interstitials, search rows, teams list, project calendar, the `/today` and `/settings` subtitles |
| **C** | the *container* — `inline-flex` (or any shrink-to-fit box) sizing itself to its text's min-content | `overflow-wrap: anywhere` on the text — the one property that adds break opportunities to min-content; `min-w-0` and `break-words` are both inert | the issue form's workload chips (`LAYOUT-008`) |

**Applying the wrong remedy is silent in both directions**, and one site in
committed code already carried the wrong one before this work. The rule is
recorded once, in `components.rs`, and deliberately not source-guarded: which
text is user-supplied is not a property a scan can read from a class string.
**The guard is the gate's fixture** — and that is where the three lessons of
this entry came from:

- `BROWSER-001` was green on a tree with three of these defects because its
  fixture title had a space at every point (`BROWSER-002`).
- It was green on project calendar and team detail because neither was in its
  page list (`LAYOUT-005`).
- It was green on the two subtitles because its fixture *display name* had a
  space at every point (`LAYOUT-007`).

**What the gate sees is exactly its fixture and its page list.** The fixture now
carries a 64-character unbroken run in the issue title, project name, team name
display name, sprint name and sprint goal; the list is eighteen pages.

**`overflow-wrap: anywhere` was tested as a single universal remedy and
rejected** (`LAYOUT-007-review.md` §4): with every existing remedy stripped it
clears nine of thirteen sites alone and fails on a `line-clamp` title and on a
`<select>`. One class would move the exception list, not remove it.

*State*: all thirteen sites closed in tree (`§10.24`, `LAYOUT-004`, `LAYOUT-005`,
`LAYOUT-007`, `LAYOUT-008`), each watched red then green. A sweep of every
route at four widths with an unbroken run in every user-text field found no
further site (2026-09-12, `LAYOUT-008-review.md`).

### 10.26 Every page overflows a 320 px phone when the display name is 21 characters — **closed in tree** by `LAYOUT-006`, 0.33.0

Found by `BROWSER-002`'s first run at 320 px: every page, 24 px, a constant.
**A third mechanism**, neither of `§10.25`'s shapes — nothing unbreakable, a
navigation row whose combined content minimum exceeded the viewport because its
right-hand block was `flex-none`, our own class. `min-w-0` was inert at every
level of the nesting until that was removed, which the dev team established by
injection rather than by reading.

The design decision, made in the handoff rather than in the code: **the account
control shows the name at every width and gives it up with an ellipsis only when
the row cannot otherwise fit** — no breakpoint, no character limit, nothing
hidden on phones. The touch target's own 44 px floor is what the name cannot
shrink past. Measured: truncated at 320 only; whole at 360 and above; an
80-character name whole at 1280.

**This is the defect 320 px exists to catch**: with the new fixture the board
defect that originally justified the width already fails at 390, so the width's
case is this entry, not `§10.24`. The gate holds it — the pre-fix navbar planted
back in fails fourteen cells, all at 320, none elsewhere.

### 10.27 Page header rows squeeze the title instead of wrapping — **closed in tree**, `LAYOUT-009`, 0.34.0; found 2026-09-12

Sprint detail's header is a flex row — title left, actions right — that never
wraps. With an **ordinary** sprint name at 320 px the title column is 54 px
wide and the `h1` runs eight lines; at 390, four. Issue detail has the same
shape and gives a 43-character title 143 px and four lines at 320, which
`LAYOUT-004`'s review accepted as existing layout without measuring it.

Not `§10.25`: nothing here is unbreakable. It is `§10.20`'s mechanism — a row
that cannot wrap — on a header, and it **passes the overflow gate**, because
nothing overflows and nothing is clipped. Worth recording for that reason
alone: 90 green cells say the page does not scroll sideways, and say nothing
about whether it reads.

The decision is in `LAYOUT-009`: when the row cannot fit both, the actions
drop below the title; no breakpoint. **Closed in tree at 0.34.0** across all
three headers of this shape — issue detail and sprint detail (`b458943`),
team detail (`fbe5a6b`). Measured with ordinary names: at 320 px each title
goes from 54–193 px and three to eleven lines to the full content width and
two lines, with every control still 44 px. Desktop is untouched on issue
detail and team detail; on sprint detail the actions drop at 768 px too,
because four controls and a title do not share a line there — accepted,
since the alternative is the breakpoint this decision rules out.

**A sweep of every `justify-between` row in `components/` found no fourth
site**, and the two exclusions that needed checking were checked rather than
read off the markup: the inbox row pairs a control with *system copy* (a
notification's title is rendered from the message table with no user-supplied
parameter, at both of its two constructors), and the sprint-plan rows bound
their title with `truncate` inside `min-w-0 flex-1` — one line, ellipsised,
the breadcrumb's strategy. The teams and sprints list rows pair user text
with a **badge**, not an action: the title keeps 71–74 % of the row against
sprint detail's 19 %, and the row itself is the link.

### 10.28 The touch-target guard's doc denies its own exception list — **closed**, `TT-006`, 0.34.0; found 2026-09-12

`touch_target_scan.rs`'s module doc says **"No exception list"** in bold, and
adds that needing one *"is a finding to report, not a line to add"*. Since
`TT-004` the module has carried `is_named_escalation_exclusion`, two
class-string arms covering the three call sites `DEC-050` excluded by design.

**The list is not the defect.** It was escalated as a finding, decided by the
owner, kept to two entries, and documented with a reason per arm and a note
that closing either is a decision rather than the guard's to make — all of
which is what the denied paragraph asked for. It sits 470 lines below that
paragraph, which never mentions it.

**The defect is that the guard's own description is stricter than the guard**,
in the file a reader opens to learn what the rule is. `§10.14` was a script tag
guarded by nothing; `§10.16` was a comment claiming a test that did not exist.
This is the same class pointed at the apparatus rather than the product, and it
was found by reading the guard for an unrelated reason — which is how six of
this register's entries were found.

`TT-006` corrected the paragraph and the count did not move. It splits the claim
the way the code does — the size clause needs no allowances, the coverage clause
has one declared exception and two named escalation exclusions — keeps the
`checkbox-xs` reasoning, points at the predicate, and **records the old
paragraph's own prediction coming true** rather than overwriting it: a future
change did need an allowance, it was reported as a finding, the decision was
taken, and a line was added. The failure was the denial, not the list.

### 10.29 An `ORDER BY` whose terms do not determine an order — **closed**, `ORD-001`/`PLAN-003`/`ORD-002`/`ORD-003`, 0.38.0; found 2026-09-23

Four ordering defects in two days, in two shapes with one cause: **a query
asked to order by something that does not carry the order it looks like it
carries.**

**Shape one — a TEXT column ordered as though alphabetical order meant
something.** Three instances:

| where | term | what users saw |
|---|---|---|
| the default issue list | `status ASC` | SQLite orders `done, in_progress, open` — **Done at the top, Open at the bottom** |
| the sprint-plan backlog | `priority DESC` | `urgent, medium, low, high` — **`high` last, below `low`**, on the screen for choosing what to work on next |
| a sprint's issue list | `status ASC` | Done first, on both surfaces that render it |

**Shape two — a timestamp with one-second resolution, and SQLite returns ties
in scan order, oldest first.** **22 orderings carried no tiebreak**, of which
three resolved *a user's current capacity* with `… DESC LIMIT 1` and so
returned the **superseded** value rather than merely a wrong order.

> **Correction, 2026-09-24, from `REL-0.38.0`'s review.** This paragraph read
> *"Every `DESC` ordering therefore read backwards within a tie. 22 sites."*
> **That is wider than the evidence.** 22 orderings lacked a tiebreak; how many
> *rendered* backwards was never measured, and at least one did not — the
> dev team drove the released 0.37.0 binary and found the team sprint list
> already newest-first, because a reverse walk of `idx_sprints_team_dates`
> returns ties in `rowid DESC` by accident of the plan. Reproduced: three
> sprints sharing a `created_at`, ordered `starts_on DESC, created_at DESC`
> with no tiebreak, come back newest first. The tiebreak is still right there —
> it makes an accident of the query plan into a stated property — but it
> changed no rendered order. **The defect this entry records is that the terms
> did not determine an order, which is true of all 22; what each one displayed
> was measured for four of them.** This is the eighth conclusion in that
> release cycle written wider than its search, and the only one written inside
> the entry recording that shape.

**None of this was found by the suite, and none of it could have been.** The
first was found by reading `position` during a design review of a different
feature; the second by enumerating the first's blast radius; the third and
fourth by the dev team while implementing the second. **No test referenced
`position` at all**, and none pinned any of the four orders — which is why
three of them had shipped since `PLAN-001` or earlier and one since `0001`.
`§10.27`'s lesson in a third place: a gate that checks a property cannot see a
property nobody stated.

**The register's own shape held again.** Three of the four were found by the
dev team, two of them by declining to follow an instruction of mine that was
wrong — `ORD-001`'s §6 asserted a before-state that was not true, and
`ORD-002`'s §1 enumerated two column names while calling the result *every
timestamp ordering*. The fourth, `issues_in_sprint`, was found and
**deliberately left alone** as a different defect rather than folded into a
timestamp handoff.

**What closes it.** Severity and status order are each written exactly once, in
`Priority::severity_rank` and `IssueStatus::lifecycle_rank`, in Rust, because a
SQL `CASE` would write the order a second time in a second language. Every
timestamp ordering carries an explicit `rowid` tiebreak in the matching
direction — including the two `ASC` sites that were correct only by accident,
which are now correct on purpose. `position`, which ordered two surfaces by a
number nobody chose, is gone.

**No fourth instance of shape one exists**: every `ORDER BY` in `crates/` was
enumerated case-insensitively and classified, and the Rust sorts with it. The
remaining non-numeric orderings are names, where alphabetical *is* the meaning,
and one role ordering written as an explicit SQL `CASE` — correct today, and
the shape to watch on the day anything in Rust needs role order.

**Not closed by a new check**, deliberately. Three instances in one file family,
all now fixed, with the enumeration recorded above, is a smaller thing than a
guard that would have to understand which columns carry meaning in their
sort order.

**What it did close is a gap in the record.** The external design specification
— the document describing what each screen presents — stated the *contents* of
every list and the *order* of none, so not one of these four was a divergence
from it and a reader checking the implementation against it would have found
every screen conformant. `§17.9` there opened and closed at 0.38.0; `§5.9` now
states each surface's order, once, as an obligation. **Nothing verifies those
lines** — they are readable, not executable, which is one more than existed and
fewer than a test.

### 10.30 A comment asserted a concurrency guarantee the code did not provide — **closed**, `CAP-001`, 0.38.0; found 2026-09-24

`user_capacities`'s module doc said the window between its overlap check and
its INSERT *"could"* admit a concurrent writer, and then closed it: **"for
peisear's single-process / WAL-serialized-write model this window is zero in
practice."** It was not zero. Twelve simultaneous saves stored **six**
overlapping rows — a state the same file, and migration `0009`, both describe
as impossible.

**The reasoning was wrong in a specific and instructive way.** WAL serializes
write *transactions*. The check was a `SELECT` on one pooled connection and the
INSERT a separate statement on another, with no transaction across them, so
there was nothing for WAL to serialize. The comment named the right hazard and
then dismissed it with a property that does not apply.

**The comment is the defect, not the race.** An acknowledged gap invites a
second look; a gap declared closed does not, and this one was declared closed
in `0009` and left for six releases. `§10.16` was a comment claiming a test
that did not exist and `§10.28` a guard's doc stricter than the guard — **this
is the third of that family and the first where the false claim was about
runtime behaviour rather than about the apparatus.**

**What closes it.** `insert` and `update` open `BEGIN IMMEDIATE`, take the
write lock *before* the check, and commit both together. The comment now opens
with **Correction**, quotes the retracted claim, states what WAL does and does
not serialize, and cites the measurement — the old claim left visible, which is
the more useful half.

**The survey it prompted found four more instances of the shape**, one of them
a guard whose failure leaves a team with no administrator, and one the
optimistic lock itself. Those are `RACE-001` and `RACE-002`, 0.39.0. **This
entry stays closed**; what the survey found is not this defect recurring but
the same mistake made independently in four places, which is the argument for
fixing them as a family rather than one at a time.

### 10.31 A deferred transaction failed writes that shared nothing — **closed**, `CAP-001`/`RACE-001`/`RACE-002`, 0.39.0; found 2026-09-24

A read-then-write transaction opened with a plain `BEGIN` is **deferred**: the
first `SELECT` takes a read snapshot and the write tries to upgrade to the
write lock. If another writer committed in between, SQLite fails the upgrade at
once with `SQLITE_BUSY_SNAPSHOT`, which **the busy timeout does not retry.**

**The cost was not the lost update it was found looking for.** Measured on the
issue write path: 64 concurrent status changes on **64 different issues**,
sharing no row —

| shape | result |
|---|---|
| deferred `BEGIN` | **6–16 of 64 succeeded; 48–58 failed** |
| `BEGIN IMMEDIATE` | **64 of 64 succeeded**, in 79–104 ms |

So the product answered a server error to three writes in four in a burst, on
rows with nothing in common, and the user was owed neither an error nor a
conflict. `RACE-002` §2 measured the user-visible half: eight racing issue
edits returned seven `500`s where a conflict was owed, while a project edit
returned eight `303`s and stored one.

**This reframes every performance number in the three handoffs that closed it.**
`CAP-001`, `RACE-001` and `RACE-002` each reported `BEGIN IMMEDIATE` as several
times slower than what it replaced under a saturating burst. On the paths that
were deferred read-then-write, **the faster baseline was faster because it gave
up**; the honest comparison is 104 ms against a failure. On the three
`RACE-001` sites, which were not in a transaction at all, the slowdown is
like-for-like and was reported as such.

**Closed, and the property is now stated.** Every read-then-write in
`peisear-storage` takes `BEGIN IMMEDIATE`; the three remaining plain `begin()`
calls (`teams::insert`, `issues::insert`, `issues::insert_sub_issue`) each
**begin with a write**, so they take the lock on their first statement and
cannot fail this way. **Nothing tests that**, and the obligation is therefore a
reader's: a transaction whose first statement is a `SELECT` and whose later
statement writes must be `IMMEDIATE`. `§17.9`'s lesson in a second place — the
rule is written down, which is one more than before and fewer than a check.

## 11. Deferred and future requirements

Accepted in principle, deliberately not scheduled.

| Area | Requirement | Source | Earliest phase |
|---|---|---|---|
| Sub-issue completion suggestion | Offer a non-blocking suggestion when all children are done | `SPEC §8.6` | Post-D |
| Focus-time estimation | Estimate uninterrupted working time | `BRIEF §1.2` | Phase 2 |
| Proposal generation | Suggest reassignment or refactoring windows, always with cited evidence, never auto-executed | `BRIEF §2.3` | Phase 2 |
| AI-assisted warnings | Use indicator data as input to generated guidance | `BRIEF §7` | Phase 2 |
| External integration | Commit logs, CI results as supplementary indicator input | `BRIEF §2.1` | Phase 2 |
| Quality indicators | Defect density, technical-debt ratio | `BRIEF §1.1` | Requires external integration |
| Webhook channel | Third notification channel | `SPEC §29.1` | Phase 2 |
| Comments | Issue comments as a first-class feature | `KICK §3.3` (optional) | Unscheduled |
| Cross-organisation analysis | Trends spanning organisations | `BRIEF §7` | Phase 3 |
| RDBMS backend | PostgreSQL alongside SQLite | `KICK §8` | Phase 3 |
| Identity provider integration | SSO / IdP / IdaaS | `KICK §8` | Phase 3 |
| CI/CD and cloud integration | Including infrastructure-as-code | `KICK §8` | Phase 3 |
| Full internationalisation | Dedicated per-language interfaces | `SPEC §34.1` | v3.0 |

---

## Appendix A — Requirement status summary

| Area | Total | Implemented | Partial | Specified | Deferred | Divergent |
|---|---|---|---|---|---|---|
| Authentication | 5 | 5 | — | — | — | — |
| Navigation | 5 | 5 | — | — | — | — |
| Projects | 4 | 4 | — | — | — | — |
| Issues | 7 | 7 | — | — | — | — |
| Sub-issues | 10 | 9 | 1 | — | — | — |
| Teams | 5 | 5 | — | — | — | — |
| Sprints | 4 | 3 | — | 1 | — | — |
| Sprint planning | 5 | — | — | 5 | — | — |
| Health | 9 | 5 | 2 | — | — | 1 (+1 unimplemented) |
| Personal | 8 | 8 | — | — | — | — |
| Notifications | 9 | 6 | 1 | 2 | — | — |
| Search | 4 | 4 | — | — | — | — |
| Calendar | 8 | 1 | — | 7 | — | — |
| Settings | 3 | 3 | — | — | — | — |
| API | 6 | 5 | — | 1 | — | — |
| Direct manipulation | 7 | 5 | 2 | — | — | — |
| **Functional total** | **99** | **75** | **6** | **16** | **0** | **2** |
| Privacy | 8 | 5 | 1 | — | — | 2 unimplemented |
| Concurrency | 7 | 7 | — | — | — | — |
| Accessibility | 9 | — | 3 | — | 1 | 5 unverified |
| Language | 5 | 1 | 2 | — | 1 | 1 |
| Security | 7 | 7 | — | — | — | — |
| Performance | 4 | 4 | — | — | — | — |
| Maintainability | 10 | 9 | 1 | — | — | — |
| Release | 7 | 5 | 1 | — | — | 1 undefined |
| Compatibility | 6 | 5 | 1 | — | — | — |
| **Non-functional total** | **63** | **43** | **9** | **0** | **2** | **9** |

Counts are indicative; the authoritative status is the per-requirement
annotation in §4 and §5.

**`FR-DM` at 0.26.0 needs its row read carefully.** Five Implemented and two
Partial does **not** mean five of seven behaviours work everywhere. It means
each of those requirements is satisfied **on the two surfaces that ship**:
the kanban board, and status change from the issue list and issue detail.
`FR-DM-001`'s five surfaces are two shipped and three unwritten; `FR-DM-002`
is in force and unmet the moment a fourth surface adds a pointer affordance
before its keyboard path.

Reading `FR-DM`'s status as global is not a hypothetical misreading — it is
the specific one recorded at 0.19.1, where `Deferred` on a shipped surface
let `FR-DM-002` be violated for four releases without anyone checking.

## Appendix B — Traceability to source documents

| Source | Contributes |
|---|---|
| `KICK` | `FR-AUTH-*`, `FR-PROJ-*`, `FR-ISS-001..003`, `NFR-SEC-*`, `NFR-CMP-*`, `NFR-MNT-*`, `NFR-REL-*` |
| `BRIEF` | `FR-HLT-*`, `FR-PER-*`, §1.6 principles, `NFR-PRIV` intent, §6.2 event capture |
| `SPEC` | `FR-NAV-*`, `FR-SUB-*`, `FR-NTF-*`, `FR-CAL-*`, `FR-DM-*`, `NFR-PRIV-*`, `NFR-CONC-*`, `NFR-A11Y-*`, `NFR-LANG-*`, §8 |
| `GUI` | §7 interface requirements, form and naming contracts, `NFR-LANG-002` |
| `V3` | Phase structure, implemented-state annotations |
| `DIGEST` | Decision records, `FR-SUB-007`, open questions |

## Appendix C — Requirements change history

| Version | Change |
|---|---|
| V1 (`KICK`) | MVP scope: authentication, projects, issues, minimal UI |
| V2.0 (`SPEC`) | Five-entry navigation, sub-issue hierarchy, calendar, sprint planning, direct manipulation, ABDD as acceptance criteria |
| V2.1 (`SPEC`, `BRIEF`) | API-level authorisation invariants, optimistic locking, computation/presentation separation, sub-issue completion suggestion as future work |
| V3 (`V3`) | Reconciliation with implemented state through 0.19.0 |
| 0.19.1 baseline | Consolidation into English normative requirements; identifiers assigned; §10 compliance gaps recorded |
| 0.20.0 | Compliance pass. Nine recorded statuses corrected — five of them P0 or P1 requirements annotated as satisfied while the code did the opposite. `NFR-PRIV-002` scoped (`DEC-019`); `NFR-CMP-001` extended with the toolchain/MSRV distinction (`DEC-044`/`DEC-045`); `NFR-LANG-005` rescheduled to 0.21.0 (`DEC-022`). §10 gains a state table and six new entries, five of which close in this release. Test inventory 65 → 82 active, 0 disabled. `RSK-001` closed |
| **0.38.0 (this baseline)** | **`FR-DM-001` is amended from five direct-manipulation surfaces to four and reaches Met.** Issue-list reordering (D-5) is **retired by owner decision** rather than built — the product already answers *what is next* with priority bands, sprint membership and planned dates, and the sprint is the better answer; a manual order would have been a second answer with no name in the UI and no visible provenance, and RFC 0004's requirement 10 makes the affordance per-row buttons on a phone. **RFC 0004 closes at four of five.** `ORD-001` removes the `position` column it would have used (migration `0018`, and `Issue.position` leaves `peisear-core`'s public API). **Four ordering defects close as one shape** (`§10.29`): a TEXT column ordered alphabetically as though that meant severity or lifecycle, three times, and 22 timestamp orderings carrying no tiebreak, three of which resolved a user's current capacity and returned the **superseded** value. **One concurrency defect closes** (`§10.30`): the capacity overlap check and its write were not atomic, and the comment saying that could not happen was wrong about what WAL serializes. `FR-DM-001` also carries a **second correction** — its own per-surface breakdown, added so a stale reading could not recur, was itself stale for four releases, and the entry now states the obligation the first correction omitted. **The suite found none of the five defects**; 272 → 297 tests. `DEC-020` closed at 0.38.0 and this document moved into the repository |
| 0.37.0 | **Definition of Done item 1 — privacy — reaches Met**, having read *Largely met* since 0.19.1; the second-oldest condition in that table to move, after item 5 at 0.33.0. **No product behaviour changed, not a line.** `NFR-PRIV-008` closes with five authorisation assertions — **not the seven the audit first counted**: two were already covered since May and the implementer checked rather than inherited, declining a duplicate that would have matched the claim while changing no coverage. **The classification is the transferable part**: a cross-user test is writable only where a cross-user request is *constructible* — an identity in the path, a resource id in the path, or nothing at all — and the middle class was missed because `auth_boundary.rs`'s own doc described it as the third. **That claim had also been published**, in 0.20.0's entry, saying the boundary *"cannot exist"*; the withdrawal it justified was right and the reason was half wrong, and 0.37.0's entry says both. **Planting measured something `§10.3` had only asserted from reading**: the handler's ownership check and the storage layer's own scoping refuse independently, two barriers, either one sufficient |
| 0.36.0 | **The calendar's day view gains a drag**, and with it a third surface acquires the optimistic-lock conflict path (`FR-CAL-003`'s second writing path). Two of the D-3 sketch's three actions were cut at acceptance for reasons the project's own rules gave — the empty-cell action has no plain-form path, and a resize handle cannot carry `NFR-A11Y-007`'s floor inside a fifteen-minute block without breaking the proportionality `DEC-050` protects. **Unlike the sprint plan, this page has no on-page control at all**, so a failure announces and reloads rather than falling back, and the plain path is the issue's own edit form. **Two corrections came out of the work**: a handoff of mine claimed no ordering rule existed on the planned dates when migration `0016`'s triggers enforce one — a conclusion one scope wider than the search that produced it, the fourth of that shape recorded here — and the rendering check the handoff required caught `h-full` missing from the vendored stylesheet, a class never used in this codebase before, which had silently collapsed the block's height from 59.875 px to 20 px while every other computed property matched. **The overflow gate would never have seen it**, which is `§10.27`'s lesson in a second place. **Test inventory 262 → 268** |
| 0.35.0 | **The feature backlog resumes.** The sprint plan gains a drag between its two columns (`FR-PLAN-002`'s second half, deferred since 0.22.0 and named in the requirement itself): the move applies on the client and the request follows, with a five-second undo. **The buttons stay and are still the only path on a phone** — HTML drag-and-drop does not fire for touch, which RFC 0004 now carries as cross-cutting requirement 10 so no later substep claims otherwise. The reconciliation that preceded it found a real defect of its own: **the remove form omitted three filter fields its own handler accepted**, so a button-driven removal silently dropped the backlog filter while an addition kept it. **Test inventory 256 → 262, and none of the six performs a drag** — the gesture is executed by no test, which is `§10.15` and not a gap this release opened. Two rounds: round 1 passed every gate and still shipped a defect a *sequence* would catch and a *state* would not, which is now a standing verification requirement in the handoff rather than a lesson |
| 0.34.0 | **The release where the record got the same treatment as the code.** One user-visible fix — page headers that pair a title with actions now wrap rather than squeezing the title, worst case a sprint title in a **54 px column eleven lines deep** at 320 px with an ordinary name (`§10.27`, `LAYOUT-009`). Nothing overflowed before or after, which is why 0.33.0's new gate was green on it throughout — **a page can pass an overflow gate and still be unreadable**, and that is the sentence this entry exists to leave behind. **And two corrections.** `§10.19`'s two disclosure-toggle overlaps were **withdrawn**: unreproducible on `HEAD` and unreproducible on `221e074`, the 2026-09-06 tree they were measured from, by two probes each validated on that tree by planting an overlap of known size. What produced the original numbers is recorded as **unidentified**, because it is. `§10.21` was one such artefact; three makes overlap measurement a technique this project distrusts on evidence, and `§17.8` now declines it as a gate assertion rather than deferring it. The second correction was **published**: 0.31.0's changelog called the adjacency guarantee *"structurally guaranteed for every control now in the tree"*, and there is one overlap — a `join` group's deliberate 1 px seam. `NFR-A11Y-007` carries the carve-out; the word that was wrong was *every*. **Test inventory 256, unchanged, and no product behaviour was added either** |
| 0.33.0 | **The release where the rendered layout is observed by a check for the first time**, and the release that showed how narrow such a check is. `BROWSER-001` (RFC 011 step 4, `DEC-048`): one assertion, `scrollWidth <= clientWidth`, eighteen pages at five widths, a CI job outside the suite by design, driven by a dependency-free CDP harness. It was watched red by four planted defects before it was trusted. **Four times it was green on a defect in front of it** — a fixture title with a space at every point (`BROWSER-002`), two pages absent from its list (`LAYOUT-005`), a fixture display name with a space at every point (`LAYOUT-007`), no sprint in the fixture at all (`LAYOUT-008`) — and each time the correction was to the fixture or the list, red first. **Thirteen surfaces overflowed on unbreakable user text in three shapes with non-interchangeable remedies** (`§10.25`), one of them on a 1280 px desktop, fixed and the rule recorded once; **every page overflowed a 320 px phone for a 21-character display name** (`§10.26`), a third mechanism, fixed by letting the name yield and nothing else. **RFC 011 is complete**: step 3 withdrawn as mis-classified (`DEC-052`) and replaced by a direct assertion of the server guarantee it was covering for (`LOCK-001`); `§10.15` becomes permanent by decision. **Definition of Done item 5 → Met**, the oldest row in that table, open since 0.19.1: `NFR-A11Y-006` verified by the architect directly after three dev-team rounds failed to reproduce between environments — four flows, three viewports, emulated touch, both status paths, every check broken first. **Test inventory 255 → 256**, one test for nine defects, and flat is again the point |
| 0.32.0 | **The release where we looked at the rendered product**, and the first whose largest findings came from outside the test suite entirely. A browser inspection — headless Chromium over CDP, no dependency, on a machine that already had it — produced four defects in three days, **none reachable by any discipline this project had**: every authenticated page scrolled sideways when the signed-in email was long (`§10.18`, a flex item's content-based minimum, not the dropdown's position); the project toolbar could not wrap (`§10.20`); the adjacency guarantee was narrower than `DEC-049` claimed, because `join` collapses borders with a negative margin rather than a gap (`§10.19`); and a 314 px empty-anchor overlap that only appears once a project has issues (`§10.21`, open). It also **cleared** the vertical-centring worry across 57 inputs and selects, which was mine. **`NFR-A11Y-007`'s named limit was measured and was not narrow** — an account menu on every page, `<summary>` toggles at 16 px that external design `§5.7` names by hand, and `FR-HLT-007`'s own basis links — so `DEC-050` replaced it with one rule and one **declared** exception, and the guard was corrected three times before it held: a name-based allowlist, then a binding-follow that checked presence rather than path, then a code shape that makes the property structural. **Each bypass was demonstrated by planting it and watching the suite stay green.** **Definition of Done item 5 moved** — the oldest condition in that table, unmoved since 0.19.1 — to *"Met, with mobile completion outstanding"*, having been put there at 0.31.0 on an unmeasured claim, held when the claim failed, and moved **once**. And `ASSET-001` brought Tailwind and DaisyUI in from two CDNs (`DEC-051`): a self-hosted instance with no egress had been rendering **unstyled** — 44 px controls at 17 px, Times New Roman — while `NFR-CMP-002` said *Implemented*. 451 KB of runtime JIT compiler became 14 KB of static CSS. **Test inventory 254 → 255, and flat is the point**: three of the four handoffs added no tests, because nothing here observes layout |
| 0.29.0 | **RFC 008 ships and `§10.4` closes — partially**, after nine releases open and three slips. Health indicators disclose their thresholds inline and link to the issues they counted (SCR-32, one new route). Both sprint charts gain a data table and a textual summary, **inheriting 0.28.0's suppression rather than carrying a second copy of it**. `NFR-A11Y-003` is met in full; `FR-HLT-007` on two limbs of three, **history deferred** and `wip_compliance` **excepted** — its basis is one person's assignment load, and the exception is structural, not a name-check at the route. **Definition of Done item 3 moves** — the first of the five conditions to move outright since 0.19.1 — to *"Met, with one limb outstanding"*, not to *"Met"*. Item 5 is corrected here: it had still listed `NFR-A11Y-003` as open. **Two designs were wrong first and are recorded that way** — the basis route as query filters (three of six indicators inexpressible) and RFC 011's movable-rule count (mine; `return`/`throw` counted as decisions) — **both caught by the implementer reproducing an architect's table before building on it**, now twice this project's largest single saving. Test inventory 225 → 240, 0 disabled; eleven scan modules. `§10.3` also closed this cycle. The 139 sub-44px controls remain, with that design pass at 0.30.0 |
| 0.28.0 | **RFC 005 completes** — fourteen sections, twenty-one handoffs. Secondary text darkened in 111 places after measurement found that many below AA (the theme's tokens were never the problem; the 130 opacity modifiers this project applied to them were). Conflict announcements moved to an assertive live region — `NFR-A11Y-008` had read "Deferred with Phase D" while D-1 and D-2 had shipped, so it was in force and unmet. Four controls resized where a mis-tap could not be undone. Sprint trajectories suppressed below two contributors — `NFR-PRIV-007`'s first implemented instance. **`updated_at` given one authority** (migration `0017`, the first since 0.23.0): `NFR-CONC-003` had read `Implemented` while the application wrote the column on the two entities the optimistic lock most protects. `§10.3` narrowed and guarded after nine releases of an entry that described a codebase which no longer existed. `§9.2` opens with a warning rather than a list. Test inventory 207 → 225 at the tag, 0 disabled. **The changelog says the product is not WCAG AA compliant**; 139 controls remain below the 44 px target, with that pass at 0.30.0 |
| 0.27.0 | Two user-visible changes and a release spent checking the checkers. All four destructive deletes now take an optimistic lock (`NFR-CONC-001` extended); the issue confirmation names its sub-issue cascade, and `FR-SUB-009` leaves §9.2's unverified list. **The finding**: `MessageKey::all()` — the list `find_violations` walks — was missing **five live `aria-label` variants**, so a **P0** guard had never read them. They were correct. `§10.16` closes: four structural guards had never run in CI, and the `DEC-007` block is now pinned to `test.yml` target by target. Five new guard modules; `peisear-core` gains its first tests ever. Test inventory 178 → 207 active, 0 disabled. **RFC 005's accessibility axes remain unstarted** |
| 0.26.0 | RFC 004a step 2 and RFC 004b — status changes apply in place with a 5-second undo on all three surfaces, and the board's three error sentences move into the message table byte-exact. `FR-DM` goes from 0 Implemented to 5, on two surfaces of five. **Two defects found by planting, neither of which would have failed a test**: the vocabulary guard had only ever read Rust, so copy in `static/*.js` was never reached by it (`static_js_scan` now covers it, with a named single-word blind spot); and the board's `<script>` tag could be deleted with the whole suite green (`§10.14`). `§10.15` opens and is not scheduled — the shipped JavaScript is executed by no test. `§10.16` opens — the four structural guards have no CI job. Test inventory 172 → 178 active at the tag, 181 after `QA-003`/`QA-004`, 0 disabled |
| 0.25.0 | RFC 010 and RFC 004a step 1. Four destructive deletes gain server-rendered confirmation interstitials, closing external design `§17.4`; the five reversible confirmations are deliberately left as dialogs. The issue detail page and issue list gain a **working** status control — the detail page had rendered three status-shaped buttons that did nothing since before 0.19.1. `FR-ISS-006` expires, satisfied for its whole term. An active sprint can no longer be deleted (`FR-SPR-002`). Test inventory 151 → 172 active, 0 disabled |
| 0.24.0 | RFC 003, the inbox refinements — shipped from a **rewritten** RFC, the first this project has returned to `proposed/` rather than amended. `FR-NTF-007` Specified → Implemented, with no schema change: the migration its first design needed was found unnecessary. §6's reservation of `0017` withdrawn — it had outlived the design that needed it by four releases. Test inventory 144 → 151 active, 0 disabled |
| 0.23.0 | RFC 002, the calendar surfaces. `§10.5`'s reduction from four date columns to two ships as designed. The project's first schema migration since the release cycle was formalised, and the first release whose rollback row is not "forward-fix only" — a downgrade across `0016` fails to start. Test inventory 125 → 144 active, 0 disabled |
| 0.22.0 | RFC 009 and RFC 001. `§10.11` closes after four releases: a team member could not be assigned an issue in their own team's project, so per-person sustainability signals were empty for every non-owner. `§10.13` closes. `NFR-PRIV-002`'s caveat discharged. The sprint planning page ships minus RFC 001's capacity hint, withdrawn as the product's first `NFR-PRIV-007` case. Test inventory 104 → 125 active, 0 disabled. This file becomes the single living baseline rather than one snapshot per release |
| 0.21.0 | RFC 006: the message table and the vocabulary guard. §1.7 gains §1.7.1 (inflection and casing) and §1.7.2 (use versus mention) — both raised by implementers who escalated an ambiguity rather than resolving it locally. §10.8 closes; §10.13 opens. Test inventory 82 → 104 active, 0 disabled. `DEC-047` settles registry publication: all seven crates publish, and `NFR-CMP-001`'s MSRV is therefore a public compatibility signal, not an internal choice |

No identifier was renumbered, withdrawn, or reused. Every change is to
text, status, priority, schedule, or the gap register (§0.5).
