// `SWP-001` (`068`) -- the rendered-contrast gate. Covers WCAG 1.4.3
// (text, 4.5:1, or 3:1 at >=24px or >=18.66px bold) and 1.4.11
// (non-text, 3:1, for the boundary/focus colours `EDG-001`-`EDG-003`
// repaired) **together**, because both are the same question asked of
// the same rendered pixel: what colour does the browser actually
// paint, and what is it painted on.
//
// **Why this exists, in one sentence** (`068/README.md` §1): *source is
// total and approximate; rendered is exact and fixture-bounded; where
// they disagree, the renderer is right about what it reached, and
// silent about the rest.* `contrast_scan`'s four static mechanisms read
// a class token or a stylesheet rule -- never a cascade, never a
// composite. This gate renders the page and reads the pixel, which is
// the one thing a static scan cannot honestly do (`contrast_scan.rs`'s
// own doc comment says so at every one of its four mechanisms).
//
// ## Measuring by painting, not parsing (`068/SWP-001` §2.1)
//
// `getComputedStyle(el).color` can read back as `oklch(...)` in this
// Chrome -- a normalisation trick that sets then re-reads a colour
// through canvas `fillStyle` finds the **same** string unchanged, so a
// numeric scan of it sees three meaningless tokens, not a colour. The
// renderer performs the real conversion and the real alpha composite
// only when it actually **paints** -- so every colour here is
// resolved by `fillRect` + `getImageData`, never by string parsing.
// `paintLayer` below detects whether an assignment was even accepted
// (the canvas 2D spec says `fillStyle` silently ignores a value it
// cannot parse, keeping the old one) with a sentinel comparison rather
// than trusting the readback.
//
// ## 1.4.3's population: every element with its own visible text
//
// Walked directly off `document.body`, inside one `browser.eval()`
// call per (page, width) cell -- not one round trip per element, which
// at ~40 text-bearing elements per cell and 120 cells would be
// thousands of WebSocket round trips for no reason `occlusion-gate.mjs`
// and `disclosure-marker-gate.mjs` don't already demonstrate is
// unnecessary (both compute their own per-page population inside one
// `eval` string too).
//
// ## 1.4.11's population: the repaired families, the broader sweep as a count
//
// `068/SWP-001` §3's own open choice: judge every element with a
// visible border, or the families `EDG-001`/`EDG-002` actually
// repaired (`.input-bordered`, `.select-bordered`, `.textarea-bordered`,
// `.checkbox` -- the literal selectors `contrast_scan`'s own
// `ACKNOWLEDGED_BOUNDARY_RULES`/`EXEMPT_BOUNDARY_RULES` key on)? **The
// repaired families**, per the architect's own lean in that handoff:
// card and alert borders were ruled decoration, out of 1.4.11's own
// test, at `0.68.0` (`§10.63`) -- a browser gate re-opening that by
// quietly asserting on every bordered element would move a decision
// `068` was told not to reopen (handoff §4). The broader population is
// still measured and logged every run, as a **count**, so the
// unmeasured surface stays visible rather than silently dropped.
//
// ## The composite, per side (`068/SWP-001` §2.3)
//
// A `border-color` composites over the control's **own fill**
// (`background-clip: border-box` is the default) -- the fact
// `EDG-002` had to measure by hand and the architect's own `/50`
// recommendation got wrong. An `outline-color` paints **outside** the
// border box, over whatever is behind that ring -- the control's own
// fill is not there. So the resting border is measured against the
// ancestor chain **including** the element's own background; the
// focused outline is measured against the chain **excluding** it.
// Both chains are painted outermost-first, the same shape the text
// scan uses.
//
// **The focused border is measured and logged, never failed on.**
// `EXEMPT_BOUNDARY_RULES` (`contrast_scan.rs`) names these three sites
// as a deliberate, screenshot-checked choice: the ring alone carries
// the focus state, so the border staying faint under it is not a
// defect this gate re-opens.
//
// ## `068/SWP-001` §3.2 -- population and sanity, not optional
//
// A sweep that cannot tell *nothing wrong* from *nothing examined* is
// `§10.36` with a browser attached, which is exactly what the
// architect's own first two prototypes were (`examined=0,
// failures=0`; `examined=1906, failures=1906`, both from a broken
// probe). Four checks, independent of whether anything genuinely
// fails this run:
//
// 1. **A floor on elements examined**, overall and per page.
// 2. **`unpaintable == 0`** -- a colour this gate could not paint is a
//    hard failure, never a silent skip.
// 3. **A known-good control pair** -- `/today`'s own `<h1>`, re-read
//    every run rather than hardcoded from one measurement.
// 4. **Neither 0% nor 100% of the raw population may be a raw WCAG
//    violation** -- "raw" meaning *before* the shared exclusion list
//    is applied. The brand dot and the status number are real,
//    expected, named exceptions to a real rule, so they keep this
//    figure off zero on a clean tree; the figure never reaching 100%
//    is what tells `examined=1906, failures=1906` apart from a real
//    run. This is the one assertion that has to keep working **after**
//    this round's own repair, not just before it.
//
// ## The shared exclusion list (`068/SWP-001` §3.3)
//
// `contrast-exclusions.json`, beside this file -- the one home both
// readers read. `contrast_scan.rs` reads every field (`classValue`,
// `liveCount`, `reason`); this gate reads only `classValue`, since it
// has no source population to check a count against and no use for a
// prose reason at runtime. That is a field one reader ignores, stated
// rather than hidden -- the honest cost of one home instead of the
// two-lists-plus-a-comparison shape the owner's review already
// rejected for this exact file, one series earlier.
//
// ## Inactive controls (`068/SWP-001` §3.1)
//
// Exempt from 1.4.3 by the criterion itself. `el.disabled || el.matches(':disabled')`
// is checked at every text element; zero sites set `disabled` on
// anything today (`contrast_scan.rs`'s own `EXEMPT_SHEET_RULES` doc
// comment confirms this by grep), so this exemption is live code with
// no live site exercising it yet, same posture that module already
// takes for the same reason.
//
// **Known limit, stated rather than hidden**: this method does not
// composite the CSS `opacity` property (which dims an element and its
// subtree as a single unit, not folded into `color`). A real pixel
// sample via `Page.captureScreenshot` would see it; this canvas
// replay does not. `contrast_scan.rs` draws the identical line for the
// same reason (its own doc comment, `text-base-content/N` vs bare
// `opacity-*`) -- the fifteen bare-`opacity` sites that entry already
// named stay unmeasured here too.

