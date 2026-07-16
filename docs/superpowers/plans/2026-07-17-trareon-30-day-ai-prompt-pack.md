# Trareon Acquire 30-Day AI Prompt Pack Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a complete, validated, copy-paste prompt pack that guides Claude Code, local-only Codex, Antigravity, and the human operator through thirty production-directed work packets and the M1-M4 rolling-wave handoff.

**Architecture:** The pack uses one self-contained runbook per day, one canonical result contract, one master checklist, exact-SHA GitHub evidence handoff, and incident-specific recovery prompts. Claude Code is the default implementer/GitHub gateway, Codex is the local-only orchestrator and independent reviewer, and Antigravity performs assigned UI/exploratory validation or an explicit no-run action. A shell validator treats missing days, missing sections, placeholders, unsafe authority, or incomplete incident routing as build failures.

**Tech Stack:** Markdown, POSIX shell, `rg`, `find`, `awk`, `shasum`, Git worktrees, Claude Code, Codex, Antigravity, GitHub Actions evidence.

## Global Constraints

- Project root is `/Users/user/Projects/Trareon/Trareon Acquire`.
- Source specification is `docs/superpowers/specs/2026-07-17-trareon-ai-prompt-operations-design.md`.
- Product authority is `RFC-Digital-Forensic-Acquisition.md`; this plan must not modify the frozen RFC.
- Month One ends at Production-Directed Engineering Alpha Candidate plus `.fsnap` v0.1 Analysis Contract Freeze, not production.
- Windows narrow storage acquisition is the first production path; Linux and macOS remain explicit `NotValidated` capabilities until their gates pass.
- Every day is sequential and self-contained; `Day` means work packet, not calendar deadline.
- Codex receives no GitHub account, token, connector, or remote-operation authority.
- Claude Code is the default GitHub gateway; Antigravity is fallback only when the daily runbook names it.
- Only one GitHub gateway may operate on one task branch.
- No prompt authorizes merge, release, repository administration, secret management, billing, signing, real evidence, or destructive system-disk access.
- Raw CI artifacts live in ignored `.ai-evidence/`; committed evidence contains exact SHA, checksum, concise result, and local path only.
- At most two bounded recovery cycles are allowed per work packet.
- Performance work must preserve byte output, hash, coverage, audit order, cancellation semantics, tamper outcome, and capability claims.
- All operator prose and copy-paste prompts are written in Indonesian; code identifiers, commands, and result field names remain English.
- The local repository is initialized at baseline commit `5bfabef0c3a9aa8194130a44072dc0dfdaab7f0e`, remote `origin` names `https://github.com/Trareon-com/Trareon-Acquire.git`, and pack generation runs on branch `docs/ai-prompt-pack` in an isolated worktree.
- Day 1 verifies the existing baseline, remote URL, branch-protection intent, and GitHub monitoring setup; it does not repeat `git init`.
- GitHub is the primary monitoring plane through one Project item per Day linked to its Issue, pull request, exact-SHA checks, and redacted evidence.

## Document Interfaces

### `TaskResult.v1`

Every AI result uses these exact fields in this order:

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

Fields that do not apply contain the literal value `NOT_APPLICABLE`; unknown fields contain `UNKNOWN` and force `gate_status: UNVERIFIED`.

### `DayRunbook.v1`

Every day file uses these exact second-level headings:

```text
## Outcome
## Entry Gate
## Risk and Autonomy
## Operator Checklist
## Author Prompt
## Expected Change Map
## Expected Result
## Reviewer Prompt
## Antigravity Prompt
## Unexpected-Output Routing
## Exit Checklist
## Handoff
```

### Gate and incident values

Gate status is exactly one of `EXPECTED_PASS`, `EXPECTED_FAIL_TDD`, `FIX`, `BLOCKED`, `UNVERIFIED`, or `HUMAN_APPROVAL_REQUIRED`.

Incident category is exactly one of `NONE`, `IMPLEMENTATION-FAILURE`, `TEST-INFRA-FAILURE`, `PLATFORM-DIVERGENCE`, `FLAKY-OR-NONDETERMINISTIC`, `PERFORMANCE-REGRESSION`, `SECURITY-FINDING`, `SPEC-AMBIGUITY`, `SCOPE-DRIFT`, `AGENT-DISAGREEMENT`, `REMOTE-STATE-DIVERGENCE`, or `ENVIRONMENT-BLOCKED`.

