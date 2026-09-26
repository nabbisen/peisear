// Kanban drag-and-drop for the project board view.
// The project id is injected into #board-root[data-project-id] by the
// server-rendered template, so this script has no string interpolation
// in its source (nothing the server writes into JS literal positions).
//
// `BOARD-001` (RFC 004b / D-2): the three announcement strings used to
// be authored here as literal `var` assignments -- outside every
// vocabulary guard this project has (`prose_scan` covers Rust;
// `static/*.js` was unexamined, not excluded). They now live in
// `peisear-i18n` and arrive as `#board-copy`, the same JSON-island
// pattern `dm.js` uses. `static_js_scan` guards against a literal
// reappearing here.
(function () {
  "use strict";
  var root = document.getElementById("board-root");
  if (!root) return;
  var projectId = root.dataset.projectId;
  if (!projectId) return;

  var copyEl = document.getElementById("board-copy");
  if (!copyEl) return;
  var copy;
  try {
    copy = JSON.parse(copyEl.textContent);
  } catch (e) {
    return;
  }
  if (
    !copy ||
    typeof copy.reloadMessage !== "string" ||
    !copy.movedTo ||
    typeof copy.undoLabel !== "string" ||
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

  var dragging = null;

  // `QA-011` §2 (`NFR-A11Y-008`): a success announcement is polite; a
  // conflict or unavailable one is assertive, so each gets its own
  // region -- both ids shared with `dm.js`
  // (`STATUS-002-review.md` §5 Q3).
  function announcePolite(message) {
    var region = document.getElementById("status-announcements");
    if (region) region.textContent = message;
  }

  function announceAssertive(message) {
    var region = document.getElementById("status-announcements-assertive");
    if (region) region.textContent = message;
  }

  // `JS-003` (RFC 011 step 2): the one place a classified outcome
  // becomes an action -- announce, then reload if the outcome says
  // to. `copy.outcomes`'s three keys (`conflict`/`unavailable`/
  // `unconfirmed`) are the classification; this is just "read it and
  // act", not policy of its own. Reverting the card, where it's
  // needed, is the caller's job -- it happens before this runs.
  function applyOutcome(outcome) {
    announceAssertive(outcome.message);
    if (outcome.reload) window.location.reload();
  }

  function postStatus(statusValue, updatedAt, issueId) {
    return fetch(
      "/projects/" +
        encodeURIComponent(projectId) +
        "/issues/" +
        encodeURIComponent(issueId) +
        "/status",
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ status: statusValue, client_updated_at: updatedAt }),
      },
    );
  }

  // `A11Y-004` (`NFR-A11Y-002`): when a toast ends -- Undo pressed or the
  // five seconds up -- and focus is *inside it*, focus returns to the control
  // that was acted on instead of falling to `body`. **Conditional on focus being
  // inside the toast**: a user who has kept working elsewhere is not touched (an
  // unconditional restore, or focusing the toast when it appears, would steal
  // focus, and would make a second Enter on a status button press Undo).
  function toastOrigin(card) {
    return card.querySelector("a");
  }

  // If the origin is gone (its element was removed from the page), focus goes to
  // the first control inside the same holder, and failing that to `<main>` (made
  // focusable with tabindex -1): always somewhere on the page the user was
  // working in, never `body`.
  function restoreFocus(card) {
    var target = toastOrigin(card);
    if (!target || !document.contains(target)) {
      target = document.contains(card) ? card.querySelector('a[href], button:not([disabled])') : null;
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
  function removeToast(card, deferRestore) {
    var toast = card._boardToast;
    if (!toast) return;
    clearTimeout(toast.timer);
    var hadFocus = toast.el.contains(document.activeElement);
    if (toast.el.parentNode) toast.el.parentNode.removeChild(toast.el);
    card._boardToast = null;
    if (hadFocus && !deferRestore) restoreFocus(card);
    return hadFocus;
  }

  function showUndoToast(card, message, onUndo) {
    removeToast(card);

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
      var refocus = removeToast(card, true);
      onUndo();
      if (refocus) restoreFocus(card);
    });

    alertBox.appendChild(text);
    alertBox.appendChild(undoButton);
    toast.appendChild(alertBox);
    // `A11Y-004`: in DOM order right after the acted-on control, so Undo is the
    // next Tab stop -- not appended to the end of `<body>`. This is a DOM-order
    // change only: `.toast` is `position: fixed`, so it is still drawn
    // bottom-right and takes no space in the layout it now sits inside.
    card.appendChild(toast);

    var timer = setTimeout(function () {
      removeToast(card);
    }, 5000);

    card._boardToast = { el: toast, timer: timer };
  }

  // Undo has no form to fall back to mid-gesture, so on failure this
  // takes the same revert-announce(-reload) posture the drag itself
  // uses (umbrella requirement 2a), never a resubmit. `moveBack`
  // undoes the drag visually right away (optimistic, same as the
  // drag's own move); `moveForward` re-applies it if the undo
  // request itself fails -- the drag did land, only the undo didn't.
  // `copy.outcomes` classifies the response (`JS-003`, RFC 011 step
  // 2); `false` from the first `.then` is a sentinel meaning "already
  // handled", distinct from a real (possibly malformed) response
  // body, so the second `.then` doesn't double-announce.
  function performUndo(card, issueId, targetStatus, moveBack, moveForward) {
    moveBack();
    postStatus(targetStatus, card.dataset.updatedAt, issueId)
      .then(function (res) {
        if (res.status === copy.outcomes.conflictStatus) {
          moveForward();
          applyOutcome(copy.outcomes.conflict);
          return false;
        }
        if (!res.ok) {
          moveForward();
          applyOutcome(copy.outcomes.unavailable);
          return false;
        }
        return res.json();
      })
      .then(function (body) {
        if (body === false) return;
        if (!body || typeof body.updated_at !== "string" || !body.updated_at) {
          moveForward();
          applyOutcome(copy.outcomes.unconfirmed);
          return;
        }
        card.dataset.updatedAt = body.updated_at;
        announcePolite(copy.movedTo[targetStatus]);
      })
      .catch(function () {
        moveForward();
        applyOutcome(copy.outcomes.unavailable);
      });
  }

  document.querySelectorAll(".issue-card").forEach(function (card) {
    card.addEventListener("dragstart", function (e) {
      dragging = card;
      card.classList.add("opacity-50");
      try { e.dataTransfer.effectAllowed = "move"; } catch (_) {}
    });
    card.addEventListener("dragend", function () {
      if (dragging) dragging.classList.remove("opacity-50");
      dragging = null;
    });
  });

  document.querySelectorAll(".column-drop").forEach(function (col) {
    col.addEventListener("dragover", function (e) {
      e.preventDefault();
      col.classList.add("bg-base-200");
    });
    col.addEventListener("dragleave", function () {
      col.classList.remove("bg-base-200");
    });
    col.addEventListener("drop", function (e) {
      e.preventDefault();
      col.classList.remove("bg-base-200");
      if (!dragging) return;

      var card = dragging;
      var issueId = card.dataset.issueId;
      var clientUpdatedAt = card.dataset.updatedAt;
      var newStatus = col.dataset.status;
      var previousStatus = card.parentElement.dataset.status;
      var originalColumn = card.parentElement;
      var originalNextSibling = card.nextSibling;

      function revert() {
        if (originalNextSibling) {
          originalColumn.insertBefore(card, originalNextSibling);
        } else {
          originalColumn.appendChild(card);
        }
      }
      function reapplyDrag() {
        col.appendChild(card);
      }

      removeToast(card); // a still-open undo from an earlier drag is now stale

      reapplyDrag(); // optimistic move

      if (!clientUpdatedAt) {
        // No lock value rendered on this card — the page is stale
        // relative to this build. Do not send a request that would
        // be rejected anyway; a silent no-op would be worse.
        revert();
        announceAssertive(copy.reloadMessage);
        return;
      }

      // `copy.outcomes` classifies the response (`JS-003`, RFC 011
      // step 2); `false` from the first `.then` is a sentinel meaning
      // "already handled", distinct from a real (possibly malformed)
      // response body, so the second `.then` doesn't double-announce
      // -- same shape as `performUndo` above.
      postStatus(newStatus, clientUpdatedAt, issueId)
        .then(function (res) {
          if (res.status === copy.outcomes.conflictStatus) {
            revert();
            // No automatic retry. Reload to pick up authoritative
            // state (fresh updated_at values on every card).
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
          if (!body || typeof body.updated_at !== "string" || !body.updated_at) {
            // A 2xx with no usable `updated_at` -- the mutation's
            // fate is unknown, so the optimistic move is reverted
            // rather than left standing on a guess, and reload picks
            // up whatever the server actually did.
            revert();
            applyOutcome(copy.outcomes.unconfirmed);
            return;
          }
          // Confirmed applied past this point -- update in place, no
          // reload, matching `dm.js`'s posture (STATUS-002).
          card.dataset.updatedAt = body.updated_at;
          var message = copy.movedTo[newStatus];
          announcePolite(message);
          showUndoToast(card, message, function () {
            performUndo(card, issueId, previousStatus, revert, reapplyDrag);
          });
        })
        .catch(function () {
          revert();
          applyOutcome(copy.outcomes.unavailable);
        });
    });
  });
})();