import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { launch } from './cdp.mjs';

const REPO_ROOT = new URL('..', import.meta.url).pathname;
const BIN_PATH = process.env.PEISEAR_BIN || join(REPO_ROOT, 'target/debug/peisear');
// `068/SWP-001` §3: a free port, clear of the five siblings' own
// 4173/4174/4175/4177/4233 -- `browser-checks/README.md`'s own warning
// about `PEISEAR_PORT` colliding under parallel runs applies the same
// way here as it does to them, so this one just needs to be distinct,
// not anything clever.
const PORT = process.env.PEISEAR_PORT || '4280';
process.env.PEISEAR_PORT = PORT;
const BASE = `http://127.0.0.1:${PORT}`;

const EXCLUSIONS = JSON.parse(readFileSync(new URL('./contrast-exclusions.json', import.meta.url), 'utf8'))
  .map(e => e.classValue);

function log(...args) {
  console.log('[contrast-gate]', ...args);
}

async function waitForServer(url, timeoutMs = 20000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const r = await fetch(url, { redirect: 'manual' });
      if (r.status) return;
    } catch {}
    await new Promise(r => setTimeout(r, 200));
  }
  throw new Error(`server at ${url} did not become ready within ${timeoutMs}ms`);
}

// Shared machinery both per-cell scans below splice into their own
// `eval()` string -- painting, luminance, the ancestor-background
// chain, visibility. Written once here as a string so both can inline
// it without a second, subtly different copy.
const SHARED_JS = `
  const SENTINEL = 'rgb(1, 2, 3)';
  const canvas = document.createElement('canvas');
  canvas.width = 1; canvas.height = 1;
  const ctx = canvas.getContext('2d', { willReadFrequently: true });
  function paintLayer(color) {
    ctx.fillStyle = SENTINEL;
    ctx.fillStyle = color;
    if (ctx.fillStyle === SENTINEL) return false;
    ctx.fillRect(0, 0, 1, 1);
    return true;
  }
  function readPixel() {
    const d = ctx.getImageData(0, 0, 1, 1).data;
    return [d[0], d[1], d[2]];
  }
  function srgbToLinear(c) {
    c = c / 255;
    return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
  }
  function luminance(rgb) {
    return 0.2126 * srgbToLinear(rgb[0]) + 0.7152 * srgbToLinear(rgb[1]) + 0.0722 * srgbToLinear(rgb[2]);
  }
  function contrastOf(a, b) {
    const L1 = luminance(a), L2 = luminance(b);
    const hi = Math.max(L1, L2), lo = Math.min(L1, L2);
    return (hi + 0.05) / (lo + 0.05);
  }
  function isVisible(el) {
    const r = el.getBoundingClientRect();
    if (r.width === 0 || r.height === 0) return false;
    const cs = getComputedStyle(el);
    return cs.visibility !== 'hidden' && cs.display !== 'none' && parseFloat(cs.opacity) > 0;
  }
  function ancestorChain(el) {
    const chain = [];
    let cur = el;
    while (cur) { chain.unshift(cur); cur = cur.parentElement; }
    return chain;
  }
  // Paints '#ffffff' (this theme's own base-100, the page's real
  // default) then every ancestor's own backgroundColor outermost
  // first -- never asks what alpha each carries, the browser's own
  // compositor does that on fillRect.
  function paintBgChain(chain) {
    if (!paintLayer('#ffffff')) return null;
    for (const a of chain) {
      if (!paintLayer(getComputedStyle(a).backgroundColor)) return null;
    }
    return readPixel();
  }
  function describe(el) {
    const cls = el.getAttribute('class') || '';
    const text = (el.textContent || '').trim().slice(0, 40);
    return { tag: el.tagName.toLowerCase(), cls, text };
  }
`;

