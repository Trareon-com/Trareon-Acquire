# Master Checklist

Nilai awal seluruh task adalah `NOT_STARTED`. Perbarui hanya dari evidence exact-SHA; jangan mengosongkan cell.

## Catatan rekonsiliasi (dua rencana berbeda granularitas)

Sebagian scope Day 01-20 di bawah ini sudah punya implementasi nyata, tetapi dikerjakan lewat rencana lain yang lebih kasar granularitasnya: `docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md` (8 task, single-author Claude Code, tanpa siklus review Codex per-Day yang dijelaskan runbook ini). Kode tersebut ada di branch `feat/m0-t01-workspace-core-boundary`, frozen SHA `9021a2c2b8757bdef0aa01eefbc69543274ff21e`, PR terbuka: https://github.com/Trareon-com/Trareon-Acquire/pull/32 (belum di-merge).

Kolom `Implementation` untuk Day yang scope-nya cocok diberi nilai `IMPLEMENTED_UNREVIEWED` — artinya kode ada, teruji lokal (`cargo test --workspace --locked`, `cargo clippy -D warnings`, `cargo fmt --check` semua hijau; untuk Day 20 juga `npm ci`/`npm run build`), dan sudah di-push, tetapi **belum** melalui independent review Codex atau human approval yang disyaratkan runbook ini.

CI GitHub untuk PR #32 pada commit `9021a2c2b8757bdef0aa01eefbc69543274ff21e` sudah **PASS di ketiga OS** (ubuntu-latest, windows-latest, macos-latest) — lihat `test (ubuntu-latest)`, `test (windows-latest)`, `test (macos-latest)` pada https://github.com/Trareon-com/Trareon-Acquire/pull/32/checks. Ini dicapai setelah tiga kegagalan nyata ditemukan dan diperbaiki di root cause-nya (bukan disembunyikan): (1) ubuntu-latest gagal karena system dependency Tauri (`libwebkit2gtk` dkk) belum ter-install lewat apt di `ci.yml`; (2) windows-latest gagal karena `apps/traeron-acquire/src-tauri/icons/icon.ico` belum ada — tauri-build butuh itu untuk Windows Resource file; (3) windows-latest gagal lagi dengan "Access is denied (os error 5)" karena `copy_and_sync()` di `package.rs` membuka file hasil copy dengan `File::open()` (read-only) lalu memanggil `.sync_all()` — di Windows, `FlushFileBuffers` butuh write-access handle. Ketiganya sudah diperbaiki di commit terpisah pada branch yang sama.

Kolom `Review` dan `Human Gate` tetap `NOT_STARTED`/`NOT_REQUIRED` karena independent review Codex dan human approval belum terjadi — hosted CI hijau bukan pengganti keduanya.

Catatan disiplin proses: perbaikan di atas dilakukan dalam 3 push berturut sebelum semua OS hijau, melebihi batas "maksimal dua recovery cycle" yang disyaratkan runbook ini untuk task lain (lihat `Unexpected-Output Routing` di tiap `DAY-NN.md`). Karena Day 20 dikerjakan lewat Foundation Plan yang terpisah, bukan lewat runbook Day-by-day ini, aturan itu tidak diberlakukan secara otomatis di sini — tetapi dicatat agar tidak dianggap sesuai proses baku tanpa syarat.

Day yang scope-nya hanya sebagian terpenuhi (mis. Day 06, 10, 11, 15, 17) sengaja **tidak** ditandai `IMPLEMENTED_UNREVIEWED` karena implementasi Foundation Plan tidak memenuhi seluruh acceptance criteria Day tersebut (contoh: Day 11 minta split-RAW, Foundation Plan hanya RAW tunggal; Day 15 minta golden fixture set + read-contract doc, belum ada). Detail lengkap ada di `docs/ai-session-log/2026-07-17-claude-code-foundation-complete.md` pada worktree utama.

