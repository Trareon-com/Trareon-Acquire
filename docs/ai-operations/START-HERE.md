# Start Here

Gunakan file ini sebagai satu-satunya pintu masuk operasional. GitHub menjadi dashboard monitoring; MacBook tetap control plane dan perangkat lain menjadi lab.

## Lima tindakan harian

1. Buka baris pertama berstatus `NOT_STARTED` di `MASTER-CHECKLIST.md` dan Issue GitHub yang terhubung.
2. Buka runbook `MONTH-01/DAY-NN.md`; pastikan Entry Gate dan frozen SHA sesuai.
3. Salin **hanya prompt yang sedang diperintahkan** ke AI yang disebut. Jangan menjalankan prompt author, reviewer, dan validator secara bersamaan.
4. Simpan `TaskResult.v1`, exact command/exit code, dan evidence; Claude Code merelay hasil Codex ke PR tanpa mengubahnya.
5. Pilih reviewer/recovery prompt berdasarkan gate dan incident. Maju hanya setelah independent review memberi `EXPECTED_PASS`.

Durasi, banyaknya file, atau klaim AI bukan bukti selesai. Hasil tanpa exact SHA dan evidence adalah `UNVERIFIED`.

## Lingkungan

- Root utama: `/Users/user/Projects/Trareon/Trareon Acquire`.
- Author bekerja di `.worktrees/claude-day-NN` atau worktree yang dicatat runbook.
- Codex membaca checkout/worktree lokal dan tidak mengakses GitHub.
- Claude Code adalah GitHub gateway default: push task branch, siapkan PR, pantau CI, relay review Codex.
- Antigravity memvalidasi UI pada frozen SHA dan menjadi gateway cadangan hanya ketika ditugaskan.
- ThinkPad Windows dan Kali mengambil trusted frozen commit; keduanya mengembalikan platform report, tidak mengedit worktree author.

## GitHub monitoring

Pantau Project dengan alur `Backlog → Ready → Claude Implementing → Codex Reviewing → Antigravity Validating → CI Running → Hardware Validation → Human Approval → Done`. Issue, PR, CI, dan evidence harus menunjuk exact SHA yang sama.

Jangan unggah barang bukti nyata, disk image sensitif, credential, signing key, atau data pribadi. Capability tanpa hardware evidence tetap `NotValidated`.

## Mulai

Mulai dari `MONTH-01/DAY-01.md`. Bila result bukan `EXPECTED_PASS`, buka file yang dipetakan di `RECOVERY-PROMPTS/INDEX.md`; jangan melompat ke Day berikutnya.
