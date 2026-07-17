# Trareon Acquire Code and Review Documents Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create an evidence-backed implementation document and snapshot review for the current Trareon Acquire M0 repository.

**Architecture:** Add two project-level Markdown documents at the repository root. The Code document maps approved specifications to concrete components and tests; the Review document evaluates the fixed product-code snapshot `5ed92860132f471a2dff3495fde27bbf1ae8616f` and separates local verification, historical hosted-CI evidence, and unvalidated hardware capability.

**Tech Stack:** Markdown, Rust 1.95 workspace, Cargo, Node.js 22.22.3, npm 10.9.8, Tauri 2, Svelte, Git.

## Global Constraints

- Create `Code-Digital-Forensic-Acquisition.md` and `Review-Digital-Forensic-Acquisition.md` at the repository root.
- Treat `RFC-Digital-Forensic-Acquisition.md` as the highest-priority product-development source, followed by the PRD, task criteria, code, verification evidence, and historical session logs.
- Use repository-relative Markdown links.
- Separate implemented behavior from planned behavior.
- Mark insufficient platform or hardware evidence as `NotValidated`.
- Hosted CI is portability evidence, not proof of raw-device or destructive-hardware behavior.
- Do not claim production readiness or full forensic-standards compliance.
- Do not modify the reusable templates under `Template/`.
- Do not modify product code while producing these documents.
- Keep unrelated DevSecOps changes and `.gemini/`/`GEMINI.md` outside all commits.

## File Structure

- Create: `Code-Digital-Forensic-Acquisition.md` — authoritative implementation map for the current M0 product snapshot.
- Create: `Review-Digital-Forensic-Acquisition.md` — evidence-backed review of product commit `5ed92860132f471a2dff3495fde27bbf1ae8616f`.
- Create: `docs/ai-session-log/2026-07-17-codex-code-review-documents.md` — factual handoff required by the repository session workflow.
- Do not modify: `PRD-Digital-Forensic-Acquisition.md`, `RFC-Digital-Forensic-Acquisition.md`, `Template/**`, `crates/**`, `apps/**`, `schemas/**`, or `fixtures/**`.

---

### Task 1: Write the Project Code Document

**Files:**
- Create: `Code-Digital-Forensic-Acquisition.md`
- Read: `PRD-Digital-Forensic-Acquisition.md`
- Read: `RFC-Digital-Forensic-Acquisition.md`
- Read: `docs/IMPLEMENTATION-ROADMAP.md`
- Read: `docs/AI-DEVELOPMENT-WORKFLOW.md`
- Read: `docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md`
- Read: `docs/WEEK-01-DISCREPANCY-REGISTER.md`
- Read: `docs/fsnap-v0.1-read-contract.md`
- Read: `crates/trareon-core/src/*.rs`
- Read: `crates/trareon-core/tests/*.rs`
- Read: `crates/trareon-verifier/src/main.rs`
- Read: `crates/trareon-verifier/tests/*.rs`
- Read: `apps/trareon-acquire/src-tauri/src/lib.rs`
- Read: `apps/trareon-acquire/src-tauri/tests/ipc.rs`
- Read: `apps/trareon-acquire/src/App.svelte`
- Read: `.github/workflows/ci.yml`

**Interfaces:**
- Consumes: Approved specification hierarchy, product snapshot `5ed92860132f471a2dff3495fde27bbf1ae8616f`, committed tests, schemas, fixtures, and CI configuration.
- Produces: A stable implementation map that the Review document can cite when assessing scope and evidence.

- [ ] **Step 1: Capture the exact documentation and product snapshots**

Run:

```bash
git rev-parse HEAD
git show -s --format='%H %s' 5ed92860132f471a2dff3495fde27bbf1ae8616f
git status --short
```

Expected:

- `HEAD` contains the approved design and this plan on `docs/code-review-documents`.
- Product snapshot resolves to `5ed92860132f471a2dff3495fde27bbf1ae8616f docs: record session log for traeron -> trareon rename (PR #35)`.
- The isolated worktree has no unrelated staged or untracked files.

- [ ] **Step 2: Verify every implementation claim against source and tests**

Run:

```bash
rg -n "pub (struct|enum|fn)|pub const fn|#\[test\]" \
  crates/trareon-core/src \
  crates/trareon-core/tests \
  crates/trareon-verifier/src \
  crates/trareon-verifier/tests \
  apps/trareon-acquire/src-tauri/src \
  apps/trareon-acquire/src-tauri/tests
```

Expected evidence map:

