# Security Policy

Result class: **Engineering Alpha**. This project is a foundation-slice
forensic acquisition tool under active development; see
`docs/IMPLEMENTATION-ROADMAP.md` for scope and status.

## Reporting a vulnerability

Report suspected vulnerabilities privately via [GitHub Security
Advisories](https://github.com/Trareon-com/Trareon-Acquire/security/advisories/new).
Do not open a public Issue with vulnerability details, exploit code, or any
sensitive data. This matches the reporting channel already linked from
`.github/ISSUE_TEMPLATE/config.yml`.

Please include: affected component/file, reproduction steps, impact, and
any suggested fix. We will acknowledge the report and work with you on
disclosure timing before any public fix or advisory is published.

## What's in scope

- The Rust core (`crates/trareon-core`, `crates/trareon-verifier`)
- The Tauri/Svelte application (`apps/trareon-acquire`)
- CI/CD workflows (`.github/workflows/`)

## Automated checks

Every pull request and push to `main` runs a `security` CI job
(`.github/workflows/ci.yml`) that:

- Runs `cargo deny check` (license policy, security advisories, banned/
  duplicate dependency bans, dependency source allow-list) against
  `deny.toml`.
- Runs `npm audit --omit=dev` against the Tauri app's production
  dependencies.
- Scans the tree for common committed-secret patterns (private key
  blocks, AWS access key IDs, hardcoded `aws_secret_access_key`).
- Publishes a `cargo metadata`-derived dependency manifest as a build
  artifact. This is provenance-intent only — it is not a full CycloneDX
  or SPDX SBOM.

Known, currently-accepted advisory exceptions are documented with their
exact `RUSTSEC-*` IDs and reasoning in `deny.toml`'s `[advisories] ignore`
list — most inherited transitively from Tauri 2's default Linux GTK3
backend, which upstream has marked unmaintained while the ecosystem moves
to GTK4.

### Dependabot alert: `glib` GHSA-wrw7-89jp-8q8g / RUSTSEC-2024-0429

| Field | Value |
|---|---|
| Package | `glib` **0.18.5** (Linux-only, transitive) |
| Severity | Medium (unsoundness in `VariantStrIter`) |
| Patched | `glib >= 0.20.0` |
| Status | **Accepted / dismissed** — no compatible upgrade on Tauri 2 |

**Why it cannot be fixed in-tree today:** Tauri 2.11.x pulls `gtk 0.18` →
`glib ^0.18`. The GTK3 `gtk-rs` line is unmaintained and has no `glib 0.20`
release. Forcing `glib 0.20` fails Cargo resolution. A real fix requires
Tauri's Linux stack to move to GTK4 / WebKitGTK6 (upstream work, not a
Trareon patch).

**Why risk is tolerable for this repo:** the advisory is Linux GUI
backend only (not compiled into Windows/macOS builds); Trareon Acquire
does not call `glib::VariantStrIter`; CI still tracks it via
`cargo deny` ignore with rationale. Re-open / upgrade when a Tauri
release ships the GTK4 backend.

GitHub: https://github.com/Trareon-com/Trareon-Acquire/security/dependabot/1  
Upstream: https://github.com/tauri-apps/tauri/issues/12048

## What is not covered here

- Physical/hardware security of any device running this software.
- Vulnerabilities in third-party dependencies themselves — report those
  upstream; `deny.toml` tracks which advisories we've reviewed and why.
- Signing, release, and provenance processes, which are out of scope until
  Track E (`docs/IMPLEMENTATION-ROADMAP.md`) is implemented.
