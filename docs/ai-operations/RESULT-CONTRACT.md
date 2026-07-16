# Result Contract

Semua AI harus mengakhiri hasil dengan schema berikut dan urutan field yang sama.

```text
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

Gunakan `NOT_APPLICABLE` untuk field yang tidak berlaku. Gunakan `UNKNOWN` bila nilainya belum dibuktikan; adanya `UNKNOWN` memaksa `gate_status: UNVERIFIED`.

Gate status: `EXPECTED_PASS`, `EXPECTED_FAIL_TDD`, `FIX`, `BLOCKED`, `UNVERIFIED`, atau `HUMAN_APPROVAL_REQUIRED`.

Incident: `NONE`, `IMPLEMENTATION-FAILURE`, `TEST-INFRA-FAILURE`, `PLATFORM-DIVERGENCE`, `FLAKY-OR-NONDETERMINISTIC`, `PERFORMANCE-REGRESSION`, `SECURITY-FINDING`, `SPEC-AMBIGUITY`, `SCOPE-DRIFT`, `AGENT-DISAGREEMENT`, `REMOTE-STATE-DIVERGENCE`, atau `ENVIRONMENT-BLOCKED`.

Maksimal dua recovery cycle untuk satu Day. Setelah dua kegagalan, status menjadi `BLOCKED`. Gate status menjawab boleh/tidaknya maju; incident menjelaskan penyebabnya.
