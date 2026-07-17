# Legal Limitations Draft (M0)

**Status:** Draft only — `LEGAL_DRAFT_ONLY`. Not legal advice. Not a warranty.
Not a certification, accreditation, or court-admissibility opinion.

## Product class

Trareon Acquire M0 is an **Engineering Alpha** laboratory / training slice.
It is intended for synthetic file-backed experimentation by Trareon and
authorized collaborators.

## Explicit non-claims

The software and documentation **do not** claim:

- Production readiness for live digital forensic evidence acquisition.
- Compliance with any named forensic standard or accreditation scheme.
- Validation on raw block devices (Linux / Windows / macOS) until Days 23–25
  produce exact hardware evidence.
- Completeness of chain-of-custody for real cases (UI case identity is an
  operator note only and is outside the cryptographic audit trail).
- Fitness for a particular purpose beyond the documented synthetic demo.

## Operator warnings

1. Do not point M0 at real evidence stores, disks, or privileged devices.
2. Do not treat a green UI card as proof beyond what `trareon-verifier` reports
   for that synthetic package.
3. Do not remove or weaken the lab-use banner for “cleaner” screenshots used
   as marketing or court exhibits.
4. Capability rows marked `NotValidated` in
   [`docs/CAPABILITY-MATRIX-M0.md`](CAPABILITY-MATRIX-M0.md) must remain labeled
   as such in any external communication.

## Attribution

- Product: **Trareon Acquire**
- Organization: **Trareon**
- Primary author / steward (M0): **Yusuf Shalahuddin Al Ayyubi As Sobari**

## Support matrix (draft)

| Channel | Scope |
|---|---|
| GitHub Issues | Engineering defects and M0 runbook tracking |
| Security reports | See [`SECURITY.md`](../SECURITY.md) |
| Legal / certification inquiries | Out of scope for M0 — escalate to human owners |

This draft may be revised before any Release Candidate. Day 30 human approval
is required before any Production-Directed Engineering Alpha Candidate label.
