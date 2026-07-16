# Trareon Acquire AI Operations

Folder ini adalah control pack untuk pengembangan Trareon Acquire. Urutan otoritasnya adalah RFC v1.0, spesifikasi prompt operations, runbook Day aktif, test oracle, lalu implementasi. Percakapan AI tidak dapat mengganti urutan tersebut.

Codex bekerja local-only. Claude Code menjadi author/GitHub gateway default. Antigravity melakukan validasi UI/eksploratori atau menjadi gateway cadangan hanya ketika runbook menugaskannya. Satu task mempunyai satu author dan satu gateway aktif.

Jalankan Day secara berurutan. Handoff menggunakan exact commit SHA; hasil lokal, remote, PR, CI, dan evidence harus menunjuk SHA yang sama. Status tanpa command, exit code, dan evidence adalah `UNVERIFIED`.

Mulai dari `START-HERE.md`. Validasi pack dengan:

```bash
sh scripts/validate-ai-operations.sh
```

Barang bukti nyata, disk image sensitif, credential, signing key, dan data pribadi tidak boleh masuk prompt, GitHub, screenshot, log, atau artifact.
