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
  // `DOC-005` §5: `createFixtures` now requires these two as well.
  backlogIssueTitle: 'Draft the onboarding email sequence',
  planItemIssueTitle: 'Add empty states to the dashboard widgets',
  // `DOC-005` §6: found reading the regenerated `sprint_plan.png` --
  // the Backlog row's own source-project name was still the gate's
  // adversarial fixture value (a module constant, not a parameter)
  // because it had never rendered on a captured page before this
  // round's fourth screen. `teamProjectName` fixes that the same way
  // `DOCS-008` §1 fixed its own three.
  teamProjectName: 'Internal Tools',
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
      // `FIV-006` §1, the same reasoning as `DATABASE_URL` above:
      // explicit, not ambient. Without this, `Config::from_env`'s
      // `JWT_SECRET` lookup falls through to whatever a developer's
      // own `.env` happens to set (`FIV-001`'s resolution order), so
      // a real local signing key would sign the cookies this
      // throwaway capture session uses. It never reaches a committed
      // image -- the database is a temp directory, deleted in
      // `finally` below -- so this is hygiene, not exposure, but the
      // capture should not depend on that distinction holding.
      JWT_SECRET: 'doc-screenshots-throwaway-signing-key-not-for-reuse',
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

    mkdirSync(OUT_DIR, { recursive: true });
    browser = await launch(VIEWPORT);
    await browser.setViewport(VIEWPORT.width, VIEWPORT.height, false);

    const written = [];

    // `FIV-004` round 2 `§9`/`§10`: `install.md`'s own "What you'll
    // see" promises "an empty project list" and then showed five
    // screenshots, none of them empty -- the exact caption-contradicts-
    // image defect `DOCS-006` exists to name, on the page the release
    // produces to avoid it. The real empty state is a moment
    // `createFixtures` already passes through, between registering and
    // creating anything -- captured here via its own `onRegistered`
    // hook, using the browser already launched above, rather than
    // adding a second fixture flow or guessing a pixel-perfect fake.
    const setCookie = async (rawCookie) => {
      const [cookieName, cookieValue] = rawCookie.split('=');
      await browser.send('Network.setCookie', {
        name: cookieName, value: cookieValue, domain: '127.0.0.1', path: '/',
      });
    };
    // Shared with the `/today` capture below, same reason: `<body>`'s
    // `min-h-screen` fills the shared 1280x900 viewport regardless of
    // how tall `#main`'s own content actually is, so a short page
    // shows real content atop a blank gap unless the viewport shrinks
    // to match it first. `projects_empty` has the identical shape --
    // a near-empty state page -- found by looking at this capture's
    // own first result, not assumed from fixing `/today`'s copy of it.
    const shrinkViewportToContent = async () => {
      const contentHeight = await browser.eval(
        "Math.ceil(document.getElementById('main').getBoundingClientRect().bottom)",
      );
      await browser.setViewport(VIEWPORT.width, contentHeight, false);
    };
    const restoreViewport = () => browser.setViewport(VIEWPORT.width, VIEWPORT.height, false);

    const { cookie, projectId, issueId, teamSlug, sprintId } = await createFixtures(
      PLAIN_STRINGS,
      async ({ cookie: earlyCookie }) => {
        await setCookie(earlyCookie);
        await browser.goto(`${BASE}/projects`);
        await browser.waitForSelector('header.navbar', 5000);
        await shrinkViewportToContent();
        const path = join(OUT_DIR, 'projects_empty.png');
        await browser.screenshot(path);
        await restoreViewport();
        const bytes = statSync(path).size;
        written.push({ label: 'projects_empty', path, bytes });
        log(`captured projects_empty -> ${path} (${bytes} bytes)`);
      },
    );
    log(`fixtures created: project=${projectId} issue=${issueId}`);
    // The session cookie does not change after registration (no
    // per-request refresh), so this re-sets the identical value --
    // kept explicit rather than assumed, since `onRegistered`'s own
    // `earlyCookie` and this `cookie` being the same thing is a fact
    // about this app's session handling, not a guarantee this
    // function's own signature makes.
    await setCookie(cookie);

    // Four pages for `first-run.md`'s own walkthrough (`DOCS-007` §3
    // dropped `/today`; `DOCS-008` §4/`DOC-005` found the sprint
    // plan's own alignment defect rather than add a fourth page over
    // an unfixed one) plus `/today` itself (`FIV-006` §2): never
    // captured before that round, though `DOCS-004` named it a
    // candidate in 0.49.0. **Not reused on `install.md`** (`FIV-004`
    // round 2 `§10`): four of these five used to sit on both pages
    // adjacently, the second time inside the walkthrough that
    // explains them -- `install.md` now carries only
    // `projects_empty.png`, captured above, and none of the five
    // below.
    const pages = {
      board: `${BASE}/projects/${projectId}`,
      list: `${BASE}/projects/${projectId}?view=list`,
      issue_detail: `${BASE}/projects/${projectId}/issues/${issueId}`,
      sprint_plan: `${BASE}/teams/${teamSlug}/sprints/${sprintId}/plan`,
      today: `${BASE}/today`,
    };

    for (const [label, url] of Object.entries(pages)) {
      await browser.goto(url);
      await browser.waitForSelector('header.navbar', 5000);
      // `FIV-006` §3: two panels that collapse by default, on
      // purpose, each documented as such in its own component --
      // `HealthStrip`'s indicator breakdown ("so the standard board
      // view stays uncluttered") and `/today`'s own Rhythm panel
      // ("Rhythm is a 'if you want to dig' surface, not first-glance
      // content"). Both sit directly beneath a README claim that
      // promises their contents (the health bullet names six
      // indicators; the dashboard bullet sells "load, rhythm and
      // sustainability"), so a capture that leaves either collapsed
      // reads as broken rather than as a fold a reader hasn't opened.
      // Expanded here, for these captures only -- the product's own
      // default is unchanged, and the alt text in README.md says
      // "expanded" so neither image is read as the page's resting
      // state.
      const detailsToExpand = { board: 'Indicators', today: 'Rhythm' }[label];
      if (detailsToExpand) {
        const opened = await browser.eval(`(() => {
          const details = Array.from(document.querySelectorAll('details'))
            .find(d => d.querySelector('summary')?.textContent.trim() === ${JSON.stringify(detailsToExpand)});
          if (details) details.open = true;
          return !!details;
        })()`);
        if (!opened) {
          throw new Error(`${label} capture: the "${detailsToExpand}" <details> was not found to expand`);
        }
      }
      // `FIV-006` round 2 §9: `/today`'s own content ends well short
      // of the shared 1280x900 viewport -- `<body>`'s `min-h-screen`
      // keeps the page's background filling the viewport regardless,
      // so the capture showed real content sitting atop a blank gap
      // roughly 40% of the image. Resized to the content's own
      // measured height via `shrinkViewportToContent` (shared with
      // `projects_empty` above, `FIV-004` round 2), after the Rhythm
      // panel above is expanded -- measuring first would crop the
      // content this round just added back in.
      if (label === 'today') {
        await shrinkViewportToContent();
      }
      const path = join(OUT_DIR, `${label}.png`);
      // `DOCS-007` §2: no `clip` argument -- `cdp.mjs`'s `screenshot()`
      // only applies its `scale: 3` (sized for evidence-log clarity)
      // when a `clip` is given. Without one, `Page.captureScreenshot`
      // captures the viewport at its natural 1x, which is both smaller
      // and the right resolution for a README or book image.
      await browser.screenshot(path);
      if (label === 'today') {
        // Restored for whichever page the loop reaches next -- `today`
        // is last in `pages` today, so nothing currently depends on
        // this, but leaving the viewport shrunk would silently affect
        // a page added after it later.
        await restoreViewport();
      }
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
