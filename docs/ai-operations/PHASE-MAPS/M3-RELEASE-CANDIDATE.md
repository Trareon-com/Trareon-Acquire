# M3 — Release Candidate

## Entry evidence

- Windows narrow-storage Lab Beta capability passes required hardware/failure matrix.
- Feature scope is frozen; unsupported OS/hardware combinations remain explicit.
- Zero unresolved P0/P1 and known risks have owner/disposition.

## Scope

- Reproducible build comparison, SBOM, provenance/attestation, dependency/license review, SAST, fuzz regression, accessibility, documentation, localization boundary, support policy, and validation report.
- Privilege broker/helper and updater/installer boundaries receive separate security review.
- Signing/notarization availability is reported per platform; absence is visible and not bypassed.
- Independent external/community validation is sought against the same frozen artifact and protocol.

## Exit evidence

- Source revision, build workflow, artifact checksum, SBOM, attestation, validation status, limitations, and support period are inspectable.
- Installer/uninstaller and every executable component are tested on exact supported platform combinations.
- Documentation can reproduce supported acquisition and all failure/limitation states.
- Two-person review is available for production decision; one maintainer cannot self-approve release-critical claims.

## Anticipated incidents

Reproducibility mismatch routes to `FLAKY-OR-NONDETERMINISTIC`; packaging/signing gaps to `ENVIRONMENT-BLOCKED`; security findings to `SECURITY-FINDING`; release-claim ambiguity to `SPEC-AMBIGUITY`.

## Trigger

Generate detailed M3 prompts only after a capability-specific M2 validation bundle and frozen feature list exist.
