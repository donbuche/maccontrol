#!/usr/bin/env node
// Renders the 1200 × 630 Open Graph image for each language with headless Chrome.
// Run after changing the OG titles or the screenshots:  node site/scripts/og.mjs
// CHROME=/path/to/chrome overrides the browser.

import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const site = join(dirname(fileURLToPath(import.meta.url)), '..');
const chrome = process.env.CHROME || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const a = (p) => pathToFileURL(join(site, 'assets', p)).href;
const cfg = (await import(pathToFileURL(join(site, 'config.mjs')).href)).default;

const html = (t) => `<!doctype html><html lang="${t.lang}"><head><meta charset="utf-8"><style>
@font-face { font-family: D; font-weight: 800; src: url(${a('fonts/bricolage-grotesque-latin-800-normal.woff2')}); }
@font-face { font-family: B; font-weight: 600; src: url(${a('fonts/figtree-latin-600-normal.woff2')}); }
html, body { margin: 0; width: 1200px; height: 630px; overflow: hidden; }
body { position: relative; color: #f3f2fb; font-family: B; background: #0b0a1a; }
.blob { position: absolute; border-radius: 50%; filter: blur(90px); }
.b1 { width: 620px; height: 620px; left: -200px; top: -260px; background: #8b5cf6; opacity: .6; }
.b2 { width: 520px; height: 520px; right: -120px; top: -80px; background: #22d3ee; opacity: .45; }
.b3 { width: 520px; height: 520px; left: 300px; bottom: -380px; background: #ec4899; opacity: .4; }
.copy { position: absolute; left: 70px; top: 70px; width: 560px; }
.brand { display: flex; align-items: center; gap: 18px; font: 800 44px/1 D; letter-spacing: -.02em; }
.brand img { width: 96px; height: 96px; }
.brand span { color: #22d3ee; }
h1 { margin: 44px 0 0; font: 800 76px/.95 D; letter-spacing: -.04em; }
h1 em { font-style: normal; background: linear-gradient(100deg, #22d3ee, #7aa8ff 45%, #8b5cf6 70%, #ec4899); -webkit-background-clip: text; color: transparent; }
.tag { display: inline-block; margin-top: 38px; padding: 14px 24px; border-radius: 999px; font: 600 26px/1 B;
  background: rgba(255,255,255,.08); border: 1px solid rgba(255,255,255,.2); }
.shot { position: absolute; right: -170px; top: 110px; width: 760px; padding: 10px; border-radius: 22px;
  background: linear-gradient(160deg, #2b2945, #12111f); border: 1px solid rgba(255,255,255,.18);
  transform: perspective(1400px) rotateY(-14deg) rotateZ(1.5deg); box-shadow: 0 50px 90px -20px rgba(0,0,0,.85); }
.shot img { display: block; width: 100%; border-radius: 13px; }
</style></head><body>
<div class="blob b1"></div><div class="blob b2"></div><div class="blob b3"></div>
<div class="copy">
  <div class="brand"><img src="${a('icons/icon-512.png')}" alt=""><div>Mac<span>Control</span></div></div>
  <h1>${t.hero.title.replace(/&nbsp;/g, ' ')}</h1>
  <div class="tag">${t.hero.eyebrow.split(' · ')[0]}</div>
</div>
<div class="shot"><img src="${a(`img/screens/${t.lang}/home.webp`)}" alt=""></div>
</body></html>`;

const outDir = join(site, 'assets', 'img', 'og');
mkdirSync(outDir, { recursive: true });
const tmp = mkdtempSync(join(tmpdir(), 'og-'));
for (const lang of cfg.languages) {
  const t = (await import(pathToFileURL(join(site, 'i18n', `${lang}.mjs`)).href)).default;
  const page = join(tmp, `${lang}.html`);
  const png = join(tmp, `${lang}.png`);
  writeFileSync(page, html(t));
  execFileSync(chrome, ['--headless=new', '--disable-gpu', '--hide-scrollbars', '--force-device-scale-factor=1',
    '--allow-file-access-from-files', '--window-size=1200,630', '--virtual-time-budget=2000',
    `--screenshot=${png}`, pathToFileURL(page).href], { stdio: 'ignore' });
  execFileSync('sips', ['-s', 'format', 'jpeg', '-s', 'formatOptions', '86', png, '--out', join(outDir, `og-${lang}.jpg`)], { stdio: 'ignore' });
  console.log(`og-${lang}.jpg`);
}
rmSync(tmp, { recursive: true, force: true });
