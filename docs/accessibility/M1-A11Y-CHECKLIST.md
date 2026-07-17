# M1 accessibility checklist (formal manual audit)

Date: 2026-07-17  
Scope: Foundation demo UI (`apps/trareon-acquire/src/App.svelte`)  
Method: Manual structural review against WCAG 2.2 AA-oriented operator checklist  
Tooling: no automated axe/Lighthouse run (explicitly noted)

| # | Check | Result | Evidence |
|---|---|---|---|
| 1 | Page has a single main landmark | Pass | `<main>` wraps demo |
| 2 | Lab-use limitation is programmatically exposed | Pass | `role="note"` + `id="lab-use-banner"` |
| 3 | Form controls have visible labels | Pass | `<label for="…">` on case/source/output/confirm |
| 4 | Help text associated where needed | Pass | `aria-describedby` on case identity + confirm |
| 5 | Progress/status announced | Pass | `aria-live="polite"` on progress + CoC summary |
| 6 | Busy state on primary actions | Pass | `aria-busy={running}` on Run/Cancel |
| 7 | Cancel reachable while running | Pass | Cancel enabled only when `running` |
| 8 | Completion never invented by UI | Pass | Status derived only from core result/error |
| 9 | About section labelled | Pass | `aria-labelledby="about-heading"` |
| 10 | Color-only success/failure | Residual | Verified/Failed also use text headings; not color-only |
| 11 | Keyboard-only path | Pass (manual) | Native inputs/buttons; no custom widgets trapping focus |
| 12 | Automated a11y scanner | Not run | Deferred — residual for M3 accessibility gate |

**Verdict:** Manual checklist **PASS** for Engineering Alpha lab demo. Automated
scanner evidence remains open and must not be claimed as complete a11y
certification.
