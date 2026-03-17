const pages = {
  integrations: {
    title: "Integrations",
    controls: [
      {
        type: "button",
        label: "Add Custom MCP",
        className: "btn btn-outline",
      },
    ],
  },
  spaces: {
    title: "Spaces",
    controls: [
      {
        type: "button",
        label: "Create space",
        className: "btn btn-primary",
      },
    ],
  },
  databases: {
    title: "Databases",
    controls: [
      {
        type: "button",
        label: "Create database",
        className: "btn btn-primary",
      },
    ],
  },
  "scheduled-tasks": {
    title: "Scheduled Tasks",
    controls: [
      {
        type: "button",
        label: "New task",
        className: "btn btn-primary",
      },
    ],
  },
  team: {
    title: "Team",
    controls: [
      {
        type: "button",
        label: "Invite members",
        className: "btn btn-primary",
      },
    ],
  },
  knowledge: {
    title: "Knowledge",
    controls: [
      {
        type: "button",
        label: "Create knowledge item",
        className: "btn btn-primary",
      },
    ],
  },
  "knowledge-item": {
    title: "Knowledge Item",
    controls: [
      {
        type: "button",
        label: "Add attachment",
        className: "btn btn-outline",
      },
      {
        type: "button",
        label: "Save",
        className: "btn btn-primary",
      },
    ],
  },
  memory: {
    title: "Memory",
    controls: [
      {
        type: "button",
        label: "Add memory",
        className: "btn btn-primary",
      },
    ],
  },
  usage: {
    title: "Usage",
    controls: [
      {
        type: "select",
        className: "select select-bordered",
        options: ["This month", "Last month", "Last 3 months"],
      },
    ],
  },
  settings: {
    title: "Settings",
    controls: [
      {
        type: "button",
        label: "Save changes",
        className: "btn btn-primary",
      },
    ],
  },
};

function getPageFromLocation() {
  const params = new URLSearchParams(window.location.search);
  return params.get("page") || "integrations";
}

function setActiveLink(page) {
  document.querySelectorAll(".wireframe-link").forEach((link) => {
    link.classList.toggle("menu-active", link.dataset.page === page);
  });
}

function renderActions(page) {
  const actions = document.getElementById("page-actions");
  actions.innerHTML = "";

  for (const control of pages[page].controls) {
    if (control.type === "button") {
      const button = document.createElement("button");
      button.className = control.className;
      button.textContent = control.label;
      actions.appendChild(button);
      continue;
    }

    if (control.type === "select") {
      const select = document.createElement("select");
      select.className = control.className;

      for (const optionLabel of control.options) {
        const option = document.createElement("option");
        option.textContent = optionLabel;
        select.appendChild(option);
      }

      actions.appendChild(select);
    }
  }
}

async function loadPage(page, pushState) {
  const safePage = pages[page] ? page : "integrations";
  const title = document.getElementById("page-title");
  const content = document.getElementById("page-content");
  const error = document.getElementById("router-error");

  title.textContent = pages[safePage].title;
  setActiveLink(safePage);
  renderActions(safePage);
  error.classList.add("hidden");

  try {
    const response = await fetch(`./${safePage}.html`, { cache: "no-store" });

    if (!response.ok) {
      throw new Error(`Failed to load ${safePage}.html`);
    }

    content.innerHTML = await response.text();

    if (pushState) {
      const url = new URL(window.location.href);
      url.searchParams.set("page", safePage);
      window.history.pushState({ page: safePage }, "", url);
    }
  } catch (err) {
    content.innerHTML = "";
    error.textContent =
      "Could not load the page partial. Serve the wireframe directory over HTTP instead of opening index.html with file://.";
    error.classList.remove("hidden");
    console.error(err);
  }
}

document.addEventListener("click", (event) => {
  const link = event.target.closest("[data-page]");
  if (!link) {
    return;
  }

  event.preventDefault();
  loadPage(link.dataset.page, true);
});

document.addEventListener("click", (event) => {
  const card = event.target.closest("[data-route]");
  if (!card) {
    return;
  }

  event.preventDefault();
  loadPage(card.dataset.route, true);
});

window.addEventListener("popstate", (event) => {
  loadPage(event.state?.page || getPageFromLocation(), false);
});

loadPage(getPageFromLocation(), false);
