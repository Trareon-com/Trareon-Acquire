# Evidence Index

Raw evidence berada di `.ai-evidence/` dan tidak di-commit. Tabel ini hanya menyimpan referensi, exact SHA, checksum, hasil teredaksi, serta lokasi lokal.

| Day | Task | Local SHA | Remote SHA | PR SHA | CI SHA | Run | Artifact SHA-256 | Local Path | Result | Reviewed By |
|---|---|---|---|---|---|---|---|---|---|---|
| 00 | Pack baseline | 5bfabef0c3a9aa8194130a44072dc0dfdaab7f0e | NOT_APPLICABLE | NOT_APPLICABLE | NOT_APPLICABLE | Local baseline | NOT_APPLICABLE | NOT_APPLICABLE | EXPECTED_PASS | Codex |
| 01–20 | Foundation + gap-fill | bad982f5218df5e5a7b39bc4f47d7b2a24a5827f | bad982f5218df5e5a7b39bc4f47d7b2a24a5827f | PR-33 | PASS_3OS (PR-33 checks) | Hosted CI | See PR-33 | Workspace | IMPLEMENTED_UNREVIEWED | Codex review NOT_STARTED |
| 21 | DevSecOps gates | 229f2109ad9af27c99729eb846605c9e33ee7a5a | 229f2109ad9af27c99729eb846605c9e33ee7a5a | PR-34 | PASS_3OS | Hosted CI | See PR-34 | deny.toml / ci.yml / SECURITY.md | IMPLEMENTED_UNREVIEWED | Codex review NOT_STARTED |
| 22 | Property and fuzz | f225d33a974aca59c3febf9a0d61e46dc89952b8 | f225d33a974aca59c3febf9a0d61e46dc89952b8 | PR-37 | PASS_3OS | Hosted CI | See PR-37 | tests/properties.rs | IMPLEMENTED_UNREVIEWED | Codex review NOT_STARTED |
| 23 | Linux feasibility | b377e3bec3e9f8286da1e26e09effa1cd9c936c4 | b377e3bec3e9f8286da1e26e09effa1cd9c936c4 | PR-47 | PASS_3OS (PR-47 checks) | Kali real HW + CI | See platform report | docs/platform/day23-linux-feasibility.md | IMPLEMENTED_UNREVIEWED | Human PROVIDED |
| 24 | Windows feasibility | f465ac323fc86094927726e3db8f4aa75e45b399 | f465ac323fc86094927726e3db8f4aa75e45b399 | PR-49 | PASS_3OS | Win10 real HW + CI | See platform report | docs/platform/day24-windows-feasibility.md | IMPLEMENTED_UNREVIEWED | Human PROVIDED |
| 25 | macOS feasibility | a4d1f09cd606edc7bf7127ed9378ebfa48252ac9 | a4d1f09cd606edc7bf7127ed9378ebfa48252ac9 | PR-50 | PASS_3OS (PR-50 checks) | M4 Pro real HW + CI | See platform report | docs/platform/day25-macos-feasibility.md | IMPLEMENTED_UNREVIEWED | Human PROVIDED |
| 26 | Performance baseline | 4008934abccd5a339c97bd58e5ab233693af8581 | 4008934abccd5a339c97bd58e5ab233693af8581 | PR-43 | PASS_3OS | Hosted CI | See docs/performance/ | tests/performance.rs | IMPLEMENTED_UNREVIEWED | Codex review NOT_STARTED |
| 27 | Capability matrix | ff5933353b1d3414ee74c3f1cdf0460b22ec5af6 | ff5933353b1d3414ee74c3f1cdf0460b22ec5af6 | PR-44 | PASS_3OS (merge train) | Docs | NOT_APPLICABLE | docs/CAPABILITY-MATRIX-M0.md | IMPLEMENTED_UNREVIEWED | Codex review NOT_STARTED |
| 28 | Docs / About / legal | f861379abfc1f48c46fbc584fd371c40b6cf9974 | f861379abfc1f48c46fbc584fd371c40b6cf9974 | PR-45 | PASS_3OS (merge train) | Docs + UI | NOT_APPLICABLE | docs/USER-GUIDE.md | IMPLEMENTED_UNREVIEWED | LEGAL_DRAFT_ONLY |
| 29 | Adversarial review | f346430457587901d74370df5e87c1c20a05fc32 | f346430457587901d74370df5e87c1c20a05fc32 | PR-46 | PASS_3OS | Local + CI | NOT_APPLICABLE | docs/ADVERSARIAL-REVIEW-M0.md | IMPLEMENTED_UNREVIEWED | No new P0/P1 |
| 30 | EAC complete redo | 4dd2828b37286f5d99de69825452502ec61bef53 | 4dd2828b37286f5d99de69825452502ec61bef53 | NOT_STARTED | NOT_STARTED | Local Day 30 redo author pass | NOT_APPLICABLE | docs/M0-MILESTONE-REVIEW.md | HUMAN_CLASSIFIED | Human (operator) |
| 30a | Prior incomplete EAC (superseded) | bb59f2cf9730f78cf981a40ef055855f815f4273 | 2b06c61ae8819e26d095d2102334514e778cb8ad | PR-48 | PASS_3OS | Hosted CI | NOT_APPLICABLE | DECISIONS/2026-07-17-day30-eac-human-approval.md | SUPERSEDED | Incomplete before Days 24–25 |

Day 30 classification (complete redo): **Production-Directed Engineering Alpha Candidate — Lab Use Only**.
`.fsnap` v0.1: **frozen for Analysis** (single-`evidence.raw` layout only).
Decision record: [`DECISIONS/2026-07-17-day30-eac-complete-redo.md`](DECISIONS/2026-07-17-day30-eac-complete-redo.md).
Milestone review: [`docs/M0-MILESTONE-REVIEW.md`](../M0-MILESTONE-REVIEW.md).

After the Day 30 complete-redo PR number and hosted CI SHA are known, update the Day 30 Local/Remote/PR/CI SHA cells to those exact values.

Jangan mencatat credential, token, serial sensitif tanpa redaksi, data pribadi, barang bukti, atau signing material.
