# Day 17 — Guided synthetic acquisition UI

## Outcome

Buat guided workflow synthetic source: case identity, preflight, confirmation, progress, result, verifier summary, dan limitation display.

## Entry Gate

- Prasyarat: Day 16 dengan independent review EXPECTED_PASS.
- Project root: `/Users/user/Projects/Trareon/Trareon Acquire`.
- Branch: `feat/m0-d17-ui`; satu author dan satu worktree aktif.
- Baca RFC, roadmap, AI workflow, hasil Day sebelumnya, serta bagian relevan `docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md`.
- Working tree harus bersih atau perubahan pengguna dicatat dan dipreservasi.

## Risk and Autonomy

- Risk: `Medium`.
- Autonomy: `AUTO_WITH_REVIEW`.
- Author: Claude Code; reviewer: Codex.
- Claude Code boleh menjadi GitHub gateway hanya setelah local commit dibekukan dan diperiksa.
- Raw device, privilege, real evidence, secret, signing, release, dependency architecture, destructive operation, atau perubahan RFC berhenti dengan `HUMAN_APPROVAL_REQUIRED`.

## Operator Checklist

- [ ] Pastikan Issue Day 17 berada di status yang benar dan tidak ada Day lain menulis file yang sama.
- [ ] Catat commit_before, branch, worktree, dan recovery count.
- [ ] Salin Author Prompt hanya ke Claude Code.
- [ ] Setelah author membekukan commit, salin Reviewer Prompt hanya ke Codex.
- [ ] Jalankan Antigravity Prompt setelah review kode bila ditugaskan.
- [ ] Jangan maju bila ada `UNKNOWN`, SHA mismatch, failing gate, atau approval belum diberikan.

## Author Prompt

```text
Anda adalah author untuk M0-D17 Trareon Acquire.

Outcome: Buat guided workflow synthetic source: case identity, preflight, confirmation, progress, result, verifier summary, dan limitation display.
Scope file/interface: apps/traeron-acquire/src/App.svelte; apps/traeron-acquire/src/lib/api.ts; UI tests.
Branch wajib: feat/m0-d17-ui. Baca authority files dan existing code sebelum perubahan. Gunakan TDD: tulis test yang gagal dengan alasan yang diharapkan, jalankan dan catat exit code, implementasikan perubahan minimum, lalu jalankan verification lengkap.

Verification wajib: npm run check; npm run build; cargo test --workspace --all-targets.
Acceptance focus: UI tidak mengarang success dan semua state berasal dari typed core response.

Jangan memperluas scope, melemahkan assertion, skip test, menghapus perubahan pengguna, memakai real evidence, mengakses raw disk/elevated privilege, menambah dependency tanpa review, merge, release, atau signing. Simpan semua status sebagai core-owned typed result. Capability tanpa evidence adalah NotValidated.

Buat commit kecil dengan task ID setelah test lulus. Jangan menyatakan berhasil tanpa command, exit code, diff, dan exact SHA. Bila blocked atau protected boundary muncul, berhenti dan klasifikasikan; jangan mengarang workaround.

Akhiri dengan schema berikut lengkap:
schema: TaskResult.v1
day_id:
task_id:
tool:
role:
gate_status:
incident_category:
branch:
worktree:
commit_before:
commit_after:
local_sha:
remote_sha:
pull_request_sha:
ci_sha:
files_changed:
files_inspected:
commands:
tests_passed:
tests_failed:
tests_skipped:
platforms_tested:
capabilities_not_validated:
unexpected_observations:
security_validity_impact:
remaining_risks:
next_action:
human_approval_required:
```

## Expected Change Map

- Expected files/interfaces: apps/traeron-acquire/src/App.svelte; apps/traeron-acquire/src/lib/api.ts; UI tests.
- Expected behavior: Buat guided workflow synthetic source: case identity, preflight, confirmation, progress, result, verifier summary, dan limitation display.
- Explicit non-goals: fitur setelah Day 17, production claim, raw/elevated operation tanpa gate, refactor di luar scope, dan perubahan RFC.
- Diff di luar map menjadi `FIX + SCOPE-DRIFT` sampai dijelaskan dan disetujui.

## Expected Result

- Verification: `npm run check; npm run build; cargo test --workspace --all-targets`.
- Expected: command relevan exit 0 setelah TDD cycle; targeted dan regression tests lulus.
- Required evidence: failing-test proof bila berlaku, commands/exit codes, files changed, exact commit, platforms tested, NotValidated list, dan remaining risks.
- Performance-sensitive change wajib menjaga byte/hash/coverage/audit/cancel/tamper equivalence.

