// `DOCS-004` (owner request, 2026-10-11): documentation screenshots,
// generated from the same harness `overflow-gate.mjs` already drives,
// rather than hand-captured and pasted in. **Not a gate** -- it asserts
// nothing, exits 0 whenever the pages it visits render at all, and must
// not be added to the `browser-overflow-gate` CI job. `browser-checks/`'s
// own README explains why that distinction matters for this directory.
// `DOCS-007` closed `DOCS-004`'s own escalation (the fixture's
// adversarial strings made every capture unusable) -- see that handoff
// for the ruling this file now implements.
//
// **Reuses `overflow-gate.mjs`'s `createFixtures` rather than building a
// second copy of the registration/team/sprint/project/issue flow** --
// that flow going through the real HTTP forms is the hard part, and it
// is already written and already proven. This script's own job is only
// to spawn a scratch server on its own port (`4176` by default, distinct
// from the three gates' `4173`/`4174`/`4175`, per this directory's own
// warning about `PEISEAR_PORT` collisions), call that fixture builder,
// and capture images instead of measuring overflow.
//
// A dynamic `import()` after setting `PEISEAR_PORT`, not a static one --
// `overflow-gate.mjs`'s own `BASE` constant is computed once, at module
// load, from `process.env.PEISEAR_PORT`. A static `import` at the top of
// this file would load that module (and fix its port) before this
// file's own first line runs, so there would be no point at which
// setting the env var here could still reach it.
//
import { spawn } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { launch } from './cdp.mjs';

const REPO_ROOT = new URL('..', import.meta.url).pathname;
const BIN_PATH = process.env.PEISEAR_BIN || join(REPO_ROOT, 'target/debug/peisear');
const PORT = process.env.PEISEAR_PORT || '4176';
process.env.PEISEAR_PORT = PORT;
const OUT_DIR = process.env.DOC_SCREENSHOT_DIR || join(REPO_ROOT, 'docs/src/assets/screenshots');

// `DOCS-007` §1: `createFixtures` now takes the fields that render on
// these pages as a parameter -- the gate passes its own adversarial set
// (`overflow-gate.mjs`'s own `ADVERSARIAL_STRINGS`, explicitly, at its
// own call site); this script passes `PLAIN_STRINGS` below instead of
// building a second fixture function. `DOCS-008` §1 found three of
// these fields were still fixture-flavoured even after `DOCS-007`'s own
// pass (`projectDescription`, `firstIssueTitle`, `assignedIssueTitle`)
// -- re-derived by reading the regenerated images, not by trusting the
// review's own three; §2 added `secondIssueDescription` and the
// optional `secondIssueSubIssueTitle`, so the issue-detail capture no
// longer contradicts `first-run.md`'s own "a description, and a place
// for sub-issues." `DOCS-007` §4.1 (owner instruction, 2026-10-11:
// nothing committed as an image may carry anything private or local):
// every value below is invented, not a real person, organisation or
// address -- the email stays on `example.com`, a reserved domain, and
// the display name reads as a fixture ("Demo Account") rather than as a
// plausible real person, per that instruction's own preference for the
// former over a plain name that merely *looks* real.
const PLAIN_STRINGS = {
  displayName: 'Demo Account',
  email: 'demo@example.com',
  teamName: 'Product Team',
  projectName: 'Website Launch',
  projectDescription: 'Tracking the launch of the redesigned marketing site.',
  sprintName: 'Sprint 12',
  sprintGoal: 'Ship the new checkout flow',
  firstIssueTitle: 'Write release notes for the launch',
  secondIssueTitle: 'Checkout page times out on mobile Safari',
  secondIssueDescription:
    'Customers on iOS report the checkout page hanging indefinitely after ' +
    'entering payment details. Reproduced on Safari 17 and 18; Chrome is unaffected.',
  secondIssueSubIssueTitle: 'Confirm whether the timeout is client-side or server-side',
  assignedIssueTitle: 'Review the new navigation design',
  plannedIssueTitle: 'Fix pagination on the activity feed',
};

// One width: a desktop width for the README and `first-run.md`, not the
// five the overflow gate sweeps -- that gate's widths exist to find
// overflow; documentation needs the screens a reader recognises, at the
// width most readers' own browsers actually render at.
const VIEWPORT = { width: 1280, height: 900 };

function log(...args) {
  console.log('[doc-screenshots]', ...args);
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

async function main() {
  const { createFixtures, BASE } = await import('./overflow-gate.mjs');

  // `DOCS-007` §4.1: a fresh, throwaway database every run, and
  // `DATABASE_URL` below is explicit in the child's own `env` object --
  // placed after `...process.env` so it always wins regardless of what
  // the invoking shell has set. That pin is load-bearing:
  // `Config::from_env` calls `dotenvy::dotenv()`, and dotenvy 0.15's
  // `dotenv()` does not override an already-set variable, so a
  // developer's own `.env` cannot redirect a capture at a real
  // database. Do not add a `DATABASE_URL` passthrough here.
  const dbDir = mkdtempSync(join(tmpdir(), 'doc-screenshots-db-'));
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

    const { cookie, projectId, issueId } = await createFixtures(PLAIN_STRINGS);
    log(`fixtures created: project=${projectId} issue=${issueId}`);

    // Three pages (`DOCS-007` §3 dropped `/today`; this round drops the
    // sprint plan for the same class of reason -- see the review
    // package's own finding): the board (the product's signature
    // screen), the list view (the dense alternative), and an issue
    // detail (where most time is spent).
    const pages = {
      board: `${BASE}/projects/${projectId}`,
      list: `${BASE}/projects/${projectId}?view=list`,
      issue_detail: `${BASE}/projects/${projectId}/issues/${issueId}`,
    };

    mkdirSync(OUT_DIR, { recursive: true });

    browser = await launch(VIEWPORT);
    const [cookieName, cookieValue] = cookie.split('=');
    await browser.send('Network.setCookie', {
      name: cookieName, value: cookieValue, domain: '127.0.0.1', path: '/',
    });
    await browser.setViewport(VIEWPORT.width, VIEWPORT.height, false);

    const written = [];
    for (const [label, url] of Object.entries(pages)) {
      await browser.goto(url);
      await browser.waitForSelector('header.navbar', 5000);
      const path = join(OUT_DIR, `${label}.png`);
      // `DOCS-007` §2: no `clip` argument -- `cdp.mjs`'s `screenshot()`
      // only applies its `scale: 3` (sized for evidence-log clarity)
      // when a `clip` is given. Without one, `Page.captureScreenshot`
      // captures the viewport at its natural 1x, which is both smaller
      // and the right resolution for a README or book image.
      await browser.screenshot(path);
      const bytes = statSync(path).size;
      written.push({ label, path, bytes });
      log(`captured ${label} -> ${path} (${bytes} bytes)`);
    }

    const totalBytes = written.reduce((sum, w) => sum + w.bytes, 0);
    log(`${written.length} screenshots, ${totalBytes} bytes total`);
    exitCode = 0;
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

main();
