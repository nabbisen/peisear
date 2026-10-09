// `GATE-006` (`NFR-A11Y-008`, `§10.15`, `§10.35`) -- a third gate in the same
// job, not a second assertion on either existing one (`DEC-048` condition 3's
// purpose, not its letter: it names `overflow-gate.mjs`, but widening
// `undo-mousedown-trap.mjs` in place with unrelated assertions would be the
// same shape).
//
// Three properties, from one drop per surface, on the two surfaces `GATE-005`
// measured as actually untested: `calendar.js` and `plan.js`. (`board.js`'s
// pair is already covered by `status_control.rs`'s HTTP test asserting the
// regions' presence; `dm.js` has no drag source at all.)
//
//   1. The live region actually receives the outcome text -- not merely that
//      the element exists. This is why the script exists: `NFR-A11Y-008`
//      cannot otherwise cite a mechanism matching its text.
//   2. The undo toast's own Tab order, measured rather than inferred --
//      `COV-001`'s own standing gap, open since 0.46.0.
//   3. The drop's stored effect, read back over HTTP -- independent of the
//      page's own optimistic DOM state, which a correct client updates from
//      the same response this reads a second time.
//
// Ordered 3, then 1, then 2 within each surface, per `GATE-006` §2: a region
// that stays empty because the handler never ran is indistinguishable, from
// the region assertion's own side, from a handler that ran and announced
// nothing. Asserting the stored effect first makes a setup failure (the
// synthetic drop not reaching the handler at all) legible as its own failure
// rather than reading as a defect in the announcement.
//
// Reuses `undo-mousedown-trap.mjs`'s own synthetic-`DragEvent` scaffolding
// verbatim for `PRODUCE.calendar`/`PRODUCE.plan` -- proven already, in a
// currently-passing gate, to reach both scripts' real `drop` handlers. No new
// CDP capability: `Input.dispatchKeyEvent` (candidate 2) goes through
// `cdp.mjs`'s existing generic `send`, the same shape
// `Input.dispatchMouseEvent` already uses there.

import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { launch } from './cdp.mjs';

const REPO_ROOT = new URL('..', import.meta.url).pathname;
const BIN_PATH = process.env.PEISEAR_BIN || join(REPO_ROOT, 'target/debug/peisear');
const PORT = process.env.PEISEAR_PORT || '4175';
const BASE = `http://127.0.0.1:${PORT}`;

function log(...args) {
  console.log('[drag-outcome-gate]', ...args);
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

// Same tiny cookie jar `overflow-gate.mjs`/`undo-mousedown-trap.mjs` use.
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
    async get(url) {
      return fetch(url, { headers: { cookie } });
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
  const page = await (await jar.get(url)).text();
  const stamp = page.match(/name="client_updated_at"\s+value="([^"]+)"/)?.[1];
  if (!stamp) throw new Error(`no lock value on ${url}`);
  return stamp;
}

async function plannedStartAt(jar, url) {
  const page = await (await jar.get(url)).text();
  const value = page.match(/name="planned_start_at"\s+value="([^"]*)"/)?.[1];
  if (value === undefined) throw new Error(`no planned_start_at field on ${url}`);
  return value;
}

const DISPLAY_NAME = 'Drag Outcome Fixture';
const EMAIL = 'dragoutcomefixture@example.org';

