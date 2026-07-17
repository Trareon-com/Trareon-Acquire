# M2-P04 — Privileged broker elevation design (spike)

Status: **DESIGN ONLY** — `StubElevationHelper` remains the only implementation.

## Current code

- Allowlisted ops: open / read-range / close
- Shell-like payloads denied
- `evaluate_broker_request` → structurally valid ops return `NotImplemented`
- `StubElevationHelper::elevate_for` never performs OS elevation

## Required before replacing the stub

1. Human approval recorded (this file alone is insufficient)
2. Separate review of helper binary / service
3. No shell, no network, no arbitrary path open
4. Session token + plan digest binding
5. Lab allowlist integration for `source_identity`

## Safe default

Keep `StubElevationHelper`. Any PR that adds real UAC/sudo without human gate is out of policy.
