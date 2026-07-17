# Capability matrix — Bagian 1 (honest status)

Status legend: ✅ implemented (synthetic), 🔬 lab live gate pending/partial, ⏳ subset only, ❌ not claimed.

| Capability | Write | Read/Verify | Notes |
|---|---|---|---|
| RAW / `.fsnap` | ✅ | ✅ | Production path for file-backed |
| Split-RAW + `.fsnap` | ✅ | ✅ | Core API; UI wire optional |
| E01-lite | ✅ | ✅ | Pure Rust subset (`trareon.e01-lite/1`), not full libewf interop |
| AFF4-lite | ⏳ | ⏳ | ZIP+JSON subset, not full AFF4 RDF |
| VMDK / VHD / QCOW2 / DMG | ⏳ | ❌ | Minimal magic+payload wrappers |
| HPA/DCO detect | 🔬 | n/a | `trareon-ata` synthetic helpers ✅; live ATA passthrough Unavailable by default |
| HPA temporary unlock | ❌ | n/a | Requires `destructive-lab` feature; disabled by default |
| Write-blocker VID/PID | ✅ | n/a | Tableau/WiebeTech/CRU classify; live USB enum partial |
| Progress UI | ✅ | n/a | Core callback + Slint bar |
| Evidence ID + QR CoC | ✅ | n/a | Seal = SHA-256 envelope (no ed25519 dep yet) |
| RFC3161 | ⏳ | n/a | Offline skip; `TRAREON_TSA_URL` records configured-not-submitted |
| Verifier CLI | ✅ | ✅ | verify, --hash-only, info, compare, export, integrity |
| File / Block sources | ✅ / 🔬 | | Block needs allowlist + live gate |
| RAM / triage / net / cloud / mobile | ✅ synthetic | | In Acquire app as source kinds; live OS adapters partial |

Do not claim court-ready full-disk acquisition until Live Gate checklists pass per OS.
