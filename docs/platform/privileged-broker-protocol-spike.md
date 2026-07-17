# Privileged Broker Protocol Spike (Track C)

Result class: **Engineering Alpha — protocol spike only**. No elevation helper,
no raw-disk I/O, no shell, no Tauri wiring.

## Why

Days 23–25 showed that unprivileged UI processes cannot open Linux loop-control,
Windows `PhysicalDrive0` (non-elevated), or macOS `/dev/rdisk0` (non-operator).
RFC Track C requires an out-of-process, authenticated broker.

## Code

- Module: `crates/trareon-core/src/broker.rs`
- API: `BrokerRequest` / `BrokerResponse` / `evaluate_broker_request`
- Allowlisted ops only: `OpenAllowlistedSource`, `ReadRange`, `CloseSource`
- Valid structural requests return `NotImplemented` until a reviewed helper exists
- Shell-like strings, bad digests, and invalid ranges return `Denied`

## Explicit non-goals (this spike)

- Spawning UAC / sudo / launchd helpers
- Opening real disks
- Signing / session crypto beyond field presence checks
- Production readiness claims

## Next (M1/M3)

Implement OS helpers behind this contract; separate security review per
`docs/ai-operations/PHASE-MAPS/M3-RELEASE-CANDIDATE.md`.