## Reviewer Prompt

```text
Anda adalah independent reviewer Codex untuk M0-D17. Jangan menulis pada worktree author. Baca RFC, active runbook, commit_before, frozen commit_after, diff, test logs, dan local evidence.

Verifikasi sendiri scope, correctness, negative paths, false-complete resistance, forensic validity, security, dependency boundary, deterministic behavior, performance equivalence, documentation, dan test oracle. Fokus khusus: UI tidak mengarang success dan semua state berasal dari typed core response.

Jalankan command aman yang relevan: npm run check; npm run build; cargo test --workspace --all-targets. Bandingkan exact local/remote/PR/CI SHA bila remote evidence tersedia. SHA mismatch adalah BLOCKED + REMOTE-STATE-DIVERGENCE. Hasil tanpa reproducible evidence adalah UNVERIFIED.

Jangan memperbaiki branch author, merge, release, sign, mengakses real evidence/raw disk, atau menyetujui scope expansion. Kembalikan finding ke author dan akhiri dengan TaskResult.v1 lengkap.
schema: TaskResult.v1
day_id:
task_id:
tool:
role:
gate_status:
incident_category:
branch:
worktree:
commit_before:
commit_after:
local_sha:
remote_sha:
pull_request_sha:
ci_sha:
files_changed:
files_inspected:
commands:
tests_passed:
tests_failed:
tests_skipped:
platforms_tested:
capabilities_not_validated:
unexpected_observations:
security_validity_impact:
remaining_risks:
next_action:
human_approval_required:
```

## Antigravity Prompt

```text
Validasi frozen SHA untuk mode VALIDATE_UI_FLOW. Jangan mengubah core semantics. Uji alur, keyboard/focus, error/limitation visibility, viewport, dan copy yang relevan. Simpan screenshot teredaksi tanpa data pribadi. Laporkan mismatch sebagai FIX; jangan memperbaiki branch author. Akhiri dengan TaskResult.v1.

schema: TaskResult.v1
day_id:
task_id:
tool:
role:
gate_status:
incident_category:
branch:
worktree:
commit_before:
commit_after:
local_sha:
remote_sha:
pull_request_sha:
ci_sha:
files_changed:
files_inspected:
commands:
tests_passed:
tests_failed:
tests_skipped:
platforms_tested:
capabilities_not_validated:
unexpected_observations:
security_validity_impact:
remaining_risks:
next_action:
human_approval_required:
```

## Unexpected-Output Routing

- Compile/behavior salah → `FIX + IMPLEMENTATION-FAILURE` → `RECOVERY-PROMPTS/IMPLEMENTATION-FAILURE.md`.
- Test runner/toolchain tidak tepercaya → `BLOCKED + TEST-INFRA-FAILURE`.
- Hasil antar-OS/hardware berbeda → `BLOCKED + PLATFORM-DIVERGENCE`.
- Repeated run berbeda/flaky → `BLOCKED + FLAKY-OR-NONDETERMINISTIC`.
- Performance turun tanpa validity failure → `FIX + PERFORMANCE-REGRESSION`.
- Secret/trust/privilege/dependency issue → `HUMAN_APPROVAL_REQUIRED + SECURITY-FINDING`.
- Requirement ambigu atau agent berbeda kesimpulan → `BLOCKED + SPEC-AMBIGUITY/AGENT-DISAGREEMENT`.
- Diff melebar → `FIX + SCOPE-DRIFT`.
- Local/remote/PR/CI SHA berbeda → `BLOCKED + REMOTE-STATE-DIVERGENCE`.
- Device/service/network tidak tersedia → `BLOCKED + ENVIRONMENT-BLOCKED`.
- Maksimal dua recovery cycle; setelah itu tetap `BLOCKED` dan minta keputusan manusia.

## Exit Checklist

- [ ] Author result lengkap dan tidak mengandung `UNKNOWN`.
- [ ] Independent reviewer memberi `EXPECTED_PASS`.
- [ ] Targeted/regression verification lulus dengan exit code tercatat.
- [ ] Exact SHA konsisten pada semua evidence yang berlaku.
- [ ] P0/P1 ditutup; limitation dan NotValidated dicatat.
- [ ] GitHub Issue/PR/Project diperbarui oleh gateway tanpa merge.
- [ ] Tidak ada protected action yang dilakukan tanpa approval.

## Handoff

Jika semua exit checklist lulus, handoff ke `DAY-18.md` dengan frozen SHA, TaskResult.v1 author/reviewer/Antigravity, CI/platform evidence, incident/recovery count, dan remaining risks. Selain itu tetap pada Day 17.