// 1.4.3: every element with its own visible text (a direct, non-
// whitespace text-node child), one `eval` per cell. `EXCLUSIONS` is
// spliced in as a literal array -- the shared list's `classValue`
// field only, per this file's own doc comment on why the other two
// fields are read on the Rust side alone.
function textScanExpr() {
  return `(() => {
    ${SHARED_JS}
    const EXCLUSIONS = ${JSON.stringify(EXCLUSIONS)};
    function hasOwnText(el) {
      for (const n of el.childNodes) {
        if (n.nodeType === 3 && n.textContent.trim().length > 0) return true;
      }
      return false;
    }
    let examined = 0;
    let rawViolations = 0;
    const failures = [];
    const unpaintable = [];
    for (const el of document.body.querySelectorAll('*')) {
      if (el.tagName === 'SCRIPT' || el.tagName === 'STYLE') continue;
      if (!hasOwnText(el)) continue;
      if (!isVisible(el)) continue;

      const cs = getComputedStyle(el);
      const fontPx = parseFloat(cs.fontSize);
      const weight = cs.fontWeight;
      const bold = weight === 'bold' || parseInt(weight, 10) >= 700;
      const large = fontPx >= 24 || (bold && fontPx >= 18.66);
      const need = large ? 3.0 : 4.5;

      const bg = paintBgChain(ancestorChain(el));
      if (!bg) { unpaintable.push(describe(el)); continue; }
      if (!paintLayer(cs.color)) { unpaintable.push(describe(el)); continue; }
      const fg = readPixel();
      const ratio = contrastOf(fg, bg);
      examined++;

      if (ratio < need) {
        rawViolations++;
        const cls = el.getAttribute('class') || '';
        const excluded = EXCLUSIONS.includes(cls);
        const inactive = el.disabled === true || (typeof el.matches === 'function' && el.matches(':disabled'));
        if (!excluded && !inactive) {
          failures.push({ ...describe(el), need, got: Math.round(ratio * 100) / 100, fontPx, bold });
        }
      }
    }
    return { examined, rawViolations, failures, unpaintable };
  })()`;
}

