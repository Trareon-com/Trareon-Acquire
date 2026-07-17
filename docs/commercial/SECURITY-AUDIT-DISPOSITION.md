# Security audit disposition (Hari 72 summary)

Authoritative policy: `deny.toml` + [`../../SECURITY.md`](../../SECURITY.md).

## Current posture

- `cargo deny check` on every PR (`security` job).
- Slint transitive advisories accepted with documented IDs where needed.
- Former Tauri/`glib` Dependabot chain removed by Slint cutover.
- Secret-pattern scan in CI.
- `cargo metadata` artifact = provenance intent, not full CycloneDX SBOM.

## Open / deferred

- Full SBOM export automation (partial via founder-build checksums).
- Independent external review (Gate 3 cash).
- Full `cargo-fuzz` beyond bounded property suite.
