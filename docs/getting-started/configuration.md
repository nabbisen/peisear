# Configuration

peisear reads all configuration from environment variables. Nothing
else. A `.env` file at the workspace root is loaded on startup as a
convenience for development and for simple deployments; production
setups are free to inject the same variables via a process supervisor
(systemd `EnvironmentFile`, Kubernetes `env:`, etc.).

## Minimum viable `.env`

```bash
cp .env.example .env
echo "JWT_SECRET=$(openssl rand -base64 48)" >> .env
```

That's it — the defaults for everything else will get you to a
working local instance.

## Full reference

| Variable | Default | Notes |
|---|---|---|
| `DATABASE_URL` | `sqlite://data/app.db` | SQLite connection string. The parent directory is auto-created on startup. Use `sqlite://:memory:` for ephemeral testing. |
| `JWT_SECRET` | dev-only default (with a warning) | HMAC secret for session tokens. **MUST** be a long random string in production. 48 bytes of base64 is a good minimum. |
| `BIND_ADDR` | `0.0.0.0:3000` | Interface and port to bind. Use `127.0.0.1:3000` for localhost-only; put a real TLS reverse proxy in front for public deployments. |
| `COOKIE_SECURE` | `0` | Set to `1` when serving over HTTPS so the session cookie is flagged `Secure`. |
| `RUST_LOG` | `info,sqlx=warn,hyper=warn,tower_http=info` | Any `tracing_subscriber` env-filter expression. Use `debug` or `trace` to see per-request details. |

## Operational notes

- **Changing `JWT_SECRET`** invalidates every existing session in one
  step. That's usually what you want for a compromise recovery.
- **`DATABASE_URL` must be reachable at startup.** Migrations run
  immediately and block the listener until complete.
- **The dev `JWT_SECRET` fallback logs a warning.** Grep your logs for
  "JWT_SECRET is not set" before shipping anything past staging.

## Next

[First run](first-run.md) takes you from configured `.env` to a
browser window.
