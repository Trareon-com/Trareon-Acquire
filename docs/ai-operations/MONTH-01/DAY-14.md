# Day 14 — Independent verifier

## Outcome

Buat traeron-verifier tanpa dependency pada writer crate/Tauri; verifikasi schema, files, sizes, hashes, audit continuity, coverage, dan terminal result.

## Entry Gate

- Prasyarat: Day 13 dengan independent review EXPECTED_PASS.
- Project root: `/Users/user/Projects/Trareon/Trareon Acquire`.
- Branch: `feat/m0-d14-verifier`; satu author dan satu worktree aktif.
- Baca RFC, roadmap, AI workflow, hasil Day sebelumnya, serta bagian relevan `docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md`.
- Working tree harus bersih atau perubahan pengguna dicatat dan dipreservasi.

## Risk and Autonomy

- Risk: `High`.
- Autonomy: `AUTO_WITH_REVIEW`.
- Author: Codex; reviewer: Claude Code.
- Codex bekerja local-only; Claude Code menjadi relay/GitHub gateway setelah commit dibekukan.
- Raw device, privilege, real evidence, secret, signing, release, dependency architecture, destructive operation, atau perubahan RFC berhenti dengan `HUMAN_APPROVAL_REQUIRED`.

## Operator Checklist

- [ ] Pastikan Issue Day 14 berada di status yang benar dan tidak ada Day lain menulis file yang sama.
- [ ] Catat commit_before, branch, worktree, dan recovery count.
- [ ] Salin Author Prompt hanya ke Codex.
- [ ] Setelah author membekukan commit, salin Reviewer Prompt hanya ke Claude Code.
- [ ] Jalankan Antigravity Prompt setelah review kode bila ditugaskan.
- [ ] Jangan maju bila ada `UNKNOWN`, SHA mismatch, failing gate, atau approval belum diberikan.

## Author Prompt

```text
Anda adalah author untuk M0-D14 Trareon Acquire.

Outcome: Buat traeron-verifier tanpa dependency pada writer crate/Tauri; verifikasi schema, files, sizes, hashes, audit continuity, coverage, dan terminal result.
Scope file/interface: crates/traeron-verifier/Cargo.toml; crates/traeron-verifier/src/main.rs; crates/traeron-verifier/tests/cli.rs; Cargo.toml.
Branch wajib: feat/m0-d14-verifier. Baca authority files dan existing code sebelum perubahan. Gunakan TDD: tulis test yang gagal dengan alasan yang diharapkan, jalankan dan catat exit code, implementasikan perubahan minimum, lalu jalankan verification lengkap.

Verification wajib: cargo tree -p traeron-verifier; cargo test -p traeron-verifier; cargo test --workspace --all-targets.
Acceptance focus: independensi nyata, exit code berbeda untuk valid/invalid/unsupported, tidak memperbaiki package.

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

- Expected files/interfaces: crates/traeron-verifier/Cargo.toml; crates/traeron-verifier/src/main.rs; crates/traeron-verifier/tests/cli.rs; Cargo.toml.
- Expected behavior: Buat traeron-verifier tanpa dependency pada writer crate/Tauri; verifikasi schema, files, sizes, hashes, audit continuity, coverage, dan terminal result.
- Explicit non-goals: fitur setelah Day 14, production claim, raw/elevated operation tanpa gate, refactor di luar scope, dan perubahan RFC.
- Diff di luar map menjadi `FIX + SCOPE-DRIFT` sampai dijelaskan dan disetujui.

## Expected Result

- Verification: `cargo tree -p traeron-verifier; cargo test -p traeron-verifier; cargo test --workspace --all-targets`.
- Expected: command relevan exit 0 setelah TDD cycle; targeted dan regression tests lulus.
- Required evidence: failing-test proof bila berlaku, commands/exit codes, files changed, exact commit, platforms tested, NotValidated list, dan remaining risks.
- Performance-sensitive change wajib menjaga byte/hash/coverage/audit/cancel/tamper equivalence.

## Reviewer Prompt

```text
Anda adalah independent reviewer Claude Code untuk M0-D14. Jangan menulis pada worktree author. Baca RFC, active runbook, commit_before, frozen commit_after, diff, test logs, dan local evidence.

Verifikasi sendiri scope, correctness, negative paths, false-complete resistance, forensic validity, security, dependency boundary, deterministic behavior, performance equivalence, documentation, dan test oracle. Fokus khusus: independensi nyata, exit code berbeda untuk valid/invalid/unsupported, tidak memperbaiki package.

Jalankan command aman yang relevan: cargo tree -p traeron-verifier; cargo test -p traeron-verifier; cargo test --workspace --all-targets. Bandingkan exact local/remote/PR/CI SHA bila remote evidence tersedia. SHA mismatch adalah BLOCKED + REMOTE-STATE-DIVERGENCE. Hasil tanpa reproducible evidence adalah UNVERIFIED.

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
Task ini tidak memerlukan visual validation. Jangan mengedit kode atau GitHub. Periksa bahwa runbook memang tidak memiliki UI artifact baru, lalu keluarkan TaskResult.v1 dengan role Antigravity dan catatan NO_RUN_NOT_VISUAL. Bila menemukan perubahan UI di luar scope, gunakan gate_status FIX dan incident_category SCOPE-DRIFT.

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

Jika semua exit checklist lulus, handoff ke `DAY-15.md` dengan frozen SHA, TaskResult.v1 author/reviewer/Antigravity, CI/platform evidence, incident/recovery count, dan remaining risks. Selain itu tetap pada Day 14.
