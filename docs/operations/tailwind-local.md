# Tailwind Self-Hosting

**There is nothing to do. The CSS has been self-hosted since `0.32.0`,
and a stock build makes no request to any third-party host.**

*Rewritten 2026-11-09 (`0.61.0`).* Until this release this page opened
with *"peisear's default `<Base>` component loads Tailwind CSS and
daisyUI from a CDN"* and then told a reader to install Node and npm,
write a `tailwind.config.js` that scans one crate, build a bundle over
the top of `static/app.css`, and edit `layout.rs` to replace CDN tags.
**Every sentence of that stopped being true at `0.32.0`** (`ASSET-001`,
`DEC-051`), twenty-nine releases before it was corrected. Following it
would have installed the exact Node dependency this project removed,
overwritten a hand-written stylesheet, and pointed the output at a
directory that no longer exists. It is recorded as `§10.53` in
`docs/specification/requirements.md` rather than quietly replaced.

## What ships today

`<Base>` links three stylesheets, all served by the binary itself from
memory:

```html
<link href="/static/daisyui.min.css" rel="stylesheet"/>
<link rel="stylesheet" href="/static/app.css"/>
<link href="/static/tailwind.css" rel="stylesheet"/>
```

- `daisyui.min.css` — vendored, pinned by version **and** SHA-256.
- `tailwind.css` — built ahead of time from this repository's own
  sources and committed.
- `app.css` — the handful of rules this project writes by hand.

They are embedded into the executable at compile time
(`crates/peisear-web/src/static_assets.rs`), so a `cargo install
peisear` with no `static/` directory beside it still serves them.

**The two reasons this page used to exist are already satisfied.**

1. **Air-gapped deployments.** A stock build fetches nothing at
   runtime. `browser-checks/overflow-gate.mjs` asserts this positively
   on every pull request — it fails every request whose host is not
   `127.0.0.1` and then asserts the blocked list is **empty**, across
   24 pages at 5 widths.
2. **Strict CSP.** No `cdn.jsdelivr.net`, no `cdn.tailwindcss.com`, no
   exception needed for either. See the section below.

## Regenerating the CSS

You do not need this to run peisear. You need it if you change a
Tailwind class in a component — a class with no rule in the shipped
stylesheet does nothing at all, which is what `§10.52` is about.

**The procedure, the pinned versions and their checksums live in
[`style/tailwindcss/README.md`](https://github.com/nabbisen/peisear/blob/main/style/tailwindcss/README.md)**
— one place, so the output path and the config's own content globs
cannot drift apart from each other again. `cargo build` runs none of
it, by design: a self-hoster needs no Node.

**`css_scan` checks the result.** Every class token the components
write must have a rule in one of the three stylesheets the binary
serves, or be named as a JavaScript hook — so a stale build fails the
test suite instead of silently rendering nothing.

## Content-Security-Policy

Because everything is served from the origin, you can tighten your
`Content-Security-Policy` to effectively `default-src 'self'`. A
reasonable starting point for a reverse proxy:

```
Content-Security-Policy: default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'self'; frame-ancestors 'none';
```

The `'unsafe-inline'` on `style-src` is because Leptos components can
embed inline `style=` attributes. If you want to tighten this further,
rewrite those into CSS classes.

## Next

- [Deployment](deployment.md) — what a deployed instance consists of
- [../security/hardening.md](../security/hardening.md) — CSP is one
  of several headers worth setting at the reverse proxy