| Capability | Implementation | Primary evidence |
|---|---|---|
| Stable build identity | `crates/trareon-core/src/lib.rs` | `crates/trareon-core/tests/foundation.rs` |
| Acquisition state invariants | `crates/trareon-core/src/domain.rs` | `crates/trareon-core/tests/domain.rs` |
| Hash-chained audit journal | `crates/trareon-core/src/audit.rs` | `crates/trareon-core/tests/audit.rs` |
| File-backed streaming acquisition | `crates/trareon-core/src/acquisition.rs` | `crates/trareon-core/tests/acquisition.rs` |
| Split-RAW generation | `crates/trareon-core/src/acquisition.rs` | split-segment tests in `crates/trareon-core/tests/acquisition.rs` |
| `.fsnap` creation and verification | `crates/trareon-core/src/package.rs` | `crates/trareon-core/tests/package.rs` |
| Independent verifier CLI | `crates/trareon-verifier/src/main.rs` | `crates/trareon-verifier/tests/cli.rs` and `fixtures/fsnap-v0.1/` |
| Tauri IPC foundation path | `apps/trareon-acquire/src-tauri/src/lib.rs` | `apps/trareon-acquire/src-tauri/tests/ipc.rs` |
| Desktop UI shell | `apps/trareon-acquire/src/App.svelte` | frontend build command; no formal browser accessibility audit |
| Three-OS hosted checks | `.github/workflows/ci.yml` | historical commit-specific CI evidence only |

- [ ] **Step 3: Create the Code document with the complete approved structure**

Create `Code-Digital-Forensic-Acquisition.md` with these exact top-level sections:

```markdown
# Code — Trareon Acquire Digital Forensic Acquisition

## 1. Document Identity and Snapshot
## 2. Source Documents and Precedence
## 3. Implemented Scope
## 4. Architecture and Component Map
## 5. Requirement-to-Code Traceability
## 6. Critical Data and Control Flows
## 7. Platform and Capability Boundaries
## 8. Security and Forensic Constraints
## 9. Verification Contract
## 10. Known Gaps and Next Slices
## 11. Handoff and Update Rules
```

Populate the sections with the evidence map from Step 2 and enforce these statements:

- The implemented product is an M0 file-backed foundation, not the complete PRD feature set.
- Acquisition output reaches `AcquiredUnverified`; independent package verification is a separate step.
- Split-RAW creation exists, but `.fsnap` packaging still assumes one `evidence.raw` and therefore does not package split segments.
- Cancellation exists in the core but is not exposed through the current desktop UI.
- The UI case-identity field is an operator note and is not yet part of the verified core/package model.
- Raw devices, privileged helpers, write blockers, real evidence, and destructive media tests are `NotValidated`.
- Windows, macOS, and Linux hosted CI proves portable build/test behavior only for the commit whose checks were observed.
- Standards named by the PRD are design targets; full compliance has not been demonstrated.

The traceability table must use these columns:

```markdown
| Requirement / behavior | Status | Implementation | Test or evidence | Limitation |
|---|---|---|---|---|
```

The verification contract must list these commands without claiming a pass until they are run in Task 2:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --locked
npm ci --prefix apps/trareon-acquire
npm run build --prefix apps/trareon-acquire
sh scripts/validate-ai-operations.sh
shasum -a 256 -c docs/RFC-BASELINE.sha256
```

- [ ] **Step 4: Inspect the Code document for unsupported claims and broken references**

Run:

```bash
rg -n "production.ready|fully compliant|raw.device.*validated|T(BD|ODO)|FIX(ME)|X{3}" Code-Digital-Forensic-Acquisition.md
git diff --check -- Code-Digital-Forensic-Acquisition.md
```

Expected:

- The first command returns no unsupported readiness/compliance claims and no placeholders.
- `git diff --check` returns no output.

Verify the traceability paths exactly:

```bash
for path in \
  crates/trareon-core/src/lib.rs \
  crates/trareon-core/src/domain.rs \
  crates/trareon-core/src/audit.rs \
  crates/trareon-core/src/acquisition.rs \
  crates/trareon-core/src/package.rs \
  crates/trareon-core/tests/foundation.rs \
  crates/trareon-core/tests/domain.rs \
  crates/trareon-core/tests/audit.rs \
  crates/trareon-core/tests/acquisition.rs \
  crates/trareon-core/tests/package.rs \
  crates/trareon-verifier/src/main.rs \
  crates/trareon-verifier/tests/cli.rs \
  apps/trareon-acquire/src-tauri/src/lib.rs \
  apps/trareon-acquire/src-tauri/tests/ipc.rs \
  apps/trareon-acquire/src/App.svelte \
  .github/workflows/ci.yml; do
  test -e "$path" || exit 1
