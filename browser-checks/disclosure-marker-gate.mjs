// `DISC-001` (`§10.58`) -- a disclosure control must look like one and
// keep its 44x44 target. `grow_text()` -- `inline-flex items-center` --
// suppresses a `<summary>`'s native `::marker` box, since a marker only
// exists on `display: list-item`; `touch_target_scan` cannot see this,
// because it reads which helper a `<summary>` was given, never the
// computed style the browser resolves from it. This is the browser check
// that can.
//
// **Scoped to what this round built**: the project board/list's own
// three summaries -- `HealthStrip`'s outer one, `WorkloadStrip`'s outer
// one, and the nested `Indicators` toggle -- all in
// `components/issues.rs`, all switched from `grow_text(...)` to
// `grow(...)` plus padding so the native `list-item` display (and its
// marker) survive. A guard asserting every `<summary>` in the product is
// correct would fail today on sites this round did not touch (found
// while measuring for this file, reported rather than fixed -- see the
// round's own completion report) -- exactly the "fails on a tree nobody
// asked it to cover" shape this project's own guards are written not to
// be.
//
// **Two properties, read together, per the handoff's own instruction**:
// a marker that costs the touch target is not the fix, and a target
// that costs the marker is the defect. Both come off the live
// `<summary>` directly -- no ancestor needs opening first.
// `getComputedStyle` on a `<summary>` is accurate even while its own
// parent `<details>` is closed: the summary is the one child a closed
// `<details>` still genuinely renders (it is the control that opens it),
// so its own box is never subject to the stale-closed-layout quirk
// `VIEW-001` found for a closed subtree's *other* descendants.
//
// **Not wired into a CI job in this handoff** -- the same posture
// `undo-mousedown-trap.mjs` states for itself: a single-purpose script
// on the CDP harness `overflow-gate.mjs` already uses, run locally the
// same way. Whether this becomes a permanent job is the architect's own
// call.

import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { launch } from './cdp.mjs';

const REPO_ROOT = new URL('..', import.meta.url).pathname;
const BIN_PATH = process.env.PEISEAR_BIN || join(REPO_ROOT, 'target/debug/peisear');
const PORT = process.env.PEISEAR_PORT || '4233';
const BASE = `http://127.0.0.1:${PORT}`;

