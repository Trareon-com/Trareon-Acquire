# Recovery Prompt Index

Gunakan incident category utama dari `TaskResult.v1`. Jangan menjalankan lebih dari satu recovery prompt sekaligus.

| Incident | Prompt | Required input | Allowed outcome |
|---|---|---|---|
| `IMPLEMENTATION-FAILURE` | `IMPLEMENTATION-FAILURE.md` | Failure, diff, tests, SHA | `FIX` atau `BLOCKED` |
| `TEST-INFRA-FAILURE` | `TEST-INFRA-FAILURE.md` | Runner/tool/fixture evidence | `BLOCKED` sampai tepercaya |
| `PLATFORM-DIVERGENCE` | `PLATFORM-DIVERGENCE.md` | Exact environment reports | `BLOCKED`, capability `NotValidated` |
| `FLAKY-OR-NONDETERMINISTIC` | `FLAKY-OR-NONDETERMINISTIC.md` | Repeated runs, seed/time/env | `BLOCKED` sampai stabil |
| `PERFORMANCE-REGRESSION` | `PERFORMANCE-REGRESSION.md` | Baseline/current benchmark | `FIX` atau `BLOCKED` |
| `SECURITY-FINDING` | `SECURITY-FINDING.md` | Redacted finding/reproducer | `FIX` atau human gate |
| `SPEC-AMBIGUITY` | `SPEC-AMBIGUITY.md` | Exact conflicting text/evidence | Decision request |
| `SCOPE-DRIFT` | `SCOPE-DRIFT.md` | Expected map dan actual diff | `FIX` atau human gate |
| `AGENT-DISAGREEMENT` | `AGENT-DISAGREEMENT.md` | Claims A/B dan neutral test | Decision request |
| `REMOTE-STATE-DIVERGENCE` | `REMOTE-STATE-DIVERGENCE.md` | Local/remote/PR/CI SHA | `BLOCKED` sampai identik |
| `ENVIRONMENT-BLOCKED` | `ENVIRONMENT-BLOCKED.md` | Exact environment error | `BLOCKED` |
| Protected boundary | `HUMAN-APPROVAL-REQUIRED.md` | Exact action/target/risk | Human decision only |

Recovery count 0 → satu repair diperbolehkan. Count 1 → repair terakhir. Count 2 → `BLOCKED`; jangan retry otomatis. Original author memperbaiki dan independent reviewer menilai commit baru.