---

### Task 1: Validation Harness and Operations Skeleton

**Files:**
- Modify: `.gitignore`
- Create: `scripts/validate-ai-operations.sh`
- Create: `docs/ai-operations/README.md`
- Create: `docs/ai-operations/MONTH-01/README.md`

**Interfaces:**
- Consumes: `DayRunbook.v1`, `TaskResult.v1`, gate values, incident values, and the source specification.
- Produces: executable validation command `sh scripts/validate-ai-operations.sh` and the canonical operations directory.

- [ ] **Step 1: Create the directory boundary and local evidence exclusion**

Create `docs/ai-operations/MONTH-01`, `docs/ai-operations/PHASE-MAPS`, `docs/ai-operations/RECOVERY-PROMPTS`, `docs/ai-operations/TEMPLATES`, and `scripts`. Preserve `.gitignore` with exactly:

```gitignore
.DS_Store
.ai-evidence/
.worktrees/
target/
node_modules/
dist/
```

- [ ] **Step 2: Write the validation harness**

Create `scripts/validate-ai-operations.sh` with this complete behavior:

```sh
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
```

- [ ] **Step 3: Write the two scope READMEs**

`docs/ai-operations/README.md` must state the authority order, local-only Codex boundary, exact-SHA handoff, sequential-day rule, and the command `sh scripts/validate-ai-operations.sh`. `docs/ai-operations/MONTH-01/README.md` must state that Day 30 is lab-only Production-Directed EAC plus `.fsnap` v0.1 freeze and list all four weekly outcomes.

- [ ] **Step 4: Run the validator and confirm the intended initial failure**

Run:

```bash
sh scripts/validate-ai-operations.sh
```

Expected: non-zero exit with `missing:` for `docs/ai-operations/START-HERE.md`. This proves the validator rejects an incomplete pack.

- [ ] **Step 5: Record the task result**

Record `EXPECTED_FAIL_TDD` with incident `NONE`, list the four created files, and commit them on `docs/ai-prompt-pack` with message `docs(ai-ops): add validation harness`.

### Task 2: Control Plane, Result Contract, and Evidence Templates

**Files:**
- Create: `docs/ai-operations/CONTROL-PLANE-PROMPT.md`
- Create: `docs/ai-operations/RESULT-CONTRACT.md`
- Create: `docs/ai-operations/EVIDENCE-INDEX.md`
- Create: `docs/ai-operations/TEMPLATES/TASK-RESULT.md`
- Create: `docs/ai-operations/TEMPLATES/DECISION-REQUEST.md`
- Create: `docs/ai-operations/TEMPLATES/PLATFORM-TEST-REPORT.md`
- Create: `docs/ai-operations/TEMPLATES/MILESTONE-REVIEW.md`

**Interfaces:**
- Consumes: `TaskResult.v1`, approved autonomy levels, GitHub access boundary, and exact-SHA evidence sequence.
- Produces: canonical result envelope and reusable control, decision, platform, and milestone forms used by every day.

- [ ] **Step 1: Write the local-only Codex control prompt**

The prompt must explicitly tell Codex to read the RFC, roadmap, current day, frozen local commit, and local evidence; prohibit GitHub operations; classify results using the exact gate and incident enums; reject success without command/exit evidence; preserve user changes; and output `TaskResult.v1`. It must stop on raw device, privilege, real evidence, secrets, signing, release, scope change, or local/remote/CI SHA mismatch.

- [ ] **Step 2: Write the canonical result contract and task-result template**

Copy all `TaskResult.v1` fields from this plan into both files. Document `NOT_APPLICABLE`, `UNKNOWN`, the forced `UNVERIFIED` rule, maximum two recovery cycles, and the distinction between gate status and incident category.

- [ ] **Step 3: Write remote and platform evidence templates**

`EVIDENCE-INDEX.md` uses columns `Day`, `Task`, `Local SHA`, `Remote SHA`, `PR SHA`, `CI SHA`, `Run`, `Artifact SHA-256`, `Local Path`, `Result`, and `Reviewed By`. `PLATFORM-TEST-REPORT.md` records exact OS build, architecture, CPU, controller, enclosure, media identity, privilege state, command, observed behavior, limitation, and frozen SHA.

