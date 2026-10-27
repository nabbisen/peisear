# Configuration

peisear reads all configuration from environment variables. Nothing
else. A `.env` file at the workspace root is loaded on startup as a
convenience for development and for simple deployments; production
setups are free to inject the same variables via a process supervisor
(systemd `EnvironmentFile`, Kubernetes `env:`, etc.).

## You don't have to write this file yourself

**A first run with no `.env` and no configuration in the environment
asks two questions — where the database lives, and what address to
listen on — and writes `.env` for you**, holding enter for both keeps
the defaults. See [First run](first-run.md) for what that looks like.

This page is the reference for what each variable means and how to
change one afterward — someone writing a systemd unit, or deploying
without a terminal to answer a prompt, still needs the names below
without running an interactive program to find them.

If you'd rather write the file by hand, `.env.example` is still there
to copy:

```bash
cp .env.example .env
```

Either way gets you a working local instance. `JWT_SECRET` is created
for you on first run regardless of which path you took (see below).

## Full reference

| Variable | Default | Notes |
|---|---|---|
| `DATABASE_URL` | `sqlite://data/app.db` | SQLite connection string. The parent directory is auto-created on startup. Use `sqlite://:memory:` for ephemeral testing. |
| `JWT_SECRET` | generated on first run | HMAC secret for session tokens. If unset, 32 random bytes are generated and persisted in a file named `jwt_secret` beside the database (`RFC 017` §4); later runs reuse it. Set this variable explicitly to override — for a multi-instance deployment sharing one secret, say — and it always wins over the generated file. |
| `BIND_ADDR` | `127.0.0.1:3000` | Interface and port to bind. Loopback by default, so a first run never publishes the instance to the network. Use `0.0.0.0:3000` to accept connections from other machines (the container image sets this for you); put a real TLS reverse proxy in front for public deployments either way. |
| `COOKIE_SECURE` | `0` | Set to `1` when serving over HTTPS so the session cookie is flagged `Secure`. |
| `RUST_LOG` | `info,sqlx=warn,hyper=warn,tower_http=info` | Any `tracing_subscriber` env-filter expression. Use `debug` or `trace` to see per-request details. |

## Operational notes

- **The generated `jwt_secret` file must be backed up with the
  database.** It lives in the same directory and `DATABASE_URL`'s
  directory is now the backup unit — see
  [`../operations/backup.md`](../operations/backup.md).
- **Deleting the key file, or changing `JWT_SECRET`,** invalidates
  every existing session in one step. That's usually what you want for
  compromise recovery.
- **`DATABASE_URL` must be reachable at startup.** Migrations run
  immediately and block the listener until complete.
- **A damaged key file fails startup loudly**, naming the path, rather
  than silently generating a new one — a wrong-length `jwt_secret` is
  never treated as absent.
- **If you want to set `JWT_SECRET` yourself** (the multi-instance
  case above, or any reason to manage it outside the generated file),
  generate a real one rather than typing a placeholder:
  `openssl rand -base64 48` (or any CSPRNG you trust). **Startup
  refuses to run with `.env.example`'s own published placeholder
  value** — it is public in the repository, so an instance using it
  unmodified could have its sessions forged by anyone who read the
  source.

## Next

[First run](first-run.md) takes you from configured `.env` to a
browser window.
