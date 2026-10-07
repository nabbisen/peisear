# Handoffs — reach after navigation

**Not RFC-governed.** `NFR-A11Y-011`, split out of `NFR-A11Y-002` at 0.41.0 and
recorded **Not met**: every native form POST leaves focus on `body`, and
reaching the content costs eleven Tab presses with no skip link.

| ID | Link | What | Release |
|---|---|---|---|
| A11Y-006 | [A11Y-006](./A11Y-006-eleven-tab-presses-to-the-content.md) | A skip link — **the only one of the requirement's three options that works without JavaScript**, which a remedy for reaching content must. One element in the layout, `<main id="main" tabindex="-1">`, and the `tabindex` measured rather than trusted. Expect the overflow gate to be the thing that catches a bad visually-hidden style. | 0.42.0 |
| A11Y-009 | [A11Y-009](./A11Y-009-an-invisible-target-and-a-second-main.md) | Two things `A11Y-006` raised. `grow()`'s unconditional `min-h-11 min-w-11` beats `sr-only`'s 1×1, so **the hidden skip link is a 44×44 clipped box in every page's top-left corner** — measure whether `clip` removes it from hit-testing before changing anything. And `sprint_plan.rs` has **two `main` landmarks**, which the skip link now makes load-bearing. | 0.42.0 |
| REL-0.42.0 | [REL-0.42.0](./REL-0.42.0-release-candidate.md) | Release candidate — the skip link, both charts' contrast, the transparent focus ring, and `NFR-A11Y-010`. **The test count does not move**, which is the thing most likely to be misread: the work was measurement, and measurement closes a requirement without adding an assertion. | 0.42.0 |

