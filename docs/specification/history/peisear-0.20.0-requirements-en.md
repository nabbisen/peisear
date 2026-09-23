# peisear — Software Requirements Specification

**Document status**: Baseline
**Covers release**: `0.20.0` (compliance pass; implementation through `0.20.0`)
**Supersedes**: `peisear-0.19.1-requirements-en.md`, retained unedited as the
record of that release
**Language**: English (normative)
**Prepared**: 2026-07-27 · **Amended**: 2026-08-03
**Intended repository location**: `docs/src/requirements.md` (mdbook-compatible)
— not yet placed; see `DEC-020`, unresolved

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
*Status*: Implemented. *Priority*: P2.

**FR-ISS-006 — Status segment is display-only until Phase D**
Until the direct-manipulation phase, the status segment MUST NOT mutate
state on click; status changes go through the edit form.
*Rationale*: shipping the affordance early without the interaction lets
the layout settle without promising behaviour that does not yet exist.
*Source*: decision `DEC` in `SPEC §27` phasing. *Acceptance*:
`edit_form_keeps_existing_select_widget` guards against the edit form
being replaced by the segment prematurely. *Status*: Implemented.
*Priority*: P2.

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
*Status*: Implemented (untested). *Priority*: P2.

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
each transition triggered by an explicit administrator action.
*Source*: `SPEC §9.1`, `SPEC §17.3`. *Status*: Implemented. *Priority*: P1.

**FR-SPR-002 — One active sprint per team**
A team MUST have at most one active sprint at a time.
*Source*: `GUI §5`. *Status*: Implemented (untested). *Priority*: P2.

**FR-SPR-003 — Carry-over is factual, not failure**
Work not completed within a sprint MUST be described neutrally
("X carried over"), never as failure or shortfall.
*Source*: `SPEC §3` vocabulary table. *Status*: Implemented.
*Priority*: P1.

**FR-SPR-004 — Completed sprints are immutable**
A completed sprint's issue membership MUST NOT be editable.
*Rationale*: editing a completed sprint rewrites history and corrupts
trend data.
*Source*: derived; RFC 0001 requirement 8. *Status*: Specified
(the planning screen enforcing this is unbuilt). *Priority*: P2.

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
*Source*: `SPEC §38.1`. *Status*: Specified. *Priority*: P2.

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
*Source*: `BRIEF §4.4`, `SPEC §28.6`. *Acceptance*: healthy and
insufficient indicators produce no explanation row;
`health_explainability` test crate. *Status*: Implemented. *Priority*: P1.

**FR-HLT-006 — Explanation neutrality**
Explanation text MUST be descriptive and MUST NOT contain evaluative
words or directives.
*Source*: `SPEC §3`, `NFR-LANG-001`. *Status*: Implemented (untested —
no automated vocabulary guard exists). *Priority*: P0.

**FR-HLT-007 — Indicator basis is traceable**
Each indicator MUST offer a route to its basis: the underlying issue
list, the calculation, and recent history — not only a tooltip.
*Source*: `SPEC §28.3`, Definition of Done `§41.3`. *Status*: **Not
implemented.** Explanation sentences exist, but there is no
"what this is based on" link or detail screen. See §10.4. *Priority*: P2.

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
*Source*: `SPEC §29.3.1`, `SPEC §19.4`. *Status*: Specified (RFC 0003).
*Priority*: P2.

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
*Source*: `SPEC §38.1`. *Status*: Specified — requires migration
`0016`. Note the scope reduction recorded in §10.5. *Priority*: P2.

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

**FR-DM-001 — Five direct-manipulation surfaces**
The system SHOULD provide direct manipulation for: status change from a
list, kanban column drag, calendar block drag, sprint-planning drag, and
issue list reordering.
*Source*: `SPEC §21.2`, `SPEC §39`. *Status*: **Partial** — kanban column
drag has shipped since approximately 0.6.0; the other four remain Deferred
(Phase D, RFC 004). *Priority*: P2.
*Correction*: recorded `Deferred` at 0.19.1 while one of its five surfaces
was in production. That misstatement is what allowed `FR-DM-002` to be
violated unnoticed — a requirement believed dormant is not checked.

