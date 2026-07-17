# M2 — Storage Lab Beta

## Entry evidence

- M1 deterministic file-backed/failure suite passes.
- Platform feasibility reports identify exact supported API/privilege candidates.
- Lab media, write-block process where applicable, hardware inventory, and destructive-test safeguards are approved.

## Scope

- Linux is the reference adapter for transparent raw-device engineering and fault injection.
- Windows is the first production validation priority: RAW/split-RAW, SHA-256, post-write verification, audit, `.fsnap`, CoC/report, and independent verifier.
- macOS adapter follows after shared contracts and Windows priority gates stabilize.
- Test stable identity, source/destination reversal, source unchanged, bad sector, disconnect, destination full, cancel, suspend/reboot/resume, long run, thermal, and performance.

## Exit evidence

- Known datasets are acquired completely or every anomaly/gap is measured and documented.
- Exact OS, architecture, controller, enclosure, media, privilege, filesystem, sector size, and security state are recorded.
- Each capability has pass/fail/NotValidated evidence; no umbrella “all OS” claim.
- Windows narrow-storage capability is ready to enter RC; other platforms advance independently.

## Anticipated incidents

Unstable device identity, permission policy differences, controller behavior, and hosted-versus-physical mismatch route to `PLATFORM-DIVERGENCE`. Unexpected write, privilege escalation, or sensitive-data exposure route to `SECURITY-FINDING` and human gate.

## Trigger

Generate detailed M2 prompts only after allowlisted hardware and the M1 exit bundle are approved.

**M1 exit prompts (draft):** `docs/ai-operations/ROLLING-WAVE/M2-FROM-M1.md` — hardware allowlist still required before destructive lab execution.

**Software prep status:** `docs/M2-SOFTWARE-PREP-STATUS.md` — allowlist + 1 MiB/64 MiB `rdisk10` samples PASS; full-disk and Lab Beta exit still gated.
