# M4 — Official Production

## Entry evidence

- Capability-specific Release Candidate passes validation, security, reproducibility, documentation, and support gates.
- Official artifact identity, exact source revision, SBOM/provenance, checksums, limitations, and independent review are complete.
- Human owner approves legal, pricing, distribution, support, and incident-response responsibilities.

## Scope

- Publish only the Windows narrow-storage capabilities with evidence; Linux/macOS or extended methods remain separate capability decisions.
- Record official artifact, supported combinations, validation report, known limitations, release/support period, and upgrade/revalidation policy.
- Keep source available according to license while ready-to-run binaries follow the approved commercial channel.

## Exit evidence

- Two-person sign-off confirms no P0/P1, all required evidence exact-SHA matches, and every claim maps to a validation result.
- Users can verify artifact checksum/provenance and see support/limitation status in-app.
- Major update, acquisition-affecting minor update, repair, or platform change has an explicit revalidation trigger.
- Analysis consumes production packages read-only through the frozen versioned contract.

## Protected human actions

Publishing, signing, notarization, store enrollment, payment, pricing, legal acceptance, customer communication, and production support activation are human actions. AI prepares evidence and drafts but cannot execute or approve them.

## Anticipated incidents

Artifact/evidence SHA mismatch routes to `REMOTE-STATE-DIVERGENCE`; signing/service failure to `ENVIRONMENT-BLOCKED`; unsupported marketing claim to `SPEC-AMBIGUITY`; attempted release without authority to `HUMAN_APPROVAL_REQUIRED`.

## Trigger

Generate release prompts only for a named frozen RC artifact after all entry evidence and two-person sign-off are present.