**FR-DM-002 — Keyboard parity**
Every direct-manipulation action MUST have a keyboard equivalent
producing the identical effect. A mouse-only action MUST NOT exist.
*Rationale*: `SPEC §32` treats the keyboard path as the contract and the
pointer path as an enhancement.
*Source*: `SPEC §21.3.1`, `SPEC §32`. *Acceptance*: `board_keyboard` test
crate (6 tests). *Status*: **Implemented for the kanban board** (0.20.0);
in force and unmet for any future direct-manipulation surface until that
surface ships its keyboard path first. *Priority*: P0.
*Correction*: recorded `Deferred` at 0.19.1. A deferred requirement cannot
be violated — but because `FR-DM-001`'s drag surface had shipped, this one
was in force and unmet: the board's status change had no keyboard path at
all. `DEC-021` now requires the no-JS path to ship *before* the pointer
affordance, so the ordering cannot recur.

**FR-DM-003 — Optimistic update with rollback**
Direct manipulation SHOULD apply the change to the interface
immediately and reconcile with the server afterward, reverting the
interface if the server rejects the change.
*Source*: `SPEC §21.3.2`. *Status*: Deferred. *Priority*: P2.

**FR-DM-004 — Conflict handling on 409**
On a conflict the interface MUST revert the optimistic change, notify
the user in neutral language, refetch the entity, and re-render the
current state. It MUST NOT retry automatically.
*Source*: `SPEC §21.4.5`. *Status*: Deferred. *Priority*: P1.

**FR-DM-005 — Conflict message vocabulary**
Conflict messages MUST state that another member changed the item first
and that the latest state is now shown. They MUST NOT use failure or
error vocabulary, and MUST NOT use danger colouring.
*Source*: `SPEC §21.4.6`, `SPEC §21.4.8`. *Status*: Deferred.
*Priority*: P1.

**FR-DM-006 — Undo window**
A completed direct manipulation SHOULD offer a brief undo affordance
(approximately five seconds).
*Source*: `SPEC §39.2`. *Status*: Deferred. *Priority*: P2.

**FR-DM-007 — No celebratory feedback**
Completion feedback MUST remain factual. Celebration, congratulation,
or achievement framing MUST NOT be used.
*Source*: `SPEC §23.5`, `SPEC §25.2`. *Status*: Deferred. *Priority*: P1.

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
*Caveat*: this requirement describes something that has never fully
existed — see `FR-ISS-002` and §10.11.

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
*Source*: `SPEC §11.5.3`. *Status*: **Not implemented.** *Priority*: P2.
*Scope correction (0.20.0)*: a suppression was added to the workload chips
during DEV-003 on the strength of this requirement, then withdrawn. It was
a misapplication: this requirement concerns *aggregates* that inadvertently
resolve to an individual. **A chip labelled with a person's name is not an
aggregate** — it is individual workload, governed by `NFR-PRIV-002`.
Suppressing it at one member would not have protected privacy; because
`project_workload` returns at most one row (§10.11), it would have
silently disabled the surface on every project in existence.
The requirement itself stands, unimplemented. No genuine aggregate
currently exists that could resolve to one person; when one is built —
sprint charts and health trends are the likely candidates — this is the
requirement it must satisfy.

**NFR-PRIV-008 — Automated authorisation regression tests**
Every endpoint in the personal-data inventory MUST have automated tests
asserting 403 for another user, 403 for an administrator, and 401 for an
unauthenticated caller.
*Source*: `SPEC §11.5.5`, `SPEC Appendix E.5`. *Status*: Partial — full
coverage for implemented endpoints; the audit completing this is
Phase E. *Priority*: P1.