// 1.4.11, the repaired families only (`068/SWP-001` §3's own choice,
// this file's doc comment). Per matching element: the resting
// border-color against the chain *including* its own background, and
// -- once focused -- the outline-color against the chain *excluding*
// it, and the focused border-color, logged but never failed
// (`EXEMPT_BOUNDARY_RULES`). The broader population (every visible
// bordered element) is counted, not asserted on.
function borderScanExpr() {
  return `(() => {
    ${SHARED_JS}
    const SELECTOR = '.input-bordered, .select-bordered, .textarea-bordered, .checkbox';
    let examined = 0;
    let rawViolations = 0;
    const failures = [];
    const unpaintable = [];
    const exemptFocusedBorder = [];
    for (const el of document.body.querySelectorAll(SELECTOR)) {
      if (!isVisible(el)) continue;
      examined++;
      const chainIncl = ancestorChain(el);
      const chainExcl = chainIncl.slice(0, -1);

      // Two states this element can already be in before this script
      // ever touches it, neither of which is the "resting" state
      // EDG-001/EDG-002 repaired and ACKNOWLEDGED_BOUNDARY_RULES
      // names:
      //
      // - **Already focused on load** (several forms carry
      //   \`autofocus\`) -- \`getComputedStyle\` before this script calls
      //   \`el.focus()\` would read the *focused* border, not resting,
      //   and measure the deliberately-faint EXEMPT figure as if it
      //   were the repaired one. The focused branch below measures it
      //   correctly either way.
      // - **\`.checkbox:checked\`** -- DaisyUI's own checked state gives
      //   the element an *opaque* fill (\`--chkbg\`) and a border that is
      //   the same colour at 0.7 alpha *painted over that same fill*,
      //   so border-vs-fill collapses toward 1:1 by construction, not
      //   by any defect: once checked, the control's own filled square
      //   is what a reader perceives as its boundary, not a now-
      //   redundant 1px edge. \`EDG-001\`/\`EDG-002\` never measured or
      //   repaired this state; this gate does not invent a finding
      //   for it either.
      const alreadyFocused = document.activeElement === el;
      const checkedCheckbox = el.matches('.checkbox') && el.checked;
      const csRest = getComputedStyle(el);
      if (!alreadyFocused && !checkedCheckbox && csRest.borderStyle !== 'none' && parseFloat(csRest.borderWidth) > 0) {
        const bg = paintBgChain(chainIncl);
        if (!bg) { unpaintable.push(describe(el)); }
        else if (!paintLayer(csRest.borderColor)) { unpaintable.push(describe(el)); }
        else {
          const ratio = contrastOf(readPixel(), bg);
          if (ratio < 3) {
            rawViolations++;
            failures.push({ ...describe(el), phase: 'resting-border', need: 3, got: Math.round(ratio * 100) / 100 });
          }
        }
      }

      el.focus();
      const csFocus = getComputedStyle(el);
      if (csFocus.outlineStyle !== 'none' && parseFloat(csFocus.outlineWidth) > 0) {
        const bg = paintBgChain(chainExcl);
        if (!bg) { unpaintable.push(describe(el)); }
        else if (!paintLayer(csFocus.outlineColor)) { unpaintable.push(describe(el)); }
        else {
          const ratio = contrastOf(readPixel(), bg);
          if (ratio < 3) {
            rawViolations++;
            failures.push({ ...describe(el), phase: 'focused-outline', need: 3, got: Math.round(ratio * 100) / 100 });
          }
        }
      }
      if (csFocus.borderStyle !== 'none' && parseFloat(csFocus.borderWidth) > 0) {
        const bg = paintBgChain(chainIncl);
        if (bg && paintLayer(csFocus.borderColor)) {
          const ratio = contrastOf(readPixel(), bg);
          // EXEMPT_BOUNDARY_RULES: logged, never failed -- EDG-002 §4's
          // own screenshot already settled that the ring alone carries
          // the focus state on these three sites. Counted into
          // rawViolations (it is a real below-3:1 value, by design)
          // so the §3.2 "neither 0% nor 100%" sanity check has a real,
          // non-degenerate population to look at on a clean tree too.
          if (ratio < 3) rawViolations++;
          exemptFocusedBorder.push({ ...describe(el), got: Math.round(ratio * 100) / 100 });
        }
      }
      el.blur();
    }

    const broaderBordered = Array.from(document.body.querySelectorAll('*')).filter(el => {
      if (!isVisible(el)) return false;
      const cs = getComputedStyle(el);
      return cs.borderStyle !== 'none' && parseFloat(cs.borderWidth) > 0;
    }).length;

    return { examined, rawViolations, failures, unpaintable, exemptFocusedBorder, broaderBordered };
  })()`;
}

