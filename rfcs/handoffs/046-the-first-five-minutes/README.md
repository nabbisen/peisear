# Handoffs — the first five minutes

**0.53.0.** `RFC 0017`. One subject: **someone who does not have Rust,
and does not know what an environment variable is, gets a board on
screen.** Five barriers, counted in the RFC; four rounds and a
candidate here.

**The RFC is still in `rfcs/proposed/`.** Its security review is done
and recorded in the RFC itself (2026-10-11, §4 and §5 both approved as
proposed); what remains is the owner's acceptance, which moves it to
`rfcs/accepted/`. Per `RFC 000`, a handoff's status is inherited from
its RFC, so **these four are proposed companions until that happens** —
written now so nothing waits on paperwork, not started before it.

---

## Rounds

| ID | Handoff | Subject | Blocked on |
|---|---|---|---|
| FIV-001 | [the key that is published](./FIV-001-the-key-that-is-published.md) | **`RFC 0017` proposals 4 and §5, which land together or not at all.** `config.rs:31` is the literal that signs session cookies on any instance started without `JWT_SECRET`, published in every copy of the source; `BIND_ADDR` defaults to every interface. **`NFR-SEC-004` is `Divergent` at P0 because of the first (`§10.40`)**, and the second is what makes it reachable from the LAN. Generate, persist beside the database, fail loudly, delete the literal; default the bind to loopback | — |
| FIV-002 | [something to download](./FIV-002-something-to-download.md) | **Proposal 1.** Five targets, built in CI, attached to the Release **after it exists and never gating it**. Closes `DOCS-003`'s finding from the other end — the Quick Start's first command opens a tarball no release provides. **`NFR-REL-002` has read `Implemented` while no release has ever carried an archive**; this is what makes it checkable | FIV-001 |
| FIV-003 | [the container image](./FIV-003-the-container-image.md) | **Proposal 2.** `ghcr.io/nabbisen/peisear` from a `Dockerfile` in the repository, same tag, one volume, one port — **and the image sets `BIND_ADDR=0.0.0.0:3000` itself**, which is the half of FIV-001's bind change that makes the container correct | FIV-001 |
| FIV-004 | [the page a newcomer lands on](./FIV-004-the-page-a-newcomer-lands-on.md) | **Proposals 3 and 5.** `installation.md` splits into **Install** (download, `docker run`, or `cargo install` for people who already have Rust) and **Build from source** (today's 48 lines, moved intact); and one line of output on a successful bind naming the URL to open, plus a warning when the bind is not loopback | FIV-001, FIV-002, FIV-003 |
| REL-0.53.0 | the release candidate | The cut, `DEC-028`, and the first release whose own page carries artefacts. Written when the four rounds close | all four |

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

---

## What this series is not

**Not a packaging round.** Three of the four rounds are a newcomer's
first five minutes; the fourth is a security defect that the other
three would otherwise make worse. The release notes say both, and lead
with the security change, because that is the one a current operator
needs to read.

**Not a place to broaden the install story.** An installer script
(`curl … | sh`) is declined in the RFC's own alternatives, and so is
container-only. If a round finds a fourth channel tempting, that is a
new RFC, not a scope change.

---

## Decisions already taken, so no round re-opens them

- **`DEC-056` stays four legs.** The asset workflow runs *after* the
  Release exists and never gates it (`RFC 0017` proposal 1). A failed
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
