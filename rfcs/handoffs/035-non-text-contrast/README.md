# Handoffs — non-text contrast

**Not RFC-governed.** `NFR-A11Y-010`, added 0.42.0: WCAG 1.4.11's 3 : 1 for
graphical objects and control boundaries, which this project had only for text.

| ID | Link | What | Release |
|---|---|---|---|
| A11Y-007 | [A11Y-007](./A11Y-007-two-charts-nobody-can-separate.md) | Both charts separate their series by lightness at **1.77 : 1** and **2.09 : 1**, with one bar at **1.86 : 1** against white. **Not a colour-blindness problem** — both are single-hue, so `NFR-A11Y-004` is Met; the reader this fails has low contrast sensitivity. Measure candidates before changing, cross-check one value by a second method, stay single-hue. | 0.42.0 |
| A11Y-008 | [A11Y-008](./A11Y-008-a-focus-ring-drawn-in-transparent.md) | The account menu's four links draw `outline: solid 2px` in a **transparent** colour; 10 of 14 board stops have a ring and these four do not. The style is the vendored `daisyui.min.css`'s, so the fix is a class, not an edit (`DEC-051`). **Measure the existing grey first** — if it already reaches 3 : 1 the right answer may be to record it and change nothing. | 0.42.0 |