- [ ] **Step 4: Write decision and milestone templates**

`DECISION-REQUEST.md` must contain the conflicting evidence, affected RFC section, safe default, options, validity/performance/schedule impact, and exact human authority requested. `MILESTONE-REVIEW.md` must prohibit production classification while any required capability is `NotValidated` or any P0/P1 finding remains open.

- [ ] **Step 5: Run focused contract checks**

Run:

```bash
rg -n 'schema: TaskResult.v1|commit_before:|ci_sha:|human_approval_required:' docs/ai-operations/CONTROL-PLANE-PROMPT.md docs/ai-operations/RESULT-CONTRACT.md docs/ai-operations/TEMPLATES/TASK-RESULT.md
rg -n 'Codex.*local|GitHub|force|merge|release|signing|barang bukti' docs/ai-operations/CONTROL-PLANE-PROMPT.md
```

Expected: all result fields are present in the three contract files and the control prompt contains the required access/stop boundaries.

### Task 3: Operator Entry Point and Master Checklist

**Files:**
- Create: `docs/ai-operations/START-HERE.md`
- Create: `docs/ai-operations/MASTER-CHECKLIST.md`
- Create: `docs/ai-operations/GITHUB-MONITORING.md`
- Create: `.github/ISSUE_TEMPLATE/daily-task.yml`
- Create: `.github/ISSUE_TEMPLATE/config.yml`
- Create: `.github/PULL_REQUEST_TEMPLATE.md`

**Interfaces:**
- Consumes: daily sequence, roles, autonomy levels, evidence index, and `TaskResult.v1`.
- Produces: the operator's only required entry point, a thirty-row monitoring dashboard, and GitHub-native Issue/PR/Project conventions.

- [ ] **Step 1: Write the five-action operator workflow**

`START-HERE.md` must direct the user to open the current unchecked day, verify entry gates, paste prompts only in the named order, store the returned result, select review/recovery based on classification, and advance only after the exit gate. It must say never paste all three prompts simultaneously and never interpret elapsed time as success.

- [ ] **Step 2: Write environment-specific launch guidance**

Include Mac control-plane paths, separate worktree naming, Windows/Kali trusted-commit pull rules, Codex local-only behavior, Claude GitHub gateway behavior, Antigravity fallback behavior, and the rule that platform devices return evidence rather than editing the author's worktree.

- [ ] **Step 3: Create the thirty-row dashboard**

Create exactly one row for Day 01 through Day 30 with columns `Day`, `Task`, `Author`, `Reviewer`, `Risk`, `Autonomy`, `Branch`, `Frozen SHA`, `Implementation`, `Review`, `CI`, `Platform`, `Incident`, `Recovery`, `Human Gate`, `Evidence`, and `Next`. Initialize status cells to `NOT_STARTED`, not blank values.

- [ ] **Step 4: Add milestone rows**

Add Week 1, Week 2, Week 3, and Day 30 milestone gates. Day 30 must require `.fsnap` v0.1 compatibility fixtures, verifier independence, limitation matrix, performance baseline, and human classification approval.

- [ ] **Step 5: Add and verify the GitHub monitoring contract**

`GITHUB-MONITORING.md` defines the Project statuses `Backlog`, `Ready`, `Claude Implementing`, `Codex Reviewing`, `Antigravity Validating`, `CI Running`, `Hardware Validation`, `Human Approval`, and `Done`; the fields Day, Task ID, Milestone, Risk, Autonomy, Author, Reviewer, Frozen SHA, CI, Hardware, Incident, Human Gate, and Evidence URL; and the rule that only redacted/non-sensitive evidence is uploaded. The Issue form captures the same task identity and acceptance gates. The PR template captures exact local/remote/PR/CI SHA, tests, Codex relay, Antigravity evidence, capabilities not validated, and human gate. Neither template authorizes merge.

Run:

Run:

```bash
test "$(rg -c '^\| [0-9]{2} \|' docs/ai-operations/MASTER-CHECKLIST.md)" -eq 30
rg -n 'jangan.*bersamaan|exact.*SHA|NotValidated|Production-Directed|fsnap.*v0.1' docs/ai-operations/START-HERE.md docs/ai-operations/MASTER-CHECKLIST.md
rg -n 'Backlog|Codex Reviewing|Hardware Validation|Evidence URL|real evidence|barang bukti' docs/ai-operations/GITHUB-MONITORING.md .github/ISSUE_TEMPLATE/daily-task.yml .github/PULL_REQUEST_TEMPLATE.md
```

