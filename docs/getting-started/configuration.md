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

**`PEISEAR_SETUP` controls whether that first run can ask at all.**
It is read from the launch environment only — never from `.env`,
since `.env`'s own existence already means *configured* — and takes
one of two values:

- `auto` (the default) — ask when a human can actually answer, start
  with defaults otherwise. A process with no terminal at all (a
  container, CI, a pipe) already starts silently either way; this
  only changes what happens when a terminal is attached but nobody
  is watching it — a job started with `&`, `nohup`, or an init
  script, which otherwise stops waiting for an answer that will
  never come.
- `never` — never ask, whatever the terminal says. Set this to make
  an unattended start deliberate rather than relying on the default
  to notice it for you — exactly what a systemd unit or a CI job
  should set, and what the published archives' own release walk uses
  on Windows, where no other signal exists.

An unrecognised value (anything other than the two above) is refused
at startup, naming the value and the two it accepts, rather than
silently treated as `auto`.

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

**On a `--features postgres` build** ([Install](install.md)'s own
"Running against PostgreSQL instead"), `DATABASE_URL` takes
PostgreSQL's own form instead of the default above —
`postgres://user:password@host:5432/dbname`, `postgresql://` accepted
too — and query parameters such as `?sslmode=require` pass through
unchanged, the normal shape against a managed provider. This is a
different build, not a second default the table above could name:
whichever driver the binary was compiled with is the only scheme it
accepts, and the other is refused by name at startup rather than
silently misread. `JWT_SECRET` must be set explicitly on this
backend — there is no filesystem path beside a connection string to
persist a generated one — so the wizard's own "created for you on
first run" above does not apply there. Proven against `postgres:17`,
one version, which is what this project's own CI actually runs
against.

## Email (SMTP)

**Optional.** peisear always has an in-app notification channel;
email is an additional one, read from seven environment variables at
startup. If `SMTP_HOST`, `SMTP_USER`, `SMTP_PASSWORD` or
`SMTP_FROM_ADDRESS` is unset, the email channel degrades quietly —
in-app delivery keeps working, and a later send attempt through email
fails at the channel layer rather than at startup.

| Variable | Required | Default | Notes |
|---|---|---|---|
| `SMTP_HOST` | for the email channel | — | SMTP server hostname, e.g. `smtp.example.com`. |
| `SMTP_USER` | for the email channel | — | SMTP AUTH username. |
| `SMTP_PASSWORD` | for the email channel | — | SMTP AUTH password. |
| `SMTP_FROM_ADDRESS` | for the email channel | — | The `From:` envelope address. |
| `SMTP_PORT` | optional | `465` | Submission port. `587` is the other common choice (STARTTLS); anything else is treated as implicit TLS. |
| `SMTP_TLS_MODE` | optional | derived from `SMTP_PORT` | `implicit` or `starttls` — see below. |
| `SMTP_FROM_NAME` | optional | unset | Display name for the `From:` header; the envelope address alone is used if unset. |

**`SMTP_TLS_MODE`'s two values, and when you need to set it at all.**
`implicit` connects with TLS before any SMTP traffic (the standard
"submissions" port, 465); `starttls` connects in plain text and
upgrades in place (the standard "submission" port, 587). Left unset,
the mode follows `SMTP_PORT`: `587` → `starttls`, everything else
(including `465` and any non-standard port) → `implicit` — set it
explicitly if you run STARTTLS on a non-standard port, since the
port-derived default cannot tell that case from implicit TLS on an
unusual port. **An unrecognised value is not refused at startup** —
unlike `PEISEAR_SETUP` above, it falls back to the port-derived
default silently, consistent with this channel's own
graceful-degradation design: a typo here costs you the TLS mode you
meant to force, not a stopped server.

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
