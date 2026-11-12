// `DISC-001` (`§10.58`) -- a disclosure control must look like one and
// keep its 44x44 target. `grow_text()` -- `inline-flex items-center` --
// suppresses a `<summary>`'s native `::marker` box, since a marker only
// exists on `display: list-item`; `touch_target_scan` cannot see this,
// because it reads which helper a `<summary>` was given, never the
// computed style the browser resolves from it. This is the browser check
// that can.
//
// **`DISC-001` scoped this to the three it built** -- the project
// board/list's own `HealthStrip`/`WorkloadStrip`/`Indicators`
// summaries in `components/issues.rs`. **`MARK-001` (`061`) extends it
// to the six more it built**, per that round's own handoff (§4): both
// of `settings.rs`'s summaries (the "add a capacity row" disclosure and
// the per-row points-edit dropdown), both of `me.rs`'s (`/today`'s
// "What do these mean?" and the Sustainability summary), and both of
// `sprints.rs`'s chart-table toggles -- the velocity table (sprints
// list page) and the burndown table (sprint detail page). The handoff's
// own parenthetical names three pages (`/today`, settings, sprint
// detail); this covers a fourth, the sprints list, because that is
// where the velocity-chart summary (one of the six this round actually
// built) lives -- a guard that claims to cover what this round built
// but skips the page one of its six sites is on would not be covering
// what it claims.
//
// **`MARK-002` (`061`) extends it to the four `MARK-001` found but did
// not build** -- `teams.rs`'s "invite a member" summary and
// `notification_preferences.rs`'s "per-kind delivery" summary (both
// moved their own row layout onto an inner `<div>`, since a `<summary>`
// is never a flex container); `settings.rs`'s close-date dropdown
// (tried moving its `btn btn-ghost btn-xs` appearance onto an inner
// `<span>` first, but `inline-flex` is an atomic inline-level box that
// wraps as a whole unit onto its own line when it does not fit beside
// the marker in a narrow container -- measured wrong at 390px, so the
// decision changed to dropping the button look entirely, matching
// `settings.rs`'s own sibling points-edit dropdown); and `/today`'s own
// Rhythm summary, which turned out on closer reading to be one of the
// ordinary single-label sites this round's shape already covers, not a
// fourth multi-child exception -- see that round's own completion
// report for all three measurements.
//
// `alignment_scan`'s own source-level scan now asserts the positive
// property this file asserts behaviourally: every real `<summary>`
// composes `grow_summary(...)` and nothing else, with no display class
// smuggled into that call's own argument either. Thirteen of this
// product's fourteen real summaries are swept by both; the fourteenth
// (`issues.rs`'s own named escalation exclusion) is deliberately below
// 44x44 and out of this file's own scope.
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

async function lockStamp(cookie, url) {
  const page = await (await fetch(url, { headers: { cookie } })).text();
  const stamp = page.match(/name="client_updated_at"\s+value="([^"]+)"/)?.[1];
  if (!stamp) throw new Error(`no lock value on ${url}`);
  return stamp;
}

