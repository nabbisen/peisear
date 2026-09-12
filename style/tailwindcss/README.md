# Vendored CSS — regeneration, not a build dependency

`ASSET-001` (`DEC-051`): `static/tailwind.css` and `static/daisyui.min.css`
are committed, built ahead of time, and shipped as-is. **`cargo build` does
not run anything in this directory.** A self-hoster runs `cargo build` and
gets a styled application with no Node, no npm, and no network access at
runtime. This directory exists for the next person who needs to regenerate
one of the two files — a Tailwind class was added and the build is stale,
or a version bump is due.

## Pinned versions

| | Version | Source | SHA-256 |
|---|---|---|---|
| DaisyUI | `4.12.14` | `https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css` | `bf619937eca81b323ca601ab7347443a3c4c8b6ad3306bc9908ef127d207d0b6` |
| Tailwind standalone CLI | `v3.4.15` | `https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.15/tailwindcss-linux-x64` | `244c0ae588397c6812f41fee5277f8ddbfd5aa186e5c71673724a1a0328e2a0e` |

Both are pinned by version **and** checksum. Nothing watches upstream for a
new release of either — that is a separate, deliberately excluded piece of
work (`DEC-051` §6.5) — so bumping a version here is a manual decision,
verified against the checksum above before it changes.

## DaisyUI — vendor the file, don't touch it

```bash
curl -L "https://cdn.jsdelivr.net/npm/daisyui@<version>/dist/full.min.css" \
  -o static/daisyui.min.css
sha256sum static/daisyui.min.css   # compare against the table above; update it on a real bump
```

**Do not run DaisyUI as a Tailwind plugin**, and do not prune its 32 themes
even though only `corporate` and `dark` ship in the product today —
`ASSET-001` §2 covers why: the standalone binary bundles first-party
plugins only, and pruning is a separate, measured change this work
deliberately excludes.

## Tailwind — rebuild with the standalone binary

`bin/` is gitignored — the ~43 MB binary itself is never committed, only
pinned by version and checksum above. Download it once:

```bash
curl -L "https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.15/tailwindcss-linux-x64" \
  -o style/tailwindcss/bin/tailwindcss-linux-x64
chmod +x style/tailwindcss/bin/tailwindcss-linux-x64
sha256sum style/tailwindcss/bin/tailwindcss-linux-x64   # compare against the table above
```

Other platforms: swap `-linux-x64` for `-macos-x64`, `-macos-arm64`,
`-linux-arm64`, or `-linux-armv7` on the same release tag.

Rebuild, **from the repo root** (the config's own `content` globs are
relative to it, not to this directory):

```bash
./style/tailwindcss/bin/tailwindcss-linux-x64 \
  -c style/tailwindcss/tailwind.config.js \
  -i style/tailwindcss/input.css \
  -o static/tailwind.css \
  --minify
```

Commit the regenerated `static/tailwind.css`.

## The trap this whole setup exists to avoid

**The Play CDN this replaced never purged — it scanned the live DOM and
generated whatever it found.** This build scans source files instead
(`tailwind.config.js`'s `content` globs), so **any class it cannot see
disappears, silently** — the page renders slightly wrong somewhere, and
nothing fails. `content` covers every crate under `crates/`, not just
`peisear-web`, because `peisear-core::project_health` returns DaisyUI
badge classes as plain strings the Play CDN could see at runtime
regardless of which crate wrote them.

**After any rebuild, verify with a computed-style diff**
(`.git-exclude/tools/cdp.mjs`, the same technique `ASSET-001` itself used),
not by re-reading the diff for plausibility. A safelist that lists classes
by hand is an exception list, and this project's history with those —
`touch_target_scan`'s own doc comment — is why `ASSET-001` treats a
non-empty diff as something to explain, not paper over with one.

## The extractor reads comments too

Tailwind's content scanner tokenises raw file text, comments included. A CSS
property or utility name written in a Rust comment — `flex-shrink`, `shrink`
— becomes a rule in `static/tailwind.css` even when no markup uses it
(`LAYOUT-006` added two such rules by explaining a fix). So the vendored file
is a **superset** of what the markup uses, not an inventory of it. When a
regeneration diff shows a class appearing from nowhere, look for it in prose
before looking for it in markup. Classes *disappearing* remain the trap that
matters, as above.

## A hand-written utility exists only once the markup uses it

`@layer utilities` entries in `input.css` are purged like every other class:
a regeneration run before any markup uses the new class adds **nothing**, and
looks like a broken build. Write the utility, use it, then regenerate
(`LAYOUT-008`, `wrap-anywhere`).
