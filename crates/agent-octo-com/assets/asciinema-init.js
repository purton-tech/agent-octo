(function () {
  function boot() {
    if (typeof window.AsciinemaPlayer === "undefined") return;

    document.querySelectorAll("[data-asciinema-src]").forEach(function (node) {
      if (!(node instanceof HTMLElement)) return;
      if (node.dataset.asciinemaMounted === "true") return;

      const src = node.dataset.asciinemaSrc;
      if (!src) return;

      const cols = Number(node.dataset.asciinemaCols || "100");
      const rows = Number(node.dataset.asciinemaRows || "28");
      const autoplay = node.dataset.asciinemaAutoplay === "true";
      const loop = node.dataset.asciinemaLoop === "true";

      window.AsciinemaPlayer.create(src, node, {
        cols,
        rows,
        autoPlay: autoplay,
        loop,
      });

      node.dataset.asciinemaMounted = "true";
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", boot, { once: true });
    return;
  }

  boot();
})();
