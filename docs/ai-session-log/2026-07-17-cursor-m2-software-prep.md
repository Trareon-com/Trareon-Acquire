# Session log — M2 software prep (Cursor)

Date: 2026-07-17  
Branch: `feat/m2-software-prep`

## Done

- `lab_policy.rs`: classify sources, hard-deny system disks, human-approved allowlist gate
- Wired into `acquire_file` preflight
- Portable fault-injection tests
- M2-P01…P05 plans + DECISION-REQUEST for lab allowlist

## Hard stop

No raw-device I/O, no elevation, no Lab Beta exit claim until human approves allowlisted media.
