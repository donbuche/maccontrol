// MacControl landing page. Vanilla JS, no dependencies.

const $ = (sel, root = document) => root.querySelector(sel);
const $$ = (sel, root = document) => [...root.querySelectorAll(sel)];
const store = {
  get(k) { try { return localStorage.getItem(k); } catch { return null; } },
  set(k, v) { try { localStorage.setItem(k, v); } catch { /* private mode */ } },
};
const reducedMotion = matchMedia('(prefers-reduced-motion: reduce)').matches;
const pageLang = document.documentElement.lang;

document.documentElement.classList.replace('no-js', 'js');

/* ---------- Analytics ---------- */

function track(name, params = {}) {
  if (typeof window.gtag === 'function') window.gtag('event', name, { page_language: pageLang, ...params });
}

document.addEventListener('click', (e) => {
  const el = e.target.closest('[data-track]');
  if (!el) return;
  const kind = el.dataset.track;
  const location = el.dataset.trackLocation;
  if (kind === 'download') track('download_click', { link_url: el.href, location });
  if (kind === 'github') track('github_click', { link_url: el.href, location });
});

/* ---------- Consent (Google Consent Mode v2) ---------- */

const consent = $('[data-consent]');
function setConsent(value) {
  store.set('mcl-consent', value);
  if (typeof window.gtag === 'function') {
    window.gtag('consent', 'update', { analytics_storage: value });
    if (value === 'granted' && window.loadGtag) window.loadGtag();
  }
  if (consent) consent.hidden = true;
  maybeSuggestLanguage();
}
if (consent) {
  if (!store.get('mcl-consent')) consent.hidden = false;
  $$('[data-consent-choice]', consent).forEach((b) =>
    b.addEventListener('click', () => setConsent(b.dataset.consentChoice))
  );
  $$('[data-consent-open]').forEach((b) =>
    b.addEventListener('click', () => {
      consent.hidden = false;
      $('[data-consent-choice="granted"]', consent).focus();
    })
  );
}

/* ---------- Header: scrolled state and mobile menu ---------- */

const header = $('[data-header]');
const onScroll = () => header?.classList.toggle('is-scrolled', window.scrollY > 24);
onScroll();
window.addEventListener('scroll', onScroll, { passive: true });

const nav = $('[data-nav]');
const navToggle = $('[data-nav-toggle]');
function setNav(open) {
  nav?.classList.toggle('is-open', open);
  navToggle?.setAttribute('aria-expanded', String(open));
  if (open) header?.classList.add('is-scrolled');
  else onScroll();
}
navToggle?.addEventListener('click', () => setNav(navToggle.getAttribute('aria-expanded') !== 'true'));
nav?.addEventListener('click', (e) => { if (e.target.closest('a')) setNav(false); });

/* ---------- Language switcher ---------- */

const langToggle = $('[data-lang-toggle]');
const langMenu = $('#lang-menu');
function setLangMenu(open) {
  if (!langMenu) return;
  langMenu.hidden = !open;
  langToggle.setAttribute('aria-expanded', String(open));
}
langToggle?.addEventListener('click', (e) => {
  e.stopPropagation();
  setLangMenu(langMenu.hidden);
  if (!langMenu.hidden) $('a', langMenu)?.focus();
});
document.addEventListener('click', (e) => { if (!e.target.closest('[data-lang]')) setLangMenu(false); });
document.addEventListener('keydown', (e) => {
  if (e.key !== 'Escape') return;
  if (langMenu && !langMenu.hidden) { setLangMenu(false); langToggle.focus(); }
  if (nav?.classList.contains('is-open')) { setNav(false); navToggle.focus(); }
});
$$('[data-lang-link]').forEach((a) =>
  a.addEventListener('click', () => {
    store.set('mcl-lang', a.dataset.langLink);
    if (a.dataset.langLink !== pageLang) track('language_switch', { from: pageLang, to: a.dataset.langLink });
  })
);

/* ---------- Suggest the visitor's language (never redirect) ---------- */

function maybeSuggestLanguage() {
  const box = $('[data-suggest]');
  if (!box || store.get('mcl-lang') || store.get('mcl-suggest-dismissed')) return;
  if (consent && !consent.hidden) return; // one banner at a time
  const strings = JSON.parse(box.dataset.suggestStrings);
  const preferred = (navigator.languages || [navigator.language])
    .map((l) => String(l).toLowerCase().split('-')[0])
    .find((l) => l in strings);
  if (!preferred || preferred === pageLang) return;
  const s = strings[preferred];
  const text = $('[data-suggest-text]', box);
  const link = $('[data-suggest-link]', box);
  text.textContent = s.text;
  text.lang = preferred;
  link.textContent = s.cta;
  link.lang = preferred;
  link.href = s.url;
  link.hreflang = preferred;
  $('[data-suggest-dismiss-label]', box).textContent = s.dismiss;
  box.hidden = false;
  link.addEventListener('click', () => {
    store.set('mcl-lang', preferred);
    track('language_suggestion_accept', { from: pageLang, to: preferred });
  });
  $('[data-suggest-dismiss]', box).addEventListener('click', () => {
    store.set('mcl-suggest-dismissed', '1');
    box.hidden = true;
  });
}
maybeSuggestLanguage();

