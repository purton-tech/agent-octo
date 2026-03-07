function clamp(n, min, max) {
  return Math.max(min, Math.min(max, n));
}

function sceneProgress(section) {
  const rect = section.getBoundingClientRect();
  const total = section.offsetHeight - window.innerHeight;
  if (total <= 0) return 0;
  const traveled = -rect.top;
  return clamp(traveled / total, 0, 1);
}

function updateSection(section) {
  const progress = sceneProgress(section);
  const messages = Array.from(section.querySelectorAll('.msg'));
  if (messages.length === 0) return;

  // Delay reveal so first message does not appear immediately.
  const revealStart = 0.12;
  const revealProgress = clamp((progress - revealStart) / (1 - revealStart), 0, 1);

  // Discrete reveal steps: 0,1,2,...N messages.
  const visibleCount = Math.floor(revealProgress * (messages.length + 1));
  messages.forEach((msg, idx) => {
    if (idx < visibleCount) {
      msg.classList.add('visible');
    } else {
      msg.classList.remove('visible');
    }
  });
}

function tick() {
  const sections = document.querySelectorAll('.story');
  sections.forEach(updateSection);
}

window.addEventListener('scroll', tick, { passive: true });
window.addEventListener('resize', tick);
window.addEventListener('load', tick);
