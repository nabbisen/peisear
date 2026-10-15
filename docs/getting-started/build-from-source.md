# Build from source

For contributors, and for the one audience `cargo install` (see
[Install](install.md)) doesn't serve: someone who wants to read or
change the code, not just run it.

peisear needs a Rust toolchain with Edition 2024 support — that is,
`rustc` **1.88 or later** (`Cargo.toml`'s `rust-version`; the
dependency tree has drifted past Edition 2024's own 1.85 floor, so
1.85 itself cannot build the workspace). Everything else — sqlite,
daisyUI, Tailwind — is vendored as a Cargo dependency or under
`static/`, so no separate package installation is required.

## Install `rustc` via `rustup`

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

This is the easiest path on any platform and is the only path that
gives you `rustup` itself — needed later if you ever want to add the
`wasm32-unknown-unknown` target for [Leptos hydration](../guides/hydration-upgrade.md).

**`rustup` is the one path this page recommends.** An earlier version
of this page offered a second: symlinking Debian/Ubuntu's `rustc-1.91`
package over `/usr/local/bin/rustc` and `/usr/local/bin/cargo`,
system-wide. That is a large, irreversible-feeling action — it shadows
the distro's own compiler for every other package that expects it — in
exchange for avoiding one `curl | sh`.

If you'd still rather use the distro package than install `rustup`,
Ubuntu 24.04 and Debian trixie carry a recent enough `rustc` as
`rustc-1.91` — no cross-compilation targets included, which the SSR
build this project uses doesn't need, but hydration work later does:

```bash
sudo apt install rustc-1.91 cargo-1.91
cargo-1.91 build --release -p peisear
```

Invoking `cargo-1.91` directly, by its versioned name, builds this one
project with it without touching what `rustc`/`cargo` resolve to
anywhere else on the machine — the scope the symlink removed.

## Verify (`rustup` path)

```bash
rustc --version
# rustc 1.88.x ...   (or newer)

cargo --version
# cargo 1.88.x ...   (or newer)
```

## Next

Head to [Configuration](configuration.md) to set up the `.env` file,
then [First run](first-run.md) to launch the server.
