# CAL-006 — a Move that says so

**Review:** `.git-exclude/reviewed/REL-0.45.0-candidate-review.md` (§2)
**The candidate is held pending this**, on the architect's recommendation;
the owner's call. Written now so it is not waiting on the decision.

**The gap is mine before it is yours.** `CAL-004` §2.4 specified *where the
user lands* and never asked *what the user sees*. Your behavioural check
asserted the redirect target and the stored row, both correctly, and neither
was written to catch this.

---

## §1 — the defect, in the day view

`change_schedule_move` returns `Redirect::to("{base}?view=…&date=…")` and
**sets nothing else**. The redirect target is right — the user returns to the
window they came from, which is `FR-NAV-005`'s principle and which I recorded
one commit before the candidate. **What is wrong is the silence.**

In the day view the window is **one day**, so:

1. A user on today's day view follows a block's link.
2. They Move it to tomorrow — the control's whole purpose.
3. They land back on today's day view, correctly.
4. **The block is gone, and nothing says anything.**

**Every successful Move from a day view makes its subject vanish from the
page the user lands on.** A reasonable reader concludes the issue was deleted.

## §2 — the fix, using only what exists

**Do not change the redirect target.** Returning the user where they came
from is correct. Add the message.

Four small pieces, all with a precedent in the tree:

1. **`CalendarQuery` gains `flash: Option<String>`**
   (`handlers/calendar.rs:29`), beside `view` and `date`.
2. **The two calendar pages pass it through.**
   `components/calendar.rs:537` and `:594` currently hand `AppShell`
   `flash={None::<String>}` — the slot is already wired and always empty.
   Pass the query's value instead.
3. **`change_schedule_move` appends it**, the way `settings.rs:145` already
   does: `Redirect::to(&format!("/settings?flash={flash}"))`, with the
   existing `super::percent_encode_query` helper. **Use that helper** — a
   date string is tame, but hand-rolling encoding beside a function that
   exists is how a difference appears later.
4. **One message key**, through `peisear-i18n`, naming **the day the issue
   moved to**. The user's question is *where did it go*, so the message must
   answer that rather than say "saved". A date the user can read, not an
   ISO stamp.

**`flash` is one-shot.** `handlers/issues.rs:70` already records that it is
*"intentionally NOT inherited"* when view state is persisted. Match that
treatment: it must not become part of any saved default, and the calendar's
own prev/next links must not carry it forward.

## §3 — the question you should answer, not assume

**Should the message name the day, or the day *and* that the length was
kept?** My instinct is the day alone — the user asked for a move, the length
not changing is the control's promise rather than news — but you will see
both rendered and I will not. **Say which you chose and why.**

**Not in scope**: a link in the message to the day it moved to. It is a good
idea and it is a second decision; raise it if you think it belongs, do not
build it.

## §4 — gates

- **The behavioural check from the candidate handoff, re-run with step 6
  added**: after the Move, **the message is present on the page you land
  on and names the target day.** That is the whole point of this round.
- **A test.** The redirect carries the flash and the landing page renders it.
  Scope the assertion to the rendered message rather than to the URL alone —
  a `flash=` parameter that no page renders is the shape this handoff exists
  to fix, and a test on the parameter would pass against it. **Assert on what
  the user sees.**
- `DEC-007` three consecutive runs; the count will rise by whatever you add.
- **The overflow gate at five widths.** A flash is new content on the
  calendar pages, which the fixture sweeps at **120 cells** — and a message
  containing a long date, on a 320 px phone, is exactly the shape `§10.25`
  and `LAYOUT-006` are about. **This is the most likely defect in this
  round.**
- `fmt`, `clippy -D warnings`, `rustdoc-links` with seven crates and the
  private-link count still **eight**.
- Both browser scripts, since components change.

## §5 — escalate rather than deciding

- **If the overflow gate moves off 120** with the message present.
- **If `flash` turns out to leak into a saved view default** — that is a
  bigger finding than this handoff and it is not yours to patch around.
- **If you think the redirect should go to the target day's window instead**
  of the origin. I ruled against it — the user keeps their place and the
  message tells them the result — but if seeing it rendered changes your
  view, say so.

## §6 — exit condition

A Move that says where the issue went, on the page the user lands on; the
redirect target unchanged; a test asserting the **rendered** message; the
overflow gate still 120; `DEC-007` three times at the new figure; and the
behavioural check re-run with the message confirmed.

**Then 0.45.0's candidate is re-cut** — the changelog gains one sentence,
which is mine. The architect's own `CAL-004` handoff is the reason this round
exists, and that is recorded in the review rather than left implicit.
