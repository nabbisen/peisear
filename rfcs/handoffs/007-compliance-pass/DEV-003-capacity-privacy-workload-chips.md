# DEV-003 — Remove capacity disclosure from workload chips

**Issued by**: Architect
**Date**: 2026-07-31
**Priority**: P0 — privacy (`NFR-PRIV-001`)
**Governing decision**: `DEC-019`
**Depends on**: nothing. Can run in parallel with DEV-001.

---

## 1. Purpose

Three surfaces render one member's **capacity value and over-capacity state**
to other members by name. `NFR-PRIV-001` (**P0**) lists capacity and WIP limit
as visible only to their subject. Remove the disclosure while keeping the
useful, permitted part of the signal.

## 2. Background

`NFR-PRIV-001` enumerates capacity and WIP limit as self-only. `NFR-PRIV-002`
(P1) permits sharing "workload distribution". The implementation resolved that
overlap silently in favour of disclosure.

`DEC-019` settles it: **"workload distribution" means relative load, not load
measured against another person's private limit.** An explicit P0 inventory
beats a general P1 permission, and `"— already at N pt over capacity"` attached
to a named person on a shared screen is precisely the oversight posture
`SPEC §11.3` refuses.

This is a live disclosure. It is on screen today for anyone with project access.

## 3. Applicable requirements

| ID | Requirement | Priority |
|---|---|---|
| `NFR-PRIV-001` | Capacity settings and WIP limit are visible only to their subject | P0 |
| `NFR-PRIV-002` | Workload distribution MAY be shared — as amended by `DEC-019` | P1 |
| `NFR-PRIV-007` | Aggregates must not be reversible to individuals; suppress where an aggregate resolves to one person | P2 |
| `NFR-LANG-001` | Non-evaluative vocabulary | P0 |
| `NFR-A11Y-004` | Meaning not carried by colour alone | P1 |

## 4. Change scope

- `crates/peisear-web/src/components/issues.rs`
  - `WorkloadStrip` (defined ~L316; rendered at L97, **project detail screen**)
  - `WorkloadHint` (defined ~L376; rendered at L806 issue-create and L1309 issue-edit)
- `crates/peisear-web/src/handlers/*` — only if the capacity field can stop
  being fetched for these views
- `crates/peisear-core/src/lib.rs` — **only** the `WorkloadState` presentation
  mapping if step 4 requires it
- New test file, e.g. `crates/peisear-web/tests/workload_privacy.rs`
- `CHANGELOG.md`

## 5. Non-change scope

- **`/today` and `/settings`.** The subject seeing their own capacity is
  correct and required (`FR-PER-002`, `FR-PER-003`). Do not touch them.
- `/api/users/{id}/capacity`. Already self-only and tested.
- `peisear_core::workload_state` / `projected_workload_state` **computation**.
  Internal computation may keep using capacity (`FR-HLT-009` — computation and
  presentation are separate concerns). This task changes *presentation only*.
- The health strip and score badge. That is DEV-004.

## 6. Required implementation

1. **Remove the capacity denominator from shared surfaces.** `{in_flight}/{cap} pt`
   becomes in-flight load alone. The `Some(cap) => format!("{}/{} pt", …)`
   arms in both `WorkloadStrip` and `WorkloadHint` go.

2. **Remove the over-capacity annotations** in `WorkloadHint`:
   `"— already at {} pt over capacity"` and `"— strained"`. Both disclose
   capacity by inference and attribute a state to a named person.

3. **Remove the capacity-derived badge colour on shared surfaces.** A badge
   whose colour is computed from `workload_state` re-encodes the same private
   fact — removing the text while keeping `badge-error` discloses it anyway.
   Present in-flight load in a neutral, non-graded style.

4. **Do not report another member's `WorkloadState` at all** on these three
   surfaces. `Overloaded` and `Strained` are evaluative labels about a person
   derived from private data; neither may reach a third party in text, colour,
   glyph, or title attribute.

5. ~~**Apply `NFR-PRIV-007` suppression**~~ — **WITHDRAWN 2026-08-01
   (ISSUE-003).** If you already implemented the `.len() <= 1` suppression,
   **revert it.**

   I misapplied the requirement. `NFR-PRIV-007` concerns *aggregates* that
   inadvertently resolve to an individual. A chip labelled with a person's name
   is not an aggregate — it is individual workload, which `NFR-PRIV-002`
   permits. And because `project_workload` joins on `projects.owner_id`, it
   returns **at most one row, always**: suppressing at n≤1 would not protect
   privacy, it would silently disable the surface on every project that exists.

