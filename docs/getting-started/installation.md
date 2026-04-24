# Installation

peisear needs a Rust toolchain with Edition 2024 support — that is,
`rustc` **1.85 or later**. Everything else (sqlite, daisyUI, Tailwind)
is either vendored as a Cargo dependency or loaded from a CDN on first
page load, so no separate package installation is required.

## Option A: rustup (recommended)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

This is the easiest path on any platform and is the only path that
gives you `rustup` itself — needed later if you ever want to add the
`wasm32-unknown-unknown` target for [Leptos hydration](../guides/hydration-upgrade.md).

## Option B: Debian / Ubuntu apt packages

If you prefer the distro's packaged toolchain on Ubuntu 24.04 or
Debian trixie, a recent enough `rustc` is available as `rustc-1.91`:

```bash
sudo apt install rustc-1.91 cargo-1.91
sudo ln -sf /usr/bin/rustc-1.91  /usr/local/bin/rustc
sudo ln -sf /usr/bin/cargo-1.91  /usr/local/bin/cargo
```

This configuration does not include any cross-compilation targets;
the SSR build doesn't need any, but hydration work later does.

## Verify

```bash
rustc --version
# rustc 1.85.x ...   (or newer)

cargo --version
# cargo 1.85.x ...   (or newer)
```

## Next

Head to [Configuration](configuration.md) to set up the `.env` file,
then [First run](first-run.md) to launch the server.
