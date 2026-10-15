# FIV-003 (RFC 017 proposal 2). Built from the repository root
# (`docker build -f infra/docker/Dockerfile .`), not from this
# directory -- the build needs the whole Cargo workspace, not just
# this one file.
#
# Not a build environment in the final image (FIV-003 §3): the builder
# stage carries the Rust toolchain, the final stage carries the
# binary alone. `static/` is embedded into the binary at compile time
# (FIV-005), so there is nothing else to COPY -- the "plus static/"
# this directory's own infra/README.md still says is stale as of this
# round; not fixed here, reported in the round's own README.

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
FROM gcr.io/distroless/cc-debian12:nonroot

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
# so the working directory IS the mount point, not a path baked in as
# a second configuration default. Reported, not silently special-cased:
# this means the volume ends up holding a `data/` subdirectory one
# level down (`/data/data/app.db`), not `app.db` directly at the mount
# root. See the round's own report for the alternative considered and
# declined.
WORKDIR /data
VOLUME /data

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/peisear"]
