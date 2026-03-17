const pageTitles = {
  dashboard: "Dashboard",
  users: "Users",
  teams: "Teams",
  billing: "Billing",
  settings: "Settings",
};

function getPageFromLocation() {
  const params = new URLSearchParams(window.location.search);
  return params.get("page") || "dashboard";
}

function setActiveLink(page) {
  document.querySelectorAll(".wireframe-link").forEach((link) => {
    link.classList.toggle("menu-active", link.dataset.page === page);
  });
}

async function loadPage(page, pushState) {
  const safePage = pageTitles[page] ? page : "dashboard";
  const title = document.getElementById("page-title");
  const content = document.getElementById("page-content");
  const error = document.getElementById("router-error");

  title.textContent = pageTitles[safePage];
  setActiveLink(safePage);
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

window.addEventListener("popstate", (event) => {
  loadPage(event.state?.page || getPageFromLocation(), false);
});

loadPage(getPageFromLocation(), false);
