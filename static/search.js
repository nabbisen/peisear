// Global search typeahead — Phase A Step 4 (peisear-feature-spec
// v2.1 §4.5).
//
// Attaches to any input with `data-typeahead="global"` and renders
// suggestion rows in the sibling `[data-typeahead-dropdown]`
// container. The input continues to work as a plain form on
// submit, so JS-disabled clients still reach /search; the JS
// just adds a live preview.
//
// Behaviour:
//
// - Debounce input by ~250ms so a fast typist doesn't fire one
//   request per keystroke.
// - Cancel the in-flight request when a new keystroke comes in.
//   Otherwise a slow earlier request can clobber a fresher
//   response (the server echoes back `q` so the client-side
//   check is also defensive).
// - Skip queries shorter than 2 characters — single characters
//   match too much to be useful.
// - Keyboard navigation: Down/Up cycle through items, Enter
//   activates the focused item, Escape closes the dropdown.
// - Click outside closes the dropdown.

(function () {
  "use strict";

  // Input event debounce window. 250ms feels responsive without
  // hammering the server during fast typing.
  const DEBOUNCE_MS = 250;

  // Below this query length, don't even hit the API. Single
  // characters match too much (a `%a%` LIKE on every issue
  // title doesn't help anyone).
  const MIN_QUERY_LENGTH = 2;

  // Find inputs that opted in via data-typeahead="global".
  // Scan once at script load — the navbar isn't dynamically
  // mounted, so a one-shot is enough.
  const inputs = document.querySelectorAll('input[data-typeahead="global"]');

  inputs.forEach(attachTypeahead);

  function attachTypeahead(input) {
    // The dropdown sibling lives inside the same form/container.
    // Search up from the input until we find one — keeps the JS
    // robust to small markup tweaks.
    const dropdown = findDropdown(input);
    if (!dropdown) return;

    let timeoutId = null;
    let abortController = null;
    let activeIndex = -1;

    input.addEventListener("input", function () {
      // Cancel any pending fetch and any pending debounce.
      if (timeoutId !== null) {
        clearTimeout(timeoutId);
        timeoutId = null;
      }
      if (abortController) {
        abortController.abort();
        abortController = null;
      }

      const q = input.value.trim();
      if (q.length < MIN_QUERY_LENGTH) {
        hideDropdown(dropdown);
        return;
      }

      timeoutId = setTimeout(function () {
        timeoutId = null;
        runQuery(q);
      }, DEBOUNCE_MS);
    });

    input.addEventListener("keydown", function (event) {
      if (event.key === "ArrowDown") {
        event.preventDefault();
        moveActive(1);
      } else if (event.key === "ArrowUp") {
        event.preventDefault();
        moveActive(-1);
      } else if (event.key === "Enter") {
        // If the user has navigated into the dropdown with the
        // arrow keys, Enter activates that item rather than
        // submitting the form.
        if (activeIndex >= 0) {
          const link = currentItem();
          if (link) {
            event.preventDefault();
            window.location.href = link.href;
          }
        }
        // Otherwise let the form's default submit go through —
        // the user is asking for the full results page.
      } else if (event.key === "Escape") {
        hideDropdown(dropdown);
        activeIndex = -1;
      }
    });

    // Click outside the input or dropdown closes the dropdown.
    document.addEventListener("click", function (event) {
      if (event.target === input) return;
      if (dropdown.contains(event.target)) return;
      hideDropdown(dropdown);
    });

    function runQuery(q) {
      abortController = new AbortController();
      fetch("/api/search?q=" + encodeURIComponent(q), {
        signal: abortController.signal,
        // Cookie-based auth is sent automatically; this is here
        // for clarity.
        credentials: "same-origin",
        headers: { Accept: "application/json" },
      })
        .then(function (resp) {
          if (!resp.ok) {
            throw new Error("search request failed: " + resp.status);
          }
          return resp.json();
        })
        .then(function (data) {
          // The server echoes `q`. If the user typed more
          // characters since this request was sent, drop the
          // stale response.
          if (data.q && data.q.trim() !== q) return;
          render(data);
        })
        .catch(function (err) {
          // AbortError is expected when a newer keystroke
          // supersedes this one.
          if (err.name === "AbortError") return;
          // Any other error: keep the dropdown closed rather
          // than showing a confusing error toast for a search
          // typeahead.
          hideDropdown(dropdown);
        });
    }

    function render(data) {
      const projects = (data.projects || []).map(function (p) {
        return (
          '<li role="option">' +
          '<a class="block px-3 py-2 hover:bg-base-200 typeahead-item" href="' +
          escapeAttr(p.url) +
          '">' +
          '<div class="font-medium">' +
          escapeText(p.name) +
          "</div>" +
          '<div class="text-xs text-base-content/60">Project</div>' +
          "</a></li>"
        );
      });

      const issues = (data.issues || []).map(function (i) {
        return (
          '<li role="option">' +
          '<a class="block px-3 py-2 hover:bg-base-200 typeahead-item" href="' +
          escapeAttr(i.url) +
          '">' +
          '<div class="font-medium">' +
          escapeText(i.title) +
          "</div>" +
          '<div class="text-xs text-base-content/60">Open issue · ' +
          escapeText(i.project_name) +
          "</div>" +
          "</a></li>"
        );
      });

      if (projects.length === 0 && issues.length === 0) {
        dropdown.innerHTML =
          '<div class="px-3 py-2 text-sm text-base-content/60 italic">' +
          "No matches" +
          "</div>";
        showDropdown(dropdown);
        activeIndex = -1;
        return;
      }

      let html = "";
      if (projects.length > 0) {
        html +=
          '<div class="px-3 pt-2 pb-1 text-xs uppercase tracking-wide text-base-content/60">' +
          "Projects" +
          "</div>";
        html += '<ul class="menu p-0">' + projects.join("") + "</ul>";
      }
      if (issues.length > 0) {
        html +=
          '<div class="px-3 pt-2 pb-1 text-xs uppercase tracking-wide text-base-content/60">' +
          "Open issues" +
          "</div>";
        html += '<ul class="menu p-0">' + issues.join("") + "</ul>";
      }
      dropdown.innerHTML = html;
      showDropdown(dropdown);
      activeIndex = -1;
    }

    function moveActive(delta) {
      const items = dropdown.querySelectorAll(".typeahead-item");
      if (items.length === 0) return;
      activeIndex = (activeIndex + delta + items.length) % items.length;
      items.forEach(function (el, i) {
        if (i === activeIndex) {
          el.classList.add("bg-base-200");
          el.scrollIntoView({ block: "nearest" });
        } else {
          el.classList.remove("bg-base-200");
        }
      });
    }

    function currentItem() {
      const items = dropdown.querySelectorAll(".typeahead-item");
      if (activeIndex < 0 || activeIndex >= items.length) return null;
      return items[activeIndex];
    }
  }

  function findDropdown(input) {
    let node = input.parentElement;
    while (node) {
      const dd = node.querySelector("[data-typeahead-dropdown]");
      if (dd) return dd;
      node = node.parentElement;
    }
    return null;
  }

  function showDropdown(dropdown) {
    dropdown.classList.remove("hidden");
  }
  function hideDropdown(dropdown) {
    dropdown.classList.add("hidden");
    dropdown.innerHTML = "";
  }

  // Prevent HTML injection of search results into the dropdown.
  // The /api/search endpoint already serialises strings safely,
  // but we render via innerHTML so we belt-and-braces escape on
  // the client too.
  function escapeText(s) {
    return String(s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#39;");
  }
  function escapeAttr(s) {
    return escapeText(s);
  }
})();
