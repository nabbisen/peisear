// Direct-manipulation status enhancement — RFC 004a step 2
// (STATUS-002), over STATUS-001's real `<form>`-based status
// controls on the issue detail page and the issue list.
//
// Every `.js-status-form` gets its submit intercepted and sent as a
// background POST to the same JSON endpoint `board.js` uses
// (`/projects/{id}/issues/{id}/status`), updating in place instead
// of reloading, with a 5-second Undo toast.
//
// The one rule everything else here serves: any failure falls back
// to submitting the form natively — not an error message, not a
// console log, the page-load path STATUS-001 already proved works
// without scripting. Every failure funnels into the single
// `fallback()` call at the bottom of the fetch chain, checkable by
// reading top to bottom — the test harness drives HTTP, not
// JavaScript, so this file has no coverage of its own (STATUS-002
// §7).
//
// No copy is authored here. `prose_scan` covers `components/` and
// `handlers/` only, so a string literal here is invisible to it;
// every string this script can show comes from `components/
// issues.rs` (`render_status_enhancement_assets`) as a JSON island
// at `#status-enhancement-copy`, read once at load.
(function () {
  "use strict";

  // Feature-detect rather than branch inside the handler. If either
  // is missing, every `.js-status-form` on the page is left exactly
  // as STATUS-001 built it — a plain form, no listener attached, so
  // there is no enhancement code path that could later fail open
  // because there is no enhancement at all.
  if (
    typeof window.fetch !== "function" ||
    typeof HTMLFormElement.prototype.requestSubmit !== "function"
  ) {
    return;
  }

  var copyEl = document.getElementById("status-enhancement-copy");
  if (!copyEl) return;

  var copy;
  try {
    copy = JSON.parse(copyEl.textContent);
  } catch (e) {
    return;
  }
  if (
    !copy ||
    !copy.movedTo ||
    typeof copy.undoLabel !== "string" ||
    typeof copy.conflictMessage !== "string"
  ) {
    return;
  }

  var forms = document.querySelectorAll(".js-status-form");
  if (!forms.length) return;

  function announce(message) {
    var region = document.getElementById("board-status");
    if (region) region.textContent = message;
  }

  function statusUrl(projectId, issueId) {
    return (
      "/projects/" +
      encodeURIComponent(projectId) +
      "/issues/" +
      encodeURIComponent(issueId) +
      "/status"
    );
  }

  // The one funnel every failure path reaches. `requestSubmit`, not
  // `submit` — it includes the clicked segment's `name=status
  // value=…` pair, which a bare `form.submit()` would drop. `dmBypass`
  // tells this form's own listener to step aside for the native
  // submission this triggers.
  function fallback(form, submitter) {
    form.dataset.dmBypass = "1";
    form.requestSubmit(submitter);
  }

  function setPressed(form, statusValue) {
    var buttons = form.querySelectorAll('button[name="status"]');
    for (var i = 0; i < buttons.length; i++) {
      var btn = buttons[i];
      var isCurrent = btn.value === statusValue;
      btn.setAttribute("aria-pressed", isCurrent ? "true" : "false");
      if (isCurrent) {
        btn.classList.remove("btn-ghost");
        btn.classList.add("btn-primary");
      } else {
        btn.classList.remove("btn-primary");
        btn.classList.add("btn-ghost");
      }
    }
  }

  function removeToast(form) {
    var toast = form._dmToast;
    if (!toast) return;
    clearTimeout(toast.timer);
    if (toast.el.parentNode) toast.el.parentNode.removeChild(toast.el);
    form._dmToast = null;
  }

  function showUndoToast(form, message, onUndo) {
    removeToast(form);

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
      removeToast(form);
      onUndo();
    });

    alertBox.appendChild(text);
    alertBox.appendChild(undoButton);
    toast.appendChild(alertBox);
    document.body.appendChild(toast);

    var timer = setTimeout(function () {
      removeToast(form);
    }, 5000);

    form._dmToast = { el: toast, timer: timer };
  }

  // Shared by the original change and by undo — undo is a second
  // call to this same endpoint, target status = the value restored.
  function postStatus(projectId, issueId, statusValue, clientUpdatedAt) {
    return fetch(statusUrl(projectId, issueId), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        status: statusValue,
        client_updated_at: clientUpdatedAt,
      }),
    });
  }

  function attachStatusForm(form) {
    var projectId = form.dataset.projectId;
    var issueId = form.dataset.issueId;
    var clientInput = form.elements["client_updated_at"];
    // No lock value, or no id to build the endpoint from — leave
    // this form unenhanced rather than intercept it into a request
    // that cannot succeed.
    if (!projectId || !issueId || !clientInput) return;

    form.addEventListener("submit", function (e) {
      if (form.dataset.dmBypass === "1") {
        delete form.dataset.dmBypass;
        return; // our own fallback's native submission — step aside
      }

      var submitter = e.submitter;
      if (!submitter || submitter.name !== "status" || !submitter.value) {
        return; // not a segment click we recognise — native path
      }

      var currentButton = form.querySelector('[aria-pressed="true"]');
      var previousStatus = currentButton ? currentButton.value : null;
      var newStatus = submitter.value;
      var clientUpdatedAt = clientInput.value;
      if (!previousStatus || !clientUpdatedAt) return; // native path

      e.preventDefault();

      postStatus(projectId, issueId, newStatus, clientUpdatedAt)
        .then(function (res) {
          if (!res.ok) throw new Error("status-change-not-ok");
          return res.json();
        })
        .then(function (body) {
          if (!body || typeof body.updated_at !== "string" || !body.updated_at) {
            throw new Error("status-change-bad-shape");
          }
          clientInput.value = body.updated_at;
          setPressed(form, newStatus);
          var message = copy.movedTo[newStatus];
          announce(message);

          showUndoToast(form, message, function () {
            performUndo(form, projectId, issueId, previousStatus, clientInput);
          });
        })
        .catch(function () {
          fallback(form, submitter);
        });
    });
  }

  // Undo has no form to fall back to — it exists only because this
  // script exists. Any failure here (conflict or otherwise) takes the
  // posture `board.js` takes when it has no form either: announce,
  // then reload for authoritative state. No retry, no force
  // (umbrella requirement 5).
  function performUndo(form, projectId, issueId, previousStatus, clientInput) {
    postStatus(projectId, issueId, previousStatus, clientInput.value)
      .then(function (res) {
        if (!res.ok) throw new Error("undo-not-ok");
        return res.json();
      })
      .then(function (body) {
        if (!body || typeof body.updated_at !== "string" || !body.updated_at) {
          throw new Error("undo-bad-shape");
        }
        clientInput.value = body.updated_at;
        setPressed(form, previousStatus);
        announce(copy.movedTo[previousStatus]);
      })
      .catch(function () {
        announce(copy.conflictMessage);
        window.location.reload();
      });
  }

  forms.forEach(attachStatusForm);
})();
