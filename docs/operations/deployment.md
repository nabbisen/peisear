# Deployment

peisear is designed to deploy as a single binary plus a directory of
static assets. This document covers the mechanics of putting that
into production.

## Build a release binary

From the workspace root:

```bash
cargo build --release
```

The resulting binary is `target/release/peisear`. It is fully static
with respect to peisear's own code — templates are compiled in as
Leptos components, migrations are baked in via `sqlx::migrate!()` —
but dynamically links against system `libc`, `libssl`, `libsqlite3`,
and so on. For fully static binaries, see "Building static" below.

## What to ship

```
/var/lib/peisear/
├── peisear              # The release binary
└── static/              # Required at runtime
    ├── app.css
    └── board.js
```

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

**`ServeDir::new("static")` resolves relative to `$PWD`.** The
`WorkingDirectory=` directive in the systemd unit above is the
mechanism that makes this resolve to `/var/lib/peisear/static/` at
runtime.

If you run the binary manually and `cd` somewhere else first, `GET
/static/app.css` will 404. This is by design — but it's the one
deploy-time gotcha worth remembering.

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
