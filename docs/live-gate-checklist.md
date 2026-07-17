# Live Gate checklist (Bagian 1)

A part that touches hardware or forensic claims is **not done** until this checklist is checked for that part.

## Common gates

- [ ] Source is allowlisted (`lab_policy`), not a system disk
- [ ] Write-blocker detected **or** manual confirm recorded in audit
- [ ] Progress UI updates during acquire
- [ ] Package verifies with `trareon-verifier integrity PATH`
- [ ] Limitations recorded (HPA unknown, platform Unavailable, etc.)
- [ ] Capability matrix row updated honestly

## Part-specific

### Part 1 — HPA/DCO
- [ ] Linux lab disk: `detect_hpa_dco` returns size + status
- [ ] Windows / macOS: Unavailable reason documented if ATA not available

### Part 2 — Write-blocker
- [ ] Hardware blocker plugged → Detected
- [ ] Without blocker → manual confirm path works

### Part 3 — E01
- [ ] Allowlisted block → E01-lite file
- [ ] Optional Autopsy/FTK open noted (may fail — subset)

### Part 8 — HPA unlock
- [ ] Disposable lab disk only
- [ ] `destructive-lab` feature + double confirm
- [ ] Restore attempted; failure → fail-closed

### Parts 11–13
- [ ] Lab VM RAM / triage / pcap / cloud-dir / mobile-dir smoke
- [ ] Matrix marks OS support accurately
