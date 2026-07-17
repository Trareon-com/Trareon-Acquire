# Commercial v1.0 scope (zero-cash, sellable unsigned)

Date: 2026-07-17  
Owner: operator  
Policy: [`ZERO-CASH-LAUNCH-PLAN.md`](ZERO-CASH-LAUNCH-PLAN.md)

## What we sell (cheap, early)

| Tier | Price (target) | Deliverable |
|------|----------------|-------------|
| Community | Rp0 | Source, self-build, `Build It For Me` artifact, verifier |
| Founder | Rp199.000 | Unsigned ready-to-run build + install guide + v1 updates |
| Individual Ready | Rp299.000 | After Founder stabilizes |

No certificate spend until project cash clears reserves (Gate 3).

## v1 capability promise (honest)

**In scope** — must work on tested combinations:

- Removable USB / lab media RAW or split-RAW acquire
- Streaming SHA-256 + append-only audit + `.fsnap` v0.1
- Independent `trareon-verifier` post-check
- Allowlist + system-disk hard-deny
- Cooperative cancel + checkpoint/resume (file-backed; raw where evidenced)
- Guided / Standard / Expert UI modes (progressive disclosure)

**Tested OS matrix (v1 target, not “all versions”):**

| OS | Target versions | Privilege | Distribution |
|----|-----------------|-----------|--------------|
| Windows | 10/11 x64 | UAC elevated for raw | Unsigned + SmartScreen doc |
| macOS | 12+ Apple Silicon (M4 lab) | `sudo` or `operator` group | Unsigned + Gatekeeper doc |
| Linux | Ubuntu 22.04+ / Kali rolling | root for loop lab; `disk` group boundary | Tarball / AppImage later |

**Explicitly out of v1 commercial claim:**

- Every macOS release / Intel Mac (unless separately evidenced)
- Every Windows edition / X270 lab
- Code signing / notarization (until funded)
- Court admissibility, accreditation, “works everywhere”
- RAM, mobile, cloud, E01/AFF4, carving, live acquisition
- Writable NTFS staging on macOS without paid driver

## Evidence already on record (2026-07-17)

- M1 file-backed Engineering Alpha exit
- macOS tiny11: file smoke, elevated open, 1 MiB + 64 MiB + full-disk `rdisk10` → Untitled (SHA recorded)
- CI green on public `main`
- Days 23–25 privilege probes (Linux/Windows/macOS)

## Success criteria for “bisa dijual” (Gate 2)

1. Same pipeline demonstrated on **Windows lab USB** (bounded + verify minimum)
2. Linux loop-device fault suite documented
3. UI cancel + verifier wired
4. Waitlist + Founder page live with limitation disclosure
5. Unsigned build script produces 3 OS artifacts from one tag
