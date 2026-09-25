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
// `BROWSER-002` round 2. 320 was held back once: it turned every page
// red by 24px, which was `LAYOUT-006`'s navbar defect rather than
// anything per-page, and `DEC-048` condition 3 forbids landing a red
// gate without an issue link. `LAYOUT-006` fixed it, so it lands now.
//
// What it buys is narrower than the original case for it. With the old
// fixture the board (`LAYOUT-003`) overflowed only below 390, so 320
// looked like the width that would have caught it; with `BROWSER-002`'s
// unbroken run that same defect fails at 390 too. 320's value is the
// defect that manifests *only* at the narrowest phone — which is
// exactly what it caught on its first run.
const WIDTHS = [
  [320, 568, true], [390, 844, true], [768, 1024, false],
  [1280, 900, false], [1920, 1080, false],
];

// A long-enough, unbreakable local part -- `LAYOUT-001` was
// conditional on exactly this shape (no hyphen for the browser's
// default word-wrap to break at), so the fixture must be too, or the
// gate would not have caught the defect it exists to catch.
const LONG_EMAIL = 'browseroverflowgatefixtureaccount@example.org';

// `BROWSER-002`: a 64-character unbroken run, carried by the issue
// title, the project name and the team name.
//
// The gate swept issue detail at 390 and 414 for a full release while
// that page overflowed by 624 and 600 px, and reported 48/48 clean.
// The reason was one character class: every fixture string above had
// a space at every point, so no container ever received text it could
// not break, and `min-width: auto` never had anything to bite on.
// Four of the five layout defects this project has found -- and both
// shapes of `LAYOUT-004` -- were conditional on exactly this input.
//
// A SHA-256 is the realistic form: 64 hex characters, no break
// opportunity anywhere in it, and the kind of thing that genuinely
// lands in an issue title. It is the empty string's digest, which
// makes it recognisable rather than arbitrary.
const UNBROKEN_RUN = 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855';
// 190 characters, against the form's own 200-char ceiling -- the run
// has to fit inside a title the product will actually accept, or the
// fixture tests a state no user can reach.
const LONG_ISSUE_TITLE =
  `An issue whose title is long enough on its own to be the kind of content that made §10.21 invisible on empty fixtures, digest ${UNBROKEN_RUN}`;
const LONG_PROJECT_NAME = `Overflow Gate Project ${UNBROKEN_RUN}`;
const LONG_TEAM_NAME = `Overflow Gate Team ${UNBROKEN_RUN}`;
// `LAYOUT-007`: the display name is user text too, and this field was
// the last one whose fixture value had a space at every point. Two
// subtitles quote it -- `/today`'s and `/settings`' -- and both
// overflowed a 320px phone on a name with an unbroken run while this
// gate swept them green. `BROWSER-002` §1, one field over.
//
// 77 characters against the register form's own 80-char `maxlength`.
// The spaced words lead so the navbar still shows something a user
// recognises once `LAYOUT-006`'s truncation takes the rest.
const LONG_DISPLAY_NAME = `Gate Fixture ${UNBROKEN_RUN}`;
// `LAYOUT-008`: the fifth deliberate fixture property. A sprint whose
// name and goal both carry the run -- three sprint pages render them,
// and none of the three was in this list until now.
const LONG_SPRINT_NAME = `Gate Fixture Sprint ${UNBROKEN_RUN}`;
const LONG_SPRINT_GOAL = `Ship the gate fixture work, digest ${UNBROKEN_RUN}`;
// `GATE-001`: the completed sprint's own issues. A completed sprint's
// issue list is user text in a list -- where `§10.25`'s three shapes were
// all found -- and the fixture's sprint had none, because its project was
// personal and only a team project's issues can join a sprint. The team
// project below carries the run in its name (as the personal one does);
// its three issues are the run in a title, a long title made of words only
// (which must *wrap*, not break), and an ordinary one.
const LONG_TEAM_PROJECT_NAME = `Overflow Gate Team Project ${UNBROKEN_RUN}`;
// 165 characters, spaces at every point: the wrapping shape, as against
// `LONG_ISSUE_TITLE`'s unbreakable one.
const WRAPPING_ISSUE_TITLE =
  'A completed sprint issue whose title is made only of ordinary words and is long enough to wrap onto several lines at the narrowest width this gate sweeps, never once needing to break';
const ORDINARY_TEAM_ISSUE_TITLE = 'An ordinary completed sprint issue';
// `NFR-PRIV-007` suppresses the burndown below two contributors, counted
// as distinct assignees of *done* issues in the sprint -- so the second
// person is a real second account, added to the team through the route.
const SECOND_CONTRIBUTOR_NAME = 'Gate Second Contributor';
const SECOND_CONTRIBUTOR_EMAIL = 'gatesecondcontributor@example.org';

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

// The value of the `<option>` whose label contains `labelPart`, from a page
// the fixture user can see. Ids are not in any redirect, so this is how a
// script reaches a team's id or a member's, exactly as a person picking from
// the list would.
async function optionValue(cookie, url, labelPart) {
  const html = await (await fetch(url, { headers: { cookie } })).text();
  for (const m of html.matchAll(/<option[^>]*value="([^"]*)"[^>]*>([^<]*)<\/option>/g)) {
    if (m[2].includes(labelPart) && m[1]) return m[1];
  }
  throw new Error(`no <option> containing "${labelPart}" on ${url}`);
}

