# Trareon Acquire

**Production-Directed Engineering Alpha Candidate — Lab Use Only.**

Trareon Acquire is a digital forensic acquisition tool under active
development. Milestone M0 is classified (Day 30) as a **file-backed
foundation slice**: streaming acquisition of a synthetic/file source,
SHA-256 hashing, an append-only hash-chained audit journal, `.fsnap`
packaging (Analysis-frozen single-`evidence.raw` layout), and an
independent CLI verifier.

It is **not** validated for Official Production evidence on real devices.
Capability without exact hardware evidence remains `NotValidated`.
Milestone review: [`docs/M0-MILESTONE-REVIEW.md`](docs/M0-MILESTONE-REVIEW.md).

## What works today (M0 foundation)

| Area | Status |
|---|---|
| Portable Rust core (`trareon-core`) | Implemented — file-backed acquire, audit chain, `.fsnap` create/verify |
| Independent verifier CLI (`trareon-verifier`) | Implemented — golden fixtures + fail-closed checks |
| Desktop shell (Tauri + Svelte) | Implemented — guided foundation demo UI |
| Cross-platform CI (Ubuntu / Windows / macOS) | Hosted CI green for the foundation slice |
| DevSecOps gates (`cargo deny`, npm audit, secret scan) | Implemented on `main` |
| Bounded property tests / fuzz corpus docs | Implemented on `main` (Day 22); full `cargo-fuzz` still deferred |
| Performance baseline (synthetic) | Implemented on `main` (Day 26); peak RSS `NotValidated` |
| Capability / limitation matrix | See [`docs/CAPABILITY-MATRIX-M0.md`](docs/CAPABILITY-MATRIX-M0.md) |
| `.fsnap` v0.1 Analysis freeze | Frozen for Analysis (single-`evidence.raw`); see read contract |
| Linux raw-device privilege probe | Feasibility spike (Day 23 / Kali); physical disk still `NotValidated` |
| macOS raw-device privilege probe | Feasibility spike (Day 25 / M4 Pro); open `/dev/rdisk0` denied without elevation |
| Raw-device adapters (Windows) | **Not started** — needs lab hardware (`MANUAL_START`) |
| Production release / signing / certification | **Out of scope** — Lab Use Only EAC only |

Authoritative status tracking: [`docs/ai-operations/MASTER-CHECKLIST.md`](docs/ai-operations/MASTER-CHECKLIST.md).

## Repository layout

```
crates/trareon-core/          # Domain, acquisition, audit, .fsnap package API
crates/trareon-verifier/      # Independent CLI: `trareon-verifier verify PATH`
apps/trareon-acquire/          # Tauri 2 + Svelte foundation demo
fixtures/fsnap-v0.1/          # Synthetic golden packages (no real evidence)
schemas/                      # Manifest JSON Schema
docs/                         # Roadmap, demo guide, contracts, AI ops
```

Core and verifier do **not** depend on Tauri. The UI depends on core; core
never depends on the UI.

## Requirements

- Rust toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml) (`1.95.0`)
- Node.js / npm versions pinned in root [`package.json`](package.json) (`node` 22.x)
- Linux CI/dev builds need the usual Tauri system libraries (see `.github/workflows/ci.yml`)

## Quick start

### Build and test

```bash
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
npm ci --prefix apps/trareon-acquire
npm run build --prefix apps/trareon-acquire
```

### Foundation demo (synthetic source)

See the full walkthrough in [`docs/FOUNDATION-DEMO.md`](docs/FOUNDATION-DEMO.md).

```bash
dd if=/dev/zero of=/tmp/trareon-source.img bs=1M count=8
npm ci --prefix apps/trareon-acquire
npm run tauri --prefix apps/trareon-acquire -- dev
# In the UI: source=/tmp/trareon-source.img, output=/tmp/trareon-output,
# confirm synthetic checkbox, Run.
cargo run -p trareon-verifier --locked -- verify /tmp/trareon-output/foundation.fsnap
```

The UI reports success only after the Rust core acquires, packages, and the
independent verifier accepts the package — the frontend does not invent a
completion state.

### Verify a golden fixture

```bash
cargo run -p trareon-verifier --locked -- verify fixtures/fsnap-v0.1/valid
```

## Documentation

| Doc | Purpose |
|---|---|
| [`docs/FOUNDATION-DEMO.md`](docs/FOUNDATION-DEMO.md) | End-to-end synthetic demo + tamper demo |
| [`docs/IMPLEMENTATION-ROADMAP.md`](docs/IMPLEMENTATION-ROADMAP.md) | Tracks, result classes, exit criteria |
| [`docs/fsnap-v0.1-read-contract.md`](docs/fsnap-v0.1-read-contract.md) | `.fsnap` v0.1 reader/verifier contract (Analysis-frozen) |
| [`docs/CAPABILITY-MATRIX-M0.md`](docs/CAPABILITY-MATRIX-M0.md) | Capability / limitation matrix |
| [`docs/M0-MILESTONE-REVIEW.md`](docs/M0-MILESTONE-REVIEW.md) | Day 30 EAC classification |
| [`docs/USER-GUIDE.md`](docs/USER-GUIDE.md) | Operator user guide |
| [`docs/LEGAL-LIMITATIONS-DRAFT.md`](docs/LEGAL-LIMITATIONS-DRAFT.md) | Legal limitations draft (`LEGAL_DRAFT_ONLY`) |
| [`docs/WEEK-01-DISCREPANCY-REGISTER.md`](docs/WEEK-01-DISCREPANCY-REGISTER.md) | Known gaps vs Day-by-day runbook |
| [`SECURITY.md`](SECURITY.md) | Vulnerability reporting + automated checks |
| [`RFC-Digital-Forensic-Acquisition.md`](RFC-Digital-Forensic-Acquisition.md) | Architecture authority |


## Known limitations (honest)

- **File-backed / synthetic only** in this slice — no raw-disk, elevated
  broker, live/RAM, mobile, or cloud acquisition.
- **No production evidence claim** — do not use on real case media.
- UI cancellation is not yet wired to the core `cancel_flag` (cancel exists
  in core tests only).
- Split-RAW segmentation exists in the core but is not yet packaged into
  multi-segment `.fsnap` output.
- Formal accessibility audit tooling has not been run (manual ARIA review only).
- Independent Codex review of Day runbooks is still outstanding even where
  hosted CI is green.

## Security

Report vulnerabilities privately via
[GitHub Security Advisories](https://github.com/Trareon-com/Trareon-Acquire/security/advisories/new).
See [`SECURITY.md`](SECURITY.md). Do not open a public issue with exploit
details or sensitive data.

## License

[MPL-2.0](LICENSE)

## Attribution

Trareon Acquire — author attribution: Yusuf Shalahuddin Al Ayyubi As Sobari.
