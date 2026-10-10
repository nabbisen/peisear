// `DOCS-004` (owner request, 2026-10-11): documentation screenshots,
// generated from the same harness `overflow-gate.mjs` already drives,
// rather than hand-captured and pasted in. **Not a gate** -- it asserts
// nothing, exits 0 whenever the pages it visits render at all, and must
// not be added to the `browser-overflow-gate` CI job. `browser-checks/`'s
// own README explains why that distinction matters for this directory.
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

    const { cookie, projectId, issueId, teamSlug, sprintId } = await createFixtures();
    log(`fixtures created: project=${projectId} issue=${issueId}`);

    // Five pages, matching `DOCS-004` §1's table: the board (the
    // product's signature screen), the list view (the dense
    // alternative), an issue detail (where most time is spent), /today
    // (the personal dashboard, which nothing else illustrates), and the
    // sprint plan (one planning surface).
    const pages = {
      board: `${BASE}/projects/${projectId}`,
      list: `${BASE}/projects/${projectId}?view=list`,
      issue_detail: `${BASE}/projects/${projectId}/issues/${issueId}`,
      today: `${BASE}/today`,
      sprint_plan: `${BASE}/teams/${teamSlug}/sprints/${sprintId}/plan`,
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
      await browser.screenshot(path, { x: 0, y: 0, width: VIEWPORT.width, height: VIEWPORT.height });
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
