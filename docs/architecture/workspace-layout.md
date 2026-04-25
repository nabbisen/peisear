# Workspace Layout

peisear is a Cargo workspace containing four published crates plus
static assets and an `infra/` placeholder. The tree:

```
peisear/
├── Cargo.toml                       # [workspace] + [workspace.dependencies]
├── Cargo.lock
├── README.md · LICENSE · NOTICE
├── CHANGELOG.md · ROADMAP.md · TERMS_OF_USE.md
├── .env.example · .gitignore
│
├── .github/                         # Community health files
│   ├── SECURITY.md
│   └── CONTRIBUTING.md
│
├── docs/                            # You are here
│   ├── README.md
│   ├── getting-started/
│   ├── architecture/
│   ├── operations/
│   ├── security/
│   └── guides/
│
├── crates/
│   ├── peisear-core/                # Pure domain types.
│   │   └── src/lib.rs               #   User, Project, Issue, IssueStatus,
│   │                                #   Priority, CurrentUser. Deps: serde,
│   │                                #   chrono, thiserror only.
│   │
│   ├── peisear-auth/                # Password hashing + JWT. No axum.
│   │   └── src/{lib,jwt,password}.rs
│   │                                #   AuthError, argon2id hash/verify,
│   │                                #   JWT issue/verify.
│   │
│   ├── peisear-storage/             # Persistence.
│   │   ├── migrations/0001_initial.sql
│   │   └── src/{lib,pool,users,projects,issues}.rs
│   │                                #   StorageError, Pool alias
│   │                                #   (SqlitePool today), per-table
│   │                                #   query modules. sqlx + core only.
│   │
│   ├── peisear-web/                 # HTTP surface (library only).
│   │   └── src/
│   │       ├── lib.rs               # Public API: build_router, AppState, Config.
│   │       ├── app.rs               # build_router(AppState) -> Router
│   │       ├── state.rs             # AppState { db, jwt_secret, cookie_secure }
│   │       ├── config.rs            # Environment loader.
│   │       ├── error.rs             # AppError + IntoResponse; From<StorageError>,
│   │       │                        # From<AuthError> bridges.
│   │       ├── extractors.rs        # AuthUser, MaybeAuthUser, AUTH_COOKIE.
│   │       ├── handlers.rs          # Shared validation-error formatter.
│   │       ├── handlers/
│   │       │   ├── auth.rs          # Register / login / logout.
│   │       │   ├── issues.rs        # Issue CRUD + JSON status change.
│   │       │   ├── projects.rs      # Project CRUD.
│   │       │   └── root.rs          # Index redirect, /health.
│   │       ├── components.rs        # Shared render helper + Column DTO.
│   │       └── components/
│   │           ├── layout.rs        # <Base>, <AppShell>, <PublicShell>
│   │           ├── auth.rs          # <LoginPage>, <RegisterPage>
│   │           ├── projects.rs      # <ProjectsListPage>, <ProjectNewPage>,
│   │           │                    # <ProjectEditPage>
│   │           ├── issues.rs        # <ProjectDetailPage> (board+list),
│   │           │                    # <IssueNewPage>, <IssueDetailPage>
│   │           └── error_page.rs    # <ErrorPage> for non-auth errors
│   │
│   └── peisear/                     # Facade crate.
│       ├── Cargo.toml               #   [[bin]] name = "peisear"
│       └── src/
│           ├── lib.rs               # Re-exports peisear_core/auth/storage/web
│           │                        # as peisear::{core, auth, storage, web}.
│           └── main.rs              # Server bootstrap; the binary that
│                                    # `cargo install peisear` ships.
│
├── static/
│   ├── app.css                      # Supplemental CSS.
│   └── board.js                     # Kanban drag-and-drop.
│
└── infra/                           # Placeholder for Dockerfile,
    └── README.md                    # compose.yaml, Terraform, GitHub
                                     # Actions workflows.
```

## Module style

The source tree follows the Rust 2018+ module layout. A module `foo`
is declared in `src/foo.rs`, and any submodules live alongside it in
`src/foo/`. There are **no `mod.rs` files** — a glance at the file
names alone tells you the module graph.

## Asset resolution

- **Migrations** are embedded at compile time via
  `sqlx::migrate!("./migrations")` relative to the `peisear-storage`
  crate's `CARGO_MANIFEST_DIR`, so the binary ships with them baked in.
- **Static files** (`static/app.css`, `static/board.js`) are served at
  runtime by `tower_http::services::ServeDir::new("static")`, which
  resolves **relative to the server's working directory**. Deploy
  scripts and systemd units must `cd` to the directory containing
  `static/` before exec-ing the binary. See
  [../operations/deployment.md](../operations/deployment.md).

## Next

- [Crate boundaries](crate-boundaries.md) — why the tree is split this way
- [Leptos SSR](leptos-ssr.md) — why `src/components/` exists at all
