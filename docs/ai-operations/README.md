# Trareon Acquire AI Operations

Folder ini adalah control pack untuk pengembangan Trareon Acquire. Urutan otoritasnya adalah RFC v1.0, spesifikasi prompt operations, runbook Day aktif, test oracle, lalu implementasi. Percakapan AI tidak dapat mengganti urutan tersebut.

Codex bekerja local-only. Claude Code menjadi author/GitHub gateway default. Antigravity melakukan validasi UI/eksploratori atau menjadi gateway cadangan hanya ketika runbook menugaskannya. Satu task mempunyai satu author dan satu gateway aktif.

Jalankan Day secara berurutan. Handoff menggunakan exact commit SHA; hasil lokal, remote, PR, CI, dan evidence harus menunjuk SHA yang sama. Status tanpa command, exit code, dan evidence adalah `UNVERIFIED`.

Mulai dari `START-HERE.md`. Validasi pack dengan:

```bash
sh scripts/validate-ai-operations.sh
```

Barang bukti nyata, disk image sensitif, credential, signing key, dan data pribadi tidak boleh masuk prompt, GitHub, screenshot, log, atau artifact.

## Navigation

- `START-HERE.md`: urutan penggunaan operator.
- `MASTER-CHECKLIST.md`: monitoring Day 01-30.
- `GITHUB-MONITORING.md`: Project/Issue/PR/Actions evidence flow.
- `MONTH-01/`: runbook copy-paste harian.
- `RECOVERY-PROMPTS/`: routing untuk hasil tidak terduga.
- `PHASE-MAPS/`: entry/exit M1-M4 tanpa mengarang hasil platform masa depan.
- `PACK-VALIDATION.md`: hasil automated dan manual coverage checks untuk pack ini.
