# Perfect Product program — status (2026-07-18)

## Phase A software (M0–M15) — implemented this session

| M | Deliverable | Verification |
|---|---|---|
| M0 | Light-default shell, prefs, nav, About/UNSIGNED | `cargo test -p acquire-slint --lib` |
| M1 | Cases, draft, recent, orphan reject | unit tests |
| M2 | disk_enum, freespace, custody, coverage, sign, archive, report, verifier lib | `trareon-core` / `trareon-verifier` tests |
| M3–M5 | Identify/encryption probe, expert gates, preflight, pause/cancel reasons | unit tests |
| M6 | EvidenceCoC seal, Ed25519, copies, ZIP archive, report, RFC3161 attempt | preserve tests + GUI seal on success |
| M7–M9 | Limited formats, SHA-512 sidecar, split-raw, sources, profiles | unit tests |
| M10–M13 | Tools hub, triage, QMS, multisource, boot dry-run, packaging scripts | unit tests + scripts |
| M14–M15 | Help strings, DESIGN, checklists, matrix notes | docs |

`cargo test -p acquire-slint --lib` — **40 passed**. GUI `cargo check -p acquire-slint` — ok.

## Phase B–E — software prep vs Perfect claim

| Area | Status | Boundary |
|---|---|---|
| M14 a11y/SOP | software/doc complete | Runtime assistive-tech review remains human |
| M15 checklists | document complete | Claims bounded by linked evidence |
| M16 live-gate | **partial** | macOS ATA Unavailable documented; physical WB/disk gates pending human lab |
| M17 format interop | E01-lite smoke + script | Autopsy/FTK open is human checklist |
| M18 Analysis | read-only timeline/browse/report + tests | Full Lab UX polish continues; no package mutation |
| M19 production | RC scripts + sign-off README | Signing/notarization/publish = human-only |

## Non-negotiable release state

`PENDING_HUMAN_SIGN_OFF`

**Perfect Product (A∩B∩C∩D∩E) is NOT claimed.** Software for Phase A is in-tree; Live Gate, court interop, and Official Production remain blocked on human/hardware/legal evidence.
