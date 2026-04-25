# First Run

With [Rust installed](installation.md) and a [`.env`](configuration.md)
in place, a working peisear instance is one command away.

## Launch

Run Cargo from the workspace root:

```bash
cargo run --release -p peisear
```

Or, because there's only one runnable binary in the workspace, the
shorthand works too:

```bash
cargo run --release
```

The first build compiles the full dependency graph (Leptos plus
axum plus sqlx is roughly 300 crates); expect ~10 minutes on a
cold cache. Subsequent runs are instant.

You should see:

```
2026-04-24T… INFO peisear_web: starting peisear database=sqlite://data/app.db addr=0.0.0.0:3000
2026-04-24T… INFO peisear_web: listening addr=0.0.0.0:3000
```

Open <http://localhost:3000>.

## First account

1. Click **Create one** on the login page to reach `/register`.
2. Fill in an email, display name, and a password of at least 8
   characters.
3. On submit you're redirected to `/projects` with an active session.

The SQLite database lives at `./data/app.db`. The parent directory is
auto-created on first run, and migrations execute before the server
starts accepting connections.

## First project, first issue

1. On `/projects` click **New project**. Name it, optionally describe
   it, submit.
2. You land on the project detail page in **Board** view. Click
   **New issue** to create one.
3. Drag issue cards between the *Open* / *In Progress* / *Done*
   columns to change their status. The drag-and-drop uses a JSON POST
   to `/projects/{id}/issues/{issue_id}/status`; the server persists
   and the page reloads to show the confirmed state.
4. Click **List** in the view toggle for a dense tabular view of every
   issue in the project.

## Health check

A trivial `/health` endpoint returns `200 ok` without authentication.
Use it for load balancers, uptime probes, and readiness gates:

```bash
curl -fsS http://localhost:3000/health
# ok
```

## Next

- [Architecture overview](../architecture/overview.md) — what you just ran
- [Deployment](../operations/deployment.md) — putting it on a real server
- [Security hardening](../security/hardening.md) — what's already protected
