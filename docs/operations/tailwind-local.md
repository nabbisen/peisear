# Tailwind Self-Hosting

peisear's default `<Base>` component loads Tailwind CSS and daisyUI
from a CDN:

```html
<link href="https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css" rel="stylesheet">
<script src="https://cdn.tailwindcss.com/3.4.15"></script>
```

This is fine for most self-hosted deployments where outbound HTTPS is
available, and it keeps the "no Node toolchain required" story
intact. But there are two reasons to self-host the CSS:

1. **Air-gapped deployments** — no outbound internet from the app host.
2. **Strict CSP** — a Content-Security-Policy that disallows
   `cdn.jsdelivr.net` and `cdn.tailwindcss.com`.

## Build Tailwind locally

You need Node.js for this step, but only at build time — not at
runtime.

```bash
cd /wherever/you/built/peisear/

# One-time: install the Tailwind CLI and daisyUI.
npm install -D tailwindcss@3 daisyui@4

# Create a minimal entry file.
cat > input.css <<'EOF'
@tailwind base;
@tailwind components;
@tailwind utilities;
EOF

# Create a minimal tailwind config that scans peisear's components.
cat > tailwind.config.js <<'EOF'
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./crates/peisear-web/src/components/**/*.rs"],
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["corporate", "dark"],
  },
};
EOF

# Produce the bundle.
npx tailwindcss -i ./input.css -o ./static/app.css --minify
```

The resulting `static/app.css` is a few dozen KB gzipped and contains
exactly the utility classes and daisyUI components that peisear
actually uses.

## Wire it up

In `crates/peisear-web/src/components/layout.rs`, find the `<Base>`
component and replace the CDN `<link>` and `<script>` with a single
local link:

```rust
// Before
view! {
    <head>
        …
        <link href="https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css" rel="stylesheet"/>
        <script src="https://cdn.tailwindcss.com/3.4.15"></script>
        <link rel="stylesheet" href="/static/app.css"/>
        …
    </head>
}

// After
view! {
    <head>
        …
        <link rel="stylesheet" href="/static/app.css"/>
        …
    </head>
}
```

Rebuild the binary (`cargo build --release`) and redeploy.

## Content-Security-Policy

With self-hosted CSS you can tighten your `Content-Security-Policy`
to effectively `default-src 'self'`. A reasonable starting point for
a reverse proxy:

```
Content-Security-Policy: default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'self'; frame-ancestors 'none';
```

The `'unsafe-inline'` on `style-src` is because Leptos components can
embed inline `style=` attributes. If you want to tighten this further,
rewrite those into CSS classes.

## Next

- [Deployment](deployment.md) — where the `static/` dir ends up
- [../security/hardening.md](../security/hardening.md) — CSP is one
  of several headers worth setting at the reverse proxy
