# Master Checklist

Nilai awal seluruh task adalah `NOT_STARTED`. Perbarui hanya dari evidence exact-SHA; jangan mengosongkan cell.

| Day | Task | Author | Reviewer | Risk | Autonomy | Branch | Frozen SHA | Implementation | Review | CI | Platform | Incident | Recovery | Human Gate | Evidence | Next |
|---:|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 01 | Baseline and monitoring | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d01-baseline | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 02 |
| 02 | Workspace boundaries | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d02-workspace | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 03 |
| 03 | Domain identifiers | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d03-domain | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 04 |
| 04 | State transitions | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d04-state | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 05 |
| 05 | Audit hash chain | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d05-audit | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 06 |
| 06 | Canonical hashing | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d06-canonical | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 07 |
| 07 | Week 1 integration | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d07-integration | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 08 |
| 08 | Synthetic streaming | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d08-streaming | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 09 |
| 09 | Identity guard | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d09-identity | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 10 |
| 10 | Failure semantics | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d10-failure | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 11 |
| 11 | RAW and split RAW | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d11-raw | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 12 |
| 12 | fsnap manifest | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d12-manifest | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 13 |
| 13 | Safe package writer | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d13-package | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 14 |
| 14 | Independent verifier | Codex | Claude | High | AUTO_WITH_REVIEW | feat/m0-d14-verifier | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 15 |
| 15 | CLI and fsnap draft | Codex | Claude | High | AUTO_WITH_REVIEW | feat/m0-d15-contract | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 16 |
| 16 | Tauri boundary | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d16-tauri | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 17 |
| 17 | Guided UI | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d17-ui | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 18 |
| 18 | Guidance and accessibility | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d18-guidance | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 19 |
| 19 | CoC and report preview | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d19-report | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 20 |
| 20 | Cross-platform CI | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d20-ci | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 21 |
| 21 | DevSecOps gates | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d21-devsecops | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | HUMAN_IF_DEPENDENCY | NOT_STARTED | 22 |
| 22 | Property and fuzz | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d22-fuzz | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 23 |
| 23 | Linux feasibility | Claude | Codex | High | MANUAL_START | feat/m0-d23-linux | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | REQUIRED_FOR_DEVICE | NOT_STARTED | 24 |
| 24 | Windows feasibility | Claude | Codex | High | MANUAL_START | feat/m0-d24-windows | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | REQUIRED_FOR_DEVICE | NOT_STARTED | 25 |
| 25 | macOS feasibility | Claude | Codex | High | MANUAL_START | feat/m0-d25-macos | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | REQUIRED_FOR_DEVICE | NOT_STARTED | 26 |
| 26 | Performance baseline | Claude | Codex | High | AUTO_WITH_REVIEW | feat/m0-d26-performance | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 27 |
| 27 | Capability matrix and freeze | Codex | Claude | High | AUTO_WITH_REVIEW | feat/m0-d27-matrix | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 28 |
| 28 | Docs About and limitations | Claude | Codex | Medium | AUTO_WITH_REVIEW | feat/m0-d28-docs | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | LEGAL_DRAFT_ONLY | NOT_STARTED | 29 |
| 29 | Adversarial review | Codex | Claude | High | AUTO_WITH_REVIEW | feat/m0-d29-adversarial | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | NOT_REQUIRED | NOT_STARTED | 30 |
| 30 | EAC and fsnap freeze gate | Codex | Human | Release-critical | HUMAN_APPROVAL | feat/m0-d30-gate | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NOT_STARTED | NONE | 0 | REQUIRED | NOT_STARTED | M1 |

## Milestone gates

- **Week 1:** deterministic state/audit foundations, clean integration, zero unresolved P0/P1.
- **Week 2:** byte-accurate synthetic acquisition, safe package, independent verifier, `.fsnap` v0.1 draft.
- **Week 3:** core-owned UI state, guidance, report, three-OS hosted CI, DevSecOps, fuzz/property baseline.
- **Day 30:** `.fsnap` v0.1 golden compatibility fixtures, verifier independence, limitation matrix, performance baseline, complete evidence index, human classification approval.

Day 30 hanya boleh menjadi `Production-Directed Engineering Alpha Candidate — Lab Use Only`. Semua capability tanpa exact hardware evidence tetap `NotValidated`.
