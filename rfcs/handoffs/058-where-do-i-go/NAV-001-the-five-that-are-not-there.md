# NAV-001 — the five that are not there

**`FR-NAV-001` is a P1 that reads *"The primary navigation MUST expose
exactly five entry points: Today, Inbox, Projects, Teams, Search"*, its
status is `Implemented`, and its acceptance is the sentence *"navigation
renders five entries"* — a restatement this document already records as
**uncheckable by anyone, including its author.** So nothing has ever
verified the five.**

**What `components/layout.rs`'s `Navbar` actually exposes**, read from
source 2026-11-04:

| required entry | how it is exposed |
|---|---|
| **Today** | inside the `dropdown-content` only |
| **Inbox** | a bell: an 18 px SVG with an `aria-label` and **no visible word** — plus a dropdown entry |
| **Projects** | **only via the brand link**, which is not named "Projects" |
| **Teams** | inside the `dropdown-content` only |
| **Search** | an input, `hidden sm:block` — **absent below the `sm` breakpoint** |
| *Settings* | in the dropdown — **and it is not one of the five** |

**So of five required entry points, none is visibly named in the bar at
every width**, one is an unlabelled glyph, one hides behind a logo, two
are behind a menu labelled with the reader's own username, and a sixth
entry is exposed that the requirement does not list.

## §0 — the decision, made by the architect; this round implements it

**"Expose" means visible and named, without an interaction.** That is the
reading this round builds, and the reason is the reader the owner named:
for someone whose acuity and short-term recall are both declining, a menu
costs a two-step interaction **plus** remembering that the menu is behind
their own name. **Hiding is not simplifying** — *less is more* means less
to decode, not fewer things on screen.

1. **The five become a visible, labelled row** in `AppShell`'s navbar:
   **Today, Inbox, Projects, Teams, Search** — as words.
2. **The bell keeps its badge and gains its word.** The unread count stays
   exactly as it is; the control is no longer glyph-only.
3. **Projects stops depending on the brand link.** The brand may still
   link wherever it links; Projects gets its own named entry, because a
   logo is a convention, not a label.
4. **Settings leaves the primary row.** It is not one of the five. It stays
   with Sign out in the user dropdown, where account-scoped actions belong.
5. **Below `sm`, the five collapse into one control labelled with a
   word** — *Menu*, or the equivalent `MessageKey` — **not** the username.
   The defect being fixed is *the reader cannot find where to go*; a word
   is findable and a username is not.

**Not this round**: changing what any of the five destinations *do*,
renaming routes (`FR-NAV-002`'s 308s stay), or touching `PublicShell`.
Nothing about the five pages' own contents — those are later units, one at
a time.

## §1 — the acceptance that cannot be checked, and the test that can

`FR-NAV-001`'s acceptance is a restatement, which is why this has gone
unverified. **This round writes the locatable replacement**: a
`peisear-web` test asserting the authenticated shell's navbar carries all
five entries **by their rendered `MessageKey` text**, and that Settings is
not among them.

**Render the keys, do not quote the words.** `REQ-005`'s own finding is
the reason: a test that hardcodes a string passes a literal-grep and
misses a rendered one, and the architect's grep for *"management role, not
an oversight role"* missed a test that renders the key. Assert against
`Locale::English.render(MessageKey::NavLinkToday)` and its siblings.

**Seen red on a plant**: remove one entry from the row, watch the test name
*which* one is missing, revert, green. **Do not report the proposed
`*Status*`/`*Acceptance*` wording as applied** — write it in the report;
the architect applies it (`CFG-001`'s division).

## §2 — the gates this round genuinely moves, which is more than usual

This changes the chrome of **every authenticated page**, so:

- **`touch_target_scan`** — `NFR-A11Y-007` holds 44 px (WCAG 2.2 **AAA**,
  kept deliberately). Every new link is a control; `min-h-11 min-w-11` is
  the existing idiom in this file. **Run the scan, do not assume it.**
- **`contrast_scan`** — new text in the navbar must not use the muted tier
  below the floor. Primary navigation is not secondary information:
  **full-strength `base-content`**, and no `text-xs`.
- **`browser-checks`** — 19 pages at 5 widths. The navbar is on all of
  them, and the `sm` collapse is exactly a width behaviour. **This is the
  gate most likely to find something**, and `§10.32`'s stopping rule does
  not forbid running it; it forbids adding fixture branches reactively.
  Report the sweep, including any overflow at the narrowest width.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`, `mdbook`,
  rustdoc baseline for `peisear-web` (**6**), and `DEC-007` from the
  runner — **this round adds tests, so the count moves; state from and to.**

## §3 — the trap

**A row of five labelled links is wider than a logo and a glyph.** The
narrowest width in the sweep is where this breaks, and the temptation is
to shrink the type or mute it to make it fit. **Both are forbidden here**
— they would reintroduce, in the primary navigation, exactly the
small-and-faint combination this whole line of work started from.

If the five do not fit at the narrowest width at full strength and
readable size, **the collapse threshold moves up** — that is the lever.
Report the threshold you chose and why.

## §4 — escalate rather than deciding

- If `FR-NAV-001`'s *Search* entry cannot be a labelled row item because
  it is an input rather than a link, **say so and propose the shape** —
  a labelled control that opens the search, or the input with a visible
  label. Do not silently drop it to four.
- If the collapse needs more than a width threshold — a different control,
  a second row — **name it and stop.** That is a design decision and it is
  the architect's.
- If `browser-checks` finds an overflow this change causes on a page whose
  own content is the cause, report it separately; it is not this round's
  to fix.

## §5 — exit conditions

1. Five named, visible entries in `AppShell`'s navbar at full width; the
   bell labelled with its badge intact; Projects independent of the brand;
   Settings out of the primary row.
2. A word-labelled collapse below the chosen threshold, never the username.
3. The locatable test, rendering keys rather than quoting strings, **seen
   red on a plant** naming the missing entry.
4. `§2`'s gates after the last edit, `browser-checks` reported including
   the narrowest width, `DEC-007` from and to.
5. Proposed `FR-NAV-001` *Acceptance* and *Status* wording written in the
   report for the architect, not applied.
