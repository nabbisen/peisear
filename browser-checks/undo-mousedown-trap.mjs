// `DM-TEST-001` item 6 / `A11Y-005` — the regression this guards against
// a text scan cannot see: a real mousedown-plus-small-movement on the
// Undo button, on a surface whose acted-on element carries
// `draggable="true"`, can be read by the browser's own drag-gesture
// recognition as the start of a drag instead of a click — the click
// event on Undo then never fires at all. `A11Y-005`'s own measurement
// found this is not probabilistic ("may drag instead of"); it is
// deterministic the moment real movement is present, and no scan over
// `board.js`/`plan.js`/`calendar.js`'s source text can observe the
// browser's own drag-vs-click decision the way a real pointer event
// sequence can. `undo_dom_order` (`crates/peisear-web/tests/undo_dom_order.rs`)
// covers what a text scan *can* see — DOM placement; this covers what
// it cannot.
//
// **Not part of `overflow-gate.mjs`, and not wired into a CI job in
// this handoff.** `browser-checks/README.md`'s own rule is "one
// assertion only" for that gate and "do not widen this gate" (`DEC-048`
// condition 3) — this is a different property entirely, not a second
// assertion bolted onto the overflow sweep. Whether this becomes its
// own CI job is left to the architect; run it locally the same way
// `overflow-gate.mjs` is run.
//
// Reuses the exact technique the evidence script that found the
// original defect used (`.git-exclude/review-request/A11Y-005-toast-draggable/evidence/probe_drag_gesture.js`):
// real `Input.dispatchMouseEvent` — mousedown, several real
// mousemoves, mouseup, the same pipeline a real OS mouse event enters
// — not a synthetic in-page `MouseEvent`. The toast itself is produced
// by a synthetic `DragEvent` sequence (that is not the gesture under
// test; it only needs to complete a drop so a toast exists to test
// against), and `dm.js`'s surface is out of scope — it has no
// `draggable` ancestor and so no trap to guard against
// (`FR-DM-002-measurement-review.md` §3).

import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { launch } from './cdp.mjs';

const REPO_ROOT = new URL('..', import.meta.url).pathname;
const BIN_PATH = process.env.PEISEAR_BIN || join(REPO_ROOT, 'target/debug/peisear');
const PORT = process.env.PEISEAR_PORT || '4174';
const BASE = `http://127.0.0.1:${PORT}`;

function log(...args) {
  console.log('[undo-mousedown-trap]', ...args);
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

// Same tiny cookie jar `overflow-gate.mjs` uses.
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

async function optionValue(cookie, url, labelPart) {
  const html = await (await fetch(url, { headers: { cookie } })).text();
  for (const m of html.matchAll(/<option[^>]*value="([^"]*)"[^>]*>([^<]*)<\/option>/g)) {
    if (m[2].includes(labelPart) && m[1]) return m[1];
  }
  throw new Error(`no <option> containing "${labelPart}" on ${url}`);
}

async function lockStamp(jar, url) {
  const page = await (await fetch(url, { headers: { cookie: jar.header() } })).text();
  const stamp = page.match(/name="client_updated_at"\s+value="([^"]+)"/)?.[1];
  if (!stamp) throw new Error(`no lock value on ${url}`);
  return stamp;
}

const DISPLAY_NAME = 'Undo Trap Fixture';
const EMAIL = 'undotrapfixture@example.org';