// The optimistic-lock value a page renders for its next write, read the way a
// browser would submit it.
async function lockStamp(jar, url, what) {
  const page = await (await fetch(url, { headers: { cookie: jar.header() } })).text();
  const stamp = page.match(/name="client_updated_at"\s+value="([^"]+)"/)?.[1];
  if (!stamp) throw new Error(`no lock value on ${url} before ${what}`);
  return stamp;
}

async function createFixtures() {
  const jar = makeCookieJar();

  const registerRes = await jar.fetchForm(`${BASE}/register`, {
    display_name: LONG_DISPLAY_NAME,
    email: LONG_EMAIL,
    password: 'password1234',
  });
  if (registerRes.status !== 303) {
    throw new Error(`register: expected 303, got ${registerRes.status}`);
  }

  const teamRes = await jar.fetchForm(`${BASE}/teams`, {
    name: LONG_TEAM_NAME,
    slug: '',
    description: '',
  });
  if (teamRes.status !== 303) throw new Error(`create team: expected 303, got ${teamRes.status}`);
  // `LAYOUT-005`: team detail is a gate page now, so the slug has to
  // come back out of here. It is generated from the name, which
  // carries the unbroken run, and truncated to `SLUG_MAX_LEN`.
  const teamSlug = teamRes.headers.get('location').split('/teams/')[1].split(/[/?]/)[0];

  // `LAYOUT-008`: a sprint, through the real form like everything else
  // here. Its name and goal carry the run; the sprints list, the sprint
  // detail page and the plan page all render one or both.
  const sprintRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints`, {
    name: LONG_SPRINT_NAME,
    goal: LONG_SPRINT_GOAL,
    starts_on: '2026-09-01',
    ends_on: '2026-09-14',
  });
  if (sprintRes.status !== 303) throw new Error(`create sprint: expected 303, got ${sprintRes.status}`);
  const sprintId = sprintRes.headers.get('location').split('/sprints/')[1].split(/[/?]/)[0];

  const projectRes = await jar.fetchForm(`${BASE}/projects`, {
    name: LONG_PROJECT_NAME,
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

  // `SPRINT-005` (`§10.27`): a **completed** sprint with a captured record.
  // The sprint above stays planned -- its plan page is the one with the
  // backlog and the drag controls -- so the completed-sprint form of the
  // sprint detail page (the Reopen control, and `SPRINT-004`'s two headings)
  // was reachable by nothing here, and the gate reported 90/90 about a
  // different page. It goes through the real routes, so the record is the one
  // `complete` captures: it is started, then completed (each with the lock
  // value the page renders). It has no members -- the fixture's project is
  // personal, and only a team project's issues can join a sprint -- so the
  // page shows the record and the completed-sprint controls and headings, not
  // an issue list.
  const completedRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints`, {
    name: `${LONG_SPRINT_NAME} (completed)`,
    goal: LONG_SPRINT_GOAL,
    starts_on: '2026-08-01',
    ends_on: '2026-08-14',
  });
  if (completedRes.status !== 303) throw new Error(`create completed sprint: expected 303, got ${completedRes.status}`);
  const completedSprintId = completedRes.headers.get('location').split('/sprints/')[1].split(/[/?]/)[0];

  // `GATE-001`: the completed sprint gets members. A second account joins the
  // team, a team project is created (the personal project above stays as it
  // is), and three issues are created in it, assigned to the two people,
  // added to the still-planned sprint through the plan route, and marked done
  // through the status route -- so `complete` below captures a real record,
  // with two contributors, from real events.
  const secondJar = makeCookieJar();
  const secondRes = await secondJar.fetchForm(`${BASE}/register`, {
    display_name: SECOND_CONTRIBUTOR_NAME,
    email: SECOND_CONTRIBUTOR_EMAIL,
    password: 'password1234',
  });
  if (secondRes.status !== 303) throw new Error(`register second contributor: expected 303, got ${secondRes.status}`);
  const memberRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/members`, {
    email: SECOND_CONTRIBUTOR_EMAIL,
    role: 'member',
  });
  if (memberRes.status !== 303) throw new Error(`add team member: expected 303, got ${memberRes.status}`);

  const teamId = await optionValue(jar.header(), `${BASE}/projects/new`, LONG_TEAM_NAME);
  const teamProjectRes = await jar.fetchForm(`${BASE}/projects`, {
    name: LONG_TEAM_PROJECT_NAME,
    description: 'Team fixture project for the GATE-001 completed sprint.',
    team_id: teamId,
  });
  if (teamProjectRes.status !== 303) throw new Error(`create team project: expected 303, got ${teamProjectRes.status}`);
  const teamProjectId = teamProjectRes.headers.get('location').split('/projects/')[1].split(/[/?]/)[0];

  const newIssueUrl = `${BASE}/projects/${teamProjectId}/issues/new`;
  const firstId = await optionValue(jar.header(), newIssueUrl, 'Gate Fixture ');
  const secondId = await optionValue(jar.header(), newIssueUrl, SECOND_CONTRIBUTOR_NAME);
  const sprintIssues = [
    { title: LONG_ISSUE_TITLE, assignee: firstId, effort: '5' },
    { title: WRAPPING_ISSUE_TITLE, assignee: secondId, effort: '3' },
    { title: ORDINARY_TEAM_ISSUE_TITLE, assignee: secondId, effort: '2' },
  ];
  const sprintIssueIds = [];
  for (const { title, assignee, effort } of sprintIssues) {
    const r = await jar.fetchForm(newIssueUrl, {
      title, description: '', status: 'open', priority: 'medium', effort, assignee_id: assignee,
    });
    if (r.status !== 303) throw new Error(`create team issue: expected 303, got ${r.status}`);
    const id = r.headers.get('location').split('/issues/')[1].split(/[/?]/)[0];
    sprintIssueIds.push(id);
    const add = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints/${completedSprintId}/plan/add`, {
      issue_id: id, project_id: teamProjectId,
    });
    if (add.status !== 303) throw new Error(`add issue to sprint: expected 303, got ${add.status}`);
  }
  const startStamp = await lockStamp(jar, `${BASE}/teams/${teamSlug}/sprints/${completedSprintId}`, 'start');
  const startRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints/${completedSprintId}/start`, {
    client_updated_at: startStamp,
  });
  if (startRes.status !== 303) throw new Error(`start sprint: expected 303, got ${startRes.status}`);
  for (const id of sprintIssueIds) {
    const stamp = await lockStamp(jar, `${BASE}/projects/${teamProjectId}/issues/${id}`, 'mark done');
    const r = await jar.fetchForm(`${BASE}/projects/${teamProjectId}/issues/${id}/status/detail`, {
      status: 'done', client_updated_at: stamp,
    });
    if (r.status !== 303) throw new Error(`mark issue done: expected 303, got ${r.status}`);
  }
  const completeStamp = await lockStamp(jar, `${BASE}/teams/${teamSlug}/sprints/${completedSprintId}`, 'complete');
  const completeRes = await jar.fetchForm(`${BASE}/teams/${teamSlug}/sprints/${completedSprintId}/complete`, {
    client_updated_at: completeStamp,
  });
  if (completeRes.status !== 303) throw new Error(`complete sprint: expected 303, got ${completeRes.status}`);

  // The page the gate sweeps must actually hold what this fixture exists to
  // put on it -- a 200 says nothing about that. Three titles in the issue
  // list, and the burndown (which two contributors are what let render).
  const completedPage = await (await fetch(`${BASE}/teams/${teamSlug}/sprints/${completedSprintId}`, {
    headers: { cookie: jar.header() },
  })).text();
  const expected = [
    'Summary at completion', 'Issues in this sprint now', 'Burndown',
    LONG_ISSUE_TITLE, WRAPPING_ISSUE_TITLE, ORDINARY_TEAM_ISSUE_TITLE,
  ];
  const missing = expected.filter(text => !completedPage.includes(text));
  if (missing.length) {
    throw new Error(`completed sprint page lacks fixture content: ${missing.map(m => JSON.stringify(m)).join(', ')}`);
  }
  log(`completed sprint page holds: ${expected.map(m => JSON.stringify(m.length > 40 ? `${m.slice(0, 37)}...` : m)).join(', ')}`);

  return { cookie: jar.header(), projectId, issueId, teamSlug, sprintId, completedSprintId };
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

    const { cookie, projectId, issueId, teamSlug, sprintId, completedSprintId } = await createFixtures();
    log(`fixtures created: project=${projectId} issue=${issueId}`);

    const pages = {
      today: `${BASE}/today`,
      inbox: `${BASE}/inbox`,
      calendar: `${BASE}/today/calendar`,
      projects: `${BASE}/projects`,
      project_detail: `${BASE}/projects/${projectId}`,
      // `LAYOUT-005`: this key was `board`, and it points at the list
      // view. The board is the default view, which `project_detail`
      // above already covers -- a coverage log should not name a page
      // after the one it is not.
      list: `${BASE}/projects/${projectId}?view=list`,
      // `LAYOUT-005`: both absent from this list until now, which is
      // why the fixture that exposed five sites never saw these two.
      project_calendar: `${BASE}/projects/${projectId}/calendar`,
      team_detail: `${BASE}/teams/${teamSlug}`,
      // `LAYOUT-008`: four more absent pages. Three render the sprint
      // name or goal; `issue_new` renders the workload hint's chips,
      // which hold display names.
      sprints: `${BASE}/teams/${teamSlug}/sprints`,
      sprint_detail: `${BASE}/teams/${teamSlug}/sprints/${sprintId}`,
      sprint_plan: `${BASE}/teams/${teamSlug}/sprints/${sprintId}/plan`,
      // `SPRINT-005`: the completed-sprint form of the detail page.
      sprint_detail_completed: `${BASE}/teams/${teamSlug}/sprints/${completedSprintId}`,
      issue_new: `${BASE}/projects/${projectId}/issues/new`,
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
