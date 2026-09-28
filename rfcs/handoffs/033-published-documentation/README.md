# Handoffs — published documentation

**`DEC-057`, owner-decided 2026-09-29.** `docs/` becomes an mdbook site on
GitHub Pages; the repository's Pages settings are already configured.

| ID | Link | What | Release |
|---|---|---|---|
| DOCS-001 | [DOCS-001](./DOCS-001-publish-docs-as-an-mdbook-site.md) | Book root `docs/` with `src = "."`, so **no existing file moves** and no inbound link breaks. The blocking problem is the four links that point *outside* `docs/` and would 404 on a site. The specification goes in whole and its superseded baselines do not. The build fails on a broken internal link, and **the link checker must be seen to fail** before it is believed. | 0.41.0 |
