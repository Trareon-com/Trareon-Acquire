# Design plan — Trareon Acquire shell (frontend-design skill)

## Perfect Product IA (M14)

Acquire is a multi-surface evidence workstation, not a single acquisition screen.
Rail groups follow **ISO/IEC 27037** process language (not internal milestone codes):

1. **Cases** establishes operator and case context (who / case id — CoC start).
2. **Identify** makes source capability and constraints visible (power, encryption, OoV).
3. **Acquire** performs the guarded collection workflow (Prepare → Acquire → Seal).
4. **Triage** is post-acquire review; deep analysis stays out of this bench.
5. **Tools / QMS / Boot** are lab utilities, clearly secondary.
6. **Help / About** keep SOP limits, release identity, and support boundaries reachable.

**Mode** (Guided / Standard / Expert) appears only on the Acquire surface — Hibshi et al.
(IMF 2011) found tool chrome that mixes process with capability raises learning cost.

**Instrument strip** (Case · Write-blocker · Integrity) stays visible like FTK Imager /
Guymager / Autopsy status areas: protection and integrity are first-class readouts, not
buried prose.

Light is the default theme for normal desk and classroom conditions; dark remains an operator
preference for low-light benches. Both themes must preserve the same hierarchy, focus treatment,
status semantics, and restraint. Navigation labels describe user work, never internal modules.

### Research anchors (2026-07-18)
- Peers: FTK Imager, Guymager, Autopsy UI layout, Magnet AXIOM Examine, EnCase density norms.
- Literature: Hibshi/Vidas/Cranor IMF 2011 usability study; ISO/IEC 27037; Altiero visualization
  thesis; usable-digital-forensics frameworks (effectiveness / efficiency / satisfaction).

## Subject
Desktop **evidence acquisition** station for DFIR lab operators.
Audience: examiners comparing Trareon to Magnet AXIOM Process / Belkasoft X acquire /
Cellebrite collection workflows — people who buy tools that look like instruments, not demos.
Single job: choose mode → confirm synthetic source → acquire → leave with a verified seal.

## Competitive read (2026)
- Magnet AXIOM / Belkasoft X: dark themes for long shifts, task-oriented panes, hairline
  structure over decoration, hash/status as first-class readouts.
- Cellebrite Inseyets: simplified guided workflows with clear primary action.
- Avoid cloning neon “cyber HUD” landing-page templates (cyan glow + void black).

## Aesthetic
**Evidence Control Room** — cool deep slate instrument panel, one copper signal for primary
action and verified state (lab brass, not acid green), asymmetric custody rail + work surface.
Quiet density. No cards-for-decoration. No cream paper. No Optima brochure look.

## Token system

### Color
| Name | Hex | Role |
|------|-----|------|
| void | `#0B1014` | Window chrome |
| ink | `#101820` | Left custody rail |
| panel | `#151E27` | Main work surface |
| raised | `#1B2632` | Inputs / seal well |
| hairline | `#2C3A48` | 1px structure |
| copper | `#C4845A` | Primary CTA + verified |
| copper-dim | `#8A5E42` | Idle signal / rail accent |
| readout | `#E6EDF3` | Primary text |
| mute | `#8B9AAB` | Secondary |
| faint | `#5C6B7A` | Eyebrows / chrome |
| deny | `#D16B6B` | Preflight hard-deny |
| ok-mist | `#3D5A4C` | Soft verified well tint |

### Type
- Brand / display: **Avenir Next** — tracked wordmark, restrained display weight
- Body: native UI sans
- Utility (hash, sizes, paths): **Menlo** — cryptographic instrument face

### Layout (operator workflow)
Real job: **Prepare → Acquire → Seal**. UI mirrors that sequence — not a settings form.

```
┌──────────┬──────────────────────────────────────┐
│ Mode rail│  Status · EN|ID · Dark|Light         │
│ Guided   │  Steps: 1 Prepare · 2 Acquire · 3 Seal│
│ Standard │  ┌─ Prepare (demo CTA or paths) ───┐ │
│ Expert   │  ├─ Acquire (one primary CTA) ─────┤ │
│          │  └─ Seal (empty invite / hash hero)┘ │
└──────────┴──────────────────────────────────────┘
```

- Guided: **Load synthetic demo** is the primary until ready; then **Start acquire**.
- Seal empty state explains the next outcome — no giant `(none)` hash.
- Prefs chips stay quiet; copper fill reserved for the primary action.

### Theme + language (runtime)
- Header segmented toggles: **Light | Dark** and **EN | ID**; light starts selected on first use
- Tokens live in `ui/theme.slint`; chrome copy in `ui/strings.slint`
- Rust `UiSnapshot.dark_mode` / `locale` drive prefs; guidance/status/preflight localize with locale
- Responsive: rail + padding compress below ~980px; body scrolls via `Flickable`; action row scrolls horizontally when tight
- Min window ~760×600; preferred 1080×740


## Critique vs AI defaults
- Rejected warm cream + serif + terracotta (prior seal pass drifted here)
- Rejected near-black + acid green / electric cyan HUD
- Rejected broadsheet dense columns
- Dark slate retained because it matches real DFIR workstation norms (AXIOM/Belkasoft),
  not as a generic “dark mode SaaS” skin — copper replaces neon as the single accent