function log(...args) {
  console.log('[disclosure-marker-gate]', ...args);
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

function makeCookieJar() {
  let cookie = '';
  return {
    async fetchForm(url, body) {
      const r = await fetch(url, {
        method: 'POST',
        headers: { 'content-type': 'application/x-www-form-urlencoded', ...(cookie ? { cookie } : {}) },
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

async function optionValue(cookie, url, labelPart) {
  const html = await (await fetch(url, { headers: { cookie } })).text();
  for (const m of html.matchAll(/<option[^>]*value="([^"]*)"[^>]*>([^<]*)<\/option>/g)) {
    if (m[2].includes(labelPart) && m[1]) return m[1];
  }
  throw new Error(`no <option> containing "${labelPart}" on ${url}`);
}

// Two members, several in-flight issues on one assignee -- the same
// fixture shape `VIEW-001`'s own evidence used, so both `HealthStrip`
// (a non-Good indicator) and `WorkloadStrip` (an assigned in-flight
// issue) actually render their own summary rather than an empty state.
async function createFixture() {
  const jar = makeCookieJar();
  const DISPLAY_NAME = 'Disclosure Marker Fixture';
  const EMAIL = 'disclosuremarkerfixture@example.org';
  const SECOND_DISPLAY_NAME = 'Disclosure Marker Teammate';
  const SECOND_EMAIL = 'disclosuremarkerteammate@example.org';

  const registerRes = await jar.fetchForm(`${BASE}/register`, {
    display_name: DISPLAY_NAME, email: EMAIL, password: 'password1234',
  });
  if (registerRes.status !== 303) throw new Error(`register: expected 303, got ${registerRes.status}`);
  const ownerCookie = jar.header();

  const teamRes = await jar.fetchForm(`${BASE}/teams`, { name: 'Disclosure Marker Team', slug: '', description: '' });
  if (teamRes.status !== 303) throw new Error(`create team: expected 303, got ${teamRes.status}`);
  const teamSlug = teamRes.headers.get('location').split('/teams/')[1].split(/[/?]/)[0];

  const secondJar = makeCookieJar();
  const secondRes = await secondJar.fetchForm(`${BASE}/register`, {
    display_name: SECOND_DISPLAY_NAME, email: SECOND_EMAIL, password: 'password1234',
  });
  if (secondRes.status !== 303) throw new Error(`register second user: expected 303, got ${secondRes.status}`);
  const addRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/members`, { email: SECOND_EMAIL, role: 'member' });
  if (addRes.status !== 303) throw new Error(`add second member: expected 303, got ${addRes.status}`);

  const teamId = await optionValue(ownerCookie, `${BASE}/projects/new`, 'Disclosure Marker Team');
  const projectRes = await jar.fetchForm(`${BASE}/projects`, {
    name: 'Disclosure Marker Project', description: '', team_id: teamId,
  });
  if (projectRes.status !== 303) throw new Error(`create project: expected 303, got ${projectRes.status}`);
  const projectId = projectRes.headers.get('location').split('/projects/')[1].split(/[/?]/)[0];

  const ownerId = await optionValue(ownerCookie, `${BASE}/projects/${projectId}/issues/new`, DISPLAY_NAME);

  for (let i = 0; i < 4; i++) {
    const res = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/new`, {
      title: `Disclosure marker fixture issue ${i}`,
      description: '', status: 'in_progress', priority: 'medium', effort: '2', assignee_id: ownerId,
    });
    if (res.status !== 303) throw new Error(`create issue ${i}: expected 303, got ${res.status}`);
  }

  return { cookie: ownerCookie, projectId };
}

// The three summaries this round rewrote, found by stable anchors --
// never the headline's own computed sentence, which depends on which
// indicator is driving it and is not this round's subject. `health`/
// `workload` are each the first `<summary>` inside their section's own
// `aria-label` (`VIEW-001`'s own stable anchor, reused here); `Indicators`
// is a fixed label (`IndicatorsSummaryLabel`), never data-dependent.
const IN_SCOPE = {
  health: `document.querySelector('[aria-label="Project health"] summary')`,
  workload: `document.querySelector('[aria-label="Workload"] summary')`,
  indicators: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.trim() === 'Indicators')`,
};

async function measureSummary(browser, expr) {
  return browser.eval(`(() => {
    const s = ${expr};
    if (!s) return null;
    const r = s.getBoundingClientRect();
    return {
      text: s.textContent.trim(),
      display: getComputedStyle(s).display,
      width: Math.round(r.width),
      height: Math.round(r.height),
    };
  })()`);
}

async function checkWidth(browser, { projectId }, viewMode, width, height) {
  await browser.setViewport(width, height, width < 768);
  const url = `${BASE}/projects/${projectId}?view=${viewMode}`;
  await browser.goto(url);
  await browser.waitForSelector('header.navbar', 5000);

  for (const [label, expr] of Object.entries(IN_SCOPE)) {
    const found = await measureSummary(browser, expr);
    if (!found) {
      throw new Error(`${viewMode}@${width}: ${label} summary not found on the page`);
    }
    if (found.display !== 'list-item') {
      throw new Error(
        `${viewMode}@${width}: ${label} summary (${JSON.stringify(found.text)}) has computed display ` +
        `${JSON.stringify(found.display)}, not "list-item" -- its ::marker is suppressed`,
      );
    }
    if (found.width < 44 || found.height < 44) {
      throw new Error(
        `${viewMode}@${width}: ${label} summary (${JSON.stringify(found.text)}) measures ` +
        `${found.width}x${found.height}, below the 44x44 floor`,
      );
    }
    log(`${viewMode}@${width}: ${label} -- display=list-item, ${found.width}x${found.height}`);
  }
}

async function main() {
  const dbDir = mkdtempSync(join(tmpdir(), 'disclosure-marker-db-'));
  const dbPath = join(dbDir, 'app.db');

  log(`starting ${BIN_PATH} on ${BASE}, db=${dbPath}`);
  const server = spawn(BIN_PATH, [], {
    env: { ...process.env, DATABASE_URL: `sqlite://${dbPath}`, BIND_ADDR: `127.0.0.1:${PORT}`, COOKIE_SECURE: 'false' },
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

    const fixture = await createFixture();
    log(`fixture created: project=${fixture.projectId}`);

    browser = await launch({ width: 1280, height: 900 });
    const [cookieName, cookieValue] = fixture.cookie.split('=');
    await browser.send('Network.setCookie', { name: cookieName, value: cookieValue, domain: '127.0.0.1', path: '/' });

    for (const viewMode of ['board', 'list']) {
      await checkWidth(browser, fixture, viewMode, 1280, 900);
      await checkWidth(browser, fixture, viewMode, 390, 844);
    }

    log('all three in-scope summaries: list-item display, >=44x44, on board and list, at both widths');
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
