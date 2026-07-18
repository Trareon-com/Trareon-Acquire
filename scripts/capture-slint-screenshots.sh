#!/usr/bin/env bash
# Capture / refresh docs/media/screenshots/01–09 from a live acquire-slint build.
# Headless automation is platform-limited; this script documents the operator pass.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/docs/media/screenshots"
mkdir -p "$OUT"

echo "Launch the GUI, then capture each surface to:"
echo "  $OUT/01-acquire-bench.png"
echo "  $OUT/02-cases.png"
echo "  $OUT/03-identify.png"
echo "  $OUT/04-telemetry-coverage.png"
echo "  $OUT/05-seal-coc.png"
echo "  $OUT/06-tools.png"
echo "  $OUT/07-triage-analysis.png"
echo "  $OUT/08-qms-boot.png"
echo "  $OUT/09-help.png"
echo
echo "Run:"
echo "  cargo run -p acquire-slint --features gui"
echo
echo "Prefer light theme, Standard mode, 1280×860. Do not use plan-ecr/ mocks as product chrome."
echo "Mark CI stale if mtime of 01–09 is older than apps/acquire-slint/ui/app.slint."

if [[ "${CHECK_STALE:-}" == "1" ]]; then
  ui="$ROOT/apps/acquire-slint/ui/app.slint"
  stale=0
  for n in 01 02 03 04 05 06 07 08 09; do
    f=$(ls "$OUT"/${n}-*.png 2>/dev/null | head -1 || true)
    if [[ -z "${f:-}" || "$ui" -nt "$f" ]]; then
      echo "STALE: ${n}-*.png"
      stale=1
    fi
  done
  exit "$stale"
fi
