# FIV-003 — the container image

**Target**: 0.53.0, after `FIV-001`. `RFC 017` proposal 2.

**The only path with no prerequisite at all** beyond a container
runtime, and one build rather than five. There is no `Dockerfile`
anywhere in the tree today.

---

## §1 — what ships

`ghcr.io/nabbisen/peisear`, built from a `Dockerfile` in the
repository, published on the same tag. One volume, one port:

```
docker run -p 3000:3000 -v ./data:/data ghcr.io/nabbisen/peisear
```

**The image sets `BIND_ADDR=0.0.0.0:3000` itself.** This is the other
half of `FIV-001`'s change and the reason this round waits for it:
inside a container, binding every interface is correct and expected,
and the container is the one context where the open bind is safe. A
loopback default inside a container is an instance nobody can reach.

**`-v ./data:/data` and `FIV-001`'s key file are the same decision seen
twice.** The database and the key that validates its sessions live in
that volume together. If they do not, a user who recreates the
container is logged out and will not know why. **Verify this directly**
— start the container, sign in, `docker rm` it, start a new one on the
same volume, and confirm the session still works.

## §2 — the same rule as FIV-002

**The image build never gates the Release.** Same reasoning, same
recovery: a failed build leaves a tag with no image and is fixed by
re-running one job. **`DEC-056` stays four legs.**

## §3 — what the image must not be

- **Not a build environment.** A multi-stage build whose final stage
  carries a Rust toolchain is a large image for no reason; the final
  stage carries the binary and `static/`. **Unless `FIV-005` embedded
  `static/` into the binary, which runs first and is the expected
  outcome** *(added 2026-10-15)* — then the stage carries the binary
  alone and this line is one fewer `COPY`. **Check what that round
  actually did**; do not copy a directory the binary no longer reads.
- **Not root, if it can be helped.** If the image runs as a non-root
  user, say which and confirm the volume is writable by it — a
  first-run key file that cannot be written is `FIV-001`'s hard
  failure, correctly, and it would be a poor first five minutes.
  **If it must run as root, say why.**
- **Not a second source of configuration truth.** Environment defaults
  baked into the image are defaults a reader cannot see in
  `configuration.md`. `BIND_ADDR` is the one this round sets; if a
  second becomes tempting, report it instead.

## §4 — specification work (`DEC-028`)

- **`NFR-CMP-002`** — self-hostable, not tied to a cloud provider,
  *"verified without network egress"* (`ASSET-001`, 0.32.0). A
  container image is a second hosting shape; check whether the entry's
  own verification claim still covers it, and say which part of it the
  image was and was not tested against.
- **A published image may want its own entry**, as `FIV-002`'s
  archives may. Same instruction: **report, do not invent**. One
  decision about what this document names as a release obligation,
  taken once at the candidate, is better than two rounds each adding
  a row.

## §5 — gates

- `DEC-007` from a fresh block, against `FIV-002`'s baseline. **No new
  tests expected.**
- `fmt`, `clippy -D warnings`, `mdbook build` with linkcheck.
- **The image, run**: `docker run` as documented, reach
  `http://localhost:3000`, register, create a project.
- **The volume test in §1**: container destroyed and recreated, same
  volume, session survives.
- **The image's size, stated.** Not a gate with a threshold — a number
  in the report, so the next person has a baseline to compare against.

## §6 — escalate rather than deciding

- **If the binary will not run in a slim base image** for a missing
  shared library. Report what is missing; the choice between a larger
  base and a static build is the architect's.
- **If publishing to `ghcr.io` needs a permission or a token the
  repository does not have.** That is the owner's, not a thing to
  work around.

## §7 — exit condition

A `Dockerfile` in the repository; an image published on the tag by a
workflow that cannot fail a release; `BIND_ADDR=0.0.0.0:3000` set in
the image and nothing else; the documented `docker run` line executed
as written and a project created through it; the volume test passing;
the image size and the user it runs as both stated; `NFR-CMP-002`
checked against the new shape and `DEC-007` unchanged and accounted
for.
