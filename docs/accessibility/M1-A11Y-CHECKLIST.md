# Accessibility checklist — Slint Acquire bench

Date: 2026-07-18  
Scope: `apps/acquire-slint` (primary desktop UI)  
Prior audit: archived Svelte foundation demo (2026-07-17) — historical only  
Method: Manual structural review against WCAG 2.2 AA-oriented operator checklist  
Tooling: no automated axe/Lighthouse run (Slint native; explicitly noted)

| # | Check | Result | Evidence |
|---|---|---|---|
| 1 | Single primary work surface | Pass | Acquire bench `nav-index == 0`; other navs are task panels |
| 2 | Lab / UNSIGNED limitation visible | Pass | Banner + founder badge copy in `strings.slint` |
| 3 | Form controls have visible labels | Pass | Eyebrow + field labels for source/output/CoC/case |
| 4 | Guided wizard exposes step state | Pass | `StepBadge` strip (Case→Source→WB→Confirm→Start) |
| 5 | Progress / status announced in UI | Pass | Instrument strip + telemetry (phase, MiB/s, ETA) |
| 6 | Busy state disables competing CTAs | Pass | `enabled: !root.busy` on destructive/path actions |
| 7 | Cancel reachable while running | Pass | Cancel enabled when `busy` |
| 8 | Completion never invented by UI | Pass | Seal panel driven only from core result hashes |
| 9 | Deny state is not color-only | Pass | Deny panel title + body + preflight text |
| 10 | Honesty labels (Path A candidate, UNSIGNED, Lab Beta) | Pass | Format chips + unsigned banner + Expert Lab Beta copy |
| 14 | Boot labeled as preview | Pass | Rail: Boot (preview); dry-run only |
| 15 | Experimental controls captioned | Pass | Advanced drawer: not applied to AcquireRequest |
| 11 | Keyboard / focus | Residual | Custom `TouchArea` chips — Tab order depends on Slint backend; verify on target OS |
| 12 | Reduced motion | Residual | Opacity phase animation is short; respect OS reduced-motion when Slint exposes it |
| 13 | Automated a11y scanner | Not run | Native Slint — deferred; do not claim certification |

**Verdict:** Manual checklist **PASS** for lab Alpha Slint shell. Keyboard/focus residual and automated scanner remain open — not a11y certification.