async function createFixtures() {
  const jar = makeCookieJar();

  const registerRes = await jar.fetchForm(`${BASE}/register`, {
    display_name: DISPLAY_NAME,
    email: EMAIL,
    password: 'password1234',
  });
  if (registerRes.status !== 303) throw new Error(`register: expected 303, got ${registerRes.status}`);

  const teamRes = await jar.fetchForm(`${BASE}/teams`, { name: 'Undo Trap Team', slug: '', description: '' });
  if (teamRes.status !== 303) throw new Error(`create team: expected 303, got ${teamRes.status}`);
  const teamSlug = teamRes.headers.get('location').split('/teams/')[1].split(/[/?]/)[0];

  const sprintRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints`, {
    name: 'Undo Trap Sprint',
    goal: '',
    starts_on: '2026-09-01',
    ends_on: '2026-09-14',
  });
  if (sprintRes.status !== 303) throw new Error(`create sprint: expected 303, got ${sprintRes.status}`);
  const sprintId = sprintRes.headers.get('location').split('/sprints/')[1].split(/[/?]/)[0];

  const teamId = await optionValue(jar.header(), `${BASE}/projects/new`, 'Undo Trap Team');
  const projectRes = await jar.fetchForm(`${BASE}/projects`, {
    name: 'Undo Trap Project',
    description: '',
    team_id: teamId,
  });
  if (projectRes.status !== 303) throw new Error(`create project: expected 303, got ${projectRes.status}`);
  const projectId = projectRes.headers.get('location').split('/projects/')[1].split(/[/?]/)[0];

  // The board card: an ordinary open issue in the project.
  const boardIssueRes = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/new`, {
    title: 'Undo trap board issue',
    description: '',
    status: 'open',
    priority: 'medium',
    effort: '',
    assignee_id: '',
  });
  if (boardIssueRes.status !== 303) throw new Error(`create board issue: expected 303, got ${boardIssueRes.status}`);

  // The sprint-plan row: a second open issue, left in the backlog
  // (not added to the sprint) -- `can_move` is a per-page flag (role
  // + sprint status), not per-row, so a backlog row is as good a
  // draggable row as a committed one.
  const planIssueRes = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/new`, {
    title: 'Undo trap plan issue',
    description: '',
    status: 'open',
    priority: 'medium',
    effort: '',
    assignee_id: '',
  });
  if (planIssueRes.status !== 303) throw new Error(`create plan issue: expected 303, got ${planIssueRes.status}`);

  // The calendar block: assigned to self, in progress, planned today
  // -- same two-step shape `overflow-gate.mjs`'s GATE-002 fixture
  // uses (the create form takes no planned window; the edit route
  // does).
  const ownerId = await optionValue(jar.header(), `${BASE}/projects/${projectId}/issues/new`, DISPLAY_NAME);
  const calIssueRes = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/new`, {
    title: 'Undo trap calendar issue',
    description: '',
    status: 'in_progress',
    priority: 'medium',
    effort: '',
    assignee_id: ownerId,
  });
  if (calIssueRes.status !== 303) throw new Error(`create calendar issue: expected 303, got ${calIssueRes.status}`);
  const calIssueId = calIssueRes.headers.get('location').split('/issues/')[1].split(/[/?]/)[0];
  const today = new Date().toISOString().slice(0, 10);
  const planStamp = await lockStamp(jar, `${BASE}/projects/${projectId}/issues/${calIssueId}/edit`);
  const scheduleRes = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/${calIssueId}`, {
    title: 'Undo trap calendar issue',
    description: '',
    status: 'in_progress',
    priority: 'medium',
    effort: '',
    assignee_id: ownerId,
    planned_start_at: `${today}T09:00`,
    planned_end_at: `${today}T10:00`,
    client_updated_at: planStamp,
  });
  if (scheduleRes.status !== 303) throw new Error(`schedule calendar issue: expected 303, got ${scheduleRes.status}`);

  return { cookie: jar.header(), projectId, teamSlug, sprintId };
}

const ARM_DRAG_LISTENER = `(() => {
  window.__dragStarts = [];
  window.__undoClicks = 0;
  document.addEventListener('dragstart', (e) => {
    window.__dragStarts.push(e.target.tagName.toLowerCase());
  }, true);
  return 1;
})()`;

// Synthetic `DragEvent` sequences that complete a drop -- producing a
// real undo toast the same way `A11Y-005`'s own evidence did. This is
// not the gesture under test.
const PRODUCE = {
  board: `(() => {
    const s = document.querySelector('.issue-card');
    const cols = [...document.querySelectorAll('.column-drop')];
    const from = s.closest('[data-status]')?.dataset.status;
    const d = cols.find(c => c.closest('[data-status]')?.dataset.status !== from) || cols[1] || cols[0];
    const dt = new DataTransfer();
    const ev = (t, el) => el.dispatchEvent(new DragEvent(t, { bubbles: true, cancelable: true, dataTransfer: dt }));
    ev('dragstart', s); ev('dragover', d); ev('drop', d); ev('dragend', s);
    return 1;
  })()`,
  plan: `(() => {
    const s = document.querySelector('[data-plan-move]');
    const here = s.closest('[data-plan-drop]').dataset.planDrop;
    const d = document.querySelector('[data-plan-drop="' + (here === 'add' ? 'remove' : 'add') + '"]');
    const dt = new DataTransfer();
    const ev = (t, el) => el.dispatchEvent(new DragEvent(t, { bubbles: true, cancelable: true, dataTransfer: dt }));
    ev('dragstart', s); ev('dragover', d); ev('drop', d); ev('dragend', s);
    return 1;
  })()`,
  calendar: `(() => {
    const s = document.querySelector('#day-view [data-issue-id]');
    const d = document.getElementById('day-view');
    const dt = new DataTransfer();
    const at = (t, el, y) => el.dispatchEvent(new DragEvent(t, { bubbles: true, cancelable: true, dataTransfer: dt, clientX: 200, clientY: y }));
    at('dragstart', s, 300); at('dragover', d, 380); at('drop', d, 380); at('dragend', s, 380);
    return 1;
  })()`,
};

const HOLDER_SEL = {
  board: '.issue-card:has(.toast)',
  plan: '[data-plan-move]:has(.toast)',
  calendar: '#day-view [data-issue-id]:has(.toast)',
};

async function realMouseGesture(browser, dx, dy, steps) {
  const btn = await browser.eval(
    `(() => { const b = document.querySelector('.toast button'); const r = b.getBoundingClientRect(); return { x: Math.round(r.left + r.width / 2), y: Math.round(r.top + r.height / 2) }; })()`,
  );
  await browser.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: btn.x, y: btn.y });
  await browser.send('Input.dispatchMouseEvent', { type: 'mousePressed', x: btn.x, y: btn.y, button: 'left', clickCount: 1 });
  for (let i = 1; i <= steps; i++) {
    await browser.send('Input.dispatchMouseEvent', {
      type: 'mouseMoved',
      x: btn.x + Math.round((dx * i) / steps),
      y: btn.y + Math.round((dy * i) / steps),
      button: 'left',
    });
    await new Promise(r => setTimeout(r, 30));
  }
  await browser.send('Input.dispatchMouseEvent', { type: 'mouseReleased', x: btn.x + dx, y: btn.y + dy, button: 'left', clickCount: 1 });
}

async function checkSurface(browser, name, url) {
  await browser.setViewport(1280, 900, false);
  await browser.goto(url);
  await browser.waitForSelector('header.navbar', 5000);
  await browser.eval(ARM_DRAG_LISTENER);
  await browser.eval(PRODUCE[name]);
  await new Promise(r => setTimeout(r, 300));
  await browser.waitForSelector(HOLDER_SEL[name], 2000);

  // Reset the drag log AFTER producing the toast (which itself
  // dispatches a synthetic `dragstart` to complete the drop) and
  // arm a click listener on the real button, so a suppressed click
  // is directly observed, not inferred.
  await browser.eval(`(() => {
    window.__dragStarts = [];
    const b = document.querySelector('.toast button');
    b.addEventListener('click', () => { window.__undoClicks++; });
    return 1;
  })()`);

  await realMouseGesture(browser, 8, 4, 6);
  await new Promise(r => setTimeout(r, 400));

  const after = await browser.eval(
    `(() => ({ dragStarts: window.__dragStarts.slice(), undoClicks: window.__undoClicks }))()`,
  );
  return { surface: name, dragStarted: after.dragStarts.length > 0, undoClickFired: after.undoClicks > 0 };
}

async function main() {
  const dbDir = mkdtempSync(join(tmpdir(), 'undo-trap-db-'));
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

    const { cookie, projectId, teamSlug, sprintId } = await createFixtures();
    log(`fixtures created: project=${projectId} team=${teamSlug} sprint=${sprintId}`);

    const urls = {
      board: `${BASE}/projects/${projectId}`,
      plan: `${BASE}/teams/${teamSlug}/sprints/${sprintId}/plan`,
      calendar: `${BASE}/today/calendar?view=day`,
    };

    browser = await launch({ width: 1280, height: 900 });
    const [cookieName, cookieValue] = cookie.split('=');
    await browser.send('Network.setCookie', { name: cookieName, value: cookieValue, domain: '127.0.0.1', path: '/' });

    const results = [];
    for (const [name, url] of Object.entries(urls)) {
      results.push(await checkSurface(browser, name, url));
    }

    const failures = results.filter(r => r.dragStarted || !r.undoClickFired);
    for (const r of results) {
      log(`${r.surface}: dragStarted=${r.dragStarted} undoClickFired=${r.undoClickFired}`);
    }
    if (failures.length > 0) {
      log(`${failures.length} FAILING SURFACE(S) -- a real mousedown + 8px move on Undo started a drag, or Undo's click never fired`);
    } else {
      log('all three draggable surfaces: no drag started, Undo click fired');
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

main();
