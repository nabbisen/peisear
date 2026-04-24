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
      var issueId = dragging.dataset.issueId;
      var newStatus = col.dataset.status;
      col.appendChild(dragging); // optimistic move

      fetch(
        "/projects/" +
          encodeURIComponent(projectId) +
          "/issues/" +
          encodeURIComponent(issueId) +
          "/status",
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ status: newStatus }),
        },
      )
        .then(function (res) {
          if (!res.ok) throw new Error("status " + res.status);
          window.location.reload();
        })
        .catch(function (err) {
          console.error("Failed to update status", err);
          alert("Failed to update status. Please refresh.");
        });
    });
  });
})();
