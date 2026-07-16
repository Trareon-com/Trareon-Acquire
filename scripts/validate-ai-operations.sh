#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
OPS="$ROOT/docs/ai-operations"
SPEC="$ROOT/docs/superpowers/specs/2026-07-17-trareon-ai-prompt-operations-design.md"

required_root="START-HERE.md MASTER-CHECKLIST.md CONTROL-PLANE-PROMPT.md RESULT-CONTRACT.md EVIDENCE-INDEX.md GITHUB-MONITORING.md README.md"
for file in $required_root; do
  test -s "$OPS/$file" || { echo "missing:$OPS/$file"; exit 1; }
done

test -s "$SPEC" || { echo "missing:$SPEC"; exit 1; }

headings='Outcome|Entry Gate|Risk and Autonomy|Operator Checklist|Author Prompt|Expected Change Map|Expected Result|Reviewer Prompt|Antigravity Prompt|Unexpected-Output Routing|Exit Checklist|Handoff'

day=1
while test "$day" -le 30; do
  padded=$(printf '%02d' "$day")
  file="$OPS/MONTH-01/DAY-$padded.md"
  test -s "$file" || { echo "missing:$file"; exit 1; }
  count=$(rg -c "^## ($headings)$" "$file")
  test "$count" -eq 12 || { echo "invalid_headings:$file:$count"; exit 1; }
  rg -q 'schema: TaskResult.v1' "$file" || { echo "missing_result_contract:$file"; exit 1; }
  rg -q 'HUMAN_APPROVAL_REQUIRED' "$file" || { echo "missing_human_gate:$file"; exit 1; }
  rg -q 'UNVERIFIED' "$file" || { echo "missing_unverified_gate:$file"; exit 1; }
  day=$((day + 1))
done

for category in IMPLEMENTATION-FAILURE TEST-INFRA-FAILURE PLATFORM-DIVERGENCE FLAKY-OR-NONDETERMINISTIC PERFORMANCE-REGRESSION SECURITY-FINDING SPEC-AMBIGUITY SCOPE-DRIFT AGENT-DISAGREEMENT REMOTE-STATE-DIVERGENCE ENVIRONMENT-BLOCKED HUMAN-APPROVAL-REQUIRED; do
  test -s "$OPS/RECOVERY-PROMPTS/$category.md" || { echo "missing_recovery:$category"; exit 1; }
done

for phase in M1-ENGINEERING-ALPHA M2-STORAGE-LAB-BETA M3-RELEASE-CANDIDATE M4-OFFICIAL-PRODUCTION; do
  test -s "$OPS/PHASE-MAPS/$phase.md" || { echo "missing_phase:$phase"; exit 1; }
done

if rg -n 'TODO|TBD|FIXME|XXX|fill in details|implement later|similar to Task' "$OPS"; then
  echo 'placeholder_detected'
  exit 1
fi

if rg -n '[[:blank:]]+$' "$OPS"; then
  echo 'trailing_whitespace_detected'
  exit 1
fi

if rg -n 'git push ([^`[:space:]]+ )?--force|git reset --hard|gh pr merge|tauri signer sign|barang bukti nyata dapat digunakan sebagai fixture' "$OPS"; then
  echo 'unsafe_authority_detected'
  exit 1
fi

echo 'ai_operations_validation:PASS'
