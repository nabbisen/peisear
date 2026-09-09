// BROWSER-001 (RFC 011 step 4) -- the horizontal-overflow gate.
//
// One assertion, on rendered pages: `scrollWidth <= clientWidth`.
// Deliberately not target-size, not overlap, not JavaScript -- see
// `rfcs/handoffs/011-browser-verification/BROWSER-001-overflow-gate.md`
// §1 for why. This is a gate, not a Rust test: it does not run inside
// `cargo test --workspace` and is not counted in `DEC-007`'s own
// inventory (§2 of the same handoff) -- a CI job drives this script
// directly.
//
// `DEC-048` condition 3: no silent retries. This script runs the
// sweep exactly once; the workflow that calls it must not wrap it in
// a retry step. A flake here is a defect with an owner (see this
// directory's own README for where that's recorded), not something
// to re-run past.

import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { launch } from './cdp.mjs';

const REPO_ROOT = new URL('..', import.meta.url).pathname;
const BIN_PATH = process.env.PEISEAR_BIN || join(REPO_ROOT, 'target/debug/peisear');
const PORT = process.env.PEISEAR_PORT || '4173';
const BASE = `http://127.0.0.1:${PORT}`;
const WIDTHS = [[390, 844, true], [768, 1024, false], [1280, 900, false], [1920, 1080, false]];

// A long-enough, unbreakable local part -- `LAYOUT-001` was
// conditional on exactly this shape (no hyphen for the browser's
// default word-wrap to break at), so the fixture must be too, or the
// gate would not have caught the defect it exists to catch.
const LONG_EMAIL = 'browseroverflowgatefixtureaccount@example.org';
const LONG_ISSUE_TITLE =
  'An issue whose title is long enough on its own to be the kind of content that made §10.21 invisible on empty fixtures';

function log(...args) {
  console.log('[overflow-gate]', ...args);
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

// Tiny cookie jar -- Node's `fetch` doesn't persist cookies across
// calls the way a browser or `curl -c/-b` does. One session's worth
// is all this script needs.
function makeCookieJar() {
  let cookie = '';
  return {
    async fetchForm(url, body) {
      const r = await fetch(url, {
        method: 'POST',
        headers: {
          'content-type': 'application/x-www-form-urlencoded',
          ...(cookie ? { cookie } : {}),
        },
        body: new URLSearchParams(body).toString(),
        redirect: 'manual',
      });
      const setCookie = r.headers.get('set-cookie');
      if (setCookie) cookie = setCookie.split(';')[0];
      return r;
    },
    header() {
      return cookie;
    },
  };
}

async function createFixtures() {
  const jar = makeCookieJar();

  const registerRes = await jar.fetchForm(`${BASE}/register`, {
    display_name: 'Overflow Gate Fixture',
    email: LONG_EMAIL,
    password: 'password1234',
  });
  if (registerRes.status !== 303) {
    throw new Error(`register: expected 303, got ${registerRes.status}`);
  }

  const teamRes = await jar.fetchForm(`${BASE}/teams`, {
    name: 'Overflow Gate Team',
    slug: '',
    description: '',
  });
  if (teamRes.status !== 303) throw new Error(`create team: expected 303, got ${teamRes.status}`);

  const projectRes = await jar.fetchForm(`${BASE}/projects`, {
    name: 'Overflow Gate Project',
    description: 'Fixture project for the BROWSER-001 overflow gate.',
    team_id: '',
  });
  if (projectRes.status !== 303) throw new Error(`create project: expected 303, got ${projectRes.status}`);
  const projectLocation = projectRes.headers.get('location');
  const projectId = projectLocation.split('/projects/')[1].split(/[/?]/)[0];

  // Two issues: an ordinary one, and one with a long title, so
  // §10.21's own lesson (invisible on empty fixtures) is built in
  // rather than left to be rediscovered.
  const issue1Res = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/new`, {
    title: 'An ordinary fixture issue',
    description: 'Some description text for the overflow gate fixtures.',
    status: 'open',
    priority: 'medium',
    effort: '3',
    assignee_id: '',
  });
  if (issue1Res.status !== 303) throw new Error(`create issue 1: expected 303, got ${issue1Res.status}`);

  const issue2Res = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/new`, {
    title: LONG_ISSUE_TITLE,
    description: '',
    status: 'in_progress',
    priority: 'high',
    effort: '5',
    assignee_id: '',
  });
  if (issue2Res.status !== 303) throw new Error(`create issue 2: expected 303, got ${issue2Res.status}`);
  const issue2Location = issue2Res.headers.get('location');
  const issueId = issue2Location.split('/issues/')[1].split(/[/?]/)[0];

  return { cookie: jar.header(), projectId, issueId };
}

async function main() {
  const dbDir = mkdtempSync(join(tmpdir(), 'overflow-gate-db-'));
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

    const { cookie, projectId, issueId } = await createFixtures();
    log(`fixtures created: project=${projectId} issue=${issueId}`);

    const pages = {
      today: `${BASE}/today`,
      inbox: `${BASE}/inbox`,
      calendar: `${BASE}/today/calendar`,
      projects: `${BASE}/projects`,
      project_detail: `${BASE}/projects/${projectId}`,
      board: `${BASE}/projects/${projectId}?view=list`,
      issue_detail: `${BASE}/projects/${projectId}/issues/${issueId}`,
      settings: `${BASE}/settings`,
      settings_notifications: `${BASE}/settings/notifications`,
      teams: `${BASE}/teams`,
      search: `${BASE}/search?q=fixture`,
      delete_interstitial: `${BASE}/projects/${projectId}/issues/${issueId}/delete`,
    };
    log(`coverage: ${Object.keys(pages).length} pages x ${WIDTHS.length} widths`);

    browser = await launch({ width: 1280, height: 900 });
    const [cookieName, cookieValue] = cookie.split('=');
    await browser.send('Network.setCookie', {
      name: cookieName, value: cookieValue, domain: '127.0.0.1', path: '/',
    });

    const blocker = await browser.blockNonLocalRequests();

    const failures = [];
    let cellCount = 0;
    for (const [label, url] of Object.entries(pages)) {
      for (const [w, h, mobile] of WIDTHS) {
        cellCount++;
        await browser.setViewport(w, h, mobile);
        await browser.goto(url);
        await browser.waitForSelector('header.navbar', 5000);
        const r = await browser.eval(`(() => {
          const de = document.documentElement;
          return {clientW: de.clientWidth, scrollW: de.scrollWidth, overflow: de.scrollWidth - de.clientWidth};
        })()`);
        if (r.overflow !== 0) {
          failures.push({ page: label, width: w, ...r });
        }
      }
    }

    const blockedUrls = await blocker.stop();

    log(`${cellCount} cells checked`);
    if (failures.length > 0) {
      log(`${failures.length} FAILING CELL(S):`);
      for (const f of failures) {
        log(`  ${f.page} @ ${f.width}px: clientWidth=${f.clientW} scrollWidth=${f.scrollW} overflow=${f.overflow}`);
      }
    } else {
      log('0 failing cells');
    }

    if (blockedUrls.length > 0) {
      log(`DEC-048 condition 1 VIOLATION -- ${blockedUrls.length} request(s) to a non-local origin:`);
      for (const u of blockedUrls) log(`  ${u}`);
    } else {
      log('0 requests to any non-local origin');
    }

    exitCode = (failures.length === 0 && blockedUrls.length === 0) ? 0 : 1;
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