Expected: the first command exits zero and the second shows each mandatory operator safeguard.

### Task 4: Week 1 Runbooks — Repository and Evidence Semantics

**Files:**
- Create: `docs/ai-operations/MONTH-01/DAY-01.md`
- Create: `docs/ai-operations/MONTH-01/DAY-02.md`
- Create: `docs/ai-operations/MONTH-01/DAY-03.md`
- Create: `docs/ai-operations/MONTH-01/DAY-04.md`
- Create: `docs/ai-operations/MONTH-01/DAY-05.md`
- Create: `docs/ai-operations/MONTH-01/DAY-06.md`
- Create: `docs/ai-operations/MONTH-01/DAY-07.md`

**Interfaces:**
- Consumes: foundation implementation plan Tasks 1-3, `DayRunbook.v1`, `TaskResult.v1`, and control-plane boundaries.
- Produces: sequential prompts for Git/toolchain baseline, workspace boundaries, domain types, state transitions, audit chain, canonical hashing, and week-one integration.

- [ ] **Step 1: Write Days 01-02**

Day 01 verifies baseline commit `5bfabef0c3a9aa8194130a44072dc0dfdaab7f0e`, the `origin` URL, branch/worktree policy, GitHub Project/Issue/PR monitoring setup, pinned toolchains, and initial CI skeleton without dependency upgrades. Day 02 creates Rust core/verifier/app boundaries and a failing public-dependency test. Both use Claude Code as author and Codex as local reviewer; Antigravity returns `NO_RUN_NOT_VISUAL` with `TaskResult.v1`.

- [ ] **Step 2: Write Days 03-04**

Day 03 defines acquisition IDs, capability states, error/result enums, and serialization tests. Day 04 defines allowed state transitions and tests every invalid transition, cancel, failure, incomplete, and forbidden false-complete path. Author prompts require TDD failure evidence before implementation.

- [ ] **Step 3: Write Days 05-06**

Day 05 creates append-only audit events and hash-chain tamper tests. Day 06 defines deterministic canonical serialization and hashing with repeated-run equality, changed-field inequality, order stability, and unsupported-version failure. Performance changes are prohibited in these tasks.

- [ ] **Step 4: Write Day 07**

Day 07 runs clean workspace tests, formatting, dependency boundary inspection, two repeated fixture runs, diff-size review, and a Week 1 discrepancy register. It cannot advance with failing tests, non-deterministic output, missing commit identity, or an unresolved P0/P1 review finding.

- [ ] **Step 5: Validate all Week 1 runbooks**

Run:

```bash
for day in 01 02 03 04 05 06 07; do
  file="docs/ai-operations/MONTH-01/DAY-$day.md"
  test "$(rg -c '^## ' "$file")" -eq 12
  rg -q 'schema: TaskResult.v1' "$file"
done
```

Expected: exit zero with no output.

### Task 5: Week 2 Runbooks — Acquisition, Package, and Independent Verifier

**Files:**
- Create: `docs/ai-operations/MONTH-01/DAY-08.md`
- Create: `docs/ai-operations/MONTH-01/DAY-09.md`
- Create: `docs/ai-operations/MONTH-01/DAY-10.md`
- Create: `docs/ai-operations/MONTH-01/DAY-11.md`
- Create: `docs/ai-operations/MONTH-01/DAY-12.md`
- Create: `docs/ai-operations/MONTH-01/DAY-13.md`
- Create: `docs/ai-operations/MONTH-01/DAY-14.md`
- Create: `docs/ai-operations/MONTH-01/DAY-15.md`

**Interfaces:**
- Consumes: Week 1 domain/audit contracts and foundation implementation plan Tasks 4-6.
- Produces: synthetic acquisition, identity guard, failure semantics, RAW/split output, `.fsnap` package, independent verifier, CLI, and v0.1 draft contract.

- [ ] **Step 1: Write Days 08-09**

Day 08 requires byte-for-byte synthetic streaming and SHA-256 oracle tests. Day 09 requires stable file-backed identity, source/destination reversal rejection, alias/symlink containment, and no output creation after failed preflight.

