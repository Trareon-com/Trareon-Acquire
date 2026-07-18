# Live-gate evidence pack — 2026-07-18

This folder is the operator record for physical-device gates. It contains command output, not a
claim that hardware was approved.

## HPA/DCO probe procedure

1. Attach only a disposable, lab-approved device and record its allowlist identity.
2. Confirm the selected path is not the system disk and the destination is separate.
3. Run:

   ```sh
   cargo run -p trareon-ata --example lab_hpa_dco_probe -- /dev/rdisk0
   ```

4. Preserve stdout/stderr in `hpa-dco-probe-rdisk0.txt`, including failure output.
5. Record write-blocker state, device serial, OS version, reviewer, and a verifier result in the
   case record. Do not unlock HPA/DCO unless the separate destructive-lab procedure is approved.

## Evidence state

- Probe output: `hpa-dco-probe-rdisk0.txt`
- Result: pending command capture; a successful process does not substitute for human lab review.
- macOS ATA access may be unavailable. If documented by probe output, that documents only the
  unavailable reason; it does not validate a live ATA capability.

## Required human additions

- [ ] Lab policy/allowlist record
- [ ] Source and destination identification
- [ ] Write-blocker evidence or manual confirmation in audit
- [ ] Operator and independent reviewer names
- [ ] Package-verifier output
- [ ] Capability-matrix and checklist review
