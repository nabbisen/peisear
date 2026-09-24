# `static/` — the files no test executes

Five scripts and three stylesheets, served by the application. **Nothing in
`cargo test --workspace` executes any of the JavaScript here**, and that is a
standing, recorded position rather than an oversight: baseline `§10.15`.

This file exists because that position has a cost, and the things that pay it
kept living in whichever handoff last remembered to ask for them.

## What the suite does and does not cover

| covered by `cargo test` | not covered by anything |
|---|---|
| the endpoints these scripts call, and their response shapes | that a script does what it says |
| the copy islands they read, key by key | the gesture itself — drag, drop, undo |
| the `<script>` tags that load them (`§10.14`) | any state the script holds between actions |
| the no-JavaScript path that works without them | |

`static_js_scan` additionally forbids a user-visible string in any file here.
It has **one** named exclusion, `search.js`, recorded in that module with its
reason. A second one is a finding, not a line to add.

## Two required evidence runs, and what each caught

A handoff that changes a script here asks for both. Neither is a `cargo test`;
both are run once for the package and reported in it. **Each has caught exactly
one defect that no gate in this project could have.**

### 1. The no-reload sequence

**Act on the same element twice, then undo, then reload once and confirm the
server agrees.** Report the element's own state — its markers, its form, what
it displays — at every step.

The optimistic update exists so a user can act repeatedly without a page load,
so that is the state the evidence has to reach. **A run that reloads between
actions verifies two first actions, not a round trip.**

*What it caught.* `PLAN-002` round 1: a row moved by drag kept its original
marker, so a second drag of the same row was silently refused until a reload.
It had passed `fmt`, `clippy`, 262 tests, the overflow gate at 90/90, and a
four-screenshot evidence run that reloaded between directions. Behind it sat a
stale form action that would have diverged the client from the server with a
toast reporting success.

### 2. The before-and-after rendering check

**When markup changes, diff the computed style for the same data, before and
after.** Not a reading of the diff — a measurement of both.

*What it caught.* `CAL-003`: the new wrapper needed `h-full` on the inner
link. That class had never been used anywhere in this codebase, so the
content-scanned stylesheet had never emitted it, and the block's height
silently collapsed from 59.875 px to 20 px while every other computed property
matched. The overflow gate would never have seen it — a 20 px block overflows
nothing. See `style/tailwindcss/README.md` for the vendoring trap it belongs
to.

## The shape every script here follows

Feature-detect and return early; read a server-rendered JSON island once and
validate its shape before attaching any listener; never author a user-visible
string; keep exactly one fallback funnel, and end it where the mutation is
confirmed. `dm.js`'s opening is the model and `JS-003` is why the outcome
classification lives in the island rather than in the script.

**Where a page has no plain control to fall back to** — the calendar has only
links — a failure announces and reloads rather than resubmitting. That is
`dm.js`'s undo rule, not its main-path rule, and `CAL-003` §3.5 records why.

## Why there is no JavaScript test harness

RFC 011 weighed one and declined it, and `§10.15` carries the reasoning. The
short form: the movable policy was moved into Rust rather than a checker being
bought to point at it, and a browser was bought for **layout** instead
(`BROWSER-001`), where it found defects nothing else could.

**Nothing above reverses that.** What it records is that the residue is larger
than it was when the decision was made — 820 lines in three files at 0.33.0,
1,663 in five at 0.37.0 — and that the two runs above are what stands in for
the tests that do not exist.
