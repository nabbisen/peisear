# HLT-001 — The basis route, and the one indicator that must not have it

**Issued by**: Architect
**Date**: 2026-08-27
**Priority**: P2 by `FR-HLT-007`, but it closes **Definition of Done item 3**
**Governing RFC**: [008](../../accepted/008-explainability.md) §1–§3
**Depends on**: nothing.

---

## 1. Scope, and what is deliberately not here

Two of `FR-HLT-007`'s three limbs: **the basis route** and **the
calculation**. **History is deferred** — it needs `QA-017`'s contributor
predicate and would put back on the project screen what 0.28.0 removed from
the sprint screen. If you find yourself building a time series, stop.

**Read RFC 008 §2 before writing any link.** One of the six indicators must not
get one, and the reason is a requirement collision rather than an oversight.

## 2. The basis link, per indicator

The project issue list already accepts `view`, `status`, `assignee`, `sort`.

| Indicator | Basis | Today |
|---|---|---|
| Throughput | Done vs all | `?status=done` gives the numerator |
| Staleness | oldest in-flight | status filter plus a sort |
| Bus factor | distribution across assignees | `?assignee=` per person |
| Activity | created or finished in 14 days | **needs `activity_since`** |
| Long-stale | in-flight untouched 14 days | **needs `stale_for`** |
| WIP compliance | which assignees are over their limit | **no link — §3** |

**Reproduce that table before building.** Especially the four "today" rows: if
an existing filter does not actually produce the claimed set, the link would
assert something false, and a link to the wrong issues is worse than no link.
**Report what each filter really returns.**

**Two new filters, not six.** `activity_since` and `stale_for` on the existing
query shape. Do not invent a filter language.

**The link goes on the explanation row, not the chip.** The chip is a status;
the sentence is the claim; the link belongs to the claim.

**Its accessible name must name the indicator**, not read "details" six times
on one screen — `board_keyboard`'s
`each_status_control_has_a_distinguishing_accessible_name` is the precedent.

## 3. WIP compliance gets no basis link, and a test says so

Its basis is **which assignees are over their WIP limit**. A WIP limit is in
`NFR-PRIV-001`'s inventory as *visible only to its subject*. The indicator's
sentence is already the aggregate — *"{count} active assignees are over their
WIP limit"* — a count, deliberately not names.

**A basis route would have to name them.** `FR-HLT-007` is amended by RFC 008
§2 to carve this exception; the owner approved that in accepting the RFC.

**Assert the absence.** A test that the WIP indicator renders no basis link,
and that no assignee name appears in its explanation area. **This is the one
test in the handoff that guards a privacy boundary rather than a feature**, and
it should be written first.

## 4. The calculation

Each indicator gets its **thresholds and derivation** — what counts as
Good/Watch/Concern, and over what window.

**Thresholds only, not current inputs.** The inputs are already on the page as
the explanation sentence's own numbers (*"Throughput is 0 / 1 (0%)"*);
repeating them under a disclosure is the same fact twice. That was decided in
RFC 008's acceptance note.

**Take the numbers from `peisear-core`'s classify functions — do not retype
them.** A threshold written twice is two homes for one fact, and this project
has recorded that shape six times. If a threshold cannot be reached from the
render site without duplicating it, **stop and report**; that is a structural
finding, not a detail.

New copy through `peisear-i18n` and §1.7 as usual.

## 5. Escalate rather than deciding

- **If any existing filter does not return the set its indicator claims**, stop
  and report before linking to it.
- **If a threshold cannot be read from `peisear-core` at the render site**,
  stop — see §4.
- If wiring the two new filters turns out to need a change to the issue list's
  own query handling beyond adding two optional parameters, report the shape
  before building it.
- If any basis link would expose a per-user value on a shared screen — not just
  WIP — **stop**. §3 is the known case; a second one is a finding.

## 6. Acceptance

1. §2's table reproduced, with what each existing filter actually returns.
2. Basis links on five indicators; each target verified to filter to the
   claimed set.
3. **WIP compliance renders none, asserted by a test written first.**
4. Two new filters, tested against the sets their indicators name.
5. Thresholds rendered from `peisear-core`, not retyped.
6. No time series anywhere.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 7. Required review-request format

Workflow §9.2. §2's reproduction as a table with the real filter results. State
plainly whether §5's fourth case was found anywhere.
