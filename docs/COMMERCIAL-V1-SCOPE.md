# Commercial v1.0 scope (zero-cash, sellable unsigned)

Date: 2026-07-17
Owner: operator
Policy: [`ZERO-CASH-LAUNCH-PLAN.md`](ZERO-CASH-LAUNCH-PLAN.md)

## What we sell (cheap, early)

| Tier | Price (target) | Deliverable |
|------|----------------|-------------|
| Community | Rp0 | Source on GitHub, self-build, `Build It For Me`, verifier |
| Founder | Rp199.000 | Full unsigned ready-to-run binary (offline fulfillment) + install guide + v1 updates |
| Individual Ready | Rp299.000 | After Founder stabilizes |

No certificate spend until project cash clears reserves (Gate 3).

**Sales channel:** paid binary is sold via **Lynk.id / Gumroad** (or equivalent
checkout). Fulfillment is **offline from the operator machine** — the paid
binary is **not** published as a GitHub Release artifact. GitHub stays source +
docs + CI only.

**No in-app license / activation.** The app is full once the buyer has the
binary. Purchase proof lives on the store (Lynk.id/Gumroad) +
`docs/commercial/fulfillment-ledger.csv`, not inside the app.

**License:** project is **GPLv3** — selling unsigned binaries is allowed; source
must be available to binary recipients (link the commit in `NOTICE.txt`).
Desktop shell is **Slint** (Tauri deprecated). See
[`docs/ai-operations/DECISIONS/2026-07-17-acquire-slint-gplv3.md`](ai-operations/DECISIONS/2026-07-17-acquire-slint-gplv3.md).

## v1 capability promise (honest)

**In scope** — must work on tested combinations:

- Removable USB / lab media RAW or split-RAW acquire
- Streaming SHA-256 + append-only audit + `.fsnap` v0.1
- Independent `trareon-verifier` post-check
- Allowlist + system-disk hard-deny
- Cooperative cancel + checkpoint/resume (file-backed; raw where evidenced)
- Guided / Standard / Expert UI modes (progressive disclosure) via **Slint**

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
5. Operator can produce a full unsigned binary locally (`scripts/founder-build.sh`)
   and deliver it privately (Lynk.id/Gumroad) with checksum + NOTICE — **not** via
   GitHub Releases