### 5.2 Concurrency and data integrity — `NFR-CONC`

**NFR-CONC-001 — Optimistic locking on owned entities**
Every mutation of an entity owned by a single record MUST carry the
client's observed `updated_at` and MUST be rejected with 409 if it does
not match the stored value.
*Source*: `SPEC §21.4.2`. *Acceptance*: `optimistic_lock` test crate (8
tests), `board_keyboard`. *Status*: Implemented (0.20.0). *Priority*: P0.
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
*Source*: implementation decision `DEC-013`, migration `0014`.
*Status*: Implemented. *Priority*: P1.

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
*Status*: Implemented (0.20.0). *Priority*: P1.
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
*Source*: `SPEC §30.1`. *Status*: Partial (per-surface; systematic audit
is Phase E). *Priority*: P0.

**NFR-A11Y-002 — Focus management**
After a mode change, focus MUST move to a defined, visible location, and
MUST NOT be sent off-screen.
*Source*: `SPEC §30.1`. *Status*: Partial. *Priority*: P1.

**NFR-A11Y-003 — Screen reader equivalence for charts**
Every chart MUST provide a one-sentence summary label, a two-to-three
sentence textual summary, and a tabular equivalent of its data.
*Source*: `SPEC §31.1`. *Status*: **Not implemented** for existing
charts. See §10.4. *Priority*: P1.

**NFR-A11Y-004 — Meaning not carried by colour alone**
State MUST be conveyed by label and icon in addition to colour.
Colour-blind-safe patterning MUST be used in charts.
*Source*: `SPEC §30.1`, `SPEC §31.2`. *Status*: Partial. *Priority*: P1.

**NFR-A11Y-005 — Contrast**
Text and background combinations MUST meet WCAG AA (4.5:1).
*Source*: `SPEC §40.1`. *Status*: Not verified — Phase E audit.
*Priority*: P1.

**NFR-A11Y-006 — Mobile completion of key flows**
The following MUST be completable on a phone: reviewing and reading
notifications; viewing `/today`; changing issue status from the issue
detail; viewing today's calendar.
*Source*: `SPEC §33.1`. *Status*: Not verified — Phase E audit.
*Priority*: P1.

**NFR-A11Y-007 — Touch target size**
Interactive elements MUST present a touch target of at least 44 × 44
pixels.
*Source*: `SPEC §33.2`. *Status*: Not verified. *Priority*: P1.

**NFR-A11Y-008 — Live region announcements**
Dynamic changes MUST be announced through an appropriate live region;
conflict notifications MUST use an assertive region.
*Source*: `SPEC §21.4.8`. *Status*: Deferred with Phase D. *Priority*: P1.

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
*Source*: `SPEC §3`, `SPEC §28.6.3`. *Status*: Implemented by
convention; no automated guard exists. *Priority*: P0.

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
*Source*: `KICK §7`. *Status*: Implemented. *Priority*: P1.

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

