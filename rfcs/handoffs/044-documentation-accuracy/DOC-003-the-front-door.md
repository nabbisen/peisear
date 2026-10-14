# DOC-003 — the front door

**`RFC 016` proposal 2**, accepted 2026-10-11 and deferred from 0.49.0 so
the copy got a round of its own rather than a corner of a release-closing
one. Plus three statements about the same subject that are wrong in three
different documents.

**Target**: 0.51.0. **`DOC-001` has landed**, so the documents this round
links to now name seven crates.

---

## §1 — the README says the product is something it is not

`README.md:3` — *"Issue tracking that fits in a single binary."*
`README.md:18` — *"a self-hostable issue management system."*
`README.md:26` — *"Most issue trackers either bloat … peisear stays in the
middle on purpose."*

`docs/specification/requirements.md §1.1`, normative and in this
repository:

> peisear is a self-hostable **project and issue management** service…
> The primary aim is **not** "getting tasks finished", but **delivering the
> observation and awareness needed to keep a project — and the people in it
> — sustainable**.

**And the README never mentions the half that makes it true.** Counted:
*health* 0, *sustain* 0, *burnout* 0, *privacy* 0, *today* 0, *sprint* 0,
*WIP* 0, *capacity* 0, *overload* 0, *notification* 0. All shipped, and
**two of them visible in the hero screenshot the README already carries**
— the board's HEALTH section and its WORKLOAD chip.

**`RFC 016 §5` holds draft copy for the hero, overview, *Why peisear?* and
features.** It is a draft, not a specification: **refine it against §1.7's
vocabulary table** — no *velocity*, no *performance*, no achievement
framing — and keep the owner's own adjectives where they still fit.
**The structure is fixed** by `NFR-REL-007`: hero, overview, why/when,
quick start, features or design notes, links.

**Three bullets stay as they are**: *You own your data / your uptime / your
roadmap*. They are the self-hosting argument and they are good. They are
simply not the positioning.

## §2 — the same claim, wrong in three places

**One subject — how hard a PostgreSQL backend would be — stated three
different ways, and the reader meets whichever they happen to open.**

| Where | Says | Measured |
|---|---|---|
| `README.md:77` | *"swapping SQLite for Postgres … is a localised change"* | Localised in **types**, not in contracts |
| `docs/operations/scaling.md:91-94` | *"no SQLite-specific tricks beyond a few `julianday` calls and `datetime('now', '-7 days')` modifier strings, both of which have direct PostgreSQL equivalents"* | **24** `julianday` uses, **10** `datetime('now'` modifiers — **and 11 triggers and 8 `RAISE(ABORT)` messages the sentence excludes**, which have no direct equivalent |
| `ROADMAP.md:582` | *"larger than the crate-shaping above suggests, and needs its own RFC before it is scheduled or promised"* | **Correct** |

**`RFC 018` is now that RFC and it is accepted**, so the roadmap's
sentence needs updating too — it says the work needs an RFC that now
exists.

**Write one assessment and let the other places point at it.**
`RFC 019 §3`'s rule applies: stated once, in the document that governs it.
**`ROADMAP.md`'s section is the natural home** — it already has the fullest
version. `README.md` and `scaling.md` then carry a sentence each, not an
argument each.

**And `scaling.md`'s cross-reference is wrong**: it points at
`ROADMAP.md`'s *"Pluggable backends beyond relational"* (line 792) when it
means *"PostgreSQL backend"* (line 569). A different subject.

## §3 — `first-run.md`'s log target

The sample log line names the tracing target **`peisear_web`**; the code's
own target is **`peisear`** (`main.rs`'s `tracing::info!` and the startup
line `REL-0.50.0` captured by eye). **Found by `REL-0.50.0`'s pass 5 and
correctly left for this round.**

**Check the rest of the sample while it is open** — a log line that went
stale in one field rarely went stale in one field only. The startup output
changed in 0.50.0 (the DSN is redacted now); **confirm the sample matches
what the binary actually prints**, by running it rather than by reading it.

## §4 — what this round must not do

- **Do not touch `docs/specification/`.** The requirements are the source
  this copy is written *from*; if the README and §1.1 disagree after this
  round, the README is wrong.
- **Do not invent features.** Every claim added must name something that
  ships. If the draft copy says something you cannot point at, cut it and
  say which.
- **Do not add a feature list that duplicates the documentation.**
  `NFR-REL-007` allows features to move out of the README entirely,
  leaving design notes; **prefer fewer, sharper bullets to a catalogue.**

## §5 — gates

- **`DEC-007`** should not move. **Rule 6 reads `README.md`'s raw URLs** —
  if the hero image moves or a new image is added, that rule covers it, and
  if you add an image it must exist. **Say whether the count moved and
  why.**
- **Rule 1 reads the root `README.md`** for the crate set. Keep seven
  crates named, or the rule will say so.
- **`mdbook build` with linkcheck**; `fmt`, `clippy -D warnings`,
  `rustdoc-links` (seven crates, eight private-link warnings, zero broken).
- **Read the rendered README** — on GitHub, not only as source. It is the
  one document whose rendering is its purpose.

## §6 — escalate rather than deciding

- **If the draft copy in `RFC 016 §5` claims something that does not
  ship.** The RFC is a draft and I wrote it in one sitting.
- **If `ROADMAP.md`'s PostgreSQL section cannot absorb §2's single
  assessment** without becoming the RFC over again.
- **If the owner's four adjectives (*Sophisticated. Solid. Really easy.
  Good UI.*) do not survive the rewrite.** They are the owner's own voice
  and are not mine to drop.

## §7 — exit condition

A README whose first three sentences describe the product the requirements
define, with the sustainability half named and every claim pointing at
something that ships; one PostgreSQL assessment, in `ROADMAP.md`, with
`README.md` and `scaling.md` carrying a sentence each and the
cross-reference fixed; `first-run.md`'s sample log line matching what the
binary prints, confirmed by running it; the rendered README read; gates
stated.