done
```

Expected: exit code 0 with no output.

- [ ] **Step 5: Commit the Code document**

```bash
git add Code-Digital-Forensic-Acquisition.md
git commit -m "docs: map Trareon acquisition implementation"
```

Expected: one commit containing only `Code-Digital-Forensic-Acquisition.md`.

---

### Task 2: Perform and Record the Snapshot Review

**Files:**
- Create: `Review-Digital-Forensic-Acquisition.md`
- Read: `Code-Digital-Forensic-Acquisition.md`
- Read: all product files and tests listed in Task 1
- Read: `docs/ai-session-log/*.md` for historical evidence boundaries

**Interfaces:**
- Consumes: Task 1 implementation map and fixed product snapshot `5ed92860132f471a2dff3495fde27bbf1ae8616f`.
- Produces: A reproducible decision, findings ledger, and explicit merge/release gates for the reviewed snapshot.

- [ ] **Step 1: Run the local verification ladder before assigning a decision**

Run in order:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --locked
npm ci --prefix apps/trareon-acquire
npm run build --prefix apps/trareon-acquire
sh scripts/validate-ai-operations.sh
shasum -a 256 -c docs/RFC-BASELINE.sha256
```

Expected at the current snapshot:

- Formatting and Clippy complete with exit code 0.
- All Rust workspace tests pass.
- npm installs exactly from the lockfile and the Svelte/Vite build succeeds.
- AI operations validation prints `ai_operations_validation:PASS`.
- RFC checksum prints `RFC-Digital-Forensic-Acquisition.md: OK`.

If a command fails, record the exact command, failure, and affected claim; do not write `PASS` or silently retry.

- [ ] **Step 2: Review correctness, failure, and trust-boundary code paths**

Inspect these exact paths:

```bash
sed -n '1,220p' crates/trareon-core/src/domain.rs
sed -n '1,360p' crates/trareon-core/src/acquisition.rs
sed -n '1,280p' crates/trareon-core/src/audit.rs
sed -n '1,320p' crates/trareon-core/src/package.rs
sed -n '1,180p' apps/trareon-acquire/src-tauri/src/lib.rs
sed -n '1,220p' apps/trareon-acquire/src-tauri/capabilities/default.json
sed -n '1,280p' apps/trareon-acquire/src/App.svelte
```

Review questions and required outcomes:

1. Can the state machine claim `VerifiedComplete` without the verifying transition?
2. Can cancellation or destination failure emit a false-complete state?
3. Does the audit reader reject tampering and unsupported state values?
4. Does package verification reject mutation, truncation, missing files, discontinuous audit, unsupported schema/build identity, unsafe paths, symlinks, and unexpected files where implemented?
5. Are source and destination reversal and path aliasing safely rejected for the file-backed scope?
6. Does the Tauri capability set grant shell or broad filesystem access?
7. Does UI copy imply stronger forensic validity than the core returns?
8. Are split segments represented in the `.fsnap` package contract?

For each problem found, assign one stable ID (`R1`, `R2`, ...), severity (`P0`, `P1`, or `P2`), exact location, impact, evidence, and recommended action. If no defect exists for a question, record the inspected evidence under the corresponding assessment section rather than creating a finding.

- [ ] **Step 3: Create the Review document with a decision derived from evidence**

Create `Review-Digital-Forensic-Acquisition.md` with these exact top-level sections:

```markdown
# Review — Trareon Acquire Digital Forensic Acquisition

## 1. Review Identity
## 2. Decision Summary
## 3. Review Method and Evidence
## 4. Findings
## 5. PRD and RFC Compliance Assessment
## 6. Correctness and Failure Semantics
## 7. Security and Forensic Validity
## 8. Testing and Platform Evidence
## 9. Known Gaps and Limitations
## 10. Required Actions and Gates
```

Use this finding table:

```markdown
| ID | Severity | Location | Impact and evidence | Required action |
|---|---|---|---|---|
```

Derive the decision using these rules:

- `blocked`: a P0 defect or missing evidence makes the stated M0 scope unsafe to evaluate.
- `fix-first`: a P1 defect violates an M0 acceptance criterion or forensic/security invariant.
- `ship`: no P0/P1 defect remains for the stated M0 file-backed scope; P2 follow-ups and `NotValidated` future capabilities are explicitly recorded.

The decision applies only to the reviewed M0 file-backed snapshot. It must not authorize production release, raw-device use, real-evidence use, or full PRD scope.

Record local results from Step 1 separately from historical CI evidence. Historical hosted-CI claims must identify the exact commit named by the session log; if current remote state is not checked, label it historical rather than current.

- [ ] **Step 4: Cross-check Review claims against Code and the fixed snapshot**

Run:

```bash
rg -n "5ed92860132f471a2dff3495fde27bbf1ae8616f|NotValidated|ship|fix-first|blocked" \
  Code-Digital-Forensic-Acquisition.md \
  Review-Digital-Forensic-Acquisition.md
rg -n "T(BD|ODO)|FIX(ME)|X{3}|production.ready|fully compliant" \
  Code-Digital-Forensic-Acquisition.md \
  Review-Digital-Forensic-Acquisition.md
git diff --check -- Review-Digital-Forensic-Acquisition.md
```

Expected:

- Both documents identify the fixed product snapshot and evidence boundaries consistently.
- The placeholder/overclaim scan returns no matches.
- `git diff --check` returns no output.

- [ ] **Step 5: Commit the Review document**

```bash
git add Review-Digital-Forensic-Acquisition.md
git commit -m "docs: review Trareon acquisition foundation"
```

Expected: one commit containing only `Review-Digital-Forensic-Acquisition.md`.

---

### Task 3: Validate the Documentation Set and Record Handoff

**Files:**
- Create: `docs/ai-session-log/2026-07-17-codex-code-review-documents.md`
- Verify: `Code-Digital-Forensic-Acquisition.md`
- Verify: `Review-Digital-Forensic-Acquisition.md`
- Verify: `docs/superpowers/specs/2026-07-17-trareon-code-review-documents-design.md`
- Verify: `docs/superpowers/plans/2026-07-17-trareon-code-review-documents.md`

**Interfaces:**
- Consumes: Completed Code and Review documents plus their verification output.
- Produces: A clean, reviewable branch with a factual session handoff.

- [ ] **Step 1: Verify the final file set and scope**

Run:

```bash
git status --short
git diff --stat 5ed92860132f471a2dff3495fde27bbf1ae8616f...HEAD
git diff --name-only 5ed92860132f471a2dff3495fde27bbf1ae8616f...HEAD
```

Expected changed-file set before the session-log commit:

```text
Code-Digital-Forensic-Acquisition.md
Review-Digital-Forensic-Acquisition.md
docs/superpowers/plans/2026-07-17-trareon-code-review-documents.md
docs/superpowers/specs/2026-07-17-trareon-code-review-documents-design.md
```

No product code, templates, DevSecOps files, or local tool configuration may appear.

- [ ] **Step 2: Run the final documentation quality gate**

Run:

```bash
git diff --check 5ed92860132f471a2dff3495fde27bbf1ae8616f...HEAD
rg -n "T(BD|ODO)|FIX(ME)|X{3}|fill[[:space:]]+in[[:space:]]+details|implement[[:space:]]+later|similar[[:space:]]+to[[:space:]]+Task" \
  Code-Digital-Forensic-Acquisition.md \
  Review-Digital-Forensic-Acquisition.md
test -s Code-Digital-Forensic-Acquisition.md
test -s Review-Digital-Forensic-Acquisition.md
```

Expected:

- `git diff --check` returns no output.
- Placeholder scan returns no matches.
- Both `test -s` commands exit 0.

- [ ] **Step 3: Write the factual session handoff**

Create `docs/ai-session-log/2026-07-17-codex-code-review-documents.md` with these sections and only observed facts:

```markdown
# AI Session Log - 2026-07-17

## Task
## Repository State Discovered
## Files Created
## Commands Run
## Verification Results
## Review Decision
## Next Step and Remaining Risks
```

Record the exact branch, reviewed product commit, documentation commits, commands and exit results, final review decision, and remaining `NotValidated` capabilities.

- [ ] **Step 4: Commit the handoff record**

```bash
git add docs/ai-session-log/2026-07-17-codex-code-review-documents.md
git commit -m "docs: record Code and Review documentation handoff"
```

Expected: the commit contains only the session log. The previously committed design, plan, Code document, and Review document remain separate commits.

- [ ] **Step 5: Confirm clean completion state**

Run:

```bash
git status --short --branch
git log --oneline --decorate -5
```

Expected:

- The isolated `docs/code-review-documents` worktree is clean.
- Recent commits separately show the design, plan, Code document, Review document, and handoff history.