// Two members, several in-flight issues on one assignee -- the same
// fixture shape `VIEW-001`'s own evidence used, so both `HealthStrip`
// (a non-Good indicator) and `WorkloadStrip` (an assigned in-flight
// issue) actually render their own summary rather than an empty state.
// The owner's own four assigned in-progress issues are also what
// `/today`'s `user_is_active` check reads, so no separate fixture is
// needed for `MARK-001`'s two `me.rs` summaries.
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

  // `MARK-001`'s two `settings.rs` summaries: an open-ended capacity
  // row renders both the "add a capacity row" disclosure (always
  // present) and the per-row points-edit dropdown (only once a row
  // exists).
  const capRes = await jar.fetchForm(`${BASE}/settings/capacity`, {
    points: '10', period_start: '2026-01-01', period_end: '',
  });
  if (capRes.status !== 303) throw new Error(`insert capacity row: expected 303, got ${capRes.status}`);

  // `MARK-001`'s two `sprints.rs` summaries: the burndown table needs
  // one completed sprint with two contributors on done issues
  // (`NFR-PRIV-007` suppresses it below two, the same gate
  // `overflow-gate.mjs`'s own fixture works around); the velocity
  // table's own page is covered by a second, simpler completed sprint,
  // since a velocity row needs two sprints to compare and carries no
  // privacy floor of its own.
  const teamProjectRes = await jar.fetchForm(`${BASE}/projects`, {
    name: 'Disclosure Marker Team Project', description: '', team_id: teamId,
  });
  if (teamProjectRes.status !== 303) throw new Error(`create team project: expected 303, got ${teamProjectRes.status}`);
  const teamProjectId = teamProjectRes.headers.get('location').split('/projects/')[1].split(/[/?]/)[0];
  const teamNewIssueUrl = `${BASE}/projects/${teamProjectId}/issues/new`;
  const teamOwnerId = await optionValue(ownerCookie, teamNewIssueUrl, DISPLAY_NAME);
  const teamSecondId = await optionValue(ownerCookie, teamNewIssueUrl, SECOND_DISPLAY_NAME);

  const sprintARes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints`, {
    name: 'Disclosure Marker Sprint A', goal: 'g', starts_on: '2026-08-01', ends_on: '2026-08-14',
  });
  if (sprintARes.status !== 303) throw new Error(`create sprint A: expected 303, got ${sprintARes.status}`);
  const sprintAId = sprintARes.headers.get('location').split('/sprints/')[1].split(/[/?]/)[0];

  const burndownIssueIds = [];
  for (const [title, assignee] of [
    ['Burndown fixture issue 1', teamOwnerId],
    ['Burndown fixture issue 2', teamSecondId],
  ]) {
    const r = await jar.fetchForm(teamNewIssueUrl, {
      title, description: '', status: 'open', priority: 'medium', effort: '3', assignee_id: assignee,
    });
    if (r.status !== 303) throw new Error(`create burndown issue: expected 303, got ${r.status}`);
    const id = r.headers.get('location').split('/issues/')[1].split(/[/?]/)[0];
    burndownIssueIds.push(id);
    const add = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints/${sprintAId}/plan/add`, {
      issue_id: id, project_id: teamProjectId,
    });
    if (add.status !== 303) throw new Error(`add issue to sprint A: expected 303, got ${add.status}`);
  }

  const startAStamp = await lockStamp(ownerCookie, `${BASE}/teams/${teamSlug}/sprints/${sprintAId}`);
  const startARes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints/${sprintAId}/start`, { client_updated_at: startAStamp });
  if (startARes.status !== 303) throw new Error(`start sprint A: expected 303, got ${startARes.status}`);

  for (const id of burndownIssueIds) {
    const stamp = await lockStamp(ownerCookie, `${BASE}/projects/${teamProjectId}/issues/${id}`);
    const r = await jar.fetchForm(`${BASE}/projects/${teamProjectId}/issues/${id}/status/detail`, {
      status: 'done', client_updated_at: stamp,
    });
    if (r.status !== 303) throw new Error(`mark burndown issue done: expected 303, got ${r.status}`);
  }

  const completeAStamp = await lockStamp(ownerCookie, `${BASE}/teams/${teamSlug}/sprints/${sprintAId}`);
  const completeARes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints/${sprintAId}/complete`, { client_updated_at: completeAStamp });
  if (completeARes.status !== 303) throw new Error(`complete sprint A: expected 303, got ${completeARes.status}`);

  const sprintBRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints`, {
    name: 'Disclosure Marker Sprint B', goal: 'g', starts_on: '2026-09-01', ends_on: '2026-09-14',
  });
  if (sprintBRes.status !== 303) throw new Error(`create sprint B: expected 303, got ${sprintBRes.status}`);
  const sprintBId = sprintBRes.headers.get('location').split('/sprints/')[1].split(/[/?]/)[0];
  const startBStamp = await lockStamp(ownerCookie, `${BASE}/teams/${teamSlug}/sprints/${sprintBId}`);
  const startBRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints/${sprintBId}/start`, { client_updated_at: startBStamp });
  if (startBRes.status !== 303) throw new Error(`start sprint B: expected 303, got ${startBRes.status}`);
  const completeBStamp = await lockStamp(ownerCookie, `${BASE}/teams/${teamSlug}/sprints/${sprintBId}`);
  const completeBRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints/${sprintBId}/complete`, { client_updated_at: completeBStamp });
  if (completeBRes.status !== 303) throw new Error(`complete sprint B: expected 303, got ${completeBRes.status}`);

  return { cookie: ownerCookie, projectId, teamSlug, sprintDetailId: sprintAId };
}

// `DISC-001`'s own three, found by stable anchors -- never the
// headline's own computed sentence, which depends on which indicator
// is driving it and is not either round's subject. `health`/`workload`
// are each the first `<summary>` inside their section's own
// `aria-label` (`VIEW-001`'s own stable anchor, reused here); `Indicators`
// is a fixed label (`IndicatorsSummaryLabel`), never data-dependent.
const PROJECT_SCREEN_SUMMARIES = {
  health: `document.querySelector('[aria-label="Project health"] summary')`,
  workload: `document.querySelector('[aria-label="Workload"] summary')`,
  indicators: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.trim() === 'Indicators')`,
};

