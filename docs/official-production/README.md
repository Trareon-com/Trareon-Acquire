# Official production preparation (M19)

This pack implements the preparation boundary from
[M4 Official Production](../ai-operations/PHASE-MAPS/M4-OFFICIAL-PRODUCTION.md). It does not
authorize a release.

## Human sign-off checklist

- [ ] Frozen RC commit and artifact identity recorded
- [ ] Validation, security, reproducibility, documentation, and support evidence reviewed
- [ ] Checksums, SBOM/provenance, known limitations, and supported combinations published
- [ ] Every public claim maps to exact validation evidence
- [ ] Two independent human reviewers confirm no P0/P1 issue
- [ ] Legal, pricing, distribution, support, and incident-response owners approve
- [ ] Signing, notarization, publishing, and support activation performed by authorized humans

## Output from `scripts/build-rc.sh`

The script builds the release binary, runs targeted tests, writes checksums, captures the source
revision, and creates an SBOM placeholder in `dist/rc/`. Replace the placeholder with a real SBOM
from an approved human release process before publishing.

## Current decision

`PENDING_HUMAN_SIGN_OFF`

AI cannot approve, sign, notarize, publish, enroll stores, set pricing, accept legal terms, or
activate production support. Do not describe an RC artifact as production signed or notarized until
the named authorized humans record those actions.
