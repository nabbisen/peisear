# Leptos SSR

Leptos is used here in **server-side rendering mode only** — no
client-side reactivity, no WebAssembly bundle, no hydration. This
document explains why and what the trade-off looks like.

## The three Leptos modes

| Feature | Role | Target(s) required |
|---|---|---|
| `csr` | Client-side rendering (SPA) | `wasm32-unknown-unknown` only |
| `hydrate` | Server render + browser hydration | Both `x86_64` *and* `wasm32` |
| **`ssr`** | **Server renders HTML, no client reactivity** | `x86_64` only |

peisear's `Cargo.toml` pulls Leptos with just the `ssr` feature:

```toml
leptos = { version = "0.8", features = ["ssr"] }
```

## What SSR buys us

### Structurally

- **One build target.** No `wasm32-unknown-unknown`, no `cargo-leptos`,
  no separate client/server compile phases, no wasm-bindgen CLI.
  Standard `rustc` from `apt` or `rustup` is enough.
- **One artifact.** `target/release/peisear` is the whole thing.
- **Typed templates.** Every page is a `#[component]` function. Props
  are compile-time checked; interpolation is auto-escaped. XSS bugs
  require deliberately reaching for `.inner_html()` — which we don't.

### Operationally

- **Fast first paint.** First response is ready-to-render HTML, not a
  JS shell waiting to execute. Users on slow connections, old
  browsers, or JS-disabled contexts get a working page.
- **SEO-friendly.** Crawlers see content immediately.
- **Minimal JS footprint.** The only client-side JS is
  `static/board.js` (~2 KB, hand-rolled, no framework), loaded only
  on the board view for drag-and-drop.

## What SSR rules out

No client-side reactivity means:

- **Form submissions reload the page.** A POST to `/projects` gets a
  303 redirect and the browser follows it. This is the classical web
  model; it works, but it's not a single-page app.
- **Signal updates don't propagate to the browser.** `RwSignal`,
  `create_effect`, and the Leptos reactivity primitives run on the
  server to *build* the HTML, then stop. The HTML that arrives at the
  browser is inert.
- **Drag-and-drop needs imperative JS.** The kanban board does its
  drag handling in plain vanilla JS (`static/board.js`), which POSTs
  to a small JSON endpoint and reloads the page on success. It's
  simple, auditable, and fast enough — but it's not the idiomatic
  Leptos experience.

## The `.to_html()` call

The full rendering plumbing for a handler return is:

```rust
pub fn render_to_html<F, V>(view: F) -> Html<String>
where
    F: FnOnce() -> V,
    V: IntoView,
{
    let body = view().into_view().to_html();
    Html(format!("<!DOCTYPE html>{body}"))
}
```

`to_html` is from `tachys::view::RenderHtml`, re-exported via
`leptos::prelude::*`. It's the single function turning a Leptos view
tree into a `String`. The `<!DOCTYPE html>` prefix is added by us
because `to_html` doesn't emit it.

## Why start with SSR and not skip straight to hydration

Two reasons:

1. **Toolchain minimalism.** Hydration requires the wasm target
   plus `cargo-leptos`; SSR doesn't. For a project that wants to be
   trivially buildable on a fresh CI runner or a minimal apt-only
   machine, SSR pulls in an order of magnitude less complexity.
2. **Upgrade path is cheap.** Components, props, and handlers do not
   change going from `ssr` to `hydrate` — only the crate features,
   the router integration, and the build tool change. Starting in
   SSR doesn't lock us out of anything.

See [../guides/hydration-upgrade.md](../guides/hydration-upgrade.md)
for the step-by-step when you're ready to cross that bridge.

## Next

- [Crate boundaries](crate-boundaries.md) — where components sit in the
  `peisear-web` crate
- [../operations/tailwind-local.md](../operations/tailwind-local.md) —
  the one part of the UI that still depends on a CDN