- [ ] **Step 2: Write Days 10-11**

Day 10 covers cancel, read failure, destination-full simulation, interrupted output, and the rule that incomplete artifacts never become verified complete. Day 11 covers RAW/split-RAW boundary sizes, segment ordering, zero-length and final-short-segment behavior, and reassembly equality.

- [ ] **Step 3: Write Days 12-13**

Day 12 defines minimal `.fsnap` manifest fields, canonical paths, schema version, coverage, audit reference, and validation errors. Day 13 implements safe package writing with path traversal, duplicate path, missing file, mutated file, and audit-discontinuity fixtures.

- [ ] **Step 4: Write Days 14-15 with role swap**

Codex authors the verifier locally and Claude Code independently reviews it. The verifier cannot depend on the writer crate or Tauri. Day 15 adds CLI round trips and freezes a draft `.fsnap` v0.1 read contract with valid, mutated, truncated, removed-file, discontinuous-audit, and unsupported-version golden packages.

- [ ] **Step 5: Validate Week 2 independence and contract clauses**

Run:

```bash
for day in 08 09 10 11 12 13 14 15; do test "$(rg -c '^## ' "docs/ai-operations/MONTH-01/DAY-$day.md")" -eq 12; done
rg -n 'tidak.*bergantung.*writer|independent|fsnap.*v0.1|unsupported.version|truncated|audit' docs/ai-operations/MONTH-01/DAY-14.md docs/ai-operations/MONTH-01/DAY-15.md
```

Expected: all day files have twelve sections and the verifier/contract safeguards are visible.

### Task 6: Week 3 Runbooks — Desktop, DevSecOps, and Guidance

**Files:**
- Create: `docs/ai-operations/MONTH-01/DAY-16.md`
- Create: `docs/ai-operations/MONTH-01/DAY-17.md`
- Create: `docs/ai-operations/MONTH-01/DAY-18.md`
- Create: `docs/ai-operations/MONTH-01/DAY-19.md`
- Create: `docs/ai-operations/MONTH-01/DAY-20.md`
- Create: `docs/ai-operations/MONTH-01/DAY-21.md`
- Create: `docs/ai-operations/MONTH-01/DAY-22.md`

**Interfaces:**
- Consumes: shared-core typed API, `.fsnap` draft, independent verifier, AI workflow, and DevSecOps RFC requirements.
- Produces: Tauri boundary, guided UI, help/accessibility, report preview, CI matrix, supply-chain gates, and fuzz/property targets.

- [ ] **Step 1: Write Days 16-17**

Day 16 permits only typed Tauri commands and core-owned final state. Day 17 creates a synthetic guided workflow; Antigravity checks start-to-result flow, state/error copy, keyboard path, viewport behavior, and screenshots against the same frozen SHA.

- [ ] **Step 2: Write Days 18-19**

Day 18 adds progressive guidance, accessibility semantics, destructive-action wording, and `Lab Use Only` status. Day 19 renders Chain of Custody and report previews only from core data and must distinguish verified, incomplete, failed, and unvalidated capabilities.

- [ ] **Step 3: Write Days 20-21**

Day 20 creates pinned CI jobs for Rust, TypeScript, fixtures, and three-OS compile checks without treating hosted CI as hardware validation. Day 21 adds secret, dependency, license, SAST, SBOM, provenance-intent, and artifact checks; a new high-risk dependency triggers `HUMAN_APPROVAL_REQUIRED`.

- [ ] **Step 4: Write Day 22**

Day 22 adds bounded property/fuzz targets for state transitions, canonical paths, manifest parsing, package containment, audit continuity, and verifier input. Corpus contents are synthetic and crashes must preserve the exact reproducer.

- [ ] **Step 5: Validate Week 3 UI/security clauses**

Run:

```bash
for day in 16 17 18 19 20 21 22; do test "$(rg -c '^## ' "docs/ai-operations/MONTH-01/DAY-$day.md")" -eq 12; done
rg -n 'Lab Use Only|core.*source|Antigravity|SBOM|SAST|hosted CI.*bukan|synthetic' docs/ai-operations/MONTH-01/DAY-{16,17,18,19,20,21,22}.md
```

