# Day 25 — macOS Read-Only Feasibility Report

Result class: **Engineering Alpha feasibility spike**. This is not a raw-device
adapter and implements no production acquisition path. It answers whether the
current privilege / SIP / TCC posture can reach macOS raw-disk control
surfaces, without requesting elevation or Full Disk Access prompts.

## Test device

- **Hardware:** physical MacBook Pro (not a VM), Apple M4 Pro, model
  identifier `Mac16,8`, 24 GB RAM. Serial redacted.
- **OS:** macOS `26.5.2` (`BuildVersion` `25F84`), Darwin `25.5.0`,
  `RELEASE_ARM64_T6041`, `arm64` (`hw.optional.arm64=1`).
- **Account:** `user` uid=501, groups include `staff` and `admin`, **not**
  `operator` (`dsmemberutil checkmembership` → not a member).
- **System disk:** internal physical `disk0` (~500.3 GB, GUID /
  Apple_APFS). **Never read from, written to, or imaged** — metadata
  enumeration only via `diskutil list` / `ls -la /dev/disk*`. Serial and
  volume contents were not used as fixtures.
- **Synthetic attach attempt:** `hdiutil create` of an 8 MiB lab DMG under
  `/tmp` failed with `Operation not permitted` in this agent session
  (TCC/session restriction). Marked `NotValidated` for synthetic image
  attach — not claimed as a platform impossibility.

## Security posture (as observed)

| Control | Observation |
|---|---|
| System Integrity Protection (`csrutil status`) | **enabled** |
| Authenticated Root (`csrutil authenticated-root status`) | **enabled** |
| TCC DB listing (`~/Library/Application Support/com.apple.TCC`) | `Operation not permitted` — DB not readable without FDA / SIP-exempt tooling |
| Privileged helper / launchd broker for Trareon | **Not installed / NotValidated** — none probed beyond absence |

No privilege prompt was requested. No Full Disk Access grant was sought.

## Enumeration (read-only)

`diskutil list` succeeded without elevation and reported:

- Internal physical `disk0` with APFS containers (system volumes under
  synthesized `disk3`, including Macintosh HD / Data / VM).
- Pre-existing read-only disk images for Simulator / tooling (`disk4`–
  `disk9`) — not opened for acquisition.

Device nodes are typically `brw-r----- root:operator` (block) /
`crw-r----- root:operator` (character `rdisk*`). Admin membership alone does
**not** imply `operator` group access.

## Raw-node open probe (no read/write through handle)

| Path | O_RDWR | O_RDONLY |
|---|---|---|
| `/dev/null` | Available | Available |
| `/dev/disk0` | `DeniedInsufficientPrivilege` (EPERM) | `DeniedInsufficientPrivilege` (EPERM) |
| `/dev/rdisk0` | `DeniedInsufficientPrivilege` (EPERM) | `DeniedInsufficientPrivilege` (EPERM) |
| `/dev/disk1` | `DeniedInsufficientPrivilege` (EACCES) | `DeniedInsufficientPrivilege` (EACCES) |
| `/dev/rdisk1` | `DeniedInsufficientPrivilege` (EACCES) | `DeniedInsufficientPrivilege` (EACCES) |

`crates/trareon-core::platform::macos::probe_rdisk0()` reimplements the
`/dev/rdisk0` open classification in Rust (open for read+write, no I/O
through the handle, no `unsafe`, no new dependency) so CI and future Track C
code exercise the same three-way result.

## Capability and limitation matrix

| Capability | Unprivileged admin (this session) |
|---|---|
| Enumerate disks (`diskutil list`) | Available |
| Stat `/dev/disk*` / `/dev/rdisk*` metadata (`ls -la`) | Available |
| Open `/dev/rdisk0` or `/dev/disk0` (rdonly/rdwr) | `DeniedInsufficientPrivilege` |
| Read system-disk contents / image internal SSD | `NotValidated` — explicitly not attempted |
| Synthetic `hdiutil` create/attach lab image | `NotValidated` — create denied (`Operation not permitted`) in this session |
| Intel Mac equivalent | `NotValidated` — this run is arm64 M4 Pro only |
| TCC Full Disk Access grant path | `NotValidated` — no prompt requested |
| Privileged helper / authenticated broker | `NotValidated` — not implemented |

## Conclusion for Track C

A production macOS raw-device path cannot assume an unprivileged (even
admin) UI process can open `/dev/rdisk*`. With SIP and Authenticated Root
enabled, and without `operator` membership or an authorized helper, opens
fail closed as `DeniedInsufficientPrivilege`. This matches the privileged-
broker design in `docs/IMPLEMENTATION-ROADMAP.md` (Track C): elevation must
be explicit and out-of-process; Day 25 does not implement that broker.