| Day | Task | Author | Reviewer | Risk | Autonomy | Branch | Frozen SHA | Implementation | Review | CI | Platform | Incident | Recovery | Human Gate | Evidence | Next |
|---:|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 01 | Baseline and monitoring | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d01-baseline | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 02 |
| 02 | Workspace boundaries | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d02-workspace | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 03 |
| 03 | Domain identifiers | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d03-domain | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 04 |
| 04 | State transitions | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d04-state | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 05 |
| 05 | Audit hash chain | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d05-audit | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 06 |
| 06 | Canonical hashing | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d06-canonical | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 07 |
| 07 | Week 1 integration | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d07-integration | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 08 |
| 08 | Synthetic streaming | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d08-streaming | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 09 |
| 09 | Identity guard | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d09-identity | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 10 |
| 10 | Failure semantics | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d10-failure | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 11 |
| 11 | RAW and split RAW | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d11-raw | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 12 |
| 12 | fsnap manifest | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d12-manifest | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 13 |
| 13 | Safe package writer | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d13-package | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 14 |
| 14 | Independent verifier | Codex | Claude | High | AUTO_WITH_REVIEW | feat/m0-d14-verifier | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 15 |
| 15 | CLI and fsnap draft | Codex | Claude | High | AUTO_WITH_REVIEW | feat/m0-d15-contract | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 16 |
| 16 | Tauri boundary | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d16-tauri | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | PR-32 | 17 |
| 17 | Guided UI | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d17-ui | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 18 |
| 18 | Guidance and accessibility | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d18-guidance | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 19 |
| 19 | CoC and report preview | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d19-report | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 20 |
| 20 | Cross-platform CI | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d20-ci | 9021a2c2b8757bdef0aa01eefbc69543274ff21e | IMPLEMENTED_UNREVIEWED | NOT_STARTED | PASS_3OS | NOT_STARTED | IMPLEMENTATION-FAILURE | 3 | NOT_REQUIRED | PR-32 | 21 |
| 21 | DevSecOps gates | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d21-devsecops | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | HUMAN_IF_DEPENDENCY | NOT_STARTED | 22 |
| 22 | Property and fuzz | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d22-fuzz | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 23 |
| 23 | Linux feasibility | Claude | Codex | High | MANUAL_START | feat/m0-d23-linux | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | REQUIRED_FOR_DEVICE | NOT_STARTED | 24 |
| 24 | Windows feasibility | Claude | Codex | High | MANUAL_START | feat/m0-d24-windows | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | REQUIRED_FOR_DEVICE | NOT_STARTED | 25 |
| 25 | macOS feasibility | Claude | Codex | High | MANUAL_START | feat/m0-d25-macos | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | REQUIRED_FOR_DEVICE | NOT_STARTED | 26 |
| 26 | Performance baseline | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d26-performance | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 27 |
| 27 | Capability matrix and freeze | Codex | Claude | High | AUTO_WITH_REVIEW | feat/m0-d27-matrix | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 28 |
| 28 | Docs About and limitations | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d28-docs | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | LEGAL_DRAFT_ONLY | NOT_STARTED | 29 |
| 29 | Adversarial review | Codex | Claude | High | AUTO_WITH_REVIEW | feat/m0-d29-adversarial | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 30 |
| 30 | EAC and fsnap freeze gate | Codex | Human | Release-critical | HUMAN_APPROVAL | feat/m0-d30-gate | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | REQUIRED | NOT_STARTED | M1 |

## Milestone gates

- **Week 1:** deterministic state/audit foundations, clean integration, zero unresolved P0/P1.
- **Week 2:** byte-accurate synthetic acquisition, safe package, independent verifier, `.fsnap` v0.1 draft.
- **Week 3:** core-owned UI state, guidance, report, three-OS hosted CI, DevSecOps, fuzz/property baseline.
- **Day 30:** `.fsnap` v0.1 golden compatibility fixtures, verifier independence, limitation matrix, performance baseline, complete evidence index, human classification approval.

Day 30 hanya boleh menjadi `Production-Directed Engineering Alpha Candidate — Lab Use Only`. Semua capability tanpa exact hardware evidence tetap `NotValidated`.