Expected: all files have twelve sections and the required trust, UI, and DevSecOps phrases are present.

### Task 7: Week 4 Runbooks — Platform Feasibility, Performance, and Alpha Gate

**Files:**
- Create: `docs/ai-operations/MONTH-01/DAY-23.md`
- Create: `docs/ai-operations/MONTH-01/DAY-24.md`
- Create: `docs/ai-operations/MONTH-01/DAY-25.md`
- Create: `docs/ai-operations/MONTH-01/DAY-26.md`
- Create: `docs/ai-operations/MONTH-01/DAY-27.md`
- Create: `docs/ai-operations/MONTH-01/DAY-28.md`
- Create: `docs/ai-operations/MONTH-01/DAY-29.md`
- Create: `docs/ai-operations/MONTH-01/DAY-30.md`

**Interfaces:**
- Consumes: trusted frozen commits, platform-test template, performance equivalence rules, `.fsnap` draft, and all previous evidence.
- Produces: three read-only feasibility reports, performance baseline, discrepancy matrix, v0.1 freeze candidate, documentation/About package, adversarial review, and Day 30 classification.

- [ ] **Step 1: Write Days 23-25**

Create Linux, Windows, and macOS read-only feasibility prompts. Each begins at `MANUAL_START`, records exact OS/hardware/security state, prohibits system-disk writes, uses virtual or allowlisted lab sources, and classifies unavailable or differing behavior as `PLATFORM-DIVERGENCE`, never success by assumption.

- [ ] **Step 2: Write Days 26-27**

Day 26 measures throughput, peak memory, cancellation latency, hashing overhead, package overhead, and repeated-run variance on synthetic sizes. Day 27 compares exact platform evidence, keeps unsupported combinations `NotValidated`, and produces the `.fsnap` v0.1 compatibility freeze candidate.

- [ ] **Step 3: Write Day 28**

Create prompts for user-guide skeleton, in-app help mapping, About/author attribution, legal limitation draft, supported/not-validated matrix, and operator warnings. Antigravity runs a comprehension and navigation check. Legal text remains a draft and cannot claim certification.

- [ ] **Step 4: Write Days 29-30**

Day 29 freezes a commit and performs adversarial review of tamper, false completion, path, privilege, dependency, nondeterminism, and performance-equivalence boundaries. Day 30 requires complete evidence index, zero open P0/P1, known-risk register, `.fsnap` v0.1 golden packages, verifier compatibility, human approval, and the explicit classification `Production-Directed Engineering Alpha Candidate — Lab Use Only`.

- [ ] **Step 5: Validate all thirty day files**

Run:

```bash
sh scripts/validate-ai-operations.sh
```

Expected at this stage: non-zero exit naming the first missing recovery prompt or phase map, not a missing day or day-section error.

### Task 8: Recovery Prompt Library

**Files:**
- Create: `docs/ai-operations/RECOVERY-PROMPTS/INDEX.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/IMPLEMENTATION-FAILURE.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/TEST-INFRA-FAILURE.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/PLATFORM-DIVERGENCE.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/FLAKY-OR-NONDETERMINISTIC.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/PERFORMANCE-REGRESSION.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/SECURITY-FINDING.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/SPEC-AMBIGUITY.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/SCOPE-DRIFT.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/AGENT-DISAGREEMENT.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/REMOTE-STATE-DIVERGENCE.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/ENVIRONMENT-BLOCKED.md`
- Create: `docs/ai-operations/RECOVERY-PROMPTS/HUMAN-APPROVAL-REQUIRED.md`

**Interfaces:**
- Consumes: exact gate/incident taxonomy, maximum two repair cycles, original-author rule, and protected authority boundaries.
- Produces: deterministic next prompts for every unexpected-output route.

- [ ] **Step 1: Write the recovery index and common contract**

The index maps each incident to its exact file, required inputs, allowed diagnosis, prohibited actions, and exit states. Every recovery prompt requires the original task/day, frozen SHA, exact error/log, prior recovery count, diff, and `TaskResult.v1` output.

- [ ] **Step 2: Write implementation, test, platform, determinism, and performance prompts**

Create `IMPLEMENTATION-FAILURE.md`, `TEST-INFRA-FAILURE.md`, `PLATFORM-DIVERGENCE.md`, `FLAKY-OR-NONDETERMINISTIC.md`, and `PERFORMANCE-REGRESSION.md`. They must forbid assertion weakening, test skipping, silent platform fallback, validity-for-speed trades, and unbounded retries.