`0016` is reserved for calendar planned-date columns (`FR-CAL-003`);
`0017` for the deferred email opt-in state (`FR-NTF-007`).

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
| `/projects/{id}/issues/{iid}/sub-issues/new` | No | If accessible | No | Yes | Yes |
| `/projects/{id}/calendar` | No | If accessible | Read | Read | Read |
| `/teams/{slug}` | No | Member only | Read | Read | Manage |
| `/teams/{slug}/sprints/{sid}` | No | Member only | Read | Read/write | Manage |
| `/teams/{slug}/sprints/{sid}/plan` | No | Member only | Read | Read/write | Read/write |
| `/settings*` | No | Own only | Own only | Own only | Own only |
| `/search` | No | Own scope | Own scope | Own scope | Own scope |
| `/api/users/{uid}/*` | 401 | Self only | Self only | Self only | Self only |

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
| 1 | **Privacy maintained** — personal data boundaries hold in UI and API | `NFR-PRIV-001..008` | Largely met. Gaps: storage-layer defence in depth (§10.3); aggregate suppression (`NFR-PRIV-007`) |
| 2 | **Quiet by design** — edge-triggered, cooled down, not chasing | `FR-NTF-001..003`, `FR-NTF-006..007` | Partially met. Silence-resume placement and deferred opt-in are Specified |
| 3 | **Explainable** — indicators state what they do and do not show | `FR-HLT-005..009`, `NFR-A11Y-003` | Partially met. Explanations exist; basis links (`FR-HLT-007`) and chart alternatives (`NFR-A11Y-003`) absent; headline score contradicts (`FR-HLT-008`) |
| 4 | **Light by default** — analysis does not obstruct basic use | `FR-PER-006..007`, `FR-PROJ-004` | Met for shipped surfaces. One-click status change awaits Phase D |
| 5 | **Reach** — keyboard, screen reader, and mobile flows succeed | `NFR-A11Y-001..007` | Not verified. Phase E audit |

---

## 9. Verification and traceability

### 9.1 Test inventory at this baseline

| Suite | Tests | Primary requirements verified |
|---|---|---|
| `smoke` | 11 | `FR-AUTH-003/004`, `FR-NAV-002`, `FR-PER-001` |
| `auth_boundary` | 10 | `FR-AUTH-005`, `FR-API-002/003`, `NFR-PRIV-003/006` |
| `search` | 9 | `FR-SCH-001..004` |
| `optimistic_lock` | 8 | `NFR-CONC-001/005` — **per entry point**, JSON and form |
| `sub_issues` | 7 | `FR-SUB-001/006/007` |
| `health_explainability` | 7 | `FR-HLT-005/008`, `NFR-LANG-002` |
| `board_keyboard` | 6 | `FR-DM-002`, `NFR-A11Y-001/004/007` |
| `view_state` | 5 | `FR-NAV-005` |
| `workload_privacy` | 4 | `NFR-PRIV-001/002` |
| `issue_edit_url` | 3 | `FR-ISS-004` |
| `today_panel` | 3 | `FR-PER-006/007` |
| `breadcrumb` | 2 | `FR-NAV-003` |
| `status_segment` | 2 | `FR-ISS-005/006` |
| storage library | 2 | `FR-SCH-004` |
| notification library | 3 | `FR-NTF-001/002` |

Totals: **77 web integration test functions, 0 disabled**, plus 5 library
tests — **82 active** (was 65 active with 1 disabled at 0.19.1).

Two conventions adopted at 0.20.0, both from defects this release found:

1. **Coverage is recorded per entry point, not per requirement.** An
   endpoint reachable by more than one route (`change_status` takes JSON
   and form) needs a test per route. Recording it per requirement is what
   let `NFR-CONC-005` read as covered while the path the shipped client
   used was untested.
2. **A test crate without a CI job does not exist.** All thirteen were
   swept against `.github/workflows/test.yml` during DEV-009; all thirteen
   have one.

### 9.2 Requirements without automated verification

The following are implemented but unverified, and are the highest-value
targets for new tests: `FR-HLT-006` (explanation neutrality — no
vocabulary guard exists), `NFR-LANG-001` (same, product-wide),
`FR-SUB-009` (cascade deletion), `FR-TEAM-003` (last-admin guard),
`FR-TEAM-004` (non-member concealment), `FR-TEAM-005` (privacy
footnote), `FR-SPR-002` (single active sprint).

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

### 10.0 How to read this register at 0.20.0

Gaps are recorded rather than resolved silently in either direction, and
**closed gaps stay here with their resolution**. A register that only lists
open items cannot show whether a class of defect recurs.

