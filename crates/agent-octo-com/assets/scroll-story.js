function octoClamp(value, min, max) {
  return Math.max(min, Math.min(max, value));
}

function octoSceneProgress(section) {
  const rect = section.getBoundingClientRect();
  const total = section.offsetHeight - window.innerHeight;
  if (total <= 0) return 0;
  const traveled = -rect.top;
  return octoClamp(traveled / total, 0, 1);
}

function updateOctoStory(section) {
  const progress = octoSceneProgress(section);
  const messages = Array.from(
    section.querySelectorAll('[data-octo-story="message"]'),
  );

  if (messages.length === 0) return;

  const revealStart = 0.12;
  const revealProgress = octoClamp(
    (progress - revealStart) / (1 - revealStart),
    0,
    1,
  );
  const visibleCount = Math.floor(revealProgress * (messages.length + 1));

  messages.forEach((message, index) => {
    if (index < visibleCount) {
      message.classList.add("visible");
    } else {
      message.classList.remove("visible");
    }
  });
}

function tickOctoStories() {
  const sections = document.querySelectorAll('[data-octo-story="scene"]');
  sections.forEach(updateOctoStory);
}

window.addEventListener("scroll", tickOctoStories, { passive: true });
window.addEventListener("resize", tickOctoStories);
window.addEventListener("load", tickOctoStories);
