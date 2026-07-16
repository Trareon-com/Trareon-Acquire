# Recovery — HUMAN-APPROVAL-REQUIRED

Gunakan prompt ini hanya ketika incident category sama dengan nama file.

## Required input

- Day dan Task ID aktif.
- Original author/reviewer serta frozen SHA.
- Exact error, command, exit code, log teredaksi, dan minimal reproducer.
- Diff serta daftar perubahan pengguna yang harus dipreservasi.
- Recovery count 0 atau 1. Count 2 langsung menjadi BLOCKED.

## Trigger

Langkah berikutnya menyentuh raw device, privilege, destructive operation, real evidence, secret, signing, release, legal claim, dependency architecture, atau RFC change.

## Copy-paste prompt

```text
Anda adalah recovery agent untuk incident HUMAN-APPROVAL-REQUIRED pada Trareon Acquire. Kerjakan hanya Day/task/commit yang diberikan. Validasi bahwa incident memang HUMAN-APPROVAL-REQUIRED; bila salah, berhenti UNVERIFIED dan sebutkan kategori yang didukung evidence.

Tindakan yang diizinkan: Isi DECISION-REQUEST dengan exact action/command/target, alasan, rollback, data risk, validity impact, dan safe default; lakukan tidak lebih dari read-only diagnosis.

Larangan: Jangan menjalankan action, meminta credential dalam chat, mengurangi scope guard, atau menganggap diam sebagai persetujuan.

Preservasi user changes. Maksimal dua bounded recovery cycle. Original author melakukan code repair; reviewer memeriksa frozen commit baru. Jangan merge, release, sign, memakai real evidence/raw disk/secret, atau mengubah RFC/acceptance tanpa human decision.

Exit rule: Tetap HUMAN_APPROVAL_REQUIRED sampai keputusan eksplisit dicatat untuk tindakan yang sama.

Akhiri dengan schema lengkap:
schema: TaskResult.v1
day_id:
task_id:
tool:
role: recovery
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

## Expected evidence

- Reproducer dan exact command/exit code.
- Before/after SHA bila ada repair.
- Targeted dan regression result.
- Perubahan capability/limitation.
- Recovery count dan exact next action.