- [ ] **Step 3: Write security, specification, scope, and disagreement prompts**

Create `SECURITY-FINDING.md`, `SPEC-AMBIGUITY.md`, `SCOPE-DRIFT.md`, and `AGENT-DISAGREEMENT.md`. Security findings preserve evidence without exploiting real systems; ambiguity and disagreement produce `DECISION-REQUEST.md` rather than code changes.

- [ ] **Step 4: Write remote, environment, and human-gate prompts**

Create `REMOTE-STATE-DIVERGENCE.md`, `ENVIRONMENT-BLOCKED.md`, and `HUMAN-APPROVAL-REQUIRED.md`. Remote recovery compares local/remote/PR/CI SHA without force-push/rebase/merge. Human-gate output states the exact command or authority requested and performs nothing sensitive before approval.

- [ ] **Step 5: Verify complete routing**

Run:

```bash
for category in IMPLEMENTATION-FAILURE TEST-INFRA-FAILURE PLATFORM-DIVERGENCE FLAKY-OR-NONDETERMINISTIC PERFORMANCE-REGRESSION SECURITY-FINDING SPEC-AMBIGUITY SCOPE-DRIFT AGENT-DISAGREEMENT REMOTE-STATE-DIVERGENCE ENVIRONMENT-BLOCKED HUMAN-APPROVAL-REQUIRED; do
  rg -q 'schema: TaskResult.v1' "docs/ai-operations/RECOVERY-PROMPTS/$category.md"
done
```

Expected: exit zero with no output.

### Task 9: M1-M4 Phase Maps and Final Pack Review

**Files:**
- Create: `docs/ai-operations/PHASE-MAPS/M1-ENGINEERING-ALPHA.md`
- Create: `docs/ai-operations/PHASE-MAPS/M2-STORAGE-LAB-BETA.md`
- Create: `docs/ai-operations/PHASE-MAPS/M3-RELEASE-CANDIDATE.md`
- Create: `docs/ai-operations/PHASE-MAPS/M4-OFFICIAL-PRODUCTION.md`
- Modify: `docs/ai-operations/README.md`

**Interfaces:**
- Consumes: Day 30 evidence contract, production schedule range, first-production scope, analysis contract, and RFC milestone gates.
- Produces: full-lifecycle navigation without speculative implementation prompts and a completely validated Month One pack.

- [ ] **Step 1: Write M1 map**

Define entry evidence from Day 30, file-backed/recovery/fuzz/reproducibility outcomes, `.fsnap` v0.1 compatibility obligations, parallel Analysis importer boundaries, exit evidence, anticipated incidents, and the trigger for generating the next rolling-wave prompt pack.

- [ ] **Step 2: Write M2 map**

Define Linux reference engineering, Windows production validation priority, macOS follow-up, allowlisted hardware matrices, bad-sector/disconnect/destination-full/resume requirements, source-unchanged proof, performance ceilings, and capability-specific exit gates.

- [ ] **Step 3: Write M3 and M4 maps**

M3 covers feature freeze, reproducibility, SBOM/provenance, accessibility, docs, independent review, signing availability, and validation reports. M4 permits only evidence-backed capability claims, exact official artifacts, support boundaries, two-person sign-off, and human-owned publish/legal/payment actions.

- [ ] **Step 4: Run full validation and RFC checksum**

Run:

```bash
sh scripts/validate-ai-operations.sh
shasum -a 256 -c docs/RFC-BASELINE.sha256
```

Expected:

```text
ai_operations_validation:PASS
RFC-Digital-Forensic-Acquisition.md: OK
```

- [ ] **Step 5: Perform manual spec-coverage review**

Inspect the final pack and record evidence that all thirty days, twelve runbook sections, three AI roles, local-only Codex, one-gateway rule, exact-SHA handoff, GitHub-only monitoring visibility, redacted hardware evidence, all incident routes, two-cycle recovery limit, performance-equivalence gate, `.fsnap` Analysis contract, Windows-first production path, M1-M4 maps, and protected human actions are covered. Commit the complete pack on `docs/ai-prompt-pack`; do not push or merge from Codex.
