// ASSET-001 (DEC-051): the standalone Tailwind CLI's own config file,
// not a Play CDN `tailwind.config` script tag. Run from the repo root
// (`style/tailwindcss/bin/tailwindcss-linux-x64 -c
// style/tailwindcss/tailwind.config.js -i style/tailwindcss/input.css
// -o static/tailwind.css --minify`), so every path below is relative
// to the repo root, not this directory.
//
// `darkMode` is copied verbatim from the inline script this replaces
// (`components/layout.rs`, pre-ASSET-001) -- same two-way toggle,
// `.dark` class or `data-theme="dark"`.
//
// No `plugins: [require('daisyui')]` here, and none should ever be
// added: DaisyUI is vendored separately as its own prebuilt
// `static/daisyui.min.css` (ASSET-001's own handoff, §2 -- "do not
// attempt to run DaisyUI as a Tailwind plugin", since the standalone
// binary only bundles first-party plugins). This build produces
// Tailwind's own utility classes only; DaisyUI's component classes
// come from the other file.
//
// `content` covers every crate, not just `peisear-web` --
// `peisear-core::project_health` returns DaisyUI badge classes
// (`badge-ghost`, `badge-info`, ...) as plain strings, and the Play
// CDN this replaces scanned the live, fully-rendered DOM, so it saw
// them regardless of which crate they came from. A build-time scan
// only sees what content globs point at; missing a crate here is
// exactly the silent-purge trap ASSET-001 §3 names.
module.exports = {
  content: [
    'crates/**/*.rs',
    'static/*.js',
  ],
  darkMode: ['class', '[data-theme="dark"]'],
  theme: {
    extend: {},
  },
  plugins: [],
  // DaisyUI's own components/utilities are not generated here (see
  // above); Preflight (base reset) stays on, matching the Play CDN's
  // own default behaviour, which this build otherwise reproduces.
  corePlugins: {
    preflight: true,
  },
};
