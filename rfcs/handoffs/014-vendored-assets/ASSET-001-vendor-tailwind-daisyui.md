# ASSET-001 — vendor Tailwind and DaisyUI

**Governing**: `DEC-051` (RFC 011). **Target release**: 0.32.0.
**Source**: `.git-exclude/tasks/architect/015-rfc-011-step-4-decision.md`

## 1. What and why, in one paragraph

`components/layout.rs:34–35` loads DaisyUI from `cdn.jsdelivr.net` and Tailwind
from `cdn.tailwindcss.com` at runtime. **With both blocked, the application
renders unstyled**: `.btn` drops from 44 px to 17 px, the type falls back to
Times New Roman, and the account menu cannot collapse. Everything still *works* —
every link, every form — but every guarantee RFC 012 established is contingent on
a third party being reachable by the end user's browser.

Both move into `static/`.

## 2. The two halves are built differently. Do not treat them alike.

**DaisyUI — vendor the prebuilt file.** `dist/full.min.css` is self-contained
CSS. **Do not attempt to run DaisyUI as a Tailwind plugin**: the standalone
binary bundles first-party plugins only, and going down that road is how this
lands in npm. Fetch the pinned `daisyui@4.12.14` file, commit it, record its
SHA-256.

**Do not prune its 32 themes**, even though only `corporate` and `dark` are used
and the saving would be large. That is a separate, measured change (`DEC-051`),
and compounding a fragile text transform on a third-party artefact into this work
would defeat its purpose.

**Tailwind — build purged CSS with the standalone binary.**
`tailwindcss-linux-x64` (and the other targets) ship on the `v3.4.15` release.
**Pin by version and SHA-256.** No Node, no npm, no `package.json`, no lockfile —
if any of those appear, stop and escalate; that is option (b), which was
considered and rejected for its continuous cost.

**Commit the built CSS.** A self-hoster runs `cargo build` and gets a styled
application. The binary is a maintainer's tool for regenerating, not a build
dependency.

## 3. The trap: purging is silent

**The Play CDN never purges — it scans the live DOM and generates whatever it
finds.** A build-time scan reads source files instead, so **any class it does not
see disappears**, and nothing fails. The page just renders slightly wrong,
somewhere.

Known ways a class escapes the scan:

- **It lives in another crate.** `peisear-core/src/lib.rs` returns
  `badge-ghost`, `badge-info`, `badge-warning`, `badge-error`, `badge-success`.
  Those are DaisyUI component classes so they survive here — **but the point
  generalises**, and the content glob must cover the whole workspace, not
  `peisear-web` alone.
- **It lives in `static/*.js`.**
- **It is composed at runtime** and never appears as a complete literal.

**Do not try to enumerate the risk. Measure the outcome.** §4.

## 4. Verification — a computed-style diff, before and after

`.git-exclude/tools/cdp.mjs`. This is the check that makes the change safe, and
it is the same technique that will make future upgrades safe.

**Before the change**, against a running instance, for each of at least
`/today`, `/inbox`, `/today/calendar`, `/projects`, a project detail page with
at least one issue, the board, an issue detail page, `/settings` and
`/settings/notifications`: walk every element and record a **computed-style
fingerprint** — at minimum `display`, `width`, `height`, `font-size`,
`font-family`, `color`, `background-color`, `padding`, `margin`, `border-width`.

**After the change**, capture the same and **diff**.

- **The expected diff is empty.** Any element whose computed style moved is a
  purged class or a cascade-order change, and is a defect until explained.
- **Report the comparison, not a summary.** "No visual regressions" is not the
  claim; the claim is *N elements compared across M pages, K differences, each
  one named*.

**Then repeat with both CDNs blocked** (`Network.setBlockedURLs` on
`*cdn.jsdelivr.net*` and `*cdn.tailwindcss.com*`). **After this change that must
change nothing at all** — which is the entire point, and the one assertion that
proves the work is done.

**Also confirm**: no overflow regression at 390/768/1280/1920 on those pages
(`LAYOUT-001`, `LAYOUT-002`), and `.btn` still 44 px (`TT-004`).

## 5. Two things to tidy while in there

- **The inline `tailwind.config` script** (`layout.rs:37`, `darkMode`) is a Play
  CDN construct. It belongs in the build's own config file, and the inline
  `<script>` goes away.
- **`contrast_scan` and `touch_target_scan` name the CDN URL** in their doc
  comments as the source of the resolved values they assert. Point them at the
  vendored file. **This is an improvement worth noticing**: those guards
  currently reason about a file that is not in the repository, and `§9.1` records
  a convention about finding a scratch copy of it in `.git-exclude/tmp/`. After
  this, the file they reason about is the file the product ships.

## 6. Escalate rather than deciding

- **If the purge cannot be made complete** without listing classes by hand. A
  safelist is an exception list, and this project's history with those is in
  `touch_target_scan`'s own doc comment.
- **If anything requires Node**, npm, or a lockfile.
- **If the computed-style diff is non-empty and the cause is not obvious.**
- **If the standalone binary's output differs from the Play CDN in a way that
  looks deliberate on Tailwind's part** — that is a finding about the two being
  different products, not a bug to paper over.

## 7. Exit condition

Both assets served locally; the application byte-for-byte equivalent in computed
style before and after; **identical with both CDNs blocked**; no overflow or
target-size regression; `DEC-007` clean; three consecutive `cargo test
--workspace` runs.

**And the figures for the record**: the size of the built Tailwind CSS, the size
of the vendored DaisyUI file, and both gzipped. `DEC-051` says bytes are not the
argument — the changelog should still state them rather than imply an
improvement nobody measured.

---

**Who holds what**: dev team — the vendoring. **What's blocked**: RFC 011 step
4's overflow gate, and `NFR-CMP-002`'s status. **What's next**: review request.
