# Day 23 — Linux Read-Only Feasibility Report

Result class: **Engineering Alpha feasibility spike**. This is not a raw-device
adapter and implements no production acquisition path. It answers one
question: can the current privilege level even reach the Linux raw-device
control surface, and what does that boundary look like in practice.

## Test device

- **Hardware:** physical Kali Linux machine (not a VM), accessed via SSH
  relay commands run by the human operator over RustDesk and reported back
  command-by-command; this session had no direct network reachability to
  the device.
- **OS:** Kali GNU/Linux Rolling, `VERSION_ID=2026.3`.
- **Kernel:** `6.18.9+kali-amd64` (`#1 SMP PREEMPT_DYNAMIC Kali 6.18.9-1kali1`), `x86_64`.
- **System disk:** `nvme0n1`, 476.9G, model `KYO`. **Never read from, written
  to, or otherwise touched** — it is this machine's own working disk
  (mounted `/` + swap), which the RFC and `docs/AI-DEVELOPMENT-WORKFLOW.md`
  explicitly forbid using as a development fixture. Serial number redacted
  per `docs/ai-operations/GITHUB-MONITORING.md`'s hardware-evidence
  redaction rule.
- **Synthetic fixture used instead:** an 8 MiB file created with
  `dd if=/dev/zero of=/tmp/trareon-lab-fixture.img bs=1M count=8` (and a
  second 4 MiB one for the non-root run), attached as a Linux loop device
  and detached again; deleted after each test. No real disk or partition
  was ever opened.

## Identity and privilege state (as observed)

| Account | uid | Groups relevant to raw-device access |
|---|---|---|
| `root` (RustDesk default session) | 0 | `disk` (implicit — root bypasses DAC) |
| `kali` (default non-root Kali user) | 1000 | `adm, dialout, cdrom, floppy, sudo, audio, dip, video, plugdev, users, netdev, scanner, bluetooth, lpadmin, wireshark, kaboxer, nordvpn` — **not** `disk` |

Block/loop device files are `brw-rw---- root:disk` (mode 660). `kali` is in
`sudo` (can elevate on demand) but is **not** in `disk`, so without invoking
`sudo` it has no direct raw-device group membership.

## Enumeration (read-only, root session)

```
lsblk -o NAME,SIZE,TYPE,MOUNTPOINT,RM,RO,MODEL,SERIAL
```

- 11 pre-existing loop devices, all snapd-mounted squashfs images
  (`gtk-common-themes`, `core24`, `gnome-46-2404`, `mesa-2404`, `snapd`,
  `telegram-desktop`), all read-only (`RO=1`).
- One real disk, `nvme0n1` (system disk, excluded as above).
- `losetup -f` correctly identified the next free loop device
  (`/dev/loop11`) without creating one.

## Synthetic loop-device attach/detach (root session)

```
dd if=/dev/zero of=/tmp/trareon-lab-fixture.img bs=1M count=8
losetup -r /dev/loop11 /tmp/trareon-lab-fixture.img   # read-only attach
lsblk /dev/loop11                                      # RO=1, SIZE=8M
blockdev --getro /dev/loop11                           # 1 (read-only confirmed)
udevadm info --query=all --name=/dev/loop11            # stable identity fields: DEVPATH, DEVNAME, DISKSEQ, by-loop-ref symlink
losetup -d /dev/loop11                                 # detach, clean
rm /tmp/trareon-lab-fixture.img
```

All steps exited 0. The attach was explicitly read-only (`-r`), confirmed by
`blockdev --getro`. `udevadm info` shows Linux's `disk/by-loop-ref/<path>`
symlink scheme is a usable, backing-file-derived stable identity signal for
loop devices specifically (not generalizable to physical disks, which use
`by-id`/`by-uuid`/`by-diskseq` instead — not tested here).

## Non-root privilege boundary (the actual finding)

```
whoami; id                                    # kali, uid=1000, no 'disk' group
lsblk -o NAME,SIZE,TYPE,MOUNTPOINT,RM,RO       # succeeds — listing is world-readable
ls -la /dev/loop11 /dev/nvme0n1                # succeeds — stat/metadata is world-readable
dd if=/dev/zero of=/tmp/nonroot-fixture.img bs=1M count=4   # succeeds — /tmp is writable
losetup -f                                     # FAILS: "cannot find an unused loop device: Permission denied"
losetup -fr /tmp/nonroot-fixture.img           # FAILS: same permission error, exit 1
```

**Metadata enumeration (`lsblk`, `ls -la`, `stat`) is available to any user.
Actually attaching/binding a loop device (`losetup`) is denied outright**
without `sudo` or `disk` group membership — the denial happens even when
merely asking `losetup -f` for a free device number, before any file is
touched. This is a real, reproducible privilege boundary, not guessed at.

`crates/trareon-core/src/platform::linux::probe_loop_control()` reimplements
this exact probe in Rust (opening `/dev/loop-control` for read+write without
issuing any ioctl) so the same classification can be exercised by CI and by
future Track C code, without granting or assuming any privilege.

## Capability and limitation matrix

| Capability | Root | Non-root (`kali`, no `disk` group) |
|---|---|---|
| Enumerate block/loop devices (`lsblk`) | Available | Available |
| Read device metadata (`ls -la`, `stat`, `udevadm info`) | Available | Available |
| Open `/dev/loop-control` for read+write | Available | `DeniedInsufficientPrivilege` |
| Attach synthetic file as read-only loop device | Available (verified) | `NotValidated` — denied before reaching the attach step |
| Attach/read a **real physical disk** | `NotValidated` — not tested; system disk was explicitly out of scope | `NotValidated` — not tested |
| Windows/macOS equivalents | `NotValidated` — Day 24/25, separate report | `NotValidated` |

Everything not marked `Available` above is `NotValidated`, not "not
supported" — no negative claim is made beyond what was actually observed on
this exact kernel/distribution/account combination.

## Conclusion for Track C

A production Linux raw-device adapter cannot run the main UI process as
root and cannot rely on unprivileged users having `disk` group membership
by default. The privileged-broker design in `docs/IMPLEMENTATION-ROADMAP.md`
(Track C) is directly validated by this finding: some out-of-process,
explicitly-authorized elevation step is required before any real loop or
block device can be bound, exactly matching the RFC's "main UI tidak
elevated; broker command surface diautentikasi" requirement.
