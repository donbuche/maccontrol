// Inline SVG icons (24 × 24, stroke based). Drawn for this site.
const stroke = (body) =>
  `<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">${body}</svg>`;

export const icons = {
  trackpad: stroke('<rect x="3" y="5" width="18" height="14" rx="3"/><circle cx="9" cy="11" r="1.2" fill="currentColor"/><circle cx="12" cy="11" r="1.2" fill="currentColor"/><circle cx="15" cy="11" r="1.2" fill="currentColor"/><path d="M9 15h6"/>'),
  keyboard: stroke('<rect x="2.5" y="6" width="19" height="12" rx="2.5"/><path d="M6 10h.01M9.5 10h.01M13 10h.01M16.5 10h.01M6 13.5h.01M18 13.5h.01M9 14.5h6"/>'),
  mouse: stroke('<rect x="6.5" y="3" width="11" height="18" rx="5.5"/><path d="M12 7v3"/>'),
  window: stroke('<rect x="3" y="4" width="18" height="16" rx="2.5"/><path d="M3 8.5h18"/><rect x="5.5" y="11" width="6" height="6.5" rx="1" fill="currentColor" stroke="none"/>'),
  apps: stroke('<rect x="3.5" y="3.5" width="7" height="7" rx="2"/><rect x="13.5" y="3.5" width="7" height="7" rx="2"/><rect x="3.5" y="13.5" width="7" height="7" rx="2"/><path d="M17 13.5v7M13.5 17h7"/>'),
  menubar: stroke('<rect x="3" y="4" width="18" height="16" rx="2.5"/><path d="M3 8h18M16 6h2.5"/>'),
  shield: stroke('<path d="M12 3 4.5 6v5.5c0 4.6 3.2 8.4 7.5 9.5 4.3-1.1 7.5-4.9 7.5-9.5V6z"/><path d="M9 12l2.2 2.2L15.5 10"/>'),
  feather: stroke('<path d="M13 2.5 11 9l5.5-2.5L13 13l5-1.5L12 21.5"/><path d="M7 12.5l5 9"/>'),
  wand: stroke('<path d="M4 20 15 9"/><path d="M15 3v2.5M19.5 4.5 18 6M21 9h-2.5M19.5 13.5 18 12M10.5 4.5 12 6"/>'),
  pause: stroke('<rect x="6" y="5" width="4" height="14" rx="1"/><rect x="14" y="5" width="4" height="14" rx="1"/>'),
  reset: stroke('<path d="M3.5 12a8.5 8.5 0 1 0 2.6-6.1L3.5 8.5"/><path d="M3.5 3.5v5h5"/>'),
  download: stroke('<path d="M12 3v12M7 10l5 5 5-5M4 20h16"/>'),
  close: stroke('<path d="M6 6l12 12M18 6 6 18"/>'),
  prev: stroke('<path d="M15 5l-7 7 7 7"/>'),
  next: stroke('<path d="M9 5l7 7-7 7"/>'),
  chevron: stroke('<path d="M6 9l6 6 6-6"/>'),
  menu: stroke('<path d="M4 7h16M4 12h16M4 17h16"/>'),
  check: stroke('<path d="M5 12.5l4.5 4.5L19 7.5"/>'),
  zoom: stroke('<circle cx="11" cy="11" r="7"/><path d="M20 20l-4-4M11 8v6M8 11h6"/>'),
  github: '<svg class="icon" viewBox="0 0 24 24" aria-hidden="true" focusable="false"><path fill="currentColor" d="M12 .5a11.5 11.5 0 0 0-3.64 22.41c.58.1.79-.25.79-.56v-2c-3.2.7-3.88-1.37-3.88-1.37-.52-1.33-1.28-1.69-1.28-1.69-1.05-.71.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.7 1.26 3.36.96.1-.75.4-1.26.73-1.55-2.55-.29-5.24-1.28-5.24-5.69 0-1.26.45-2.29 1.19-3.09-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.17 1.18a11 11 0 0 1 5.77 0c2.2-1.49 3.17-1.18 3.17-1.18.63 1.59.23 2.76.11 3.05.74.8 1.19 1.83 1.19 3.09 0 4.42-2.7 5.4-5.26 5.68.41.36.78 1.06.78 2.14v3.17c0 .31.21.67.8.56A11.5 11.5 0 0 0 12 .5z"/></svg>',
  laptop: stroke('<rect x="4" y="4" width="16" height="11" rx="1.5"/><path d="M2 19h20"/>'),
};
