#!/usr/bin/env bash
# Founder/community unsigned release build helper (Hari 55).
# Does not sign or notarize. Writes checksums next to the binary.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="${HOME}/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:${PATH}"

OUT_DIR="${1:-dist/founder-unsigned}"
mkdir -p "$OUT_DIR"

cargo build -p acquire-slint --features gui --release --locked

BIN="target/release/trareon-acquire"
EXT=""
case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*|Windows_NT) BIN="target/release/trareon-acquire.exe"; EXT=".exe" ;;
esac

cp "$BIN" "$OUT_DIR/trareon-acquire${EXT}"
COMMIT="$(git rev-parse HEAD)"
{
  echo "Trareon Acquire — Founder/Community unsigned build"
  echo "commit=${COMMIT}"
  echo "license=GPL-3.0-only"
  echo "corresponding_source=https://github.com/Trareon-com/Trareon-Acquire/tree/${COMMIT}"
  echo "class=Unsigned / Not validated for production evidence acquisition"
} >"$OUT_DIR/NOTICE.txt"

if command -v shasum >/dev/null 2>&1; then
  (cd "$OUT_DIR" && shasum -a 256 "trareon-acquire${EXT}" > SHA256SUMS.txt)
elif command -v sha256sum >/dev/null 2>&1; then
  (cd "$OUT_DIR" && sha256sum "trareon-acquire${EXT}" > SHA256SUMS.txt)
fi

echo "FOUNDER_BUILD_OK dir=${OUT_DIR} commit=${COMMIT}"
