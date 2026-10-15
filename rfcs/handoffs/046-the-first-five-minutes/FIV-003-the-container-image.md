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

---

# FIV-003 round 2 — the nesting had a third option

Round 1 is accepted (see the review): the image is right, the
distroless choice is argued from a real `ldd`, and the volume test is
the discriminating one rather than "it starts again".

## §6 — `WORKDIR /` removes the nested `data/` with no second default

Round 1 reports `/data/data/app.db` as *"a fact about the chosen
shape, not fixed — fixing it would mean baking in `DATABASE_URL`,
which is the thing declined above."*

**That is a choice between two options when there are three.** The
nesting comes from `WORKDIR /data` meeting a relative default of
`sqlite://data/app.db`. Move the working directory up one:

```dockerfile
WORKDIR /
VOLUME /data
```

The default is unchanged, nothing is baked in, no second environment
variable appears — and `data/app.db` relative to `/` **is**
`/data/app.db`, the mount root, with `jwt_secret` beside it. A host
`-v ./data:/data` then holds `app.db` directly, which is what
`docs/operations/backup.md` describes as the backup unit.

**Check it rather than taking it** — run the volume test again and
report the actual paths inside the volume, because this is a claim
about how two relative paths compose and it deserves the same
treatment the `:nonroot` attempt got.

**The reasoning that declined baking `DATABASE_URL` was right.** This
does not reverse it; it reaches the same goal without paying that
price. **Say in the comment why `WORKDIR` is `/` and not `/data`**, or
the next reader will "tidy" it back.

## §7 — the root decision stands, with its third option named too

Running as root is accepted, and the measurement behind it is the kind
this project wants: `:nonroot` tried, failed on bind-mount ownership,
reported rather than worked around.

**Recorded for completeness, not as a change to make**: a *named*
volume (`-v peisear-data:/data`) takes its ownership from the image,
so `:nonroot` would work with one — the bind mount is what forbids it.
Round 1's reasoning for preferring a bind mount is sound and
unstated: host-visible data is what makes this project's own backup
story (`cp` the directory) true. **State that preference where the
decision is made**, so the next person sees three options and a reason
rather than two options and a constraint.

## §8 — a comment that describes a thing the round then fixed

The Dockerfile says `infra/README.md`'s *"plus `static/`"* is *"stale
as of this round; not fixed here, reported in the round's own
README."* **It was fixed** — `infra/README.md:8` now reads *"Not 'plus
`static/`' as this file used to say."* Good fix, stale comment. One
line.

## §9 — exit condition

`WORKDIR /` with the volume test re-run and the **actual paths inside
the volume** reported; the bind-mount-over-named-volume preference
stated where the root decision is made; `§8`'s comment corrected;
`DEC-007` unchanged at **486** and confirmed unchanged rather than
assumed; all four gates re-run after the round's last edit.
