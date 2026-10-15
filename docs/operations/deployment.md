# Deployment

peisear is designed to deploy as a single binary. This document covers
the mechanics of putting that into production.

## Build a release binary

From the workspace root:

```bash
cargo build --release
```

The resulting binary is `target/release/peisear`. It is fully static
with respect to peisear's own code — templates are compiled in as
Leptos components, migrations are baked in via `sqlx::migrate!()`,
and `static/`'s own assets are embedded too (`FIV-005`) — and `ldd`
against it shows exactly three dynamic dependencies: `libc`,
`libgcc_s`, `libm` (`FIV-003`'s own measurement, carried here rather
than left to drift from the claim this line used to make — no
`libssl`, since this project's `sqlx` uses `rustls`, not native TLS;
no `libsqlite3`, bundled and statically linked by default). For fully
static binaries anyway, see "Building static" below.

Alternatively, install the published binary directly from crates.io:

```bash
cargo install peisear
```

This compiles and installs the `peisear` binary into `~/.cargo/bin/`
and is suitable for development hosts and one-off deployments. For
production you generally still want a release tarball checked into
your config-management system.

## What to ship

```
/var/lib/peisear/
└── peisear              # The release binary -- static/ is embedded in it
    app.css
    board.js
    calendar.js
    daisyui.min.css
    dm.js
    plan.js
    search.js
    tailwind.css
```

**These eight files are embedded into the binary at compile time**
(`FIV-005`) and served from memory, not read from a `static/`
directory beside it at request time — nothing under `peisear` above is
a real path; it is what the binary carries, for reference. The list
itself is **derived from what the rendered HTML actually references**
— the `<link>`/`<script src>` tags in `peisear-web`'s own components
(`components/layout.rs` for the three stylesheets and `search.js`,
loaded on every page; `components/{calendar,sprint_plan,issues}.rs`
for the remaining four scripts, loaded only on the pages that use
them) — not from listing `static/`'s contents directly. All eight are
referenced by something; none is a dead asset. It is checked against
`static/`'s real contents in both directions (`docs_scan` rule 3) — a
drift here now means the binary was built from a `static/` this list
no longer describes, not a missing copy step.

The SQLite database (`data/app.db` by default) is created on first
run.

## systemd unit

```ini
[Unit]
Description=peisear
After=network.target

[Service]
Type=exec
WorkingDirectory=/var/lib/peisear
ExecStart=/var/lib/peisear/peisear
EnvironmentFile=/etc/peisear.env
Restart=on-failure
RestartSec=5s
User=peisear
Group=peisear

# Hardening
NoNewPrivileges=true
ProtectSystem=strict
ReadWritePaths=/var/lib/peisear
ProtectHome=true
PrivateTmp=true
PrivateDevices=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictNamespaces=true
RestrictRealtime=true
LockPersonality=true
MemoryDenyWriteExecute=true
CapabilityBoundingSet=

[Install]
WantedBy=multi-user.target
```

Save as `/etc/systemd/system/peisear.service`, then:

```bash
useradd --system --home /var/lib/peisear --shell /usr/sbin/nologin peisear
chown -R peisear:peisear /var/lib/peisear
systemctl daemon-reload
systemctl enable --now peisear
```

## The working-directory rule

**`DATABASE_URL`'s default, `sqlite://data/app.db`, resolves relative
to `$PWD`** — and so does the generated JWT signing key file beside it
(`FIV-001`). The `WorkingDirectory=` directive in the systemd unit
above is the mechanism that makes this resolve to
`/var/lib/peisear/data/` at runtime.

If you run the binary manually and `cd` somewhere else first, it opens
or creates a *different* database and key file at `./data/` under
whatever directory you happened to be standing in — no error, no
warning, just a second, empty instance. This is by design — but it's
the one deploy-time gotcha worth remembering. **It used to apply to
`static/` too**; `FIV-005` embedded that directory into the binary, so
a `cd` before starting it no longer produces a 404 for every asset,
only (if anything) a second database.

**The same principle governs the container image** (`FIV-003`): its
`WORKDIR /` puts the mount point itself where `DATABASE_URL`'s
relative default resolves, so `-v ./data:/data` holds `app.db` and
`jwt_secret` directly, not nested a level down. There is no
`WorkingDirectory=`-equivalent to misconfigure inside the image —
`WORKDIR` is fixed at build time — but the same relative-path default
is what both deployment shapes are resolving.

## Environment file

The companion `/etc/peisear.env` that the systemd unit loads should
contain the same variables as a local `.env`:

```bash
DATABASE_URL=sqlite:///var/lib/peisear/data/app.db
JWT_SECRET=<48-byte base64 string from `openssl rand -base64 48`>
BIND_ADDR=127.0.0.1:3000
COOKIE_SECURE=1
RUST_LOG=info,sqlx=warn
```

`BIND_ADDR=127.0.0.1:3000` keeps the server behind a reverse proxy.
Set `COOKIE_SECURE=1` once the proxy is terminating TLS.

## Reverse proxy sketch (nginx)

```nginx
server {
    listen 443 ssl http2;
    server_name peisear.example.com;

    ssl_certificate     /etc/letsencrypt/live/peisear.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/peisear.example.com/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host              $host;
        proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Health checks

`GET /health` returns `200 ok` with no auth. Use it for load-balancer
health, Kubernetes readiness, and uptime monitoring.

## Building static

For a fully static binary:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

The result at
`target/x86_64-unknown-linux-musl/release/peisear` can be copied to a
scratch or distroless container.

## Next

- [Backup](backup.md) — before you need it
- [Tailwind self-hosting](tailwind-local.md) — removing the one
  remaining CDN dependency
- [../security/hardening.md](../security/hardening.md) — what peisear
  does for you and what the operator needs to do
