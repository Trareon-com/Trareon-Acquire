# Unsigned distribution — limitations (Hari 29)

Status: `LEGAL_DRAFT_ONLY` companion to [`../LEGAL-LIMITATIONS-DRAFT.md`](../LEGAL-LIMITATIONS-DRAFT.md).

Trareon Acquire Founder / Community builds are **unsigned** until project cash
funds signing/notarization (Gate 3). This is intentional under the zero-cash plan.

## What unsigned means

| Platform | What users see | What we claim |
|----------|----------------|---------------|
| Windows | SmartScreen / “Unknown publisher” | Binary is buildable from public source; not Microsoft-signed |
| macOS | Gatekeeper block until right-click Open / remove quarantine | Not Apple-notarized |
| Linux | No vendor signature on tarball | Checksum + source commit are the provenance |

## Operator obligations before use

1. Verify checksum against the release/workflow notice.
2. Prefer building from a signed git tag or `Build It For Me`.
3. Treat Lab Use Only / Engineering Alpha banners as binding.
4. GPLv3: corresponding source is the repository at the recorded commit.

## What we do **not** claim

- Court admissibility
- Antivirus / enterprise allowlisting
- “Safe because unsigned warning disappeared”
- Cross-OS raw-acquire PASS without matrix evidence

See per-OS guides: [`windows-unsigned.md`](windows-unsigned.md),
[`macos-unsigned.md`](macos-unsigned.md), [`linux-tarball.md`](linux-tarball.md).
