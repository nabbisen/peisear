// `OCCL-001` (`061`, `§10.55`) -- the occlusion gate.
//
// `overflow-gate.mjs` measures one property, `scrollWidth <=
// clientWidth`, and that property cannot see two controls covering
// each other: at 768/800px the account menu's clickable box covered
// the search field's right edge -- `document.elementFromPoint` 1px and
// 4px inside the input's own right edge returned the account control,
// not the input -- without the page scrolling by a single pixel. The
// account control is `btn btn-ghost`, so the overlapping region is
// transparent padding and a screenshot looks correct too. Two reasons,
// both of which hid it from everything this project owned before this
// file: an overflow check cannot see it, and a screenshot cannot
// either.
//
// **Its own gate, not a fourth property inside `overflow-gate.mjs`**
// (`§3`'s own open choice) -- that file's own README says "do not
// widen this gate," for the same reason `DISC-001`'s own
// `disclosure-marker-gate.mjs` is a sibling script rather than a new
// assertion bolted onto an existing one: a second, unrelated
// assertion on a gate that has run stable in CI is exactly how
// `DEC-048` condition 3 starts costing attention. This also keeps
// `overflow-gate.mjs`'s own one-line promise -- "one assertion,
// `scrollWidth <= clientWidth`" -- literally true.
//
// **Reuses `overflow-gate.mjs`'s own `WIDTHS`, `ADVERSARIAL_STRINGS`,
// `createFixtures` and `buildPages`, imported rather than copied** --
// "the gate's own five widths, on the pages it already visits" (`§3`)
// is then true by construction, not by two lists kept in sync by hand.
//
// **The check**: for every visible interactive control inside
// `header.navbar` (an `<a>`, `<button>`, `<input>`, or a
// `tabindex`-bearing `<label>` -- a dropdown trigger, not a real
// `<button>`, is how both the mobile menu and the account menu open),
// no two overlap. "Visible" excludes anything with zero area or
// `display:none`/`visibility:hidden` -- a closed dropdown's own menu
// items (`visibility:hidden` while closed, per `static/daisyui.min.css`'s
// own `.dropdown-content` rule) and whichever of the primary nav /
// hamburger menu is hidden at the current width (`md:flex`/`md:hidden`)
// are correctly never candidates. This generalises `§1`'s own finding
// (one specific pair) into a property of the row -- any future control
// added to this header that stops shrinking gets caught the same way,
// not just the one pair this round measured.

import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { launch } from './cdp.mjs';

const REPO_ROOT = new URL('..', import.meta.url).pathname;
const BIN_PATH = process.env.PEISEAR_BIN || join(REPO_ROOT, 'target/debug/peisear');
// `doc-screenshots.mjs`'s own pattern, for the same reason: this
// script's `createFixtures`/`buildPages` come from `overflow-gate.mjs`,
// whose own `BASE` is computed once, at module load, from
// `process.env.PEISEAR_PORT`. Setting the env var here and importing
// that module dynamically (in `main()`, below) is the only order in
// which setting it still reaches that constant -- a static `import` at
// the top of this file would load `overflow-gate.mjs` before this
// line runs.
const PORT = process.env.PEISEAR_PORT || '4177';
process.env.PEISEAR_PORT = PORT;
const BASE = `http://127.0.0.1:${PORT}`;

function log(...args) {
  console.log('[occlusion-gate]', ...args);
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

// Every pair of visible interactive controls inside `header.navbar`
// that share pixels. A pair "shares a row" by construction here: the
// header is itself one flex row (`static/daisyui.min.css`'s own
// `.navbar{display:flex}`) that never wraps (confirmed directly by
// `OCCL-001`'s own measurement at 768px, every direct child's own
// `top` identical), so every surviving candidate is already in it --
// no separate row-membership test is needed, and inventing one would
// be machinery this check does not need.
const MEASURE_OVERLAPS = `(() => {
  const header = document.querySelector('header.navbar');
  if (!header) return { error: 'no header.navbar found' };

  const isVisible = (el) => {
    const r = el.getBoundingClientRect();
    if (r.width === 0 || r.height === 0) return false;
    const cs = getComputedStyle(el);
    return cs.visibility !== 'hidden' && cs.display !== 'none';
  };

  const describe = (el) => {
    const label = el.getAttribute('aria-label') || el.textContent || el.getAttribute('name') || '';
    return \`\${el.tagName.toLowerCase()}[\${label.trim().slice(0, 40)}]\`;
  };

  const candidates = Array.from(header.querySelectorAll('a, button, input, label[tabindex]'));
  const visible = candidates.filter(isVisible).map(el => ({
    label: describe(el),
    rect: el.getBoundingClientRect(),
  }));

  const overlaps = [];
  for (let i = 0; i < visible.length; i++) {
    for (let j = i + 1; j < visible.length; j++) {
      const a = visible[i].rect, b = visible[j].rect;
      const overlapX = Math.min(a.right, b.right) - Math.max(a.left, b.left);
      const overlapY = Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top);
      if (overlapX > 0 && overlapY > 0) {
        overlaps.push({
          a: visible[i].label, b: visible[j].label,
          overlapX: Math.round(overlapX), overlapY: Math.round(overlapY),
        });
      }
    }
  }
  return { controlCount: visible.length, overlaps };
})()`;

async function main() {
  const dbDir = mkdtempSync(join(tmpdir(), 'occlusion-gate-db-'));
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
    log(`coverage: ${Object.keys(pages).length} pages x ${WIDTHS.length} widths`);

    browser = await launch({ width: 1280, height: 900 });
    const [cookieName, cookieValue] = fixture.cookie.split('=');
    await browser.send('Network.setCookie', {
      name: cookieName, value: cookieValue, domain: '127.0.0.1', path: '/',
    });

    const failures = [];
    let cellCount = 0;
    let minControlCount = Infinity;
    for (const [label, url] of Object.entries(pages)) {
      for (const [w, h, mobile] of WIDTHS) {
        cellCount++;
        await browser.setViewport(w, h, mobile);
        await browser.goto(url);
        await browser.waitForSelector('header.navbar', 5000);
        const r = await browser.eval(MEASURE_OVERLAPS);
        if (r.error) {
          failures.push({ page: label, width: w, error: r.error });
          continue;
        }
        minControlCount = Math.min(minControlCount, r.controlCount);
        for (const o of r.overlaps) {
          failures.push({ page: label, width: w, ...o });
        }
      }
    }

    log(`${cellCount} cells checked, >=${minControlCount} interactive header control(s) measured per cell`);
    if (failures.length > 0) {
      log(`${failures.length} FAILING CELL(S):`);
      for (const f of failures) {
        if (f.error) {
          log(`  ${f.page} @ ${f.width}px: ${f.error}`);
        } else {
          log(`  ${f.page} @ ${f.width}px: "${f.a}" overlaps "${f.b}" by ${f.overlapX}x${f.overlapY}px`);
        }
      }
    } else {
      log('0 failing cells');
    }

    exitCode = failures.length === 0 ? 0 : 1;
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
