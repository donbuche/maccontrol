#!/usr/bin/env bash
# Builds MacControl.app, replaces the copy in /Applications and launches it.
# For day-to-day testing; no .dmg involved.
#   ./scripts/install-local.sh
set -euo pipefail

APP_NAME="MacControl"
EXE_NAME="MacControl"
DEST="/Applications/$APP_NAME.app"

cd "$(dirname "$0")/.."

# Sign with the fixed certificate from setup-signing.sh when it exists, so the
# Accessibility permission survives reinstalls.
CERT="MacControl Local Signing"
if [ -z "${SIGN_IDENTITY:-}" ] && security find-certificate -c "$CERT" >/dev/null 2>&1; then
  export SIGN_IDENTITY="$CERT"
fi
if [ -z "${SIGN_IDENTITY:-}" ]; then
  echo "Note: no signing certificate; run ./scripts/setup-signing.sh once so"
  echo "      macOS stops asking for Accessibility after every install."
fi
./scripts/package-macos.sh --no-dmg

echo "==> Quitting the running $APP_NAME"
if pgrep -x "$EXE_NAME" >/dev/null; then
  pkill -x "$EXE_NAME" || true
  for _ in $(seq 1 50); do pgrep -x "$EXE_NAME" >/dev/null || break; sleep 0.1; done
  pkill -9 -x "$EXE_NAME" 2>/dev/null || true
fi

echo "==> Installing to $DEST"
rm -rf "$DEST"
ditto "dist/$APP_NAME.app" "$DEST"

echo "==> Launching"
open "$DEST"
for _ in $(seq 1 50); do pgrep -x "$EXE_NAME" >/dev/null && break; sleep 0.1; done
if pgrep -x "$EXE_NAME" >/dev/null; then
  echo "Running: $DEST (pid $(pgrep -x "$EXE_NAME"))"
else
  echo "Error: $APP_NAME did not start" >&2
  exit 1
fi
