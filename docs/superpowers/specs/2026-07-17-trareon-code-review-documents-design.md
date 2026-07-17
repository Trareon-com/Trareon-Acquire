# Trareon Acquire Code and Review Documents Design

## Status

- **Date:** 2026-07-17
- **State:** Approved for implementation planning
- **Product:** Trareon Acquire
- **Deliverables:**
  - `Code-Digital-Forensic-Acquisition.md`
  - `Review-Digital-Forensic-Acquisition.md`

## Purpose

Create two project-specific, evidence-backed documents that connect the approved product and architecture specifications to the implementation that actually exists in the repository.

The documents must help a maintainer answer two different questions:

1. **Code:** What has been implemented, where does it live, which requirement does it satisfy, and how is it verified?
2. **Review:** Is the implementation correct and safe enough for its stated scope, what evidence supports that conclusion, and what remains unresolved?

These are living project documents, not replacements for the reusable templates under `Template/Desktop/`.

## Source of Truth

The documents derive claims from the following sources, in priority order:

1. `RFC-Digital-Forensic-Acquisition.md`
2. `PRD-Digital-Forensic-Acquisition.md`
3. Task acceptance criteria and the approved foundation plan
4. Repository code and committed fixtures
5. Verification commands and observed results
6. Session logs for historical context only

When sources disagree, the document must report the discrepancy rather than silently choose the more favorable claim.

## Selected Approach

Add one authoritative Code document and one authoritative Review document at the repository root.

This approach was selected because it:

- places the implementation record beside the existing project-level PRD and RFC;
- gives contributors a single entry point for current implementation and review status;
- avoids fragmenting the initial M0 record across milestone-specific files; and
- preserves the existing templates as reusable blank forms.

The tradeoff is that both documents must be updated when a milestone materially changes implementation or review status.

## Deliverable 1: Code Document

`Code-Digital-Forensic-Acquisition.md` will describe the repository as implemented, not repeat the entire PRD.

### Required sections

1. **Document identity and snapshot**
   - Product, branch, commit, date, lifecycle status, and scope.
2. **Source documents and precedence**
   - Links to PRD, RFC, plans, workflow, and relevant contracts.
3. **Implemented scope**
   - Current M0 capabilities and explicit non-capabilities.
4. **Architecture and component map**
   - Shared Rust core, verifier, Tauri/Svelte adapter, schemas, fixtures, and CI boundaries.
5. **Requirement-to-code traceability**
   - A compact table mapping implemented requirements to files, tests, and validation status.
6. **Critical data and control flows**
   - Acquisition, audit journal, package creation, independent verification, and UI-to-core flow.
7. **Platform and capability boundaries**
   - Distinguish portable tests from raw-device or hardware validation.
8. **Security and forensic constraints**
   - Synthetic evidence policy, no broad shell/filesystem capability, state semantics, tamper behavior, and dependency boundaries.
9. **Verification contract**
   - Exact commands, expected outcomes, and evidence limitations.
10. **Known gaps and next implementation slices**
    - Only gaps supported by repository evidence, with no speculative completion claims.
11. **Handoff/update rules**
    - What future contributors must update when behavior changes.

### Content rules

- Use repository-relative Markdown links.
- Name exact files and tests where practical.
- Mark untested platform or hardware capability as `NotValidated`.
- Separate implemented behavior from planned behavior.
- Do not claim standards compliance beyond the evidence currently present.

## Deliverable 2: Review Document

`Review-Digital-Forensic-Acquisition.md` will be an evidence-backed review of a fixed repository snapshot.

### Required sections

1. **Review identity**
   - Reviewed commit, branch, reviewer, date, scope, and risk classification.
2. **Decision summary**
   - One of `ship`, `fix-first`, or `blocked`, with a concise rationale.
3. **Review method and evidence**
   - Specifications inspected, code paths reviewed, commands run, and evidence not available.
4. **Findings**
   - Stable finding IDs, severity, location, impact, evidence, and recommended action.
5. **PRD/RFC compliance assessment**
   - Implemented, partial, planned, or `NotValidated` status without inflating M0 into full-product completion.
6. **Correctness and failure semantics**
   - State transitions, cancellation, error paths, package integrity, and verifier behavior.
7. **Security and forensic validity**
   - Trust boundaries, privilege, immutability, audit continuity, dependency/supply-chain posture, and capability claims.
8. **Testing and platform evidence**
   - Unit/integration/fixture coverage, hosted three-OS results, and missing hardware or platform validation.
9. **Known gaps and limitations**
   - Existing documented gaps plus newly verified review findings.
10. **Required actions and merge/release gate**
    - Separate merge blockers, follow-up work, and release-only gates.

### Review rules

- Review the committed snapshot, not an unspecified moving worktree.
- Re-run practical local checks before asserting that they pass.
- Do not copy historical CI status as current without identifying its commit.
- Treat hosted CI as portability evidence, not raw-device evidence.
- Distinguish defects from intentionally deferred product scope.
- If evidence is insufficient, record `NotValidated` rather than infer success.

## Verification Strategy

Before the two final documents are considered complete:

1. Inspect the final Markdown diff.
2. Verify every linked repository path exists.
3. Check that referenced commits and branch names are exact.
4. Run the strongest practical code gates relevant to claims in the documents.
5. Compare all stated gaps against the latest session logs and current code.
6. Scan for `TBD`, `TODO`, contradictory status labels, and unsupported claims.
7. Confirm unrelated staged or untracked files are not included in the documentation commit.

## Acceptance Criteria

- Both project-specific documents exist at the repository root.
- The Code document traces implemented M0 behavior to concrete code and tests.
- The Review document contains an actual snapshot review and explicit decision.
- Planned full-product features are not described as implemented.
- Platform and hardware limitations use explicit evidence labels.
- All repository links resolve.
- Verification commands and outcomes are recorded accurately.
- Existing generic templates remain unchanged unless a concrete defect is found and separately approved.
- The change is isolated from unrelated DevSecOps and local tool configuration files.

## Non-Goals

- Rewriting the PRD, RFC, or implementation roadmap.
- Implementing or fixing product code as part of the documentation change.
- Declaring production readiness or full forensic standards compliance.
- Replacing per-task packets, pull-request reviews, or human approval gates.
- Modifying the reusable Desktop or Website templates.

## Update Policy

Future changes must update the Code document when architecture, implemented capability, or verification commands change. The Review document must identify a new reviewed commit and be re-evaluated when findings close, risk boundaries change, or a milestone seeks a new merge or release decision.