/* ---------- Reveal on scroll ---------- */

const revealables = $$('[data-reveal]');
if (reducedMotion || !('IntersectionObserver' in window)) {
  revealables.forEach((el) => el.classList.add('is-visible'));
} else {
  const io = new IntersectionObserver(
    (entries) => entries.forEach((en) => {
      if (!en.isIntersecting) return;
      en.target.classList.add('is-visible');
      io.unobserve(en.target);
    }),
    { rootMargin: '0px 0px -8% 0px', threshold: 0.08 }
  );
  revealables.forEach((el, i) => {
    // Stagger siblings inside the same list for a cascading entrance.
    const idx = [...el.parentElement.children].indexOf(el);
    el.style.transitionDelay = `${Math.min(idx, 6) * 70}ms`;
    io.observe(el);
  });
}

/* ---------- Interactive window-snapping demo ---------- */

const screen = $('[data-demo]');
if (screen) {
  const S = JSON.parse(screen.dataset.strings);
  const desk = $('[data-desk]', screen);
  const win = $('[data-win]', screen);
  const handle = $('[data-handle]', screen);
  const preview = $('[data-preview]', screen);
  const hint = $('[data-hint]', screen);
  const live = $('[data-live]');

  // Rectangles as fractions of the desk: x, y, width, height.
  const RECTS = {
    left: [0, 0, 0.5, 1], right: [0.5, 0, 0.5, 1], max: [0, 0, 1, 1], center: [0.2, 0.14, 0.6, 0.72],
    tl: [0, 0, 0.5, 0.5], tr: [0.5, 0, 0.5, 0.5], bl: [0, 0.5, 0.5, 0.5], br: [0.5, 0.5, 0.5, 0.5],
  };
  const FREE = [0.16, 0.12, 0.46, 0.62];
  const GAP = 6; // px between a snapped window and the screen edge
  let rect = FREE;
  let used = false;

  const apply = (el, [x, y, w, h], gap = 0) => {
    const W = desk.clientWidth, H = desk.clientHeight;
    el.style.left = `${x * W + gap}px`;
    el.style.top = `${y * H + gap}px`;
    el.style.width = `${w * W - gap * 2}px`;
    el.style.height = `${h * H - gap * 2}px`;
  };
  const firstUse = () => {
    if (used) return;
    used = true;
    hint.classList.add('is-gone');
  };
  const pressed = (pos) => $$('[data-place]').forEach((b) => b.setAttribute('aria-pressed', String(b.dataset.place === pos)));

  function place(pos, method) {
    rect = RECTS[pos];
    win.classList.add('is-animating');
    apply(win, rect, GAP);
    win.dataset.pos = pos;
    pressed(pos);
    live.textContent = S.moved.replace('{pos}', S.positions[pos]);
    firstUse();
    track('demo_snap', { position: pos, method });
  }

  // Which zone the pointer is in: corners first, then the edges.
  function zoneAt(px, py) {
    const W = desk.clientWidth, H = desk.clientHeight;
    const edge = Math.max(14, W * 0.035);
    const corner = Math.max(36, Math.min(W, H) * 0.14);
    const nearL = px <= edge, nearR = px >= W - edge, nearT = py <= edge, nearB = py >= H - edge;
    const cx = px <= corner ? 'l' : px >= W - corner ? 'r' : '';
    const cy = py <= corner ? 't' : py >= H - corner ? 'b' : '';
    if ((nearL || nearR || nearT || nearB) && cx && cy) return cy + cx;
    if (nearL) return 'left';
    if (nearR) return 'right';
    if (nearT) return 'max';
    return null;
  }

  let drag = null;
  handle.addEventListener('pointerdown', (e) => {
    if (e.button !== 0) return;
    e.preventDefault();
    handle.setPointerCapture(e.pointerId);
    win.classList.remove('is-animating');
    const box = win.getBoundingClientRect();
    const d = desk.getBoundingClientRect();
    if (win.dataset.pos) {
      // A snapped window gets its old size back as soon as it's dragged, like in the app.
      const w = FREE[2] * d.width, h = FREE[3] * d.height;
      win.style.width = `${w}px`;
      win.style.height = `${h}px`;
      win.style.left = `${e.clientX - d.left - w / 2}px`;
      win.style.top = `${e.clientY - d.top - 12}px`;
      delete win.dataset.pos;
      pressed(null);
      drag = { dx: w / 2, dy: 12, zone: null };
    } else {
      drag = { dx: e.clientX - box.left, dy: e.clientY - box.top, zone: null };
    }
    win.classList.add('is-dragging');
    firstUse();
  });
  handle.addEventListener('pointermove', (e) => {
    if (!drag) return;
    const d = desk.getBoundingClientRect();
    const px = Math.min(Math.max(e.clientX - d.left, 0), d.width);
    const py = Math.min(Math.max(e.clientY - d.top, 0), d.height);
    const w = win.offsetWidth;
    win.style.left = `${Math.min(Math.max(px - drag.dx, -w * 0.6), d.width - w * 0.4)}px`;
    win.style.top = `${Math.min(Math.max(py - drag.dy, 0), d.height - 30)}px`;
    const zone = zoneAt(px, py);
    if (zone !== drag.zone) {
      drag.zone = zone;
      preview.classList.toggle('is-on', !!zone);
      if (zone) apply(preview, RECTS[zone], GAP);
    }
  });
  const end = () => {
    if (!drag) return;
    const zone = drag.zone;
    drag = null;
    win.classList.remove('is-dragging');
    preview.classList.remove('is-on');
    if (zone) place(zone, 'drag');
    else {
      const W = desk.clientWidth, H = desk.clientHeight;
      rect = [win.offsetLeft / W, win.offsetTop / H, win.offsetWidth / W, win.offsetHeight / H];
    }
  };
  handle.addEventListener('pointerup', end);
  handle.addEventListener('pointercancel', end);

  $$('[data-place]').forEach((b) => b.addEventListener('click', () => place(b.dataset.place, 'button')));

  // ⌃⌥← / ⌃⌥→ / ⌃⌥↩ / ⌃⌥C while the demo is on screen, like MacControl's default rules.
  let visible = false;
  new IntersectionObserver(([en]) => { visible = en.intersectionRatio > 0.4; }, { threshold: [0, 0.4, 1] }).observe(screen);
  document.addEventListener('keydown', (e) => {
    if (!visible || !e.ctrlKey || !e.altKey || e.metaKey) return;
    const pos = { ArrowLeft: 'left', ArrowRight: 'right', Enter: 'max', KeyC: 'center' }[e.code];
    if (pos) { e.preventDefault(); place(pos, 'keys'); }
  });

  // Keep the window where it is (as fractions of the desk) when the page is resized.
  if ('ResizeObserver' in window) {
    new ResizeObserver(() => {
      if (drag) return;
      win.classList.remove('is-animating');
      apply(win, rect, win.dataset.pos ? GAP : 0);
    }).observe(desk);
  }
  apply(win, rect);
}

