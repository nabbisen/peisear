# Handoffs — `static/` is public

**Not RFC-governed.** `ServeDir::new("static")` serves that directory in full,
so anything placed there is a public URL on every deployment. Found while
assembling `REL-0.38.0`'s candidate, before the tag.

| ID | Link | What | Release |
|---|---|---|---|
| STATIC-001 | [STATIC-001](./STATIC-001-a-doc-under-static-is-a-public-url.md) | `static/README.md` — a document describing **what the test suite does not cover** — would become a public URL on every self-hosted deployment, and 0.38.0 would be the first release to publish it. Moved to `docs/`, with a pointer where a developer looks and an allow-list assertion in the module that already walks the directory. **Blocks the 0.38.0 tag.** | 0.38.0 |
