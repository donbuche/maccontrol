#!/usr/bin/env bash
# Renders the icon sources into the PNGs the app embeds, with a transparent background:
#   assets/icon.svg -> assets/icon.png (1024 px; package-macos.sh makes the .icns from it)
#   assets/logo.svg -> assets/logo.png (1024 px; the logo inside the app's window)
#   assets/tray.svg -> assets/tray.png (36 px menu-bar template image)
# Needs Google Chrome (used headless as the SVG renderer).
#   ./scripts/render-icons.sh
set -euo pipefail
cd "$(dirname "$0")/.."

CHROME="${CHROME:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
[ -x "$CHROME" ] || { echo "Chrome not found; set CHROME=/path/to/chrome" >&2; exit 1; }

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

render() { # svg, png, size
  local svg="$1" png="$2" size="$3"
  cat > "$TMP/page.html" <<EOF
<!doctype html><html><head><style>html,body{margin:0;background:transparent}img{display:block;width:${size}px;height:${size}px}</style></head>
<body><img src="file://$PWD/$svg"></body></html>
EOF
  "$CHROME" --headless=new --disable-gpu --hide-scrollbars --force-device-scale-factor=1 \
    --default-background-color=00000000 --window-size="$size,$size" \
    --allow-file-access-from-files --screenshot="$PWD/$png" "file://$TMP/page.html" >/dev/null 2>&1
  echo "==> $png"
}

render assets/icon.svg assets/icon.png 1024
render assets/logo.svg assets/logo.png 1024
render assets/tray.svg assets/tray.png 36
