# Handoffs — the first five minutes

**0.53.0.** `RFC 017`. One subject: **someone who does not have Rust,
and does not know what an environment variable is, gets a board on
screen.** Five barriers counted in the RFC, plus two found afterwards
by walking the path rather than reading it; six rounds and a candidate
here.

**`RFC 017` is accepted** (owner, 2026-10-14) and lives in
`rfcs/accepted/`. Its security review is recorded in the RFC itself
(2026-10-11, §4 and §5 both approved as proposed). Per `RFC 000` these
rounds inherit that status, so nothing here is waiting on paperwork.

**Two rounds are not in the RFC.** `FIV-005` and `FIV-006` were found
on 2026-10-14, after the series was scoped, and they are in it because
both are about the same five minutes — not because the scope grew to
fit them. `FIV-005` is a defect in the install path the RFC assumed
worked; `FIV-006` is an owner request. **Neither licenses a third
addition**: see *What this series is not*, below.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
| FIV-001 | [the key that is published](./FIV-001-the-key-that-is-published.md) | **`RFC 017` proposals 4 and §5, which land together or not at all.** `config.rs:31` is the literal that signs session cookies on any instance started without `JWT_SECRET`, published in every copy of the source; `BIND_ADDR` defaults to every interface. **`NFR-SEC-004` is `Divergent` at P0 because of the first (`§10.40`)**, and the second is what makes it reachable from the LAN. Generate, persist beside the database, fail loudly, delete the literal; default the bind to loopback | — | **Closed** — 3 rounds |
| FIV-002 | [something to download](./FIV-002-something-to-download.md) | **Proposal 1.** Five targets, built in CI, attached to the Release **after it exists and never gating it**. Closes `DOCS-003`'s finding from the other end — the Quick Start's first command opens a tarball no release provides. **`NFR-REL-002` has read `Implemented` while no release has ever carried an archive**; this is what makes it checkable | FIV-001, **FIV-005** | **Closed** — 2 rounds |
| FIV-003 | [the container image](./FIV-003-the-container-image.md) | **Proposal 2.** `ghcr.io/nabbisen/peisear` from a `Dockerfile` in the repository, same tag, one volume, one port — **and the image sets `BIND_ADDR=0.0.0.0:3000` itself**, which is the half of FIV-001's bind change that makes the container correct | FIV-001, **FIV-005** | Round 2 open |
| FIV-004 | [the page a newcomer lands on](./FIV-004-the-page-a-newcomer-lands-on.md) | **Proposals 3 and 5.** `installation.md` splits into **Install** (download, `docker run`, or `cargo install` for people who already have Rust) and **Build from source** (today's 48 lines, moved intact); and one line of output on a successful bind naming the URL to open, plus a warning when the bind is not loopback | FIV-001, FIV-002, FIV-003 | **Next after FIV-003** |
| FIV-005 | [the binary that is not one](./FIV-005-the-binary-that-is-not-one.md) | **Not in the RFC; `§10.41`, found 2026-10-14.** `README.md`'s first instruction is `cargo install peisear`, and **`static/` is in no published crate** — it sits outside every package directory and no `Cargo.toml` includes it — while `app.rs:259` serves it relative to the working directory. Eight assets, eight 404s: the primary documented path installs an unstyled application. Embed `static/` in the binary, which is what makes the README's own *"One binary deploy"* true | — ; **blocks FIV-002 and FIV-003** | **Closed** — 3 rounds |
| FIV-006 | [two pictures for a newcomer](./FIV-006-two-pictures-for-a-newcomer.md) | **Owner request, 2026-10-14**, with the privacy floor restated as its condition. `/today` has never been captured though `DOCS-004` named it in 0.49.0, and it is the screen the README sells hardest and shows least. One new capture, one reused image, two in `README.md` — and the health `<details>` expanded, because today's board image shows an empty `Indicators` label directly beneath the README's six-indicators claim | — | Round 2 open |
| REL-0.53.0 | the release candidate | The cut, `DEC-028`, and the first release whose own page carries artefacts. Written when the six rounds close | all six | Blocked |

**`FIV-001` is first and the order is not a preference.** The RFC says
it: *"Making startup easier makes this worse, not better. A one-command
start that skips configuration is a one-command start into that
state."* Shipping a download or an image before the key is generated
would hand a published signing key to exactly the audience this
release exists to serve, at the moment they are least able to notice.
**If `FIV-001` stalls, the release stalls.** It does not ship partially.

**`FIV-004` is last** because it documents what the other three
produce. An Install page written before the download URLs and the
`docker run` line are real is a page that describes an intention.

**Status lives in the table above, not in a message.** It is updated
when a round closes, so the next person to pick one up reads it here
rather than asking. *(Column added 2026-10-15, after the dev team
correctly stopped to ask which round was next — the index stated an
order and never stated where the order had got to.)*

**`FIV-005` is next, and the sentence that used to be here was wrong.**
It said *"`FIV-005` and `FIV-006` are blocked on nothing and block
nothing."* **`FIV-005` blocks two rounds**, found when the dev team
stopped to ask which to start (2026-10-15) rather than guessing:

- **`FIV-002` §4 is titled *what the archive contains*** and asks
  whether `static/` belongs in it, against `deployment.md`'s manifest.
- **`FIV-003` says its *"stage carries the binary and `static/`"***.

**`FIV-005` embeds `static/` into the binary, so both of those answers
change** — the archive becomes a binary and its documentation, and the
image's copy step disappears. Building either first means building it
around a layout `FIV-005` then invalidates, and `NFR-REL-002`'s own
archive layout would be amended twice in one release.

`FIV-005`'s own §4 saw half of this and recorded it as *"coordinate
rather than both editing it"* — a note where an ordering was needed.
Coordination between two rounds that are not running at the same time
is just an order nobody wrote down.

**So: `FIV-005` → `FIV-002` → `FIV-003` → `FIV-004`.** `FIV-004` is
last for its own reason, below, and `FIV-005` preceding it was already
true for a second reason: the Install page's third path is
`cargo install peisear`, which produces an unstyled application until
`FIV-005` lands.

**`FIV-006` is the one round that really is independent.** It touches
the capture harness and `README.md`'s images — not the key, the bind,
the artefacts, or the asset layout. **Run it whenever there is
capacity, including alongside any of the above.**

---

## What this series is not

**Not a packaging round.** Four of the six rounds are a newcomer's
first five minutes; one is a security defect that the others would
otherwise make worse, and one is two pictures. The release notes say both, and lead
with the security change, because that is the one a current operator
needs to read.

**Not a place to broaden the install story.** An installer script
(`curl … | sh`) is declined in the RFC's own alternatives, and so is
container-only. If a round finds a fourth channel tempting, that is a
new RFC, not a scope change.

**Not open for a seventh round.** Two were added on 2026-10-14 and the
reason is on the record above. A release that absorbs whatever is found
while walking it never cuts — **0.51.0 is the precedent and it cost
every row in `ROADMAP.md` one release.** Anything further found here
goes to me and waits for its own release, including the carried
`docs_scan` rule for `RFC 0NNN` citations, which is deliberately not in
0.53.0 for exactly this reason.

---

## How `FIV-001`'s open half closes, since no round does it

*Added 2026-10-15, answering the dev team's own question.* `§10.40` and
`NFR-SEC-004` wait on `.env.example`'s `JWT_SECRET` line, which no tool
in this project may read.

**Release cycle exit criterion 9 replaces the per-round walk; it does
not supplement it.** `FIV-001` §9b already removed the walk from the
rounds — test the behaviour with the value set directly, report the
file's state as a conditional, do not route around the deny rule. What
was left over was a question being carried in conversation, which is
where facts go stale. Criterion 9 puts it in the gate: **the documented
start is walked on the release artefact, and it either works or it does
not.**

**So no round in this series closes `§10.40`**, and none should try.
`REL-0.53.0` reports what criterion 9's walk showed, and the entry and
the requirement move then — on a walk, not on an answer from anyone.

---

## Decisions already taken, so no round re-opens them

- **`DEC-056` stays four legs.** The asset workflow runs *after* the
  Release exists and never gates it (`RFC 017` proposal 1). A failed
  matrix leaves a Release with no assets — today's state — and is
  fixed by re-running one job, not by re-cutting a release.
- **The generated key lives in a file beside the database, not in the
  database.** Considered and declined: `main.rs:22` builds `Config`
  before `pool::connect`, so a key read from the database would make
  `Config::from_env()` depend on a pool and stop being total. The file
  keeps the change inside `config.rs`.
- **The bind default is `127.0.0.1:3000`**, with the container image
  setting `0.0.0.0:3000` explicitly. Already the project's own
  documented production posture — `deployment.md`'s systemd example
  sets loopback today — so a deployment following the existing guide
  is unaffected.
- **The `peisear-i18n`/`peisear-core` guard-location question is
  closed** (declined, `NFR-MNT-005`, 0.52.0). Not this series' concern.
