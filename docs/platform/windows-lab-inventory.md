# Windows lab inventory (Hari 8)

Status: **SOFTWARE PREP** — no Windows host evidence recorded yet.
Do not copy macOS tiny11 SHA rows into Windows capability cells.

Plan: [`m2-p01-windows-narrow-storage-plan.md`](m2-p01-windows-narrow-storage-plan.md)
Example: `cargo run -p trareon-core --example lab_windows_bounded_smoke`
Allowlist template: `fixtures/lab-allowlists/windows-usb-template.json`
Decision request: [`../ai-operations/DECISIONS/2026-07-17-windows-lab-media-decision-request.md`](../ai-operations/DECISIONS/2026-07-17-windows-lab-media-decision-request.md)

## Objective (Hari 8)

Record exact Windows lab host + removable USB identity so an operator can
approve a media-specific allowlist (`human_approved: true`) before any
elevated content smoke (Hari 9–11).

## Inventory checklist (fill on the Windows lab host)

| Field | Value (operator) |
|-------|------------------|
| Host name | |
| Windows edition / build (`winver`) | |
| Arch (x64 / ARM64) | |
| Controller / enclosure (redact serial if public) | |
| Media model / size | |
| Bus (USB 2/3 / other) | |
| `Get-Disk` number for removable media | |
| Device path | `\\.\PhysicalDriveN` (**never** `PhysicalDrive0`) |
| Sector size | |
| Free space on **different** staging volume | |
| Elevated admin available? (UAC) | |

## Safe discovery commands (PowerShell)

```powershell
Get-Disk | Format-Table Number, FriendlyName, BusType, Size, PartitionStyle, IsSystem, IsBoot
Get-Partition | Format-Table DiskNumber, PartitionNumber, DriveLetter, Size, Type
# Confirm system disk number before any allowlist edit:
Get-Disk | Where-Object { $_.IsSystem -or $_.IsBoot }
```

## Allowlist authoring (after HUMAN_APPROVAL)

1. Copy `fixtures/lab-allowlists/windows-usb-template.json` to a media-specific
   file (for example `fixtures/lab-allowlists/windows-usb-<short-id>.json`).
2. Set `entries[0].source_identity` to the exact `\\.\PhysicalDriveN` path.
3. Set `human_approved` to `true` and `approved_by` to the operator decision
   reference **only after** the decision file is accepted.
4. Keep `PhysicalDrive0` out of every allowlist.

## Next lab steps (not claimed here)

| Hari | Action | Gate |
|------|--------|------|
| 9 | Commit media-specific allowlist JSON | HUMAN_APPROVAL |
| 10 | Non-elevated open of `PhysicalDriveN` → expect Access Denied; elevated open-only | Operator UAC |
| 11 | Bounded 1 MiB + `trareon-verifier` via `lab_windows_bounded_smoke` | Operator UAC |

## Explicit non-claims

- No Windows raw content acquire PASS yet
- No Lab Beta / Official Production claim
- macOS tiny11 evidence does not promote Windows rows
