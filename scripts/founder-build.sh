#!/usr/bin/env bash
# Founder unsigned offline build helper.
# Full app binary for private sale (Lynk.id / Gumroad). Not for GitHub Releases.
# Does not sign or notarize. Does not embed a license key.
# Output under dist/ (gitignored) — keep on the operator machine only.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="${HOME}/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:${PATH}"

OUT_DIR="${1:-dist/founder-unsigned}"
mkdir -p "$OUT_DIR"

cargo build -p acquire-slint --features gui --release --locked

TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
BIN="$TARGET_DIR/release/trareon-acquire"
EXT=""
case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*|Windows_NT) BIN="$TARGET_DIR/release/trareon-acquire.exe"; EXT=".exe" ;;
esac

if [[ ! -f "$BIN" ]]; then
  echo "founder-build: missing binary at $BIN" >&2
  exit 1
fi

cp "$BIN" "$OUT_DIR/trareon-acquire${EXT}"
COMMIT="$(git rev-parse HEAD)"
{
  echo "Trareon Acquire — Founder unsigned full binary (offline fulfillment)"
  echo "commit=${COMMIT}"
  echo "license=GPL-3.0-only"
  echo "corresponding_source=https://github.com/Trareon-com/Trareon-Acquire/tree/${COMMIT}"
  echo "distribution=private Lynk.id/Gumroad delivery — not a GitHub Release artifact"
  echo "activation=none — no license key required"
  echo "class=Unsigned / Not validated for production evidence acquisition"
} >"$OUT_DIR/NOTICE.txt"

if command -v shasum >/dev/null 2>&1; then
  (cd "$OUT_DIR" && shasum -a 256 "trareon-acquire${EXT}" > SHA256SUMS.txt)
elif command -v sha256sum >/dev/null 2>&1; then
  (cd "$OUT_DIR" && sha256sum "trareon-acquire${EXT}" > SHA256SUMS.txt)
fi

echo "FOUNDER_BUILD_OK dir=${OUT_DIR} commit=${COMMIT}"
