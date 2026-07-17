# Linux Test Validation Report

**Date:** 2026-07-17
**Host:** Kali Linux, x86_64
**Rust:** 1.96.0 (stable)
**Toolchain required:** 1.95.0 (pinned in rust-toolchain.toml) — 1.96 is forward-compatible
**Environment:** `/tmp` mounted on tmpfs (12 GB), `/dev/nvme0n1p1` root (453 GB, 414 GB used)
**Slint GUI deps:** cmake 3.31.6 (portable), pkg-config, libfontconfig, libfreetype, libxkbcommon, libwayland, libxcb-*, libgl1-mesa

---

## Trearon Acquire — Full Test Results

| # | Test | Command | Result | Notes |
|---|------|---------|--------|-------|
| 1 | **Format check** | `cargo fmt --all --check` | ✅ PASS | — |
| 2 | **Clippy workspace** | `cargo clippy --workspace --all-targets --all-features --exclude acquire-slint -- -D warnings` | ✅ PASS | — |
| 3 | **Clippy GUI** | `cargo clippy -p acquire-slint --all-targets --features gui -- -D warnings` | ✅ PASS | — |
| 4 | **Workspace tests** | `cargo test --workspace --locked --exclude acquire-slint` | ✅ PASS | 7 verifier tests, 1 generate fixtures (ignored) |
| 5 | **GUI tests (no-features)** | `cargo test -p acquire-slint --no-default-features --tests --locked` | ✅ PASS | — |
| 6 | **GUI tests (gui features)** | `cargo test -p acquire-slint --features gui --locked` | ✅ PASS | — |
| 7 | **Property tests** | `cargo test -p trareon-core --test properties --locked` | ✅ PASS | 9 property tests: state transition matrix, audit chain, hostile inputs, roundtrip |
| 8 | **Performance baseline** | `cargo test -p trareon-core --test performance --locked` | ✅ PASS | 3/4 tests passed; 1 ignored (writes docs); buffer bounded, cancel latency, hash stability all OK |
| 9 | **Golden fixture verify** | `cargo run -p trareon-verifier --locked -- verify fixtures/fsnap-v0.1/valid` | ✅ PASS | VALID `a933f0d5caf28c5a050e04ba707ba3920438ec226c6191c0a9b921bbc8f1889a` |
| 10 | **Mutated fixture reject** | `cargo run -p trareon-verifier --locked -- verify fixtures/fsnap-v0.1/mutated` | ✅ PASS | INVALID (evidence size mismatch), exit code 2 |
| 11 | **Truncated fixture reject** | `cargo run -p trareon-verifier --locked -- verify fixtures/fsnap-v0.1/truncated` | ✅ PASS | INVALID (evidence size mismatch), exit code 2 |
| 12 | **Removed file reject** | `cargo run -p trareon-verifier --locked -- verify fixtures/fsnap-v0.1/removed-file` | ✅ PASS | INVALID (I/O error), exit code 2 |
| 13 | **Foundation demo** | `cargo test -p acquire-slint --features gui foundation_demo -- --nocapture` | ✅ PASS | Reports SHA and size on 4 MB random source |
| 14 | **GUI build** | `cargo build -p acquire-slint --features gui --locked` | ✅ PASS | — |
| 15 | **Secret pattern scan** | grep for private keys, AWS keys | ✅ PASS | No secrets committed |
| 16 | **Self-test aggregate** | `scripts/self-test.sh` (fmt+clippy+tests+golden) | ✅ PASS | All sub-steps passed |

**Overall: 16/16 ✅ PASS**

---

## Trearon Lab — Full Test Results

| # | Test | Command | Result | Notes |
|---|------|---------|--------|-------|
| 1 | **Format check** | `cargo fmt --all --check` | ✅ PASS | — |
| 2 | **Clippy workspace** | `cargo clippy --workspace --all-targets --exclude lab-slint -- -D warnings` | ✅ PASS | — |
| 3 | **Clippy GUI** | `cargo clippy -p lab-slint --all-targets --features gui -- -D warnings` | ✅ PASS | — |
| 4 | **Workspace tests** | `cargo test --workspace --exclude lab-slint` | ✅ PASS | All crates pass |
| 5 | **GUI tests (no-features)** | `cargo test -p lab-slint --no-default-features --tests` | ✅ PASS | 3 UI model tests + 1 validation hooks test |
| 6 | **GUI tests (gui features)** | `cargo test -p lab-slint --features gui` | ✅ PASS | Same 4 tests + docs shell |
| 7 | **Artifact parsers** | `cargo test -p lab-artifacts --tests` | ✅ PASS | Prefetch, LNK, JumpList, macOS Unified Log, Linux auth/syslog parsers all OK |
| 8 | **Transfer sign/verify** | `cargo test -p lab-transfer --tests` | ✅ PASS | Export-import OK + tamper rejected |
| 9 | **Timeline merge** | `cargo test -p lab-timeline --lib` | ✅ PASS | Merge sort test |
| 10 | **Packaging smoke** | `./packaging/smoke.sh` | ✅ PASS | Smoke artifact generated |
| 11 | **E2E core path** | `./scripts/e2e-smoke.sh` (incl. packaging smoke) | ✅ PASS | workspace tests + artifacts + transfer + timeline + packaging all OK |
| 12 | **GUI build** | `cargo build -p lab-slint --features gui --locked` | ✅ PASS | Slint + rusqlite + all crates compile |
| 13 | **Secret pattern scan** | grep for private keys, AWS keys | ✅ PASS | No secrets committed |

**Overall: 13/13 ✅ PASS**

---

## Environment Notes

1. **Disk space:** `/tmp` tmpfs has 12 GB capacity. Rust incremental builds + debug symbols can consume ~6 GB per repo. Parallel builds of both repos can exhaust space. Sequential builds or `rm -rf target/debug/incremental` between builds resolves this.

2. **Missing deps on stock Kali:** `cmake` is not pre-installed. Installed via portable binary from GitHub releases (cmake-3.31.6-linux-x86_64). Slint GUI requires: pkg-config, libfontconfig1-dev, libfreetype6-dev, libx11-dev, libxcursor-dev, libxkbcommon-dev, libwayland-dev, libgl1-mesa-dev.

3. **No sudo available:** All Slint system deps were pre-installed on this Kali system. cmake was installed as a portable binary in `/tmp/cmake-3.31.6-linux-x86_64/bin/`.

4. **Rust version:** System has rustc 1.96.0. The project pins 1.95.0 in `rust-toolchain.toml`. All tests pass on 1.96.0 with no compatibility issues.

5. **Headless GUI tests:** Tests pass without a display server (headless CI). The foundation_demo test exercises the full acquire pipeline without needing a window.

---

## Signed-off

**Validation performed by:** Tohka Yatogami (Hermes Agent)
**Test environment:** `Linux kali 6.18.9-kali-amd64`
**Timestamp:** 2026-07-17T22:45:00Z