async function createFixtures() {
  const jar = makeCookieJar();

  const registerRes = await jar.fetchForm(`${BASE}/register`, {
    display_name: DISPLAY_NAME,
    email: EMAIL,
    password: 'password1234',
  });
  if (registerRes.status !== 303) throw new Error(`register: expected 303, got ${registerRes.status}`);

  const teamRes = await jar.fetchForm(`${BASE}/teams`, { name: 'Drag Outcome Team', slug: '', description: '' });
  if (teamRes.status !== 303) throw new Error(`create team: expected 303, got ${teamRes.status}`);
  const teamSlug = teamRes.headers.get('location').split('/teams/')[1].split(/[/?]/)[0];

  const sprintRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints`, {
    name: 'Drag Outcome Sprint',
    goal: '',
    starts_on: '2026-09-01',
    ends_on: '2026-09-14',
  });
  if (sprintRes.status !== 303) throw new Error(`create sprint: expected 303, got ${sprintRes.status}`);
  const sprintId = sprintRes.headers.get('location').split('/sprints/')[1].split(/[/?]/)[0];

  const teamId = await optionValue(jar.header(), `${BASE}/projects/new`, 'Drag Outcome Team');
  const projectRes = await jar.fetchForm(`${BASE}/projects`, {
    name: 'Drag Outcome Project',
    description: '',
    team_id: teamId,
  });
  if (projectRes.status !== 303) throw new Error(`create project: expected 303, got ${projectRes.status}`);
  const projectId = projectRes.headers.get('location').split('/projects/')[1].split(/[/?]/)[0];

  // The plan row: an open issue left in the backlog (not added to the
  // sprint). `PRODUCE.plan` below always drags toward the opposite of a
  // row's current column, so starting in the backlog makes the drop's
  // direction ("moved to the sprint") deterministic.
  const planIssueRes = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/new`, {
    title: 'Drag outcome plan issue',
    description: '',
    status: 'open',
    priority: 'medium',
    effort: '',
    assignee_id: '',
  });
  if (planIssueRes.status !== 303) throw new Error(`create plan issue: expected 303, got ${planIssueRes.status}`);

  // The calendar block: assigned to self, in progress, planned today --
  // same two-step shape `overflow-gate.mjs`'s GATE-002 fixture and
  // `undo-mousedown-trap.mjs`'s own fixture use.
  const ownerId = await optionValue(jar.header(), `${BASE}/projects/${projectId}/issues/new`, DISPLAY_NAME);
  const calIssueRes = await jar.fetchForm(`${BASE}/projects/${projectId}/issues/new`, {
    title: 'Drag outcome calendar issue',
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
    title: 'Drag outcome calendar issue',
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

  return { cookie: jar.header(), jar, projectId, calIssueId, teamSlug, sprintId };
}

// Synthetic `DragEvent` sequences that complete a real drop -- not the
// gesture under test (that question, `GATE-005` candidate 4, needs
// `Input.dispatchDragEvent`/`Input.setInterceptDrags`, which `cdp.mjs` does
// not wrap and this handoff does not add). Verbatim from
// `undo-mousedown-trap.mjs`, already proven against these two scripts' real
// `drop` handlers in a gate that passes today.
const PRODUCE = {
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
  plan: '[data-plan-move]:has(.toast)',
  calendar: '#day-view [data-issue-id]:has(.toast)',
};

// The element to Tab *from*. For the calendar block there is exactly
// one focusable descendant (the title link), so it is also
// `toastOrigin`'s own element and Undo is the row's only other
// focusable child. For a plan row there are two -- the title link,
// then the row's own keyboard move button -- so the control
// immediately before Undo in DOM order is the move button, not the
// link; `toastOrigin` itself (`row.querySelector("button, a")`)
// picks the link because that selector list has no priority, which
// is a fact about `toastOrigin`'s own focus-restore job (return focus
// to *something* in the row), not about Tab order from here.
const ACTED_ON_SEL = {
  plan: '[data-plan-move] form button[type="submit"]',
  calendar: '#day-view [data-issue-id] a',
};

async function realTabPress(browser) {
  const common = { key: 'Tab', code: 'Tab', windowsVirtualKeyCode: 9, nativeVirtualKeyCode: 9 };
  await browser.send('Input.dispatchKeyEvent', { type: 'rawKeyDown', ...common });
  await browser.send('Input.dispatchKeyEvent', { type: 'keyUp', ...common });
}

async function pollUntil(fn, timeoutMs, label) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const v = await fn();
    if (v) return v;
    await new Promise(r => setTimeout(r, 100));
  }
  throw new Error(`timed out waiting for ${label}`);
}