const KNOWN_GOOD_EXPR = `(() => {
  ${SHARED_JS}
  const h1 = document.querySelector('h1');
  if (!h1) return { error: 'no <h1> on this page' };
  const bg = paintBgChain(ancestorChain(h1));
  if (!bg || !paintLayer(getComputedStyle(h1).color)) return { error: 'unpaintable' };
  const ratio = contrastOf(readPixel(), bg);
  return { ratio: Math.round(ratio * 100) / 100, fg: readPixel(), bg };
})()`;

async function main() {
  const dbDir = mkdtempSync(join(tmpdir(), 'contrast-gate-db-'));
  const dbPath = join(dbDir, 'app.db');

  log(`starting ${BIN_PATH} on ${BASE}, db=${dbPath}`);
  const server = spawn(BIN_PATH, [], {
    env: {
      ...process.env,
      DATABASE_URL: `sqlite://${dbPath}`,
      BIND_ADDR: `127.0.0.1:${PORT}`,
      COOKIE_SECURE: 'false',
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let serverOutput = '';
  server.stdout.on('data', d => { serverOutput += d; });
  server.stderr.on('data', d => { serverOutput += d; });

  let exitCode = 1;
  let browser = null;
  try {
    await waitForServer(`${BASE}/login`);
    log('server ready');

    const { WIDTHS, ADVERSARIAL_STRINGS, createFixtures, buildPages } = await import('./overflow-gate.mjs');
    const fixture = await createFixtures(ADVERSARIAL_STRINGS);
    log(`fixtures created: project=${fixture.projectId} issue=${fixture.issueId}`);
    const pages = buildPages(fixture);
    log(`coverage: ${Object.keys(pages).length} pages x ${WIDTHS.length} widths, exclusions=${JSON.stringify(EXCLUSIONS)}`);

    browser = await launch({ width: 1280, height: 900 });
    const [cookieName, cookieValue] = fixture.cookie.split('=');
    await browser.send('Network.setCookie', {
      name: cookieName, value: cookieValue, domain: '127.0.0.1', path: '/',
    });

    const textExpr = textScanExpr();
    const borderExpr = borderScanExpr();

    let cellCount = 0;
    let textExamined = 0;
    let textRawViolations = 0;
    let borderExamined = 0;
    let borderRawViolations = 0;
    let broaderBorderedTotal = 0;
    const textUnpaintable = [];
    const borderUnpaintable = [];
    const textFailures = [];
    const borderFailures = [];
    const exemptFocusedBorderSeen = [];
    const examinedByPage = {};
    let knownGood = null;

    for (const [label, url] of Object.entries(pages)) {
      examinedByPage[label] = 0;
      for (const [w, h, mobile] of WIDTHS) {
        cellCount++;
        await browser.setViewport(w, h, mobile);
        await browser.goto(url);
        await browser.waitForSelector('header.navbar', 5000);

        const t = await browser.eval(textExpr);
        textExamined += t.examined;
        textRawViolations += t.rawViolations;
        examinedByPage[label] += t.examined;
        for (const f of t.failures) textFailures.push({ page: label, width: w, ...f });
        for (const u of t.unpaintable) textUnpaintable.push({ page: label, width: w, ...u });

        const b = await browser.eval(borderExpr);
        borderExamined += b.examined;
        borderRawViolations += b.rawViolations;
        broaderBorderedTotal += b.broaderBordered;
        for (const f of b.failures) borderFailures.push({ page: label, width: w, ...f });
        for (const u of b.unpaintable) borderUnpaintable.push({ page: label, width: w, ...u });
        for (const e of b.exemptFocusedBorder) exemptFocusedBorderSeen.push({ page: label, width: w, ...e });

        if (label === 'today' && w === 1280) {
          knownGood = await browser.eval(KNOWN_GOOD_EXPR);
        }
      }
    }

    log(`${cellCount} cells x text + border scans`);
    log(`text (1.4.3): ${textExamined} elements examined, ${textRawViolations} raw (pre-exclusion) violations, ${textFailures.length} unacknowledged failures, ${textUnpaintable.length} unpaintable`);
    const exemptFocusedViolations = exemptFocusedBorderSeen.filter(e => e.got < 3).length;
    log(`border/outline (1.4.11), repaired families: ${borderExamined} elements examined (focused ${exemptFocusedBorderSeen.length} times), ${borderRawViolations} raw violations (${exemptFocusedViolations} of which are EXEMPT_BOUNDARY_RULES focused-border sites, logged not failed), ${borderFailures.length} unacknowledged failures, ${borderUnpaintable.length} unpaintable; broader bordered-element population (count only, not asserted): ${broaderBorderedTotal}`);
    log(`known-good control (/today <h1>): ${knownGood && 'ratio' in knownGood ? `${knownGood.ratio}:1` : JSON.stringify(knownGood)}`);

    const sanityFailures = [];
    const TEXT_FLOOR = 2000;
    if (textExamined < TEXT_FLOOR) {
      sanityFailures.push(`text population floor: examined ${textExamined}, expected >= ${TEXT_FLOOR} -- a sweep that examines almost nothing reads exactly like one that found nothing wrong`);
    }
    const emptyPages = Object.entries(examinedByPage).filter(([, n]) => n === 0).map(([k]) => k);
    if (emptyPages.length > 0) {
      sanityFailures.push(`these pages contributed zero examined text elements: ${emptyPages.join(', ')}`);
    }
    if (textUnpaintable.length > 0) {
      sanityFailures.push(`${textUnpaintable.length} text colour(s) could not be painted (hard failure, not a skip): ${JSON.stringify(textUnpaintable.slice(0, 5))}`);
    }
    if (borderUnpaintable.length > 0) {
      sanityFailures.push(`${borderUnpaintable.length} border/outline colour(s) could not be painted: ${JSON.stringify(borderUnpaintable.slice(0, 5))}`);
    }
    if (!knownGood || !('ratio' in knownGood) || knownGood.ratio < 10) {
      sanityFailures.push(`known-good control (/today <h1>) measured ${JSON.stringify(knownGood)}, expected >= 10:1 -- if this moved, something upstream of this gate broke, not the gate`);
    }
    if (textRawViolations === 0 || textRawViolations === textExamined) {
      sanityFailures.push(`text raw-violation rate is ${textRawViolations}/${textExamined} -- 0% or 100% is itself an error (both of the architect's broken prototypes read this way)`);
    }
    if (borderExamined > 0 && (borderRawViolations === 0 || borderRawViolations === borderExamined)) {
      sanityFailures.push(`border/outline raw-violation rate is ${borderRawViolations}/${borderExamined} -- 0% or 100% is itself an error`);
    }

    if (sanityFailures.length > 0) {
      log(`${sanityFailures.length} SANITY FAILURE(S) -- §3.2, the part that is not optional:`);
      for (const s of sanityFailures) log(`  ${s}`);
    }

    if (textFailures.length > 0) {
      log(`${textFailures.length} TEXT (1.4.3) FAILURE(S):`);
      for (const f of textFailures) log(`  ${f.page}@${f.width} <${f.tag} class="${f.cls}"> "${f.text}" needs ${f.need}:1, measured ${f.got}:1`);
    }
    if (borderFailures.length > 0) {
      log(`${borderFailures.length} BORDER/OUTLINE (1.4.11) FAILURE(S):`);
      for (const f of borderFailures) log(`  ${f.page}@${f.width} <${f.tag} class="${f.cls}"> ${f.phase} needs ${f.need}:1, measured ${f.got}:1`);
    }
    if (textFailures.length === 0 && borderFailures.length === 0 && sanityFailures.length === 0) {
      log('0 failing cells, 0 sanity failures');
    }

    exitCode = (textFailures.length === 0 && borderFailures.length === 0 && sanityFailures.length === 0) ? 0 : 1;
  } catch (err) {
    log('ERROR:', err.stack || err.message);
    log('--- server output ---');
    log(serverOutput);
    exitCode = 1;
  } finally {
    if (browser) browser.close();
    server.kill('SIGTERM');
    try { rmSync(dbDir, { recursive: true, force: true }); } catch {}
  }

  process.exit(exitCode);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main();
}
