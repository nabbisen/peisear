# FIV-003 (RFC 017 proposal 2). Built from the repository root
# (`docker build -f infra/docker/Dockerfile .`), not from this
# directory -- the build needs the whole Cargo workspace, not just
# this one file.
#
# Not a build environment in the final image (FIV-003 §3): the builder
# stage carries the Rust toolchain, the final stage carries the
# binary alone. `static/` is embedded into the binary at compile time
# (FIV-005), so there is nothing else to COPY -- `infra/README.md`'s
# own "plus static/" line was corrected to say so, in the same commit
# as this file (FIV-003 round 2 §8: an earlier draft of this comment
# said that correction was still pending after it had already landed).

# Same toolchain this project's own CI builds and tests against
# (rust-toolchain.toml pins 1.97.1), not merely "whatever rust:latest
# resolves to" -- the same reasoning FIV-002 round 2 applied to the
# release archives applies here.
FROM rust:1.97.1-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release -p peisear

# `ldd` on the built binary shows exactly three dynamic dependencies:
# libc, libgcc_s, libm -- no libssl (this workspace's sqlx uses
# runtime-tokio-rustls, not native-tls), no libsqlite3 (bundled and
# statically linked by libsqlite3-sys by default). `distroless/cc`
# carries precisely that set and nothing else: no shell, no package
# manager, no second way into the container once it is running.
# `-debian12` matches the builder's own Debian release, so the two
# images' glibc versions agree.
#
# **Runs as root, and here is why** (FIV-003 §3's own "if it must run
# as root, say why"): the `:nonroot` variant (uid 65532) was tried
# first and measured, not assumed -- `evidence/` in the round's own
# report. `-v ./data:/data` is a bind mount, so `/data`'s ownership on
# disk is the *host* directory's, which Docker never changes to match
# the image; a fresh `./data` a new user creates is owned by their own
# host account, uid 65532 cannot write the first-run database or
# `FIV-001`'s generated key file into it, and the app exits with
# exactly the hard failure that handoff calls correct -- just at the
# worst possible moment, the first command a newcomer runs. Requiring
# a manual `chown 65532 ./data` before the documented `docker run`
# line defeats what this round exists to remove: a prerequisite beyond
# "have a container runtime." Root inside this container is still
# unprivileged (no added Linux capabilities, no `--privileged`) --
# contained, not equivalent to host root -- and distroless/cc carries
# no shell for that root user to reach a second way in with regardless.
#
# **Why a bind mount at all, when a named volume would let `:nonroot`
# work** (FIV-003 round 2 §7): a named volume takes its ownership from
# the image on first use, so `:nonroot` would have been writable there
# with no chown and no root. Declined anyway: a named volume is
# Docker-managed storage, not a host path, and this project's own
# backup story -- `docs/operations/backup.md`'s `cp` of the database
# file while the process is stopped -- depends on the data living
# somewhere the host's own tools can reach directly. Trading that for
# `:nonroot` would fix the permission question by creating the one
# this project's documentation already answers for every other
# deployment shape.
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /build/target/release/peisear /usr/local/bin/peisear

# RFC 017 §5 / FIV-001: the other half of the loopback-default change.
# Binding every interface is correct and expected inside a container --
# the container boundary is what makes it safe, and a loopback default
# in here would be an instance nobody outside it can reach. The one
# environment default this image sets, per FIV-003 §3's own
# instruction not to let a second one become tempting without saying so.
ENV BIND_ADDR=0.0.0.0:3000

# The database and the generated JWT signing key (FIV-001) both land
# under the process's working directory by their own, unchanged
# relative defaults (`sqlite://data/app.db` and the key beside it) --
# so the working directory is what decides where they end up, not a
# path baked in as a second configuration default.
#
# **`WORKDIR /`, not `WORKDIR /data`** (FIV-003 round 2 §6: round 1
# used `/data` here, which nests the volume one level --
# `/data/data/app.db` -- round 1's own report named as a consequence
# rather than fixed, framed as a choice against baking in
# `DATABASE_URL`). There was a third option: `sqlite://data/app.db`
# resolved against `/` **is** `/data/app.db` -- the mount root, no
# default changed, nothing baked in. A host `-v ./data:/data` then
# holds `app.db` and `jwt_secret` directly, which is what
# `docs/operations/backup.md` already describes as the unit to copy.
WORKDIR /
VOLUME /data

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/peisear"]
