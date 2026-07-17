#!/usr/bin/env bash
# One-command self-test (Hari 38) — no evidence bytes, no elevation.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="${HOME}/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:${PATH}"

echo "== fmt =="
cargo fmt --all --check

echo "== clippy (non-slint) =="
cargo clippy --workspace --all-targets --all-features --exclude acquire-slint -- -D warnings

echo "== clippy (slint) =="
cargo clippy -p acquire-slint --all-targets --features gui -- -D warnings

echo "== tests =="
cargo test --workspace --locked --exclude acquire-slint
cargo test -p acquire-slint --features gui --locked
cargo test -p trareon-core --test properties --locked
cargo test -p trareon-core --test performance --locked

echo "== golden verifier =="
cargo run -q -p trareon-verifier --locked -- verify fixtures/fsnap-v0.1/valid

echo "== ai-ops docs =="
sh scripts/validate-ai-operations.sh

echo "SELF_TEST_OK"
