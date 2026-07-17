# M0 Risk Register (Day 30)

Frozen against `main` tip `6594840f30a7119174eb19b8c22552869ae51f60`.  
Companion docs: capability matrix, adversarial review, discrepancy register, milestone review.

| ID | Risk | Severity | Likelihood | Status | Mitigation / residual |
|---|---|---|---|---|---|
| R01 | False-complete acquisition | P0-class if present | Low (tested) | Mitigated in file-backed slice | State machine + cancel property tests; UI still cannot arm cancel |
| R02 | Silent package repair / partial verify | P0-class if present | Low (tested) | Mitigated | Fail-closed `verify_fsnap` + golden negative fixtures |
| R03 | Evidence / audit tamper accepted | P0-class if present | Low (tested) | Mitigated | Golden mutated/truncated/audit-discontinuous + property tests |
| R04 | Path escape / extra files in package | High | Low (tested) | Mitigated | Containment + allow-list checks |
| R05 | Privilege / shell escape via Tauri | High | Low (boundary review) | Mitigated for current grant | `core:default` only; no shell plugin; re-check on any capability change |
| R06 | Supply-chain / unmaintained crates | Medium | Known | Accepted with disclosure | `deny.toml` explicit allow-list per advisory ID |
| R07 | Raw-device acquire without privileged broker | High | High if assumed | Open / `NotValidated` | Day 23 Linux privilege boundary confirmed; Days 24–25 pending |
| R08 | Split-RAW not in `.fsnap` | Medium | Certain | Open limitation | Documented; Analysis freeze excludes multi-segment |
| R09 | Missing independent Codex reviews | Medium (process) | Certain | Open | `Review` cells `NOT_STARTED`; EAC is Lab Use Only, not process-complete |
| R10 | Peak RSS / memory ceiling unknown | Medium | Unknown | `NotValidated` | Day 26 measured wall time / hashes only |
| R11 | Full fuzz campaign not run | Medium | Certain | `NotValidated` | Bounded property suite only |
| R12 | Legal/certification overclaim | High | Process | Mitigated by classification | LEGAL_DRAFT_ONLY; Lab Use Only wording; no court claim |
| R13 | Cross-OS fixture CRLF drift | High if regress | Low | Mitigated | `.gitattributes` `-text` on `fixtures/fsnap-v0.1/**` |

## Open P0 / P1

None against the implemented M0 file-backed scope as of Day 30.

## Residual acceptance for EAC

Human classification accepts the residuals above for **Production-Directed Engineering Alpha Candidate — Lab Use Only** and for **Analysis-only** `.fsnap` v0.1 freeze of the single-RAW layout. Residuals do **not** authorize Official Production or raw-device claims.
