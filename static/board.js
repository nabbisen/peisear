// Kanban drag-and-drop for the project board view.
// The project id is injected into #board-root[data-project-id] by the
// server-rendered template, so this script has no string interpolation
// in its source (nothing the server writes into JS literal positions).
(function () {
  "use strict";
  var root = document.getElementById("board-root");
  if (!root) return;
  var projectId = root.dataset.projectId;
  if (!projectId) return;

  var dragging = null;

  // §21.4 / §1.7: messages go into this status region rather than
  // alert(); no failure vocabulary ("Failed", "Error").
  var RELOAD_MESSAGE =
    "This page is showing an earlier version of the board. Reload to see the current state.";
  var CONFLICT_MESSAGE =
    "Another member changed this issue first. The board now shows the current state.";
  var UNAVAILABLE_MESSAGE =
    "This status change could not be completed. The card has been returned to its previous column.";

  function announce(message) {
    // `STATUS-002-review.md` §5 Q3: renamed from "board-status" to
    // "status-announcements" -- the region is now shared with the
    // issue list and issue detail pages too, so it is named for what
    // it is rather than where it used to live only. Permitted as a
    // one-line exception to "no change to board.js" (§10): that
    // prohibition keeps the board's behaviour out of STATUS-002, and
    // renaming an id it reads is not a behaviour change.
    var region = document.getElementById("status-announcements");
    if (region) region.textContent = message;
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
      var originalColumn = card.parentElement;
      var originalNextSibling = card.nextSibling;

      function revert() {
        if (originalNextSibling) {
          originalColumn.insertBefore(card, originalNextSibling);
        } else {
          originalColumn.appendChild(card);
        }
      }

      col.appendChild(card); // optimistic move

      if (!clientUpdatedAt) {
        // No lock value rendered on this card — the page is stale
        // relative to this build. Do not send a request that would
        // be rejected anyway; a silent no-op would be worse.
        revert();
        announce(RELOAD_MESSAGE);
        return;
      }

      fetch(
        "/projects/" +
          encodeURIComponent(projectId) +
          "/issues/" +
          encodeURIComponent(issueId) +
          "/status",
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            status: newStatus,
            client_updated_at: clientUpdatedAt,
          }),
        },
      )
        .then(function (res) {
          if (res.status === 409) {
            revert();
            announce(CONFLICT_MESSAGE);
            // No automatic retry. Reload to pick up authoritative
            // state (fresh updated_at values on every card).
            window.location.reload();
            return;
          }
          if (!res.ok) {
            revert();
            announce(UNAVAILABLE_MESSAGE);
            return;
          }
          window.location.reload();
        })
        .catch(function () {
          revert();
          announce(UNAVAILABLE_MESSAGE);
        });
    });
  });
})();
