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
//
// Round 2 (`PLAN-002-review.md`): round 1 set a moved row's
// `data-plan-move` to the action just performed, which is that
// value's *opposite* on every column by construction -- the row
// carried a fact about where it had come from, not where it was
// sitting, so a second drag of the same row (no reload in between)
// silently failed its `data-plan-move !== data-plan-drop` guard and
// did nothing. `applyMove`/`performUndo` now read the row's new
// value straight off the column it lands in (`data-plan-row-move`,
// rendered server-side beside `data-plan-drop`) rather than
// inverting anything client-side, and `syncRowForm` brings the row's
// own form -- action, the `project_id` field, the button's label --
// into line with its new position, so the fallback path and a plain
// click on the still-visible button both stay correct after any
// number of drags.
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
    typeof copy.undoUnavailableMessage !== "string" ||
    typeof copy.sprintButtonLabel !== "string" ||
    typeof copy.backlogButtonLabel !== "string"
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

  // Round 2 (`PLAN-002-review.md` §6c): brings the row's own form --
  // its fallback path -- into line with its new position after a
  // confirmed move, so the fallback (and a plain click on the
  // still-visible button) submits to the right place. `newMove` is
  // the value the row now carries (`data-plan-row-move` of the
  // column it's sitting in, never computed by inverting anything
  // here -- see the row/column marker comments in
  // `components/sprint_plan.rs`). `newUrl` is always the *other*
  // column's `data-plan-url` -- the endpoint a further move from here
  // would need, which is also exactly what undoing the move just
  // performed would post to. The button's label comes from the copy
  // island (`sprintButtonLabel`/`backlogButtonLabel`), never authored
  // here -- this is real DOM work, not markup authorship, the same
  // distinction `ensureList`/`ensureEmpty` already draw.
  function syncRowForm(row, newMove, newUrl) {
    var form = row.querySelector("form");
    if (!form) return;
    form.action = newUrl;

    var projectIdInput = form.querySelector('input[name="project_id"]');
    if (newMove === "add") {
      // `plan_add` requires `project_id`; `plan_remove` doesn't
      // declare the field at all (`PlanRemoveForm`).
      if (!projectIdInput) {
        projectIdInput = document.createElement("input");
        projectIdInput.type = "hidden";
        projectIdInput.name = "project_id";
        form.appendChild(projectIdInput);
      }
      projectIdInput.value = row.dataset.planProjectId;
    } else if (projectIdInput) {
      projectIdInput.remove();
    }

    var button = form.querySelector('button[type="submit"]');
    if (button) {
      button.textContent = newMove === "add" ? copy.sprintButtonLabel : copy.backlogButtonLabel;
    }
  }

  // `A11Y-004` (`NFR-A11Y-002`): when a toast ends -- Undo pressed or the
  // five seconds up -- and focus is *inside it*, focus returns to the control
  // that was acted on instead of falling to `body`. **Conditional on focus being
  // inside the toast**: a user who has kept working elsewhere is not touched (an
  // unconditional restore, or focusing the toast when it appears, would steal
  // focus, and would make a second Enter on a status button press Undo).
  function toastOrigin(row) {
    return row.querySelector("button, a");
  }

  // If the origin is gone (its element was removed from the page), focus goes to
  // the first control inside the same holder, and failing that to `<main>` (made
  // focusable with tabindex -1): always somewhere on the page the user was
  // working in, never `body`.
  function restoreFocus(row) {
    var target = toastOrigin(row);
    if (!target || !document.contains(target)) {
      target = document.contains(row) ? row.querySelector('a[href], button:not([disabled])') : null;
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
  function removeToast(row, deferRestore) {
    var toast = row._planToast;
    if (!toast) return;
    clearTimeout(toast.timer);
    var hadFocus = toast.el.contains(document.activeElement);
    if (toast.el.parentNode) toast.el.parentNode.removeChild(toast.el);
    row._planToast = null;
    if (hadFocus && !deferRestore) restoreFocus(row);
    return hadFocus;
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
      var refocus = removeToast(row, true);
      // The undo moves the row back only after its request returns, so focus is
      // restored now (never `body` in between) and again once the row has moved
      // (`performUndo`), which would otherwise blur it. Cleared there.
      row._planRefocus = refocus;
      onUndo();
      if (refocus) restoreFocus(row);
    });

    alertBox.appendChild(text);
    alertBox.appendChild(undoButton);
    toast.appendChild(alertBox);
    // `A11Y-004`: in DOM order right after the acted-on control, so Undo is the
    // next Tab stop -- not appended to the end of `<body>`. This is a DOM-order
    // change only: `.toast` is `position: fixed`, so it is still drawn
    // bottom-right and takes no space in the layout it now sits inside.
    row.appendChild(toast);

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
  //
  // `undoContainer` is where the row must go back to; `currentContainer`
  // is where it sits right now, pre-undo (the destination the
  // original move landed it in).
  function performUndo(row, undoContainer, currentContainer) {
    var performedMove = undoContainer.dataset.planDrop; // the action *this* undo performs
    var undoUrl = undoContainer.dataset.planUrl;
    var body = new URLSearchParams();
    body.set("issue_id", row.dataset.planIssueId);
    if (performedMove === "add") {
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
        if (row._planRefocus) {
          row._planRefocus = false;
          restoreFocus(row);
        }
        ensureEmpty(currentContainer, emptyMessageFor(currentContainer));
        // Round 2 (`PLAN-002-review.md` §4/§6a): the row's new
        // `data-plan-move` is `undoContainer`'s own
        // `data-plan-row-move` -- the fact the server already
        // renders for "a row sitting here" -- never `performedMove`
        // (the action just taken) and never computed by inverting
        // it. Those two coincided in round 1's code by accident and
        // diverge the moment a row is dragged, undone, then dragged
        // again.
        var newMove = undoContainer.dataset.planRowMove;
        row.dataset.planMove = newMove;
        syncRowForm(row, newMove, currentContainer.dataset.planUrl);
        announcePolite(movedMessage(performedMove));
      })
      .catch(function () {
        announceAssertive(copy.undoUnavailableMessage);
        window.location.reload();
      });
  }

  // The mutation already succeeded server-side here. `performedMove`
  // is the action just taken (for the announcement); the row's new
  // `data-plan-move` is a separate fact -- `destContainer`'s own
  // `data-plan-row-move` (round 2, same distinction `performUndo`
  // draws above) -- read directly rather than derived from
  // `performedMove`, which is its opposite on every column by
  // construction and was round 1's defect.
  function applyMove(row, performedMove, originContainer, destContainer) {
    try {
      var newMove = destContainer.dataset.planRowMove;
      row.dataset.planMove = newMove;
      syncRowForm(row, newMove, originContainer.dataset.planUrl);
      var message = movedMessage(performedMove);
      announcePolite(message);
      showUndoToast(row, message, function () {
        performUndo(row, originContainer, destContainer);
      });
    } catch (e) {
      announcePolite(movedMessage(performedMove));
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

      var performedMove = column.dataset.planDrop;
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
      //
      // Round 2 (`PLAN-002-review.md` §6b): posts `column.dataset
      // .planUrl`, the destination column's own server-rendered URL
      // -- not `form.action`. They agree on a row's first drag, but
      // not after a confirmed move has left the row's own form
      // stale until `syncRowForm` catches it up; `performUndo`
      // already posted its own container's URL for the same reason.
      fetch(column.dataset.planUrl, { method: "POST", body: body, redirect: "manual" })
        .then(function (res) {
          if (res.type !== "opaqueredirect") {
            throw new Error("plan-move-not-redirect");
          }
          applyMove(row, performedMove, originContainer, destContainer);
        })
        .catch(function () {
          revertMove(row, originContainer, destContainer);
          fallback(form);
        });
    });
  });
})();
