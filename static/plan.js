// Drag between the backlog and the sprint on the sprint plan page
// (`PLAN-002`, RFC 004c D-4). The move buttons stay exactly as
// `PLAN-001` built them -- the no-JavaScript path, the keyboard path
// and the touch path -- and this script adds a drag affordance over
// the same two endpoints (`.../plan/add`, `.../plan/remove`) rather
// than replacing them.
//
// The rule everything else here serves, same as `dm.js`'s: any
// failure *before the mutation lands* falls back to a native submit
// of the dragged row's own move form -- the page-load path already
// proved works without scripting. Every such failure funnels into
// the single `fallback()` call inside the drop handler's fetch
// chain, checkable top to bottom. Once the server has applied the
// change there is nothing to fall back to: `applyMove` never
// resubmits, only announces and, if it can't update the page in
// place, reloads instead.
//
// **No `outcomes` block, unlike `dm.js`/`board.js`.** Requirement 6
// forbids a conflict path and `sprint_issues` carries no
// `updated_at` for one to check against, so there is no 409 to
// classify (`PLAN-002` §3.5). Every failure this script can see is
// either "before the mutation, fall back" or "undo failed, reload".
//
// No copy is authored here -- `static_js_scan` covers `static/*.js`
// with no allowlist entry for this file. Every string this script
// can show comes from `components/sprint_plan.rs`
// (`render_plan_copy_assets`) as a JSON island at `#plan-copy`, read
// once at load.
(function () {
  "use strict";

  if (typeof window.fetch !== "function") {
    return;
  }

  var copyEl = document.getElementById("plan-copy");
  if (!copyEl) return;

  var copy;
  try {
    copy = JSON.parse(copyEl.textContent);
  } catch (e) {
    return;
  }
  if (
    !copy ||
    typeof copy.movedToSprint !== "string" ||
    typeof copy.movedToBacklog !== "string" ||
    typeof copy.undoLabel !== "string" ||
    typeof copy.noBacklogIssuesMessage !== "string" ||
    typeof copy.noSprintItemsMessage !== "string" ||
    typeof copy.undoUnavailableMessage !== "string"
  ) {
    return;
  }

  var rows = document.querySelectorAll("[data-plan-move]");
  var columns = document.querySelectorAll("[data-plan-drop]");
  if (!rows.length || !columns.length) return;

  // `QA-011` §2 (`NFR-A11Y-008`): a success announcement is polite; a
  // failed undo is assertive -- both ids shared with `dm.js`/`board.js`.
  function announcePolite(message) {
    var region = document.getElementById("status-announcements");
    if (region) region.textContent = message;
  }

  function announceAssertive(message) {
    var region = document.getElementById("status-announcements-assertive");
    if (region) region.textContent = message;
  }

  function movedMessage(move) {
    return move === "add" ? copy.movedToSprint : copy.movedToBacklog;
  }

  function emptyMessageFor(container) {
    return container.dataset.planDrop === "add"
      ? copy.noSprintItemsMessage
      : copy.noBacklogIssuesMessage;
  }

  // The one funnel every *pre-mutation* failure reaches, same shape
  // as `dm.js`'s `fallback()`. `dataset.planBypass` lets the row's
  // own form step aside for this native submission -- see
  // `attachRowForm` below for why a listener is there at all to step
  // aside from.
  function fallback(form) {
    form.dataset.planBypass = "1";
    form.requestSubmit();
  }

  function attachRowForm(row) {
    var form = row.querySelector("form");
    if (!form) return;
    form.addEventListener("submit", function () {
      if (form.dataset.planBypass === "1") {
        delete form.dataset.planBypass;
        return; // our own fallback's native submission -- step aside
      }
      // A genuine click on the still-visible move button: left
      // completely alone, same as before this script existed (§1).
    });
  }

  function findList(container) {
    return container.querySelector("[data-plan-list]");
  }

  function findEmpty(container) {
    return container.querySelector("[data-plan-empty]");
  }

  // Ensures `container` has a visible list to append into, creating
  // one (replacing the empty-state paragraph) if only that paragraph
  // is currently rendered. No copy authored here -- an empty `<ul>`
  // carries no text.
  function ensureList(container) {
    var ul = findList(container);
    if (ul) return ul;
    ul = document.createElement("ul");
    ul.className = "divide-y";
    ul.setAttribute("data-plan-list", "");
    var empty = findEmpty(container);
    if (empty && empty.parentNode) {
      empty.parentNode.replaceChild(ul, empty);
    } else {
      container.appendChild(ul);
    }
    return ul;
  }

  // Ensures `container` shows its empty state, creating the
  // paragraph (replacing the now-empty list) if the list has no rows
  // left. `message` always comes from the copy island -- never
  // authored here.
  function ensureEmpty(container, message) {
    var ul = findList(container);
    if (!ul || ul.children.length > 0) return;
    var p = document.createElement("p");
    p.className = "text-sm text-base-content/70 italic";
    p.setAttribute("data-plan-empty", "");
    p.textContent = message;
    ul.parentNode.replaceChild(p, ul);
  }

  function removeToast(row) {
    var toast = row._planToast;
    if (!toast) return;
    clearTimeout(toast.timer);
    if (toast.el.parentNode) toast.el.parentNode.removeChild(toast.el);
    row._planToast = null;
  }

  function showUndoToast(row, message, onUndo) {
    removeToast(row);

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
      removeToast(row);
      onUndo();
    });

    alertBox.appendChild(text);
    alertBox.appendChild(undoButton);
    toast.appendChild(alertBox);
    document.body.appendChild(toast);

    var timer = setTimeout(function () {
      removeToast(row);
    }, 5000);

    row._planToast = { el: toast, timer: timer };
  }

  // Puts `row` back in its origin column, re-creating that column's
  // list if the optimistic move had emptied and replaced it, and
  // restoring the destination's empty state if the move had been its
  // only row. Does not restore `row`'s exact prior position within
  // the list -- the fallback this always precedes reloads the page
  // right after, so ordering here is cosmetic for the moment before
  // that happens, never load-bearing.
  function revertMove(row, originContainer, destContainer) {
    ensureList(originContainer).appendChild(row);
    ensureEmpty(destContainer, emptyMessageFor(destContainer));
  }

  // Undo has no form to fall back to (`dm.js`'s rule, carried over
  // unchanged): every outcome here ends in announce + reload, never
  // a resubmit.
  function performUndo(row, undoContainer, currentContainer) {
    var undoMove = undoContainer.dataset.planDrop;
    var undoUrl = undoContainer.dataset.planUrl;
    var body = new URLSearchParams();
    body.set("issue_id", row.dataset.planIssueId);
    if (undoMove === "add") {
      body.set("project_id", row.dataset.planProjectId);
    }

    // Same `opaqueredirect`-as-success idiom as the drop handler
    // above (§3.1) -- see that comment for why.
    fetch(undoUrl, { method: "POST", body: body, redirect: "manual" })
      .then(function (res) {
        if (res.type !== "opaqueredirect") {
          throw new Error("plan-undo-not-redirect");
        }
        var destList = ensureList(undoContainer);
        destList.appendChild(row);
        ensureEmpty(currentContainer, emptyMessageFor(currentContainer));
        row.dataset.planMove = undoMove;
        announcePolite(movedMessage(undoMove));
      })
      .catch(function () {
        announceAssertive(copy.undoUnavailableMessage);
        window.location.reload();
      });
  }

  // The mutation already succeeded server-side here -- `row.dataset
  // .planMove` updates first (metadata only, never the row's visible
  // markup -- §3.4) so a second drag reads the row's new direction
  // correctly even before any reload.
  function applyMove(row, targetMove, originContainer) {
    try {
      row.dataset.planMove = targetMove;
      var message = movedMessage(targetMove);
      announcePolite(message);
      showUndoToast(row, message, function () {
        performUndo(row, originContainer, row.closest("[data-plan-drop]"));
      });
    } catch (e) {
      announcePolite(movedMessage(targetMove));
      window.location.reload();
    }
  }

  var dragging = null;

  rows.forEach(function (row) {
    attachRowForm(row);
    row.addEventListener("dragstart", function (e) {
      dragging = row;
      row.classList.add("opacity-50");
      try {
        e.dataTransfer.effectAllowed = "move";
      } catch (_) {}
    });
    row.addEventListener("dragend", function () {
      if (dragging) dragging.classList.remove("opacity-50");
      dragging = null;
    });
  });

  columns.forEach(function (column) {
    column.addEventListener("dragover", function (e) {
      if (!dragging || dragging.dataset.planMove !== column.dataset.planDrop) {
        return;
      }
      e.preventDefault();
      column.classList.add("bg-base-200");
    });
    column.addEventListener("dragleave", function () {
      column.classList.remove("bg-base-200");
    });
    column.addEventListener("drop", function (e) {
      e.preventDefault();
      column.classList.remove("bg-base-200");
      if (!dragging) return;

      // `dragging` is left set here -- `dragend` (which always fires
      // after `drop` for the same gesture) is what clears it and
      // removes the opacity class, same division of labour as
      // `board.js`.
      var row = dragging;
      if (row.dataset.planMove !== column.dataset.planDrop) return; // own column or mismatch -- no-op

      var form = row.querySelector("form");
      if (!form) return; // defensive; can_move gates both together

      var originContainer = row.closest("[data-plan-drop]");
      var destContainer = column;
      if (!originContainer || originContainer === destContainer) return;

      removeToast(row); // a still-open undo from an earlier move is now stale

      // Optimistic move.
      var destList = ensureList(destContainer);
      destList.appendChild(row);
      ensureEmpty(originContainer, emptyMessageFor(originContainer));

      var targetMove = column.dataset.planDrop;
      var body = new URLSearchParams(new FormData(form));

      // `PLAN-002` §3.1: `plan_add`/`plan_remove` both return a `303`
      // to the plan page, not JSON -- there is no lock value to hand
      // back, unlike `/status`. With `redirect: "manual"`, a fetch
      // that lands on a redirect resolves with an *opaque* response
      // (`res.type === "opaqueredirect"`, no readable status or
      // body) rather than following it -- this is what a successful
      // POST looks like here. Anything else (a real `res` with a
      // readable status -- 400/403/404, or a rejected promise) is a
      // failure and reaches `fallback()` below. Deliberately not
      // `redirect: "follow"` with `res.ok`: that would re-render and
      // discard the whole plan page server-side per move, on exactly
      // the screen where a user makes many in a row.
      fetch(form.action, { method: "POST", body: body, redirect: "manual" })
        .then(function (res) {
          if (res.type !== "opaqueredirect") {
            throw new Error("plan-move-not-redirect");
          }
          applyMove(row, targetMove, originContainer);
        })
        .catch(function () {
          revertMove(row, originContainer, destContainer);
          fallback(form);
        });
    });
  });
})();