// `MARK-001`'s six, by the same stable-anchor discipline: a fixed
// label where one exists (`AddCapacityRowSummary`'s own text,
// `ChartTableSummaryLabel`'s own text), index-based only where the
// page structurally has exactly one summary of that kind
// (`WhatDoTheseMeanLabel` is `/today`'s only non-Rhythm summary in
// this fixture; the points-edit dropdown is the only `<summary>`
// inside a table row).
const SETTINGS_SUMMARIES = {
  add_capacity_row: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.includes('Add a capacity row'))`,
  points_edit_dropdown: `document.querySelector('tr summary')`,
  // `MARK-002`: the close-date dropdown, converted from a bare
  // `btn btn-ghost btn-xs` summary to plain text -- found by the
  // stable `CloseOnDateSummary` text.
  close_date_dropdown: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.includes('Close on date'))`,
};
const ME_SUMMARIES = {
  // `MARK-002`: `/today`'s own Rhythm summary -- found markerless
  // while measuring for `MARK-001` (literal `inline-flex` baked into
  // `grow(...)`'s argument, not `grow_text(`) and, on closer reading,
  // a single-label site like every other one this round converts, not
  // one of the genuinely multi-child sites it was first named beside.
  // Its own `aria-label` is a fixed string (`RhythmAriaLabel`), set on
  // the `<summary>` itself.
  rhythm: `document.querySelector('summary[aria-label^="Rhythm"]')`,
  what_do_these_mean: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.includes('What do these mean'))`,
  sustainability: `document.querySelector('[aria-label="Sustainability"] summary')`,
};
const SPRINTS_LIST_SUMMARIES = {
  velocity_chart_table: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.includes('View as a table'))`,
};
const SPRINT_DETAIL_SUMMARIES = {
  burndown_chart_table: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.includes('View as a table'))`,
};
// `MARK-002`'s two multi-child sites, each row layout now on an inner
// <div> rather than the <summary> itself; found by the stable
// `PerKindDeliverySummary`/`InviteMemberSummary` text.
const NOTIFICATION_PREFERENCES_SUMMARIES = {
  per_kind_delivery: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.includes('Per-kind delivery'))`,
};
const TEAM_DETAIL_SUMMARIES = {
  invite_member: `Array.from(document.querySelectorAll('summary')).find(s => s.textContent.includes('Invite a member'))`,
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

async function checkPage(browser, pageLabel, url, summaries, width, height) {
  await browser.setViewport(width, height, width < 768);
  await browser.goto(url);
  await browser.waitForSelector('header.navbar', 5000);

  for (const [label, expr] of Object.entries(summaries)) {
    const found = await measureSummary(browser, expr);
    if (!found) {
      throw new Error(`${pageLabel}@${width}: ${label} summary not found on the page`);
    }
    if (found.display !== 'list-item') {
      throw new Error(
        `${pageLabel}@${width}: ${label} summary (${JSON.stringify(found.text)}) has computed display ` +
        `${JSON.stringify(found.display)}, not "list-item" -- its ::marker is suppressed`,
      );
    }
    if (found.width < 44 || found.height < 44) {
      throw new Error(
        `${pageLabel}@${width}: ${label} summary (${JSON.stringify(found.text)}) measures ` +
        `${found.width}x${found.height}, below the 44x44 floor`,
      );
    }
    log(`${pageLabel}@${width}: ${label} -- display=list-item, ${found.width}x${found.height}`);
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

    for (const [width, height] of [[1280, 900], [390, 844]]) {
      for (const viewMode of ['board', 'list']) {
        await checkPage(
          browser, viewMode, `${BASE}/projects/${fixture.projectId}?view=${viewMode}`,
          PROJECT_SCREEN_SUMMARIES, width, height,
        );
      }
      await checkPage(browser, 'settings', `${BASE}/settings`, SETTINGS_SUMMARIES, width, height);
      await checkPage(browser, 'today', `${BASE}/today`, ME_SUMMARIES, width, height);
      await checkPage(
        browser, 'sprints_list', `${BASE}/teams/${fixture.teamSlug}/sprints`,
        SPRINTS_LIST_SUMMARIES, width, height,
      );
      await checkPage(
        browser, 'sprint_detail', `${BASE}/teams/${fixture.teamSlug}/sprints/${fixture.sprintDetailId}`,
        SPRINT_DETAIL_SUMMARIES, width, height,
      );
      await checkPage(
        browser, 'settings_notifications', `${BASE}/settings/notifications`,
        NOTIFICATION_PREFERENCES_SUMMARIES, width, height,
      );
      await checkPage(
        browser, 'team_detail', `${BASE}/teams/${fixture.teamSlug}`,
        TEAM_DETAIL_SUMMARIES, width, height,
      );
    }

    log('all thirteen checkable summaries (DISC-001 three + MARK-001 six + MARK-002 four): list-item display, >=44x44, at both widths. The fourteenth real <summary>, issues.rs\'s own named escalation exclusion, is deliberately below 44x44 and outside this gate\'s own scope.');
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
