#!/usr/bin/env bash
# Builds MacControl.app (and optionally MacControl.dmg) into dist/.
#   ./scripts/package-macos.sh            # .app + .dmg
#   ./scripts/package-macos.sh --no-dmg   # .app only (used by install-local.sh)
# SIGN_IDENTITY=<name> signs with that certificate instead of ad-hoc ("-").
set -euo pipefail

MAKE_DMG=1
[ "${1:-}" = "--no-dmg" ] && MAKE_DMG=0
SIGN_IDENTITY="${SIGN_IDENTITY:--}"

APP_NAME="MacControl"
BIN_NAME="maccontrol"
EXE_NAME="MacControl"
BUNDLE_ID="com.joanet.maccontrol"

cd "$(dirname "$0")/.."
ROOT="$PWD"
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
DIST="$ROOT/dist"
APP="$DIST/$APP_NAME.app"
DMG="$DIST/$APP_NAME-$VERSION.dmg"

echo "==> cargo build --release"
cargo build --release
BIN="$ROOT/target/release/$BIN_NAME"

echo "==> Building $APP_NAME.app"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/$EXE_NAME"

# Icon: scale assets/icon.png (rendered from assets/icon.svg by render-icons.sh) into an .icns.
ICONSET="$(mktemp -d)/$APP_NAME.iconset"
mkdir -p "$ICONSET"
for s in 16 32 128 256 512; do
  sips -z "$s" "$s" assets/icon.png --out "$ICONSET/icon_${s}x${s}.png" >/dev/null
  sips -z "$((s * 2))" "$((s * 2))" assets/icon.png --out "$ICONSET/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/$APP_NAME.icns"

cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>               <string>$APP_NAME</string>
  <key>CFBundleDisplayName</key>        <string>$APP_NAME</string>
  <key>CFBundleIdentifier</key>         <string>$BUNDLE_ID</string>
  <key>CFBundleExecutable</key>         <string>$EXE_NAME</string>
  <key>CFBundleIconFile</key>           <string>$APP_NAME</string>
  <key>CFBundlePackageType</key>        <string>APPL</string>
  <key>CFBundleVersion</key>            <string>$VERSION</string>
  <key>CFBundleShortVersionString</key> <string>$VERSION</string>
  <key>LSMinimumSystemVersion</key>     <string>12.0</string>
  <key>NSHighResolutionCapable</key>    <true/>
  <key>LSApplicationCategoryType</key>  <string>public.app-category.utilities</string>
</dict>
</plist>
EOF

# Ad-hoc by default: free, no Apple account. An ad-hoc identity changes with
# every build, so macOS may drop the Accessibility permission after reinstalling;
# a fixed SIGN_IDENTITY (see setup-signing.sh) avoids that.
echo "==> Signing ($SIGN_IDENTITY)"
codesign --force --deep --sign "$SIGN_IDENTITY" --identifier "$BUNDLE_ID" "$APP"

if [ "$MAKE_DMG" = 0 ]; then
  rm -rf "$(dirname "$ICONSET")"
  echo "Done: $APP"
  exit 0
fi

echo "==> Building $(basename "$DMG")"
STAGE="$(mktemp -d)"
cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"
rm -f "$DMG"
hdiutil create -volname "$APP_NAME" -srcfolder "$STAGE" -ov -format UDZO "$DMG" >/dev/null
rm -rf "$STAGE" "$(dirname "$ICONSET")"

echo
echo "Done:"
echo "  $APP"
echo "  $DMG"
