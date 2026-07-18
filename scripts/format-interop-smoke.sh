#!/usr/bin/env bash
# Synthetic E01-lite smoke only; external-tool verification remains human-gated.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

OUT_DIR="${1:-dist/format-interop-smoke}"
mkdir -p "$OUT_DIR"
ARTIFACT="$OUT_DIR/trareon-e01-lite-smoke.E01"

cargo run -q -p trareon-core --example format_e01_smoke -- "$ARTIFACT" | tee "$OUT_DIR/smoke.log"
if command -v shasum >/dev/null 2>&1; then
  shasum -a 256 "$ARTIFACT" > "$OUT_DIR/SHA256.txt"
else
  sha256sum "$ARTIFACT" > "$OUT_DIR/SHA256.txt"
fi
printf 'FORMAT_INTEROP_SMOKE_OK artifact=%s\n' "$ARTIFACT"