6. **Check the `title=` attributes.** `WorkloadStrip` builds
   `"{display_name} — {n} in-flight issues"`. In-flight count is permitted
   workload distribution and may stay — but re-read every tooltip on these
   surfaces for capacity leakage before you finish.

7. **Keep the assignment-time signal that is permitted.** In-flight issue count
   and in-flight points per member remain visible. The planner keeps the "who
   is carrying a lot right now" signal; they lose "who is over *their* limit".

## 7. Required tests

New integration tests asserting, for a fixture where user B has a capacity set
and is over it, and user A is viewing:

1. Project detail (`/projects/{id}`) does not contain B's capacity value.
2. Issue create form does not contain B's capacity value.
3. Issue edit form does not contain B's capacity value, nor
   `"over capacity"`, nor `"strained"`.
4. None of the three surfaces contains a capacity-derived danger badge class
   for B.
5. B's own `/today` and `/settings` **do** show B's capacity — a guard against
   over-correction.
6. ~~A strip resolving to exactly one member is not rendered.~~ **WITHDRAWN** —
   see §6 item 5. Every real invocation has exactly one row, so this assertion
   would be trivially true for the wrong reason.

Tests 1–4 are constructible using the **project owner** as the disclosing
party: user A views a project owned by user B, and B's capacity must not
appear while B's in-flight load does. That is a real, correct test of the
actual defect — the disclosure was always the owner's capacity, not an
arbitrary member's.

Assert on rendered HTML, not on internal state — these are disclosure tests.

## 8. Acceptance criteria

1. No surface renders another user's capacity value, WIP limit, or a state
   derived from either, in any form — text, colour, glyph, tooltip, or
   attribute.
2. The subject's own view is unchanged.
3. ~~In-flight load per member remains visible where it was.~~ **WITHDRAWN** —
   unachievable as written. `project_workload` has only ever returned the
   project owner, so "per member" describes something that has never existed.
   Replaced by: *the owner's in-flight load remains visible; only capacity and
   capacity-derived state are removed.*
4. ~~Single-member strips are suppressed.~~ **WITHDRAWN** — see §6 item 5.
5. fmt, clippy `-D warnings`, and the full web test suite are clean.

## 9. Prohibited shortcuts

- **Do not** hide the chips with CSS or a template conditional while the values
  remain in the HTML. `NFR-PRIV-004`: the UI is not the security boundary. If
  it is in the response body, it is disclosed.
- **Do not** solve this by gating on role. A team admin must not see it either
  (`NFR-PRIV-003`) — there is no role for whom this is permitted.
- **Do not** delete `workload_state` from `peisear-core`. Computation stays;
  presentation changes.
- **Do not** amend `NFR-PRIV-001` to legalise the current behaviour. The
  requirement is correct; the code is wrong. The baseline amendment
  (clarifying `NFR-PRIV-002`) is the architect's to make, not yours.

## 10. Known risks

| Risk | Mitigation |
|---|---|
| Planners lose a signal they use | Accepted and deliberate per `DEC-019`. In-flight load is retained; only the private denominator goes |
| Capacity may leak on a surface not listed in §4 | Grep the whole web crate for `capacity_points` before finishing and report every render site you find, including ones this handoff missed |
| Over-correction removing the subject's own view | Test 5 guards this |

## 11. Required evidence

- Changed-file list.
- fmt and clippy output.
- Full output of the new test file plus `auth_boundary`.
- The result of the `capacity_points` grep across `crates/peisear-web/src`,
  with each hit classified as self-only, shared-and-fixed, or shared-and-remaining.
- Before/after rendered HTML excerpts for one shared surface.

## 12. Required review-request format

Per workflow §9.2, into `.git-exclude/review-request/`. Request focused review
on the grep results in §11 — the three sites in §4 are the ones I found, and
completeness is the thing most likely to be wrong.

**Escalate rather than deciding** if you find capacity reaching any surface not
listed in §4, or if removing it breaks a test that asserts current behaviour —
that would mean a test encodes the disclosure, which is a separate finding.
