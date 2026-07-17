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

Known, currently-accepted advisory exceptions (all "unmaintained crate"
findings, not known vulnerabilities) are documented with their exact
`RUSTSEC-*` IDs and reasoning in `deny.toml`'s `[advisories] ignore` list —
most of them inherited transitively from Tauri 2's default Linux GTK3
backend, which upstream has marked unmaintained while the ecosystem moves
to GTK4.

## What is not covered here

- Physical/hardware security of any device running this software.
- Vulnerabilities in third-party dependencies themselves — report those
  upstream; `deny.toml` tracks which advisories we've reviewed and why.
- Signing, release, and provenance processes, which are out of scope until
  Track E (`docs/IMPLEMENTATION-ROADMAP.md`) is implemented.
