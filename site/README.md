# Landing page

Static, multilingual landing page for MacControl, published with GitHub Pages at <https://donbuche.github.io/maccontrol/>.

- English is the default language and lives at the root. Spanish and Catalan live at `/es/` and `/ca/`.
- Every language is a full static HTML page (no client-side translation), with its own title, description, canonical URL, `hreflang` alternates, Open Graph image and JSON-LD (`SoftwareApplication`, `WebPage`, `FAQPage`).
- No dependencies and no framework: a Node script renders `src/template.mjs` with the strings in `i18n/`. The version shown on the page comes from `Cargo.toml`.

## Commands

Run them from the repository root. You need Node 20 or later.

| Command | Description |
|---|---|
| `node site/build.mjs --serve` | Build and serve at <http://localhost:4321/maccontrol/> |
| `node site/build.mjs` | Build into `site/_site/` (ignored by git) |
| `node site/scripts/og.mjs` | Regenerate the Open Graph images (`assets/img/og/og-<lang>.jpg`) with headless Google Chrome |

## Configuration

Everything that changes between environments is in [`config.mjs`](config.mjs): the public URL, the Google Analytics ID, the repository and download links, and the list of languages.

### Google Analytics

Set `gaMeasurementId` in `config.mjs`, or define a `GA_MEASUREMENT_ID` variable in **Settings → Secrets and variables → Actions → Variables**, which takes precedence in the deploy workflow.

The page uses Google Consent Mode v2 with every storage type denied until the visitor accepts the cookie banner. With `consentMode: 'advanced'` gtag loads straight away and sends cookieless pings; with `'basic'` it isn't loaded until the visitor accepts. The choice is stored in `localStorage` (`mcl-consent`) and can be changed from the footer link.

Custom events: `download_click`, `github_click`, `language_switch`, `language_suggestion_accept`, `demo_snap` (with `position` and `method`: drag, button or keys), `screenshot_open` and `faq_open`. Each one carries `page_language`.

### Custom domain

Change `siteUrl` in `config.mjs` and add a `static/CNAME` file containing the domain. Files in `static/` are copied to the site root as they are.

## Content

- Strings: `i18n/<lang>.mjs`. All files have the same keys.
- Markup: `src/template.mjs`. Styles: `src/styles.css`. Behaviour: `src/main.js` (consent, menus, language suggestion, reveal on scroll, the window-snapping demo and the screenshot lightbox). Icons: `src/icons.mjs`.
- Screenshots: one set per language, at 1600 and 800 px wide, in `assets/img/screens/<lang>/` (the English set is also in `docs/screenshots/` for the main README). To refresh them, run the app with `MACCONTROL_LANG=<lang> maccontrol --screenshot <file.png> --page <name> --scale 1.8` (see the main README), convert with `cwebp -q 85 -resize 1600 0` (and `800 0`), and run `og.mjs` again.
- Fonts: Bricolage Grotesque, Figtree and JetBrains Mono, self-hosted (SIL Open Font License).

To add a language, copy `i18n/en.mjs`, translate it, add a flag in `assets/flags/`, add the code to `languages` in `config.mjs` and run `node site/scripts/og.mjs`.

## Deployment

[`.github/workflows/pages.yml`](../.github/workflows/pages.yml) builds and deploys the site on every push to `main` that touches `site/` or `Cargo.toml` (the version number appears on the page). It can also be run by hand from the Actions tab. In **Settings → Pages**, the source must be **GitHub Actions** (no `gh-pages` branch).

## SEO notes

- The page never redirects by language. It shows a small banner suggesting the visitor's browser language instead, so crawlers always see every version.
- On a project site (`user.github.io/repo`), crawlers only read `robots.txt` from the host root, so submit `sitemap.xml` in Google Search Console. A custom domain removes that limitation.
