// Drag a day-view block up or down to reschedule it (`CAL-003`, RFC
// 004d D-3). Both planned timestamps move by one snapped delta, so
// the appointment keeps its duration and the block keeps its height
// (requirement 8).
//
// **Unlike `dm.js`/`board.js`, there is no form on this page to fall
// back to** (handoff §2f) — the day view has only links. Every
// failure, before or after the mutation, takes `dm.js`'s *undo*
// posture: revert the block, classify the outcome through the copy
// island, announce, and reload where the outcome says to. Never
// resubmit.
//
// **The lock applies in full, unlike `PLAN-002`.** `/schedule` shares
// `check_optimistic_lock` with every other mutation path
// (`apply_schedule_change`), so this island carries an `outcomes`
// block the same shape `dm.js`'s and `board.js`'s do
// (`response_outcomes.rs` covers this as a third surface) — the
// opposite of `PLAN-002`'s deliberate omission, and that difference
// is the lock, not a change of mind.
//
// No copy is authored here — `static_js_scan` covers `static/*.js`
// with no allowlist entry for this file. Every string comes from
// `components/calendar.rs` (`render_calendar_copy_assets`) as a JSON
// island at `#calendar-copy`, read once at load — including the
// **success announcement**, sent back fully rendered in the
// `/schedule` response itself (`announcement`), because a
// rescheduled time has no enumerable set of values the way a status
// or a plan-drag direction does (§3.6, extended).
(function () {
  "use strict";

  if (typeof window.fetch !== "function") {
    return;
  }

  var dayView = document.getElementById("day-view");
  if (!dayView) return;

  var copyEl = document.getElementById("calendar-copy");
  if (!copyEl) return;

  var copy;
  try {
    copy = JSON.parse(copyEl.textContent);
  } catch (e) {
    return;
  }
  if (
    !copy ||
    typeof copy.undoLabel !== "string" ||
    typeof copy.snapMinutes !== "number" ||
    copy.snapMinutes <= 0 ||
    !copy.outcomes ||
    typeof copy.outcomes.conflictStatus !== "number" ||
    !copy.outcomes.conflict ||
    typeof copy.outcomes.conflict.message !== "string" ||
    !copy.outcomes.unavailable ||
    typeof copy.outcomes.unavailable.message !== "string" ||
    !copy.outcomes.unconfirmed ||
    typeof copy.outcomes.unconfirmed.message !== "string"
  ) {
    return;
  }

  var blocks = dayView.querySelectorAll("[data-issue-id]");
  if (!blocks.length) return;

  // The day container's own fixed height (`components/calendar.rs`'s
  // `style="height: 960px;"`, handoff §2d) — mechanics, not policy,
  // so it stays a literal here rather than moving to the island
  // (§3.4's distinction).
  var DAY_HEIGHT_PX = 960;
  var DAY_SECONDS = 24 * 3600;
  var SNAP_SECONDS = copy.snapMinutes * 60;

  // `QA-011` §2 (`NFR-A11Y-008`): shared ids with `dm.js`/`board.js`.
  function announcePolite(message) {
    var region = document.getElementById("status-announcements");
    if (region) region.textContent = message;
  }

  function announceAssertive(message) {
    var region = document.getElementById("status-announcements-assertive");
    if (region) region.textContent = message;
  }

  // `JS-003` (RFC 011 step 2): read the classified outcome and act —
  // announce, then reload if the outcome says to. Reverting the
  // block, where needed, is the caller's job and happens before this
  // runs, same division `board.js` already uses.
  function applyOutcome(outcome) {
    announceAssertive(outcome.message);
    if (outcome.reload) window.location.reload();
  }

  function scheduleUrl(projectId, issueId) {
    return (
      "/projects/" +
      encodeURIComponent(projectId) +
      "/issues/" +
      encodeURIComponent(issueId) +
      "/schedule"
    );
  }

  function postSchedule(projectId, issueId, startAt, endAt, clientUpdatedAt) {
    return fetch(scheduleUrl(projectId, issueId), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        planned_start_at: startAt,
        planned_end_at: endAt,
        client_updated_at: clientUpdatedAt,
      }),
    });
  }

  // `datetime-local` shape (`YYYY-MM-DDTHH:MM`) from a `Date`, using
  // *local* getters throughout — matching a native `datetime-local`
  // input's own `.value` format, which is the convention
  // `parse_planned_datetime` already parses (handoff §2c/§3.1). Using
  // local getters end to end (never a UTC one) is what keeps this a
  // pure wall-clock shift: the digits move by the delta and nothing
  // else touches them, the same "naive stamp, not a time-zone
  // conversion" contract `parse_planned_datetime`'s own doc comment
  // states.
  function pad(n) {
    return n < 10 ? "0" + n : String(n);
  }
  function toDateTimeLocal(date) {
    return (
      date.getFullYear() +
      "-" +
      pad(date.getMonth() + 1) +
      "-" +
      pad(date.getDate()) +
      "T" +
      pad(date.getHours()) +
      ":" +
      pad(date.getMinutes())
    );
  }

  // `A11Y-004` (`NFR-A11Y-002`): when a toast ends -- Undo pressed or the
  // five seconds up -- and focus is *inside it*, focus returns to the control
  // that was acted on instead of falling to `body`. **Conditional on focus being
  // inside the toast**: a user who has kept working elsewhere is not touched (an
  // unconditional restore, or focusing the toast when it appears, would steal
  // focus, and would make a second Enter on a status button press Undo).
  function toastOrigin(block) {
    return block.querySelector("a");
  }

  // If the origin is gone (its element was removed from the page), focus goes to
  // the first control inside the same holder, and failing that to `<main>` (made
  // focusable with tabindex -1): always somewhere on the page the user was
  // working in, never `body`.
  function restoreFocus(block) {
    var target = toastOrigin(block);
    if (!target || !document.contains(target)) {
      target = document.contains(block) ? block.querySelector('a[href], button:not([disabled])') : null;
    }
    if (!target) {
      var main = document.querySelector("main");
      if (main) {
        if (!main.hasAttribute("tabindex")) main.setAttribute("tabindex", "-1");
        target = main;
      }
    }
    if (target) target.focus();
  }

  // Returns whether focus was inside the toast. `deferRestore` lets the Undo
  // click restore focus itself *after* the undo has moved its element back --
  // the board and plan undos move the card/row synchronously, which would
  // blur a focus restored before it (measured: focus fell to `body`).
  function removeToast(block, deferRestore) {
    var toast = block._calToast;
    if (!toast) return;
    clearTimeout(toast.timer);
    var hadFocus = toast.el.contains(document.activeElement);
    if (toast.el.parentNode) toast.el.parentNode.removeChild(toast.el);
    block._calToast = null;
    if (hadFocus && !deferRestore) restoreFocus(block);
    return hadFocus;
  }

  function showUndoToast(block, message, onUndo) {
    removeToast(block);

    var toast = document.createElement("div");
    toast.className = "toast toast-end toast-bottom z-50";

    var alertBox = document.createElement("div");
    alertBox.className = "alert alert-info text-sm";

    var text = document.createElement("span");
    text.textContent = message;

    var undoButton = document.createElement("button");
    undoButton.type = "button";
    undoButton.className = "btn btn-xs";
    undoButton.textContent = copy.undoLabel;
    undoButton.addEventListener("click", function () {
      var refocus = removeToast(block, true);
      onUndo();
      if (refocus) restoreFocus(block);
    });

    alertBox.appendChild(text);
    alertBox.appendChild(undoButton);
    toast.appendChild(alertBox);
    // `A11Y-004`: in DOM order right after the acted-on control, so Undo is the
    // next Tab stop -- not appended to the end of `<body>`. This is a DOM-order
    // change only: `.toast` is `position: fixed`, so it is still drawn
    // bottom-right and takes no space in the layout it now sits inside.
    block.appendChild(toast);

    var timer = setTimeout(function () {
      removeToast(block);
    }, 5000);

    block._calToast = { el: toast, timer: timer };
  }

  // §3.6: brings the block's four stale facts into line after a
  // *confirmed* mutation — `data-updated-at`, both `data-planned-*`
  // attributes, and the visible time label. All four, every time, or
  // a second drag computes from stale values and an undo sends a
  // stale lock — `PLAN-002` round 1's defect, not rediscovered here.
  // `timeLabel`/`announcement` both come from the response, never
  // formatted or composed here.
  function syncBlockState(block, link, updatedAt, startAttr, endAttr, timeLabel) {
    block.dataset.updatedAt = updatedAt;
    block.dataset.plannedStartAt = startAttr;
    block.dataset.plannedEndAt = endAttr;
    var timeSpan = link.querySelector(".tabular-nums");
    if (timeSpan) timeSpan.textContent = timeLabel;
  }

  function attachBlock(block) {
    var link = block.querySelector("a");
    if (!link) return; // defensive; the wrapper always has one

    // One `drop` listener per drag gesture, added at `dragstart` and
    // always removed at `dragend` -- `dragend` fires after `drop` on
    // a completed gesture (so `handleDrop` has already run by then;
    // removing a listener that already consumed itself is a no-op)
    // and, critically, also fires on a *cancelled* one (dropped
    // outside any valid target, or the gesture aborted) where `drop`
    // never does. Without this, a cancelled drag would leave a stale
    // listener on `dayView` forever, and a later successful drag
    // would fire every stale listener alongside the current one.
    var pendingDropHandler = null;

    block.addEventListener("dragstart", function (e) {
      block.classList.add("opacity-50");
      try {
        e.dataTransfer.effectAllowed = "move";
      } catch (_) {}

      var startClientY = e.clientY;
      var startTopPercent = parseFloat(block.style.top) || 0;
      var heightPercent = parseFloat(block.style.height) || 0;
      var originalStartAttr = block.dataset.plannedStartAt;
      var originalEndAttr = block.dataset.plannedEndAt;

      function handleDrop(dropEvent) {
        dropEvent.preventDefault();

        var deltaPixels = dropEvent.clientY - startClientY;
        // Clamp so the block's optimistic move stays inside the
        // visible day (top >= 0, top + height <= 100) — a drag past
        // either edge would move the appointment onto a different
        // calendar day, which this single-day view has no way to
        // show moving onto; the delta actually sent is clamped the
        // same way so the client never displays a position it isn't
        // also asking the server for.
        var minDeltaPixels = -startTopPercent * (DAY_HEIGHT_PX / 100);
        var maxDeltaPixels =
          (100 - heightPercent - startTopPercent) * (DAY_HEIGHT_PX / 100);
        if (deltaPixels < minDeltaPixels) deltaPixels = minDeltaPixels;
        if (deltaPixels > maxDeltaPixels) deltaPixels = maxDeltaPixels;

        var deltaSeconds = (deltaPixels / DAY_HEIGHT_PX) * DAY_SECONDS;
        deltaSeconds = Math.round(deltaSeconds / SNAP_SECONDS) * SNAP_SECONDS;
        if (deltaSeconds === 0) return; // no-op drag -- nothing to send

        var deltaMs = deltaSeconds * 1000;
        var newStart = new Date(new Date(originalStartAttr).getTime() + deltaMs);
        var newStartAttr = toDateTimeLocal(newStart);
        var newEndAttr = "";
        if (originalEndAttr) {
          var newEnd = new Date(new Date(originalEndAttr).getTime() + deltaMs);
          newEndAttr = toDateTimeLocal(newEnd);
        }

        var newTopPercent =
          startTopPercent + (deltaSeconds / DAY_SECONDS) * 100;

        performMove(
          block,
          link,
          newTopPercent,
          newStartAttr,
          newEndAttr,
          startTopPercent,
          originalStartAttr,
          originalEndAttr,
        );
      }

      pendingDropHandler = handleDrop;
      dayView.addEventListener("drop", handleDrop, { once: true });
    });

    block.addEventListener("dragend", function () {
      block.classList.remove("opacity-50");
      if (pendingDropHandler) {
        dayView.removeEventListener("drop", pendingDropHandler);
        pendingDropHandler = null;
      }
    });
  }

  dayView.addEventListener("dragover", function (e) {
    e.preventDefault();
  });

  // Applies the optimistic move, POSTs the new schedule, and either
  // confirms (§3.6 sync + undo toast) or reverts + classifies the
  // failure (§3.5) — never resubmits either way, matching `dm.js`'s
  // undo posture, the only one available on a page with no form.
  //
  // `restoreStartAttr`/`restoreEndAttr` are the values the block held
  // *before this move* — threaded through as parameters (from the
  // `dragstart` closure that read them fresh off `block.dataset`)
  // rather than stashed on the element and read back later, so
  // there's no separate copy that could go stale between a move and
  // its own undo the way `PLAN-002` round 1's inverted marker did.
  function performMove(
    block,
    link,
    newTopPercent,
    newStartAttr,
    newEndAttr,
    revertTopPercent,
    restoreStartAttr,
    restoreEndAttr,
  ) {
    var projectId = block.dataset.projectId;
    var issueId = block.dataset.issueId;
    var clientUpdatedAt = block.dataset.updatedAt;
    if (!projectId || !issueId || !clientUpdatedAt) return;

    removeToast(block); // a still-open undo from an earlier move is now stale

    block.style.top = newTopPercent + "%";

    function revert() {
      block.style.top = revertTopPercent + "%";
    }

    postSchedule(projectId, issueId, newStartAttr, newEndAttr, clientUpdatedAt)
      .then(function (res) {
        if (res.status === copy.outcomes.conflictStatus) {
          revert();
          applyOutcome(copy.outcomes.conflict);
          return false;
        }
        if (!res.ok) {
          revert();
          applyOutcome(copy.outcomes.unavailable);
          return false;
        }
        return res.json();
      })
      .then(function (body) {
        if (body === false) return;
        if (
          !body ||
          typeof body.updated_at !== "string" ||
          !body.updated_at ||
          typeof body.time_label !== "string" ||
          typeof body.announcement !== "string"
        ) {
          revert();
          applyOutcome(copy.outcomes.unconfirmed);
          return;
        }
        syncBlockState(block, link, body.updated_at, newStartAttr, newEndAttr, body.time_label);
        announcePolite(body.announcement);
        showUndoToast(block, body.announcement, function () {
          performUndo(
            block,
            link,
            revertTopPercent,
            newTopPercent,
            restoreStartAttr,
            restoreEndAttr,
          );
        });
      })
      .catch(function () {
        revert();
        applyOutcome(copy.outcomes.unavailable);
      });
  }

  // Undo posts the inverse mutation — the exact pre-move timestamps
  // `performMove` closed over, restoring both together. Carries the
  // lock value the confirming move just returned (`block.dataset
  // .updatedAt`, already synced by `syncBlockState`), per the handoff
  // §4's own requirement.
  function performUndo(
    block,
    link,
    restoreTopPercent,
    currentTopPercent,
    restoreStartAttr,
    restoreEndAttr,
  ) {
    var projectId = block.dataset.projectId;
    var issueId = block.dataset.issueId;
    var clientUpdatedAt = block.dataset.updatedAt;

    block.style.top = restoreTopPercent + "%"; // optimistic

    function reapply() {
      block.style.top = currentTopPercent + "%";
    }

    postSchedule(projectId, issueId, restoreStartAttr, restoreEndAttr, clientUpdatedAt)
      .then(function (res) {
        if (res.status === copy.outcomes.conflictStatus) {
          reapply();
          applyOutcome(copy.outcomes.conflict);
          return false;
        }
        if (!res.ok) {
          reapply();
          applyOutcome(copy.outcomes.unavailable);
          return false;
        }
        return res.json();
      })
      .then(function (body) {
        if (body === false) return;
        if (
          !body ||
          typeof body.updated_at !== "string" ||
          !body.updated_at ||
          typeof body.time_label !== "string" ||
          typeof body.announcement !== "string"
        ) {
          reapply();
          applyOutcome(copy.outcomes.unconfirmed);
          return;
        }
        syncBlockState(block, link, body.updated_at, restoreStartAttr, restoreEndAttr, body.time_label);
        announcePolite(body.announcement);
      })
      .catch(function () {
        reapply();
        applyOutcome(copy.outcomes.unavailable);
      });
  }

  blocks.forEach(attachBlock);
})();
