# M2-P04 — Privileged broker elevation design (spike)

Status: **DESIGN + allowlist-bound stub** — `StubElevationHelper` remains the only
implementation; **no OS elevation**.

## Current code

- Allowlisted ops: open / read-range / close
- Shell-like payloads denied
- System-disk identities hard-denied (`rdisk0`, `PhysicalDrive0`, `nvme0n1`, …)
- Block-device identities require human-approved lab allowlist
  (`evaluate_broker_request_with_allowlist`, `StubElevationHelper::with_allowlist`)
- Structurally valid ops still return `NotImplemented` (no open/read)

## Threat model (lab helper)

| Threat | Mitigation |
|---|---|
| Shell / script injection | Deny shell-like tokens in identity and auth fields |
| System disk imaging | Hard-deny path patterns; never allowlist `disk0` |
| Path confusion | Exact allowlist string match only |
| Network exfil from helper | Helper must have no network APIs (future binary review) |
| Silent privilege claim | Stub never returns `Accepted` for elevation |

## Future helper sketch (not implemented)

1. Separate helper binary / service per OS (macOS `root:operator`, Windows UAC, Linux `CAP_SYS_ADMIN` / disk group)
2. Local IPC only (Unix socket / named pipe); no TCP
3. Session token + plan digest binding; nonce anti-replay
4. Ops limited to open / pread range / close on allowlisted identity
5. Human gate recorded before enabling any non-stub helper in CI or releases

## Required before replacing the stub

1. Human approval recorded (decision file — this plan alone is insufficient)
2. Separate review of helper binary / service
3. No shell, no network, no arbitrary path open
4. Session token + plan digest binding
5. Lab allowlist integration for `source_identity` — **done in core protocol**

## Safe default

Keep `StubElevationHelper`. Any PR that adds real UAC/sudo without human gate is out of policy.