| Gap | State at 0.20.0 |
|---|---|
| §10.1 indicator set differs from `SPEC §28.1` | Decided (`DEC-030`), amendment pending |
| §10.2 health presentation exceeds the ceiling | **Closed** |
| §10.3 storage-layer authorisation absent | Open — Phase E |
| §10.4 explainability affordances incomplete | Open — 0.25.0, RFC 008 |
| §10.5 calendar schema scope reduced | Open, deliberate |
| §10.6 kanban endpoint bypassed the optimistic lock | **Closed** |
| §10.7 capacity disclosed to non-subjects | **Closed** |
| §10.8 domain crate generates user-visible prose | Open — RFC 006, 0.21.0 |
| §10.9 `fmt`/`clippy` had never passed | **Closed** |
| §10.10 `/today` rendered danger colouring | **Closed** |
| §10.11 workload and assignee queries return only the owner | **Open** |
| §10.12 health **summary prose** names the unclamped state | **Closed** — 0.20.1 |

Six closed, six open. Of the closed six, **five were not recorded as gaps at
0.19.1 at all** — they were requirements annotated as satisfied. That is the
pattern this release exists to break, and §10.0 exists so the next reader can
see whether it held.

**It did not hold at 0.20.0.** §10.12 was found days after this baseline was
written, by the same means as the others — someone reading the code for an
unrelated reason. §10.2 and §10.10 were closed while a third instance of the
same violation, on the same screen, went unexamined.

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

*Assessment*: the current boundary is correct and tested; this is a
resilience gap rather than an open hole. Scheduled with the Phase E
authorisation audit (`NFR-PRIV-005`).

### 10.4 Explainability affordances incomplete

Two `SPEC` requirements in the explainability family are unimplemented:

- `FR-HLT-007` — no "what this is based on" link or detail view exists;
  indicators offer a sentence but no route to the underlying issue list,
  calculation, or history.
- `NFR-A11Y-003` — existing charts do not provide tabular equivalents or
  textual summaries.

*Assessment*: both are named in Definition of Done item 3, so this
condition cannot be considered met until they land.

### 10.5 Calendar schema scope reduced

`SPEC §38.1` lists four date columns (`start_date`, `due_date`,
`planned_start_at`, `planned_end_at`). RFC 0002 reduces this to two
planned-time columns, reasoning that `start_date` duplicates the date
part of `planned_start_at` and that carrying both `due_date` and
`planned_end_at` invites ambiguity about which is authoritative.

*Assessment*: a deliberate, recorded narrowing. It is listed here so the
divergence from `SPEC` is not mistaken for an oversight. Revisit if a
firm deadline must be distinguished from a soft estimate.

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

### 10.8 The domain crate generates user-visible prose — open

`peisear-core::Indicator::human_explanation()` returns English sentences,
so the computation crate performs presentation. This sits badly with
`FR-HLT-009` and `NFR-MNT-001`, and it means a vocabulary guard over the
web layer alone would have a hole exactly where `FR-HLT-006` applies.

*Addressed by RFC 006 §D3 at 0.21.0*: the domain emits a message key plus
typed parameters; presentation renders it.

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

### 10.11 Workload and assignee queries return only the project owner — **open**

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

---

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
| Direct manipulation | 7 | — | — | — | 7 | — |
| **Functional total** | **99** | **70** | **4** | **16** | **7** | **2** |
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
| **0.20.0 (this baseline)** | Compliance pass. Nine recorded statuses corrected — five of them P0 or P1 requirements annotated as satisfied while the code did the opposite. `NFR-PRIV-002` scoped (`DEC-019`); `NFR-CMP-001` extended with the toolchain/MSRV distinction (`DEC-044`/`DEC-045`); `NFR-LANG-005` rescheduled to 0.21.0 (`DEC-022`). §10 gains a state table and six new entries, five of which close in this release. Test inventory 65 → 82 active, 0 disabled. `RSK-001` closed |

No identifier was renumbered, withdrawn, or reused. Every change is to
text, status, priority, schedule, or the gap register (§0.5).
