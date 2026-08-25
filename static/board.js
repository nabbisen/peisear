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
    typeof copy.conflictMessage !== "string" ||
    typeof copy.unavailableMessage !== "string" ||
    !copy.movedTo ||
    typeof copy.undoLabel !== "string"
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

  function removeToast(card) {
    var toast = card._boardToast;
    if (!toast) return;
    clearTimeout(toast.timer);
    if (toast.el.parentNode) toast.el.parentNode.removeChild(toast.el);
    card._boardToast = null;
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
      removeToast(card);
      onUndo();
    });

    alertBox.appendChild(text);
    alertBox.appendChild(undoButton);
    toast.appendChild(alertBox);
    document.body.appendChild(toast);

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
  function performUndo(card, issueId, targetStatus, moveBack, moveForward) {
    moveBack();
    postStatus(targetStatus, card.dataset.updatedAt, issueId)
      .then(function (res) {
        if (res.status === 409) {
          moveForward();
          announceAssertive(copy.conflictMessage);
          window.location.reload();
          return;
        }
        if (!res.ok) {
          moveForward();
          announceAssertive(copy.unavailableMessage);
          return;
        }
        return res.json();
      })
      .then(function (body) {
        if (!body || typeof body.updated_at !== "string" || !body.updated_at) return;
        card.dataset.updatedAt = body.updated_at;
        announcePolite(copy.movedTo[targetStatus]);
      })
      .catch(function () {
        moveForward();
        announceAssertive(copy.unavailableMessage);
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

      postStatus(newStatus, clientUpdatedAt, issueId)
        .then(function (res) {
          if (res.status === 409) {
            revert();
            announceAssertive(copy.conflictMessage);
            // No automatic retry. Reload to pick up authoritative
            // state (fresh updated_at values on every card).
            window.location.reload();
            return;
          }
          if (!res.ok) {
            revert();
            announceAssertive(copy.unavailableMessage);
            return;
          }
          return res.json();
        })
        .then(function (body) {
          if (!body || typeof body.updated_at !== "string" || !body.updated_at) return;
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
          announceAssertive(copy.unavailableMessage);
        });
    });
  });
})();
