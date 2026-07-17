# Day 24 — Windows Read-Only Feasibility Report

Result class: **Engineering Alpha feasibility spike**. This is not a raw-device
adapter and implements no production acquisition path, and the system disk
was never written to. It answers one question: can the current privilege
level even reach the Windows raw-device control surface, and what does that
boundary look like in practice.

## Test device

- **Hardware:** the user's own Windows machine (`DESKTOP-1HPI419`), accessed
  via RustDesk with the human operator running each command and reporting
  results back verbatim. This session had no direct network reachability to
  the device — a same-subnet SSH attempt (`192.168.0.109`, port 22, service
  confirmed `Running`) failed with "No route to host" from this session's
  environment, same as the earlier Linux attempt on a different subnet,
  confirming this session's sandbox has no real LAN access at all rather
  than a routing/firewall issue specific to either machine.
- **OS:** `Windows 10 Pro`, `WindowsVersion 2009` (20H2), `OsHardwareAbstractionLayer 10.0.22621.2506`.
  The HAL build number (22621, a Windows 11 22H2-era kernel build) reported
  alongside a `WindowsVersion` of `2009` (Windows 10 20H2) is recorded
  exactly as observed — a divergence worth treating as `NotValidated` for
  any "exact Windows 10 20H2 behavior" claim, not smoothed over.
- **Architecture:** x64-based PC (`CsSystemType`).
- **System disk:** `PhysicalDrive0`, model `KYO 1TB`, NVMe SSD, `IsSystem: True`, `IsBoot: True`. **Never written to.** A read-only handle was opened and immediately closed (see below); no bytes were ever read or written through it. Serial number redacted per `docs/ai-operations/GITHUB-MONITORING.md`'s hardware-evidence redaction rule.
- **Volumes:** `C:` NTFS 268.3 GB, `D:` NTFS 755.8 GB, one unlettered FAT32 100 MB volume (consistent with an EFI system partition).

## Identity and privilege state (as observed)

| Session | Account | `IsInRole(Administrator)` | `\\.\PhysicalDrive0` read-handle open |
|---|---|---|---|
| Elevated PowerShell (opened via admin prompt) | `DESKTOP-1HPI419\Ucup` (member of `BUILTIN\Administrators`) | `True` | **Available** |
| Regular (non-elevated) PowerShell, same account | `DESKTOP-1HPI419\Ucup` | `False` | **Denied** — `Access to the path '\\.\PhysicalDrive0' is denied.` |

This is the key finding, and it is more specific than a group-membership
check: **the same user account, same machine, same moment**, produces
different raw-device access depending only on whether the current process
holds an elevated (UAC-elevated) token. Group membership in
`BUILTIN\Administrators` is necessary but not sufficient — the process must
actually be running elevated. This is a materially different privilege
model from Linux's DAC-group-based boundary observed on Day 23, and any
future Windows adapter/broker design must account for triggering and
verifying UAC elevation explicitly, not just checking group membership.

## Read-only enumeration (elevated session)

```powershell
Get-Disk | Select-Object Number, FriendlyName, SerialNumber, Size, IsSystem, IsBoot, OperationalStatus
Get-Volume | Select-Object DriveLetter, FileSystemType, Size, SizeRemaining
Get-PhysicalDisk | Select-Object DeviceId, FriendlyName, MediaType, BusType
```

All three commands succeeded and returned metadata only — no content was
read from any volume or disk.

## Raw handle open/close (elevated session, read-only, no bytes touched)

```powershell
try {
    $stream = [System.IO.File]::Open("\\.\PhysicalDrive0", [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::ReadWrite)
    "Open succeeded (handle only, no bytes read)"
    $stream.Close()
} catch {
    "Open failed: $($_.Exception.Message)"
}
```

Result: `Open succeeded (handle only, no bytes read)`. The handle was
explicitly opened with `FileAccess.Read` and `FileShare.ReadWrite` (allowing
the OS to keep its own concurrent access to the boot/system disk) and closed
immediately — no `Seek`/`Read` call was ever issued.

## Raw handle open (non-elevated session, same account)

Same exact command, run from a regular (non-elevated) PowerShell window
under the same `Ucup` account:

```
Open failed: Exception calling "Open" with "4" argument(s): "Access to the path '\\.\PhysicalDrive0' is denied."
```

Confirms the privilege boundary is elevation-state-based, not merely
account-based.

## Code

`crates/trareon-core/src/platform.rs`'s `windows::probe_physical_drive_zero()`
reimplements this exact probe in Rust: opens `\\.\PhysicalDrive0` with
`OpenOptionsExt::share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)` (matching
the PowerShell probe's `FileShare.ReadWrite`), read+write access requested,
never reads or writes through the handle. No `unsafe`, no new dependency —
`std::os::windows::fs::OpenOptionsExt` is standard library. Classifies the
same three states (`Available` / `DeniedInsufficientPrivilege` /
`NotValidated`) as the Linux probe from Day 23, so CI on `windows-latest`
exercises the same code path (its actual result there depends on the hosted
runner's elevation state, which is not asserted to a fixed value).

## Capability and limitation matrix

| Capability | Elevated (UAC) | Non-elevated, same admin-group account |
|---|---|---|
| Enumerate disks/volumes (`Get-Disk`, `Get-Volume`, `Get-PhysicalDisk`) | Available | `NotValidated` — not tested non-elevated, but these are standard PowerShell cmdlets with no raw-device access, expected `Available` |
| Open raw physical-disk read handle | Available (verified) | `DeniedInsufficientPrivilege` (verified) |
| Read/write raw physical-disk bytes | `NotValidated` — not tested; system disk was explicitly out of scope beyond the handle-open probe | `NotValidated` |
| ThinkPad X270 (lower-spec Windows lab machine per `docs/AI-DEVELOPMENT-WORKFLOW.md`) | `NotValidated` — this report used the user's primary Windows machine, not X270 | `NotValidated` |
| Linux/macOS equivalents | Linux: see Day 23 report. macOS: `NotValidated` — Day 25, separate report | `NotValidated` |

Everything not marked `Available`/`DeniedInsufficientPrivilege` above is
`NotValidated`, not "not supported" — matching this Day's acceptance focus
that X270 (or any single machine) must not become the only production
validation, and that untested OS/hardware combinations stay `NotValidated`.

## Conclusion for Track C

A production Windows raw-device adapter cannot assume that Administrators-
group membership alone grants raw-disk access — it must trigger and verify
actual UAC elevation (e.g. via a manifest requiring `requireAdministrator`,
or a separate elevated broker process/service launched with explicit user
consent), consistent with the RFC's "main UI tidak elevated; broker command
surface diautentikasi" requirement and directly paralleling the Linux
privileged-broker finding from Day 23, but via a different underlying
mechanism (UAC token elevation vs. DAC group membership).