async function checkPlan(browser, jar, { projectId, teamSlug, sprintId }) {
  const planUrl = `${BASE}/teams/${teamSlug}/sprints/${sprintId}/plan`;
  const before = await (await jar.get(planUrl)).text();
  if (!before.includes('Drag outcome plan issue')) {
    throw new Error('plan fixture issue not found on the plan page before the drop');
  }

  await browser.setViewport(1280, 900, false);
  await browser.goto(planUrl);
  await browser.waitForSelector('[data-plan-move]', 5000);
  await browser.eval(PRODUCE.plan);
  await browser.waitForSelector(HOLDER_SEL.plan, 2000);

  // 1 (ordered first, per GATE-006 §2): the stored effect, read back
  // over HTTP -- independent of the page's own optimistic DOM state.
  // The row drags toward whichever column it is not currently in
  // (`PRODUCE.plan`'s own logic), so the fixture's choice to start it in
  // the backlog makes "did it leave the backlog section" the one
  // question this HTTP read needs to answer. Bounded to `</section>`,
  // not a fixed character count -- an empty backlog's own "no issues"
  // copy is short enough that a fixed window spills into the next
  // `<section>` and reads its content as the backlog's own.
  const after = await (await jar.get(planUrl)).text();
  const backlogSection = (after.split('id="backlog-heading"')[1] ?? '').split('</section>')[0];
  const stillInBacklog = backlogSection.includes('Drag outcome plan issue');
  if (stillInBacklog) {
    throw new Error(
      `plan drop did not take effect -- fixture issue still listed under the backlog section after the drop: ${after.slice(0, 4000)}`,
    );
  }
  log('plan: stored effect confirmed over HTTP -- issue left the backlog section');

  // 2: the live region receives the outcome text.
  const regionText = await browser.eval(
    `document.getElementById('status-announcements')?.textContent || ''`,
  );
  const expected = 'Moved to the sprint.';
  if (regionText !== expected) {
    throw new Error(`plan: expected the polite region to read ${JSON.stringify(expected)}, got ${JSON.stringify(regionText)}`);
  }
  log(`plan: live region reads ${JSON.stringify(regionText)}`);

  // 3: the undo toast's own Tab order.
  await browser.eval(`document.querySelector('${ACTED_ON_SEL.plan}').focus()`);
  await realTabPress(browser);
  const tabbedToUndo = await browser.eval(
    `document.activeElement === document.querySelector('[data-plan-move] .toast button')`,
  );
  if (!tabbedToUndo) {
    throw new Error('plan: Tab from the acted-on row did not land on the undo button');
  }
  log('plan: Tab from the acted-on row lands on the undo button');
}

async function checkCalendar(browser, jar, { projectId, calIssueId }) {
  const editUrl = `${BASE}/projects/${projectId}/issues/${calIssueId}/edit`;
  const before = await plannedStartAt(jar, editUrl);

  const calUrl = `${BASE}/today/calendar?view=day`;
  await browser.setViewport(1280, 900, false);
  await browser.goto(calUrl);
  await browser.waitForSelector('#day-view [data-issue-id]', 5000);
  await browser.eval(PRODUCE.calendar);
  await browser.waitForSelector(HOLDER_SEL.calendar, 2000);

  // 1 (ordered first): the stored effect, read back over HTTP.
  const after = await pollUntil(
    async () => {
      const v = await plannedStartAt(jar, editUrl);
      return v !== before ? v : null;
    },
    3000,
    'planned_start_at to change after the calendar drop',
  );
  log(`calendar: stored effect confirmed over HTTP -- planned_start_at ${before} -> ${after}`);

  // 2: the live region receives the outcome text. `calendar.js`'s
  // announcement embeds a server-computed time label this script does not
  // reproduce -- asserting the stable prefix is the honest amount of
  // precision; the full text is `CalendarRescheduledAnnouncement`'s own.
  const regionText = await browser.eval(
    `document.getElementById('status-announcements')?.textContent || ''`,
  );
  if (!regionText.startsWith('Rescheduled to ')) {
    throw new Error(`calendar: expected the polite region to start with "Rescheduled to ", got ${JSON.stringify(regionText)}`);
  }
  log(`calendar: live region reads ${JSON.stringify(regionText)}`);

  // 3: the undo toast's own Tab order.
  await browser.eval(`document.querySelector('${ACTED_ON_SEL.calendar}').focus()`);
  await realTabPress(browser);
  const tabbedToUndo = await browser.eval(
    `document.activeElement === document.querySelector('#day-view [data-issue-id] .toast button')`,
  );
  if (!tabbedToUndo) {
    throw new Error('calendar: Tab from the acted-on block did not land on the undo button');
  }
  log('calendar: Tab from the acted-on block lands on the undo button');
}

async function main() {
  const dbDir = mkdtempSync(join(tmpdir(), 'drag-outcome-db-'));
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

    const fixtures = await createFixtures();
    log(`fixtures created: project=${fixtures.projectId} team=${fixtures.teamSlug} sprint=${fixtures.sprintId}`);

    browser = await launch({ width: 1280, height: 900 });
    const [cookieName, cookieValue] = fixtures.cookie.split('=');
    await browser.send('Network.setCookie', { name: cookieName, value: cookieValue, domain: '127.0.0.1', path: '/' });

    await checkPlan(browser, fixtures.jar, fixtures);
    await checkCalendar(browser, fixtures.jar, fixtures);

    log('both surfaces: stored effect confirmed, live region text correct, Tab reaches Undo');
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
