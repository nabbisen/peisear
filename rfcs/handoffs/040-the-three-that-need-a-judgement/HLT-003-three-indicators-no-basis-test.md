# HLT-003 — three indicators, no basis test

**Requirement**: `FR-HLT-007` (P2). **Register**: `§10.35`. **Target**: 0.47.0.

`FR-HLT-007` says **each** indicator must offer a route to its basis. There
are **six** `IndicatorKind` variants (`peisear-core/src/lib.rs:856`); one
(`WipCompliance`) is excepted by owner-approved amendment with a confirmed
absence-test. **Of the remaining five, two are tested.**

`basis_route`'s five tests are: the exception's own absence test, a 404 edge
case, `throughput_basis_is_exactly_the_done_issues`,
`staleness_basis_is_exactly_the_oldest_in_flight_issue`, and
`linked_indicators_render_distinguishing_basis_links` — **which, despite the
plural, exercises Throughput alone.** I verified that by extracting every
indicator slug in its body: one.

**Untested: `Activity`, `BusFactor`, `LongStale`.**

---

## §0 — the slugs, because two of us have already got them wrong

`from_slug` (`lib.rs:914`) maps:

| kind | slug | basis set (`basis_for`, `:713`) |
|---|---|---|
| `Activity` | **`activity`** | `recent_activity_issue_ids` |
| `BusFactor` | **`bus-factor`** | `in_flight_issue_ids` |
| `LongStale` | **`long-stale`** | `long_stale_issue_ids` |

**Hyphens, not underscores.** `REQ-005` reported these three as untested after
grepping `health/bus_factor` and `health/long_stale` — **the wrong needles**.
I re-checked with the real slugs and **the conclusion holds**: zero test
references for all three, one each for `throughput`, `staleness` and
`wip-compliance`. **But a negative obtained with the wrong needle is not
evidence, even when the answer survives** — and if you write a test against
`bus_factor` you will get a 404 from `from_slug` and may read it as the route
being broken.

## §1 — why this is not one shared code path

`health_indicator_basis` (`handlers/issues.rs:311`) is shared, which is why
this looked like a legitimate sample. **It is not.** The per-indicator step is
`raw.basis_for(kind)`, a `match` returning **a different field for each
kind** — three different sets for these three. `REQ-004`'s `FR-SUB-006`
near-miss declined to score a gap because `list_in_project` had **one call
site and one path**; here the path forks per kind, so three untested kinds are
three untested basis computations.

**Each test therefore needs a fixture that makes its own set non-empty**, and
they are three different fixtures: recent activity, in-flight issues, and
long-stale issues.

## §2 — the distinction to get right

`basis_for` returns `Option<&[String]>`, and the handler turns `None` into
**404**. Only `WipCompliance` returns `None`.

So for these three, **an empty basis is `Some(&[])`** — the route returns
**200 with an empty list**, not 404. **Assert the populated case**, and if you
also assert the empty case, assert it as *200 with nothing listed* rather than
as a 404. **Confusing those two is the most likely way these tests pass while
proving nothing**: a 404 assertion would pass today against a slug typo.

**Follow `throughput_basis_is_exactly_the_done_issues`'s shape.** Its name
carries the property — *exactly* — and that is the right assertion: the basis
list contains the issues that belong and **not** ones that do not. A test that
only checks the right issue appears passes against a route listing every
issue in the project.

## §3 — and rename the plural test

`linked_indicators_render_distinguishing_basis_links` covers one indicator.
**That name is why `FR-HLT-007` read like a sample for six releases**, and it
is `§10.17`'s shape in a test name — the same defect `DM-TEST-002` renamed
`undo_dom_order` for.

**Either** rename it to say what it checks, **or** widen it to the indicators
its name promises. **Your call, with the reasoning** — if widening it is
cheap now that you have three fixtures, that is the better outcome; if the
fixtures do not compose, rename it. **Do not leave the name as it is.**

## §4 — gates

- **`DEC-007`** three consecutive runs, composition reported.
- **Each test seen to fail.** For the *exactly* property, plant an extra issue
  into the project that should **not** be in the basis and confirm the test
  fails. A basis test that passes against an over-broad list is the failure
  mode here.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.
- **No component should change.** If one does, a gap is a defect.
- **Overflow gate** only if markup changes; expected not to.

## §5 — escalate rather than deciding

- **If any of the three routes does not work.** Three untested basis
  computations is exactly where a real defect could be sitting, and that
  stops being a test task.
- **If a fixture cannot make a set non-empty honestly** — particularly
  `recent_activity_issue_ids`, whose window may need event rows.
- **If `WipCompliance`'s absence-test turns out not to assert what the
  amendment says it does.** `COV-001` read it and found it real; if you
  disagree, say so.

## §6 — exit condition

Three tests, one per untested indicator, each asserting its basis **exactly**
and each seen to fail against an over-broad list; the plural-named test
renamed or widened **with the reasoning**; `DEC-007` three times at the new
figure.

**`FR-HLT-007` then has an acceptance covering five of five non-excepted
indicators**, which is mine to record.
