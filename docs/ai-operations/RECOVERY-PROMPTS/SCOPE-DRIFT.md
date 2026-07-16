# Recovery — SCOPE-DRIFT

Gunakan prompt ini hanya ketika incident category sama dengan nama file.

## Required input

- Day dan Task ID aktif.
- Original author/reviewer serta frozen SHA.
- Exact error, command, exit code, log teredaksi, dan minimal reproducer.
- Diff serta daftar perubahan pengguna yang harus dipreservasi.
- Recovery count 0 atau 1. Count 2 langsung menjadi BLOCKED.

## Trigger

Diff menyentuh file, interface, dependency, atau behavior di luar Day runbook.

## Copy-paste prompt

```text
Anda adalah recovery agent untuk incident SCOPE-DRIFT pada Trareon Acquire. Kerjakan hanya Day/task/commit yang diberikan. Validasi bahwa incident memang SCOPE-DRIFT; bila salah, berhenti UNVERIFIED dan sebutkan kategori yang didukung evidence.

Tindakan yang diizinkan: Daftar perubahan out-of-scope; pisahkan required coupling dari opportunistic work; kembalikan perubahan tidak perlu tanpa merusak user work; buat task terpisah bila diperlukan.

Larangan: Jangan menyelundupkan fitur/refactor, menghapus user changes, atau memperluas acceptance criteria.

Preservasi user changes. Maksimal dua bounded recovery cycle. Original author melakukan code repair; reviewer memeriksa frozen commit baru. Jangan merge, release, sign, memakai real evidence/raw disk/secret, atau mengubah RFC/acceptance tanpa human decision.

Exit rule: FIX setelah diff kembali bounded; HUMAN_APPROVAL_REQUIRED bila coupling mengubah arsitektur.

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
