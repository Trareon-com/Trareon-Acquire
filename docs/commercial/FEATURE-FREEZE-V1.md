# Feature freeze note — commercial v1 software slice

Date: 2026-07-17
Status: **SOFTWARE FREEZE CANDIDATE** (not a release gate)

## Frozen for Founder unsigned path (software)

- Slint shell: Guided / Standard / Expert modes, cancel → `cancel_flag`,
  SHA/size display, allowlist/elevation preflight, CoC JSON export
- Core file-backed acquire / audit / `.fsnap` / verifier
- Lab allowlist + system-disk hard-deny
- Community Build It For Me + `scripts/founder-build.sh` / `self-test.sh`
- Install + commercial drafts under `docs/install/` and `docs/commercial/`

## Explicitly not frozen / still open

- Windows raw lab evidence → [`../platform/WINDOWS-LAB-OPERATOR-PACK.md`](../platform/WINDOWS-LAB-OPERATOR-PACK.md)
- Linux physical loop attach (software prep ready)
- Signing / notarization (Gate 3)
- Progressive polish beyond three modes
- Waitlist / Founder live pages (drafts only)

## Rule

No scope expansion into RAM/mobile/cloud/E01 until after Founder Gate 2 decision.
