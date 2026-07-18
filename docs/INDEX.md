# Documentation index — Trareon Acquire

**Primary desktop:** `apps/acquire-slint` (Slint + Rust). Former Tauri/Svelte app removed from the tree (see [decision](ai-operations/DECISIONS/2026-07-17-acquire-slint-gplv3.md)).

## Now / Next / Not Yet

| Band | Meaning | Where |
|------|---------|-------|
| **Now** | File-backed + synthetic acquire → `.fsnap`/RAW/EWF/ZFF, seal, verify, CoC; Slint Guided/Standard/Expert; macOS lab raw allowlist evidence where matrix says PASS | [CAPABILITY-MATRIX-M0.md](CAPABILITY-MATRIX-M0.md), [EVIDENCE.md](format-interop/EVIDENCE.md) |
| **Next** | Cross-OS live-gate rows green; Path A Autopsy/FTK human evidence; Path B libewf oracle green; Windows/Linux Storage Lab Beta | [live-gate-checklist.md](live-gate-checklist.md), matrix gaps |
| **Not Yet** | Court / Official Production; RAM/cloud/mobile/boot write; full Analysis suite | [PRD](../PRD-Digital-Forensic-Acquisition.md) / [RFC](../RFC-Digital-Forensic-Acquisition.md) — product authority only, not daily guides |

## Active (authoritative today)

| Doc | Audience | Authority | Status |
|-----|----------|-----------|--------|
| [CAPABILITY-MATRIX-M0.md](CAPABILITY-MATRIX-M0.md) | Operator + eng | **Current capability SoT** (scoped by OS/device/op) | Active |
| [format-interop/EVIDENCE.md](format-interop/EVIDENCE.md) | Operator + eng | **Interop evidence SoT** | Active |
| [tutorials/OPERATOR-TUTORIAL.md](tutorials/OPERATOR-TUTORIAL.md) | Operator | Full procedure | Active |
| [USER-GUIDE.md](USER-GUIDE.md) | Operator | One-page reference | Active |
| [live-gate-checklist.md](live-gate-checklist.md) | Human lab | Live gates AI cannot check | Active |
| [COMMERCIAL-LAUNCH-STATUS.md](COMMERCIAL-LAUNCH-STATUS.md) | Business | Launch status | Active |
| [ai-operations/START-HERE.md](ai-operations/START-HERE.md) | Agents | Operational pack entry | Active |
| [../.cursor/skills/frontend-design/DESIGN-TRAREON-ACQUIRE.md](../.cursor/skills/frontend-design/DESIGN-TRAREON-ACQUIRE.md) | UI eng | Visual SoT (internal) | Active |
| [../SECURITY.md](../SECURITY.md) | Security | Vulnerability reporting | Active |

## Product authority (not daily implementation guides)

| Doc | Role |
|-----|------|
| [../PRD-Digital-Forensic-Acquisition.md](../PRD-Digital-Forensic-Acquisition.md) | Requirements |
| [../RFC-Digital-Forensic-Acquisition.md](../RFC-Digital-Forensic-Acquisition.md) | Architecture |

## Frozen / historical

Mark as snapshots — do not treat as current UI stack or capability without checking the matrix.

| Doc | Note |
|-----|------|
| [M0-MILESTONE-REVIEW.md](M0-MILESTONE-REVIEW.md), [M1-MILESTONE-REVIEW.md](M1-MILESTONE-REVIEW.md) | Frozen milestone evidence |
| [ai-operations/MASTER-CHECKLIST.md](ai-operations/MASTER-CHECKLIST.md) | M0 day ledger (stale for UI stack) |
| [WEEK-01-DISCREPANCY-REGISTER.md](WEEK-01-DISCREPANCY-REGISTER.md) | Frozen |
| [ADVERSARIAL-REVIEW-M0.md](ADVERSARIAL-REVIEW-M0.md), [M1-FAILURE-MATRIX.md](M1-FAILURE-MATRIX.md) | Frozen-at-SHA reviews |
| [IMPLEMENTATION-ROADMAP.md](IMPLEMENTATION-ROADMAP.md), [AI-DEVELOPMENT-WORKFLOW.md](AI-DEVELOPMENT-WORKFLOW.md) | **HISTORICAL / pre-Slint** in places |
| [superpowers/](superpowers/) | Design provenance; operational pack is `ai-operations/` |
| [COMMERCIAL-90-DAY-DAILY-PLAN.md](COMMERCIAL-90-DAY-DAILY-PLAN.md) | Retrospective implementation map (not a live tracker) |
| [ai-session-log/](ai-session-log/) | Immutable session evidence — not navigation |

## M2 lab evidence (keep these; status lives in matrix)

| Doc | Role |
|-----|------|
| [platform/m2-lab-tiny11-2311-disk10.md](platform/m2-lab-tiny11-2311-disk10.md) | Lab report |
| [platform/m2-tiny11-disk10-operator-runbook.md](platform/m2-tiny11-disk10-operator-runbook.md) | Operator procedure |
| [M2-SOFTWARE-PREP-STATUS.md](M2-SOFTWARE-PREP-STATUS.md) | Prep notes (prefer matrix for status) |

## Deprecated tree

Former `apps/trareon-acquire` (Tauri + Svelte) was archived then **removed** from the repository. Do not restore as the product UI.