/* ---------- Screenshot lightbox ---------- */

const lightbox = $('[data-lightbox]');
if (lightbox && typeof lightbox.showModal === 'function') {
  const shots = $$('[data-shot]');
  const img = $('[data-lightbox-img]', lightbox);
  const cap = $('[data-lightbox-caption]', lightbox);
  let current = 0;
  let opener = null;

  const show = (i) => {
    current = (i + shots.length) % shots.length;
    const btn = shots[current];
    const caption = btn.closest('figure').querySelector('figcaption').textContent;
    img.src = btn.dataset.full;
    img.alt = caption;
    cap.textContent = caption;
  };

  shots.forEach((btn, i) =>
    btn.addEventListener('click', () => {
      opener = btn;
      show(i);
      lightbox.showModal();
      track('screenshot_open', { screenshot: btn.dataset.full.split('/').pop() });
    })
  );
  $('[data-lightbox-close]', lightbox).addEventListener('click', () => lightbox.close());
  $('[data-lightbox-prev]', lightbox).addEventListener('click', () => show(current - 1));
  $('[data-lightbox-next]', lightbox).addEventListener('click', () => show(current + 1));
  lightbox.addEventListener('click', (e) => { if (e.target === lightbox) lightbox.close(); });
  lightbox.addEventListener('keydown', (e) => {
    if (e.key === 'ArrowLeft') show(current - 1);
    if (e.key === 'ArrowRight') show(current + 1);
  });
  lightbox.addEventListener('close', () => opener?.focus());
}

/* ---------- FAQ engagement ---------- */

$$('.faq__item').forEach((d) =>
  d.addEventListener('toggle', () => {
    if (d.open) track('faq_open', { question: $('h3', d).textContent });
  })
);
