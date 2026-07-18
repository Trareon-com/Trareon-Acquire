# Live Gate checklist (Bagian 1)

A part that touches hardware or forensic claims is **not done** until this checklist is checked for that part.

## Common gates

- [ ] Source is allowlisted (`lab_policy`), not a system disk — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [ ] Write-blocker detected **or** manual confirm recorded in audit — probe reports `Uncertain`; pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [ ] Progress UI updates during acquire — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [ ] Package verifies with `trareon-verifier integrity PATH` — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [x] Limitations recorded (HPA unknown, platform Unavailable, etc.) — [macOS probe output](live-gate-evidence/2026-07-18/hpa-dco-probe-rdisk0.txt) records ATA passthrough as Unavailable
- [x] Capability matrix row updated honestly — M15 notes desktop wiring and live-gate limits

## Part-specific

### Part 1 — HPA/DCO
- [ ] Linux lab disk: `detect_hpa_dco` returns size + status — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [x] Windows / macOS: Unavailable reason documented if ATA not available — [macOS probe output](live-gate-evidence/2026-07-18/hpa-dco-probe-rdisk0.txt) says ATA HPA/DCO passthrough is not implemented on macOS
- Operator helper: `cargo run -p trareon-ata --example lab_hpa_dco_probe -- <path>`
  (prints write-blocker heuristic + HPA/DCO status; does not unlock HPA)

### Part 2 — Write-blocker
- [ ] Hardware blocker plugged → Detected — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [ ] Without blocker → manual confirm path works — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)

### Part 3 — E01
- [ ] Allowlisted block → E01-lite file — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [ ] Optional Autopsy/FTK open noted (may fail — subset) — pending human verification; procedure in `docs/format-interop/README.md`

### Part 8 — HPA unlock
- [ ] Disposable lab disk only — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [ ] `destructive-lab` feature + double confirm — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [ ] Restore attempted; failure → fail-closed — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)

### Parts 11–13
- [ ] Lab VM RAM / triage / pcap / cloud-dir / mobile-dir smoke — pending human lab; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
- [ ] Matrix marks OS support accurately — pending human lab review; procedure in [evidence README](live-gate-evidence/2026-07-18/README.md)
