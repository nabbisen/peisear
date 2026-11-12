# MARK-001 — the other six

**Series**: `061`, `0.63.0`'s authorized scope. **Register**: `§10.58`.
**Blocked on**: nothing.

## 1. What is wrong, measured at source 2026-11-12

Fourteen `<summary>` elements ship in `crates/peisear-web/src/components`.
**Seven render a disclosure marker. Six do not.**

| file | markerless sites |
|---|---|
| `components/me.rs` | lines 360, 509 |
| `components/settings.rs` | lines 142, 284 |
| `components/sprints.rs` | lines 435, 979 |

Each takes `grow_text(...)`, which appends `inline-flex items-center`;
a `::marker` box exists only on a `display: list-item` element, so the
triangle is **never created**. `DISC-001` fixed three sites in
`issues.rs` and measured — not fixed — these six.

**The fifteenth site is not a defect**: `issues.rs:595`, the *"How this
is calculated"* toggle, takes no helper at all. It is already
`list-item`, renders its marker, and is a named escalation exclusion in
`touch_target_scan` — deliberately below 44 × 44. **Do not change it**,
and say in the report that you read it rather than assuming.

## 2. What to build

`DISC-001`'s shape, applied to the six: `grow(...)` instead of
`grow_text(...)`, with the centring supplied by **symmetric padding
whose arithmetic reaches the context's own line-height**, stated in a
comment at each site as `DISC-001` did. Only classes already in the
shipped stylesheet — `css_scan` is the check that catches you, and it
caught `pt-3` last time.

## 3. The decision this round may need to make, and its rule

`alignment_scan`'s `is_padding_centred_disclosure` is a **literal list
of two padding suffixes**, kept narrow on purpose. Six more sites may
not all share those two suffixes.

**If the exclusion would name more than a handful, replace the list with
the property it stands for** — `tag == "summary"` **and** a symmetric
`py-*` class present — and record in the module doc why the shape
changed. That scan's own doc already draws this line: *"a sixth such
binding would need the same verification, not an extension to this scan,
unless it stops being a handful."* **Six is where it stops being a
handful.** Verify each site by a real render before widening, exactly as
`DISC-001` did, and say so.

## 4. The guard, which is the point of the round

**A source scan: no `<summary>` is rendered with `grow_text(`.** It
cannot pass while any instance of this defect exists, which the browser
gate cannot promise for pages it does not visit. Population stated and
pinned, the way `css_scan` pins its own.

`browser-checks/disclosure-marker-gate.mjs` **also** extends to the
pages these six live on (`/today`, settings, sprint detail) — the
behavioural proof beside the source proof, not instead of it.

## 5. Acceptance

- All six render `display: list-item` with a marker, **measured in a
  browser at 390 and 1280**, and each still measures ≥ 44 × 44.
- `touch_target_scan`'s three assertions stay green.
- The source scan **planted** — put `grow_text(` back on one site and
  show it failing by name.
- `DEC-007` from → to, re-derived with the runner.
- The six documentation screenshots regenerated if any of them shows one
  of these surfaces, and opened before committing.

## 6. What this round must not do

- Do not touch `issues.rs:595` or any other escalation exclusion.
- Do not change what any disclosure contains, or what is open by default
  (`FR-PER-007`, `FR-NAV-006`, `external-design §5.10` all still hold).
- Do not weaken `grow_text` for controls that are not `<summary>`.
- Do not amend `requirements.md`; `§10.58` records this already.
