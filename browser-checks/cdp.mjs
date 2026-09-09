// Minimal CDP driver: no dependencies, Node's built-in WebSocket and
// fetch. Written 2026-09-06 for the inspection recorded in
// `.git-exclude/tasks/architect/013-browser-inspection-findings.md`;
// tracked by `BROWSER-001` (RFC 011 step 4) because the overflow gate
// now depends on it.
//
// **Known limits, carried forward rather than fixed** (`BROWSER-001`
// §3): it drives one page at a time -- no concurrent tabs, no
// multi-context isolation beyond one browser instance per `launch()`
// call. It waits on `document.readyState`, nothing more -- a page
// whose content is still streaming in after `'complete'` fires (a
// fetch kicked off by an onload handler, say) is not something this
// harness's `goto()` knows to wait for. It has no notion of animation
// completion -- a CSS transition or a DaisyUI dropdown's open/close
// animation (`LAYOUT-001`'s own `scale(0.95)` finding) can still be
// mid-flight when `goto()` returns, which is why a check that cares
// about a specific element's settled state should wait for that
// element directly (`waitForSelector`) rather than trust `goto()`
// alone.
import { spawn, spawnSync } from 'node:child_process';
import { writeFileSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

/// The browser executable to launch. No bundled binary and no npm
/// dependency (`DEC-048` condition 4's own constraint on this
/// handoff) -- this harness uses whatever is already on the machine.
/// `CDP_BROWSER_BIN` overrides the search entirely (set it if none of
/// these names match). Otherwise the first name found on `$PATH`
/// wins: `google-chrome-stable` first because that is what GitHub's
/// `ubuntu-latest` runner image ships preinstalled (no extra
/// `apt-get`/marketplace-action step needed in CI); `chromium` and
/// `chromium-browser` after, for machines where that is what is
/// actually installed (this project's own dev machines, so far).
function resolveBrowserBin() {
  if (process.env.CDP_BROWSER_BIN) return process.env.CDP_BROWSER_BIN;
  const candidates = ['google-chrome-stable', 'google-chrome', 'chromium', 'chromium-browser'];
  for (const name of candidates) {
    if (spawnSync('which', [name]).status === 0) return name;
  }
  throw new Error(
    `no browser found on $PATH among ${candidates.join(', ')} -- set CDP_BROWSER_BIN`,
  );
}

export async function launch({ width = 1280, height = 900, port = 9222 } = {}) {
  const browserBin = resolveBrowserBin();
  const userDataDir = mkdtempSync(join(tmpdir(), 'cdp-profile-'));
  const args = [
    '--headless=new', `--remote-debugging-port=${port}`, '--no-first-run',
    '--no-default-browser-check', '--disable-gpu', '--no-sandbox',
    `--window-size=${width},${height}`,
    `--user-data-dir=${userDataDir}`,
    'about:blank',
  ];
  const proc = spawn(browserBin, args, { stdio: 'ignore', detached: false });
  let target = null;
  for (let i = 0; i < 100; i++) {
    try {
      const r = await fetch(`http://127.0.0.1:${port}/json/list`);
      const list = await r.json();
      target = list.find(t => t.type === 'page');
      if (target) break;
    } catch {}
    await new Promise(r => setTimeout(r, 200));
  }
  if (!target) throw new Error(`${browserBin} did not expose a page target`);
  const ws = new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((res, rej) => { ws.onopen = res; ws.onerror = rej; });

  let id = 0;
  const pending = new Map();
  const requestListeners = new Set();
  ws.onmessage = ev => {
    const m = JSON.parse(ev.data);
    if (m.id && pending.has(m.id)) { pending.get(m.id)(m); pending.delete(m.id); return; }
    if (m.method === 'Network.requestWillBeSent') {
      for (const fn of requestListeners) fn(m.params.request.url);
    }
    if (m.method === 'Fetch.requestPaused') {
      fetchPausedHandler?.(m.params);
    }
  };
  let fetchPausedHandler = null;
  const send = (method, params = {}) => new Promise((res, rej) => {
    const myId = ++id;
    pending.set(myId, m => m.error ? rej(new Error(method + ': ' + m.error.message)) : res(m.result));
    ws.send(JSON.stringify({ id: myId, method, params }));
  });

  await send('Page.enable');
  await send('Runtime.enable');
  await send('Network.enable');

  const api = {
    send,
    async setViewport(w, h, mobile = false) {
      await send('Emulation.setDeviceMetricsOverride', {
        width: w, height: h, deviceScaleFactor: 1, mobile,
      });
    },
    async goto(url) {
      await send('Page.navigate', { url });
      // Wait on the one signal available without a page-specific
      // hook: readyState. No trailing fixed-duration sleep -- a page
      // that isn't stable by then needs `waitForSelector` for
      // whatever it's still waiting on, per `BROWSER-001` §2's own
      // "wait on a signal, never on a duration."
      for (let i = 0; i < 150; i++) {
        const r = await api.eval('document.readyState');
        if (r === 'complete') break;
        await new Promise(r => setTimeout(r, 100));
      }
    },
    /// Poll for `selector` to exist in the DOM, up to `timeoutMs`.
    /// Returns once found; throws if it never appears. The
    /// page-specific settle signal `goto()` alone can't provide (see
    /// this module's own doc comment) -- pass a selector known to
    /// render only once the page's own content, not just its shell,
    /// is present.
    async waitForSelector(selector, timeoutMs = 5000) {
      const deadline = Date.now() + timeoutMs;
      while (Date.now() < deadline) {
        const found = await api.eval(`!!document.querySelector(${JSON.stringify(selector)})`);
        if (found) return;
        await new Promise(r => setTimeout(r, 50));
      }
      throw new Error(`waitForSelector(${JSON.stringify(selector)}) timed out after ${timeoutMs}ms`);
    },
    async eval(expr) {
      const r = await send('Runtime.evaluate', {
        expression: `(() => { try { return JSON.stringify(${expr}) } catch (e) { return JSON.stringify({__err: String(e)}) } })()`,
        returnByValue: true, awaitPromise: true,
      });
      const v = r.result?.value;
      if (v === undefined) return undefined;
      try { return JSON.parse(v); } catch { return v; }
    },
    async screenshot(path, clip) {
      const params = { format: 'png' };
      if (clip) params.clip = { ...clip, scale: 3 };
      const r = await send('Page.captureScreenshot', params);
      writeFileSync(path, Buffer.from(r.data, 'base64'));
      return path;
    },
    /// Record every request URL seen from now until `stop()` is
    /// called. Returns `{stop}`, `stop()` returns the collected list.
    /// `DEC-048` condition 1 (`BROWSER-001` §4.1): used to assert a
    /// page requested nothing outside `127.0.0.1`, positively --
    /// not just "nothing broke because it was blocked" but "here is
    /// the exact list of hosts every request went to."
    recordRequests() {
      const urls = [];
      const fn = url => urls.push(url);
      requestListeners.add(fn);
      return { stop: () => { requestListeners.delete(fn); return urls; } };
    },
    /// Actively fail every request whose host is not `127.0.0.1` or
    /// `localhost`, from now until `stop()` is called -- the literal
    /// reading of `DEC-048` condition 1 / `BROWSER-001` §4.1:
    /// *"`Network.setBlockedURLs` on `*` for everything but
    /// `127.0.0.1`, and the run must be unaffected."* Uses the
    /// `Fetch` domain rather than `Network.setBlockedURLs` because
    /// the latter's patterns can't express an exception for one host
    /// -- `Fetch.requestPaused` lets every request be inspected and
    /// allowed or failed individually. Returns `{stop}`; `stop()`
    /// disables interception and returns the list of URLs that were
    /// actually blocked (expected to be empty on a tree with no CDN
    /// dependency left).
    async blockNonLocalRequests() {
      const blocked = [];
      fetchPausedHandler = async params => {
        let host;
        try { host = new URL(params.request.url).hostname; } catch { host = ''; }
        if (host === '127.0.0.1' || host === 'localhost') {
          await send('Fetch.continueRequest', { requestId: params.requestId });
        } else {
          blocked.push(params.request.url);
          await send('Fetch.failRequest', { requestId: params.requestId, errorReason: 'BlockedByClient' });
        }
      };
      await send('Fetch.enable', { patterns: [{ urlPattern: '*' }] });
      return {
        stop: async () => {
          fetchPausedHandler = null;
          await send('Fetch.disable');
          return blocked;
        },
      };
    },
    close() { try { ws.close(); } catch {} proc.kill('SIGTERM'); },
  };
  return api;
}
